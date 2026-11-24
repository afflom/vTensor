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
            let a3 = 94u64;
            if if nat_eq(a2, a3) {
                true
            } else {
                let a4 = v1;
                let a5 = 95u64;
                nat_eq(a4, a5)
            } {
                Ok(Adt9::C0(f1({
                    let a6 = v1;
                    let a7 = 95u64;
                    nat_eq(a6, a7)
                }, v3, v4, v5, {
                    let m8 = v6;
                    match m8 {
                        None => {
                            let m9 = v7;
                            match m9 {
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
                let a10 = v1;
                let a11 = 96u64;
                if if nat_eq(a10, a11) {
                    true
                } else {
                    let a12 = v1;
                    let a13 = 97u64;
                    nat_eq(a12, a13)
                } {
                    Ok(Adt9::C0(f2({
                        let a14 = v1;
                        let a15 = 96u64;
                        nat_eq(a14, a15)
                    }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                } else {
                    let v15: bool = {
                        let m16 = v7;
                        match m16 {
                            None => {
                                v8
                            }
                            Some(_) => {
                                true
                            }
                        }
                    };
                    let a17 = v1;
                    let a18 = 85u64;
                    if nat_eq(a17, a18) {
                        Ok(Adt9::C0(f3(v2, v3, v4, v5, {
                            let m19 = v6;
                            match m19 {
                                None => {
                                    v15
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        }, v11)?))
                    } else {
                        let a20 = v1;
                        let a21 = 87u64;
                        if nat_eq(a20, a21) {
                            Ok(Adt9::C0(f4(v3, v4, v5, v6, v15, v11)?))
                        } else {
                            let a22 = v1;
                            let a23 = 64u64;
                            if nat_eq(a22, a23) {
                                Ok(Adt9::C0(f5(v2, v11)?))
                            } else {
                                let m24 = v3;
                                match m24 {
                                    None => {
                                        Ok(Adt9::C0(Adt10::C1))
                                    }
                                    Some(v17) => {
                                        let a25 = v1;
                                        let a26 = 48u64;
                                        if if nat_eq(a25, a26) {
                                            true
                                        } else {
                                            let a27 = v1;
                                            let a28 = 56u64;
                                            nat_eq(a27, a28)
                                        } {
                                            Ok(Adt9::C0(f6({
                                                let a29 = v1;
                                                let a30 = 56u64;
                                                nat_eq(a29, a30)
                                            }, v2, v17, v4, v11)?))
                                        } else {
                                            let a31 = v1;
                                            let a32 = 53u64;
                                            if if nat_eq(a31, a32) {
                                                true
                                            } else {
                                                let a33 = v1;
                                                let a34 = 54u64;
                                                nat_eq(a33, a34)
                                            } {
                                                Ok(Adt9::C0(f7({
                                                    let a35 = v1;
                                                    let a36 = 54u64;
                                                    nat_eq(a35, a36)
                                                }, v2, v17, v4, v11)?))
                                            } else {
                                                let a37 = v1;
                                                let a38 = 78u64;
                                                if nat_eq(a37, a38) {
                                                    Ok(Adt9::C0(f8(v2, v17, v4, v5, v11)?))
                                                } else {
                                                    let a39 = v1;
                                                    let a40 = 23u64;
                                                    if if if nat_eq(a39, a40) {
                                                        true
                                                    } else {
                                                        let a41 = v1;
                                                        let a42 = 26u64;
                                                        nat_eq(a41, a42)
                                                    } {
                                                        true
                                                    } else {
                                                        let a43 = v1;
                                                        let a44 = 25u64;
                                                        nat_eq(a43, a44)
                                                    } {
                                                        Ok(Adt9::C0(f9(v1, v2, v17, v4, v11)?))
                                                    } else {
                                                        let a45 = v1;
                                                        let a46 = 24u64;
                                                        if nat_eq(a45, a46) {
                                                            Ok(Adt9::C0(f10(v2, v17, v11)?))
                                                        } else {
                                                            let a47 = v1;
                                                            let a48 = 27u64;
                                                            if nat_eq(a47, a48) {
                                                                Ok(Adt9::C0(f11(v2, v17, v11)?))
                                                            } else {
                                                                let a49 = v1;
                                                                let a50 = 3u64;
                                                                if nat_eq(a49, a50) {
                                                                    Ok(Adt9::C0(f12(v17, v4, v5, v11)?))
                                                                } else {
                                                                    f13(v1, v2, v17, v4, v11)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
            f16(v23, v24, v25, v26, v27)
        }
        Adt0::C2(v38) => {
            f18(v38)
        }
        Adt0::C3(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51) => {
            Ok(f19(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51))
        }
        Adt0::C4(v18, v19, v20, v21) => {
            let a52 = 0u64;
            let a53 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h51, _, _, _, _, _, _, _) => {
                    h51
                }
            };
            if nat_lt(a52, a53) {
                f14(v18, v19, v20, v21)
            } else {
                f15(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a54 = 3240099840u64;
                let a55 = v22;
                if if nat_le(a54, a55) {
                    let a56 = v22;
                    let a57 = 4286578688u64;
                    nat_le(a56, a57)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a58 = 1092616192u64;
                    let a59 = v22;
                    if if nat_le(a58, a59) {
                        let a60 = v22;
                        let a61 = 2139095040u64;
                        nat_le(a60, a61)
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
            f17(v28, v29, v30, v31, v32, true, v33, v34, v35, v36, v37)
        }
    }
}

pub fn f1(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m62 = v1.clone();
        match m62 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m63 = v2.clone();
        match m63 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m64 = v1.clone();
        match m64 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m65 = v3.clone();
        match m65 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m66 = v2.clone();
        match m66 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a72 = {
            let a69 = match v10.clone() {
                Adt2::C0(_, _, h67, _, _, _, _, _, _, _, _) => {
                    h67
                }
            };
            let a70 = match v10.clone() {
                Adt2::C0(_, _, _, h68, _, _, _, _, _, _, _) => {
                    h68
                }
            };
            nat_mul(a69, a70)?
        };
        let a73 = match v10.clone() {
            Adt2::C0(_, _, _, _, h71, _, _, _, _, _, _) => {
                h71
            }
        };
        nat_mul(a72, a73)?
    };
    let m74 = v1;
    if if if if if if if if if if match m74 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m75 = v2;
        match m75 {
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
            let m76 = v3;
            match m76 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m77 = v3;
            !match m77 {
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
        let a78 = 0u64;
        let a79 = v14;
        nat_lt(a78, a79)
    } else {
        false
    } {
        let a81 = 0u64;
        let a82 = match v10.clone() {
            Adt2::C0(_, h80, _, _, _, _, _, _, _, _, _) => {
                h80
            }
        };
        nat_lt(a81, a82)
    } else {
        false
    } {
        f20(v10.clone(), v13.clone())
    } else {
        false
    } {
        f21(v10.clone(), v14, v0)?
    } else {
        false
    } {
        f21(v13.clone(), v14, v0)?
    } else {
        false
    } {
        if v0 {
            let a84 = match v7.clone() {
                Adt2::C0(h83, _, _, _, _, _, _, _, _, _, _) => {
                    h83
                }
            };
            let a85 = 0u64;
            if if if if nat_eq(a84, a85) {
                let a86 = f22(v7.clone(), 1u64)?;
                let a87 = 1u64;
                nat_eq(a86, a87)
            } else {
                false
            } {
                let a89 = 4u64;
                let a90 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h88, _) => {
                        h88
                    }
                };
                nat_le(a89, a90)
            } else {
                false
            } {
                f20(v10.clone(), v5.clone())
            } else {
                false
            } {
                f21(v5.clone(), v14, true)?
            } else {
                false
            }
        } else {
            let a92 = match v5.clone() {
                Adt2::C0(h91, _, _, _, _, _, _, _, _, _, _) => {
                    h91
                }
            };
            let a93 = 0u64;
            if if nat_eq(a92, a93) {
                let a94 = f22(v5.clone(), 1u64)?;
                let a95 = 1u64;
                nat_eq(a94, a95)
            } else {
                false
            } {
                let a97 = 4u64;
                let a98 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h96, _) => {
                        h96
                    }
                };
                nat_le(a97, a98)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a104 = {
                let a102 = 2u64;
                let a103 = {
                    let a100 = v14;
                    let a101 = f23(v0, match v10.clone() {
                        Adt2::C0(_, h99, _, _, _, _, _, _, _, _, _) => {
                            h99
                        }
                    })?;
                    nat_mul(a100, a101)?
                };
                nat_add(a102, a103)?
            };
            let a105 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a104, a105)?
        }, 4u64, f24(v10, 1u64), f24(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h106, _, _, _, _) => {
                h106
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m107 = v3.clone();
        match m107 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m108 = v4.clone();
        match m108 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m109 = v5.clone();
        match m109 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m110 = v6.clone();
        match m110 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m111 = v7.clone();
        match m111 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m112 = v5.clone();
        match m112 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m113 = v3;
    if if if if if if if if if if if match m113 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m114 = v4;
        match m114 {
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
        let m115 = v5;
        match m115 {
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
            let m116 = v6;
            if match m116 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m117 = v7;
                match m117 {
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
            let m118 = v6;
            if !match m118 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m119 = v7;
                !match m119 {
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
            let a126 = {
                let a123 = {
                    let a120 = v1;
                    let a121 = 4u64;
                    let a122 = 0u64;
                    nat_quot(a120, a121, a122)
                };
                let a124 = 2u64;
                let a125 = 0u64;
                nat_rem(a123, a124, a125)
            };
            let a127 = 1u64;
            nat_eq(a126, a127)
        } else {
            false
        } {
            let a129 = match v11.clone() {
                Adt2::C0(h128, _, _, _, _, _, _, _, _, _, _) => {
                    h128
                }
            };
            let a130 = 0u64;
            nat_eq(a129, a130)
        } else {
            false
        } {
            let a132 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h131, _, _, _, _, _) => {
                    h131
                }
            };
            let a133 = 4u64;
            nat_eq(a132, a133)
        } else {
            false
        } {
            f25(v11.clone(), f22(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f26(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a135 = match v11.clone() {
                Adt2::C0(_, h134, _, _, _, _, _, _, _, _, _) => {
                    h134
                }
            };
            let a136 = 1u64;
            if nat_le(a135, a136) {
                true
            } else {
                let a139 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h137, _, _, _, _, _) => {
                        h137
                    }
                };
                let a140 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h138, _, _, _, _, _) => {
                        h138
                    }
                };
                nat_eq(a139, a140)
            }
        } else {
            false
        } {
            let a142 = match v11.clone() {
                Adt2::C0(_, _, h141, _, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            if nat_le(a142, a143) {
                true
            } else {
                let a146 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h144, _, _, _, _) => {
                        h144
                    }
                };
                let a147 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h145, _, _, _, _) => {
                        h145
                    }
                };
                nat_eq(a146, a147)
            }
        } else {
            false
        } {
            let a149 = match v11.clone() {
                Adt2::C0(_, _, _, h148, _, _, _, _, _, _, _) => {
                    h148
                }
            };
            let a150 = 1u64;
            if nat_le(a149, a150) {
                true
            } else {
                let a153 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h151, _, _, _) => {
                        h151
                    }
                };
                let a154 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h152, _, _, _) => {
                        h152
                    }
                };
                nat_eq(a153, a154)
            }
        } else {
            false
        } {
            let a156 = match v11.clone() {
                Adt2::C0(_, _, _, _, h155, _, _, _, _, _, _) => {
                    h155
                }
            };
            let a157 = 1u64;
            if nat_le(a156, a157) {
                true
            } else {
                let a160 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h158, _, _) => {
                        h158
                    }
                };
                let a161 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h159, _, _) => {
                        h159
                    }
                };
                nat_eq(a160, a161)
            }
        } else {
            false
        }
    } else {
        false
    } {
        f26(v11.clone(), v13)?
    } else {
        false
    } {
        if v0 {
            if f26(v11.clone(), v15)? {
                f26(v11.clone(), v17)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a163 = match v20.clone() {
            Adt2::C0(h162, _, _, _, _, _, _, _, _, _, _) => {
                h162
            }
        };
        let a164 = 0u64;
        nat_eq(a163, a164)
    } else {
        false
    } {
        let a165 = f22(v20.clone(), 1u64)?;
        let a166 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a165, a166)
    } else {
        false
    } {
        let a168 = if v0 {
            28u64
        } else {
            8u64
        };
        let a169 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h167, _) => {
                h167
            }
        };
        nat_le(a168, a169)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a172 = f27(v0);
            let a173 = {
                let a170 = f22(v11.clone(), 1u64)?;
                let a171 = f28(v0);
                nat_mul(a170, a171)?
            };
            nat_add(a172, a173)?
        }, 4u64, f24(v11, 1u64), f24(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m174 = v1.clone();
        match m174 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m175 = v2.clone();
        match m175 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m176 = v3.clone();
        match m176 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h177, _, _, _, _, _, _, _, _, _, _) => {
            h177
        }
    };
    let m178 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m178 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m179 = v2;
        match m179 {
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
        let m180 = v3;
        match m180 {
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
        let a182 = match v7.clone() {
            Adt2::C0(h181, _, _, _, _, _, _, _, _, _, _) => {
                h181
            }
        };
        let a183 = 0u64;
        if if nat_eq(a182, a183) {
            f25(v7.clone(), f22(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f29(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a185 = match v9.clone() {
            Adt2::C0(h184, _, _, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a186 = 0u64;
        if if nat_eq(a185, a186) {
            f25(v9.clone(), f22(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f29(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a188 = match v11.clone() {
            Adt2::C0(h187, _, _, _, _, _, _, _, _, _, _) => {
                h187
            }
        };
        let a189 = 0u64;
        if if nat_eq(a188, a189) {
            f25(v11.clone(), f22(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f29(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a191 = match v7.clone() {
            Adt2::C0(_, h190, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        let a192 = 24u64;
        nat_eq(a191, a192)
    } else {
        false
    } {
        let a194 = match v7.clone() {
            Adt2::C0(_, _, _, h193, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a195 = 1u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a197 = match v7.clone() {
            Adt2::C0(_, _, _, _, h196, _, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = 1u64;
        nat_eq(a197, a198)
    } else {
        false
    } {
        let a200 = match v11.clone() {
            Adt2::C0(_, h199, _, _, _, _, _, _, _, _, _) => {
                h199
            }
        };
        let a201 = 24u64;
        nat_eq(a200, a201)
    } else {
        false
    } {
        let a203 = match v11.clone() {
            Adt2::C0(_, _, h202, _, _, _, _, _, _, _, _) => {
                h202
            }
        };
        let a204 = 1u64;
        nat_eq(a203, a204)
    } else {
        false
    } {
        let a206 = match v11.clone() {
            Adt2::C0(_, _, _, h205, _, _, _, _, _, _, _) => {
                h205
            }
        };
        let a207 = 1u64;
        nat_eq(a206, a207)
    } else {
        false
    } {
        let a209 = match v11.clone() {
            Adt2::C0(_, _, _, _, h208, _, _, _, _, _, _) => {
                h208
            }
        };
        let a210 = 1u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a212 = match v9.clone() {
            Adt2::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                h211
            }
        };
        let a213 = 1u64;
        nat_eq(a212, a213)
    } else {
        false
    } {
        let a215 = match v9.clone() {
            Adt2::C0(_, _, _, h214, _, _, _, _, _, _, _) => {
                h214
            }
        };
        let a216 = 1u64;
        nat_eq(a215, a216)
    } else {
        false
    } {
        let a218 = match v9.clone() {
            Adt2::C0(_, _, _, _, h217, _, _, _, _, _, _) => {
                h217
            }
        };
        let a219 = 1u64;
        nat_eq(a218, a219)
    } else {
        false
    } {
        let a221 = 3u64;
        let a222 = match v9.clone() {
            Adt2::C0(_, h220, _, _, _, _, _, _, _, _, _) => {
                h220
            }
        };
        nat_le(a221, a222)
    } else {
        false
    } {
        let a224 = match v5.clone() {
            Adt2::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        };
        let a225 = 4u64;
        nat_eq(a224, a225)
    } else {
        false
    } {
        let a227 = match v5.clone() {
            Adt2::C0(_, _, h226, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a228 = 4u64;
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a231 = match v5.clone() {
            Adt2::C0(_, _, _, h229, _, _, _, _, _, _, _) => {
                h229
            }
        };
        let a232 = match v7.clone() {
            Adt2::C0(_, _, h230, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a234 = match v5.clone() {
            Adt2::C0(_, _, _, _, h233, _, _, _, _, _, _) => {
                h233
            }
        };
        let a235 = 1u64;
        nat_eq(a234, a235)
    } else {
        false
    } {
        f30(v5.clone())?
    } else {
        false
    } {
        let a236 = 0u64;
        let a237 = v12;
        nat_lt(a236, a237)
    } else {
        false
    } {
        let a238 = v12;
        let a239 = 2147483648u64;
        nat_lt(a238, a239)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a241 = match v7.clone() {
                Adt2::C0(_, _, h240, _, _, _, _, _, _, _, _) => {
                    h240
                }
            };
            let a242 = f31(v12)?;
            nat_mul(a241, a242)?
        }, 4u64, f24(v7, 1u64), f24(v5, 1u64), Adt4::C38(f24(v9, 1u64), f24(v11, 1u64), match v0 {
            Adt1::C0(h243, _, _, _, _, _, _, _, _, _, _, _) => {
                h243
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m244 = v0.clone();
        match m244 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m245 = v1.clone();
        match m245 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m246 = v2.clone();
        match m246 {
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
        let m247 = v0;
        match m247 {
            None => {
                false
            }
            Some(_) => {
                let m248 = v1;
                match m248 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m249 = v2;
                        match m249 {
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
        let a251 = match v7.clone() {
            Adt2::C0(h250, _, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        };
        let a252 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a251, a252) {
            let a254 = match v7.clone() {
                Adt2::C0(_, _, _, _, h253, _, _, _, _, _, _) => {
                    h253
                }
            };
            let a255 = 1u64;
            nat_eq(a254, a255)
        } else {
            false
        } {
            f25(v7.clone(), f22(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f29(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a257 = match v9.clone() {
                Adt2::C0(h256, _, _, _, _, _, _, _, _, _, _) => {
                    h256
                }
            };
            let a258 = 0u64;
            if if if nat_eq(a257, a258) {
                let a260 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h259, _, _, _, _, _, _) => {
                        h259
                    }
                };
                let a261 = 1u64;
                nat_eq(a260, a261)
            } else {
                false
            } {
                f25(v9.clone(), f22(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f29(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a263 = match v11.clone() {
                Adt2::C0(h262, _, _, _, _, _, _, _, _, _, _) => {
                    h262
                }
            };
            let a264 = 0u64;
            if if if nat_eq(a263, a264) {
                let a266 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h265, _, _, _, _, _, _) => {
                        h265
                    }
                };
                let a267 = 1u64;
                nat_eq(a266, a267)
            } else {
                false
            } {
                f25(v11.clone(), f22(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f29(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a269 = match v7.clone() {
                Adt2::C0(_, _, _, h268, _, _, _, _, _, _, _) => {
                    h268
                }
            };
            let a270 = 1u64;
            nat_eq(a269, a270)
        } else {
            false
        } {
            let a272 = match v11.clone() {
                Adt2::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                    h271
                }
            };
            let a273 = 1u64;
            nat_eq(a272, a273)
        } else {
            false
        } {
            let a276 = match v9.clone() {
                Adt2::C0(_, h274, _, _, _, _, _, _, _, _, _) => {
                    h274
                }
            };
            let a277 = match v7.clone() {
                Adt2::C0(_, h275, _, _, _, _, _, _, _, _, _) => {
                    h275
                }
            };
            nat_eq(a276, a277)
        } else {
            false
        } {
            let a280 = match v9.clone() {
                Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                    h278
                }
            };
            let a281 = match v7.clone() {
                Adt2::C0(_, _, h279, _, _, _, _, _, _, _, _) => {
                    h279
                }
            };
            nat_eq(a280, a281)
        } else {
            false
        } {
            let a284 = match v11.clone() {
                Adt2::C0(_, h282, _, _, _, _, _, _, _, _, _) => {
                    h282
                }
            };
            let a285 = match v9.clone() {
                Adt2::C0(_, _, h283, _, _, _, _, _, _, _, _) => {
                    h283
                }
            };
            nat_eq(a284, a285)
        } else {
            false
        } {
            let a288 = match v11.clone() {
                Adt2::C0(_, _, h286, _, _, _, _, _, _, _, _) => {
                    h286
                }
            };
            let a289 = match v7.clone() {
                Adt2::C0(_, _, h287, _, _, _, _, _, _, _, _) => {
                    h287
                }
            };
            nat_eq(a288, a289)
        } else {
            false
        } {
            let a291 = 0u64;
            let a292 = match v9.clone() {
                Adt2::C0(_, _, h290, _, _, _, _, _, _, _, _) => {
                    h290
                }
            };
            if nat_lt(a291, a292) {
                f20(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f30(v5.clone())?
        } else {
            false
        } {
            let m293 = v3.clone();
            match m293 {
                None => {
                    true
                }
                Some(v15) => {
                    let a295 = match v15.clone() {
                        Adt2::C0(h294, _, _, _, _, _, _, _, _, _, _) => {
                            h294
                        }
                    };
                    let a296 = 0u64;
                    if if if if if if nat_eq(a295, a296) {
                        let a298 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h297, _, _, _, _, _, _) => {
                                h297
                            }
                        };
                        let a299 = 1u64;
                        nat_eq(a298, a299)
                    } else {
                        false
                    } {
                        f25(v15.clone(), f22(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f29(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a302 = match v15.clone() {
                            Adt2::C0(_, h300, _, _, _, _, _, _, _, _, _) => {
                                h300
                            }
                        };
                        let a303 = match v9.clone() {
                            Adt2::C0(_, _, h301, _, _, _, _, _, _, _, _) => {
                                h301
                            }
                        };
                        nat_eq(a302, a303)
                    } else {
                        false
                    } {
                        let a306 = match v15.clone() {
                            Adt2::C0(_, _, h304, _, _, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        let a307 = match v9.clone() {
                            Adt2::C0(_, _, h305, _, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        nat_eq(a306, a307)
                    } else {
                        false
                    } {
                        let a310 = match v15 {
                            Adt2::C0(_, _, _, h308, _, _, _, _, _, _, _) => {
                                h308
                            }
                        };
                        let a311 = match v7.clone() {
                            Adt2::C0(_, _, h309, _, _, _, _, _, _, _, _) => {
                                h309
                            }
                        };
                        nat_eq(a310, a311)
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
            let m312 = v3;
            match m312 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f24(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a314 = f22(v5.clone(), 1u64)?;
            let a315 = f32(match v9.clone() {
                Adt2::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                    h313
                }
            }, v17.clone())?;
            nat_mul(a314, a315)?
        }, 4u64, f24(v7, 1u64), f24(v5, 1u64), Adt4::C37(f24(v9, 1u64), f24(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f22(v1.clone(), 1u64)?;
    let a317 = match v1.clone() {
        Adt2::C0(h316, _, _, _, _, _, _, _, _, _, _) => {
            h316
        }
    };
    let a318 = 0u64;
    if if if if if if if if nat_eq(a317, a318) {
        let a320 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h319, _, _, _, _, _) => {
                h319
            }
        };
        let a321 = 4u64;
        nat_eq(a320, a321)
    } else {
        false
    } {
        f33(match v0.clone() {
            Adt1::C0(h322, _, _, _, _, _, _, _, _, _, _, _) => {
                h322
            }
        })
    } else {
        false
    } {
        f33(match v0.clone() {
            Adt1::C0(_, h323, _, _, _, _, _, _, _, _, _, _) => {
                h323
            }
        })
    } else {
        false
    } {
        let a325 = 0u64;
        let a326 = match v0.clone() {
            Adt1::C0(_, _, h324, _, _, _, _, _, _, _, _, _) => {
                h324
            }
        };
        nat_lt(a325, a326)
    } else {
        false
    } {
        let a328 = match v0.clone() {
            Adt1::C0(_, _, h327, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        };
        let a329 = 2139095040u64;
        nat_lt(a328, a329)
    } else {
        false
    } {
        let a330 = v2;
        let a331 = 9223372036854775808u64;
        nat_lt(a330, a331)
    } else {
        false
    } {
        let a333 = f34(v2)?;
        let a334 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h332, _) => {
                h332
            }
        };
        nat_le(a333, a334)
    } else {
        false
    } {
        Ok(Adt10::C4(f35(v2)?, 4u64, f24(v1.clone(), 1u64), f24(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h335, _, _, _, _, _, _, _, _, _, _, _) => {
                h335
            }
        }, match v0.clone() {
            Adt1::C0(_, h336, _, _, _, _, _, _, _, _, _, _) => {
                h336
            }
        }, match v0 {
            Adt1::C0(_, _, h337, _, _, _, _, _, _, _, _, _) => {
                h337
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m338 = v3;
    match m338 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f36(v0, v2.clone());
            let v7: Adt3 = f36(v0, v5.clone());
            let v8: Adt3 = f36(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h339, _, _, _, _, _, _, _) => {
                    h339
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h340, _, _, _, _, _, _) => {
                    h340
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h341, _, _, _, _, _) => {
                    h341
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h342, _, _, _, _) => {
                    h342
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h343, _, _, _, _, _, _, _) => {
                    h343
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h344, _, _, _, _, _, _) => {
                    h344
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h345, _, _, _, _, _) => {
                    h345
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h346, _, _, _, _, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let v17: u64 = {
                let a351 = v13;
                let a352 = {
                    let a349 = {
                        let a347 = v9;
                        let a348 = 1u64;
                        nat_sub(a347, a348)
                    };
                    let a350 = v16;
                    nat_mul(a349, a350)?
                };
                nat_add(a351, a352)?
            };
            let v18: u64 = {
                let a357 = v14;
                let a358 = {
                    let a355 = {
                        let a353 = v10;
                        let a354 = 1u64;
                        nat_sub(a353, a354)
                    };
                    let a356 = v16;
                    nat_mul(a355, a356)?
                };
                nat_add(a357, a358)?
            };
            let v19: u64 = {
                let a361 = {
                    let a359 = v9;
                    let a360 = v10;
                    nat_mul(a359, a360)?
                };
                let a362 = v12;
                nat_mul(a361, a362)?
            };
            let v20: u64 = {
                let a365 = {
                    let a363 = v13;
                    let a364 = v14;
                    nat_mul(a363, a364)?
                };
                let a366 = v15;
                nat_mul(a365, a366)?
            };
            let v21: u64 = {
                let a367 = v19;
                let a368 = 32u64;
                if nat_lt(a367, a368) {
                    v19
                } else {
                    32u64
                }
            };
            let a370 = match v2.clone() {
                Adt2::C0(h369, _, _, _, _, _, _, _, _, _, _) => {
                    h369
                }
            };
            let a371 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a370, a371) {
                true
            } else {
                let a373 = match v2.clone() {
                    Adt2::C0(h372, _, _, _, _, _, _, _, _, _, _) => {
                        h372
                    }
                };
                let a374 = 1u64;
                nat_eq(a373, a374)
            } {
                let a376 = match v5.clone() {
                    Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                        h375
                    }
                };
                let a377 = 0u64;
                nat_eq(a376, a377)
            } else {
                false
            } {
                let a379 = match v4.clone() {
                    Adt2::C0(h378, _, _, _, _, _, _, _, _, _, _) => {
                        h378
                    }
                };
                let a380 = 0u64;
                nat_eq(a379, a380)
            } else {
                false
            } {
                let a381 = v16;
                let a382 = 2147483648u64;
                nat_lt(a381, a382)
            } else {
                false
            } {
                let a383 = 0u64;
                let a384 = v9;
                nat_lt(a383, a384)
            } else {
                false
            } {
                let a385 = 0u64;
                let a386 = v10;
                nat_lt(a385, a386)
            } else {
                false
            } {
                let a387 = 0u64;
                let a388 = v11;
                nat_lt(a387, a388)
            } else {
                false
            } {
                let a389 = 0u64;
                let a390 = v12;
                nat_lt(a389, a390)
            } else {
                false
            } {
                let a391 = 0u64;
                let a392 = v13;
                nat_lt(a391, a392)
            } else {
                false
            } {
                let a393 = 0u64;
                let a394 = v14;
                nat_lt(a393, a394)
            } else {
                false
            } {
                let a395 = 0u64;
                let a396 = v15;
                nat_lt(a395, a396)
            } else {
                false
            } {
                let a397 = 0u64;
                let a398 = v16;
                nat_lt(a397, a398)
            } else {
                false
            } {
                let a400 = match v6.clone() {
                    Adt3::C0(_, _, _, h399, _, _, _, _) => {
                        h399
                    }
                };
                let a401 = v11;
                nat_eq(a400, a401)
            } else {
                false
            } {
                let a403 = match v8.clone() {
                    Adt3::C0(h402, _, _, _, _, _, _, _) => {
                        h402
                    }
                };
                let a404 = v17;
                nat_eq(a403, a404)
            } else {
                false
            } {
                let a406 = match v8.clone() {
                    Adt3::C0(_, h405, _, _, _, _, _, _) => {
                        h405
                    }
                };
                let a407 = v18;
                nat_eq(a406, a407)
            } else {
                false
            } {
                let a409 = match v8.clone() {
                    Adt3::C0(_, _, h408, _, _, _, _, _) => {
                        h408
                    }
                };
                let a410 = v15;
                nat_eq(a409, a410)
            } else {
                false
            } {
                let a412 = match v8.clone() {
                    Adt3::C0(_, _, _, h411, _, _, _, _) => {
                        h411
                    }
                };
                let a413 = v12;
                nat_eq(a412, a413)
            } else {
                false
            } {
                f37(v4.clone())?
            } else {
                false
            } {
                let a418 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h414, _, _, _, _, _) => {
                        h414
                    }
                };
                let a419 = {
                    let a416 = match v2.clone() {
                        Adt2::C0(h415, _, _, _, _, _, _, _, _, _, _) => {
                            h415
                        }
                    };
                    let a417 = 1u64;
                    if nat_eq(a416, a417) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a418, a419)
            } else {
                false
            } {
                let a421 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h420, _, _, _, _, _) => {
                        h420
                    }
                };
                let a422 = 4u64;
                nat_eq(a421, a422)
            } else {
                false
            } {
                f29(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f29(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a427 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h423, _, _, _, _) => {
                            h423
                        }
                    };
                    let a428 = {
                        let a425 = v13;
                        let a426 = match v2 {
                            Adt2::C0(_, _, _, _, _, h424, _, _, _, _, _) => {
                                h424
                            }
                        };
                        nat_mul(a425, a426)?
                    };
                    nat_eq(a427, a428)
                } else {
                    let a430 = match v1.clone() {
                        Adt1::C0(_, h429, _, _, _, _, _, _, _, _, _, _) => {
                            h429
                        }
                    };
                    let a431 = 0u64;
                    if if if if if if nat_eq(a430, a431) {
                        let a433 = match v1 {
                            Adt1::C0(_, _, h432, _, _, _, _, _, _, _, _, _) => {
                                h432
                            }
                        };
                        let a434 = 1u64;
                        nat_eq(a433, a434)
                    } else {
                        false
                    } {
                        let a436 = match v2 {
                            Adt2::C0(_, _, _, _, h435, _, _, _, _, _, _) => {
                                h435
                            }
                        };
                        let a437 = 1u64;
                        nat_eq(a436, a437)
                    } else {
                        false
                    } {
                        let a439 = match v5.clone() {
                            Adt2::C0(_, _, _, h438, _, _, _, _, _, _, _) => {
                                h438
                            }
                        };
                        let a440 = 1u64;
                        nat_eq(a439, a440)
                    } else {
                        false
                    } {
                        let a442 = match v5 {
                            Adt2::C0(_, _, _, _, h441, _, _, _, _, _, _) => {
                                h441
                            }
                        };
                        let a443 = 1u64;
                        nat_eq(a442, a443)
                    } else {
                        false
                    } {
                        let a445 = match v4.clone() {
                            Adt2::C0(_, _, _, h444, _, _, _, _, _, _, _) => {
                                h444
                            }
                        };
                        let a446 = 1u64;
                        nat_eq(a445, a446)
                    } else {
                        false
                    } {
                        let a448 = match v4 {
                            Adt2::C0(_, _, _, _, h447, _, _, _, _, _, _) => {
                                h447
                            }
                        };
                        let a449 = 1u64;
                        nat_eq(a448, a449)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a450 = v11;
                    let a451 = v20;
                    nat_mul(a450, a451)?
                }, v21, {
                    let a452 = v21;
                    let a453 = v11;
                    nat_mul(a452, a453)?
                }, {
                    let a454 = v21;
                    let a455 = v20;
                    nat_mul(a454, a455)?
                }, {
                    let a460 = {
                        let a456 = v17;
                        let a457 = v18;
                        nat_mul(a456, a457)?
                    };
                    let a461 = {
                        let a458 = v15;
                        let a459 = v12;
                        nat_mul(a458, a459)?
                    };
                    nat_mul(a460, a461)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f7(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m462 = v3;
    match m462 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h463, _, _, _, _, _, _, _, _, _) => {
                    h463
                }
            }, match v1.clone() {
                Adt1::C0(h464, _, _, _, _, _, _, _, _, _, _, _) => {
                    h464
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h465, _, _, _, _, _, _, _, _) => {
                        h465
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h466, _, _, _, _, _, _, _, _, _) => {
                        h466
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h467, _, _, _, _, _) => {
                        h467
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h468, _, _, _, _, _, _, _) => {
                        h468
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h469, _, _, _, _, _, _, _, _) => {
                    h469
                }
            }, match v1.clone() {
                Adt1::C0(_, h470, _, _, _, _, _, _, _, _, _, _) => {
                    h470
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h471, _, _, _, _, _, _, _) => {
                        h471
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h472, _, _, _, _, _, _, _, _) => {
                        h472
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h473, _, _, _, _) => {
                        h473
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h474, _, _, _, _, _, _) => {
                        h474
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h475, _, _, _, _, _, _, _) => {
                        h475
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h476, _, _, _, _, _, _, _, _, _) => {
                        h476
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h477, _, _, _, _, _, _) => {
                        h477
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h478, _, _, _) => {
                        h478
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f38(v6.clone(), match v5.clone() {
                Adt2::C0(_, h479, _, _, _, _, _, _, _, _, _) => {
                    h479
                }
            })?;
            let v10: u64 = f38(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h480, _, _, _, _, _, _, _, _) => {
                    h480
                }
            })?;
            let v11: u64 = if v0 {
                f38(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h481, _, _, _, _, _, _, _) => {
                        h481
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h482, _, _) => {
                        h482
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h483, _, _, _, _, _, _, _) => {
                        h483
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h484, _) => {
                        h484
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h485, _, _, _, _, _, _) => {
                        h485
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h486) => {
                        h486
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h487, _, _, _, _, _, _) => {
                        h487
                    }
                }
            };
            let v15: u64 = {
                let a492 = {
                    let a488 = v9;
                    let a489 = v10;
                    nat_mul(a488, a489)?
                };
                let a493 = {
                    let a490 = v11;
                    let a491 = v13;
                    nat_mul(a490, a491)?
                };
                nat_mul(a492, a493)?
            };
            let v16: u64 = {
                let a501 = {
                    let a496 = match v6.clone() {
                        Adt6::C0(h494, _, _, _) => {
                            h494
                        }
                    };
                    let a497 = match v7.clone() {
                        Adt6::C0(h495, _, _, _) => {
                            h495
                        }
                    };
                    nat_mul(a496, a497)?
                };
                let a502 = {
                    let a499 = match v8.clone() {
                        Adt6::C0(h498, _, _, _) => {
                            h498
                        }
                    };
                    let a500 = v12;
                    nat_mul(a499, a500)?
                };
                nat_mul(a501, a502)?
            };
            let v17: u64 = {
                let a503 = v15;
                let a504 = 32u64;
                if nat_lt(a503, a504) {
                    v15
                } else {
                    32u64
                }
            };
            let a506 = match v2.clone() {
                Adt2::C0(h505, _, _, _, _, _, _, _, _, _, _) => {
                    h505
                }
            };
            let a507 = 0u64;
            if if if if if if if if if if if nat_eq(a506, a507) {
                true
            } else {
                let a509 = match v2.clone() {
                    Adt2::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                        h508
                    }
                };
                let a510 = 1u64;
                nat_eq(a509, a510)
            } {
                let a512 = match v5.clone() {
                    Adt2::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                        h511
                    }
                };
                let a513 = 0u64;
                nat_eq(a512, a513)
            } else {
                false
            } {
                let a515 = match v4.clone() {
                    Adt2::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
                        h514
                    }
                };
                let a516 = 0u64;
                nat_eq(a515, a516)
            } else {
                false
            } {
                let a518 = match v4.clone() {
                    Adt2::C0(_, h517, _, _, _, _, _, _, _, _, _) => {
                        h517
                    }
                };
                let a519 = v9;
                nat_eq(a518, a519)
            } else {
                false
            } {
                let a521 = match v4.clone() {
                    Adt2::C0(_, _, h520, _, _, _, _, _, _, _, _) => {
                        h520
                    }
                };
                let a522 = v10;
                nat_eq(a521, a522)
            } else {
                false
            } {
                if v0 {
                    let a523 = 0u64;
                    let a524 = v12;
                    if if if if if if if nat_lt(a523, a524) {
                        let a525 = v12;
                        let a526 = 2147483648u64;
                        nat_lt(a525, a526)
                    } else {
                        false
                    } {
                        let a527 = v14;
                        let a528 = 2147483648u64;
                        nat_lt(a527, a528)
                    } else {
                        false
                    } {
                        let a529 = v13;
                        let a530 = 2147483648u64;
                        nat_lt(a529, a530)
                    } else {
                        false
                    } {
                        let a534 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h531, _, _, _, _, _, _) => {
                                h531
                            }
                        };
                        let a535 = {
                            let a532 = v12;
                            let a533 = v14;
                            nat_mul(a532, a533)?
                        };
                        nat_eq(a534, a535)
                    } else {
                        false
                    } {
                        let a539 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h536, _, _, _, _, _, _) => {
                                h536
                            }
                        };
                        let a540 = {
                            let a537 = v12;
                            let a538 = v13;
                            nat_mul(a537, a538)?
                        };
                        nat_eq(a539, a540)
                    } else {
                        false
                    } {
                        let a542 = match v4.clone() {
                            Adt2::C0(_, _, _, h541, _, _, _, _, _, _, _) => {
                                h541
                            }
                        };
                        let a543 = v11;
                        nat_eq(a542, a543)
                    } else {
                        false
                    } {
                        let a547 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h544, _, _, _, _, _, _) => {
                                h544
                            }
                        };
                        let a548 = {
                            let a545 = v14;
                            let a546 = v13;
                            nat_mul(a545, a546)?
                        };
                        nat_eq(a547, a548)
                    } else {
                        false
                    }
                } else {
                    let a550 = match v2.clone() {
                        Adt2::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                            h549
                        }
                    };
                    let a551 = v12;
                    if if if nat_eq(a550, a551) {
                        let a553 = match v4.clone() {
                            Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                                h552
                            }
                        };
                        let a554 = v14;
                        nat_eq(a553, a554)
                    } else {
                        false
                    } {
                        let a556 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h555, _, _, _, _, _, _) => {
                                h555
                            }
                        };
                        let a557 = v13;
                        nat_eq(a556, a557)
                    } else {
                        false
                    } {
                        let a558 = 0u64;
                        let a559 = v16;
                        nat_lt(a558, a559)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a560 = 0u64;
                let a561 = v14;
                nat_lt(a560, a561)
            } else {
                false
            } {
                let a562 = 0u64;
                let a563 = v15;
                nat_lt(a562, a563)
            } else {
                false
            } {
                f29(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f29(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a564 = v16;
                    let a565 = v14;
                    nat_mul(a564, a565)?
                }, v17, {
                    let a566 = v17;
                    let a567 = v16;
                    nat_mul(a566, a567)?
                }, {
                    let a568 = v17;
                    let a569 = v14;
                    nat_mul(a568, a569)?
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

pub fn f8(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m570 = v2;
    match m570 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m571 = v3;
            match m571 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a573 = match v1.clone() {
                        Adt2::C0(h572, _, _, _, _, _, _, _, _, _, _) => {
                            h572
                        }
                    };
                    let a574 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a573, a574) {
                        let a576 = match v5.clone() {
                            Adt2::C0(h575, _, _, _, _, _, _, _, _, _, _) => {
                                h575
                            }
                        };
                        let a577 = 0u64;
                        nat_eq(a576, a577)
                    } else {
                        false
                    } {
                        let a579 = match v6.clone() {
                            Adt2::C0(h578, _, _, _, _, _, _, _, _, _, _) => {
                                h578
                            }
                        };
                        let a580 = 0u64;
                        nat_eq(a579, a580)
                    } else {
                        false
                    } {
                        let a582 = match v4.clone() {
                            Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                                h581
                            }
                        };
                        let a583 = 0u64;
                        nat_eq(a582, a583)
                    } else {
                        false
                    } {
                        f20(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f20(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f37(v1.clone())?
                    } else {
                        false
                    } {
                        f37(v5.clone())?
                    } else {
                        false
                    } {
                        f37(v6)?
                    } else {
                        false
                    } {
                        f37(v4.clone())?
                    } else {
                        false
                    } {
                        let a589 = match v1.clone() {
                            Adt2::C0(_, h584, _, _, _, _, _, _, _, _, _) => {
                                h584
                            }
                        };
                        let a590 = {
                            let a587 = match v5.clone() {
                                Adt2::C0(_, h585, _, _, _, _, _, _, _, _, _) => {
                                    h585
                                }
                            };
                            let a588 = match v5.clone() {
                                Adt2::C0(_, h586, _, _, _, _, _, _, _, _, _) => {
                                    h586
                                }
                            };
                            nat_mul(a587, a588)?
                        };
                        nat_eq(a589, a590)
                    } else {
                        false
                    } {
                        let a596 = match v1.clone() {
                            Adt2::C0(_, _, h591, _, _, _, _, _, _, _, _) => {
                                h591
                            }
                        };
                        let a597 = {
                            let a594 = match v5.clone() {
                                Adt2::C0(_, _, h592, _, _, _, _, _, _, _, _) => {
                                    h592
                                }
                            };
                            let a595 = match v5.clone() {
                                Adt2::C0(_, _, _, h593, _, _, _, _, _, _, _) => {
                                    h593
                                }
                            };
                            nat_mul(a594, a595)?
                        };
                        nat_eq(a596, a597)
                    } else {
                        false
                    } {
                        let a600 = match v1.clone() {
                            Adt2::C0(_, _, _, h598, _, _, _, _, _, _, _) => {
                                h598
                            }
                        };
                        let a601 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
                                h599
                            }
                        };
                        nat_eq(a600, a601)
                    } else {
                        false
                    } {
                        let a603 = match v0.clone() {
                            Adt1::C0(h602, _, _, _, _, _, _, _, _, _, _, _) => {
                                h602
                            }
                        };
                        let a604 = 4294967296u64;
                        nat_lt(a603, a604)
                    } else {
                        false
                    } {
                        let a606 = match v0.clone() {
                            Adt1::C0(h605, _, _, _, _, _, _, _, _, _, _, _) => {
                                h605
                            }
                        };
                        let a607 = 0u64;
                        if nat_eq(a606, a607) {
                            if f29(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a610 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h608) => {
                                        h608
                                    }
                                };
                                let a611 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h609) => {
                                        h609
                                    }
                                };
                                nat_eq(a610, a611)
                            }
                        } else {
                            let a614 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h612) => {
                                    h612
                                }
                            };
                            let a615 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h613) => {
                                    h613
                                }
                            };
                            nat_eq(a614, a615)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a617 = match v4.clone() {
                                Adt2::C0(_, h616, _, _, _, _, _, _, _, _, _) => {
                                    h616
                                }
                            };
                            let a618 = 0u64;
                            if if if nat_eq(a617, a618) {
                                true
                            } else {
                                let a620 = match v4.clone() {
                                    Adt2::C0(_, _, h619, _, _, _, _, _, _, _, _) => {
                                        h619
                                    }
                                };
                                let a621 = 0u64;
                                nat_eq(a620, a621)
                            } {
                                true
                            } else {
                                let a623 = match v4.clone() {
                                    Adt2::C0(_, _, _, h622, _, _, _, _, _, _, _) => {
                                        h622
                                    }
                                };
                                let a624 = 0u64;
                                nat_eq(a623, a624)
                            } {
                                true
                            } else {
                                let a626 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h625, _, _, _, _, _, _) => {
                                        h625
                                    }
                                };
                                let a627 = 0u64;
                                nat_eq(a626, a627)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h628, _, _, _, _, _, _, _, _, _) => {
                                h628
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a630 = match v0 {
                                Adt1::C0(h629, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h629
                                }
                            };
                            let a631 = 0u64;
                            if nat_eq(a630, a631) {
                                f22(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f39(v8, match v5.clone() {
                                Adt2::C0(_, _, h632, _, _, _, _, _, _, _, _) => {
                                    h632
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h633, _, _, _, _, _, _, _) => {
                                    h633
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h634, _, _, _, _, _, _) => {
                                    h634
                                }
                            })?
                        };
                        Ok(Adt10::C4(f40(v9, v8, v10)?, 4u64, f24(v1, 1u64), f24(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f9(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f41(v2.clone());
    let v6: Adt14 = f41(v4.clone());
    let v8: Adt15 = {
        let m635 = v3;
        match m635 {
            None => {
                f42(v0, match v1.clone() {
                    Adt1::C0(h636, _, _, _, _, _, _, _, _, _, _, _) => {
                        h636
                    }
                }, match v1 {
                    Adt1::C0(_, h637, _, _, _, _, _, _, _, _, _, _) => {
                        h637
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f42(v0, match v1.clone() {
                    Adt1::C0(h638, _, _, _, _, _, _, _, _, _, _, _) => {
                        h638
                    }
                }, match v1 {
                    Adt1::C0(_, h639, _, _, _, _, _, _, _, _, _, _) => {
                        h639
                    }
                }, true, v5, f41(v7), v6)?
            }
        }
    };
    let m640 = v8;
    match m640 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f24(v2, 1u64), f24(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f10(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h641, _, _, _, _, _, _, _, _, _, _) => {
            h641
        }
    };
    let v4: u64 = f43(v3);
    let a643 = match v1.clone() {
        Adt2::C0(h642, _, _, _, _, _, _, _, _, _, _) => {
            h642
        }
    };
    let a644 = 0u64;
    if if if if if if if if nat_eq(a643, a644) {
        true
    } else {
        let a646 = match v1.clone() {
            Adt2::C0(h645, _, _, _, _, _, _, _, _, _, _) => {
                h645
            }
        };
        let a647 = 30u64;
        nat_eq(a646, a647)
    } {
        let a650 = match v1.clone() {
            Adt2::C0(h648, _, _, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a651 = match v2.clone() {
            Adt2::C0(h649, _, _, _, _, _, _, _, _, _, _) => {
                h649
            }
        };
        nat_eq(a650, a651)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f44(match v0.clone() {
            Adt1::C0(h652, _, _, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        })
    } else {
        false
    } {
        f45(v1.clone(), v4)
    } else {
        false
    } {
        f45(v2.clone(), v4)
    } else {
        false
    } {
        if f29(v1.clone(), v2.clone())? {
            true
        } else {
            f46(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f47(match v1.clone() {
            Adt2::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
                h653
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h654, _, _, _, _, _, _, _, _) => {
                h654
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h655, _, _, _, _, _, _, _) => {
                h655
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h656, _, _, _, _, _, _) => {
                h656
            }
        }) {
            0u64
        } else {
            let a662 = match v1.clone() {
                Adt2::C0(_, _, h657, _, _, _, _, _, _, _, _) => {
                    h657
                }
            };
            let a663 = {
                let a660 = match v1.clone() {
                    Adt2::C0(_, _, _, h658, _, _, _, _, _, _, _) => {
                        h658
                    }
                };
                let a661 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h659, _, _, _, _, _, _) => {
                        h659
                    }
                };
                nat_mul(a660, a661)?
            };
            nat_mul(a662, a663)?
        };
        let v6: u64 = {
            let a664 = v5;
            let a665 = 0u64;
            if if nat_eq(a664, a665) {
                true
            } else {
                f48(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a667 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h666, _) => {
                        h666
                    }
                };
                let a668 = v4;
                let a669 = 0u64;
                nat_quot(a667, a668, a669)
            }
        };
        Ok(Adt10::C4(f49(v3, match v1.clone() {
            Adt2::C0(_, h670, _, _, _, _, _, _, _, _, _) => {
                h670
            }
        }, v5, v6)?, v4, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h671, _, _, _, _, _, _, _, _, _, _, _) => {
                h671
            }
        }, v5, f50(match v1 {
            Adt2::C0(_, h672, _, _, _, _, _, _, _, _, _) => {
                h672
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f11(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a674 = match v1.clone() {
        Adt2::C0(h673, _, _, _, _, _, _, _, _, _, _) => {
            h673
        }
    };
    let a675 = 0u64;
    if if if if if nat_eq(a674, a675) {
        let a677 = match v2.clone() {
            Adt2::C0(h676, _, _, _, _, _, _, _, _, _, _) => {
                h676
            }
        };
        let a678 = 0u64;
        nat_eq(a677, a678)
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        f51(match v0.clone() {
            Adt1::C0(h679, _, _, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        })
    } else {
        false
    } {
        if f29(v1.clone(), v2.clone())? {
            true
        } else {
            f46(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f24(v1, 1u64);
        let v4: u64 = {
            let a685 = match v3.clone() {
                Adt3::C0(_, h680, _, _, _, _, _, _) => {
                    h680
                }
            };
            let a686 = {
                let a683 = match v3.clone() {
                    Adt3::C0(_, _, h681, _, _, _, _, _) => {
                        h681
                    }
                };
                let a684 = match v3.clone() {
                    Adt3::C0(_, _, _, h682, _, _, _, _) => {
                        h682
                    }
                };
                nat_mul(a683, a684)?
            };
            nat_mul(a685, a686)?
        };
        Ok(Adt10::C4(f52(match v3.clone() {
            Adt3::C0(h687, _, _, _, _, _, _, _) => {
                h687
            }
        }, v4)?, 4u64, v3, f24(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h688, _, _, _, _, _, _, _, _, _, _, _) => {
                h688
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f12(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m689 = v1;
    match m689 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m690 = v2;
            match m690 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a692 = match v0.clone() {
                        Adt2::C0(h691, _, _, _, _, _, _, _, _, _, _) => {
                            h691
                        }
                    };
                    let a693 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a692, a693) {
                        let a695 = match v4.clone() {
                            Adt2::C0(h694, _, _, _, _, _, _, _, _, _, _) => {
                                h694
                            }
                        };
                        let a696 = 0u64;
                        nat_eq(a695, a696)
                    } else {
                        false
                    } {
                        let a698 = match v5.clone() {
                            Adt2::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
                                h697
                            }
                        };
                        let a699 = 26u64;
                        nat_eq(a698, a699)
                    } else {
                        false
                    } {
                        let a701 = match v3.clone() {
                            Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                                h700
                            }
                        };
                        let a702 = 0u64;
                        nat_eq(a701, a702)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a705 = match v0.clone() {
                            Adt2::C0(_, h703, _, _, _, _, _, _, _, _, _) => {
                                h703
                            }
                        };
                        let a706 = match v4.clone() {
                            Adt2::C0(_, h704, _, _, _, _, _, _, _, _, _) => {
                                h704
                            }
                        };
                        nat_eq(a705, a706)
                    } else {
                        false
                    } {
                        let a709 = match v0.clone() {
                            Adt2::C0(_, _, h707, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        };
                        let a710 = match v5.clone() {
                            Adt2::C0(_, h708, _, _, _, _, _, _, _, _, _) => {
                                h708
                            }
                        };
                        nat_eq(a709, a710)
                    } else {
                        false
                    } {
                        let a713 = match v0.clone() {
                            Adt2::C0(_, _, _, h711, _, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        let a714 = match v5.clone() {
                            Adt2::C0(_, _, h712, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        nat_eq(a713, a714)
                    } else {
                        false
                    } {
                        let a716 = 0u64;
                        let a717 = match v5.clone() {
                            Adt2::C0(_, _, _, h715, _, _, _, _, _, _, _) => {
                                h715
                            }
                        };
                        nat_lt(a716, a717)
                    } else {
                        false
                    } {
                        let a719 = 0u64;
                        let a720 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h718, _, _, _, _, _, _) => {
                                h718
                            }
                        };
                        nat_lt(a719, a720)
                    } else {
                        false
                    } {
                        let a722 = 4u64;
                        let a723 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h721, _, _, _, _, _) => {
                                h721
                            }
                        };
                        nat_le(a722, a723)
                    } else {
                        false
                    } {
                        let a725 = 4u64;
                        let a726 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h724, _, _, _, _, _) => {
                                h724
                            }
                        };
                        nat_le(a725, a726)
                    } else {
                        false
                    } {
                        let a728 = 4u64;
                        let a729 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h727, _, _, _, _, _) => {
                                h727
                            }
                        };
                        nat_le(a728, a729)
                    } else {
                        false
                    } {
                        let a731 = 4u64;
                        let a732 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h730, _, _, _, _, _) => {
                                h730
                            }
                        };
                        nat_le(a731, a732)
                    } else {
                        false
                    } {
                        f29(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f29(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f29(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f53(match v0.clone() {
                            Adt2::C0(_, h733, _, _, _, _, _, _, _, _, _) => {
                                h733
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h734, _, _, _, _, _, _, _, _) => {
                                h734
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h735, _, _, _, _, _, _, _) => {
                                h735
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h736, _, _, _, _, _, _) => {
                                h736
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

pub fn f13(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a737 = v0;
    let a738 = 28u64;
    if if if nat_eq(a737, a738) {
        true
    } else {
        let a739 = v0;
        let a740 = 29u64;
        nat_eq(a739, a740)
    } {
        let a742 = 0u64;
        let a743 = match v1.clone() {
            Adt1::C0(h741, _, _, _, _, _, _, _, _, _, _, _) => {
                h741
            }
        };
        if nat_lt(a742, a743) {
            true
        } else {
            let a745 = 0u64;
            let a746 = match v1.clone() {
                Adt1::C0(_, _, _, h744, _, _, _, _, _, _, _, _) => {
                    h744
                }
            };
            nat_lt(a745, a746)
        }
    } else {
        false
    } {
        let m750 = f54(match v2.clone() {
            Adt2::C0(h747, _, _, _, _, _, _, _, _, _, _) => {
                h747
            }
        }, match v1.clone() {
            Adt1::C0(h748, _, _, _, _, _, _, _, _, _, _, _) => {
                h748
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h749, _, _, _, _, _, _, _, _) => {
                h749
            }
        });
        match m750 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f55(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f55(v0, v1, v2, v3, v4)
    }
}

pub fn f14(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h751, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h751
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h752, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h752
        }
    };
    let v6: u64 = {
        let a753 = v3;
        let a754 = v5;
        let a755 = 0u64;
        nat_rem(a753, a754, a755)
    };
    let a756 = v1;
    let a757 = 0u64;
    if nat_eq(a756, a757) {
        let a759 = v3;
        let a760 = match v0.clone() {
            Adt8::C0(_, _, _, h758, _, _, _, _, _, _, _, _, _, _, _) => {
                h758
            }
        };
        if nat_lt(a759, a760) {
            Ok(Adt9::C8(Some(f56(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h761, _, _, _, _, _, _) => {
                    h761
                }
            }, {
                let a765 = v6;
                let a766 = {
                    let m764 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h763, _, _, _, _, _, _) => {
                            h763
                        }
                    };
                    match m764 {
                        Adt3::C0(h762, _, _, _, _, _, _, _) => {
                            h762
                        }
                    }
                };
                let a767 = 0u64;
                nat_rem(a765, a766, a767)
            }, {
                let a777 = {
                    let a771 = v6;
                    let a772 = {
                        let m770 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h769, _, _, _, _, _, _) => {
                                h769
                            }
                        };
                        match m770 {
                            Adt3::C0(h768, _, _, _, _, _, _, _) => {
                                h768
                            }
                        }
                    };
                    let a773 = 0u64;
                    nat_quot(a771, a772, a773)
                };
                let a778 = {
                    let m776 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h775, _, _, _, _, _, _) => {
                            h775
                        }
                    };
                    match m776 {
                        Adt3::C0(_, h774, _, _, _, _, _, _) => {
                            h774
                        }
                    }
                };
                let a779 = 0u64;
                nat_rem(a777, a778, a779)
            }, {
                let a789 = {
                    let a783 = v6;
                    let a784 = {
                        let m782 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h781, _, _, _, _, _, _) => {
                                h781
                            }
                        };
                        match m782 {
                            Adt3::C0(h780, _, _, _, _, _, _, _) => {
                                h780
                            }
                        }
                    };
                    let a785 = 0u64;
                    nat_quot(a783, a784, a785)
                };
                let a790 = {
                    let m788 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h787, _, _, _, _, _, _) => {
                            h787
                        }
                    };
                    match m788 {
                        Adt3::C0(_, h786, _, _, _, _, _, _) => {
                            h786
                        }
                    }
                };
                let a791 = 0u64;
                nat_quot(a789, a790, a791)
            }, {
                let a792 = v3;
                let a793 = v5;
                let a794 = 0u64;
                nat_quot(a792, a793, a794)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a800 = v2;
            let a801 = {
                let a797 = v3;
                let a798 = {
                    let a795 = v1;
                    let a796 = 1u64;
                    if nat_eq(a795, a796) {
                        v4
                    } else {
                        v5
                    }
                };
                let a799 = 0u64;
                nat_quot(a797, a798, a799)
            };
            nat_add(a800, a801)?
        };
        let a803 = v7;
        let a804 = match v0.clone() {
            Adt8::C0(h802, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h802
            }
        };
        if nat_lt(a803, a804) {
            let v8: u64 = {
                let a808 = v7;
                let a809 = {
                    let m807 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h806, _, _, _, _, _) => {
                            h806
                        }
                    };
                    match m807 {
                        Adt3::C0(h805, _, _, _, _, _, _, _) => {
                            h805
                        }
                    }
                };
                let a810 = 0u64;
                nat_rem(a808, a809, a810)
            };
            let v9: u64 = {
                let a814 = v7;
                let a815 = {
                    let m813 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h812, _, _, _, _, _) => {
                            h812
                        }
                    };
                    match m813 {
                        Adt3::C0(h811, _, _, _, _, _, _, _) => {
                            h811
                        }
                    }
                };
                let a816 = 0u64;
                nat_quot(a814, a815, a816)
            };
            let v10: u64 = {
                let a820 = v9;
                let a821 = {
                    let m819 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h818, _, _, _, _, _) => {
                            h818
                        }
                    };
                    match m819 {
                        Adt3::C0(_, h817, _, _, _, _, _, _) => {
                            h817
                        }
                    }
                };
                let a822 = 0u64;
                nat_rem(a820, a821, a822)
            };
            let v11: u64 = {
                let a826 = v9;
                let a827 = {
                    let m825 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h824, _, _, _, _, _) => {
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
            };
            let a829 = v1;
            let a830 = 1u64;
            if nat_eq(a829, a830) {
                Ok(Adt9::C8(Some(f56(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h831, _, _, _, _, _) => {
                        h831
                    }
                }, v8, v10, {
                    let a832 = v3;
                    let a833 = v4;
                    let a834 = 0u64;
                    nat_rem(a832, a833, a834)
                }, v11)?)))
            } else {
                let a835 = v1;
                let a836 = 2u64;
                if nat_eq(a835, a836) {
                    Ok(Adt9::C8(Some({
                        let a882 = f56(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h837, _, _, _, _) => {
                                h837
                            }
                        }, {
                            let a849 = {
                                let a841 = v8;
                                let a842 = {
                                    let m840 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h839, _, _, _) => {
                                            h839
                                        }
                                    };
                                    match m840 {
                                        Adt6::C0(_, h838, _, _) => {
                                            h838
                                        }
                                    }
                                };
                                nat_mul(a841, a842)?
                            };
                            let a850 = {
                                let a846 = v6;
                                let a847 = {
                                    let m845 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h844, _, _, _, _, _, _) => {
                                            h844
                                        }
                                    };
                                    match m845 {
                                        Adt3::C0(h843, _, _, _, _, _, _, _) => {
                                            h843
                                        }
                                    }
                                };
                                let a848 = 0u64;
                                nat_rem(a846, a847, a848)
                            };
                            nat_add(a849, a850)?
                        }, {
                            let a868 = {
                                let a854 = v10;
                                let a855 = {
                                    let m853 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h852, _, _, _) => {
                                            h852
                                        }
                                    };
                                    match m853 {
                                        Adt6::C0(_, h851, _, _) => {
                                            h851
                                        }
                                    }
                                };
                                nat_mul(a854, a855)?
                            };
                            let a869 = {
                                let a865 = {
                                    let a859 = v6;
                                    let a860 = {
                                        let m858 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h857, _, _, _, _, _, _) => {
                                                h857
                                            }
                                        };
                                        match m858 {
                                            Adt3::C0(h856, _, _, _, _, _, _, _) => {
                                                h856
                                            }
                                        }
                                    };
                                    let a861 = 0u64;
                                    nat_quot(a859, a860, a861)
                                };
                                let a866 = {
                                    let m864 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h863, _, _, _, _, _, _) => {
                                            h863
                                        }
                                    };
                                    match m864 {
                                        Adt3::C0(_, h862, _, _, _, _, _, _) => {
                                            h862
                                        }
                                    }
                                };
                                let a867 = 0u64;
                                nat_rem(a865, a866, a867)
                            };
                            nat_add(a868, a869)?
                        }, {
                            let a879 = {
                                let a873 = v6;
                                let a874 = {
                                    let m872 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h871, _, _, _, _, _, _) => {
                                            h871
                                        }
                                    };
                                    match m872 {
                                        Adt3::C0(h870, _, _, _, _, _, _, _) => {
                                            h870
                                        }
                                    }
                                };
                                let a875 = 0u64;
                                nat_quot(a873, a874, a875)
                            };
                            let a880 = {
                                let m878 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h877, _, _, _, _, _, _) => {
                                        h877
                                    }
                                };
                                match m878 {
                                    Adt3::C0(_, h876, _, _, _, _, _, _) => {
                                        h876
                                    }
                                }
                            };
                            let a881 = 0u64;
                            nat_quot(a879, a880, a881)
                        }, v11)?;
                        let a883 = 4u64;
                        let a884 = 0u64;
                        nat_quot(a882, a883, a884)
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

pub fn f15(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a885 = v1;
    let a886 = 0u64;
    if nat_eq(a885, a886) {
        let a888 = v3;
        let a889 = match v0.clone() {
            Adt8::C0(_, _, _, h887, _, _, _, _, _, _, _, _, _, _, _) => {
                h887
            }
        };
        if nat_lt(a888, a889) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h890, _, _, _, _, _, _) => {
                    h890
                }
            };
            let v5: u64 = {
                let a892 = v3;
                let a893 = match v0.clone() {
                    Adt8::C0(_, _, h891, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h891
                    }
                };
                let a894 = 0u64;
                nat_quot(a892, a893, a894)
            };
            Ok(Adt9::C8(Some(f56(v4.clone(), f57(v5, v4.clone(), 0u64), f57(v5, v4.clone(), 1u64), f57(v5, v4, 2u64), f58(v0.clone(), v5, {
                let a896 = v3;
                let a897 = match v0 {
                    Adt8::C0(_, _, h895, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h895
                    }
                };
                let a898 = 0u64;
                nat_rem(a896, a897, a898)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h899, _, _, _, _) => {
                h899
            }
        };
        let v7: u64 = {
            let a900 = v1;
            let a901 = 1u64;
            if nat_eq(a900, a901) {
                match v0.clone() {
                    Adt8::C0(_, h902, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h902
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h903, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h903
                    }
                }
            }
        };
        let v8: u64 = {
            let a907 = v2;
            let a908 = {
                let a904 = v3;
                let a905 = v7;
                let a906 = 0u64;
                nat_quot(a904, a905, a906)
            };
            nat_add(a907, a908)?
        };
        let v9: u64 = {
            let a909 = v3;
            let a910 = v7;
            let a911 = 0u64;
            nat_rem(a909, a910, a911)
        };
        let v10: u64 = {
            let a913 = v8;
            let a914 = match v6.clone() {
                Adt3::C0(h912, _, _, _, _, _, _, _) => {
                    h912
                }
            };
            let a915 = 0u64;
            nat_rem(a913, a914, a915)
        };
        let v11: u64 = {
            let a917 = v8;
            let a918 = match v6.clone() {
                Adt3::C0(h916, _, _, _, _, _, _, _) => {
                    h916
                }
            };
            let a919 = 0u64;
            nat_quot(a917, a918, a919)
        };
        let v12: u64 = {
            let a921 = v11;
            let a922 = match v6.clone() {
                Adt3::C0(_, h920, _, _, _, _, _, _) => {
                    h920
                }
            };
            let a923 = 0u64;
            nat_rem(a921, a922, a923)
        };
        let v13: u64 = {
            let a925 = v11;
            let a926 = match v6.clone() {
                Adt3::C0(_, h924, _, _, _, _, _, _) => {
                    h924
                }
            };
            let a927 = 0u64;
            nat_quot(a925, a926, a927)
        };
        let a929 = v8;
        let a930 = match v0.clone() {
            Adt8::C0(h928, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h928
            }
        };
        if nat_lt(a929, a930) {
            let a931 = v1;
            let a932 = 1u64;
            if nat_eq(a931, a932) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h933, _, _, _, _, _, _) => {
                        h933
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h934, _, _, _, _, _) => {
                        h934
                    }
                };
                let m937 = f59(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h935, _, _, _) => {
                        h935
                    }
                }, match v15.clone() {
                    Adt3::C0(h936, _, _, _, _, _, _, _) => {
                        h936
                    }
                }, v10, f57(v9, v14.clone(), 0u64))?;
                match m937 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m940 = f59(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h938, _, _) => {
                                h938
                            }
                        }, match v15 {
                            Adt3::C0(_, h939, _, _, _, _, _, _) => {
                                h939
                            }
                        }, v12, f57(v9, v14, 1u64))?;
                        match m940 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f60(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a941 = v1;
                let a942 = 2u64;
                if nat_eq(a941, a942) {
                    Ok(Adt9::C8(Some({
                        let a944 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h943) => {
                                h943
                            }
                        };
                        let a945 = 0u64;
                        if nat_eq(a944, a945) {
                            f56(v6, v10, v12, v9, v13)?
                        } else {
                            f56(v6.clone(), v10, v12, {
                                let a947 = v13;
                                let a948 = match v6.clone() {
                                    Adt3::C0(_, _, h946, _, _, _, _, _) => {
                                        h946
                                    }
                                };
                                let a949 = 0u64;
                                nat_rem(a947, a948, a949)
                            }, {
                                let a957 = {
                                    let a955 = {
                                        let a951 = v13;
                                        let a952 = match v6 {
                                            Adt3::C0(_, _, h950, _, _, _, _, _) => {
                                                h950
                                            }
                                        };
                                        let a953 = 0u64;
                                        nat_quot(a951, a952, a953)
                                    };
                                    let a956 = match v0 {
                                        Adt8::C0(_, _, h954, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h954
                                        }
                                    };
                                    nat_mul(a955, a956)?
                                };
                                let a958 = v9;
                                nat_add(a957, a958)?
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

pub fn f16(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f17(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f17(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f61(v3.clone(), {
        let m959 = v3.clone();
        match m959 {
            Adt4::C0 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f73(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C19(v72, v73) => {
                f74(v72, v0, v1, v2, v73, v4)?
            }
            Adt4::C20(v74, v75) => {
                f75(v74, v0, v1, v2, v75, v4)?
            }
            Adt4::C21 => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C22(v76, v77) => {
                f76(v0, v1, v2, v76, v77, v4)?
            }
            Adt4::C23(v91, v92, v93, v94) => {
                f83(v0, v94, v91, v92, v93, v4)?
            }
            Adt4::C24(v90) => {
                if v5 {
                    f81(Adt16::C0(v90), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f62(v0, v1, v2, v3)?
            }
            Adt4::C26(v86, v87, v88, v89) => {
                let a961 = v86;
                let a962 = 64u64;
                if if if nat_eq(a961, a962) {
                    true
                } else {
                    let a963 = v86;
                    let a964 = 65u64;
                    nat_eq(a963, a964)
                } {
                    true
                } else {
                    let a965 = v86;
                    let a966 = 66u64;
                    nat_eq(a965, a966)
                } {
                    if v5 {
                        f81(Adt16::C1(v86, v87, v88, v89), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a967 = v86;
                    let a968 = 0u64;
                    if if nat_eq(a967, a968) {
                        true
                    } else {
                        let a969 = v86;
                        let a970 = 30u64;
                        nat_eq(a969, a970)
                    } {
                        f82(v0, v1, v2, v86, v87, v88, v89, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v85) => {
                f80(v0, v1, v2, v85)?
            }
            Adt4::C28(v81) => {
                f77(v0, v1, v2, v81, v4)?
            }
            Adt4::C29(v82) => {
                f78(v0, v1, v2, v82)?
            }
            Adt4::C30(v83, _) => {
                f79(v0, v1, v2, v83)?
            }
            Adt4::C31(v33, v34) => {
                f63(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f64(v0, match v1 {
                    Adt3::C0(h960, _, _, _, _, _, _, _) => {
                        h960
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f65(v0, v1, v36, v2)?
            }
            Adt4::C34(v52, v53, v54, v55) => {
                f72(v0, v1, v52, v2, v53, v54, v55)?
            }
            Adt4::C35(v51) => {
                f71(v0, v51, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f66(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f67(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f68(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f69(v0, v1, v47)?
            }
            Adt4::C40(v48, v49, v50) => {
                f70(v0, v1, v2, v48, v49, v50, v4)?
            }
        }
    }))
}

pub fn f18(v0: Adt4) -> R<Adt9> {
    let m971 = v0;
    match m971 {
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
        Adt4::C25(v69, _, v71) => {
            Ok(Adt9::C6({
                let a977 = {
                    let a973 = 0u64;
                    let a974 = v69;
                    if nat_lt(a973, a974) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a978 = {
                    let a975 = 0u64;
                    let a976 = v71;
                    if nat_lt(a975, a976) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a977, a978)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v77, _, _, v80) => {
            Ok(Adt9::C6({
                let a979 = v77;
                let a980 = 66u64;
                if nat_eq(a979, a980) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v80))
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
        Adt4::C35(v42) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v42))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m972 = v32;
                match m972 {
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
    }
}

pub fn f19(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a981 = v1;
    let a982 = 5u64;
    if if if if if nat_lt(a981, a982) {
        true
    } else {
        f84(v0, v1, v2, v3, v4)
    } {
        let a983 = v5;
        let a984 = 5u64;
        nat_lt(a983, a984)
    } else {
        false
    } {
        let a985 = v7;
        let a986 = 5u64;
        nat_lt(a985, a986)
    } else {
        false
    } {
        let a987 = v9;
        let a988 = 2u64;
        if nat_lt(a987, a988) {
            true
        } else {
            let a989 = v9;
            let a990 = 3u64;
            nat_eq(a989, a990)
        }
    } else {
        false
    } {
        Adt9::C7(f85(v0, v1, v2, v3, v4), f86(v5, v6), f86(v7, v8), f87({
            let a991 = v9;
            let a992 = 0u64;
            if nat_eq(a991, a992) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f20(v0: Adt2, v1: Adt2) -> bool {
    let a995 = match v0.clone() {
        Adt2::C0(_, h993, _, _, _, _, _, _, _, _, _) => {
            h993
        }
    };
    let a996 = match v1.clone() {
        Adt2::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
            h994
        }
    };
    if if if nat_eq(a995, a996) {
        let a999 = match v0.clone() {
            Adt2::C0(_, _, h997, _, _, _, _, _, _, _, _) => {
                h997
            }
        };
        let a1000 = match v1.clone() {
            Adt2::C0(_, _, h998, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        nat_eq(a999, a1000)
    } else {
        false
    } {
        let a1003 = match v0.clone() {
            Adt2::C0(_, _, _, h1001, _, _, _, _, _, _, _) => {
                h1001
            }
        };
        let a1004 = match v1.clone() {
            Adt2::C0(_, _, _, h1002, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        nat_eq(a1003, a1004)
    } else {
        false
    } {
        let a1007 = match v0 {
            Adt2::C0(_, _, _, _, h1005, _, _, _, _, _, _) => {
                h1005
            }
        };
        let a1008 = match v1 {
            Adt2::C0(_, _, _, _, h1006, _, _, _, _, _, _) => {
                h1006
            }
        };
        nat_eq(a1007, a1008)
    } else {
        false
    }
}

pub fn f21(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1010 = match v0.clone() {
        Adt2::C0(h1009, _, _, _, _, _, _, _, _, _, _) => {
            h1009
        }
    };
    let a1011 = 0u64;
    if if if nat_eq(a1010, a1011) {
        let a1013 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1012, _, _, _, _, _) => {
                h1012
            }
        };
        let a1014 = 4u64;
        nat_eq(a1013, a1014)
    } else {
        false
    } {
        if v2 {
            f37(v0.clone())?
        } else {
            true
        }
    } else {
        false
    } {
        let a1026 = {
            let a1023 = {
                let a1018 = {
                    let a1015 = v1;
                    let a1016 = 1u64;
                    nat_sub(a1015, a1016)
                };
                let a1019 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1017, _, _, _, _) => {
                        h1017
                    }
                };
                nat_mul(a1018, a1019)?
            };
            let a1024 = {
                let a1021 = 4u64;
                let a1022 = match v0.clone() {
                    Adt2::C0(_, h1020, _, _, _, _, _, _, _, _, _) => {
                        h1020
                    }
                };
                nat_mul(a1021, a1022)?
            };
            nat_add(a1023, a1024)?
        };
        let a1027 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1025, _) => {
                h1025
            }
        };
        Ok(nat_le(a1026, a1027))
    } else {
        Ok(false)
    }
}

pub fn f22(v0: Adt2, v1: u64) -> R<u64> {
    let a1039 = {
        let a1033 = {
            let a1029 = match v0.clone() {
                Adt2::C0(_, h1028, _, _, _, _, _, _, _, _, _) => {
                    h1028
                }
            };
            let a1030 = v1;
            let a1031 = 0u64;
            nat_quot(a1029, a1030, a1031)
        };
        let a1034 = match v0.clone() {
            Adt2::C0(_, _, h1032, _, _, _, _, _, _, _, _) => {
                h1032
            }
        };
        nat_mul(a1033, a1034)?
    };
    let a1040 = {
        let a1037 = match v0.clone() {
            Adt2::C0(_, _, _, h1035, _, _, _, _, _, _, _) => {
                h1035
            }
        };
        let a1038 = match v0 {
            Adt2::C0(_, _, _, _, h1036, _, _, _, _, _, _) => {
                h1036
            }
        };
        nat_mul(a1037, a1038)?
    };
    nat_mul(a1039, a1040)
}

pub fn f23(v0: bool, v1: u64) -> R<u64> {
    let a1043 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1044 = {
        let a1041 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1042 = v1;
        nat_mul(a1041, a1042)?
    };
    nat_add(a1043, a1044)
}

pub fn f24(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1046 = match v0.clone() {
            Adt2::C0(_, h1045, _, _, _, _, _, _, _, _, _) => {
                h1045
            }
        };
        let a1047 = v1;
        let a1048 = 0u64;
        nat_quot(a1046, a1047, a1048)
    }, match v0.clone() {
        Adt2::C0(_, _, h1049, _, _, _, _, _, _, _, _) => {
            h1049
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1050, _, _, _, _, _, _, _) => {
            h1050
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1051, _, _, _, _, _, _) => {
            h1051
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1052, _, _, _, _, _) => {
            h1052
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1053, _, _, _, _) => {
            h1053
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1054, _, _, _) => {
            h1054
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1055, _, _) => {
            h1055
        }
    })
}

pub fn f25(v0: Adt2, v1: u64) -> R<bool> {
    let a1056 = v1;
    let a1057 = 0u64;
    if nat_eq(a1056, a1057) {
        Ok(true)
    } else {
        let a1063 = {
            let a1060 = f88({
                let a1058 = v1;
                let a1059 = 1u64;
                nat_sub(a1058, a1059)
            }, f24(v0.clone(), 1u64))?;
            let a1061 = 4u64;
            nat_add(a1060, a1061)?
        };
        let a1064 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1062, _) => {
                h1062
            }
        };
        Ok(nat_le(a1063, a1064))
    }
}

pub fn f26(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1066 = match v1.clone() {
        Adt2::C0(h1065, _, _, _, _, _, _, _, _, _, _) => {
            h1065
        }
    };
    let a1067 = 0u64;
    if if nat_eq(a1066, a1067) {
        f20(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1068 = f22(v0.clone(), 1u64)?;
        let a1069 = 0u64;
        if nat_eq(a1068, a1069) {
            Ok(true)
        } else {
            let a1075 = {
                let a1072 = f88({
                    let a1070 = f22(v0.clone(), 1u64)?;
                    let a1071 = 1u64;
                    nat_sub(a1070, a1071)
                }, f24(v0, 1u64))?;
                let a1073 = 4u64;
                nat_add(a1072, a1073)?
            };
            let a1076 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1074, _) => {
                    h1074
                }
            };
            Ok(nat_le(a1075, a1076))
        }
    } else {
        Ok(false)
    }
}

pub fn f27(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        4u64
    }
}

pub fn f28(v0: bool) -> u64 {
    if v0 {
        15u64
    } else {
        3u64
    }
}

pub fn f29(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1078 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1077, _) => {
            h1077
        }
    };
    let a1079 = 0u64;
    if nat_eq(a1078, a1079) {
        Ok(true)
    } else {
        let a1081 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1080, _) => {
                h1080
            }
        };
        let a1082 = 0u64;
        if nat_eq(a1081, a1082) {
            Ok(true)
        } else {
            let a1084 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1083) => {
                    h1083
                }
            };
            let a1085 = 0u64;
            if nat_eq(a1084, a1085) {
                Ok(true)
            } else {
                let a1087 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1086) => {
                        h1086
                    }
                };
                let a1088 = 0u64;
                if nat_eq(a1087, a1088) {
                    Ok(true)
                } else {
                    let a1094 = {
                        let a1091 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1089) => {
                                h1089
                            }
                        };
                        let a1092 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1090, _) => {
                                h1090
                            }
                        };
                        nat_add(a1091, a1092)?
                    };
                    let a1095 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1093) => {
                            h1093
                        }
                    };
                    if nat_le(a1094, a1095) {
                        Ok(true)
                    } else {
                        let a1101 = {
                            let a1098 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1096) => {
                                    h1096
                                }
                            };
                            let a1099 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1097, _) => {
                                    h1097
                                }
                            };
                            nat_add(a1098, a1099)?
                        };
                        let a1102 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1100) => {
                                h1100
                            }
                        };
                        Ok(nat_le(a1101, a1102))
                    }
                }
            }
        }
    }
}

pub fn f30(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1108 = match v0.clone() {
            Adt2::C0(_, _, h1103, _, _, _, _, _, _, _, _) => {
                h1103
            }
        };
        let a1109 = {
            let a1106 = match v0.clone() {
                Adt2::C0(_, _, _, h1104, _, _, _, _, _, _, _) => {
                    h1104
                }
            };
            let a1107 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1105, _, _, _, _, _, _) => {
                    h1105
                }
            };
            nat_mul(a1106, a1107)?
        };
        nat_mul(a1108, a1109)?
    };
    let v2: u64 = f22(v0.clone(), 1u64)?;
    let a1111 = match v0.clone() {
        Adt2::C0(h1110, _, _, _, _, _, _, _, _, _, _) => {
            h1110
        }
    };
    let a1112 = 0u64;
    if if if if if if if nat_eq(a1111, a1112) {
        let a1114 = match v0.clone() {
            Adt2::C0(_, h1113, _, _, _, _, _, _, _, _, _) => {
                h1113
            }
        };
        let a1115 = 2147483647u64;
        nat_le(a1114, a1115)
    } else {
        false
    } {
        let a1116 = v1;
        let a1117 = 2147483647u64;
        nat_le(a1116, a1117)
    } else {
        false
    } {
        let a1119 = match v0.clone() {
            Adt2::C0(_, h1118, _, _, _, _, _, _, _, _, _) => {
                h1118
            }
        };
        let a1120 = 1u64;
        if nat_eq(a1119, a1120) {
            true
        } else {
            let a1122 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1121, _, _, _, _, _) => {
                    h1121
                }
            };
            let a1123 = 4u64;
            nat_eq(a1122, a1123)
        }
    } else {
        false
    } {
        let a1124 = v1;
        let a1125 = 1u64;
        if nat_le(a1124, a1125) {
            true
        } else {
            let a1130 = {
                let a1127 = match v0.clone() {
                    Adt2::C0(_, h1126, _, _, _, _, _, _, _, _, _) => {
                        h1126
                    }
                };
                let a1128 = 4u64;
                nat_mul(a1127, a1128)?
            };
            let a1131 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1129, _, _, _, _) => {
                    h1129
                }
            };
            nat_le(a1130, a1131)
        }
    } else {
        false
    } {
        let a1133 = match v0.clone() {
            Adt2::C0(_, _, _, h1132, _, _, _, _, _, _, _) => {
                h1132
            }
        };
        let a1134 = 1u64;
        if nat_eq(a1133, a1134) {
            true
        } else {
            let a1140 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1135, _, _, _) => {
                    h1135
                }
            };
            let a1141 = {
                let a1138 = match v0.clone() {
                    Adt2::C0(_, _, h1136, _, _, _, _, _, _, _, _) => {
                        h1136
                    }
                };
                let a1139 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1137, _, _, _, _) => {
                        h1137
                    }
                };
                nat_mul(a1138, a1139)?
            };
            nat_eq(a1140, a1141)
        }
    } else {
        false
    } {
        let a1143 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1142, _, _, _, _, _, _) => {
                h1142
            }
        };
        let a1144 = 1u64;
        if nat_eq(a1143, a1144) {
            true
        } else {
            let a1153 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1145, _, _) => {
                    h1145
                }
            };
            let a1154 = {
                let a1151 = match v0.clone() {
                    Adt2::C0(_, _, _, h1146, _, _, _, _, _, _, _) => {
                        h1146
                    }
                };
                let a1152 = {
                    let a1149 = match v0.clone() {
                        Adt2::C0(_, _, h1147, _, _, _, _, _, _, _, _) => {
                            h1147
                        }
                    };
                    let a1150 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1148, _, _, _, _) => {
                            h1148
                        }
                    };
                    nat_mul(a1149, a1150)?
                };
                nat_mul(a1151, a1152)?
            };
            nat_eq(a1153, a1154)
        }
    } else {
        false
    } {
        let a1155 = v2;
        let a1156 = 0u64;
        if nat_eq(a1155, a1156) {
            Ok(true)
        } else {
            let a1162 = {
                let a1159 = f88({
                    let a1157 = v2;
                    let a1158 = 1u64;
                    nat_sub(a1157, a1158)
                }, f24(v0.clone(), 1u64))?;
                let a1160 = 4u64;
                nat_add(a1159, a1160)?
            };
            let a1163 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1161, _) => {
                    h1161
                }
            };
            Ok(nat_le(a1162, a1163))
        }
    } else {
        Ok(false)
    }
}

pub fn f31(v0: u64) -> R<u64> {
    let a1170 = 148u64;
    let a1171 = {
        let a1168 = 36u64;
        let a1169 = {
            let a1166 = {
                let a1164 = 2u64;
                let a1165 = v0;
                nat_mul(a1164, a1165)?
            };
            let a1167 = 1u64;
            nat_sub(a1166, a1167)
        };
        nat_mul(a1168, a1169)?
    };
    nat_add(a1170, a1171)
}

pub fn f32(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1172 = v1;
    match m1172 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1175 = 1u64;
            let a1176 = {
                let a1173 = 2u64;
                let a1174 = v0;
                nat_mul(a1173, a1174)?
            };
            nat_add(a1175, a1176)
        }
    }
}

pub fn f33(v0: u64) -> bool {
    let a1177 = v0;
    let a1178 = 4294967296u64;
    if nat_lt(a1177, a1178) {
        let a1182 = {
            let a1179 = v0;
            let a1180 = 2147483648u64;
            let a1181 = 0u64;
            nat_rem(a1179, a1180, a1181)
        };
        let a1183 = 2139095040u64;
        nat_lt(a1182, a1183)
    } else {
        false
    }
}

pub fn f34(v0: u64) -> R<u64> {
    let a1184 = 4u64;
    let a1185 = v0;
    nat_mul(a1184, a1185)
}

pub fn f35(v0: u64) -> R<u64> {
    let a1186 = v0;
    let a1187 = 0u64;
    if nat_eq(a1186, a1187) {
        Ok(0u64)
    } else {
        let a1190 = 4u64;
        let a1191 = {
            let a1188 = 3u64;
            let a1189 = v0;
            nat_mul(a1188, a1189)?
        };
        nat_add(a1190, a1191)
    }
}

pub fn f36(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f24(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1192, _, _, _, _, _, _, _, _, _) => {
                h1192
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1193, _, _, _, _, _, _, _, _) => {
                h1193
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1194, _, _, _, _, _, _, _) => {
                h1194
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1195, _, _, _, _, _) => {
                h1195
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1196, _, _, _, _) => {
                h1196
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1197, _, _, _) => {
                h1197
            }
        })
    }
}

pub fn f37(v0: Adt2) -> R<bool> {
    let a1199 = match v0.clone() {
        Adt2::C0(_, h1198, _, _, _, _, _, _, _, _, _) => {
            h1198
        }
    };
    let a1200 = 1u64;
    if if if if nat_eq(a1199, a1200) {
        true
    } else {
        let a1202 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1201, _, _, _, _, _) => {
                h1201
            }
        };
        let a1203 = 4u64;
        nat_eq(a1202, a1203)
    } {
        let a1205 = match v0.clone() {
            Adt2::C0(_, _, h1204, _, _, _, _, _, _, _, _) => {
                h1204
            }
        };
        let a1206 = 1u64;
        if nat_eq(a1205, a1206) {
            true
        } else {
            let a1211 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1207, _, _, _, _) => {
                    h1207
                }
            };
            let a1212 = {
                let a1209 = match v0.clone() {
                    Adt2::C0(_, h1208, _, _, _, _, _, _, _, _, _) => {
                        h1208
                    }
                };
                let a1210 = 4u64;
                nat_mul(a1209, a1210)?
            };
            nat_eq(a1211, a1212)
        }
    } else {
        false
    } {
        let a1214 = match v0.clone() {
            Adt2::C0(_, _, _, h1213, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1215 = 1u64;
        if nat_eq(a1214, a1215) {
            true
        } else {
            let a1223 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1216, _, _, _) => {
                    h1216
                }
            };
            let a1224 = {
                let a1221 = {
                    let a1219 = match v0.clone() {
                        Adt2::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                            h1217
                        }
                    };
                    let a1220 = match v0.clone() {
                        Adt2::C0(_, _, h1218, _, _, _, _, _, _, _, _) => {
                            h1218
                        }
                    };
                    nat_mul(a1219, a1220)?
                };
                let a1222 = 4u64;
                nat_mul(a1221, a1222)?
            };
            nat_eq(a1223, a1224)
        }
    } else {
        false
    } {
        let a1226 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1225, _, _, _, _, _, _) => {
                h1225
            }
        };
        let a1227 = 1u64;
        if nat_eq(a1226, a1227) {
            Ok(true)
        } else {
            let a1238 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1228, _, _) => {
                    h1228
                }
            };
            let a1239 = {
                let a1236 = {
                    let a1234 = {
                        let a1231 = match v0.clone() {
                            Adt2::C0(_, h1229, _, _, _, _, _, _, _, _, _) => {
                                h1229
                            }
                        };
                        let a1232 = match v0.clone() {
                            Adt2::C0(_, _, h1230, _, _, _, _, _, _, _, _) => {
                                h1230
                            }
                        };
                        nat_mul(a1231, a1232)?
                    };
                    let a1235 = match v0 {
                        Adt2::C0(_, _, _, h1233, _, _, _, _, _, _, _) => {
                            h1233
                        }
                    };
                    nat_mul(a1234, a1235)?
                };
                let a1237 = 4u64;
                nat_mul(a1236, a1237)?
            };
            Ok(nat_eq(a1238, a1239))
        }
    } else {
        Ok(false)
    }
}

pub fn f38(v0: Adt6, v1: u64) -> R<u64> {
    let a1241 = match v0.clone() {
        Adt6::C0(h1240, _, _, _) => {
            h1240
        }
    };
    let a1242 = 0u64;
    if nat_eq(a1241, a1242) {
        f89(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1243, _, _) => {
                h1243
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1244, _) => {
                h1244
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1245) => {
                h1245
            }
        }), {
            let a1247 = v1;
            let a1248 = match v0 {
                Adt6::C0(_, _, _, h1246) => {
                    h1246
                }
            };
            nat_add(a1247, a1248)?
        })
    } else {
        f89(v0, v1)
    }
}

pub fn f39(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1253 = {
        let a1249 = v0;
        let a1250 = v1;
        nat_mul(a1249, a1250)?
    };
    let a1254 = {
        let a1251 = v2;
        let a1252 = f90(v3);
        nat_mul(a1251, a1252)?
    };
    nat_mul(a1253, a1254)
}

pub fn f40(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1257 = v0;
    let a1258 = {
        let a1255 = f91(v1)?;
        let a1256 = v2;
        nat_mul(a1255, a1256)?
    };
    nat_add(a1257, a1258)
}

pub fn f41(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1259, _, _, _, _, _, _, _, _, _, _) => {
            h1259
        }
    }, match v0.clone() {
        Adt2::C0(_, h1260, _, _, _, _, _, _, _, _, _) => {
            h1260
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1261, _, _, _, _, _, _, _, _) => {
            h1261
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1262, _, _, _, _, _, _, _) => {
            h1262
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1263, _, _, _, _, _, _) => {
            h1263
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1264, _, _, _, _, _) => {
            h1264
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1265, _, _, _, _) => {
            h1265
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1266, _, _, _) => {
            h1266
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1267, _, _) => {
            h1267
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1268, _) => {
            h1268
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1269) => {
            h1269
        }
    })
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1270 = v0;
    let a1271 = 23u64;
    if nat_eq(a1270, a1271) {
        f92(v1, v4, v6)
    } else {
        let a1272 = v0;
        let a1273 = 26u64;
        if nat_eq(a1272, a1273) {
            f93(v1, v2, v4, v6)
        } else {
            let a1274 = v0;
            let a1275 = 25u64;
            if nat_eq(a1274, a1275) {
                f94(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f43(v0: u64) -> u64 {
    let a1276 = v0;
    let a1277 = 30u64;
    if nat_eq(a1276, a1277) {
        2u64
    } else {
        4u64
    }
}

pub fn f44(v0: u64) -> bool {
    let a1278 = v0;
    let a1279 = 2139095040u64;
    if nat_le(a1278, a1279) {
        true
    } else {
        let a1280 = v0;
        let a1281 = 2147483648u64;
        nat_eq(a1280, a1281)
    }
}

pub fn f45(v0: Adt2, v1: u64) -> bool {
    let a1283 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1282, _, _, _, _, _) => {
            h1282
        }
    };
    let a1284 = v1;
    if if nat_eq(a1283, a1284) {
        let a1289 = {
            let a1286 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1285) => {
                    h1285
                }
            };
            let a1287 = v1;
            let a1288 = 0u64;
            nat_rem(a1286, a1287, a1288)
        };
        let a1290 = 0u64;
        nat_eq(a1289, a1290)
    } else {
        false
    } {
        f95(match v0.clone() {
            Adt2::C0(_, _, h1291, _, _, _, _, _, _, _, _) => {
                h1291
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1292, _, _, _, _, _, _, _) => {
                h1292
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1293, _, _, _, _, _, _) => {
                h1293
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1294, _, _, _, _) => {
                h1294
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1295, _, _, _) => {
                h1295
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1296, _, _) => {
                h1296
            }
        }, v1)
    } else {
        false
    }
}

pub fn f46(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f20(v0.clone(), v1.clone()) {
        let a1299 = match v0.clone() {
            Adt2::C0(h1297, _, _, _, _, _, _, _, _, _, _) => {
                h1297
            }
        };
        let a1300 = match v1.clone() {
            Adt2::C0(h1298, _, _, _, _, _, _, _, _, _, _) => {
                h1298
            }
        };
        nat_eq(a1299, a1300)
    } else {
        false
    } {
        let a1303 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1301) => {
                h1301
            }
        };
        let a1304 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1302) => {
                h1302
            }
        };
        nat_eq(a1303, a1304)
    } else {
        false
    } {
        let a1307 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1305, _, _, _, _, _) => {
                h1305
            }
        };
        let a1308 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1306, _, _, _, _, _) => {
                h1306
            }
        };
        nat_eq(a1307, a1308)
    } else {
        false
    } {
        let a1311 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1309, _, _, _, _) => {
                h1309
            }
        };
        let a1312 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1310, _, _, _, _) => {
                h1310
            }
        };
        nat_eq(a1311, a1312)
    } else {
        false
    } {
        let a1315 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1313, _, _, _) => {
                h1313
            }
        };
        let a1316 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1314, _, _, _) => {
                h1314
            }
        };
        nat_eq(a1315, a1316)
    } else {
        false
    } {
        let a1319 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1317, _, _) => {
                h1317
            }
        };
        let a1320 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1318, _, _) => {
                h1318
            }
        };
        nat_eq(a1319, a1320)
    } else {
        false
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1321 = v0;
    let a1322 = 0u64;
    if if if nat_eq(a1321, a1322) {
        true
    } else {
        let a1323 = v1;
        let a1324 = 0u64;
        nat_eq(a1323, a1324)
    } {
        true
    } else {
        let a1325 = v2;
        let a1326 = 0u64;
        nat_eq(a1325, a1326)
    } {
        true
    } else {
        let a1327 = v3;
        let a1328 = 0u64;
        nat_eq(a1327, a1328)
    }
}

pub fn f48(v0: Adt2, v1: u64) -> R<bool> {
    f96({
        let a1330 = match v0.clone() {
            Adt2::C0(_, h1329, _, _, _, _, _, _, _, _, _) => {
                h1329
            }
        };
        let a1331 = v1;
        nat_mul(a1330, a1331)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1332, _, _, _, _, _, _, _, _) => {
            h1332
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1333, _, _, _, _, _, _, _) => {
            h1333
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1334, _, _, _, _, _, _) => {
            h1334
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1335, _, _, _, _) => {
            h1335
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1336, _, _, _) => {
            h1336
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1337, _, _) => {
            h1337
        }
    })
}

pub fn f49(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1338 = v1;
    let a1339 = 0u64;
    if if nat_eq(a1338, a1339) {
        true
    } else {
        let a1340 = v2;
        let a1341 = 0u64;
        nat_eq(a1340, a1341)
    } {
        Ok(0u64)
    } else {
        let a1348 = {
            let a1342 = f97(v1, v2, v3)?;
            let a1343 = f98(v0, v1, v2)?;
            nat_add(a1342, a1343)?
        };
        let a1349 = {
            let a1346 = f99(v0);
            let a1347 = {
                let a1344 = v1;
                let a1345 = v2;
                nat_mul(a1344, a1345)?
            };
            nat_mul(a1346, a1347)?
        };
        nat_add(a1348, a1349)
    }
}

pub fn f50(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1350 = v0;
    let a1351 = 0u64;
    if if nat_eq(a1350, a1351) {
        true
    } else {
        let a1352 = v1;
        let a1353 = 0u64;
        nat_eq(a1352, a1353)
    } {
        Ok(3u64)
    } else {
        let a1356 = {
            let a1354 = 3u64;
            let a1355 = v1;
            nat_add(a1354, a1355)?
        };
        let a1357 = v2;
        nat_add(a1356, a1357)
    }
}

pub fn f51(v0: u64) -> bool {
    let a1358 = v0;
    let a1359 = 2139095040u64;
    if nat_le(a1358, a1359) {
        true
    } else {
        let a1360 = v0;
        let a1361 = 2147483648u64;
        nat_eq(a1360, a1361)
    }
}

pub fn f52(v0: u64, v1: u64) -> R<u64> {
    let a1362 = v0;
    let a1363 = 0u64;
    if nat_eq(a1362, a1363) {
        Ok(0u64)
    } else {
        let a1364 = v1;
        let a1365 = f100(v0)?;
        nat_mul(a1364, a1365)
    }
}

pub fn f53(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1366 = v0;
    let a1367 = 0u64;
    if if if if nat_eq(a1366, a1367) {
        true
    } else {
        let a1368 = v1;
        let a1369 = 0u64;
        nat_eq(a1368, a1369)
    } {
        true
    } else {
        let a1370 = v2;
        let a1371 = 0u64;
        nat_eq(a1370, a1371)
    } {
        true
    } else {
        let a1372 = v3;
        let a1373 = 0u64;
        nat_eq(a1372, a1373)
    } {
        Ok(0u64)
    } else {
        let a1386 = {
            let a1376 = 2u64;
            let a1377 = {
                let a1374 = v1;
                let a1375 = v2;
                nat_mul(a1374, a1375)?
            };
            nat_mul(a1376, a1377)?
        };
        let a1387 = {
            let a1384 = {
                let a1380 = {
                    let a1378 = v1;
                    let a1379 = v2;
                    nat_mul(a1378, a1379)?
                };
                let a1381 = v3;
                nat_mul(a1380, a1381)?
            };
            let a1385 = {
                let a1382 = 1u64;
                let a1383 = v0;
                nat_add(a1382, a1383)?
            };
            nat_mul(a1384, a1385)?
        };
        nat_add(a1386, a1387)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1388 = v1;
        let a1389 = 0u64;
        if if if nat_eq(a1388, a1389) {
            true
        } else {
            let a1390 = v1;
            let a1391 = 10u64;
            nat_eq(a1390, a1391)
        } {
            true
        } else {
            let a1392 = v1;
            let a1393 = 15u64;
            nat_eq(a1392, a1393)
        } {
            true
        } else {
            let a1394 = v1;
            let a1395 = 20u64;
            nat_eq(a1394, a1395)
        }
    };
    let v4: bool = {
        let a1396 = v2;
        let a1397 = 0u64;
        if if if if if nat_eq(a1396, a1397) {
            true
        } else {
            let a1398 = v2;
            let a1399 = 10u64;
            nat_eq(a1398, a1399)
        } {
            true
        } else {
            let a1400 = v2;
            let a1401 = 15u64;
            nat_eq(a1400, a1401)
        } {
            true
        } else {
            let a1402 = v2;
            let a1403 = 20u64;
            nat_eq(a1402, a1403)
        } {
            true
        } else {
            let a1404 = v2;
            let a1405 = 30u64;
            nat_eq(a1404, a1405)
        } {
            true
        } else {
            let a1406 = v2;
            let a1407 = 40u64;
            nat_eq(a1406, a1407)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1408 = v0;
            let a1409 = 30u64;
            if if nat_eq(a1408, a1409) {
                let a1410 = v2;
                let a1411 = 10u64;
                nat_eq(a1410, a1411)
            } else {
                false
            } {
                true
            } else {
                if f101(v0) {
                    let a1412 = v2;
                    let a1413 = 10u64;
                    if if nat_eq(a1412, a1413) {
                        true
                    } else {
                        let a1414 = v2;
                        let a1415 = 15u64;
                        nat_eq(a1414, a1415)
                    } {
                        true
                    } else {
                        let a1416 = v2;
                        let a1417 = 20u64;
                        nat_eq(a1416, a1417)
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

pub fn f55(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1418, _, _, _, _, _, _, _, _, _, _, _) => {
            h1418
        }
    };
    Ok(Adt9::C0({
        let a1419 = v0;
        let a1420 = 86u64;
        if nat_eq(a1419, a1420) {
            let m1421 = v3;
            match m1421 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f102(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1422 = v0;
            let a1423 = 55u64;
            if nat_eq(a1422, a1423) {
                let m1424 = v3;
                match m1424 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f103(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1425 = v0;
                let a1426 = 22u64;
                if nat_eq(a1425, a1426) {
                    let m1427 = v3;
                    match m1427 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f104(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1428 = v0;
                    let a1429 = 44u64;
                    if nat_eq(a1428, a1429) {
                        f105(v1, v2, v3, v4)?
                    } else {
                        let a1430 = v0;
                        let a1431 = 52u64;
                        if nat_eq(a1430, a1431) {
                            f106(v1, v2, v4)?
                        } else {
                            let a1432 = v0;
                            let a1433 = 73u64;
                            if nat_eq(a1432, a1433) {
                                f107(v2, v3, v4)?
                            } else {
                                let a1434 = v0;
                                let a1435 = 57u64;
                                if if nat_eq(a1434, a1435) {
                                    true
                                } else {
                                    let a1436 = v0;
                                    let a1437 = 58u64;
                                    nat_eq(a1436, a1437)
                                } {
                                    let m1438 = v3;
                                    match m1438 {
                                        None => {
                                            f108({
                                                let a1439 = v0;
                                                let a1440 = 58u64;
                                                nat_eq(a1439, a1440)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a1441 = v0;
                                    let a1442 = 49u64;
                                    if nat_eq(a1441, a1442) {
                                        f109(v1, v2, v3, v4)?
                                    } else {
                                        let a1443 = v0;
                                        let a1444 = 51u64;
                                        if nat_eq(a1443, a1444) {
                                            f110(v1, v2, v3, v4)?
                                        } else {
                                            let a1445 = v0;
                                            let a1446 = 39u64;
                                            if nat_eq(a1445, a1446) {
                                                f111(v2, v3, v4)?
                                            } else {
                                                let a1447 = v0;
                                                let a1448 = 77u64;
                                                if nat_eq(a1447, a1448) {
                                                    f112(v2, v4)?
                                                } else {
                                                    let a1449 = v0;
                                                    let a1450 = 40u64;
                                                    if nat_eq(a1449, a1450) {
                                                        f113(v2, v3, v4)?
                                                    } else {
                                                        if f114(v0) {
                                                            let a1451 = v0;
                                                            let a1452 = 33u64;
                                                            if if if nat_eq(a1451, a1452) {
                                                                let a1454 = match v2.clone() {
                                                                    Adt2::C0(h1453, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1453
                                                                    }
                                                                };
                                                                let a1455 = 0u64;
                                                                nat_eq(a1454, a1455)
                                                            } else {
                                                                false
                                                            } {
                                                                let a1457 = match v4.clone() {
                                                                    Adt2::C0(h1456, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1456
                                                                    }
                                                                };
                                                                let a1458 = 26u64;
                                                                nat_eq(a1457, a1458)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1459 = v3;
                                                                match m1459 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f115(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a1460 = v0;
                                                                let a1461 = 33u64;
                                                                if if if nat_eq(a1460, a1461) {
                                                                    let a1463 = match v4.clone() {
                                                                        Adt2::C0(h1462, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1462
                                                                        }
                                                                    };
                                                                    let a1464 = 0u64;
                                                                    nat_eq(a1463, a1464)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f116(match v2.clone() {
                                                                        Adt2::C0(h1465, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1465
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f117(v2, v3, v4)?
                                                                } else {
                                                                    f118(v0, v2, {
                                                                        let m1466 = v3;
                                                                        match m1466 {
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
                                                            let a1467 = v0;
                                                            let a1468 = 19u64;
                                                            if nat_eq(a1467, a1468) {
                                                                f119(v2, v4)?
                                                            } else {
                                                                let a1469 = v0;
                                                                let a1470 = 21u64;
                                                                if nat_eq(a1469, a1470) {
                                                                    f120(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a1471 = v0;
                                                                    let a1472 = 61u64;
                                                                    if if if nat_eq(a1471, a1472) {
                                                                        true
                                                                    } else {
                                                                        let a1473 = v0;
                                                                        let a1474 = 62u64;
                                                                        nat_eq(a1473, a1474)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a1475 = v0;
                                                                        let a1476 = 63u64;
                                                                        nat_eq(a1475, a1476)
                                                                    } {
                                                                        f121(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a1477 = v0;
                                                                        let a1478 = 4u64;
                                                                        if if nat_eq(a1477, a1478) {
                                                                            true
                                                                        } else {
                                                                            let a1479 = v0;
                                                                            let a1480 = 32u64;
                                                                            nat_eq(a1479, a1480)
                                                                        } {
                                                                            f122(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a1481 = v0;
                                                                            let a1482 = 20u64;
                                                                            if nat_eq(a1481, a1482) {
                                                                                f123(v2, v4)?
                                                                            } else {
                                                                                let a1483 = v0;
                                                                                let a1484 = 42u64;
                                                                                if nat_eq(a1483, a1484) {
                                                                                    f124(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a1485 = v0;
                                                                                    let a1486 = 69u64;
                                                                                    if nat_eq(a1485, a1486) {
                                                                                        let a1487 = v5;
                                                                                        let a1488 = 4u64;
                                                                                        if nat_lt(a1487, a1488) {
                                                                                            f124(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a1489 = v0;
                                                                                        let a1490 = 70u64;
                                                                                        if nat_eq(a1489, a1490) {
                                                                                            f125(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a1491 = v0;
                                                                                            let a1492 = 75u64;
                                                                                            if if nat_eq(a1491, a1492) {
                                                                                                true
                                                                                            } else {
                                                                                                let a1493 = v0;
                                                                                                let a1494 = 76u64;
                                                                                                nat_eq(a1493, a1494)
                                                                                            } {
                                                                                                f126({
                                                                                                    let a1495 = v0;
                                                                                                    let a1496 = 76u64;
                                                                                                    nat_eq(a1495, a1496)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a1497 = v0;
                                                                                                let a1498 = 31u64;
                                                                                                if if if nat_eq(a1497, a1498) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1499 = v0;
                                                                                                    let a1500 = 47u64;
                                                                                                    nat_eq(a1499, a1500)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1501 = v0;
                                                                                                    let a1502 = 68u64;
                                                                                                    nat_eq(a1501, a1502)
                                                                                                } {
                                                                                                    f127(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1503 = v0;
                                                                                                    let a1504 = 15u64;
                                                                                                    if nat_eq(a1503, a1504) {
                                                                                                        f128(v2, v4)?
                                                                                                    } else {
                                                                                                        let a1505 = v0;
                                                                                                        let a1506 = 17u64;
                                                                                                        if nat_eq(a1505, a1506) {
                                                                                                            f129(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1507 = v0;
                                                                                                            let a1508 = 18u64;
                                                                                                            if nat_eq(a1507, a1508) {
                                                                                                                f130(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a1509 = v0;
                                                                                                                let a1510 = 13u64;
                                                                                                                if if if nat_eq(a1509, a1510) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1511 = v0;
                                                                                                                    let a1512 = 14u64;
                                                                                                                    nat_eq(a1511, a1512)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1513 = v0;
                                                                                                                    let a1514 = 16u64;
                                                                                                                    nat_eq(a1513, a1514)
                                                                                                                } {
                                                                                                                    f131(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m1515 = f132(v0, v5)?;
                                                                                                                    match m1515 {
                                                                                                                        None => {
                                                                                                                            f133(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f134(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f56(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f135(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1516, _, _, _) => {
            h1516
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1517, _, _) => {
            h1517
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1518, _) => {
            h1518
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1519) => {
            h1519
        }
    })
}

pub fn f57(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1521 = v0;
        let a1522 = match v1.clone() {
            Adt3::C0(h1520, _, _, _, _, _, _, _) => {
                h1520
            }
        };
        let a1523 = 0u64;
        nat_quot(a1521, a1522, a1523)
    };
    let v4: u64 = {
        let a1525 = v3;
        let a1526 = match v1.clone() {
            Adt3::C0(_, h1524, _, _, _, _, _, _) => {
                h1524
            }
        };
        let a1527 = 0u64;
        nat_quot(a1525, a1526, a1527)
    };
    let a1528 = v2;
    let a1529 = 0u64;
    if nat_eq(a1528, a1529) {
        let a1531 = v0;
        let a1532 = match v1 {
            Adt3::C0(h1530, _, _, _, _, _, _, _) => {
                h1530
            }
        };
        let a1533 = 0u64;
        nat_rem(a1531, a1532, a1533)
    } else {
        let a1534 = v2;
        let a1535 = 1u64;
        if nat_eq(a1534, a1535) {
            let a1537 = v3;
            let a1538 = match v1 {
                Adt3::C0(_, h1536, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1539 = 0u64;
            nat_rem(a1537, a1538, a1539)
        } else {
            let a1540 = v2;
            let a1541 = 2u64;
            if nat_eq(a1540, a1541) {
                let a1543 = v4;
                let a1544 = match v1 {
                    Adt3::C0(_, _, h1542, _, _, _, _, _) => {
                        h1542
                    }
                };
                let a1545 = 0u64;
                nat_rem(a1543, a1544, a1545)
            } else {
                let a1547 = v4;
                let a1548 = match v1 {
                    Adt3::C0(_, _, h1546, _, _, _, _, _) => {
                        h1546
                    }
                };
                let a1549 = 0u64;
                nat_quot(a1547, a1548, a1549)
            }
        }
    }
}

pub fn f58(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1551 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1550) => {
            h1550
        }
    };
    let a1552 = 0u64;
    if nat_eq(a1551, a1552) {
        Ok(v2)
    } else {
        let a1557 = {
            let a1554 = v2;
            let a1555 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1553) => {
                    h1553
                }
            };
            nat_mul(a1554, a1555)?
        };
        let a1558 = f57(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1556, _, _, _, _, _, _) => {
                h1556
            }
        }, 3u64);
        nat_add(a1557, a1558)
    }
}

pub fn f59(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1568 = {
            let a1565 = {
                let a1560 = v2;
                let a1561 = match v0.clone() {
                    Adt6::C0(_, h1559, _, _) => {
                        h1559
                    }
                };
                nat_mul(a1560, a1561)?
            };
            let a1566 = {
                let a1563 = v3;
                let a1564 = match v0.clone() {
                    Adt6::C0(_, _, _, h1562) => {
                        h1562
                    }
                };
                nat_mul(a1563, a1564)?
            };
            nat_add(a1565, a1566)?
        };
        let a1569 = f136(match v0.clone() {
            Adt6::C0(_, _, h1567, _) => {
                h1567
            }
        });
        nat_add(a1568, a1569)?
    };
    let v5: u64 = f137(match v0 {
        Adt6::C0(_, _, h1570, _) => {
            h1570
        }
    });
    let a1571 = v5;
    let a1572 = v4;
    if if nat_le(a1571, a1572) {
        let a1575 = {
            let a1573 = v4;
            let a1574 = v5;
            nat_sub(a1573, a1574)
        };
        let a1576 = v1;
        nat_lt(a1575, a1576)
    } else {
        false
    } {
        Ok(Some({
            let a1577 = v4;
            let a1578 = v5;
            nat_sub(a1577, a1578)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f60(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1579, _, _, _, _, _) => {
            h1579
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1580, _, _, _, _, _, _) => {
            h1580
        }
    };
    let v7: u64 = f57(v1, v6.clone(), 2u64);
    let a1582 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1581) => {
            h1581
        }
    };
    let a1583 = 0u64;
    if nat_eq(a1582, a1583) {
        Ok(Adt9::C8(Some(f56(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1586 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1585, _, _, _, _) => {
                    h1585
                }
            };
            match m1586 {
                Adt3::C0(_, _, h1584, _, _, _, _, _) => {
                    h1584
                }
            }
        };
        let m1592 = f59(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1587, _) => {
                h1587
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1588, _, _, _, _, _) => {
                h1588
            }
        }, {
            let a1589 = v2;
            let a1590 = v8;
            let a1591 = 0u64;
            nat_rem(a1589, a1590, a1591)
        }, v7)?;
        match m1592 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1599 = {
                        let a1597 = {
                            let a1593 = v2;
                            let a1594 = v8;
                            let a1595 = 0u64;
                            nat_quot(a1593, a1594, a1595)
                        };
                        let a1598 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1596) => {
                                h1596
                            }
                        };
                        nat_mul(a1597, a1598)?
                    };
                    let a1600 = f57(v1, v6, 3u64);
                    nat_add(a1599, a1600)?
                };
                Ok(Adt9::C8(Some(f56(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f61(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1601 = v1;
    match m1601 {
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
            f138(f139(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f140(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f141(v0.clone(), v24);
            f138(f142(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f62(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1602 = v3;
    match m1602 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f88(v0, v1)?, f88(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f144(v0, v1, v2.clone())?, f88(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f145(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f146(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f147(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f146(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f148(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f149(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f150(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f88(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f151(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v51, v52) => {
            f152(v0, v1, v2, v51, v52)
        }
        Adt4::C12(v53, v54, v55) => {
            f153(v0, v1, v2, v53, v54, v55)
        }
        Adt4::C13(_, v57, _, _) => {
            Ok(Adt9::C3(v57, v57, f88(v0, v1)?, f88(v0, v2)?))
        }
        Adt4::C14(v60, v61, v62) => {
            let a1605 = v60;
            let a1606 = 23u64;
            if nat_eq(a1605, a1606) {
                f154(v0, v1, v2, v61, v62)
            } else {
                Ok(Adt9::C3(v61, v62, f88(v0, v1)?, f88(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f155(v0, v1, v2)
        }
        Adt4::C16(v63, v64) => {
            f156(v0, v1, v2, v63, v64)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v65, v66) => {
            f157(v0, v1, v66, v65)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f88(f143(v0, match v2.clone() {
                Adt3::C0(h1603, _, _, _, _, _, _, _) => {
                    h1603
                }
            }, match v2.clone() {
                Adt3::C0(_, h1604, _, _, _, _, _, _) => {
                    h1604
                }
            })?, v1)?, f88(v0, v2)?))
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
        Adt4::C25(v71, v72, v73) => {
            f158(v0, v71, v72, v73)
        }
        Adt4::C26(v79, _, _, _) => {
            let a1607 = v79;
            let a1608 = 64u64;
            if if if nat_eq(a1607, a1608) {
                true
            } else {
                let a1609 = v79;
                let a1610 = 65u64;
                nat_eq(a1609, a1610)
            } {
                true
            } else {
                let a1611 = v79;
                let a1612 = 66u64;
                nat_eq(a1611, a1612)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v78) => {
            f80(v0, v1, v2, v78)
        }
        Adt4::C28(v74) => {
            f77(v0, v1, v2, v74, 0u64)
        }
        Adt4::C29(v75) => {
            f78(v0, v1, v2, v75)
        }
        Adt4::C30(v76, _) => {
            f79(v0, v1, v2, v76)
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
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f159(v3, v4);
    let v6: u64 = {
        let a1613 = v0;
        let a1614 = v5;
        let a1615 = 0u64;
        nat_quot(a1613, a1614, a1615)
    };
    let a1616 = v5;
    let a1617 = 1u64;
    if nat_eq(a1616, a1617) {
        Ok(f138(f160(v3, v4), v3, v4, 0u64, f88(v6, v1)?, 2u64, 0u64, 3u64, f88(v6, v2)?))
    } else {
        let a1621 = {
            let a1618 = v0;
            let a1619 = 2u64;
            let a1620 = 0u64;
            nat_rem(a1618, a1619, a1620)
        };
        let a1622 = 0u64;
        if nat_eq(a1621, a1622) {
            Ok(f138(f160(v3, 0u64), v3, 0u64, 0u64, f88(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f138(f160(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f88(v6, v2)?))
        }
    }
}

pub fn f64(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f161(v1)?;
    let v4: u64 = {
        let a1623 = v0;
        let a1624 = v3;
        let a1625 = 0u64;
        nat_rem(a1623, a1624, a1625)
    };
    let v5: u64 = {
        let a1628 = {
            let a1626 = v1;
            let a1627 = 3u64;
            nat_mul(a1626, a1627)?
        };
        let a1629 = 1u64;
        nat_add(a1628, a1629)?
    };
    let a1630 = v4;
    let a1631 = 0u64;
    if nat_eq(a1630, a1631) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1632 = v4;
        let a1633 = v5;
        if nat_eq(a1632, a1633) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1638 = v4;
                let a1639 = {
                    let a1634 = v4;
                    let a1635 = v5;
                    if nat_lt(a1634, a1635) {
                        1u64
                    } else {
                        let a1636 = v5;
                        let a1637 = 1u64;
                        nat_add(a1636, a1637)?
                    }
                };
                nat_sub(a1638, a1639)
            };
            let v7: u64 = {
                let a1640 = v6;
                let a1641 = 3u64;
                let a1642 = 0u64;
                nat_rem(a1640, a1641, a1642)
            };
            let v8: u64 = {
                let a1653 = 4u64;
                let a1654 = {
                    let a1651 = {
                        let a1646 = {
                            let a1643 = v0;
                            let a1644 = v3;
                            let a1645 = 0u64;
                            nat_quot(a1643, a1644, a1645)
                        };
                        let a1647 = v1;
                        nat_mul(a1646, a1647)?
                    };
                    let a1652 = {
                        let a1648 = v6;
                        let a1649 = 3u64;
                        let a1650 = 0u64;
                        nat_quot(a1648, a1649, a1650)
                    };
                    nat_add(a1651, a1652)?
                };
                nat_mul(a1653, a1654)?
            };
            let a1655 = v4;
            let a1656 = v5;
            if nat_lt(a1655, a1656) {
                let a1657 = v7;
                let a1658 = 0u64;
                if nat_eq(a1657, a1658) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1659 = v7;
                    let a1660 = 1u64;
                    if nat_eq(a1659, a1660) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1661 = v7;
                let a1662 = 0u64;
                if nat_eq(a1661, a1662) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1663 = v7;
                    let a1664 = 1u64;
                    if nat_eq(a1663, a1664) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f65(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1665 = v0;
        let a1666 = 6u64;
        let a1667 = 0u64;
        nat_quot(a1665, a1666, a1667)
    };
    let v5: u64 = {
        let a1668 = v0;
        let a1669 = 6u64;
        let a1670 = 0u64;
        nat_rem(a1668, a1669, a1670)
    };
    let a1671 = v5;
    let a1672 = 0u64;
    if nat_eq(a1671, a1672) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f88(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1673 = v5;
        let a1674 = 1u64;
        if nat_eq(a1673, a1674) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1675 = v5;
            let a1676 = 2u64;
            if nat_eq(a1675, a1676) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f88(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1677 = v5;
                let a1678 = 3u64;
                if nat_eq(a1677, a1678) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1679 = v5;
                    let a1680 = 4u64;
                    if nat_eq(a1679, a1680) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f88(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f88(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f66(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f162(match v1.clone() {
        Adt3::C0(_, h1681, _, _, _, _, _, _) => {
            h1681
        }
    }, v4)?;
    let v8: u64 = {
        let a1682 = v0;
        let a1683 = v7;
        let a1684 = 0u64;
        nat_rem(a1682, a1683, a1684)
    };
    let v9: u64 = {
        let a1685 = v0;
        let a1686 = v7;
        let a1687 = 0u64;
        nat_quot(a1685, a1686, a1687)
    };
    let a1690 = {
        let a1688 = v8;
        let a1689 = 1u64;
        nat_add(a1688, a1689)?
    };
    let a1691 = v7;
    if nat_eq(a1690, a1691) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f88(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1692 = v8;
            let a1693 = v6;
            let a1694 = 0u64;
            nat_quot(a1692, a1693, a1694)
        };
        let v11: u64 = {
            let a1695 = v8;
            let a1696 = v6;
            let a1697 = 0u64;
            nat_rem(a1695, a1696, a1697)
        };
        let v12: u64 = {
            let a1699 = v9;
            let a1700 = match v1.clone() {
                Adt3::C0(h1698, _, _, _, _, _, _, _) => {
                    h1698
                }
            };
            let a1701 = 0u64;
            nat_rem(a1699, a1700, a1701)
        };
        let v13: u64 = {
            let a1703 = v9;
            let a1704 = match v1.clone() {
                Adt3::C0(h1702, _, _, _, _, _, _, _) => {
                    h1702
                }
            };
            let a1705 = 0u64;
            nat_quot(a1703, a1704, a1705)
        };
        let v14: u64 = if v4 {
            let a1717 = {
                let a1712 = {
                    let a1707 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1706, _, _, _) => {
                            h1706
                        }
                    };
                    let a1708 = v12;
                    nat_mul(a1707, a1708)?
                };
                let a1713 = {
                    let a1710 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1709, _, _) => {
                            h1709
                        }
                    };
                    let a1711 = v10;
                    nat_mul(a1710, a1711)?
                };
                nat_add(a1712, a1713)?
            };
            let a1718 = {
                let a1715 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1714, _) => {
                        h1714
                    }
                };
                let a1716 = v13;
                nat_mul(a1715, a1716)?
            };
            nat_add(a1717, a1718)?
        } else {
            let a1725 = {
                let a1720 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1719, _, _, _) => {
                        h1719
                    }
                };
                let a1721 = v10;
                nat_mul(a1720, a1721)?
            };
            let a1726 = {
                let a1723 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1722, _, _) => {
                        h1722
                    }
                };
                let a1724 = v13;
                nat_mul(a1723, a1724)?
            };
            nat_add(a1725, a1726)?
        };
        if if v4 {
            let a1727 = v11;
            let a1728 = 0u64;
            nat_eq(a1727, a1728)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1731 = {
                let a1729 = v11;
                let a1730 = 1u64;
                nat_add(a1729, a1730)?
            };
            let a1732 = v6;
            if nat_eq(a1731, a1732) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1733 = v10;
                    let a1734 = 0u64;
                    if nat_eq(a1733, a1734) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1746 = {
                        let a1741 = {
                            let a1736 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1735, _, _, _) => {
                                    h1735
                                }
                            };
                            let a1737 = v12;
                            nat_mul(a1736, a1737)?
                        };
                        let a1742 = {
                            let a1739 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1738, _, _) => {
                                    h1738
                                }
                            };
                            let a1740 = v10;
                            nat_mul(a1739, a1740)?
                        };
                        nat_add(a1741, a1742)?
                    };
                    let a1747 = {
                        let a1744 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1743, _) => {
                                h1743
                            }
                        };
                        let a1745 = v13;
                        nat_mul(a1744, a1745)?
                    };
                    nat_add(a1746, a1747)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f32(match v2.clone() {
        Adt3::C0(_, h1748, _, _, _, _, _, _) => {
            h1748
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1749 = v0;
        let a1750 = v6;
        let a1751 = 0u64;
        nat_rem(a1749, a1750, a1751)
    };
    let v8: u64 = {
        let a1752 = v0;
        let a1753 = v6;
        let a1754 = 0u64;
        nat_quot(a1752, a1753, a1754)
    };
    let v9: u64 = {
        let a1756 = v8;
        let a1757 = match v5.clone() {
            Adt3::C0(h1755, _, _, _, _, _, _, _) => {
                h1755
            }
        };
        let a1758 = 0u64;
        nat_rem(a1756, a1757, a1758)
    };
    let v10: u64 = {
        let a1760 = v8;
        let a1761 = match v5.clone() {
            Adt3::C0(h1759, _, _, _, _, _, _, _) => {
                h1759
            }
        };
        let a1762 = 0u64;
        nat_quot(a1760, a1761, a1762)
    };
    let v11: u64 = {
        let a1764 = v10;
        let a1765 = match v2.clone() {
            Adt3::C0(_, h1763, _, _, _, _, _, _) => {
                h1763
            }
        };
        let a1766 = 0u64;
        nat_rem(a1764, a1765, a1766)
    };
    let v12: u64 = {
        let a1768 = v10;
        let a1769 = match v2.clone() {
            Adt3::C0(_, h1767, _, _, _, _, _, _) => {
                h1767
            }
        };
        let a1770 = 0u64;
        nat_quot(a1768, a1769, a1770)
    };
    let a1771 = v7;
    let a1772 = 0u64;
    if nat_eq(a1771, a1772) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1779 = {
                let a1774 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1773, _, _, _) => {
                        h1773
                    }
                };
                let a1775 = v9;
                nat_mul(a1774, a1775)?
            };
            let a1780 = {
                let a1777 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1776, _, _) => {
                        h1776
                    }
                };
                let a1778 = v12;
                nat_mul(a1777, a1778)?
            };
            nat_add(a1779, a1780)?
        }), Adt12::C0(2u64, {
            let a1787 = {
                let a1782 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1781, _, _, _) => {
                        h1781
                    }
                };
                let a1783 = v11;
                nat_mul(a1782, a1783)?
            };
            let a1788 = {
                let a1785 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1784, _, _) => {
                        h1784
                    }
                };
                let a1786 = v12;
                nat_mul(a1785, a1786)?
            };
            nat_add(a1787, a1788)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1789 = v4;
        match m1789 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1801 = {
                        let a1796 = {
                            let a1791 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1790, _, _, _) => {
                                    h1790
                                }
                            };
                            let a1792 = v9;
                            nat_mul(a1791, a1792)?
                        };
                        let a1797 = {
                            let a1794 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1793, _, _) => {
                                    h1793
                                }
                            };
                            let a1795 = v11;
                            nat_mul(a1794, a1795)?
                        };
                        nat_add(a1796, a1797)?
                    };
                    let a1802 = {
                        let a1799 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1798, _) => {
                                h1798
                            }
                        };
                        let a1800 = v12;
                        nat_mul(a1799, a1800)?
                    };
                    nat_add(a1801, a1802)?
                }), Adt13::C1(f88(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1803 = v7;
                    let a1804 = 1u64;
                    nat_sub(a1803, a1804)
                };
                let v15: u64 = {
                    let a1805 = v14;
                    let a1806 = 2u64;
                    let a1807 = 0u64;
                    nat_quot(a1805, a1806, a1807)
                };
                let a1811 = {
                    let a1808 = v14;
                    let a1809 = 2u64;
                    let a1810 = 0u64;
                    nat_rem(a1808, a1809, a1810)
                };
                let a1812 = 0u64;
                if nat_eq(a1811, a1812) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1824 = {
                            let a1819 = {
                                let a1814 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1813, _, _, _) => {
                                        h1813
                                    }
                                };
                                let a1815 = v9;
                                nat_mul(a1814, a1815)?
                            };
                            let a1820 = {
                                let a1817 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1816, _, _) => {
                                        h1816
                                    }
                                };
                                let a1818 = v15;
                                nat_mul(a1817, a1818)?
                            };
                            nat_add(a1819, a1820)?
                        };
                        let a1825 = {
                            let a1822 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1821, _) => {
                                    h1821
                                }
                            };
                            let a1823 = v12;
                            nat_mul(a1822, a1823)?
                        };
                        nat_add(a1824, a1825)?
                    }), Adt12::C0(3u64, {
                        let a1837 = {
                            let a1832 = {
                                let a1827 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1826, _, _, _) => {
                                        h1826
                                    }
                                };
                                let a1828 = v11;
                                nat_mul(a1827, a1828)?
                            };
                            let a1833 = {
                                let a1830 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1829, _, _) => {
                                        h1829
                                    }
                                };
                                let a1831 = v15;
                                nat_mul(a1830, a1831)?
                            };
                            nat_add(a1832, a1833)?
                        };
                        let a1838 = {
                            let a1835 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1834, _) => {
                                    h1834
                                }
                            };
                            let a1836 = v12;
                            nat_mul(a1835, a1836)?
                        };
                        nat_add(a1837, a1838)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1841 = {
                            let a1839 = v7;
                            let a1840 = 1u64;
                            nat_add(a1839, a1840)?
                        };
                        let a1842 = v6;
                        if nat_eq(a1841, a1842) {
                            Adt13::C1(f88(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f31(v6)?;
    let v9: u64 = {
        let a1843 = v0;
        let a1844 = v8;
        let a1845 = 0u64;
        nat_rem(a1843, a1844, a1845)
    };
    let v10: u64 = {
        let a1846 = v0;
        let a1847 = v8;
        let a1848 = 0u64;
        nat_quot(a1846, a1847, a1848)
    };
    let a1849 = v9;
    let a1850 = 148u64;
    if nat_lt(a1849, a1850) {
        f163({
            let a1851 = v9;
            let a1852 = 37u64;
            let a1853 = 0u64;
            nat_rem(a1851, a1852, a1853)
        }, {
            let a1854 = v9;
            let a1855 = 37u64;
            let a1856 = 0u64;
            nat_quot(a1854, a1855, a1856)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f164({
            let a1857 = v9;
            let a1858 = 148u64;
            nat_sub(a1857, a1858)
        }, v10, v4, v5)
    }
}

pub fn f69(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f27(v2);
    let a1859 = v0;
    let a1860 = v3;
    if nat_lt(a1859, a1860) {
        let a1861 = v0;
        let a1862 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1861, a1862) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1863 = 4u64;
                let a1864 = v0;
                nat_mul(a1863, a1864)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1865 = v0;
            let a1866 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1865, a1866) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1869 = v0;
                let a1870 = {
                    let a1867 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1868 = 1u64;
                    nat_add(a1867, a1868)?
                };
                if nat_eq(a1869, a1870) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1871 = v0;
                    let a1872 = 9u64;
                    if nat_eq(a1871, a1872) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1873 = v0;
            let a1874 = v3;
            nat_sub(a1873, a1874)
        };
        let v5: u64 = f28(v2);
        let v6: u64 = {
            let a1875 = v4;
            let a1876 = v5;
            let a1877 = 0u64;
            nat_rem(a1875, a1876, a1877)
        };
        let v7: u64 = f88({
            let a1878 = v4;
            let a1879 = v5;
            let a1880 = 0u64;
            nat_quot(a1878, a1879, a1880)
        }, v1)?;
        if v2 {
            let a1881 = v6;
            let a1882 = 7u64;
            if nat_lt(a1881, a1882) {
                let a1883 = v6;
                let a1884 = 3u64;
                if nat_lt(a1883, a1884) {
                    let a1885 = v6;
                    let a1886 = 1u64;
                    if nat_lt(a1885, a1886) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1887 = v6;
                        let a1888 = 2u64;
                        if nat_lt(a1887, a1888) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1889 = v6;
                    let a1890 = 5u64;
                    if nat_lt(a1889, a1890) {
                        let a1891 = v6;
                        let a1892 = 4u64;
                        if nat_lt(a1891, a1892) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1893 = v6;
                        let a1894 = 6u64;
                        if nat_lt(a1893, a1894) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1895 = v6;
                let a1896 = 11u64;
                if nat_lt(a1895, a1896) {
                    let a1897 = v6;
                    let a1898 = 9u64;
                    if nat_lt(a1897, a1898) {
                        let a1899 = v6;
                        let a1900 = 8u64;
                        if nat_lt(a1899, a1900) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1901 = v6;
                        let a1902 = 10u64;
                        if nat_lt(a1901, a1902) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1903 = v6;
                    let a1904 = 13u64;
                    if nat_lt(a1903, a1904) {
                        let a1905 = v6;
                        let a1906 = 12u64;
                        if nat_lt(a1905, a1906) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1907 = v6;
                        let a1908 = 14u64;
                        if nat_lt(a1907, a1908) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1909 = v6;
            let a1910 = 1u64;
            if nat_lt(a1909, a1910) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1911 = v6;
                let a1912 = 2u64;
                if nat_lt(a1911, a1912) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: u64) -> R<Adt9> {
    let v7: u64 = match v1.clone() {
        Adt3::C0(h1913, _, _, _, _, _, _, _) => {
            h1913
        }
    };
    let v8: u64 = f23(v3, v7)?;
    let v9: u64 = {
        let a1914 = v0;
        let a1915 = 2u64;
        nat_sub(a1914, a1915)
    };
    let v10: u64 = {
        let a1916 = v9;
        let a1917 = v8;
        let a1918 = 0u64;
        nat_quot(a1916, a1917, a1918)
    };
    let v11: u64 = {
        let a1919 = v9;
        let a1920 = v8;
        let a1921 = 0u64;
        nat_rem(a1919, a1920, a1921)
    };
    let a1922 = v0;
    let a1923 = 0u64;
    if nat_eq(a1922, a1923) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a1924 = v0;
        let a1925 = 1u64;
        if nat_eq(a1924, a1925) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a1926 = v10;
            let a1927 = v5;
            if nat_eq(a1926, a1927) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a1930 = v11;
                let a1931 = {
                    let a1928 = 2u64;
                    let a1929 = v7;
                    nat_mul(a1928, a1929)?
                };
                if nat_lt(a1930, a1931) {
                    let v12: u64 = {
                        let a1932 = v11;
                        let a1933 = 2u64;
                        let a1934 = 0u64;
                        nat_quot(a1932, a1933, a1934)
                    };
                    let a1938 = {
                        let a1935 = v11;
                        let a1936 = 2u64;
                        let a1937 = 0u64;
                        nat_rem(a1935, a1936, a1937)
                    };
                    let a1939 = 0u64;
                    if nat_eq(a1938, a1939) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a1940 = v12;
                            let a1941 = 0u64;
                            if nat_eq(a1940, a1941) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a1947 = {
                                let a1943 = v10;
                                let a1944 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h1942, _, _) => {
                                        h1942
                                    }
                                };
                                nat_mul(a1943, a1944)?
                            };
                            let a1948 = {
                                let a1945 = 4u64;
                                let a1946 = v12;
                                nat_mul(a1945, a1946)?
                            };
                            nat_add(a1947, a1948)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a1949 = v6;
                            let a1950 = 2u64;
                            if nat_eq(a1949, a1950) {
                                let a1951 = v12;
                                let a1952 = 0u64;
                                if nat_eq(a1951, a1952) {
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
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a1962 = v11;
                    let a1963 = {
                        let a1960 = 6u64;
                        let a1961 = v7;
                        nat_mul(a1960, a1961)?
                    };
                    if nat_lt(a1962, a1963) {
                        let v13: u64 = {
                            let a1966 = v11;
                            let a1967 = {
                                let a1964 = 2u64;
                                let a1965 = v7;
                                nat_mul(a1964, a1965)?
                            };
                            nat_sub(a1966, a1967)
                        };
                        let v14: u64 = {
                            let a1968 = v13;
                            let a1969 = 4u64;
                            let a1970 = 0u64;
                            nat_quot(a1968, a1969, a1970)
                        };
                        let a1974 = {
                            let a1971 = v13;
                            let a1972 = 4u64;
                            let a1973 = 0u64;
                            nat_rem(a1971, a1972, a1973)
                        };
                        let a1975 = 2u64;
                        if nat_lt(a1974, a1975) {
                            let a1979 = {
                                let a1976 = v13;
                                let a1977 = 4u64;
                                let a1978 = 0u64;
                                nat_rem(a1976, a1977, a1978)
                            };
                            let a1980 = 1u64;
                            if nat_lt(a1979, a1980) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a1986 = {
                                        let a1982 = v10;
                                        let a1983 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1981, _, _) => {
                                                h1981
                                            }
                                        };
                                        nat_mul(a1982, a1983)?
                                    };
                                    let a1987 = {
                                        let a1984 = 4u64;
                                        let a1985 = v14;
                                        nat_mul(a1984, a1985)?
                                    };
                                    nat_add(a1986, a1987)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a1993 = {
                                            let a1989 = v10;
                                            let a1990 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h1988, _, _) => {
                                                    h1988
                                                }
                                            };
                                            nat_mul(a1989, a1990)?
                                        };
                                        let a1994 = {
                                            let a1991 = 4u64;
                                            let a1992 = v14;
                                            nat_mul(a1991, a1992)?
                                        };
                                        nat_add(a1993, a1994)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a1998 = {
                                let a1995 = v13;
                                let a1996 = 4u64;
                                let a1997 = 0u64;
                                nat_rem(a1995, a1996, a1997)
                            };
                            let a1999 = 3u64;
                            if nat_lt(a1998, a1999) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2005 = {
                                            let a2001 = v10;
                                            let a2002 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2000, _, _) => {
                                                    h2000
                                                }
                                            };
                                            nat_mul(a2001, a2002)?
                                        };
                                        let a2006 = {
                                            let a2003 = 4u64;
                                            let a2004 = v14;
                                            nat_mul(a2003, a2004)?
                                        };
                                        nat_add(a2005, a2006)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2007 = v14;
                                    let a2008 = 0u64;
                                    if nat_eq(a2007, a2008) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2013 = v11;
                        let a2014 = {
                            let a2011 = {
                                let a2009 = 6u64;
                                let a2010 = v7;
                                nat_mul(a2009, a2010)?
                            };
                            let a2012 = 2u64;
                            nat_add(a2011, a2012)?
                        };
                        if nat_lt(a2013, a2014) {
                            let a2017 = v11;
                            let a2018 = {
                                let a2015 = 6u64;
                                let a2016 = v7;
                                nat_mul(a2015, a2016)?
                            };
                            if nat_eq(a2017, a2018) {
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
                                    let a2023 = v11;
                                    let a2024 = {
                                        let a2021 = {
                                            let a2019 = 6u64;
                                            let a2020 = v7;
                                            nat_mul(a2019, a2020)?
                                        };
                                        let a2022 = 2u64;
                                        nat_add(a2021, a2022)?
                                    };
                                    nat_sub(a2023, a2024)
                                };
                                let v16: u64 = {
                                    let a2025 = v15;
                                    let a2026 = 3u64;
                                    let a2027 = 0u64;
                                    nat_quot(a2025, a2026, a2027)
                                };
                                let a2031 = {
                                    let a2028 = v15;
                                    let a2029 = 3u64;
                                    let a2030 = 0u64;
                                    nat_rem(a2028, a2029, a2030)
                                };
                                let a2032 = 1u64;
                                if nat_lt(a2031, a2032) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2038 = {
                                            let a2034 = v10;
                                            let a2035 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2033, _, _) => {
                                                    h2033
                                                }
                                            };
                                            nat_mul(a2034, a2035)?
                                        };
                                        let a2039 = {
                                            let a2036 = 4u64;
                                            let a2037 = v16;
                                            nat_mul(a2036, a2037)?
                                        };
                                        nat_add(a2038, a2039)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2043 = {
                                        let a2040 = v15;
                                        let a2041 = 3u64;
                                        let a2042 = 0u64;
                                        nat_rem(a2040, a2041, a2042)
                                    };
                                    let a2044 = 2u64;
                                    if nat_lt(a2043, a2044) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2049 = {
                                                let a2045 = v10;
                                                let a2046 = v4;
                                                nat_mul(a2045, a2046)?
                                            };
                                            let a2050 = {
                                                let a2047 = 4u64;
                                                let a2048 = v16;
                                                nat_mul(a2047, a2048)?
                                            };
                                            nat_add(a2049, a2050)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2056 = {
                                                let a2052 = v10;
                                                let a2053 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2051, _, _) => {
                                                        h2051
                                                    }
                                                };
                                                nat_mul(a2052, a2053)?
                                            };
                                            let a2057 = {
                                                let a2054 = 4u64;
                                                let a2055 = v16;
                                                nat_mul(a2054, a2055)?
                                            };
                                            nat_add(a2056, a2057)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2062 = v11;
                                let a2063 = {
                                    let a2060 = {
                                        let a2058 = 11u64;
                                        let a2059 = v7;
                                        nat_mul(a2058, a2059)?
                                    };
                                    let a2061 = 2u64;
                                    nat_add(a2060, a2061)?
                                };
                                if nat_lt(a2062, a2063) {
                                    let v17: u64 = {
                                        let a2068 = v11;
                                        let a2069 = {
                                            let a2066 = {
                                                let a2064 = 6u64;
                                                let a2065 = v7;
                                                nat_mul(a2064, a2065)?
                                            };
                                            let a2067 = 2u64;
                                            nat_add(a2066, a2067)?
                                        };
                                        nat_sub(a2068, a2069)
                                    };
                                    let v18: u64 = {
                                        let a2070 = v17;
                                        let a2071 = 5u64;
                                        let a2072 = 0u64;
                                        nat_quot(a2070, a2071, a2072)
                                    };
                                    let a2076 = {
                                        let a2073 = v17;
                                        let a2074 = 5u64;
                                        let a2075 = 0u64;
                                        nat_rem(a2073, a2074, a2075)
                                    };
                                    let a2077 = 2u64;
                                    if nat_lt(a2076, a2077) {
                                        let a2081 = {
                                            let a2078 = v17;
                                            let a2079 = 5u64;
                                            let a2080 = 0u64;
                                            nat_rem(a2078, a2079, a2080)
                                        };
                                        let a2082 = 1u64;
                                        if nat_lt(a2081, a2082) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2088 = {
                                                    let a2084 = v10;
                                                    let a2085 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2083, _, _) => {
                                                            h2083
                                                        }
                                                    };
                                                    nat_mul(a2084, a2085)?
                                                };
                                                let a2089 = {
                                                    let a2086 = 4u64;
                                                    let a2087 = v18;
                                                    nat_mul(a2086, a2087)?
                                                };
                                                nat_add(a2088, a2089)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2093 = {
                                            let a2090 = v17;
                                            let a2091 = 5u64;
                                            let a2092 = 0u64;
                                            nat_rem(a2090, a2091, a2092)
                                        };
                                        let a2094 = 3u64;
                                        if nat_lt(a2093, a2094) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2099 = {
                                                    let a2095 = v10;
                                                    let a2096 = v4;
                                                    nat_mul(a2095, a2096)?
                                                };
                                                let a2100 = {
                                                    let a2097 = 4u64;
                                                    let a2098 = v18;
                                                    nat_mul(a2097, a2098)?
                                                };
                                                nat_add(a2099, a2100)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2104 = {
                                                let a2101 = v17;
                                                let a2102 = 5u64;
                                                let a2103 = 0u64;
                                                nat_rem(a2101, a2102, a2103)
                                            };
                                            let a2105 = 4u64;
                                            if nat_lt(a2104, a2105) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2106 = v18;
                                                    let a2107 = 0u64;
                                                    if nat_eq(a2106, a2107) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2112 = v11;
                                    let a2113 = {
                                        let a2110 = {
                                            let a2108 = 11u64;
                                            let a2109 = v7;
                                            nat_mul(a2108, a2109)?
                                        };
                                        let a2111 = 2u64;
                                        nat_add(a2110, a2111)?
                                    };
                                    if nat_eq(a2112, a2113) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2114 = v10;
                                            let a2115 = 0u64;
                                            if nat_eq(a2114, a2115) {
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

pub fn f71(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2116 = v0;
    let a2117 = v1;
    if nat_lt(a2116, a2117) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f88(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2118 = v0;
            let a2119 = v1;
            nat_sub(a2118, a2119)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f88(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2126 = {
            let a2124 = 3u64;
            let a2125 = {
                let a2122 = match v4.clone() {
                    Adt6::C0(h2120, _, _, _) => {
                        h2120
                    }
                };
                let a2123 = match v5.clone() {
                    Adt6::C0(h2121, _, _, _) => {
                        h2121
                    }
                };
                nat_mul(a2122, a2123)?
            };
            nat_mul(a2124, a2125)?
        };
        let a2127 = 2u64;
        nat_add(a2126, a2127)?
    };
    let v8: u64 = {
        let a2128 = v0;
        let a2129 = v7;
        let a2130 = 0u64;
        nat_quot(a2128, a2129, a2130)
    };
    let v9: u64 = {
        let a2131 = v0;
        let a2132 = v7;
        let a2133 = 0u64;
        nat_rem(a2131, a2132, a2133)
    };
    let a2134 = v9;
    let a2135 = 0u64;
    if nat_eq(a2134, a2135) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2138 = {
            let a2136 = v9;
            let a2137 = 1u64;
            nat_add(a2136, a2137)?
        };
        let a2139 = v7;
        if nat_eq(a2138, a2139) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f88(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2140 = v9;
                let a2141 = 1u64;
                nat_sub(a2140, a2141)
            };
            let v11: u64 = {
                let a2142 = v10;
                let a2143 = 3u64;
                let a2144 = 0u64;
                nat_quot(a2142, a2143, a2144)
            };
            let v12: u64 = {
                let a2146 = v11;
                let a2147 = match v4.clone() {
                    Adt6::C0(h2145, _, _, _) => {
                        h2145
                    }
                };
                let a2148 = 0u64;
                nat_rem(a2146, a2147, a2148)
            };
            let v13: u64 = {
                let a2150 = v11;
                let a2151 = match v4.clone() {
                    Adt6::C0(h2149, _, _, _) => {
                        h2149
                    }
                };
                let a2152 = 0u64;
                nat_quot(a2150, a2151, a2152)
            };
            let v14: u64 = {
                let a2153 = v10;
                let a2154 = 3u64;
                let a2155 = 0u64;
                nat_rem(a2153, a2154, a2155)
            };
            let v15: u64 = f57(v8, v3.clone(), 2u64);
            let m2157 = f59(v4, match v2.clone() {
                Adt3::C0(h2156, _, _, _, _, _, _, _) => {
                    h2156
                }
            }, f57(v8, v3.clone(), 0u64), v12)?;
            match m2157 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2159 = f59(v5, match v2.clone() {
                        Adt3::C0(_, h2158, _, _, _, _, _, _) => {
                            h2158
                        }
                    }, f57(v8, v3.clone(), 1u64), v13)?;
                    match m2159 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2160 = v14;
                            let a2161 = 0u64;
                            if nat_eq(a2160, a2161) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f56(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f165(v6), 4u64))
                            } else {
                                let a2162 = v14;
                                let a2163 = 1u64;
                                if nat_eq(a2162, a2163) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f56(v2, v16, v17, v15, f57(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f166(match v1.clone() {
        Adt3::C0(h2164, _, _, _, _, _, _, _) => {
            h2164
        }
    })?;
    let v5: u64 = {
        let a2165 = v0;
        let a2166 = v4;
        let a2167 = 0u64;
        nat_quot(a2165, a2166, a2167)
    };
    let v6: u64 = {
        let a2168 = v0;
        let a2169 = v4;
        let a2170 = 0u64;
        nat_rem(a2168, a2169, a2170)
    };
    let a2171 = v6;
    let a2172 = 0u64;
    if nat_eq(a2171, a2172) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2173 = v6;
        let a2174 = 1u64;
        if nat_eq(a2173, a2174) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2177 = {
                let a2175 = v6;
                let a2176 = 1u64;
                nat_add(a2175, a2176)?
            };
            let a2178 = v4;
            if nat_eq(a2177, a2178) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f88(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2179 = v6;
                    let a2180 = 2u64;
                    nat_sub(a2179, a2180)
                };
                let v8: u64 = {
                    let a2181 = v7;
                    let a2182 = 3u64;
                    let a2183 = 0u64;
                    nat_quot(a2181, a2182, a2183)
                };
                let v9: u64 = {
                    let a2184 = v7;
                    let a2185 = 3u64;
                    let a2186 = 0u64;
                    nat_rem(a2184, a2185, a2186)
                };
                let v10: u64 = f88({
                    let a2190 = {
                        let a2188 = v5;
                        let a2189 = match v1.clone() {
                            Adt3::C0(h2187, _, _, _, _, _, _, _) => {
                                h2187
                            }
                        };
                        nat_mul(a2188, a2189)?
                    };
                    let a2191 = v8;
                    nat_add(a2190, a2191)?
                }, v1)?;
                let a2192 = v9;
                let a2193 = 0u64;
                if nat_eq(a2192, a2193) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2194 = v9;
                    let a2195 = 1u64;
                    if nat_eq(a2194, a2195) {
                        let a2196 = v3;
                        let a2197 = 2u64;
                        if nat_eq(a2196, a2197) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2198 = v3;
                        let a2199 = 2u64;
                        if if nat_eq(a2198, a2199) {
                            true
                        } else {
                            let a2200 = v3;
                            let a2201 = 3u64;
                            nat_eq(a2200, a2201)
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

pub fn f74(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2203 = 2u64;
        let a2204 = match v2.clone() {
            Adt3::C0(_, h2202, _, _, _, _, _, _) => {
                h2202
            }
        };
        nat_mul(a2203, a2204)?
    };
    let a2205 = v1;
    let a2206 = v6;
    if nat_lt(a2205, a2206) {
        let a2210 = {
            let a2207 = v1;
            let a2208 = 2u64;
            let a2209 = 0u64;
            nat_rem(a2207, a2208, a2209)
        };
        let a2211 = 0u64;
        if nat_eq(a2210, a2211) {
            Ok(Adt9::C4(19u64, 4u64, f88({
                let a2212 = v1;
                let a2213 = 2u64;
                let a2214 = 0u64;
                nat_quot(a2212, a2213, a2214)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2215 = v5;
            let a2216 = 2147483648u64;
            if if nat_lt(a2215, a2216) {
                let a2218 = v5;
                let a2219 = match v3 {
                    Adt3::C0(_, h2217, _, _, _, _, _, _) => {
                        h2217
                    }
                };
                nat_lt(a2218, a2219)
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
            let a2222 = match v3.clone() {
                Adt3::C0(h2220, _, _, _, _, _, _, _) => {
                    h2220
                }
            };
            let a2223 = match v3.clone() {
                Adt3::C0(_, h2221, _, _, _, _, _, _) => {
                    h2221
                }
            };
            nat_mul(a2222, a2223)?
        };
        let a2226 = {
            let a2224 = v1;
            let a2225 = v6;
            nat_sub(a2224, a2225)
        };
        let a2227 = v7;
        if nat_lt(a2226, a2227) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f88({
                let a2228 = v1;
                let a2229 = v6;
                nat_sub(a2228, a2229)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2232 = {
                    let a2230 = v1;
                    let a2231 = v6;
                    nat_sub(a2230, a2231)
                };
                let a2233 = v7;
                nat_sub(a2232, a2233)
            };
            let v9: u64 = {
                let a2235 = 1u64;
                let a2236 = f167(v0, match v2.clone() {
                    Adt3::C0(h2234, _, _, _, _, _, _, _) => {
                        h2234
                    }
                })?;
                nat_add(a2235, a2236)?
            };
            let v10: u64 = {
                let a2237 = v8;
                let a2238 = v9;
                let a2239 = 0u64;
                nat_quot(a2237, a2238, a2239)
            };
            let v11: u64 = {
                let a2240 = v8;
                let a2241 = v9;
                let a2242 = 0u64;
                nat_rem(a2240, a2241, a2242)
            };
            let a2243 = v11;
            let a2244 = 0u64;
            if nat_eq(a2243, a2244) {
                Ok(Adt9::C4(19u64, 4u64, f88(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f168(v0) {
                    let a2248 = {
                        let a2245 = v11;
                        let a2246 = 2u64;
                        let a2247 = 0u64;
                        nat_rem(a2245, a2246, a2247)
                    };
                    let a2249 = 1u64;
                    if nat_eq(a2248, a2249) {
                        Ok(Adt9::C4(6u64, 0u64, f88({
                            let a2258 = {
                                let a2251 = v10;
                                let a2252 = match v2.clone() {
                                    Adt3::C0(h2250, _, _, _, _, _, _, _) => {
                                        h2250
                                    }
                                };
                                nat_mul(a2251, a2252)?
                            };
                            let a2259 = {
                                let a2255 = {
                                    let a2253 = v11;
                                    let a2254 = 1u64;
                                    nat_sub(a2253, a2254)
                                };
                                let a2256 = 2u64;
                                let a2257 = 0u64;
                                nat_quot(a2255, a2256, a2257)
                            };
                            nat_add(a2258, a2259)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f88({
                            let a2268 = {
                                let a2261 = v5;
                                let a2262 = match v2.clone() {
                                    Adt3::C0(h2260, _, _, _, _, _, _, _) => {
                                        h2260
                                    }
                                };
                                nat_mul(a2261, a2262)?
                            };
                            let a2269 = {
                                let a2265 = {
                                    let a2263 = v11;
                                    let a2264 = 1u64;
                                    nat_sub(a2263, a2264)
                                };
                                let a2266 = 2u64;
                                let a2267 = 0u64;
                                nat_quot(a2265, a2266, a2267)
                            };
                            nat_add(a2268, a2269)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f88({
                            let a2278 = {
                                let a2271 = v5;
                                let a2272 = match v2 {
                                    Adt3::C0(h2270, _, _, _, _, _, _, _) => {
                                        h2270
                                    }
                                };
                                nat_mul(a2271, a2272)?
                            };
                            let a2279 = {
                                let a2275 = {
                                    let a2273 = v11;
                                    let a2274 = 1u64;
                                    nat_sub(a2273, a2274)
                                };
                                let a2276 = 2u64;
                                let a2277 = 0u64;
                                nat_quot(a2275, a2276, a2277)
                            };
                            nat_add(a2278, a2279)?
                        }, v3)?))
                    }
                } else {
                    f169(v0, v10, {
                        let a2280 = v11;
                        let a2281 = 1u64;
                        nat_sub(a2280, a2281)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2282 = v1;
    let a2283 = f170(v2.clone(), v4.clone())?;
    if nat_lt(a2282, a2283) {
        let a2286 = v1;
        let a2287 = {
            let a2284 = 2u64;
            let a2285 = f171(v4.clone())?;
            nat_mul(a2284, a2285)?
        };
        if nat_lt(a2286, a2287) {
            let a2291 = {
                let a2288 = v1;
                let a2289 = 2u64;
                let a2290 = 0u64;
                nat_rem(a2288, a2289, a2290)
            };
            let a2292 = 0u64;
            if nat_eq(a2291, a2292) {
                f172({
                    let a2293 = v1;
                    let a2294 = 2u64;
                    let a2295 = 0u64;
                    nat_quot(a2293, a2294, a2295)
                }, v4)
            } else {
                if f173(match v2 {
                    Adt3::C0(_, h2296, _, _, _, _, _, _) => {
                        h2296
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2299 = v1;
                let a2300 = {
                    let a2297 = 2u64;
                    let a2298 = f171(v4.clone())?;
                    nat_mul(a2297, a2298)?
                };
                nat_sub(a2299, a2300)
            };
            let v7: u64 = {
                let a2304 = v6;
                let a2305 = {
                    let a2302 = 1u64;
                    let a2303 = match v2.clone() {
                        Adt3::C0(h2301, _, _, _, _, _, _, _) => {
                            h2301
                        }
                    };
                    nat_add(a2302, a2303)?
                };
                let a2306 = 0u64;
                nat_quot(a2304, a2305, a2306)
            };
            let v8: u64 = {
                let a2310 = v6;
                let a2311 = {
                    let a2308 = 1u64;
                    let a2309 = match v2.clone() {
                        Adt3::C0(h2307, _, _, _, _, _, _, _) => {
                            h2307
                        }
                    };
                    nat_add(a2308, a2309)?
                };
                let a2312 = 0u64;
                nat_rem(a2310, a2311, a2312)
            };
            let a2313 = v8;
            let a2314 = 0u64;
            if nat_eq(a2313, a2314) {
                f172(v7, v4)
            } else {
                f174(v0, v7, {
                    let a2315 = v8;
                    let a2316 = 1u64;
                    nat_sub(a2315, a2316)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2319 = match v4.clone() {
            Adt3::C0(h2317, _, _, _, _, _, _, _) => {
                h2317
            }
        };
        let a2320 = match v4.clone() {
            Adt3::C0(_, h2318, _, _, _, _, _, _) => {
                h2318
            }
        };
        nat_mul(a2319, a2320)?
    };
    let v7: u64 = {
        let a2321 = 2u64;
        let a2322 = v6;
        nat_mul(a2321, a2322)?
    };
    let v8: u64 = f175(match v3.clone() {
        Adt3::C0(_, h2323, _, _, _, _, _, _) => {
            h2323
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2324, _, _, _, _, _) => {
            h2324
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2325, _, _, _, _) => {
            h2325
        }
    });
    let a2326 = v0;
    let a2327 = v7;
    if nat_lt(a2326, a2327) {
        let a2331 = {
            let a2328 = v0;
            let a2329 = 2u64;
            let a2330 = 0u64;
            nat_rem(a2328, a2329, a2330)
        };
        let a2332 = 0u64;
        if nat_eq(a2331, a2332) {
            Ok(Adt9::C4(19u64, 5u64, f88({
                let a2333 = v0;
                let a2334 = 2u64;
                let a2335 = 0u64;
                nat_quot(a2333, a2334, a2335)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f176(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2336 = v0;
            let a2337 = v7;
            nat_sub(a2336, a2337)
        };
        let v10: u64 = {
            let a2341 = v9;
            let a2342 = {
                let a2339 = 1u64;
                let a2340 = match v1.clone() {
                    Adt3::C0(h2338, _, _, _, _, _, _, _) => {
                        h2338
                    }
                };
                nat_add(a2339, a2340)?
            };
            let a2343 = 0u64;
            nat_quot(a2341, a2342, a2343)
        };
        let v11: u64 = {
            let a2347 = v9;
            let a2348 = {
                let a2345 = 1u64;
                let a2346 = match v1.clone() {
                    Adt3::C0(h2344, _, _, _, _, _, _, _) => {
                        h2344
                    }
                };
                nat_add(a2345, a2346)?
            };
            let a2349 = 0u64;
            nat_rem(a2347, a2348, a2349)
        };
        let a2350 = v11;
        let a2351 = 0u64;
        if nat_eq(a2350, a2351) {
            Ok(Adt9::C4(19u64, 5u64, f88({
                let a2352 = v10;
                let a2353 = v6;
                let a2354 = 0u64;
                nat_rem(a2352, a2353, a2354)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f176(v5, v8) {
                let v12: u64 = {
                    let a2360 = {
                        let a2356 = v10;
                        let a2357 = match v1.clone() {
                            Adt3::C0(h2355, _, _, _, _, _, _, _) => {
                                h2355
                            }
                        };
                        nat_mul(a2356, a2357)?
                    };
                    let a2361 = {
                        let a2358 = v11;
                        let a2359 = 1u64;
                        nat_sub(a2358, a2359)
                    };
                    nat_add(a2360, a2361)?
                };
                let a2362 = v5;
                let a2363 = 4294967295u64;
                if nat_eq(a2362, a2363) {
                    Ok(Adt9::C1(false, f88(v12, v1)?, f88(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f88(v12, v1)?, 4u64, f88({
                        let a2369 = {
                            let a2365 = v5;
                            let a2366 = match v3.clone() {
                                Adt3::C0(h2364, _, _, _, _, _, _, _) => {
                                    h2364
                                }
                            };
                            nat_mul(a2365, a2366)?
                        };
                        let a2370 = {
                            let a2367 = v11;
                            let a2368 = 1u64;
                            nat_sub(a2367, a2368)
                        };
                        nat_add(a2369, a2370)?
                    }, v3)?, 3u64, f88(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f177(v3.clone())?;
    let v6: u64 = {
        let a2371 = v0;
        let a2372 = v5;
        let a2373 = 0u64;
        nat_quot(a2371, a2372, a2373)
    };
    let v7: u64 = {
        let a2374 = v0;
        let a2375 = v5;
        let a2376 = 0u64;
        nat_rem(a2374, a2375, a2376)
    };
    let v8: u64 = f178(v3.clone())?;
    let a2377 = v7;
    let a2378 = 0u64;
    if nat_eq(a2377, a2378) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2379, _) => {
                h2379
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2383 = v5;
        let a2384 = {
            let a2381 = v7;
            let a2382 = if match v3.clone() {
                Adt7::C0(_, _, _, h2380, _) => {
                    h2380
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2381, a2382)?
        };
        if nat_le(a2383, a2384) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2385, _) => {
                    h2385
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f88(v6, v2)?))
            } else {
                let a2388 = {
                    let a2386 = v7;
                    let a2387 = 2u64;
                    nat_add(a2386, a2387)?
                };
                let a2389 = v5;
                if nat_eq(a2388, a2389) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2390, _, _) => {
                            h2390
                        }
                    } {
                        let a2397 = {
                            let m2393 = match v3.clone() {
                                Adt7::C0(h2392, _, _, _, _) => {
                                    h2392
                                }
                            };
                            match m2393 {
                                Adt6::C0(h2391, _, _, _) => {
                                    h2391
                                }
                            }
                        };
                        let a2398 = {
                            let m2396 = match v3 {
                                Adt7::C0(_, h2395, _, _, _) => {
                                    h2395
                                }
                            };
                            match m2396 {
                                Adt6::C0(h2394, _, _, _) => {
                                    h2394
                                }
                            }
                        };
                        nat_mul(a2397, a2398)?
                    } else {
                        f179(match v3 {
                            Adt7::C0(h2399, _, _, _, _) => {
                                h2399
                            }
                        }, match v1 {
                            Adt3::C0(h2400, _, _, _, _, _, _, _) => {
                                h2400
                            }
                        }, f57(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2401 = v9;
                        let a2402 = 0u64;
                        if nat_eq(a2401, a2402) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f88(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2405 = {
                    let a2403 = v7;
                    let a2404 = 1u64;
                    nat_sub(a2403, a2404)
                };
                let a2406 = v8;
                let a2407 = 0u64;
                nat_rem(a2405, a2406, a2407)
            };
            let m2413 = f180(v3.clone(), v1, v2, v6, {
                let a2410 = {
                    let a2408 = v7;
                    let a2409 = 1u64;
                    nat_sub(a2408, a2409)
                };
                let a2411 = v8;
                let a2412 = 0u64;
                nat_quot(a2410, a2411, a2412)
            })?;
            match m2413 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2415 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2414) => {
                            h2414
                        }
                    };
                    let a2416 = 1u64;
                    if if nat_eq(a2415, a2416) {
                        let a2417 = v10;
                        let a2418 = 0u64;
                        nat_eq(a2417, a2418)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2419, _) => {
                                h2419
                            }
                        } {
                            let a2422 = {
                                let a2420 = v10;
                                let a2421 = 1u64;
                                nat_add(a2420, a2421)?
                            };
                            let a2423 = v8;
                            if nat_eq(a2422, a2423) {
                                let a2424 = v4;
                                let a2425 = 2u64;
                                if nat_eq(a2424, a2425) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2427 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2426) => {
                                                h2426
                                            }
                                        };
                                        let a2428 = 1u64;
                                        if nat_eq(a2427, a2428) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2430 = match v3 {
                                            Adt7::C0(_, _, _, _, h2429) => {
                                                h2429
                                            }
                                        };
                                        let a2431 = 1u64;
                                        if nat_eq(a2430, a2431) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2433 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2432) => {
                                            h2432
                                        }
                                    };
                                    let a2434 = 1u64;
                                    if nat_eq(a2433, a2434) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2436 = match v3 {
                                        Adt7::C0(_, _, _, _, h2435) => {
                                            h2435
                                        }
                                    };
                                    let a2437 = 1u64;
                                    if nat_eq(a2436, a2437) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2439 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2438) => {
                                        h2438
                                    }
                                };
                                let a2440 = 1u64;
                                if nat_eq(a2439, a2440) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2442 = match v3 {
                                    Adt7::C0(_, _, _, _, h2441) => {
                                        h2441
                                    }
                                };
                                let a2443 = 1u64;
                                if nat_eq(a2442, a2443) {
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

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f181(match v3.clone() {
        Adt3::C0(h2444, _, _, _, _, _, _, _) => {
            h2444
        }
    })?;
    let v5: u64 = {
        let a2445 = v0;
        let a2446 = v4;
        let a2447 = 0u64;
        nat_rem(a2445, a2446, a2447)
    };
    let v6: u64 = {
        let a2448 = v0;
        let a2449 = v4;
        let a2450 = 0u64;
        nat_quot(a2448, a2449, a2450)
    };
    let a2451 = v5;
    let a2452 = 0u64;
    if nat_eq(a2451, a2452) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2455 = {
            let a2453 = v5;
            let a2454 = 1u64;
            nat_add(a2453, a2454)?
        };
        let a2456 = v4;
        if nat_eq(a2455, a2456) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f88(v6, v2)?))
        } else {
            let a2460 = {
                let a2457 = v5;
                let a2458 = 2u64;
                let a2459 = 0u64;
                nat_rem(a2457, a2458, a2459)
            };
            let a2461 = 0u64;
            if nat_eq(a2460, a2461) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2464 = {
                        let a2462 = v5;
                        let a2463 = 1u64;
                        nat_sub(a2462, a2463)
                    };
                    let a2465 = 2u64;
                    let a2466 = 0u64;
                    nat_quot(a2464, a2465, a2466)
                };
                let v8: u64 = f57(v6, v2.clone(), 0u64);
                let v9: u64 = f57(v6, v2.clone(), 1u64);
                let v10: u64 = f57(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f56(v1, {
                    let a2467 = v9;
                    let a2468 = v7;
                    nat_add(a2467, a2468)?
                }, v8, v10, 0u64)?, 4u64, f56(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f182(v3.clone())?;
    let v5: u64 = {
        let a2469 = v0;
        let a2470 = v4;
        let a2471 = 0u64;
        nat_rem(a2469, a2470, a2471)
    };
    let v6: u64 = {
        let a2472 = v0;
        let a2473 = v4;
        let a2474 = 0u64;
        nat_quot(a2472, a2473, a2474)
    };
    let a2475 = v5;
    let a2476 = 0u64;
    if nat_eq(a2475, a2476) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2479 = {
            let a2477 = v5;
            let a2478 = 1u64;
            nat_add(a2477, a2478)?
        };
        let a2480 = v4;
        if nat_eq(a2479, a2480) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f88(v6, v2)?))
        } else {
            let a2484 = {
                let a2481 = v5;
                let a2482 = 2u64;
                let a2483 = 0u64;
                nat_rem(a2481, a2482, a2483)
            };
            let a2485 = 0u64;
            if nat_eq(a2484, a2485) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2487 = f57(v6, v2.clone(), 0u64);
                    let a2488 = match v3.clone() {
                        Adt6::C0(_, _, h2486, _) => {
                            h2486
                        }
                    };
                    nat_add(a2487, a2488)?
                };
                let v8: u64 = {
                    let a2496 = {
                        let a2493 = {
                            let a2490 = v7;
                            let a2491 = match v3.clone() {
                                Adt6::C0(_, h2489, _, _) => {
                                    h2489
                                }
                            };
                            nat_add(a2490, a2491)?
                        };
                        let a2494 = match v3.clone() {
                            Adt6::C0(h2492, _, _, _) => {
                                h2492
                            }
                        };
                        nat_sub(a2493, a2494)
                    };
                    let a2497 = match v3.clone() {
                        Adt6::C0(_, h2495, _, _) => {
                            h2495
                        }
                    };
                    let a2498 = 0u64;
                    nat_quot(a2496, a2497, a2498)
                };
                let v9: u64 = {
                    let a2504 = v8;
                    let a2505 = {
                        let a2501 = {
                            let a2499 = v5;
                            let a2500 = 1u64;
                            nat_sub(a2499, a2500)
                        };
                        let a2502 = 2u64;
                        let a2503 = 0u64;
                        nat_quot(a2501, a2502, a2503)
                    };
                    nat_add(a2504, a2505)?
                };
                let v10: u64 = {
                    let a2507 = v9;
                    let a2508 = match v3.clone() {
                        Adt6::C0(_, h2506, _, _) => {
                            h2506
                        }
                    };
                    nat_mul(a2507, a2508)?
                };
                let v11: u64 = {
                    let a2509 = v7;
                    let a2510 = v10;
                    nat_sub(a2509, a2510)
                };
                let a2512 = v9;
                let a2513 = match v1.clone() {
                    Adt3::C0(_, h2511, _, _, _, _, _, _) => {
                        h2511
                    }
                };
                if if if nat_lt(a2512, a2513) {
                    let a2514 = v10;
                    let a2515 = v7;
                    nat_le(a2514, a2515)
                } else {
                    false
                } {
                    let a2517 = v11;
                    let a2518 = match v3.clone() {
                        Adt6::C0(h2516, _, _, _) => {
                            h2516
                        }
                    };
                    nat_lt(a2517, a2518)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f56(v1, {
                        let a2522 = {
                            let a2520 = f57(v6, v2, 1u64);
                            let a2521 = match v3 {
                                Adt6::C0(h2519, _, _, _) => {
                                    h2519
                                }
                            };
                            nat_mul(a2520, a2521)?
                        };
                        let a2523 = v11;
                        nat_add(a2522, a2523)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f57(v0, v2.clone(), 0u64);
    let v5: u64 = f57(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2525 = v5;
        let a2526 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2524, _) => {
                h2524
            }
        };
        let a2527 = 0u64;
        nat_rem(a2525, a2526, a2527)
    };
    let v7: u64 = {
        let a2529 = v5;
        let a2530 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2528, _) => {
                h2528
            }
        };
        let a2531 = 0u64;
        nat_quot(a2529, a2530, a2531)
    };
    let v8: Option<u64> = f59(match v3.clone() {
        Adt5::C0(h2532, _, _, _, _, _, _, _) => {
            h2532
        }
    }, match v1.clone() {
        Adt3::C0(h2533, _, _, _, _, _, _, _) => {
            h2533
        }
    }, f57(v0, v2.clone(), 1u64), {
        let a2537 = v4;
        let a2538 = {
            let m2536 = match v3.clone() {
                Adt5::C0(h2535, _, _, _, _, _, _, _) => {
                    h2535
                }
            };
            match m2536 {
                Adt6::C0(h2534, _, _, _) => {
                    h2534
                }
            }
        };
        let a2539 = 0u64;
        nat_rem(a2537, a2538, a2539)
    })?;
    let v9: u64 = {
        let a2543 = v4;
        let a2544 = {
            let m2542 = match v3.clone() {
                Adt5::C0(h2541, _, _, _, _, _, _, _) => {
                    h2541
                }
            };
            match m2542 {
                Adt6::C0(h2540, _, _, _) => {
                    h2540
                }
            }
        };
        let a2545 = 0u64;
        nat_quot(a2543, a2544, a2545)
    };
    let v10: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, h2546, _, _, _, _, _, _) => {
            h2546
        }
    }, match v1.clone() {
        Adt3::C0(_, h2547, _, _, _, _, _, _) => {
            h2547
        }
    }, f57(v0, v2.clone(), 2u64), {
        let a2551 = v9;
        let a2552 = {
            let m2550 = match v3.clone() {
                Adt5::C0(_, h2549, _, _, _, _, _, _) => {
                    h2549
                }
            };
            match m2550 {
                Adt6::C0(h2548, _, _, _) => {
                    h2548
                }
            }
        };
        let a2553 = 0u64;
        nat_rem(a2551, a2552, a2553)
    })?;
    let v11: u64 = {
        let a2557 = v9;
        let a2558 = {
            let m2556 = match v3.clone() {
                Adt5::C0(_, h2555, _, _, _, _, _, _) => {
                    h2555
                }
            };
            match m2556 {
                Adt6::C0(h2554, _, _, _) => {
                    h2554
                }
            }
        };
        let a2559 = 0u64;
        nat_quot(a2557, a2558, a2559)
    };
    let v12: Option<u64> = f59(match v3.clone() {
        Adt5::C0(_, _, h2560, _, _, _, _, _) => {
            h2560
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2561, _, _, _, _, _) => {
            h2561
        }
    }, v6, {
        let a2565 = v11;
        let a2566 = {
            let m2564 = match v3.clone() {
                Adt5::C0(_, _, h2563, _, _, _, _, _) => {
                    h2563
                }
            };
            match m2564 {
                Adt6::C0(h2562, _, _, _) => {
                    h2562
                }
            }
        };
        let a2567 = 0u64;
        nat_rem(a2565, a2566, a2567)
    })?;
    let v13: u64 = {
        let a2571 = v11;
        let a2572 = {
            let m2570 = match v3.clone() {
                Adt5::C0(_, _, h2569, _, _, _, _, _) => {
                    h2569
                }
            };
            match m2570 {
                Adt6::C0(h2568, _, _, _) => {
                    h2568
                }
            }
        };
        let a2573 = 0u64;
        nat_quot(a2571, a2572, a2573)
    };
    let v17: Adt12 = {
        let m2574 = v8;
        match m2574 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2575 = v10;
                match m2575 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2576 = v12;
                        match m2576 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2585 = {
                                        let a2580 = f56(v1, v14, v15, v16, 0u64)?;
                                        let a2581 = {
                                            let a2578 = v7;
                                            let a2579 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2577, _, _, _) => {
                                                    h2577
                                                }
                                            };
                                            nat_mul(a2578, a2579)?
                                        };
                                        nat_add(a2580, a2581)?
                                    };
                                    let a2586 = {
                                        let a2583 = v13;
                                        let a2584 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2582, _, _, _, _) => {
                                                h2582
                                            }
                                        };
                                        nat_mul(a2583, a2584)?
                                    };
                                    nat_add(a2585, a2586)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2587, _, _) => {
            h2587
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2588) => {
            h2588
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f88(v0, v2)?), {
        let a2590 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2589, _, _) => {
                h2589
            }
        };
        let a2591 = 0u64;
        if nat_eq(a2590, a2591) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2593 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2592) => {
                h2592
            }
        };
        let a2594 = 0u64;
        if nat_eq(a2593, a2594) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f81(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2607 = f183(v0, v1, match v2.clone() {
        Adt3::C0(h2595, _, _, _, _, _, _, _) => {
            h2595
        }
    }, match v2.clone() {
        Adt3::C0(_, h2596, _, _, _, _, _, _) => {
            h2596
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2597, _, _, _, _, _) => {
            h2597
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2598, _, _, _, _) => {
            h2598
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2599, _, _, _) => {
            h2599
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2600, _, _) => {
            h2600
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2601, _) => {
            h2601
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2602) => {
            h2602
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2603, _, _, _) => {
            h2603
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2604, _, _) => {
            h2604
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2605, _) => {
            h2605
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2606) => {
            h2606
        }
    }, v4)?;
    match m2607 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f19(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2610 = v6;
        let a2611 = {
            let a2608 = 3u64;
            let a2609 = v5;
            nat_add(a2608, a2609)?
        };
        nat_sub(a2610, a2611)
    };
    let a2613 = v0;
    let a2614 = f49(v3, match v1.clone() {
        Adt3::C0(h2612, _, _, _, _, _, _, _) => {
            h2612
        }
    }, v5, v8)?;
    if nat_lt(a2613, a2614) {
        let v9: u64 = f97(match v1.clone() {
            Adt3::C0(h2615, _, _, _, _, _, _, _) => {
                h2615
            }
        }, v5, v8)?;
        let v10: u64 = f98(v3, match v1.clone() {
            Adt3::C0(h2616, _, _, _, _, _, _, _) => {
                h2616
            }
        }, v5)?;
        let a2617 = v0;
        let a2618 = v9;
        if nat_lt(a2617, a2618) {
            f184(v0, v2, f43(v3), {
                let a2619 = 3u64;
                let a2620 = v5;
                nat_add(a2619, a2620)?
            }, v7)
        } else {
            let a2623 = v0;
            let a2624 = {
                let a2621 = v9;
                let a2622 = v10;
                nat_add(a2621, a2622)?
            };
            if nat_lt(a2623, a2624) {
                let v11: u64 = {
                    let a2625 = v0;
                    let a2626 = v9;
                    nat_sub(a2625, a2626)
                };
                let v12: u64 = f185(v3, match v1.clone() {
                    Adt3::C0(h2627, _, _, _, _, _, _, _) => {
                        h2627
                    }
                })?;
                f186({
                    let a2628 = v11;
                    let a2629 = v12;
                    let a2630 = 0u64;
                    nat_rem(a2628, a2629, a2630)
                }, {
                    let a2631 = v11;
                    let a2632 = v12;
                    let a2633 = 0u64;
                    nat_quot(a2631, a2632, a2633)
                }, v1, v3, v4, v7)
            } else {
                f187({
                    let a2636 = v0;
                    let a2637 = {
                        let a2634 = v9;
                        let a2635 = v10;
                        nat_add(a2634, a2635)?
                    };
                    nat_sub(a2636, a2637)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f83(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2638 = v0;
    let a2639 = f35(v1)?;
    if nat_lt(a2638, a2639) {
        let a2640 = v0;
        let a2641 = 4u64;
        if nat_lt(a2640, a2641) {
            let a2642 = v0;
            let a2643 = 0u64;
            if nat_eq(a2642, a2643) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2644 = v0;
                let a2645 = 1u64;
                if nat_eq(a2644, a2645) {
                    let a2646 = v5;
                    let a2647 = 0u64;
                    if nat_eq(a2646, a2647) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2648 = v0;
                    let a2649 = 2u64;
                    if nat_eq(a2648, a2649) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2650 = v0;
            let a2651 = 4u64;
            if if nat_eq(a2650, a2651) {
                let a2652 = f188(v5)?;
                let a2653 = v1;
                !nat_eq(a2652, a2653)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2656 = {
                        let a2654 = v0;
                        let a2655 = 4u64;
                        nat_sub(a2654, a2655)
                    };
                    let a2657 = 3u64;
                    let a2658 = 0u64;
                    nat_quot(a2656, a2657, a2658)
                };
                let v7: u64 = {
                    let a2661 = {
                        let a2659 = v0;
                        let a2660 = 4u64;
                        nat_sub(a2659, a2660)
                    };
                    let a2662 = 3u64;
                    let a2663 = 0u64;
                    nat_rem(a2661, a2662, a2663)
                };
                let a2664 = v7;
                let a2665 = 0u64;
                if nat_eq(a2664, a2665) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2666 = v7;
                    let a2667 = 1u64;
                    if nat_eq(a2666, a2667) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f34(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f84(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2668 = v1;
        let a2669 = 5u64;
        nat_eq(a2668, a2669)
    } else {
        false
    } {
        let a2670 = v2;
        let a2671 = 16u64;
        nat_eq(a2670, a2671)
    } else {
        false
    } {
        let a2672 = v3;
        let a2673 = 0u64;
        nat_eq(a2672, a2673)
    } else {
        false
    } {
        let a2674 = v4;
        let a2675 = 0u64;
        nat_eq(a2674, a2675)
    } else {
        false
    }
}

pub fn f85(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2676 = v1;
    let a2677 = 0u64;
    if nat_eq(a2676, a2677) {
        if v0 {
            let a2678 = v3;
            let a2679 = 0u64;
            if if if nat_eq(a2678, a2679) {
                let a2680 = v4;
                let a2681 = 43u64;
                nat_eq(a2680, a2681)
            } else {
                false
            } {
                true
            } else {
                let a2682 = v3;
                let a2683 = 43u64;
                if nat_eq(a2682, a2683) {
                    let a2684 = v4;
                    let a2685 = 0u64;
                    nat_eq(a2684, a2685)
                } else {
                    false
                }
            } {
                f160(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2686 = v1;
        let a2687 = 1u64;
        if nat_eq(a2686, a2687) {
            Adt11::C1(v2)
        } else {
            let a2688 = v1;
            let a2689 = 2u64;
            if nat_eq(a2688, a2689) {
                Adt11::C5(v2)
            } else {
                let a2690 = v1;
                let a2691 = 3u64;
                if nat_eq(a2690, a2691) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2692 = v1;
                    let a2693 = 4u64;
                    if nat_eq(a2692, a2693) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f84(v0, v1, v2, v3, v4) {
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

pub fn f86(v0: u64, v1: u64) -> Adt12 {
    let a2694 = v0;
    let a2695 = 0u64;
    if nat_eq(a2694, a2695) {
        Adt12::C0(0u64, v1)
    } else {
        let a2696 = v0;
        let a2697 = 1u64;
        if nat_eq(a2696, a2697) {
            Adt12::C1(v1)
        } else {
            let a2698 = v0;
            let a2699 = 2u64;
            if nat_eq(a2698, a2699) {
                Adt12::C2(v1)
            } else {
                let a2700 = v0;
                let a2701 = 3u64;
                if nat_eq(a2700, a2701) {
                    Adt12::C3(v1)
                } else {
                    let a2702 = v0;
                    let a2703 = 13u64;
                    if nat_lt(a2702, a2703) {
                        Adt12::C0({
                            let a2704 = v0;
                            let a2705 = 3u64;
                            nat_sub(a2704, a2705)
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
    let a2706 = v0;
    let a2707 = 1u64;
    if nat_eq(a2706, a2707) {
        Adt13::C0(v1)
    } else {
        let a2708 = v0;
        let a2709 = 3u64;
        if if nat_eq(a2708, a2709) {
            true
        } else {
            let a2710 = v0;
            let a2711 = 6u64;
            nat_eq(a2710, a2711)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f88(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2713 = v0;
        let a2714 = match v1.clone() {
            Adt3::C0(h2712, _, _, _, _, _, _, _) => {
                h2712
            }
        };
        let a2715 = 0u64;
        nat_quot(a2713, a2714, a2715)
    };
    let v3: u64 = {
        let a2717 = v2;
        let a2718 = match v1.clone() {
            Adt3::C0(_, h2716, _, _, _, _, _, _) => {
                h2716
            }
        };
        let a2719 = 0u64;
        nat_quot(a2717, a2718, a2719)
    };
    f135({
        let a2721 = v0;
        let a2722 = match v1.clone() {
            Adt3::C0(h2720, _, _, _, _, _, _, _) => {
                h2720
            }
        };
        let a2723 = 0u64;
        nat_rem(a2721, a2722, a2723)
    }, {
        let a2725 = v2;
        let a2726 = match v1.clone() {
            Adt3::C0(_, h2724, _, _, _, _, _, _) => {
                h2724
            }
        };
        let a2727 = 0u64;
        nat_rem(a2725, a2726, a2727)
    }, {
        let a2729 = v3;
        let a2730 = match v1.clone() {
            Adt3::C0(_, _, h2728, _, _, _, _, _) => {
                h2728
            }
        };
        let a2731 = 0u64;
        nat_rem(a2729, a2730, a2731)
    }, {
        let a2733 = v3;
        let a2734 = match v1.clone() {
            Adt3::C0(_, _, h2732, _, _, _, _, _) => {
                h2732
            }
        };
        let a2735 = 0u64;
        nat_quot(a2733, a2734, a2735)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2736, _, _, _) => {
            h2736
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2737, _, _) => {
            h2737
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2738, _) => {
            h2738
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2739) => {
            h2739
        }
    })
}

pub fn f89(v0: Adt6, v1: u64) -> R<u64> {
    let a2741 = 0u64;
    let a2742 = match v0.clone() {
        Adt6::C0(h2740, _, _, _) => {
            h2740
        }
    };
    if if if if if nat_lt(a2741, a2742) {
        let a2744 = 0u64;
        let a2745 = match v0.clone() {
            Adt6::C0(_, h2743, _, _) => {
                h2743
            }
        };
        nat_lt(a2744, a2745)
    } else {
        false
    } {
        let a2747 = 0u64;
        let a2748 = match v0.clone() {
            Adt6::C0(_, _, _, h2746) => {
                h2746
            }
        };
        nat_lt(a2747, a2748)
    } else {
        false
    } {
        let a2750 = match v0.clone() {
            Adt6::C0(_, h2749, _, _) => {
                h2749
            }
        };
        let a2751 = 2147483648u64;
        nat_lt(a2750, a2751)
    } else {
        false
    } {
        let a2753 = match v0.clone() {
            Adt6::C0(_, _, _, h2752) => {
                h2752
            }
        };
        let a2754 = 2147483648u64;
        nat_lt(a2753, a2754)
    } else {
        false
    } {
        let v2: u64 = {
            let a2758 = v1;
            let a2759 = {
                let a2756 = 2u64;
                let a2757 = f137(match v0.clone() {
                    Adt6::C0(_, _, h2755, _) => {
                        h2755
                    }
                });
                nat_mul(a2756, a2757)?
            };
            nat_add(a2758, a2759)?
        };
        let v3: u64 = {
            let a2771 = {
                let a2766 = 1u64;
                let a2767 = {
                    let a2764 = match v0.clone() {
                        Adt6::C0(_, _, _, h2760) => {
                            h2760
                        }
                    };
                    let a2765 = {
                        let a2762 = match v0.clone() {
                            Adt6::C0(h2761, _, _, _) => {
                                h2761
                            }
                        };
                        let a2763 = 1u64;
                        nat_sub(a2762, a2763)
                    };
                    nat_mul(a2764, a2765)?
                };
                nat_add(a2766, a2767)?
            };
            let a2772 = {
                let a2769 = 2u64;
                let a2770 = f136(match v0.clone() {
                    Adt6::C0(_, _, h2768, _) => {
                        h2768
                    }
                });
                nat_mul(a2769, a2770)?
            };
            nat_add(a2771, a2772)?
        };
        let a2773 = v3;
        let a2774 = v2;
        if nat_le(a2773, a2774) {
            let a2781 = 1u64;
            let a2782 = {
                let a2778 = {
                    let a2775 = v2;
                    let a2776 = v3;
                    nat_sub(a2775, a2776)
                };
                let a2779 = match v0 {
                    Adt6::C0(_, h2777, _, _) => {
                        h2777
                    }
                };
                let a2780 = 0u64;
                nat_quot(a2778, a2779, a2780)
            };
            nat_add(a2781, a2782)
        } else {
            let a2786 = {
                let a2783 = v3;
                let a2784 = v2;
                nat_sub(a2783, a2784)
            };
            let a2787 = match v0 {
                Adt6::C0(_, h2785, _, _) => {
                    h2785
                }
            };
            if nat_lt(a2786, a2787) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f90(v0: u64) -> u64 {
    let v1: u64 = {
        let a2788 = v0;
        let a2789 = 4294967296u64;
        let a2790 = 0u64;
        nat_rem(a2788, a2789, a2790)
    };
    let a2791 = v1;
    let a2792 = 2147483648u64;
    if nat_lt(a2791, a2792) {
        v1
    } else {
        0u64
    }
}

pub fn f91(v0: u64) -> R<u64> {
    let a2795 = 2u64;
    let a2796 = {
        let a2793 = 2u64;
        let a2794 = v0;
        nat_mul(a2793, a2794)?
    };
    nat_add(a2795, a2796)
}

pub fn f92(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2798 = match v1.clone() {
        Adt14::C0(h2797, _, _, _, _, _, _, _, _, _, _) => {
            h2797
        }
    };
    let a2799 = 0u64;
    if if if if nat_eq(a2798, a2799) {
        let a2801 = match v2.clone() {
            Adt14::C0(h2800, _, _, _, _, _, _, _, _, _, _) => {
                h2800
            }
        };
        let a2802 = 0u64;
        nat_eq(a2801, a2802)
    } else {
        false
    } {
        f189(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f190(match v1.clone() {
            Adt14::C0(_, h2803, _, _, _, _, _, _, _, _, _) => {
                h2803
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2804, _, _, _, _, _, _, _, _) => {
                h2804
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2805, _, _, _, _, _, _, _) => {
                h2805
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2806, _, _, _, _, _, _) => {
                h2806
            }
        }) {
            true
        } else {
            if if if if f44(v0) {
                f191(v1.clone(), v2.clone())
            } else {
                false
            } {
                f192(v1.clone())
            } else {
                false
            } {
                f192(v2.clone())
            } else {
                false
            } {
                if f193(v1.clone(), v2.clone())? {
                    true
                } else {
                    f194(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f190(match v1.clone() {
            Adt14::C0(_, h2807, _, _, _, _, _, _, _, _, _) => {
                h2807
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2808, _, _, _, _, _, _, _, _) => {
                h2808
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2809, _, _, _, _, _, _, _) => {
                h2809
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2810, _, _, _, _, _, _) => {
                h2810
            }
        }) {
            0u64
        } else {
            let a2816 = match v1.clone() {
                Adt14::C0(_, _, h2811, _, _, _, _, _, _, _, _) => {
                    h2811
                }
            };
            let a2817 = {
                let a2814 = match v1.clone() {
                    Adt14::C0(_, _, _, h2812, _, _, _, _, _, _, _) => {
                        h2812
                    }
                };
                let a2815 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2813, _, _, _, _, _, _) => {
                        h2813
                    }
                };
                nat_mul(a2814, a2815)?
            };
            nat_mul(a2816, a2817)?
        };
        let v4: u64 = {
            let a2818 = v3;
            let a2819 = 0u64;
            if if nat_eq(a2818, a2819) {
                true
            } else {
                f195(v2.clone())?
            } {
                0u64
            } else {
                let a2821 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2820, _) => {
                        h2820
                    }
                };
                let a2822 = 4u64;
                let a2823 = 0u64;
                nat_quot(a2821, a2822, a2823)
            }
        };
        Ok(Adt15::C1(f196(f197(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2824, _, _, _, _, _) => {
                h2824
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2825, _, _, _, _, _) => {
                h2825
            }
        }), match v1 {
            Adt14::C0(_, h2826, _, _, _, _, _, _, _, _, _) => {
                h2826
            }
        }, v3, v4)?, 4u64, 64u64, v3, f198(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f93(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f199(v3.clone());
    let a2828 = match v2.clone() {
        Adt14::C0(h2827, _, _, _, _, _, _, _, _, _, _) => {
            h2827
        }
    };
    let a2829 = 0u64;
    if if if if nat_eq(a2828, a2829) {
        let a2831 = match v3.clone() {
            Adt14::C0(h2830, _, _, _, _, _, _, _, _, _, _) => {
                h2830
            }
        };
        let a2832 = 0u64;
        nat_eq(a2831, a2832)
    } else {
        false
    } {
        f189(v2.clone(), v3)
    } else {
        false
    } {
        if f190(match v2.clone() {
            Adt14::C0(_, h2833, _, _, _, _, _, _, _, _, _) => {
                h2833
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2834, _, _, _, _, _, _, _, _) => {
                h2834
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2835, _, _, _, _, _, _, _) => {
                h2835
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2836, _, _, _, _, _, _) => {
                h2836
            }
        }) {
            true
        } else {
            let a2838 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2837, _, _, _, _, _) => {
                    h2837
                }
            };
            let a2839 = 4u64;
            if if if if if nat_eq(a2838, a2839) {
                f200(match v2.clone() {
                    Adt14::C0(_, h2840, _, _, _, _, _, _, _, _, _) => {
                        h2840
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2841, _, _, _, _, _, _, _, _) => {
                        h2841
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2842, _, _, _, _, _, _, _) => {
                        h2842
                    }
                }, v0)?
            } else {
                false
            } {
                f192(v2.clone())
            } else {
                false
            } {
                f192(v4.clone())
            } else {
                false
            } {
                let a2844 = f201(v4.clone())?;
                let a2845 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2843, _) => {
                        h2843
                    }
                };
                nat_le(a2844, a2845)
            } else {
                false
            } {
                if f193(v2.clone(), v4.clone())? {
                    true
                } else {
                    f194(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f190(match v2.clone() {
            Adt14::C0(_, h2846, _, _, _, _, _, _, _, _, _) => {
                h2846
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2847, _, _, _, _, _, _, _, _) => {
                h2847
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2848, _, _, _, _, _, _, _) => {
                h2848
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2849, _, _, _, _, _, _) => {
                h2849
            }
        }) {
            true
        } else {
            f195(v4.clone())?
        } {
            0u64
        } else {
            let a2851 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2850, _) => {
                    h2850
                }
            };
            let a2852 = 4u64;
            let a2853 = 0u64;
            nat_quot(a2851, a2852, a2853)
        };
        Ok(Adt15::C1(f202(match v2.clone() {
            Adt14::C0(_, h2854, _, _, _, _, _, _, _, _, _) => {
                h2854
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2855, _, _, _, _, _, _, _, _) => {
                h2855
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2856, _, _, _, _, _, _, _) => {
                h2856
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2857, _, _, _, _, _, _) => {
                h2857
            }
        }, v0, v5)?, 4u64, 65u64, v0, f198(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f94(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f199(v4.clone());
    if if if if if if if v1 {
        let a2859 = match v2.clone() {
            Adt14::C0(h2858, _, _, _, _, _, _, _, _, _, _) => {
                h2858
            }
        };
        let a2860 = 0u64;
        nat_eq(a2859, a2860)
    } else {
        false
    } {
        let a2862 = match v3.clone() {
            Adt14::C0(h2861, _, _, _, _, _, _, _, _, _, _) => {
                h2861
            }
        };
        let a2863 = 0u64;
        nat_eq(a2862, a2863)
    } else {
        false
    } {
        let a2865 = match v4.clone() {
            Adt14::C0(h2864, _, _, _, _, _, _, _, _, _, _) => {
                h2864
            }
        };
        let a2866 = 0u64;
        nat_eq(a2865, a2866)
    } else {
        false
    } {
        f189(v2.clone(), v3.clone())
    } else {
        false
    } {
        f189(v2.clone(), v4)
    } else {
        false
    } {
        if f190(match v2.clone() {
            Adt14::C0(_, h2867, _, _, _, _, _, _, _, _, _) => {
                h2867
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2868, _, _, _, _, _, _, _, _) => {
                h2868
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2869, _, _, _, _, _, _, _) => {
                h2869
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2870, _, _, _, _, _, _) => {
                h2870
            }
        }) {
            true
        } else {
            let a2872 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2871, _, _, _, _, _) => {
                    h2871
                }
            };
            let a2873 = 4u64;
            if if if if if if if nat_eq(a2872, a2873) {
                let a2875 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2874, _, _, _, _, _) => {
                        h2874
                    }
                };
                let a2876 = 4u64;
                nat_eq(a2875, a2876)
            } else {
                false
            } {
                f192(v2.clone())
            } else {
                false
            } {
                f192(v3.clone())
            } else {
                false
            } {
                f192(v5.clone())
            } else {
                false
            } {
                let a2878 = f201(v5.clone())?;
                let a2879 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2877, _) => {
                        h2877
                    }
                };
                nat_le(a2878, a2879)
            } else {
                false
            } {
                if f193(v2.clone(), v5.clone())? {
                    true
                } else {
                    f194(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f193(v3.clone(), v5.clone())? {
                    true
                } else {
                    f194(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f190(match v2.clone() {
            Adt14::C0(_, h2880, _, _, _, _, _, _, _, _, _) => {
                h2880
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2881, _, _, _, _, _, _, _, _) => {
                h2881
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2882, _, _, _, _, _, _, _) => {
                h2882
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2883, _, _, _, _, _, _) => {
                h2883
            }
        }) {
            0u64
        } else {
            let a2889 = match v2.clone() {
                Adt14::C0(_, _, h2884, _, _, _, _, _, _, _, _) => {
                    h2884
                }
            };
            let a2890 = {
                let a2887 = match v2.clone() {
                    Adt14::C0(_, _, _, h2885, _, _, _, _, _, _, _) => {
                        h2885
                    }
                };
                let a2888 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2886, _, _, _, _, _, _) => {
                        h2886
                    }
                };
                nat_mul(a2887, a2888)?
            };
            nat_mul(a2889, a2890)?
        };
        let v7: u64 = {
            let a2891 = v6;
            let a2892 = 0u64;
            if if nat_eq(a2891, a2892) {
                true
            } else {
                f195(v5.clone())?
            } {
                0u64
            } else {
                let a2894 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2893, _) => {
                        h2893
                    }
                };
                let a2895 = 4u64;
                let a2896 = 0u64;
                nat_quot(a2894, a2895, a2896)
            }
        };
        Ok(Adt15::C1(f203(match v2 {
            Adt14::C0(_, h2897, _, _, _, _, _, _, _, _, _) => {
                h2897
            }
        }, v6, v7)?, 4u64, 66u64, v6, f198(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f95(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f204(v0, v3, v6) {
        f204(v1, v4, v6)
    } else {
        false
    } {
        f204(v2, v5, v6)
    } else {
        false
    }
}

pub fn f96(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f205(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f205(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f205(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f205(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f205(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f205(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2898 = v2;
    let a2899 = 0u64;
    if nat_eq(a2898, a2899) {
        Ok(0u64)
    } else {
        let a2902 = 2u64;
        let a2903 = {
            let a2900 = v0;
            let a2901 = v1;
            nat_mul(a2900, a2901)?
        };
        nat_mul(a2902, a2903)
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2904 = v2;
    let a2905 = f185(v0, v1)?;
    nat_mul(a2904, a2905)
}

pub fn f99(v0: u64) -> u64 {
    let a2906 = v0;
    let a2907 = 30u64;
    if nat_eq(a2906, a2907) {
        3u64
    } else {
        1u64
    }
}

pub fn f100(v0: u64) -> R<u64> {
    let a2910 = 5u64;
    let a2911 = {
        let a2908 = 4u64;
        let a2909 = v0;
        nat_mul(a2908, a2909)?
    };
    nat_add(a2910, a2911)
}

pub fn f101(v0: u64) -> bool {
    let m2912 = f206(v0);
    match m2912 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a2914 = 1u64;
            let a2915 = match v1 {
                Adt19::C0(_, h2913, _) => {
                    h2913
                }
            };
            nat_lt(a2914, a2915)
        }
    }
}

pub fn f102(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a2917 = 0u64;
        let a2918 = match v0.clone() {
            Adt1::C0(_, h2916, _, _, _, _, _, _, _, _, _, _) => {
                h2916
            }
        };
        nat_lt(a2917, a2918)
    };
    let a2920 = match v1.clone() {
        Adt2::C0(h2919, _, _, _, _, _, _, _, _, _, _) => {
            h2919
        }
    };
    let a2921 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a2920, a2921) {
        let a2923 = match v2.clone() {
            Adt2::C0(h2922, _, _, _, _, _, _, _, _, _, _) => {
                h2922
            }
        };
        let a2924 = 0u64;
        nat_eq(a2923, a2924)
    } else {
        false
    } {
        let a2926 = match v3.clone() {
            Adt2::C0(h2925, _, _, _, _, _, _, _, _, _, _) => {
                h2925
            }
        };
        let a2927 = 0u64;
        nat_eq(a2926, a2927)
    } else {
        false
    } {
        let a2929 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2928, _, _, _, _, _, _) => {
                h2928
            }
        };
        let a2930 = 1u64;
        nat_eq(a2929, a2930)
    } else {
        false
    } {
        let a2932 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2931, _, _, _, _, _, _) => {
                h2931
            }
        };
        let a2933 = 1u64;
        nat_eq(a2932, a2933)
    } else {
        false
    } {
        let a2935 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2934, _, _, _, _, _, _) => {
                h2934
            }
        };
        let a2936 = 1u64;
        nat_eq(a2935, a2936)
    } else {
        false
    } {
        let a2938 = 0u64;
        let a2939 = match v1.clone() {
            Adt2::C0(_, _, h2937, _, _, _, _, _, _, _, _) => {
                h2937
            }
        };
        nat_lt(a2938, a2939)
    } else {
        false
    } {
        let a2941 = match v3.clone() {
            Adt2::C0(_, _, _, h2940, _, _, _, _, _, _, _) => {
                h2940
            }
        };
        let a2942 = 1u64;
        nat_eq(a2941, a2942)
    } else {
        false
    } {
        let a2945 = match v3.clone() {
            Adt2::C0(_, h2943, _, _, _, _, _, _, _, _, _) => {
                h2943
            }
        };
        let a2946 = match v1.clone() {
            Adt2::C0(_, h2944, _, _, _, _, _, _, _, _, _) => {
                h2944
            }
        };
        nat_eq(a2945, a2946)
    } else {
        false
    } {
        let a2949 = match v3.clone() {
            Adt2::C0(_, _, h2947, _, _, _, _, _, _, _, _) => {
                h2947
            }
        };
        let a2950 = match v1.clone() {
            Adt2::C0(_, _, _, h2948, _, _, _, _, _, _, _) => {
                h2948
            }
        };
        nat_eq(a2949, a2950)
    } else {
        false
    } {
        if v4 {
            f20(v1.clone(), v2.clone())
        } else {
            let a2953 = match v2.clone() {
                Adt2::C0(_, h2951, _, _, _, _, _, _, _, _, _) => {
                    h2951
                }
            };
            let a2954 = match v1.clone() {
                Adt2::C0(_, _, h2952, _, _, _, _, _, _, _, _) => {
                    h2952
                }
            };
            if if nat_eq(a2953, a2954) {
                let a2957 = match v2.clone() {
                    Adt2::C0(_, _, h2955, _, _, _, _, _, _, _, _) => {
                        h2955
                    }
                };
                let a2958 = match v1.clone() {
                    Adt2::C0(_, _, _, h2956, _, _, _, _, _, _, _) => {
                        h2956
                    }
                };
                nat_eq(a2957, a2958)
            } else {
                false
            } {
                let a2960 = match v2.clone() {
                    Adt2::C0(_, _, _, h2959, _, _, _, _, _, _, _) => {
                        h2959
                    }
                };
                let a2961 = 1u64;
                nat_eq(a2960, a2961)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f25(v1.clone(), f22(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f25(v2.clone(), f22(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f30(v3.clone())?
    } else {
        false
    } {
        f29(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f29(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2963 = f22(v3.clone(), 1u64)?;
            let a2964 = f162(match v1.clone() {
                Adt2::C0(_, _, h2962, _, _, _, _, _, _, _, _) => {
                    h2962
                }
            }, v4)?;
            nat_mul(a2963, a2964)?
        }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C36(f24(v2, 1u64), v4, match v0 {
            Adt1::C0(h2965, _, _, _, _, _, _, _, _, _, _, _) => {
                h2965
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f103(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h2966, _, _, _, _, _, _, _, _, _) => {
            h2966
        }
    }, match v0.clone() {
        Adt1::C0(h2967, _, _, _, _, _, _, _, _, _, _, _) => {
            h2967
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h2968, _, _, _, _, _, _, _, _, _) => {
            h2968
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h2969, _, _, _, _, _, _, _) => {
            h2969
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h2970, _, _, _, _, _, _, _, _) => {
            h2970
        }
    }, match v0.clone() {
        Adt1::C0(_, h2971, _, _, _, _, _, _, _, _, _, _) => {
            h2971
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h2972, _, _, _, _, _, _, _, _) => {
            h2972
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h2973, _, _, _, _, _, _) => {
            h2973
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h2974, _, _, _, _, _, _, _, _, _, _) => {
            h2974
        }
    };
    let v7: bool = {
        let a2977 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h2975, _, _, _) => {
                h2975
            }
        };
        let a2978 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2976, _, _, _, _, _) => {
                h2976
            }
        };
        nat_lt(a2977, a2978)
    };
    let a2979 = v6;
    let a2980 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a2979, a2980) {
        true
    } else {
        let a2981 = v6;
        let a2982 = 1u64;
        nat_eq(a2981, a2982)
    } {
        let a2984 = match v2.clone() {
            Adt2::C0(h2983, _, _, _, _, _, _, _, _, _, _) => {
                h2983
            }
        };
        let a2985 = 0u64;
        nat_eq(a2984, a2985)
    } else {
        false
    } {
        let a2987 = match v3.clone() {
            Adt2::C0(h2986, _, _, _, _, _, _, _, _, _, _) => {
                h2986
            }
        };
        let a2988 = 0u64;
        nat_eq(a2987, a2988)
    } else {
        false
    } {
        let a2990 = match v1.clone() {
            Adt2::C0(_, _, _, h2989, _, _, _, _, _, _, _) => {
                h2989
            }
        };
        let a2991 = 1u64;
        nat_eq(a2990, a2991)
    } else {
        false
    } {
        let a2994 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2992, _, _, _, _, _, _) => {
                h2992
            }
        };
        let a2995 = match v2.clone() {
            Adt2::C0(_, _, _, h2993, _, _, _, _, _, _, _) => {
                h2993
            }
        };
        nat_eq(a2994, a2995)
    } else {
        false
    } {
        let a2998 = match v3.clone() {
            Adt2::C0(_, _, _, h2996, _, _, _, _, _, _, _) => {
                h2996
            }
        };
        let a2999 = match v2.clone() {
            Adt2::C0(_, _, _, h2997, _, _, _, _, _, _, _) => {
                h2997
            }
        };
        nat_eq(a2998, a2999)
    } else {
        false
    } {
        let a3002 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3000, _, _, _, _, _, _) => {
                h3000
            }
        };
        let a3003 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3001, _, _, _, _, _, _) => {
                h3001
            }
        };
        nat_eq(a3002, a3003)
    } else {
        false
    } {
        let a3005 = 0u64;
        let a3006 = match v4.clone() {
            Adt6::C0(h3004, _, _, _) => {
                h3004
            }
        };
        if if if if if nat_lt(a3005, a3006) {
            let a3008 = 0u64;
            let a3009 = match v4.clone() {
                Adt6::C0(_, h3007, _, _) => {
                    h3007
                }
            };
            nat_lt(a3008, a3009)
        } else {
            false
        } {
            let a3011 = match v4.clone() {
                Adt6::C0(_, h3010, _, _) => {
                    h3010
                }
            };
            let a3012 = 2147483648u64;
            nat_lt(a3011, a3012)
        } else {
            false
        } {
            let a3014 = 0u64;
            let a3015 = match v4.clone() {
                Adt6::C0(_, _, _, h3013) => {
                    h3013
                }
            };
            nat_lt(a3014, a3015)
        } else {
            false
        } {
            let a3017 = match v4.clone() {
                Adt6::C0(_, _, _, h3016) => {
                    h3016
                }
            };
            let a3018 = 2147483648u64;
            nat_lt(a3017, a3018)
        } else {
            false
        } {
            let a3021 = match v3.clone() {
                Adt2::C0(_, h3019, _, _, _, _, _, _, _, _, _) => {
                    h3019
                }
            };
            let a3022 = f89(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3020, _, _, _, _, _, _, _, _, _) => {
                    h3020
                }
            })?;
            nat_eq(a3021, a3022)
        } else {
            false
        }
    } else {
        false
    } {
        let a3024 = 0u64;
        let a3025 = match v5.clone() {
            Adt6::C0(h3023, _, _, _) => {
                h3023
            }
        };
        if if if if if nat_lt(a3024, a3025) {
            let a3027 = 0u64;
            let a3028 = match v5.clone() {
                Adt6::C0(_, h3026, _, _) => {
                    h3026
                }
            };
            nat_lt(a3027, a3028)
        } else {
            false
        } {
            let a3030 = match v5.clone() {
                Adt6::C0(_, h3029, _, _) => {
                    h3029
                }
            };
            let a3031 = 2147483648u64;
            nat_lt(a3030, a3031)
        } else {
            false
        } {
            let a3033 = 0u64;
            let a3034 = match v5.clone() {
                Adt6::C0(_, _, _, h3032) => {
                    h3032
                }
            };
            nat_lt(a3033, a3034)
        } else {
            false
        } {
            let a3036 = match v5.clone() {
                Adt6::C0(_, _, _, h3035) => {
                    h3035
                }
            };
            let a3037 = 2147483648u64;
            nat_lt(a3036, a3037)
        } else {
            false
        } {
            let a3040 = match v3.clone() {
                Adt2::C0(_, _, h3038, _, _, _, _, _, _, _, _) => {
                    h3038
                }
            };
            let a3041 = f89(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3039, _, _, _, _, _, _, _, _) => {
                    h3039
                }
            })?;
            nat_eq(a3040, a3041)
        } else {
            false
        }
    } else {
        false
    } {
        f207(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f207(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f207(v1.clone(), {
            let a3042 = v6;
            let a3043 = 0u64;
            if nat_eq(a3042, a3043) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f29(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f29(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3052 = {
                let a3050 = {
                    let a3048 = 3u64;
                    let a3049 = {
                        let a3046 = match v4.clone() {
                            Adt6::C0(h3044, _, _, _) => {
                                h3044
                            }
                        };
                        let a3047 = match v5.clone() {
                            Adt6::C0(h3045, _, _, _) => {
                                h3045
                            }
                        };
                        nat_mul(a3046, a3047)?
                    };
                    nat_mul(a3048, a3049)?
                };
                let a3051 = 2u64;
                nat_add(a3050, a3051)?
            };
            let a3053 = f22(v3.clone(), 1u64)?;
            nat_mul(a3052, a3053)?
        }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C34(f24(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f104(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f20(v0.clone(), v1.clone()) {
        f20(v0.clone(), v2.clone())
    } else {
        false
    } {
        f30(v0.clone())?
    } else {
        false
    } {
        f30(v1.clone())?
    } else {
        false
    } {
        f30(v2.clone())?
    } else {
        false
    } {
        if f29(v0.clone(), v2.clone())? {
            true
        } else {
            f46(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f29(v1.clone(), v2.clone())? {
            true
        } else {
            f46(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3054 = 6u64;
            let a3055 = f22(v0.clone(), 1u64)?;
            nat_mul(a3054, a3055)?
        }, 2u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C33(f24(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f105(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3056 = v2;
    match m3056 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3057, _, _, _, _, _, _, _, _, _) => {
                    h3057
                }
            };
            let v6: u64 = {
                let a3063 = match v1.clone() {
                    Adt2::C0(_, _, h3058, _, _, _, _, _, _, _, _) => {
                        h3058
                    }
                };
                let a3064 = {
                    let a3061 = match v1.clone() {
                        Adt2::C0(_, _, _, h3059, _, _, _, _, _, _, _) => {
                            h3059
                        }
                    };
                    let a3062 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3060, _, _, _, _, _, _) => {
                            h3060
                        }
                    };
                    nat_mul(a3061, a3062)?
                };
                nat_mul(a3063, a3064)?
            };
            if if if if if if if if if if f20(v1.clone(), v4.clone()) {
                f20(v1.clone(), v3.clone())
            } else {
                false
            } {
                f208(v1.clone(), v6)?
            } else {
                false
            } {
                f208(v4.clone(), v6)?
            } else {
                false
            } {
                f208(v3.clone(), v6)?
            } else {
                false
            } {
                if f29(v1.clone(), v3.clone())? {
                    true
                } else {
                    f46(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f29(v4.clone(), v3.clone())? {
                    true
                } else {
                    f46(v4, v3.clone())
                }
            } else {
                false
            } {
                let a3065 = v5;
                let a3066 = 2147483647u64;
                nat_le(a3065, a3066)
            } else {
                false
            } {
                let a3067 = v6;
                let a3068 = 2147483647u64;
                nat_le(a3067, a3068)
            } else {
                false
            } {
                let a3070 = match v0.clone() {
                    Adt1::C0(h3069, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3069
                    }
                };
                let a3071 = 4294967295u64;
                if nat_le(a3070, a3071) {
                    let a3073 = match v0.clone() {
                        Adt1::C0(_, h3072, _, _, _, _, _, _, _, _, _, _) => {
                            h3072
                        }
                    };
                    let a3074 = 4294967295u64;
                    nat_le(a3073, a3074)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3075 = v5;
                    let a3076 = 0u64;
                    if nat_eq(a3075, a3076) {
                        0u64
                    } else {
                        let a3077 = v6;
                        let a3078 = f161(v5)?;
                        nat_mul(a3077, a3078)?
                    }
                }, 4u64, f24(v1, 1u64), f24(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3079, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3079
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f106(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3080, _, _, _, _, _, _, _, _, _, _, _) => {
            h3080
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3081, _, _, _, _, _, _, _, _, _, _) => {
            h3081
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3082, _, _, _, _, _, _, _, _, _) => {
            h3082
        }
    };
    let v6: u64 = {
        let a3084 = match v1.clone() {
            Adt2::C0(_, h3083, _, _, _, _, _, _, _, _, _) => {
                h3083
            }
        };
        let a3085 = v4;
        let a3086 = 0u64;
        nat_quot(a3084, a3085, a3086)
    };
    let v7: u64 = {
        let a3088 = match v1.clone() {
            Adt2::C0(h3087, _, _, _, _, _, _, _, _, _, _) => {
                h3087
            }
        };
        let a3089 = 0u64;
        if nat_eq(a3088, a3089) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f168(match v1.clone() {
        Adt2::C0(h3090, _, _, _, _, _, _, _, _, _, _) => {
            h3090
        }
    }) {
        let a3093 = match v1.clone() {
            Adt2::C0(h3091, _, _, _, _, _, _, _, _, _, _) => {
                h3091
            }
        };
        let a3094 = match v2.clone() {
            Adt2::C0(h3092, _, _, _, _, _, _, _, _, _, _) => {
                h3092
            }
        };
        nat_eq(a3093, a3094)
    } else {
        false
    } {
        let a3095 = 0u64;
        let a3096 = v3;
        nat_lt(a3095, a3096)
    } else {
        false
    } {
        let a3097 = v3;
        let a3098 = 2147483648u64;
        nat_lt(a3097, a3098)
    } else {
        false
    } {
        let a3099 = 0u64;
        let a3100 = v4;
        nat_lt(a3099, a3100)
    } else {
        false
    } {
        let a3101 = v4;
        let a3102 = 2147483648u64;
        nat_lt(a3101, a3102)
    } else {
        false
    } {
        let a3103 = v5;
        let a3104 = 2147483648u64;
        nat_lt(a3103, a3104)
    } else {
        false
    } {
        let a3105 = 0u64;
        let a3106 = v6;
        nat_lt(a3105, a3106)
    } else {
        false
    } {
        let a3108 = 0u64;
        let a3109 = match v2.clone() {
            Adt2::C0(_, h3107, _, _, _, _, _, _, _, _, _) => {
                h3107
            }
        };
        nat_lt(a3108, a3109)
    } else {
        false
    } {
        let a3113 = match v1.clone() {
            Adt2::C0(_, h3110, _, _, _, _, _, _, _, _, _) => {
                h3110
            }
        };
        let a3114 = {
            let a3111 = v6;
            let a3112 = v4;
            nat_mul(a3111, a3112)?
        };
        nat_eq(a3113, a3114)
    } else {
        false
    } {
        let a3116 = match v2.clone() {
            Adt2::C0(_, _, h3115, _, _, _, _, _, _, _, _) => {
                h3115
            }
        };
        let a3117 = v4;
        nat_eq(a3116, a3117)
    } else {
        false
    } {
        let a3130 = {
            let a3123 = {
                let a3121 = match v2.clone() {
                    Adt2::C0(_, h3118, _, _, _, _, _, _, _, _, _) => {
                        h3118
                    }
                };
                let a3122 = {
                    let a3119 = v5;
                    let a3120 = 2u64;
                    nat_mul(a3119, a3120)?
                };
                nat_add(a3121, a3122)?
            };
            let a3124 = v3;
            nat_add(a3123, a3124)?
        };
        let a3131 = {
            let a3128 = {
                let a3126 = match v1.clone() {
                    Adt2::C0(_, _, h3125, _, _, _, _, _, _, _, _) => {
                        h3125
                    }
                };
                let a3127 = v3;
                nat_mul(a3126, a3127)?
            };
            let a3129 = v6;
            nat_add(a3128, a3129)?
        };
        nat_eq(a3130, a3131)
    } else {
        false
    } {
        f209(v1.clone(), v7)?
    } else {
        false
    } {
        f209(v2.clone(), v7)?
    } else {
        false
    } {
        f29(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3132 = f22(v2.clone(), 1u64)?;
            let a3133 = f182(v8.clone())?;
            nat_mul(a3132, a3133)?
        }, v7, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3134, _, _, _, _, _, _, _, _, _, _) => {
                h3134
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3135 = v1;
    match m3135 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3137 = match v0.clone() {
                Adt2::C0(h3136, _, _, _, _, _, _, _, _, _, _) => {
                    h3136
                }
            };
            let a3138 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3137, a3138) {
                let a3140 = match v3.clone() {
                    Adt2::C0(h3139, _, _, _, _, _, _, _, _, _, _) => {
                        h3139
                    }
                };
                let a3141 = 0u64;
                nat_eq(a3140, a3141)
            } else {
                false
            } {
                let a3143 = match v2.clone() {
                    Adt2::C0(h3142, _, _, _, _, _, _, _, _, _, _) => {
                        h3142
                    }
                };
                let a3144 = 0u64;
                nat_eq(a3143, a3144)
            } else {
                false
            } {
                let a3146 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3145, _, _, _, _, _, _) => {
                        h3145
                    }
                };
                let a3147 = 1u64;
                nat_eq(a3146, a3147)
            } else {
                false
            } {
                let a3149 = match v3.clone() {
                    Adt2::C0(_, _, _, h3148, _, _, _, _, _, _, _) => {
                        h3148
                    }
                };
                let a3150 = 1u64;
                nat_eq(a3149, a3150)
            } else {
                false
            } {
                let a3152 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3151, _, _, _, _, _, _) => {
                        h3151
                    }
                };
                let a3153 = 1u64;
                nat_eq(a3152, a3153)
            } else {
                false
            } {
                let a3156 = match v0.clone() {
                    Adt2::C0(_, _, h3154, _, _, _, _, _, _, _, _) => {
                        h3154
                    }
                };
                let a3157 = match v3.clone() {
                    Adt2::C0(_, _, h3155, _, _, _, _, _, _, _, _) => {
                        h3155
                    }
                };
                nat_eq(a3156, a3157)
            } else {
                false
            } {
                let a3162 = match v3.clone() {
                    Adt2::C0(_, h3158, _, _, _, _, _, _, _, _, _) => {
                        h3158
                    }
                };
                let a3163 = {
                    let a3160 = match v0.clone() {
                        Adt2::C0(_, h3159, _, _, _, _, _, _, _, _, _) => {
                            h3159
                        }
                    };
                    let a3161 = 1u64;
                    nat_add(a3160, a3161)?
                };
                nat_le(a3162, a3163)
            } else {
                false
            } {
                let a3166 = match v2.clone() {
                    Adt2::C0(_, h3164, _, _, _, _, _, _, _, _, _) => {
                        h3164
                    }
                };
                let a3167 = match v0.clone() {
                    Adt2::C0(_, _, h3165, _, _, _, _, _, _, _, _) => {
                        h3165
                    }
                };
                nat_eq(a3166, a3167)
            } else {
                false
            } {
                let a3175 = match v2.clone() {
                    Adt2::C0(_, _, h3168, _, _, _, _, _, _, _, _) => {
                        h3168
                    }
                };
                let a3176 = {
                    let a3173 = {
                        let a3170 = match v0.clone() {
                            Adt2::C0(_, h3169, _, _, _, _, _, _, _, _, _) => {
                                h3169
                            }
                        };
                        let a3171 = 1u64;
                        nat_add(a3170, a3171)?
                    };
                    let a3174 = match v3.clone() {
                        Adt2::C0(_, h3172, _, _, _, _, _, _, _, _, _) => {
                            h3172
                        }
                    };
                    nat_sub(a3173, a3174)
                };
                nat_eq(a3175, a3176)
            } else {
                false
            } {
                let a3179 = match v2.clone() {
                    Adt2::C0(_, _, _, h3177, _, _, _, _, _, _, _) => {
                        h3177
                    }
                };
                let a3180 = match v0.clone() {
                    Adt2::C0(_, _, _, h3178, _, _, _, _, _, _, _) => {
                        h3178
                    }
                };
                nat_eq(a3179, a3180)
            } else {
                false
            } {
                let a3182 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3181, _, _, _, _, _, _) => {
                        h3181
                    }
                };
                let a3183 = 1u64;
                nat_eq(a3182, a3183)
            } else {
                false
            } {
                f37(v2.clone())?
            } else {
                false
            } {
                let a3185 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3184, _, _, _, _, _) => {
                        h3184
                    }
                };
                let a3186 = 4u64;
                nat_eq(a3185, a3186)
            } else {
                false
            } {
                let a3188 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3187, _, _, _, _, _) => {
                        h3187
                    }
                };
                let a3189 = 4u64;
                nat_eq(a3188, a3189)
            } else {
                false
            } {
                let a3194 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3190, _, _, _, _) => {
                        h3190
                    }
                };
                let a3195 = {
                    let a3192 = match v0.clone() {
                        Adt2::C0(_, h3191, _, _, _, _, _, _, _, _, _) => {
                            h3191
                        }
                    };
                    let a3193 = 4u64;
                    nat_mul(a3192, a3193)?
                };
                nat_eq(a3194, a3195)
            } else {
                false
            } {
                let a3200 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3196, _, _, _, _) => {
                        h3196
                    }
                };
                let a3201 = {
                    let a3198 = match v3.clone() {
                        Adt2::C0(_, h3197, _, _, _, _, _, _, _, _, _) => {
                            h3197
                        }
                    };
                    let a3199 = 4u64;
                    nat_mul(a3198, a3199)?
                };
                nat_eq(a3200, a3201)
            } else {
                false
            } {
                f29(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f29(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3203 = f22(v2.clone(), 1u64)?;
                    let a3204 = f181(match v3.clone() {
                        Adt2::C0(_, h3202, _, _, _, _, _, _, _, _, _) => {
                            h3202
                        }
                    })?;
                    nat_mul(a3203, a3204)?
                }, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C29(f24(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f108(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h3205, _, _, _, _, _, _, _, _, _, _) => {
            h3205
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h3206, _, _, _, _, _, _, _, _) => {
                h3206
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3207, _, _, _, _, _, _, _, _, _) => {
                h3207
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h3208, _, _, _, _, _, _) => {
                h3208
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h3209, _, _, _, _, _, _, _, _) => {
                h3209
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h3210, _, _, _, _, _, _, _, _, _) => {
                h3210
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h3211, _, _, _, _, _, _, _) => {
                h3211
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h3212, _, _, _, _, _) => {
                h3212
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3214 = match v1.clone() {
        Adt1::C0(h3213, _, _, _, _, _, _, _, _, _, _, _) => {
            h3213
        }
    };
    let a3215 = 1u64;
    if if if if if if if if if if if nat_le(a3214, a3215) {
        let a3217 = match v2.clone() {
            Adt2::C0(h3216, _, _, _, _, _, _, _, _, _, _) => {
                h3216
            }
        };
        let a3218 = 1u64;
        nat_le(a3217, a3218)
    } else {
        false
    } {
        let a3220 = match v3.clone() {
            Adt2::C0(h3219, _, _, _, _, _, _, _, _, _, _) => {
                h3219
            }
        };
        let a3221 = 0u64;
        nat_eq(a3220, a3221)
    } else {
        false
    } {
        let a3226 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3222, _, _, _, _, _) => {
                h3222
            }
        };
        let a3227 = {
            let a3224 = match v2.clone() {
                Adt2::C0(h3223, _, _, _, _, _, _, _, _, _, _) => {
                    h3223
                }
            };
            let a3225 = 0u64;
            if nat_eq(a3224, a3225) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3226, a3227)
    } else {
        false
    } {
        f37(v3.clone())?
    } else {
        false
    } {
        f29(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f210(v4.clone(), match v2.clone() {
            Adt2::C0(_, h3228, _, _, _, _, _, _, _, _, _) => {
                h3228
            }
        }, match v3.clone() {
            Adt2::C0(_, h3229, _, _, _, _, _, _, _, _, _) => {
                h3229
            }
        })?
    } else {
        false
    } {
        if v0 {
            f210(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3230, _, _, _, _, _, _, _, _) => {
                    h3230
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h3231, _, _, _, _, _, _, _, _) => {
                    h3231
                }
            })?
        } else {
            let a3234 = match v2.clone() {
                Adt2::C0(_, _, h3232, _, _, _, _, _, _, _, _) => {
                    h3232
                }
            };
            let a3235 = match v3.clone() {
                Adt2::C0(_, _, h3233, _, _, _, _, _, _, _, _) => {
                    h3233
                }
            };
            nat_eq(a3234, a3235)
        }
    } else {
        false
    } {
        let a3238 = match v2.clone() {
            Adt2::C0(_, _, _, h3236, _, _, _, _, _, _, _) => {
                h3236
            }
        };
        let a3239 = match v3.clone() {
            Adt2::C0(_, _, _, h3237, _, _, _, _, _, _, _) => {
                h3237
            }
        };
        nat_eq(a3238, a3239)
    } else {
        false
    } {
        let a3242 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3240, _, _, _, _, _, _) => {
                h3240
            }
        };
        let a3243 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3241, _, _, _, _, _, _) => {
                h3241
            }
        };
        nat_eq(a3242, a3243)
    } else {
        false
    } {
        let a3248 = {
            let a3246 = match v4.clone() {
                Adt6::C0(h3244, _, _, _) => {
                    h3244
                }
            };
            let a3247 = match v5.clone() {
                Adt6::C0(h3245, _, _, _) => {
                    h3245
                }
            };
            nat_mul(a3246, a3247)?
        };
        let a3249 = 2147483648u64;
        nat_lt(a3248, a3249)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a3251 = match v1 {
                Adt1::C0(h3250, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3250
                }
            };
            let a3252 = 0u64;
            nat_eq(a3251, a3252)
        }, match v2.clone() {
            Adt2::C0(h3253, _, _, _, _, _, _, _, _, _, _) => {
                h3253
            }
        });
        Ok(Adt10::C4({
            let a3254 = f22(v3.clone(), 1u64)?;
            let a3255 = f177(v6.clone())?;
            nat_mul(a3254, a3255)?
        }, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f109(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3256 = v2;
    match m3256 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a3258 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3257, _, _, _, _, _) => {
                        h3257
                    }
                };
                let a3259 = 1u64;
                nat_eq(a3258, a3259)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3260, _, _, _, _, _, _, _, _, _) => {
                    h3260
                }
            }, match v0.clone() {
                Adt1::C0(h3261, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3261
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3262, _, _, _, _, _, _, _, _, _) => {
                    h3262
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3263, _, _, _, _, _, _, _) => {
                    h3263
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h3264, _, _, _, _, _, _, _, _) => {
                        h3264
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h3265, _, _, _, _, _, _, _, _, _, _) => {
                        h3265
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h3266, _, _, _, _, _, _, _, _) => {
                        h3266
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h3267, _, _, _, _, _, _) => {
                        h3267
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h3268, _, _, _, _, _, _, _) => {
                        h3268
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h3269, _, _, _, _, _, _, _, _) => {
                        h3269
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h3270, _, _, _, _, _, _) => {
                        h3270
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h3271, _, _, _, _, _, _, _) => {
                        h3271
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h3272, _, _, _, _, _, _, _, _) => {
                        h3272
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h3273, _, _, _, _, _, _, _, _, _, _) => {
                    h3273
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h3274, _, _, _, _, _, _, _, _, _, _) => {
                    h3274
                }
            };
            let v14: u64 = {
                let a3275 = v12;
                let a3276 = 0u64;
                if nat_eq(a3275, a3276) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a3278 = match v4.clone() {
                    Adt2::C0(_, h3277, _, _, _, _, _, _, _, _, _) => {
                        h3277
                    }
                };
                let a3279 = v14;
                nat_mul(a3278, a3279)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3280, _, _, _) => {
                        h3280
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3281, _, _, _, _) => {
                        h3281
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3282, _, _) => {
                        h3282
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3283, _, _, _) => {
                        h3283
                    }
                }
            };
            let a3285 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h3284, _, _, _, _, _) => {
                    h3284
                }
            };
            let a3286 = 1u64;
            if if if if if if if if if if nat_le(a3285, a3286) {
                let a3287 = v12;
                let a3288 = 0u64;
                if nat_eq(a3287, a3288) {
                    true
                } else {
                    let a3289 = v12;
                    let a3290 = 1u64;
                    if nat_eq(a3289, a3290) {
                        let a3291 = v13;
                        let a3292 = 1u64;
                        nat_eq(a3291, a3292)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3293 = v13;
                let a3294 = 0u64;
                if nat_eq(a3293, a3294) {
                    true
                } else {
                    let a3295 = v13;
                    let a3296 = 1u64;
                    nat_eq(a3295, a3296)
                }
            } else {
                false
            } {
                let a3298 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3297, _, _, _, _, _) => {
                        h3297
                    }
                };
                let a3299 = v14;
                nat_eq(a3298, a3299)
            } else {
                false
            } {
                let a3307 = match v3.clone() {
                    Adt2::C0(_, h3300, _, _, _, _, _, _, _, _, _) => {
                        h3300
                    }
                };
                let a3308 = {
                    let a3305 = v9;
                    let a3306 = {
                        let a3303 = match v6.clone() {
                            Adt6::C0(h3301, _, _, _) => {
                                h3301
                            }
                        };
                        let a3304 = match v7.clone() {
                            Adt6::C0(h3302, _, _, _) => {
                                h3302
                            }
                        };
                        nat_mul(a3303, a3304)?
                    };
                    nat_mul(a3305, a3306)?
                };
                nat_eq(a3307, a3308)
            } else {
                false
            } {
                let a3311 = match v3.clone() {
                    Adt2::C0(_, _, h3309, _, _, _, _, _, _, _, _) => {
                        h3309
                    }
                };
                let a3312 = f89(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h3310, _, _, _, _, _, _, _, _, _) => {
                        h3310
                    }
                })?;
                nat_eq(a3311, a3312)
            } else {
                false
            } {
                let a3314 = 0u64;
                let a3315 = match v3.clone() {
                    Adt2::C0(_, _, h3313, _, _, _, _, _, _, _, _) => {
                        h3313
                    }
                };
                nat_lt(a3314, a3315)
            } else {
                false
            } {
                if v5 {
                    let a3317 = match v1 {
                        Adt2::C0(_, _, _, h3316, _, _, _, _, _, _, _) => {
                            h3316
                        }
                    };
                    let a3318 = v9;
                    if if if nat_eq(a3317, a3318) {
                        let a3320 = match v3.clone() {
                            Adt2::C0(_, _, _, h3319, _, _, _, _, _, _, _) => {
                                h3319
                            }
                        };
                        let a3321 = f89(v7.clone(), v11)?;
                        nat_eq(a3320, a3321)
                    } else {
                        false
                    } {
                        let a3323 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3322, _, _, _, _, _, _) => {
                                h3322
                            }
                        };
                        let a3324 = v10;
                        nat_eq(a3323, a3324)
                    } else {
                        false
                    } {
                        let a3326 = 0u64;
                        let a3327 = match v3.clone() {
                            Adt2::C0(_, _, _, h3325, _, _, _, _, _, _, _) => {
                                h3325
                            }
                        };
                        nat_lt(a3326, a3327)
                    } else {
                        false
                    }
                } else {
                    let a3329 = match v1 {
                        Adt2::C0(_, _, h3328, _, _, _, _, _, _, _, _) => {
                            h3328
                        }
                    };
                    let a3330 = v9;
                    if if if nat_eq(a3329, a3330) {
                        let a3332 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3331, _, _, _, _, _, _) => {
                                h3331
                            }
                        };
                        let a3333 = 1u64;
                        nat_eq(a3332, a3333)
                    } else {
                        false
                    } {
                        let a3335 = match v3.clone() {
                            Adt2::C0(_, _, _, h3334, _, _, _, _, _, _, _) => {
                                h3334
                            }
                        };
                        let a3336 = v10;
                        nat_eq(a3335, a3336)
                    } else {
                        false
                    } {
                        let a3338 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3337, _, _, _, _, _, _) => {
                                h3337
                            }
                        };
                        let a3339 = 1u64;
                        nat_eq(a3338, a3339)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f47(match v4.clone() {
                    Adt2::C0(_, h3340, _, _, _, _, _, _, _, _, _) => {
                        h3340
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a3360 = {
                        let a3357 = {
                            let a3355 = {
                                let a3349 = {
                                    let a3343 = {
                                        let a3341 = v10;
                                        let a3342 = 1u64;
                                        nat_sub(a3341, a3342)
                                    };
                                    let a3344 = v17;
                                    nat_mul(a3343, a3344)?
                                };
                                let a3350 = {
                                    let a3347 = {
                                        let a3345 = v9;
                                        let a3346 = 1u64;
                                        nat_sub(a3345, a3346)
                                    };
                                    let a3348 = v16;
                                    nat_mul(a3347, a3348)?
                                };
                                nat_add(a3349, a3350)?
                            };
                            let a3356 = {
                                let a3353 = {
                                    let a3351 = v11;
                                    let a3352 = 1u64;
                                    nat_sub(a3351, a3352)
                                };
                                let a3354 = v15;
                                nat_mul(a3353, a3354)?
                            };
                            nat_add(a3355, a3356)?
                        };
                        let a3358 = v15;
                        nat_add(a3357, a3358)?
                    };
                    let a3361 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3359, _) => {
                            h3359
                        }
                    };
                    nat_le(a3360, a3361)
                }
            } else {
                false
            } {
                f29(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f22(v3.clone(), 1u64)?, {
                    let a3362 = v13;
                    let a3363 = 0u64;
                    if nat_eq(a3362, a3363) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h3364, _, _, _, _, _, _, _, _, _) => {
                        h3364
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f24(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h3365, _, _, _, _, _, _, _, _, _) => {
                            h3365
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3366, _, _, _, _, _, _, _, _) => {
                            h3366
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h3367, _, _, _, _, _, _, _) => {
                            h3367
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h3368, _, _, _, _, _) => {
                            h3368
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3369, _, _, _, _) => {
                            h3369
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h3370, _, _, _) => {
                            h3370
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f110(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3371 = v2;
    match m3371 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3372, _, _, _, _, _, _, _, _, _) => {
                    h3372
                }
            }, match v0.clone() {
                Adt1::C0(h3373, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3373
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3374, _, _, _, _, _, _, _, _) => {
                    h3374
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3375, _, _, _, _, _) => {
                    h3375
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3376, _, _, _, _, _, _, _, _) => {
                    h3376
                }
            }, match v0.clone() {
                Adt1::C0(_, h3377, _, _, _, _, _, _, _, _, _, _) => {
                    h3377
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3378, _, _, _, _, _, _, _) => {
                    h3378
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3379, _, _, _, _) => {
                    h3379
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3380, _, _, _, _, _, _, _) => {
                    h3380
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3381, _, _, _, _, _, _, _, _, _) => {
                    h3381
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3382, _, _, _, _, _, _) => {
                    h3382
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3383, _, _, _) => {
                    h3383
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3384, _, _) => {
                    h3384
                }
            };
            let v9: u64 = f89(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3385, _, _, _, _, _, _, _) => {
                    h3385
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3386, _, _, _, _, _, _, _, _, _, _) => {
                    h3386
                }
            };
            let v11: u64 = {
                let a3394 = v8;
                let a3395 = {
                    let a3392 = match v5.clone() {
                        Adt6::C0(h3387, _, _, _) => {
                            h3387
                        }
                    };
                    let a3393 = {
                        let a3390 = match v6.clone() {
                            Adt6::C0(h3388, _, _, _) => {
                                h3388
                            }
                        };
                        let a3391 = match v7.clone() {
                            Adt6::C0(h3389, _, _, _) => {
                                h3389
                            }
                        };
                        nat_mul(a3390, a3391)?
                    };
                    nat_mul(a3392, a3393)?
                };
                nat_mul(a3394, a3395)?
            };
            let a3397 = match v4.clone() {
                Adt2::C0(h3396, _, _, _, _, _, _, _, _, _, _) => {
                    h3396
                }
            };
            let a3398 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3397, a3398) {
                let a3399 = v10;
                let a3400 = 0u64;
                if nat_eq(a3399, a3400) {
                    true
                } else {
                    let a3401 = v10;
                    let a3402 = 1u64;
                    nat_eq(a3401, a3402)
                }
            } else {
                false
            } {
                let a3403 = 0u64;
                let a3404 = v8;
                nat_lt(a3403, a3404)
            } else {
                false
            } {
                let a3405 = v8;
                let a3406 = 2147483648u64;
                nat_lt(a3405, a3406)
            } else {
                false
            } {
                let a3408 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3407, _, _, _, _, _) => {
                        h3407
                    }
                };
                let a3409 = 4u64;
                nat_eq(a3408, a3409)
            } else {
                false
            } {
                let a3410 = 0u64;
                let a3411 = v9;
                nat_lt(a3410, a3411)
            } else {
                false
            } {
                let a3413 = 0u64;
                let a3414 = match v3.clone() {
                    Adt2::C0(_, _, h3412, _, _, _, _, _, _, _, _) => {
                        h3412
                    }
                };
                nat_lt(a3413, a3414)
            } else {
                false
            } {
                let a3416 = 0u64;
                let a3417 = match v3.clone() {
                    Adt2::C0(_, _, _, h3415, _, _, _, _, _, _, _) => {
                        h3415
                    }
                };
                nat_lt(a3416, a3417)
            } else {
                false
            } {
                let a3419 = match v3.clone() {
                    Adt2::C0(_, h3418, _, _, _, _, _, _, _, _, _) => {
                        h3418
                    }
                };
                let a3420 = v11;
                nat_eq(a3419, a3420)
            } else {
                false
            } {
                let a3423 = match v3.clone() {
                    Adt2::C0(_, _, h3421, _, _, _, _, _, _, _, _) => {
                        h3421
                    }
                };
                let a3424 = f89(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3422, _, _, _, _, _, _, _, _, _) => {
                        h3422
                    }
                })?;
                nat_eq(a3423, a3424)
            } else {
                false
            } {
                let a3427 = match v3.clone() {
                    Adt2::C0(_, _, _, h3425, _, _, _, _, _, _, _) => {
                        h3425
                    }
                };
                let a3428 = f89(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3426, _, _, _, _, _, _, _, _) => {
                        h3426
                    }
                })?;
                nat_eq(a3427, a3428)
            } else {
                false
            } {
                let a3436 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3429, _, _, _, _, _, _) => {
                        h3429
                    }
                };
                let a3437 = {
                    let a3434 = v9;
                    let a3435 = {
                        let a3431 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3430, _, _, _, _, _, _) => {
                                h3430
                            }
                        };
                        let a3432 = v8;
                        let a3433 = 0u64;
                        nat_quot(a3431, a3432, a3433)
                    };
                    nat_mul(a3434, a3435)?
                };
                nat_eq(a3436, a3437)
            } else {
                false
            } {
                f29(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f22(v3.clone(), 1u64)?, {
                    let a3438 = v10;
                    let a3439 = 0u64;
                    if nat_eq(a3438, a3439) {
                        4u64
                    } else {
                        2u64
                    }
                }, f24(v4.clone(), 1u64), f24(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3440, _, _) => {
                        h3440
                    }
                }, {
                    let a3442 = v8;
                    let a3443 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3441, _, _) => {
                            h3441
                        }
                    };
                    nat_mul(a3442, a3443)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f111(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3444 = v1;
    match m3444 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f211(v0, v3, v2)
        }
    }
}

pub fn f112(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3446 = match v0.clone() {
        Adt2::C0(h3445, _, _, _, _, _, _, _, _, _, _) => {
            h3445
        }
    };
    let a3447 = 1u64;
    if if nat_eq(a3446, a3447) {
        true
    } else {
        let a3449 = match v0.clone() {
            Adt2::C0(h3448, _, _, _, _, _, _, _, _, _, _) => {
                h3448
            }
        };
        let a3450 = 30u64;
        nat_eq(a3449, a3450)
    } {
        let a3452 = match v1.clone() {
            Adt2::C0(h3451, _, _, _, _, _, _, _, _, _, _) => {
                h3451
            }
        };
        let a3453 = 1u64;
        if if if if if if if if if if if nat_eq(a3452, a3453) {
            let a3456 = match v0.clone() {
                Adt2::C0(_, h3454, _, _, _, _, _, _, _, _, _) => {
                    h3454
                }
            };
            let a3457 = match v1.clone() {
                Adt2::C0(_, h3455, _, _, _, _, _, _, _, _, _) => {
                    h3455
                }
            };
            nat_eq(a3456, a3457)
        } else {
            false
        } {
            let a3459 = 0u64;
            let a3460 = match v1.clone() {
                Adt2::C0(_, _, h3458, _, _, _, _, _, _, _, _) => {
                    h3458
                }
            };
            nat_lt(a3459, a3460)
        } else {
            false
        } {
            let a3463 = match v1.clone() {
                Adt2::C0(_, _, h3461, _, _, _, _, _, _, _, _) => {
                    h3461
                }
            };
            let a3464 = match v1.clone() {
                Adt2::C0(_, _, _, h3462, _, _, _, _, _, _, _) => {
                    h3462
                }
            };
            nat_eq(a3463, a3464)
        } else {
            false
        } {
            let a3466 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3465, _, _, _, _, _, _) => {
                    h3465
                }
            };
            let a3467 = 1u64;
            nat_eq(a3466, a3467)
        } else {
            false
        } {
            let a3475 = match v0.clone() {
                Adt2::C0(_, _, h3468, _, _, _, _, _, _, _, _) => {
                    h3468
                }
            };
            let a3476 = {
                let a3473 = {
                    let a3471 = match v1.clone() {
                        Adt2::C0(_, _, h3469, _, _, _, _, _, _, _, _) => {
                            h3469
                        }
                    };
                    let a3472 = match v1.clone() {
                        Adt2::C0(_, _, h3470, _, _, _, _, _, _, _, _) => {
                            h3470
                        }
                    };
                    nat_add(a3471, a3472)?
                };
                let a3474 = 1u64;
                nat_sub(a3473, a3474)
            };
            nat_eq(a3475, a3476)
        } else {
            false
        } {
            let a3478 = 0u64;
            let a3479 = match v0.clone() {
                Adt2::C0(_, _, _, h3477, _, _, _, _, _, _, _) => {
                    h3477
                }
            };
            nat_lt(a3478, a3479)
        } else {
            false
        } {
            let a3481 = 0u64;
            let a3482 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3480, _, _, _, _, _, _) => {
                    h3480
                }
            };
            nat_lt(a3481, a3482)
        } else {
            false
        } {
            let a3484 = 2u64;
            let a3485 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3483, _, _, _, _, _) => {
                    h3483
                }
            };
            nat_le(a3484, a3485)
        } else {
            false
        } {
            let a3487 = 2u64;
            let a3488 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3486, _, _, _, _, _) => {
                    h3486
                }
            };
            nat_le(a3487, a3488)
        } else {
            false
        } {
            f29(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f22(v1.clone(), 1u64)?, 2u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f113(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3489 = v1;
    match m3489 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f212(v0, v3, v2)
        }
    }
}

pub fn f114(v0: u64) -> bool {
    let a3490 = v0;
    let a3491 = 1u64;
    if if nat_eq(a3490, a3491) {
        true
    } else {
        let a3492 = v0;
        let a3493 = 33u64;
        nat_eq(a3492, a3493)
    } {
        true
    } else {
        let a3494 = v0;
        let a3495 = 34u64;
        nat_eq(a3494, a3495)
    }
}

pub fn f115(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f22(v0.clone(), 1u64)?;
    if if if if if f46(v1, v2.clone()) {
        let a3496 = v3;
        let a3497 = f22(v2.clone(), 1u64)?;
        nat_eq(a3496, a3497)
    } else {
        false
    } {
        f25(v0.clone(), v3)?
    } else {
        false
    } {
        f25(v2.clone(), v3)?
    } else {
        false
    } {
        let a3498 = v3;
        let a3499 = 0u64;
        if nat_eq(a3498, a3499) {
            true
        } else {
            let a3501 = 4u64;
            let a3502 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3500, _, _, _, _, _) => {
                    h3500
                }
            };
            nat_le(a3501, a3502)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3503 = 2u64;
            let a3504 = v3;
            nat_mul(a3503, a3504)?
        }, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: u64) -> bool {
    if f213(v0) {
        !f214(v0)
    } else {
        false
    }
}

pub fn f117(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3505 = v1;
    match m3505 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3507 = f206(match v0.clone() {
                Adt2::C0(h3506, _, _, _, _, _, _, _, _, _, _) => {
                    h3506
                }
            });
            match m3507 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3508, _) => {
                            h3508
                        }
                    };
                    if if if if if if f46(v3, v2.clone()) {
                        f29(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3509 = f22(v0.clone(), 1u64)?;
                        let a3510 = f22(v2.clone(), 1u64)?;
                        nat_eq(a3509, a3510)
                    } else {
                        false
                    } {
                        let a3515 = {
                            let a3512 = match v0.clone() {
                                Adt2::C0(_, h3511, _, _, _, _, _, _, _, _, _) => {
                                    h3511
                                }
                            };
                            let a3513 = v5;
                            let a3514 = 1u64;
                            nat_rem(a3512, a3513, a3514)
                        };
                        let a3516 = 0u64;
                        if nat_eq(a3515, a3516) {
                            let a3519 = match v4 {
                                Adt19::C0(_, _, h3517) => {
                                    h3517
                                }
                            };
                            let a3520 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3518, _, _, _, _, _) => {
                                    h3518
                                }
                            };
                            nat_le(a3519, a3520)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3521 = v5;
                        let a3522 = 1u64;
                        if nat_eq(a3521, a3522) {
                            let a3524 = 4u64;
                            let a3525 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3523, _, _, _, _, _) => {
                                    h3523
                                }
                            };
                            nat_le(a3524, a3525)
                        } else {
                            let a3527 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3526, _, _, _, _, _) => {
                                    h3526
                                }
                            };
                            let a3528 = 4u64;
                            nat_eq(a3527, a3528)
                        }
                    } else {
                        false
                    } {
                        let a3533 = {
                            let a3530 = match v2.clone() {
                                Adt2::C0(_, h3529, _, _, _, _, _, _, _, _, _) => {
                                    h3529
                                }
                            };
                            let a3531 = v5;
                            let a3532 = 1u64;
                            nat_rem(a3530, a3531, a3532)
                        };
                        let a3534 = 0u64;
                        if nat_eq(a3533, a3534) {
                            true
                        } else {
                            f37(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f22(v0.clone(), v5)?, 4u64, f24(v0.clone(), v5), f24(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3535, _, _, _, _, _, _, _, _, _, _) => {
                                h3535
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

pub fn f118(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3538 = match v1.clone() {
        Adt2::C0(h3536, _, _, _, _, _, _, _, _, _, _) => {
            h3536
        }
    };
    let a3539 = match v3.clone() {
        Adt2::C0(h3537, _, _, _, _, _, _, _, _, _, _) => {
            h3537
        }
    };
    if nat_eq(a3538, a3539) {
        f215(v0, v1, v2, v3)
    } else {
        if if if if f214(match v1.clone() {
            Adt2::C0(h3540, _, _, _, _, _, _, _, _, _, _) => {
                h3540
            }
        }) {
            f214(match v3.clone() {
                Adt2::C0(h3541, _, _, _, _, _, _, _, _, _, _) => {
                    h3541
                }
            })
        } else {
            false
        } {
            let a3542 = v0;
            let a3543 = 33u64;
            if nat_eq(a3542, a3543) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3544 = f22(v1.clone(), 1u64)?;
            let a3545 = f22(v3.clone(), 1u64)?;
            nat_eq(a3544, a3545)
        } else {
            false
        } {
            let m3547 = f206(match v3.clone() {
                Adt2::C0(h3546, _, _, _, _, _, _, _, _, _, _) => {
                    h3546
                }
            });
            match m3547 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3548, _, _, _, _, _, _, _, _, _, _) => {
                            h3548
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3549, _, _, _, _, _, _, _, _, _, _) => {
                            h3549
                        }
                    };
                    let v7: bool = {
                        let a3550 = v5;
                        let a3551 = 43u64;
                        if nat_eq(a3550, a3551) {
                            true
                        } else {
                            let a3552 = v6;
                            let a3553 = 43u64;
                            nat_eq(a3552, a3553)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3554 = f22(v3.clone(), 1u64)?;
                        let a3555 = if v7 {
                            f159(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3554, a3555)?
                    }, match v4 {
                        Adt19::C0(_, _, h3556) => {
                            h3556
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

pub fn f119(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3558 = match v0.clone() {
        Adt2::C0(h3557, _, _, _, _, _, _, _, _, _, _) => {
            h3557
        }
    };
    let a3559 = 0u64;
    if if if if nat_eq(a3558, a3559) {
        true
    } else {
        let a3561 = match v0.clone() {
            Adt2::C0(h3560, _, _, _, _, _, _, _, _, _, _) => {
                h3560
            }
        };
        let a3562 = 1u64;
        nat_eq(a3561, a3562)
    } {
        true
    } else {
        let a3564 = match v0.clone() {
            Adt2::C0(h3563, _, _, _, _, _, _, _, _, _, _) => {
                h3563
            }
        };
        let a3565 = 25u64;
        if nat_eq(a3564, a3565) {
            true
        } else {
            let a3567 = match v0.clone() {
                Adt2::C0(h3566, _, _, _, _, _, _, _, _, _, _) => {
                    h3566
                }
            };
            let a3568 = 26u64;
            nat_eq(a3567, a3568)
        }
    } {
        true
    } else {
        let a3570 = match v0.clone() {
            Adt2::C0(h3569, _, _, _, _, _, _, _, _, _, _) => {
                h3569
            }
        };
        let a3571 = 30u64;
        nat_eq(a3570, a3571)
    } {
        let a3574 = match v0.clone() {
            Adt2::C0(h3572, _, _, _, _, _, _, _, _, _, _) => {
                h3572
            }
        };
        let a3575 = match v1.clone() {
            Adt2::C0(h3573, _, _, _, _, _, _, _, _, _, _) => {
                h3573
            }
        };
        if if nat_eq(a3574, a3575) {
            f216(match v1.clone() {
                Adt2::C0(_, h3576, _, _, _, _, _, _, _, _, _) => {
                    h3576
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3577, _, _, _, _, _, _, _, _) => {
                    h3577
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3578, _, _, _, _, _, _, _) => {
                    h3578
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3579, _, _, _, _, _, _) => {
                    h3579
                }
            }, match v0.clone() {
                Adt2::C0(_, h3580, _, _, _, _, _, _, _, _, _) => {
                    h3580
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3581, _, _, _, _, _, _, _, _) => {
                    h3581
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3582, _, _, _, _, _, _, _) => {
                    h3582
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3583, _, _, _, _, _, _) => {
                    h3583
                }
            })
        } else {
            false
        } {
            let m3585 = f206(match v0.clone() {
                Adt2::C0(h3584, _, _, _, _, _, _, _, _, _, _) => {
                    h3584
                }
            });
            match m3585 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f22(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3586) => {
                            h3586
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

pub fn f120(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3587 = v2;
    match m3587 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3588 = v0;
            let a3589 = 4u64;
            if if if nat_lt(a3588, a3589) {
                let a3592 = match v1.clone() {
                    Adt2::C0(h3590, _, _, _, _, _, _, _, _, _, _) => {
                        h3590
                    }
                };
                let a3593 = match v4.clone() {
                    Adt2::C0(h3591, _, _, _, _, _, _, _, _, _, _) => {
                        h3591
                    }
                };
                if nat_eq(a3592, a3593) {
                    let a3596 = match v1.clone() {
                        Adt2::C0(h3594, _, _, _, _, _, _, _, _, _, _) => {
                            h3594
                        }
                    };
                    let a3597 = match v3.clone() {
                        Adt2::C0(h3595, _, _, _, _, _, _, _, _, _, _) => {
                            h3595
                        }
                    };
                    nat_eq(a3596, a3597)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f217(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3598, _, _, _, _, _, _, _, _, _) => {
                        h3598
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3599, _, _, _, _, _, _, _, _, _) => {
                        h3599
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3600, _, _, _, _, _, _, _, _, _) => {
                        h3600
                    }
                })? {
                    f217(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3601, _, _, _, _, _, _, _, _) => {
                            h3601
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3602, _, _, _, _, _, _, _, _) => {
                            h3602
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3603, _, _, _, _, _, _, _, _) => {
                            h3603
                        }
                    })?
                } else {
                    false
                } {
                    f217(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3604, _, _, _, _, _, _, _) => {
                            h3604
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3605, _, _, _, _, _, _, _) => {
                            h3605
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                            h3606
                        }
                    })?
                } else {
                    false
                } {
                    f217(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3607, _, _, _, _, _, _) => {
                            h3607
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3608, _, _, _, _, _, _) => {
                            h3608
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3609, _, _, _, _, _, _) => {
                            h3609
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3611 = f206(match v1.clone() {
                    Adt2::C0(h3610, _, _, _, _, _, _, _, _, _, _) => {
                        h3610
                    }
                });
                match m3611 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3612, _) => {
                                h3612
                            }
                        };
                        let a3617 = {
                            let a3614 = match v1.clone() {
                                Adt2::C0(_, h3613, _, _, _, _, _, _, _, _, _) => {
                                    h3613
                                }
                            };
                            let a3615 = v6;
                            let a3616 = 1u64;
                            nat_rem(a3614, a3615, a3616)
                        };
                        let a3618 = 0u64;
                        if if if nat_eq(a3617, a3618) {
                            let a3623 = {
                                let a3620 = match v4.clone() {
                                    Adt2::C0(_, h3619, _, _, _, _, _, _, _, _, _) => {
                                        h3619
                                    }
                                };
                                let a3621 = v6;
                                let a3622 = 1u64;
                                nat_rem(a3620, a3621, a3622)
                            };
                            let a3624 = 0u64;
                            nat_eq(a3623, a3624)
                        } else {
                            false
                        } {
                            let a3629 = {
                                let a3626 = match v3.clone() {
                                    Adt2::C0(_, h3625, _, _, _, _, _, _, _, _, _) => {
                                        h3625
                                    }
                                };
                                let a3627 = v6;
                                let a3628 = 1u64;
                                nat_rem(a3626, a3627, a3628)
                            };
                            let a3630 = 0u64;
                            nat_eq(a3629, a3630)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f22(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3631) => {
                                    h3631
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

pub fn f121(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3633 = match v2.clone() {
        Adt2::C0(h3632, _, _, _, _, _, _, _, _, _, _) => {
            h3632
        }
    };
    let a3634 = 0u64;
    if if nat_eq(a3633, a3634) {
        let a3636 = match v3.clone() {
            Adt2::C0(h3635, _, _, _, _, _, _, _, _, _, _) => {
                h3635
            }
        };
        let a3637 = 0u64;
        nat_eq(a3636, a3637)
    } else {
        false
    } {
        let a3638 = v0;
        let a3639 = 61u64;
        if nat_eq(a3638, a3639) {
            let v4: bool = {
                let a3641 = 0u64;
                let a3642 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3640, _, _, _) => {
                        h3640
                    }
                };
                nat_lt(a3641, a3642)
            };
            if if if if if if f218(match v2.clone() {
                Adt2::C0(_, h3643, _, _, _, _, _, _, _, _, _) => {
                    h3643
                }
            }, match v1.clone() {
                Adt1::C0(h3644, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3644
                }
            }, match v1.clone() {
                Adt1::C0(_, h3645, _, _, _, _, _, _, _, _, _, _) => {
                    h3645
                }
            }, match v3.clone() {
                Adt2::C0(_, h3646, _, _, _, _, _, _, _, _, _) => {
                    h3646
                }
            })? {
                f218(match v2.clone() {
                    Adt2::C0(_, _, h3647, _, _, _, _, _, _, _, _) => {
                        h3647
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3648, _, _, _, _, _, _, _, _, _) => {
                        h3648
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3649, _, _, _, _, _, _, _, _) => {
                        h3649
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3650, _, _, _, _, _, _, _, _) => {
                        h3650
                    }
                })?
            } else {
                false
            } {
                f218(match v2.clone() {
                    Adt2::C0(_, _, _, h3651, _, _, _, _, _, _, _) => {
                        h3651
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3652, _, _, _, _, _, _, _) => {
                        h3652
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3653, _, _, _, _, _, _) => {
                        h3653
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3654, _, _, _, _, _, _, _) => {
                        h3654
                    }
                })?
            } else {
                false
            } {
                f218(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3655, _, _, _, _, _, _) => {
                        h3655
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3656, _, _, _, _, _) => {
                        h3656
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3657, _, _, _, _) => {
                        h3657
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3658, _, _, _, _, _, _) => {
                        h3658
                    }
                })?
            } else {
                false
            } {
                let a3660 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3659, _, _, _) => {
                        h3659
                    }
                };
                let a3661 = 4294967295u64;
                nat_le(a3660, a3661)
            } else {
                false
            } {
                if v4 {
                    !f47(match v2.clone() {
                        Adt2::C0(_, h3662, _, _, _, _, _, _, _, _, _) => {
                            h3662
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3663, _, _, _, _, _, _, _, _) => {
                            h3663
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3664, _, _, _, _, _, _, _) => {
                            h3664
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3665, _, _, _, _, _, _) => {
                            h3665
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f22(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3666, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3666
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3667, _, _, _, _, _, _, _, _, _) => {
                        h3667
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3668, _, _, _, _, _, _, _) => {
                        h3668
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3669, _, _, _, _, _) => {
                        h3669
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3670 = v0;
            let a3671 = 62u64;
            if nat_eq(a3670, a3671) {
                let a3673 = match v1.clone() {
                    Adt1::C0(h3672, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3672
                    }
                };
                let a3674 = 2147483648u64;
                if if if if if nat_lt(a3673, a3674) {
                    let a3676 = match v1.clone() {
                        Adt1::C0(_, h3675, _, _, _, _, _, _, _, _, _, _) => {
                            h3675
                        }
                    };
                    let a3677 = 2147483648u64;
                    nat_lt(a3676, a3677)
                } else {
                    false
                } {
                    let a3680 = match v1.clone() {
                        Adt1::C0(h3678, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3678
                        }
                    };
                    let a3681 = match v2.clone() {
                        Adt2::C0(_, h3679, _, _, _, _, _, _, _, _, _) => {
                            h3679
                        }
                    };
                    nat_lt(a3680, a3681)
                } else {
                    false
                } {
                    let a3684 = match v1.clone() {
                        Adt1::C0(_, h3682, _, _, _, _, _, _, _, _, _, _) => {
                            h3682
                        }
                    };
                    let a3685 = match v2.clone() {
                        Adt2::C0(_, h3683, _, _, _, _, _, _, _, _, _) => {
                            h3683
                        }
                    };
                    nat_lt(a3684, a3685)
                } else {
                    false
                } {
                    let a3688 = match v2.clone() {
                        Adt2::C0(_, _, h3686, _, _, _, _, _, _, _, _) => {
                            h3686
                        }
                    };
                    let a3689 = match v3.clone() {
                        Adt2::C0(_, _, h3687, _, _, _, _, _, _, _, _) => {
                            h3687
                        }
                    };
                    if if if nat_eq(a3688, a3689) {
                        let a3692 = match v2.clone() {
                            Adt2::C0(_, _, _, h3690, _, _, _, _, _, _, _) => {
                                h3690
                            }
                        };
                        let a3693 = match v3.clone() {
                            Adt2::C0(_, _, _, h3691, _, _, _, _, _, _, _) => {
                                h3691
                            }
                        };
                        nat_eq(a3692, a3693)
                    } else {
                        false
                    } {
                        let a3696 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3694, _, _, _, _, _, _) => {
                                h3694
                            }
                        };
                        let a3697 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3695, _, _, _, _, _, _) => {
                                h3695
                            }
                        };
                        nat_eq(a3696, a3697)
                    } else {
                        false
                    } {
                        let a3706 = match v3.clone() {
                            Adt2::C0(_, h3698, _, _, _, _, _, _, _, _, _) => {
                                h3698
                            }
                        };
                        let a3707 = {
                            let a3704 = {
                                let a3701 = match v2.clone() {
                                    Adt2::C0(_, h3699, _, _, _, _, _, _, _, _, _) => {
                                        h3699
                                    }
                                };
                                let a3702 = match v1.clone() {
                                    Adt1::C0(h3700, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3700
                                    }
                                };
                                nat_add(a3701, a3702)?
                            };
                            let a3705 = match v1.clone() {
                                Adt1::C0(_, h3703, _, _, _, _, _, _, _, _, _, _) => {
                                    h3703
                                }
                            };
                            nat_add(a3704, a3705)?
                        };
                        nat_eq(a3706, a3707)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f22(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3708, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3708
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3709 = v0;
                let a3710 = 63u64;
                if nat_eq(a3709, a3710) {
                    if if if if if f219(match v2.clone() {
                        Adt2::C0(_, h3711, _, _, _, _, _, _, _, _, _) => {
                            h3711
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3712, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3712
                        }
                    })? {
                        f219(match v2.clone() {
                            Adt2::C0(_, _, h3713, _, _, _, _, _, _, _, _) => {
                                h3713
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3714, _, _, _, _, _, _, _, _, _, _) => {
                                h3714
                            }
                        })?
                    } else {
                        false
                    } {
                        f219(match v2.clone() {
                            Adt2::C0(_, _, _, h3715, _, _, _, _, _, _, _) => {
                                h3715
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3716, _, _, _, _, _, _, _, _, _) => {
                                h3716
                            }
                        })?
                    } else {
                        false
                    } {
                        f219(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3717, _, _, _, _, _, _) => {
                                h3717
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3718, _, _, _, _, _, _, _, _) => {
                                h3718
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3721 = match v2.clone() {
                            Adt2::C0(_, h3719, _, _, _, _, _, _, _, _, _) => {
                                h3719
                            }
                        };
                        let a3722 = match v3.clone() {
                            Adt2::C0(_, h3720, _, _, _, _, _, _, _, _, _) => {
                                h3720
                            }
                        };
                        if if if nat_eq(a3721, a3722) {
                            let a3725 = match v2.clone() {
                                Adt2::C0(_, _, h3723, _, _, _, _, _, _, _, _) => {
                                    h3723
                                }
                            };
                            let a3726 = match v3.clone() {
                                Adt2::C0(_, _, h3724, _, _, _, _, _, _, _, _) => {
                                    h3724
                                }
                            };
                            nat_eq(a3725, a3726)
                        } else {
                            false
                        } {
                            let a3729 = match v2.clone() {
                                Adt2::C0(_, _, _, h3727, _, _, _, _, _, _, _) => {
                                    h3727
                                }
                            };
                            let a3730 = match v3.clone() {
                                Adt2::C0(_, _, _, h3728, _, _, _, _, _, _, _) => {
                                    h3728
                                }
                            };
                            nat_eq(a3729, a3730)
                        } else {
                            false
                        } {
                            let a3733 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3731, _, _, _, _, _, _) => {
                                    h3731
                                }
                            };
                            let a3734 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3732, _, _, _, _, _, _) => {
                                    h3732
                                }
                            };
                            nat_eq(a3733, a3734)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f22(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3735, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3735
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3736, _, _, _, _, _, _, _, _, _, _) => {
                                h3736
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3737, _, _, _, _, _, _, _, _, _) => {
                                h3737
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3738, _, _, _, _, _, _, _, _) => {
                                h3738
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

pub fn f122(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3739 = v3;
    match m3739 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3740 = v0;
                let a3741 = 4u64;
                nat_eq(a3740, a3741)
            };
            let v7: bool = {
                let a3743 = 0u64;
                let a3744 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3742, _, _, _, _, _, _, _) => {
                        h3742
                    }
                };
                nat_lt(a3743, a3744)
            };
            let v8: u64 = f22(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3745, _, _, _, _, _, _, _, _, _) => {
                    h3745
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3746, _, _, _, _, _, _, _, _) => {
                    h3746
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3747, _, _, _, _, _, _, _) => {
                    h3747
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3748, _, _, _, _, _, _) => {
                    h3748
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3749, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3749
                }
            }, match v1.clone() {
                Adt1::C0(_, h3750, _, _, _, _, _, _, _, _, _, _) => {
                    h3750
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3751, _, _, _, _, _, _, _, _, _) => {
                    h3751
                }
            });
            if if if if if if if if if if if if v6 {
                let a3753 = match v2.clone() {
                    Adt2::C0(h3752, _, _, _, _, _, _, _, _, _, _) => {
                        h3752
                    }
                };
                let a3754 = 0u64;
                if nat_eq(a3753, a3754) {
                    let a3756 = match v5.clone() {
                        Adt2::C0(h3755, _, _, _, _, _, _, _, _, _, _) => {
                            h3755
                        }
                    };
                    let a3757 = 0u64;
                    nat_eq(a3756, a3757)
                } else {
                    false
                }
            } else {
                let a3759 = match v2.clone() {
                    Adt2::C0(h3758, _, _, _, _, _, _, _, _, _, _) => {
                        h3758
                    }
                };
                let a3760 = 0u64;
                if if nat_eq(a3759, a3760) {
                    true
                } else {
                    let a3762 = match v2.clone() {
                        Adt2::C0(h3761, _, _, _, _, _, _, _, _, _, _) => {
                            h3761
                        }
                    };
                    let a3763 = 26u64;
                    nat_eq(a3762, a3763)
                } {
                    let a3765 = match v5.clone() {
                        Adt2::C0(h3764, _, _, _, _, _, _, _, _, _, _) => {
                            h3764
                        }
                    };
                    let a3766 = 0u64;
                    if nat_eq(a3765, a3766) {
                        true
                    } else {
                        let a3768 = match v5.clone() {
                            Adt2::C0(h3767, _, _, _, _, _, _, _, _, _, _) => {
                                h3767
                            }
                        };
                        let a3769 = 26u64;
                        nat_eq(a3768, a3769)
                    }
                } else {
                    false
                }
            } {
                let a3772 = match v2.clone() {
                    Adt2::C0(h3770, _, _, _, _, _, _, _, _, _, _) => {
                        h3770
                    }
                };
                let a3773 = match v4.clone() {
                    Adt2::C0(h3771, _, _, _, _, _, _, _, _, _, _) => {
                        h3771
                    }
                };
                nat_eq(a3772, a3773)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())
            } else {
                false
            } {
                f37(v2.clone())?
            } else {
                false
            } {
                f37(v4.clone())?
            } else {
                false
            } {
                let a3775 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3774, _, _, _, _, _) => {
                        h3774
                    }
                };
                let a3776 = 4u64;
                nat_eq(a3775, a3776)
            } else {
                false
            } {
                let a3777 = v8;
                let a3778 = f22(v2.clone(), 1u64)?;
                nat_le(a3777, a3778)
            } else {
                false
            } {
                let a3780 = match v1.clone() {
                    Adt1::C0(_, _, _, h3779, _, _, _, _, _, _, _, _) => {
                        h3779
                    }
                };
                let a3781 = 1073741824u64;
                nat_lt(a3780, a3781)
            } else {
                false
            } {
                let a3783 = match v1.clone() {
                    Adt1::C0(h3782, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3782
                    }
                };
                let a3784 = 4294967295u64;
                if if if if nat_le(a3783, a3784) {
                    let a3786 = match v1.clone() {
                        Adt1::C0(_, h3785, _, _, _, _, _, _, _, _, _, _) => {
                            h3785
                        }
                    };
                    let a3787 = 4294967295u64;
                    nat_le(a3786, a3787)
                } else {
                    false
                } {
                    let a3789 = match v1.clone() {
                        Adt1::C0(_, _, h3788, _, _, _, _, _, _, _, _, _) => {
                            h3788
                        }
                    };
                    let a3790 = 4294967295u64;
                    nat_le(a3789, a3790)
                } else {
                    false
                } {
                    let a3792 = match v1.clone() {
                        Adt1::C0(_, _, _, h3791, _, _, _, _, _, _, _, _) => {
                            h3791
                        }
                    };
                    let a3793 = 4294967295u64;
                    nat_le(a3792, a3793)
                } else {
                    false
                } {
                    let a3795 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3794, _, _, _, _, _, _, _) => {
                            h3794
                        }
                    };
                    let a3796 = 4294967295u64;
                    nat_le(a3795, a3796)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a3799 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3797) => {
                            h3797
                        }
                    };
                    let a3800 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3798) => {
                            h3798
                        }
                    };
                    nat_eq(a3799, a3800)
                } else {
                    true
                }
            } else {
                false
            } {
                f220(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3801, _, _, _, _, _, _, _, _) => {
                        h3801
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3802, _) => {
                        h3802
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f22(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a3803 = v10;
                    let a3804 = v8;
                    nat_add(a3803, a3804)?
                }, 4u64, f24(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3805, _, _, _, _, _, _, _, _) => {
                        h3805
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f123(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3807 = match v0.clone() {
        Adt2::C0(h3806, _, _, _, _, _, _, _, _, _, _) => {
            h3806
        }
    };
    let a3808 = 0u64;
    if if if nat_eq(a3807, a3808) {
        let a3810 = match v1.clone() {
            Adt2::C0(h3809, _, _, _, _, _, _, _, _, _, _) => {
                h3809
            }
        };
        let a3811 = 0u64;
        nat_eq(a3810, a3811)
    } else {
        false
    } {
        f216(match v0.clone() {
            Adt2::C0(_, h3812, _, _, _, _, _, _, _, _, _) => {
                h3812
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3813, _, _, _, _, _, _, _, _) => {
                h3813
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3814, _, _, _, _, _, _, _) => {
                h3814
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3815, _, _, _, _, _, _) => {
                h3815
            }
        }, match v1.clone() {
            Adt2::C0(_, h3816, _, _, _, _, _, _, _, _, _) => {
                h3816
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3817, _, _, _, _, _, _, _, _) => {
                h3817
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3818, _, _, _, _, _, _, _) => {
                h3818
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3819, _, _, _, _, _, _) => {
                h3819
            }
        })
    } else {
        false
    } {
        let v2: u64 = f22(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3820 = v2;
            let a3821 = f22(v0.clone(), 1u64)?;
            nat_add(a3820, a3821)?
        }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f124(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3823 = match v1.clone() {
        Adt2::C0(h3822, _, _, _, _, _, _, _, _, _, _) => {
            h3822
        }
    };
    let a3824 = 0u64;
    if if if nat_eq(a3823, a3824) {
        true
    } else {
        let a3826 = match v1.clone() {
            Adt2::C0(h3825, _, _, _, _, _, _, _, _, _, _) => {
                h3825
            }
        };
        let a3827 = 1u64;
        nat_eq(a3826, a3827)
    } {
        true
    } else {
        let a3829 = match v1.clone() {
            Adt2::C0(h3828, _, _, _, _, _, _, _, _, _, _) => {
                h3828
            }
        };
        let a3830 = 30u64;
        nat_eq(a3829, a3830)
    } {
        let a3833 = match v1.clone() {
            Adt2::C0(h3831, _, _, _, _, _, _, _, _, _, _) => {
                h3831
            }
        };
        let a3834 = match v2.clone() {
            Adt2::C0(h3832, _, _, _, _, _, _, _, _, _, _) => {
                h3832
            }
        };
        if if if if if nat_eq(a3833, a3834) {
            let a3837 = match v1.clone() {
                Adt2::C0(_, h3835, _, _, _, _, _, _, _, _, _) => {
                    h3835
                }
            };
            let a3838 = match v2.clone() {
                Adt2::C0(_, h3836, _, _, _, _, _, _, _, _, _) => {
                    h3836
                }
            };
            nat_eq(a3837, a3838)
        } else {
            false
        } {
            let a3841 = match v1.clone() {
                Adt2::C0(_, _, _, h3839, _, _, _, _, _, _, _) => {
                    h3839
                }
            };
            let a3842 = match v2.clone() {
                Adt2::C0(_, _, _, h3840, _, _, _, _, _, _, _) => {
                    h3840
                }
            };
            nat_eq(a3841, a3842)
        } else {
            false
        } {
            let a3845 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3843, _, _, _, _, _, _) => {
                    h3843
                }
            };
            let a3846 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3844, _, _, _, _, _, _) => {
                    h3844
                }
            };
            nat_eq(a3845, a3846)
        } else {
            false
        } {
            let a3847 = v0;
            let a3848 = 4u64;
            if nat_eq(a3847, a3848) {
                let a3850 = match v1.clone() {
                    Adt2::C0(_, _, h3849, _, _, _, _, _, _, _, _) => {
                        h3849
                    }
                };
                let a3851 = 1u64;
                if nat_eq(a3850, a3851) {
                    let a3854 = match v1.clone() {
                        Adt2::C0(_, h3852, _, _, _, _, _, _, _, _, _) => {
                            h3852
                        }
                    };
                    let a3855 = match v2.clone() {
                        Adt2::C0(_, _, h3853, _, _, _, _, _, _, _, _) => {
                            h3853
                        }
                    };
                    nat_eq(a3854, a3855)
                } else {
                    false
                }
            } else {
                let a3856 = v0;
                let a3857 = 4u64;
                if if nat_lt(a3856, a3857) {
                    let a3860 = match v1.clone() {
                        Adt2::C0(_, h3858, _, _, _, _, _, _, _, _, _) => {
                            h3858
                        }
                    };
                    let a3861 = match v1.clone() {
                        Adt2::C0(_, _, h3859, _, _, _, _, _, _, _, _) => {
                            h3859
                        }
                    };
                    nat_eq(a3860, a3861)
                } else {
                    false
                } {
                    let a3864 = match v1.clone() {
                        Adt2::C0(_, _, h3862, _, _, _, _, _, _, _, _) => {
                            h3862
                        }
                    };
                    let a3865 = match v2.clone() {
                        Adt2::C0(_, _, h3863, _, _, _, _, _, _, _, _) => {
                            h3863
                        }
                    };
                    nat_eq(a3864, a3865)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m3867 = f206(match v1.clone() {
                Adt2::C0(h3866, _, _, _, _, _, _, _, _, _, _) => {
                    h3866
                }
            });
            match m3867 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f22(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h3868) => {
                            h3868
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

pub fn f125(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f221(match v1.clone() {
        Adt2::C0(h3869, _, _, _, _, _, _, _, _, _, _) => {
            h3869
        }
    }, v0) {
        let a3872 = match v1.clone() {
            Adt2::C0(h3870, _, _, _, _, _, _, _, _, _, _) => {
                h3870
            }
        };
        let a3873 = match v2.clone() {
            Adt2::C0(h3871, _, _, _, _, _, _, _, _, _, _) => {
                h3871
            }
        };
        nat_eq(a3872, a3873)
    } else {
        false
    } {
        let a3876 = match v1.clone() {
            Adt2::C0(_, h3874, _, _, _, _, _, _, _, _, _) => {
                h3874
            }
        };
        let a3877 = match v2.clone() {
            Adt2::C0(_, h3875, _, _, _, _, _, _, _, _, _) => {
                h3875
            }
        };
        if if if nat_eq(a3876, a3877) {
            let a3880 = match v1.clone() {
                Adt2::C0(_, _, h3878, _, _, _, _, _, _, _, _) => {
                    h3878
                }
            };
            let a3881 = match v2.clone() {
                Adt2::C0(_, _, h3879, _, _, _, _, _, _, _, _) => {
                    h3879
                }
            };
            nat_eq(a3880, a3881)
        } else {
            false
        } {
            let a3884 = match v1.clone() {
                Adt2::C0(_, _, _, h3882, _, _, _, _, _, _, _) => {
                    h3882
                }
            };
            let a3885 = match v2.clone() {
                Adt2::C0(_, _, _, h3883, _, _, _, _, _, _, _) => {
                    h3883
                }
            };
            nat_eq(a3884, a3885)
        } else {
            false
        } {
            let a3888 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3886, _, _, _, _, _, _) => {
                    h3886
                }
            };
            let a3889 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3887, _, _, _, _, _, _) => {
                    h3887
                }
            };
            nat_eq(a3888, a3889)
        } else {
            false
        }
    } else {
        false
    } {
        let m3891 = f206(match v1.clone() {
            Adt2::C0(h3890, _, _, _, _, _, _, _, _, _, _) => {
                h3890
            }
        });
        match m3891 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f22(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h3892) => {
                        h3892
                    }
                }, f24(v1.clone(), 1u64), f24(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h3893, _, _, _, _, _, _, _, _, _, _) => {
                        h3893
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f126(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h3894, _, _, _, _, _, _, _, _, _, _, _) => {
                h3894
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3895, _, _, _, _, _, _, _, _, _) => {
                h3895
            }
        }
    };
    let a3896 = 0u64;
    let a3897 = v4;
    if if nat_lt(a3896, a3897) {
        let a3898 = v4;
        let a3899 = 2147483648u64;
        nat_lt(a3898, a3899)
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
        let v7: u64 = f222(match v5.clone() {
            Adt2::C0(_, _, h3900, _, _, _, _, _, _, _, _) => {
                h3900
            }
        }, v4)?;
        let v8: u64 = f222(match v5.clone() {
            Adt2::C0(_, _, _, h3901, _, _, _, _, _, _, _) => {
                h3901
            }
        }, v4)?;
        let a3903 = match v5.clone() {
            Adt2::C0(h3902, _, _, _, _, _, _, _, _, _, _) => {
                h3902
            }
        };
        let a3904 = 0u64;
        if if if if if nat_eq(a3903, a3904) {
            let a3906 = match v6.clone() {
                Adt2::C0(h3905, _, _, _, _, _, _, _, _, _, _) => {
                    h3905
                }
            };
            let a3907 = 0u64;
            nat_eq(a3906, a3907)
        } else {
            false
        } {
            let a3909 = match v5.clone() {
                Adt2::C0(_, _, _, _, h3908, _, _, _, _, _, _) => {
                    h3908
                }
            };
            let a3910 = 1u64;
            nat_eq(a3909, a3910)
        } else {
            false
        } {
            let a3913 = match v5.clone() {
                Adt2::C0(_, h3911, _, _, _, _, _, _, _, _, _) => {
                    h3911
                }
            };
            let a3914 = match v6.clone() {
                Adt2::C0(_, h3912, _, _, _, _, _, _, _, _, _) => {
                    h3912
                }
            };
            nat_eq(a3913, a3914)
        } else {
            false
        } {
            if v0 {
                let a3916 = match v5.clone() {
                    Adt2::C0(_, _, h3915, _, _, _, _, _, _, _, _) => {
                        h3915
                    }
                };
                let a3917 = 2147483648u64;
                if if if nat_lt(a3916, a3917) {
                    let a3919 = match v5.clone() {
                        Adt2::C0(_, _, _, h3918, _, _, _, _, _, _, _) => {
                            h3918
                        }
                    };
                    let a3920 = 2147483648u64;
                    nat_lt(a3919, a3920)
                } else {
                    false
                } {
                    let a3923 = {
                        let a3921 = v7;
                        let a3922 = v8;
                        nat_mul(a3921, a3922)?
                    };
                    let a3924 = 2147483648u64;
                    nat_le(a3923, a3924)
                } else {
                    false
                } {
                    f223(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a3926 = match v1.clone() {
                    Adt1::C0(h3925, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3925
                    }
                };
                let a3927 = v7;
                if if if if nat_eq(a3926, a3927) {
                    let a3929 = match v1 {
                        Adt1::C0(_, h3928, _, _, _, _, _, _, _, _, _, _) => {
                            h3928
                        }
                    };
                    let a3930 = v8;
                    nat_eq(a3929, a3930)
                } else {
                    false
                } {
                    f224(v7, v8, v4)?
                } else {
                    false
                } {
                    let a3932 = match v6.clone() {
                        Adt2::C0(_, _, h3931, _, _, _, _, _, _, _, _) => {
                            h3931
                        }
                    };
                    let a3933 = v4;
                    if nat_eq(a3932, a3933) {
                        let a3935 = match v6.clone() {
                            Adt2::C0(_, _, _, h3934, _, _, _, _, _, _, _) => {
                                h3934
                            }
                        };
                        let a3936 = v4;
                        nat_eq(a3935, a3936)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a3940 = match v6 {
                        Adt2::C0(_, _, _, _, h3937, _, _, _, _, _, _) => {
                            h3937
                        }
                    };
                    let a3941 = {
                        let a3938 = v7;
                        let a3939 = v8;
                        nat_mul(a3938, a3939)?
                    };
                    nat_eq(a3940, a3941)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f22(v3.clone(), 1u64)?, 4u64, f24(v2, 1u64), f24(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f127(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3943 = match v2.clone() {
        Adt2::C0(h3942, _, _, _, _, _, _, _, _, _, _) => {
            h3942
        }
    };
    let a3944 = 0u64;
    if if if if if nat_eq(a3943, a3944) {
        true
    } else {
        let a3946 = match v2.clone() {
            Adt2::C0(h3945, _, _, _, _, _, _, _, _, _, _) => {
                h3945
            }
        };
        let a3947 = 1u64;
        nat_eq(a3946, a3947)
    } {
        true
    } else {
        let a3949 = match v2.clone() {
            Adt2::C0(h3948, _, _, _, _, _, _, _, _, _, _) => {
                h3948
            }
        };
        let a3950 = 30u64;
        nat_eq(a3949, a3950)
    } {
        let a3953 = match v2.clone() {
            Adt2::C0(h3951, _, _, _, _, _, _, _, _, _, _) => {
                h3951
            }
        };
        let a3954 = match v3.clone() {
            Adt2::C0(h3952, _, _, _, _, _, _, _, _, _, _) => {
                h3952
            }
        };
        nat_eq(a3953, a3954)
    } else {
        false
    } {
        if if f20(v2.clone(), v3.clone()) {
            let a3956 = match v1.clone() {
                Adt1::C0(h3955, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3955
                }
            };
            let a3957 = 4294967296u64;
            nat_lt(a3956, a3957)
        } else {
            false
        } {
            let a3959 = match v1.clone() {
                Adt1::C0(_, h3958, _, _, _, _, _, _, _, _, _, _) => {
                    h3958
                }
            };
            let a3960 = 4294967296u64;
            nat_lt(a3959, a3960)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f22(v3.clone(), 1u64)?, {
            let a3962 = match v2.clone() {
                Adt2::C0(h3961, _, _, _, _, _, _, _, _, _, _) => {
                    h3961
                }
            };
            let a3963 = 0u64;
            if nat_eq(a3962, a3963) {
                4u64
            } else {
                2u64
            }
        }, f24(v2.clone(), 1u64), f24(v3, 1u64), Adt4::C13({
            let a3964 = v0;
            let a3965 = 31u64;
            if nat_eq(a3964, a3965) {
                0u64
            } else {
                let a3966 = v0;
                let a3967 = 47u64;
                if nat_eq(a3966, a3967) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h3968, _, _, _, _, _, _, _, _, _, _) => {
                h3968
            }
        }, match v1.clone() {
            Adt1::C0(h3969, _, _, _, _, _, _, _, _, _, _, _) => {
                h3969
            }
        }, match v1 {
            Adt1::C0(_, h3970, _, _, _, _, _, _, _, _, _, _) => {
                h3970
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f128(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3972 = match v0.clone() {
        Adt2::C0(h3971, _, _, _, _, _, _, _, _, _, _) => {
            h3971
        }
    };
    let a3973 = 0u64;
    if if if nat_eq(a3972, a3973) {
        let a3975 = match v1.clone() {
            Adt2::C0(h3974, _, _, _, _, _, _, _, _, _, _) => {
                h3974
            }
        };
        let a3976 = 0u64;
        nat_eq(a3975, a3976)
    } else {
        false
    } {
        f20(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f22(v1.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f129(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3978 = match v0.clone() {
        Adt2::C0(h3977, _, _, _, _, _, _, _, _, _, _) => {
            h3977
        }
    };
    let a3979 = 0u64;
    if if nat_eq(a3978, a3979) {
        let a3981 = match v1.clone() {
            Adt2::C0(h3980, _, _, _, _, _, _, _, _, _, _) => {
                h3980
            }
        };
        let a3982 = 26u64;
        nat_eq(a3981, a3982)
    } else {
        false
    } {
        let a3984 = match v0.clone() {
            Adt2::C0(_, _, _, h3983, _, _, _, _, _, _, _) => {
                h3983
            }
        };
        let a3985 = 1u64;
        if if if if if if if nat_eq(a3984, a3985) {
            let a3987 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3986, _, _, _, _, _, _) => {
                    h3986
                }
            };
            let a3988 = 1u64;
            nat_eq(a3987, a3988)
        } else {
            false
        } {
            let a3990 = match v0.clone() {
                Adt2::C0(_, h3989, _, _, _, _, _, _, _, _, _) => {
                    h3989
                }
            };
            let a3991 = 2147483647u64;
            nat_le(a3990, a3991)
        } else {
            false
        } {
            let a3994 = match v1.clone() {
                Adt2::C0(_, h3992, _, _, _, _, _, _, _, _, _) => {
                    h3992
                }
            };
            let a3995 = match v0.clone() {
                Adt2::C0(_, _, h3993, _, _, _, _, _, _, _, _) => {
                    h3993
                }
            };
            nat_eq(a3994, a3995)
        } else {
            false
        } {
            let a3997 = match v1.clone() {
                Adt2::C0(_, _, h3996, _, _, _, _, _, _, _, _) => {
                    h3996
                }
            };
            let a3998 = 1u64;
            nat_eq(a3997, a3998)
        } else {
            false
        } {
            let a4000 = match v1.clone() {
                Adt2::C0(_, _, _, h3999, _, _, _, _, _, _, _) => {
                    h3999
                }
            };
            let a4001 = 1u64;
            nat_eq(a4000, a4001)
        } else {
            false
        } {
            let a4003 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4002, _, _, _, _, _, _) => {
                    h4002
                }
            };
            let a4004 = 1u64;
            nat_eq(a4003, a4004)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4007 = match v0.clone() {
                    Adt2::C0(_, _, h4005, _, _, _, _, _, _, _, _) => {
                        h4005
                    }
                };
                let a4008 = f166(match v0.clone() {
                    Adt2::C0(_, h4006, _, _, _, _, _, _, _, _, _) => {
                        h4006
                    }
                })?;
                nat_mul(a4007, a4008)?
            }, 4u64, f24(v0, 1u64), f24(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f130(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4009 = v1;
    match m4009 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f171(f24(v0.clone(), 1u64))?;
            let a4011 = match v0.clone() {
                Adt2::C0(h4010, _, _, _, _, _, _, _, _, _, _) => {
                    h4010
                }
            };
            let a4012 = 26u64;
            if if if nat_eq(a4011, a4012) {
                let a4014 = match v3.clone() {
                    Adt2::C0(h4013, _, _, _, _, _, _, _, _, _, _) => {
                        h4013
                    }
                };
                let a4015 = 26u64;
                nat_eq(a4014, a4015)
            } else {
                false
            } {
                let a4017 = match v2.clone() {
                    Adt2::C0(h4016, _, _, _, _, _, _, _, _, _, _) => {
                        h4016
                    }
                };
                let a4018 = 27u64;
                nat_eq(a4017, a4018)
            } else {
                false
            } {
                if if if if if if f20(v0.clone(), v3.clone()) {
                    let a4020 = match v2.clone() {
                        Adt2::C0(_, h4019, _, _, _, _, _, _, _, _, _) => {
                            h4019
                        }
                    };
                    let a4021 = 1u64;
                    nat_eq(a4020, a4021)
                } else {
                    false
                } {
                    let a4023 = match v2.clone() {
                        Adt2::C0(_, _, h4022, _, _, _, _, _, _, _, _) => {
                            h4022
                        }
                    };
                    let a4024 = 1u64;
                    nat_eq(a4023, a4024)
                } else {
                    false
                } {
                    let a4026 = match v2.clone() {
                        Adt2::C0(_, _, _, h4025, _, _, _, _, _, _, _) => {
                            h4025
                        }
                    };
                    let a4027 = 1u64;
                    nat_eq(a4026, a4027)
                } else {
                    false
                } {
                    let a4029 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4028, _, _, _, _, _, _) => {
                            h4028
                        }
                    };
                    let a4030 = 1u64;
                    nat_eq(a4029, a4030)
                } else {
                    false
                } {
                    let a4031 = v4;
                    let a4032 = 9223372036854775808u64;
                    nat_lt(a4031, a4032)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4035 = 2u64;
                        let a4036 = {
                            let a4033 = v4;
                            let a4034 = 2u64;
                            nat_mul(a4033, a4034)?
                        };
                        nat_add(a4035, a4036)?
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

pub fn f131(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4037, _, _, _, _, _, _, _, _, _, _) => {
            h4037
        }
    };
    let a4038 = v3;
    let a4039 = 0u64;
    if if nat_eq(a4038, a4039) {
        true
    } else {
        let a4040 = v0;
        let a4041 = 13u64;
        if nat_eq(a4040, a4041) {
            let a4042 = v3;
            let a4043 = 1u64;
            if nat_eq(a4042, a4043) {
                true
            } else {
                let a4044 = v3;
                let a4045 = 30u64;
                nat_eq(a4044, a4045)
            }
        } else {
            false
        }
    } {
        let a4047 = v3;
        let a4048 = match v2.clone() {
            Adt2::C0(h4046, _, _, _, _, _, _, _, _, _, _) => {
                h4046
            }
        };
        if if nat_eq(a4047, a4048) {
            let a4049 = v0;
            let a4050 = 13u64;
            if nat_eq(a4049, a4050) {
                let a4052 = match v2.clone() {
                    Adt2::C0(_, h4051, _, _, _, _, _, _, _, _, _) => {
                        h4051
                    }
                };
                let a4053 = 1u64;
                if if if nat_eq(a4052, a4053) {
                    let a4055 = match v2.clone() {
                        Adt2::C0(_, _, h4054, _, _, _, _, _, _, _, _) => {
                            h4054
                        }
                    };
                    let a4056 = 1u64;
                    nat_eq(a4055, a4056)
                } else {
                    false
                } {
                    let a4058 = match v2.clone() {
                        Adt2::C0(_, _, _, h4057, _, _, _, _, _, _, _) => {
                            h4057
                        }
                    };
                    let a4059 = 1u64;
                    nat_eq(a4058, a4059)
                } else {
                    false
                } {
                    let a4061 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4060, _, _, _, _, _, _) => {
                            h4060
                        }
                    };
                    let a4062 = 1u64;
                    nat_eq(a4061, a4062)
                } else {
                    false
                }
            } else {
                let a4064 = match v2.clone() {
                    Adt2::C0(_, h4063, _, _, _, _, _, _, _, _, _) => {
                        h4063
                    }
                };
                let a4065 = 1u64;
                if if if nat_eq(a4064, a4065) {
                    let a4068 = match v1.clone() {
                        Adt2::C0(_, _, h4066, _, _, _, _, _, _, _, _) => {
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
                } {
                    let a4072 = match v1.clone() {
                        Adt2::C0(_, _, _, h4070, _, _, _, _, _, _, _) => {
                            h4070
                        }
                    };
                    let a4073 = match v2.clone() {
                        Adt2::C0(_, _, _, h4071, _, _, _, _, _, _, _) => {
                            h4071
                        }
                    };
                    nat_eq(a4072, a4073)
                } else {
                    false
                } {
                    let a4076 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4074, _, _, _, _, _, _) => {
                            h4074
                        }
                    };
                    let a4077 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4075, _, _, _, _, _, _) => {
                            h4075
                        }
                    };
                    nat_eq(a4076, a4077)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f24(v1, 1u64);
            Ok(Adt10::C4(f225(v4.clone(), v0, v3)?, {
                let a4078 = v3;
                let a4079 = 0u64;
                if nat_eq(a4078, a4079) {
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

pub fn f132(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4080 = v0;
    let a4081 = 89u64;
    if if nat_eq(a4080, a4081) {
        let a4082 = v1;
        let a4083 = 14u64;
        nat_eq(a4082, a4083)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a4084 = v0;
        let a4085 = 89u64;
        if if nat_eq(a4084, a4085) {
            let a4086 = v1;
            let a4087 = 15u64;
            nat_eq(a4086, a4087)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a4088 = v0;
            let a4089 = 89u64;
            if if nat_eq(a4088, a4089) {
                let a4090 = 8u64;
                let a4091 = v1;
                if nat_le(a4090, a4091) {
                    let a4092 = v1;
                    let a4093 = 9u64;
                    nat_le(a4092, a4093)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a4094 = v1;
                    let a4095 = 12u64;
                    nat_add(a4094, a4095)?
                }))
            } else {
                let a4096 = v0;
                let a4097 = 89u64;
                if if nat_eq(a4096, a4097) {
                    let a4098 = v1;
                    let a4099 = 4u64;
                    nat_eq(a4098, a4099)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a4100 = 10u64;
                    let a4101 = v0;
                    if if nat_le(a4100, a4101) {
                        let a4102 = v0;
                        let a4103 = 12u64;
                        nat_le(a4102, a4103)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a4104 = v0;
                            let a4105 = 3u64;
                            nat_add(a4104, a4105)?
                        }))
                    } else {
                        let a4106 = v0;
                        let a4107 = 89u64;
                        if if nat_eq(a4106, a4107) {
                            let a4108 = v1;
                            let a4109 = 7u64;
                            nat_eq(a4108, a4109)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a4110 = v0;
                            let a4111 = 89u64;
                            if if nat_eq(a4110, a4111) {
                                let a4112 = v1;
                                let a4113 = 10u64;
                                nat_eq(a4112, a4113)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a4114 = v0;
                                let a4115 = 89u64;
                                if if nat_eq(a4114, a4115) {
                                    let a4116 = v1;
                                    let a4117 = 13u64;
                                    nat_eq(a4116, a4117)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a4118 = v0;
                                    let a4119 = 8u64;
                                    if nat_eq(a4118, a4119) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a4120 = v0;
                                        let a4121 = 9u64;
                                        if nat_eq(a4120, a4121) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a4122 = v0;
                                            let a4123 = 89u64;
                                            if nat_eq(a4122, a4123) {
                                                let a4124 = v1;
                                                let a4125 = 0u64;
                                                if nat_eq(a4124, a4125) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a4126 = v1;
                                                    let a4127 = 1u64;
                                                    if nat_eq(a4126, a4127) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a4128 = v1;
                                                        let a4129 = 2u64;
                                                        if nat_eq(a4128, a4129) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a4130 = v1;
                                                            let a4131 = 3u64;
                                                            if nat_eq(a4130, a4131) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a4132 = v1;
                                                                let a4133 = 6u64;
                                                                if nat_eq(a4132, a4133) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a4134 = v1;
                                                                    let a4135 = 12u64;
                                                                    if nat_eq(a4134, a4135) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a4136 = v1;
                                                                        let a4137 = 11u64;
                                                                        if nat_eq(a4136, a4137) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a4138 = v1;
                                                                            let a4139 = 18u64;
                                                                            if nat_eq(a4138, a4139) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a4140 = v1;
                                                                                let a4141 = 19u64;
                                                                                if nat_eq(a4140, a4141) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a4142 = v1;
                                                                                    let a4143 = 20u64;
                                                                                    if nat_eq(a4142, a4143) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a4144 = v1;
                                                                                        let a4145 = 21u64;
                                                                                        if nat_eq(a4144, a4145) {
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

pub fn f133(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4146 = f226(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4146 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4147 = v0;
            let a4148 = 2u64;
            if if if if nat_eq(a4147, a4148) {
                true
            } else {
                let a4149 = v0;
                let a4150 = 5u64;
                nat_eq(a4149, a4150)
            } {
                true
            } else {
                let a4151 = v0;
                let a4152 = 6u64;
                nat_eq(a4151, a4152)
            } {
                true
            } else {
                let a4153 = v0;
                let a4154 = 7u64;
                nat_eq(a4153, a4154)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f24(v2.clone(), 1u64), f24(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4155 = v3;
                    match m4155 {
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

pub fn f134(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4157 = match v1.clone() {
        Adt2::C0(h4156, _, _, _, _, _, _, _, _, _, _) => {
            h4156
        }
    };
    let a4158 = 0u64;
    if if if if if nat_eq(a4157, a4158) {
        true
    } else {
        let a4160 = match v1.clone() {
            Adt2::C0(h4159, _, _, _, _, _, _, _, _, _, _) => {
                h4159
            }
        };
        let a4161 = 1u64;
        nat_eq(a4160, a4161)
    } {
        true
    } else {
        let a4163 = match v1.clone() {
            Adt2::C0(h4162, _, _, _, _, _, _, _, _, _, _) => {
                h4162
            }
        };
        let a4164 = 30u64;
        nat_eq(a4163, a4164)
    } {
        let a4167 = match v1.clone() {
            Adt2::C0(h4165, _, _, _, _, _, _, _, _, _, _) => {
                h4165
            }
        };
        let a4168 = match v2.clone() {
            Adt2::C0(h4166, _, _, _, _, _, _, _, _, _, _) => {
                h4166
            }
        };
        if nat_eq(a4167, a4168) {
            true
        } else {
            let a4170 = match v2.clone() {
                Adt2::C0(h4169, _, _, _, _, _, _, _, _, _, _) => {
                    h4169
                }
            };
            let a4171 = 0u64;
            nat_eq(a4170, a4171)
        }
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4173 = f22(v2.clone(), 1u64)?;
            let a4174 = f227(v0, match v2.clone() {
                Adt2::C0(h4172, _, _, _, _, _, _, _, _, _, _) => {
                    h4172
                }
            });
            nat_mul(a4173, a4174)?
        }, {
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
        }, f24(v1.clone(), 1u64), f24(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h4178, _, _, _, _, _, _, _, _, _, _) => {
                h4178
            }
        }, match v2 {
            Adt2::C0(h4179, _, _, _, _, _, _, _, _, _, _) => {
                h4179
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f135(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4192 = {
        let a4188 = {
            let a4184 = {
                let a4180 = v0;
                let a4181 = v4;
                nat_mul(a4180, a4181)?
            };
            let a4185 = {
                let a4182 = v1;
                let a4183 = v5;
                nat_mul(a4182, a4183)?
            };
            nat_add(a4184, a4185)?
        };
        let a4189 = {
            let a4186 = v2;
            let a4187 = v6;
            nat_mul(a4186, a4187)?
        };
        nat_add(a4188, a4189)?
    };
    let a4193 = {
        let a4190 = v3;
        let a4191 = v7;
        nat_mul(a4190, a4191)?
    };
    nat_add(a4192, a4193)
}

pub fn f136(v0: u64) -> u64 {
    let a4194 = v0;
    let a4195 = 2147483648u64;
    if nat_lt(a4194, a4195) {
        0u64
    } else {
        let a4196 = 4294967296u64;
        let a4197 = v0;
        nat_sub(a4196, a4197)
    }
}

pub fn f137(v0: u64) -> u64 {
    let a4198 = v0;
    let a4199 = 2147483648u64;
    if nat_lt(a4198, a4199) {
        v0
    } else {
        0u64
    }
}

pub fn f138(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4200 = v7;
        let a4201 = 6u64;
        if nat_eq(a4200, a4201) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f86(v3, v4), f86(v5, v6), f87(v7, v8), f165(v1), f165(v9))
}

pub fn f139(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4202 = v0;
    match m4202 {
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
        Adt4::C13(v53, v54, v55, v56) => {
            Adt11::C4(v53, v54, v55, v56)
        }
        Adt4::C14(v57, _, _) => {
            let a4203 = 13u64;
            let a4204 = v57;
            if nat_le(a4203, a4204) {
                Adt11::C7(v57, v1, v2)
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
        Adt4::C27(v77) => {
            Adt11::C2(match v77.clone() {
                Adt5::C0(_, _, _, _, _, h4205, _, _) => {
                    h4205
                }
            }, match v77 {
                Adt5::C0(_, _, _, _, _, _, _, h4206) => {
                    h4206
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
    }
}

pub fn f140(v0: Adt4) -> u64 {
    let m4207 = v0;
    match m4207 {
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
        Adt4::C14(v55, _, _) => {
            v55
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
    }
}

pub fn f141(v0: Adt4, v1: u64) -> u64 {
    let m4208 = v0;
    match m4208 {
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
        Adt4::C16(_, v60) => {
            v60
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v63, _) => {
            let a4209 = v1;
            let a4210 = 6u64;
            if nat_eq(a4209, a4210) {
                v63
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
        Adt4::C26(v77, _, _, _) => {
            let a4219 = v1;
            let a4220 = 6u64;
            if if if nat_eq(a4219, a4220) {
                true
            } else {
                let a4221 = v1;
                let a4222 = 7u64;
                nat_eq(a4221, a4222)
            } {
                let a4223 = v77;
                let a4224 = 30u64;
                nat_eq(a4223, a4224)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v76) => {
            match v76 {
                Adt5::C0(_, _, _, _, _, _, _, h4218) => {
                    h4218
                }
            }
        }
        Adt4::C28(v72) => {
            let a4211 = v1;
            let a4212 = 6u64;
            if nat_eq(a4211, a4212) {
                match v72 {
                    Adt7::C0(_, _, _, _, h4213) => {
                        h4213
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v75) => {
            let a4214 = v1;
            let a4215 = 6u64;
            if if nat_eq(a4214, a4215) {
                true
            } else {
                let a4216 = v1;
                let a4217 = 7u64;
                nat_eq(a4216, a4217)
            } {
                v75
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
    }
}

pub fn f142(v0: Adt4, v1: u64) -> Adt11 {
    let a4225 = v1;
    let a4226 = 4u64;
    if nat_lt(a4225, a4226) {
        Adt11::C0(v1)
    } else {
        let a4227 = v1;
        let a4228 = 4u64;
        if nat_eq(a4227, a4228) {
            Adt11::C1(0u64)
        } else {
            let a4229 = v1;
            let a4230 = 5u64;
            if nat_eq(a4229, a4230) {
                Adt11::C1(3u64)
            } else {
                let a4231 = v1;
                let a4232 = 6u64;
                if nat_eq(a4231, a4232) {
                    Adt11::C2(f141(v0, v1), 0u64, false)
                } else {
                    let a4233 = v1;
                    let a4234 = 7u64;
                    if nat_eq(a4233, a4234) {
                        Adt11::C2(0u64, f141(v0, v1), false)
                    } else {
                        let a4235 = v1;
                        let a4236 = 16u64;
                        if nat_lt(a4235, a4236) {
                            Adt11::C1({
                                let a4237 = v1;
                                let a4238 = 8u64;
                                nat_sub(a4237, a4238)
                            })
                        } else {
                            let a4239 = v1;
                            let a4240 = 20u64;
                            if nat_lt(a4239, a4240) {
                                Adt11::C5({
                                    let a4241 = v1;
                                    let a4242 = 16u64;
                                    nat_sub(a4241, a4242)
                                })
                            } else {
                                let a4243 = v1;
                                let a4244 = 20u64;
                                if nat_eq(a4243, a4244) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4245 = v1;
                                    let a4246 = 21u64;
                                    if nat_eq(a4245, a4246) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4247 = v1;
                                        let a4248 = 22u64;
                                        if nat_eq(a4247, a4248) {
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

pub fn f143(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4266 = {
        let a4261 = f228(v2, {
            let a4252 = {
                let a4249 = v0;
                let a4250 = v1;
                let a4251 = 0u64;
                nat_quot(a4249, a4250, a4251)
            };
            let a4253 = v2;
            let a4254 = 0u64;
            nat_rem(a4252, a4253, a4254)
        }, {
            let a4258 = {
                let a4255 = v0;
                let a4256 = v1;
                let a4257 = 0u64;
                nat_quot(a4255, a4256, a4257)
            };
            let a4259 = v2;
            let a4260 = 0u64;
            nat_quot(a4258, a4259, a4260)
        })?;
        let a4262 = v1;
        nat_mul(a4261, a4262)?
    };
    let a4267 = {
        let a4263 = v0;
        let a4264 = v1;
        let a4265 = 0u64;
        nat_rem(a4263, a4264, a4265)
    };
    nat_add(a4266, a4267)
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4269 = v0;
        let a4270 = match v2.clone() {
            Adt3::C0(h4268, _, _, _, _, _, _, _) => {
                h4268
            }
        };
        let a4271 = 0u64;
        nat_quot(a4269, a4270, a4271)
    };
    let v4: u64 = {
        let a4273 = v3;
        let a4274 = match v2.clone() {
            Adt3::C0(_, h4272, _, _, _, _, _, _) => {
                h4272
            }
        };
        let a4275 = 0u64;
        nat_quot(a4273, a4274, a4275)
    };
    f229({
        let a4277 = v0;
        let a4278 = match v2.clone() {
            Adt3::C0(h4276, _, _, _, _, _, _, _) => {
                h4276
            }
        };
        let a4279 = 0u64;
        nat_rem(a4277, a4278, a4279)
    }, {
        let a4281 = v3;
        let a4282 = match v2.clone() {
            Adt3::C0(_, h4280, _, _, _, _, _, _) => {
                h4280
            }
        };
        let a4283 = 0u64;
        nat_rem(a4281, a4282, a4283)
    }, {
        let a4285 = v4;
        let a4286 = match v2.clone() {
            Adt3::C0(_, _, h4284, _, _, _, _, _) => {
                h4284
            }
        };
        let a4287 = 0u64;
        nat_rem(a4285, a4286, a4287)
    }, {
        let a4289 = v4;
        let a4290 = match v2 {
            Adt3::C0(_, _, h4288, _, _, _, _, _) => {
                h4288
            }
        };
        let a4291 = 0u64;
        nat_quot(a4289, a4290, a4291)
    }, match v1.clone() {
        Adt3::C0(h4292, _, _, _, _, _, _, _) => {
            h4292
        }
    }, match v1.clone() {
        Adt3::C0(_, h4293, _, _, _, _, _, _) => {
            h4293
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4294, _, _, _, _, _) => {
            h4294
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4295, _, _, _, _) => {
            h4295
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4296, _, _, _) => {
            h4296
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4297, _, _) => {
            h4297
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4298, _) => {
            h4298
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4299) => {
            h4299
        }
    })
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4301 = v0;
        let a4302 = match v3.clone() {
            Adt3::C0(h4300, _, _, _, _, _, _, _) => {
                h4300
            }
        };
        let a4303 = 0u64;
        nat_quot(a4301, a4302, a4303)
    };
    let v6: u64 = {
        let a4305 = v5;
        let a4306 = match v3.clone() {
            Adt3::C0(_, h4304, _, _, _, _, _, _) => {
                h4304
            }
        };
        let a4307 = 0u64;
        nat_quot(a4305, a4306, a4307)
    };
    let v7: u64 = {
        let a4309 = v0;
        let a4310 = match v3.clone() {
            Adt3::C0(h4308, _, _, _, _, _, _, _) => {
                h4308
            }
        };
        let a4311 = 0u64;
        nat_rem(a4309, a4310, a4311)
    };
    let v8: u64 = {
        let a4313 = v5;
        let a4314 = match v3.clone() {
            Adt3::C0(_, h4312, _, _, _, _, _, _) => {
                h4312
            }
        };
        let a4315 = 0u64;
        nat_rem(a4313, a4314, a4315)
    };
    let v9: u64 = {
        let a4317 = v6;
        let a4318 = match v3.clone() {
            Adt3::C0(_, _, h4316, _, _, _, _, _) => {
                h4316
            }
        };
        let a4319 = 0u64;
        nat_rem(a4317, a4318, a4319)
    };
    let v10: u64 = {
        let a4321 = v6;
        let a4322 = match v3.clone() {
            Adt3::C0(_, _, h4320, _, _, _, _, _) => {
                h4320
            }
        };
        let a4323 = 0u64;
        nat_quot(a4321, a4322, a4323)
    };
    let v11: u64 = {
        let a4324 = v4;
        let a4325 = 0u64;
        if nat_eq(a4324, a4325) {
            match v1.clone() {
                Adt3::C0(h4326, _, _, _, _, _, _, _) => {
                    h4326
                }
            }
        } else {
            let a4327 = v4;
            let a4328 = 1u64;
            if nat_eq(a4327, a4328) {
                match v1.clone() {
                    Adt3::C0(_, h4329, _, _, _, _, _, _) => {
                        h4329
                    }
                }
            } else {
                let a4330 = v4;
                let a4331 = 2u64;
                if nat_eq(a4330, a4331) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4332, _, _, _, _, _) => {
                            h4332
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4333, _, _, _, _) => {
                            h4333
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4340 = v11;
        let a4341 = {
            let a4334 = v4;
            let a4335 = 0u64;
            if nat_eq(a4334, a4335) {
                v7
            } else {
                let a4336 = v4;
                let a4337 = 1u64;
                if nat_eq(a4336, a4337) {
                    v8
                } else {
                    let a4338 = v4;
                    let a4339 = 2u64;
                    if nat_eq(a4338, a4339) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4340, a4341)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f135(if if v12 {
        let a4342 = v4;
        let a4343 = 0u64;
        nat_eq(a4342, a4343)
    } else {
        false
    } {
        let a4344 = v7;
        let a4345 = v11;
        nat_sub(a4344, a4345)
    } else {
        v7
    }, if if v12 {
        let a4346 = v4;
        let a4347 = 1u64;
        nat_eq(a4346, a4347)
    } else {
        false
    } {
        let a4348 = v8;
        let a4349 = v11;
        nat_sub(a4348, a4349)
    } else {
        v8
    }, if if v12 {
        let a4350 = v4;
        let a4351 = 2u64;
        nat_eq(a4350, a4351)
    } else {
        false
    } {
        let a4352 = v9;
        let a4353 = v11;
        nat_sub(a4352, a4353)
    } else {
        v9
    }, if if v12 {
        let a4354 = v4;
        let a4355 = 3u64;
        nat_eq(a4354, a4355)
    } else {
        false
    } {
        let a4356 = v10;
        let a4357 = v11;
        nat_sub(a4356, a4357)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4358, _, _, _) => {
            h4358
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4359, _, _) => {
            h4359
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4360, _) => {
            h4360
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4361) => {
            h4361
        }
    })?, f88(v0, v3)?))
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f57(v0, v2.clone(), 0u64);
    let v9: u64 = f57(v0, v2.clone(), 1u64);
    let v10: u64 = f57(v0, v2.clone(), 2u64);
    let v11: u64 = f57(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f56(v1.clone(), f230(v8, v3, match v1.clone() {
            Adt3::C0(h4362, _, _, _, _, _, _, _) => {
                h4362
            }
        })?, f230(v9, v4, match v1.clone() {
            Adt3::C0(_, h4363, _, _, _, _, _, _) => {
                h4363
            }
        })?, f230(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4364, _, _, _, _, _) => {
                h4364
            }
        })?, f230(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4365, _, _, _, _) => {
                h4365
            }
        })?)?, f88(v0, v2)?))
    } else {
        if if if if f231(v8, v3, match v1.clone() {
            Adt3::C0(h4366, _, _, _, _, _, _, _) => {
                h4366
            }
        })? {
            f231(v9, v4, match v1.clone() {
                Adt3::C0(_, h4367, _, _, _, _, _, _) => {
                    h4367
                }
            })?
        } else {
            false
        } {
            f231(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4368, _, _, _, _, _) => {
                    h4368
                }
            })?
        } else {
            false
        } {
            f231(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4369, _, _, _, _) => {
                    h4369
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f56(v1, f232(v8, v3)?, f232(v9, v4)?, f232(v10, v5)?, f232(v11, v6)?)?, f88(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f88(v0, v2)?))
        }
    }
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f57(v0, v2.clone(), 0u64);
    let v5: u64 = f57(v0, v2.clone(), 1u64);
    let v6: u64 = f57(v0, v2.clone(), 2u64);
    let v7: u64 = f57(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f56(v1.clone(), f233(v4, v3, match v1 {
        Adt3::C0(h4370, _, _, _, _, _, _, _) => {
            h4370
        }
    })?, v5, v6, v7)?, f88(v0, v2)?))
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4371 = v0;
    let a4372 = v3;
    if nat_lt(a4371, a4372) {
        Ok(Adt9::C1(false, {
            let a4373 = v0;
            let a4374 = 4u64;
            nat_mul(a4373, a4374)?
        }, {
            let a4375 = v0;
            let a4376 = 4u64;
            nat_mul(a4375, a4376)?
        }))
    } else {
        let v6: u64 = {
            let a4377 = v0;
            let a4378 = v3;
            nat_sub(a4377, a4378)
        };
        let v7: u64 = f88(v6, v1)?;
        let v8: u64 = f234(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4379 = v0;
    let a4380 = v3;
    if nat_lt(a4379, a4380) {
        Ok(Adt9::C1(true, 0u64, f88(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4381 = v0;
            let a4382 = v3;
            nat_sub(a4381, a4382)
        };
        let v5: u64 = f144(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f88(v4, v1)?, 3u64, v5))
    }
}

pub fn f150(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4384 = v1;
        let a4385 = match v3.clone() {
            Adt3::C0(h4383, _, _, _, _, _, _, _) => {
                h4383
            }
        };
        let a4386 = 0u64;
        nat_rem(a4384, a4385, a4386)
    };
    let v5: u64 = {
        let a4392 = {
            let a4388 = v1;
            let a4389 = match v3.clone() {
                Adt3::C0(h4387, _, _, _, _, _, _, _) => {
                    h4387
                }
            };
            let a4390 = 0u64;
            nat_quot(a4388, a4389, a4390)
        };
        let a4393 = match v3.clone() {
            Adt3::C0(_, h4391, _, _, _, _, _, _) => {
                h4391
            }
        };
        let a4394 = 0u64;
        nat_rem(a4392, a4393, a4394)
    };
    let v6: bool = f235(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f144(v1, v2, v3.clone())?
    } else {
        0u64
    }, f88(v1, v3)?))
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4395 = v3;
        let a4396 = 8u64;
        nat_add(a4395, a4396)?
    }, 0u64, f88(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f144(v0, v5, v2.clone())?
    }, 3u64, f88(v0, v2)?))
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4397 = v4;
    let a4398 = 0u64;
    if if nat_eq(a4397, a4398) {
        f116(v3)
    } else {
        false
    } {
        let m4399 = f206(v3);
        match m4399 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4400, _) => {
                        h4400
                    }
                };
                Ok(Adt9::C5(v3, f88(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4401) => {
                        h4401
                    }
                }, f88({
                    let a4402 = v0;
                    let a4403 = v6;
                    nat_mul(a4402, a4403)?
                }, v2.clone())?, {
                    let a4409 = {
                        let a4407 = {
                            let a4404 = v6;
                            let a4405 = 1u64;
                            nat_sub(a4404, a4405)
                        };
                        let a4408 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4406, _, _, _) => {
                                h4406
                            }
                        };
                        nat_mul(a4407, a4408)?
                    };
                    let a4410 = 4u64;
                    nat_add(a4409, a4410)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4411, _, _, _) => {
                        h4411
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f88(v0, v1)?, f88(v0, v2)?))
    }
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4413 = v0;
        let a4414 = match v2.clone() {
            Adt3::C0(h4412, _, _, _, _, _, _, _) => {
                h4412
            }
        };
        let a4415 = 0u64;
        nat_rem(a4413, a4414, a4415)
    };
    let v7: u64 = {
        let a4417 = v0;
        let a4418 = match v2.clone() {
            Adt3::C0(h4416, _, _, _, _, _, _, _) => {
                h4416
            }
        };
        let a4419 = 0u64;
        nat_quot(a4417, a4418, a4419)
    };
    let v8: u64 = {
        let a4421 = v7;
        let a4422 = match v2.clone() {
            Adt3::C0(_, h4420, _, _, _, _, _, _) => {
                h4420
            }
        };
        let a4423 = 0u64;
        nat_rem(a4421, a4422, a4423)
    };
    let v9: u64 = {
        let a4425 = v7;
        let a4426 = match v2.clone() {
            Adt3::C0(_, h4424, _, _, _, _, _, _) => {
                h4424
            }
        };
        let a4427 = 0u64;
        nat_quot(a4425, a4426, a4427)
    };
    let v10: u64 = {
        let a4429 = v9;
        let a4430 = match v2.clone() {
            Adt3::C0(_, _, h4428, _, _, _, _, _) => {
                h4428
            }
        };
        let a4431 = 0u64;
        nat_rem(a4429, a4430, a4431)
    };
    let v11: u64 = {
        let a4433 = v9;
        let a4434 = match v2.clone() {
            Adt3::C0(_, _, h4432, _, _, _, _, _) => {
                h4432
            }
        };
        let a4435 = 0u64;
        nat_quot(a4433, a4434, a4435)
    };
    let v12: u64 = f88(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4444 = {
                let a4439 = {
                    let a4436 = v10;
                    let a4437 = v4;
                    let a4438 = 0u64;
                    nat_quot(a4436, a4437, a4438)
                };
                let a4440 = v5;
                nat_mul(a4439, a4440)?
            };
            let a4445 = {
                let a4441 = v8;
                let a4442 = v4;
                let a4443 = 0u64;
                nat_quot(a4441, a4442, a4443)
            };
            nat_add(a4444, a4445)?
        };
        let v14: u64 = {
            let a4465 = v6;
            let a4466 = {
                let a4463 = match v1.clone() {
                    Adt3::C0(h4446, _, _, _, _, _, _, _) => {
                        h4446
                    }
                };
                let a4464 = {
                    let a4461 = {
                        let a4447 = v8;
                        let a4448 = v4;
                        let a4449 = 0u64;
                        nat_rem(a4447, a4448, a4449)
                    };
                    let a4462 = {
                        let a4459 = match v1.clone() {
                            Adt3::C0(_, h4450, _, _, _, _, _, _) => {
                                h4450
                            }
                        };
                        let a4460 = {
                            let a4457 = {
                                let a4451 = v10;
                                let a4452 = v4;
                                let a4453 = 0u64;
                                nat_rem(a4451, a4452, a4453)
                            };
                            let a4458 = {
                                let a4455 = match v1.clone() {
                                    Adt3::C0(_, _, h4454, _, _, _, _, _) => {
                                        h4454
                                    }
                                };
                                let a4456 = v13;
                                nat_mul(a4455, a4456)?
                            };
                            nat_add(a4457, a4458)?
                        };
                        nat_mul(a4459, a4460)?
                    };
                    nat_add(a4461, a4462)?
                };
                nat_mul(a4463, a4464)?
            };
            nat_add(a4465, a4466)?
        };
        Ok(Adt9::C1(false, f88(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4472 = {
                let a4470 = {
                    let a4467 = v11;
                    let a4468 = v5;
                    let a4469 = 0u64;
                    nat_rem(a4467, a4468, a4469)
                };
                let a4471 = v4;
                nat_mul(a4470, a4471)?
            };
            let a4473 = v8;
            nat_add(a4472, a4473)?
        };
        let v16: u64 = {
            let a4479 = {
                let a4477 = {
                    let a4474 = v11;
                    let a4475 = v5;
                    let a4476 = 0u64;
                    nat_quot(a4474, a4475, a4476)
                };
                let a4478 = v4;
                nat_mul(a4477, a4478)?
            };
            let a4480 = v10;
            nat_add(a4479, a4480)?
        };
        let a4482 = v15;
        let a4483 = match v1.clone() {
            Adt3::C0(_, h4481, _, _, _, _, _, _) => {
                h4481
            }
        };
        if if nat_lt(a4482, a4483) {
            let a4485 = v16;
            let a4486 = match v1.clone() {
                Adt3::C0(_, _, h4484, _, _, _, _, _) => {
                    h4484
                }
            };
            nat_lt(a4485, a4486)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f88({
                let a4495 = v6;
                let a4496 = {
                    let a4493 = match v1.clone() {
                        Adt3::C0(h4487, _, _, _, _, _, _, _) => {
                            h4487
                        }
                    };
                    let a4494 = {
                        let a4491 = v15;
                        let a4492 = {
                            let a4489 = match v1.clone() {
                                Adt3::C0(_, h4488, _, _, _, _, _, _) => {
                                    h4488
                                }
                            };
                            let a4490 = v16;
                            nat_mul(a4489, a4490)?
                        };
                        nat_add(a4491, a4492)?
                    };
                    nat_mul(a4493, a4494)?
                };
                nat_add(a4495, a4496)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f227(23u64, v4);
    let v6: u64 = {
        let a4497 = v0;
        let a4498 = v5;
        let a4499 = 0u64;
        nat_rem(a4497, a4498, a4499)
    };
    let v7: u64 = {
        let a4500 = v0;
        let a4501 = v5;
        let a4502 = 0u64;
        nat_quot(a4500, a4501, a4502)
    };
    let v8: Adt13 = Adt13::C1(f88(v7, v2)?);
    let a4503 = v6;
    let a4504 = 0u64;
    if nat_eq(a4503, a4504) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f88(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f165(v3), 4u64))
    } else {
        let a4505 = v6;
        let a4506 = 1u64;
        if nat_eq(a4505, a4506) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4507 = v4;
                let a4508 = 0u64;
                if nat_eq(a4507, a4508) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f165(v4)))
        }
    }
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f88(v0, v2.clone())?;
    let v4: u64 = f88(v0, v1.clone())?;
    let a4513 = {
        let a4510 = v0;
        let a4511 = match v1 {
            Adt3::C0(h4509, _, _, _, _, _, _, _) => {
                h4509
            }
        };
        let a4512 = 0u64;
        nat_rem(a4510, a4511, a4512)
    };
    let a4514 = 0u64;
    if nat_eq(a4513, a4514) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f88({
            let a4515 = v0;
            let a4516 = 1u64;
            nat_sub(a4515, a4516)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4517 = v3;
    let a4518 = 13u64;
    if nat_eq(a4517, a4518) {
        f236(v0, v1, v4)
    } else {
        let v5: u64 = f237(match v1.clone() {
            Adt3::C0(h4519, _, _, _, _, _, _, _) => {
                h4519
            }
        }, {
            let a4520 = v3;
            let a4521 = 16u64;
            nat_eq(a4520, a4521)
        })?;
        let v6: u64 = {
            let a4522 = v0;
            let a4523 = v5;
            let a4524 = 0u64;
            nat_quot(a4522, a4523, a4524)
        };
        let v7: u64 = {
            let a4525 = v0;
            let a4526 = v5;
            let a4527 = 0u64;
            nat_rem(a4525, a4526, a4527)
        };
        let v8: u64 = {
            let a4531 = 1u64;
            let a4532 = {
                let a4529 = match v1.clone() {
                    Adt3::C0(h4528, _, _, _, _, _, _, _) => {
                        h4528
                    }
                };
                let a4530 = 2u64;
                nat_mul(a4529, a4530)?
            };
            nat_add(a4531, a4532)?
        };
        let a4533 = v7;
        let a4534 = v8;
        if nat_lt(a4533, a4534) {
            f238(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f88(v6, v2)?;
            let a4535 = v3;
            let a4536 = 14u64;
            if nat_eq(a4535, a4536) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4537 = v7;
                let a4538 = v8;
                if nat_eq(a4537, a4538) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4541 = v7;
                    let a4542 = {
                        let a4539 = v8;
                        let a4540 = 1u64;
                        nat_add(a4539, a4540)?
                    };
                    if nat_eq(a4541, a4542) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4543, _, _, _, _, _, _, _) => {
                                h4543
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

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4544 = v0;
    let a4545 = 0u64;
    if nat_eq(a4544, a4545) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4552 = {
            let a4546 = v0;
            let a4547 = 1u64;
            nat_add(a4546, a4547)?
        };
        let a4553 = {
            let a4550 = 2u64;
            let a4551 = {
                let a4548 = v3;
                let a4549 = 2u64;
                nat_mul(a4548, a4549)?
            };
            nat_add(a4550, a4551)?
        };
        if nat_eq(a4552, a4553) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4554 = v0;
                let a4555 = 1u64;
                nat_sub(a4554, a4555)
            };
            let v5: u64 = {
                let a4556 = v4;
                let a4557 = 2u64;
                let a4558 = 0u64;
                nat_quot(a4556, a4557, a4558)
            };
            let a4562 = {
                let a4559 = v4;
                let a4560 = 2u64;
                let a4561 = 0u64;
                nat_rem(a4559, a4560, a4561)
            };
            let a4563 = 0u64;
            if nat_eq(a4562, a4563) {
                Ok(Adt9::C4(17u64, 0u64, f88(v5, v1)?, 4u64, f88(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f158(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4564 = v0;
    let a4565 = f40(v1, v2, v3)?;
    if nat_lt(a4564, a4565) {
        let a4566 = v0;
        let a4567 = v1;
        if nat_lt(a4566, a4567) {
            Ok(Adt9::C1(false, {
                let a4568 = 4u64;
                let a4569 = v0;
                nat_mul(a4568, a4569)?
            }, {
                let a4570 = 4u64;
                let a4571 = v0;
                nat_mul(a4570, a4571)?
            }))
        } else {
            let v4: u64 = {
                let a4572 = v0;
                let a4573 = v1;
                nat_sub(a4572, a4573)
            };
            let v5: u64 = f239(v4, v2)?;
            let a4574 = v5;
            let a4575 = 2u64;
            if nat_lt(a4574, a4575) {
                Ok(Adt9::C4(6u64, {
                    let a4576 = v5;
                    let a4577 = 0u64;
                    if nat_eq(a4576, a4577) {
                        4u64
                    } else {
                        5u64
                    }
                }, f240(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f241(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f242(v4, v2)? {
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

pub fn f159(v0: u64, v1: u64) -> u64 {
    let a4578 = v0;
    let a4579 = 0u64;
    if if nat_eq(a4578, a4579) {
        true
    } else {
        let a4580 = v1;
        let a4581 = 0u64;
        nat_eq(a4580, a4581)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f160(v0: u64, v1: u64) -> Adt11 {
    let a4582 = v0;
    let a4583 = 43u64;
    if nat_eq(a4582, a4583) {
        Adt11::C8(3u64)
    } else {
        let a4584 = v1;
        let a4585 = 43u64;
        if nat_eq(a4584, a4585) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f161(v0: u64) -> R<u64> {
    let a4588 = 2u64;
    let a4589 = {
        let a4586 = 6u64;
        let a4587 = v0;
        nat_mul(a4586, a4587)?
    };
    nat_add(a4588, a4589)
}

pub fn f162(v0: u64, v1: bool) -> R<u64> {
    let a4592 = {
        let a4590 = v0;
        let a4591 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4590, a4591)?
    };
    let a4593 = 1u64;
    nat_add(a4592, a4593)
}

pub fn f163(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4594 = v0;
    let a4595 = 16u64;
    if nat_lt(a4594, a4595) {
        let v9: u64 = {
            let a4596 = v0;
            let a4597 = 4u64;
            let a4598 = 0u64;
            nat_quot(a4596, a4597, a4598)
        };
        let v10: u64 = {
            let a4603 = {
                let a4601 = 8u64;
                let a4602 = {
                    let a4599 = 4u64;
                    let a4600 = v1;
                    nat_mul(a4599, a4600)?
                };
                nat_add(a4601, a4602)?
            };
            let a4604 = v9;
            nat_add(a4603, a4604)?
        };
        let v11: u64 = f56(v6, v9, v1, v2, 0u64)?;
        let a4608 = {
            let a4605 = v0;
            let a4606 = 4u64;
            let a4607 = 0u64;
            nat_rem(a4605, a4606, a4607)
        };
        let a4609 = 0u64;
        if nat_eq(a4608, a4609) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f56(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f56(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4613 = {
                let a4610 = v0;
                let a4611 = 4u64;
                let a4612 = 0u64;
                nat_rem(a4610, a4611, a4612)
            };
            let a4614 = 1u64;
            if nat_eq(a4613, a4614) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f56(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4618 = {
                    let a4615 = v0;
                    let a4616 = 4u64;
                    let a4617 = 0u64;
                    nat_rem(a4615, a4616, a4617)
                };
                let a4619 = 2u64;
                if nat_eq(a4618, a4619) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4620 = v9;
                        let a4621 = 0u64;
                        if nat_eq(a4620, a4621) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4622 = v8;
                        let a4623 = 2u64;
                        if nat_eq(a4622, a4623) {
                            let a4624 = v9;
                            let a4625 = 0u64;
                            if nat_eq(a4624, a4625) {
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
        let a4626 = v0;
        let a4627 = 28u64;
        if nat_lt(a4626, a4627) {
            let v12: u64 = {
                let a4628 = v0;
                let a4629 = 16u64;
                nat_sub(a4628, a4629)
            };
            let v13: u64 = {
                let a4630 = v12;
                let a4631 = 3u64;
                let a4632 = 0u64;
                nat_quot(a4630, a4631, a4632)
            };
            let v14: u64 = f56(v6, v13, v1, v2, 0u64)?;
            let a4636 = {
                let a4633 = v12;
                let a4634 = 3u64;
                let a4635 = 0u64;
                nat_rem(a4633, a4634, a4635)
            };
            let a4637 = 0u64;
            if nat_eq(a4636, a4637) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4641 = {
                    let a4638 = v12;
                    let a4639 = 3u64;
                    let a4640 = 0u64;
                    nat_rem(a4638, a4639, a4640)
                };
                let a4642 = 1u64;
                if nat_eq(a4641, a4642) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4643 = v13;
                        let a4644 = 0u64;
                        if nat_eq(a4643, a4644) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4645 = v0;
            let a4646 = 28u64;
            if nat_eq(a4645, a4646) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4647 = v0;
                    let a4648 = 29u64;
                    nat_sub(a4647, a4648)
                };
                let v16: u64 = {
                    let a4649 = v15;
                    let a4650 = 2u64;
                    let a4651 = 0u64;
                    nat_quot(a4649, a4650, a4651)
                };
                let v17: u64 = f56(v6, v16, v1, v2, 0u64)?;
                let a4655 = {
                    let a4652 = v15;
                    let a4653 = 2u64;
                    let a4654 = 0u64;
                    nat_rem(a4652, a4653, a4654)
                };
                let a4656 = 0u64;
                if nat_eq(a4655, a4656) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f164(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4657 = v0;
        let a4658 = 9u64;
        let a4659 = 0u64;
        nat_rem(a4657, a4658, a4659)
    };
    let v5: u64 = {
        let a4663 = {
            let a4660 = v0;
            let a4661 = 9u64;
            let a4662 = 0u64;
            nat_quot(a4660, a4661, a4662)
        };
        let a4664 = 4u64;
        let a4665 = 0u64;
        nat_rem(a4663, a4664, a4665)
    };
    let v6: bool = {
        let a4672 = {
            let a4669 = {
                let a4666 = v0;
                let a4667 = 36u64;
                let a4668 = 0u64;
                nat_quot(a4666, a4667, a4668)
            };
            let a4670 = 2u64;
            let a4671 = 0u64;
            nat_rem(a4669, a4670, a4671)
        };
        let a4673 = 0u64;
        nat_eq(a4672, a4673)
    };
    let v7: u64 = {
        let a4674 = v4;
        let a4675 = 4u64;
        if nat_lt(a4674, a4675) {
            v4
        } else {
            let a4676 = v4;
            let a4677 = 5u64;
            nat_sub(a4676, a4677)
        }
    };
    let v8: u64 = f56(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a4678 = v4;
    let a4679 = 4u64;
    if nat_lt(a4678, a4679) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4680 = v4;
            let a4681 = 0u64;
            if nat_eq(a4680, a4681) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4682 = v4;
        let a4683 = 4u64;
        if nat_eq(a4682, a4683) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f165(v0: u64) -> u64 {
    let m4684 = f206(v0);
    match m4684 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4686 = match v1.clone() {
                Adt19::C0(_, h4685, _) => {
                    h4685
                }
            };
            let a4687 = 1u64;
            if if nat_eq(a4686, a4687) {
                let a4689 = match v1.clone() {
                    Adt19::C0(_, _, h4688) => {
                        h4688
                    }
                };
                let a4690 = 1u64;
                if if if nat_eq(a4689, a4690) {
                    true
                } else {
                    let a4692 = match v1.clone() {
                        Adt19::C0(_, _, h4691) => {
                            h4691
                        }
                    };
                    let a4693 = 2u64;
                    nat_eq(a4692, a4693)
                } {
                    true
                } else {
                    let a4695 = match v1.clone() {
                        Adt19::C0(_, _, h4694) => {
                            h4694
                        }
                    };
                    let a4696 = 4u64;
                    nat_eq(a4695, a4696)
                } {
                    true
                } else {
                    let a4698 = match v1.clone() {
                        Adt19::C0(_, _, h4697) => {
                            h4697
                        }
                    };
                    let a4699 = 8u64;
                    nat_eq(a4698, a4699)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4700) => {
                        h4700
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f166(v0: u64) -> R<u64> {
    let a4703 = 3u64;
    let a4704 = {
        let a4701 = v0;
        let a4702 = 3u64;
        nat_mul(a4701, a4702)?
    };
    nat_add(a4703, a4704)
}

pub fn f167(v0: u64, v1: u64) -> R<u64> {
    if f168(v0) {
        let a4705 = 2u64;
        let a4706 = v1;
        nat_mul(a4705, a4706)
    } else {
        let a4707 = v1;
        let a4708 = f243(v0);
        let a4709 = 0u64;
        Ok(nat_quot(a4707, a4708, a4709))
    }
}

pub fn f168(v0: u64) -> bool {
    let a4710 = v0;
    let a4711 = 0u64;
    if if nat_eq(a4710, a4711) {
        true
    } else {
        let a4712 = v0;
        let a4713 = 1u64;
        nat_eq(a4712, a4713)
    } {
        true
    } else {
        let a4714 = v0;
        let a4715 = 30u64;
        nat_eq(a4714, a4715)
    }
}

pub fn f169(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4716 = f206(v0);
    match m4716 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f135(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4717, _, _, _) => {
                    h4717
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4718, _, _) => {
                    h4718
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4719, _) => {
                    h4719
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4720) => {
                    h4720
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4721) => {
                    h4721
                }
            }, f135({
                let a4723 = v2;
                let a4724 = match v6.clone() {
                    Adt19::C0(_, h4722, _) => {
                        h4722
                    }
                };
                nat_mul(a4723, a4724)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4725, _, _, _) => {
                    h4725
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4726, _, _) => {
                    h4726
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4727, _) => {
                    h4727
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4728) => {
                    h4728
                }
            })?, match v6 {
                Adt19::C0(_, h4729, _) => {
                    h4729
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4730, _, _, _) => {
                    h4730
                }
            }))
        }
    }
}

pub fn f170(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4738 = {
        let a4731 = 2u64;
        let a4732 = f171(v1.clone())?;
        nat_mul(a4731, a4732)?
    };
    let a4739 = {
        let a4736 = f171(v1)?;
        let a4737 = {
            let a4734 = 1u64;
            let a4735 = match v0 {
                Adt3::C0(h4733, _, _, _, _, _, _, _) => {
                    h4733
                }
            };
            nat_add(a4734, a4735)?
        };
        nat_mul(a4736, a4737)?
    };
    nat_add(a4738, a4739)
}

pub fn f171(v0: Adt3) -> R<u64> {
    if f47(match v0.clone() {
        Adt3::C0(h4740, _, _, _, _, _, _, _) => {
            h4740
        }
    }, match v0.clone() {
        Adt3::C0(_, h4741, _, _, _, _, _, _) => {
            h4741
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4742, _, _, _, _, _) => {
            h4742
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4743, _, _, _, _) => {
            h4743
        }
    }) {
        Ok(0u64)
    } else {
        let a4752 = match v0.clone() {
            Adt3::C0(h4744, _, _, _, _, _, _, _) => {
                h4744
            }
        };
        let a4753 = {
            let a4750 = match v0.clone() {
                Adt3::C0(_, h4745, _, _, _, _, _, _) => {
                    h4745
                }
            };
            let a4751 = {
                let a4748 = match v0.clone() {
                    Adt3::C0(_, _, h4746, _, _, _, _, _) => {
                        h4746
                    }
                };
                let a4749 = match v0 {
                    Adt3::C0(_, _, _, h4747, _, _, _, _) => {
                        h4747
                    }
                };
                nat_mul(a4748, a4749)?
            };
            nat_mul(a4750, a4751)?
        };
        nat_mul(a4752, a4753)
    }
}

pub fn f172(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f88(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f173(v0: u64, v1: u64) -> bool {
    let a4754 = v1;
    let a4755 = 2147483647u64;
    if nat_le(a4754, a4755) {
        let a4756 = v1;
        let a4757 = v0;
        nat_lt(a4756, a4757)
    } else {
        false
    }
}

pub fn f174(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4758 = f206(v0);
    match m4758 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a4760 = v2;
                let a4761 = match v7.clone() {
                    Adt19::C0(_, h4759, _) => {
                        h4759
                    }
                };
                nat_mul(a4760, a4761)?
            };
            let m4774 = f244(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4762, _, _, _, _, _, _, _) => {
                    h4762
                }
            }, match v5.clone() {
                Adt3::C0(_, h4763, _, _, _, _, _, _) => {
                    h4763
                }
            }, match v5 {
                Adt3::C0(_, _, h4764, _, _, _, _, _) => {
                    h4764
                }
            }, {
                let a4767 = match v3.clone() {
                    Adt3::C0(h4765, _, _, _, _, _, _, _) => {
                        h4765
                    }
                };
                let a4768 = match v7.clone() {
                    Adt19::C0(_, h4766, _) => {
                        h4766
                    }
                };
                nat_mul(a4767, a4768)?
            }, match v3.clone() {
                Adt3::C0(_, h4769, _, _, _, _, _, _) => {
                    h4769
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4770, _, _, _) => {
                    h4770
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4771, _, _) => {
                    h4771
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4772, _) => {
                    h4772
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4773) => {
                    h4773
                }
            })?;
            match m4774 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4775 = v0;
                    let a4776 = 0u64;
                    if if nat_eq(a4775, a4776) {
                        true
                    } else {
                        let a4777 = v0;
                        let a4778 = 26u64;
                        nat_eq(a4777, a4778)
                    } {
                        Ok(Adt9::C1(false, v9, f88({
                            let a4782 = {
                                let a4780 = v1;
                                let a4781 = match v4.clone() {
                                    Adt3::C0(h4779, _, _, _, _, _, _, _) => {
                                        h4779
                                    }
                                };
                                nat_mul(a4780, a4781)?
                            };
                            let a4783 = v8;
                            nat_add(a4782, a4783)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h4784) => {
                                h4784
                            }
                        }, f88({
                            let a4788 = {
                                let a4786 = v1;
                                let a4787 = match v4.clone() {
                                    Adt3::C0(h4785, _, _, _, _, _, _, _) => {
                                        h4785
                                    }
                                };
                                nat_mul(a4786, a4787)?
                            };
                            let a4789 = v8;
                            nat_add(a4788, a4789)?
                        }, v4.clone())?, {
                            let a4796 = {
                                let a4794 = {
                                    let a4791 = match v7 {
                                        Adt19::C0(_, h4790, _) => {
                                            h4790
                                        }
                                    };
                                    let a4792 = 1u64;
                                    nat_sub(a4791, a4792)
                                };
                                let a4795 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4793, _, _, _) => {
                                        h4793
                                    }
                                };
                                nat_mul(a4794, a4795)?
                            };
                            let a4797 = 4u64;
                            nat_add(a4796, a4797)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4798, _, _, _) => {
                                h4798
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f175(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4799 = v1;
    let a4800 = 0u64;
    if if nat_eq(a4799, a4800) {
        true
    } else {
        let a4801 = v2;
        let a4802 = 0u64;
        nat_eq(a4801, a4802)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f176(v0: u64, v1: u64) -> bool {
    let a4803 = v0;
    let a4804 = 4294967295u64;
    if nat_eq(a4803, a4804) {
        true
    } else {
        let a4805 = v0;
        let a4806 = 2147483648u64;
        if nat_lt(a4805, a4806) {
            let a4807 = v0;
            let a4808 = v1;
            nat_lt(a4807, a4808)
        } else {
            false
        }
    }
}

pub fn f177(v0: Adt7) -> R<u64> {
    let a4820 = if match v0.clone() {
        Adt7::C0(_, _, _, h4809, _) => {
            h4809
        }
    } {
        2u64
    } else {
        3u64
    };
    let a4821 = {
        let a4818 = {
            let a4816 = {
                let m4812 = match v0.clone() {
                    Adt7::C0(h4811, _, _, _, _) => {
                        h4811
                    }
                };
                match m4812 {
                    Adt6::C0(h4810, _, _, _) => {
                        h4810
                    }
                }
            };
            let a4817 = {
                let m4815 = match v0.clone() {
                    Adt7::C0(_, h4814, _, _, _) => {
                        h4814
                    }
                };
                match m4815 {
                    Adt6::C0(h4813, _, _, _) => {
                        h4813
                    }
                }
            };
            nat_mul(a4816, a4817)?
        };
        let a4819 = f178(v0)?;
        nat_mul(a4818, a4819)?
    };
    nat_add(a4820, a4821)
}

pub fn f178(v0: Adt7) -> R<u64> {
    let a4826 = if match v0.clone() {
        Adt7::C0(_, _, _, h4822, _) => {
            h4822
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4827 = {
        let a4824 = match v0 {
            Adt7::C0(_, _, _, _, h4823) => {
                h4823
            }
        };
        let a4825 = 1u64;
        if nat_eq(a4824, a4825) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4826, a4827)
}

pub fn f179(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4832 = {
            let a4829 = v2;
            let a4830 = match v0.clone() {
                Adt6::C0(_, h4828, _, _) => {
                    h4828
                }
            };
            nat_mul(a4829, a4830)?
        };
        let a4833 = f136(match v0.clone() {
            Adt6::C0(_, _, h4831, _) => {
                h4831
            }
        });
        nat_add(a4832, a4833)?
    };
    let v4: u64 = f137(match v0.clone() {
        Adt6::C0(_, _, h4834, _) => {
            h4834
        }
    });
    let v5: u64 = {
        let a4838 = {
            let a4836 = v3;
            let a4837 = match v0 {
                Adt6::C0(h4835, _, _, _) => {
                    h4835
                }
            };
            nat_add(a4836, a4837)?
        };
        let a4839 = v4;
        nat_sub(a4838, a4839)
    };
    let a4844 = {
        let a4840 = v5;
        let a4841 = v1;
        if nat_lt(a4840, a4841) {
            v5
        } else {
            v1
        }
    };
    let a4845 = {
        let a4842 = v3;
        let a4843 = v4;
        nat_sub(a4842, a4843)
    };
    Ok(nat_sub(a4844, a4845))
}

pub fn f180(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4854 = f59(match v0.clone() {
        Adt7::C0(h4846, _, _, _, _) => {
            h4846
        }
    }, match v1.clone() {
        Adt3::C0(h4847, _, _, _, _, _, _, _) => {
            h4847
        }
    }, f57(v3, v2.clone(), 0u64), {
        let a4851 = v4;
        let a4852 = {
            let m4850 = match v0.clone() {
                Adt7::C0(h4849, _, _, _, _) => {
                    h4849
                }
            };
            match m4850 {
                Adt6::C0(h4848, _, _, _) => {
                    h4848
                }
            }
        };
        let a4853 = 0u64;
        nat_rem(a4851, a4852, a4853)
    })?;
    match m4854 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4863 = f59(match v0.clone() {
                Adt7::C0(_, h4855, _, _, _) => {
                    h4855
                }
            }, match v1.clone() {
                Adt3::C0(_, h4856, _, _, _, _, _, _) => {
                    h4856
                }
            }, f57(v3, v2.clone(), 1u64), {
                let a4860 = v4;
                let a4861 = {
                    let m4859 = match v0 {
                        Adt7::C0(h4858, _, _, _, _) => {
                            h4858
                        }
                    };
                    match m4859 {
                        Adt6::C0(h4857, _, _, _) => {
                            h4857
                        }
                    }
                };
                let a4862 = 0u64;
                nat_quot(a4860, a4861, a4862)
            })?;
            match m4863 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f56(v1, v5, v6, f57(v3, v2.clone(), 2u64), f57(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f181(v0: u64) -> R<u64> {
    let a4866 = 2u64;
    let a4867 = {
        let a4864 = v0;
        let a4865 = 2u64;
        nat_mul(a4864, a4865)?
    };
    nat_add(a4866, a4867)
}

pub fn f182(v0: Adt6) -> R<u64> {
    let a4880 = 2u64;
    let a4881 = {
        let a4878 = 2u64;
        let a4879 = {
            let a4875 = {
                let a4872 = match v0.clone() {
                    Adt6::C0(h4868, _, _, _) => {
                        h4868
                    }
                };
                let a4873 = {
                    let a4870 = match v0.clone() {
                        Adt6::C0(_, h4869, _, _) => {
                            h4869
                        }
                    };
                    let a4871 = 1u64;
                    nat_sub(a4870, a4871)
                };
                nat_add(a4872, a4873)?
            };
            let a4876 = match v0 {
                Adt6::C0(_, h4874, _, _) => {
                    h4874
                }
            };
            let a4877 = 0u64;
            nat_quot(a4875, a4876, a4877)
        };
        nat_mul(a4878, a4879)?
    };
    nat_add(a4880, a4881)
}

pub fn f183(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m4882 = v0;
    match m4882 {
        Adt16::C0(v19) => {
            f245(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4883 = v20;
            let a4884 = 66u64;
            if nat_eq(a4883, a4884) {
                f246(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f247(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f184(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4891 = v3;
        let a4892 = {
            let a4888 = f88({
                let a4885 = v0;
                let a4886 = 2u64;
                let a4887 = 0u64;
                nat_quot(a4885, a4886, a4887)
            }, v1)?;
            let a4889 = v2;
            let a4890 = 0u64;
            nat_quot(a4888, a4889, a4890)
        };
        nat_add(a4891, a4892)?
    };
    let a4896 = {
        let a4893 = v0;
        let a4894 = 2u64;
        let a4895 = 0u64;
        nat_rem(a4893, a4894, a4895)
    };
    let a4897 = 0u64;
    if nat_eq(a4896, a4897) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4898 = v4;
        let a4899 = 0u64;
        if nat_eq(a4898, a4899) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f185(v0: u64, v1: u64) -> R<u64> {
    let a4902 = 8u64;
    let a4903 = {
        let a4900 = f248(v0);
        let a4901 = v1;
        nat_mul(a4900, a4901)?
    };
    nat_add(a4902, a4903)
}

pub fn f186(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4904, _, _, _, _, _, _, _) => {
            h4904
        }
    };
    let v7: u64 = f248(v3);
    let v8: u64 = {
        let a4907 = 1u64;
        let a4908 = {
            let a4905 = v6;
            let a4906 = v7;
            nat_mul(a4905, a4906)?
        };
        nat_add(a4907, a4908)?
    };
    let a4909 = v0;
    let a4910 = 0u64;
    if nat_eq(a4909, a4910) {
        Ok(f249(0u64))
    } else {
        let a4911 = v0;
        let a4912 = v8;
        if nat_lt(a4911, a4912) {
            let v9: u64 = {
                let a4913 = v0;
                let a4914 = 1u64;
                nat_sub(a4913, a4914)
            };
            let v10: u64 = {
                let a4915 = v9;
                let a4916 = v7;
                let a4917 = 0u64;
                nat_rem(a4915, a4916, a4917)
            };
            let v11: u64 = f88({
                let a4923 = {
                    let a4918 = v1;
                    let a4919 = v6;
                    nat_mul(a4918, a4919)?
                };
                let a4924 = {
                    let a4920 = v9;
                    let a4921 = v7;
                    let a4922 = 0u64;
                    nat_quot(a4920, a4921, a4922)
                };
                nat_add(a4923, a4924)?
            }, v2)?;
            let a4925 = v3;
            let a4926 = 30u64;
            if nat_eq(a4925, a4926) {
                let a4927 = v10;
                let a4928 = 0u64;
                if nat_eq(a4927, a4928) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4929 = v10;
                    let a4930 = 1u64;
                    if nat_eq(a4929, a4930) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4931 = v10;
                        let a4932 = 2u64;
                        if nat_eq(a4931, a4932) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4933 = v10;
                let a4934 = 0u64;
                if nat_eq(a4933, a4934) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4935 = v10;
                    let a4936 = 1u64;
                    if nat_eq(a4935, a4936) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4937 = v0;
                let a4938 = v8;
                nat_sub(a4937, a4938)
            };
            let a4939 = v12;
            let a4940 = 0u64;
            if nat_eq(a4939, a4940) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4941 = v12;
                let a4942 = 1u64;
                if nat_eq(a4941, a4942) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4943 = v12;
                    let a4944 = 2u64;
                    if nat_eq(a4943, a4944) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4945 = v12;
                        let a4946 = 3u64;
                        if nat_eq(a4945, a4946) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4947 = v12;
                            let a4948 = 4u64;
                            if nat_eq(a4947, a4948) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4949 = v12;
                                let a4950 = 5u64;
                                if nat_eq(a4949, a4950) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4951 = v12;
                                    let a4952 = 6u64;
                                    if nat_eq(a4951, a4952) {
                                        if f250(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4953 = 3u64;
                                                let a4954 = v1;
                                                nat_add(a4953, a4954)?
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

pub fn f187(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4955 = v0;
        let a4956 = f99(v3);
        let a4957 = 0u64;
        nat_quot(a4955, a4956, a4957)
    };
    let v5: u64 = {
        let a4962 = 3u64;
        let a4963 = {
            let a4959 = v4;
            let a4960 = match v1.clone() {
                Adt3::C0(h4958, _, _, _, _, _, _, _) => {
                    h4958
                }
            };
            let a4961 = 0u64;
            nat_quot(a4959, a4960, a4961)
        };
        nat_add(a4962, a4963)?
    };
    let v6: u64 = f88(v4, v1)?;
    let v7: u64 = f88(v4, v2)?;
    let a4964 = v3;
    let a4965 = 30u64;
    if nat_eq(a4964, a4965) {
        let a4969 = {
            let a4966 = v0;
            let a4967 = 3u64;
            let a4968 = 0u64;
            nat_rem(a4966, a4967, a4968)
        };
        let a4970 = 0u64;
        if nat_eq(a4969, a4970) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4974 = {
                let a4971 = v0;
                let a4972 = 3u64;
                let a4973 = 0u64;
                nat_rem(a4971, a4972, a4973)
            };
            let a4975 = 1u64;
            if nat_eq(a4974, a4975) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f188(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4976 = v0;
        let a4977 = 8388608u64;
        let a4978 = 0u64;
        nat_quot(a4976, a4977, a4978)
    };
    let a4979 = v0;
    let a4980 = 2147483648u64;
    if if if nat_lt(a4979, a4980) {
        let a4981 = 127u64;
        let a4982 = v1;
        nat_le(a4981, a4982)
    } else {
        false
    } {
        let a4983 = v1;
        let a4984 = 190u64;
        nat_lt(a4983, a4984)
    } else {
        false
    } {
        let v2: u64 = {
            let a4988 = 8388608u64;
            let a4989 = {
                let a4985 = v0;
                let a4986 = 8388608u64;
                let a4987 = 0u64;
                nat_rem(a4985, a4986, a4987)
            };
            nat_add(a4988, a4989)?
        };
        let a4990 = v1;
        let a4991 = 150u64;
        if nat_lt(a4990, a4991) {
            let v3: u64 = f251({
                let a4992 = 150u64;
                let a4993 = v1;
                nat_sub(a4992, a4993)
            });
            let a4997 = {
                let a4994 = v2;
                let a4995 = v3;
                let a4996 = 0u64;
                nat_rem(a4994, a4995, a4996)
            };
            let a4998 = 0u64;
            if nat_eq(a4997, a4998) {
                let a4999 = v2;
                let a5000 = v3;
                let a5001 = 0u64;
                Ok(nat_quot(a4999, a5000, a5001))
            } else {
                Ok(0u64)
            }
        } else {
            let a5004 = f251({
                let a5002 = v1;
                let a5003 = 150u64;
                nat_sub(a5002, a5003)
            });
            let a5005 = v2;
            nat_mul(a5004, a5005)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f189(v0: Adt14, v1: Adt14) -> bool {
    let a5008 = match v0.clone() {
        Adt14::C0(_, h5006, _, _, _, _, _, _, _, _, _) => {
            h5006
        }
    };
    let a5009 = match v1.clone() {
        Adt14::C0(_, h5007, _, _, _, _, _, _, _, _, _) => {
            h5007
        }
    };
    if if if nat_eq(a5008, a5009) {
        let a5012 = match v0.clone() {
            Adt14::C0(_, _, h5010, _, _, _, _, _, _, _, _) => {
                h5010
            }
        };
        let a5013 = match v1.clone() {
            Adt14::C0(_, _, h5011, _, _, _, _, _, _, _, _) => {
                h5011
            }
        };
        nat_eq(a5012, a5013)
    } else {
        false
    } {
        let a5016 = match v0.clone() {
            Adt14::C0(_, _, _, h5014, _, _, _, _, _, _, _) => {
                h5014
            }
        };
        let a5017 = match v1.clone() {
            Adt14::C0(_, _, _, h5015, _, _, _, _, _, _, _) => {
                h5015
            }
        };
        nat_eq(a5016, a5017)
    } else {
        false
    } {
        let a5020 = match v0 {
            Adt14::C0(_, _, _, _, h5018, _, _, _, _, _, _) => {
                h5018
            }
        };
        let a5021 = match v1 {
            Adt14::C0(_, _, _, _, h5019, _, _, _, _, _, _) => {
                h5019
            }
        };
        nat_eq(a5020, a5021)
    } else {
        false
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5022 = v0;
    let a5023 = 0u64;
    if if if nat_eq(a5022, a5023) {
        true
    } else {
        let a5024 = v1;
        let a5025 = 0u64;
        nat_eq(a5024, a5025)
    } {
        true
    } else {
        let a5026 = v2;
        let a5027 = 0u64;
        nat_eq(a5026, a5027)
    } {
        true
    } else {
        let a5028 = v3;
        let a5029 = 0u64;
        nat_eq(a5028, a5029)
    }
}

pub fn f191(v0: Adt14, v1: Adt14) -> bool {
    if f197(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5030, _, _, _, _, _) => {
            h5030
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5031, _, _, _, _, _) => {
            h5031
        }
    }) {
        let a5033 = match v0 {
            Adt14::C0(_, h5032, _, _, _, _, _, _, _, _, _) => {
                h5032
            }
        };
        let a5034 = 2147483647u64;
        nat_le(a5033, a5034)
    } else {
        true
    }
}

pub fn f192(v0: Adt14) -> bool {
    let a5039 = {
        let a5036 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5035) => {
                h5035
            }
        };
        let a5037 = 4u64;
        let a5038 = 0u64;
        nat_rem(a5036, a5037, a5038)
    };
    let a5040 = 0u64;
    if if nat_eq(a5039, a5040) {
        f252(match v0.clone() {
            Adt14::C0(_, h5041, _, _, _, _, _, _, _, _, _) => {
                h5041
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5042, _, _, _, _, _) => {
                h5042
            }
        })
    } else {
        false
    } {
        f95(match v0.clone() {
            Adt14::C0(_, _, h5043, _, _, _, _, _, _, _, _) => {
                h5043
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5044, _, _, _, _, _, _, _) => {
                h5044
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5045, _, _, _, _, _, _) => {
                h5045
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5046, _, _, _, _) => {
                h5046
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5047, _, _, _) => {
                h5047
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5048, _, _) => {
                h5048
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f193(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5050 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5049, _) => {
            h5049
        }
    };
    let a5051 = 0u64;
    if nat_eq(a5050, a5051) {
        Ok(true)
    } else {
        let a5053 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5052, _) => {
                h5052
            }
        };
        let a5054 = 0u64;
        if nat_eq(a5053, a5054) {
            Ok(true)
        } else {
            let a5056 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5055) => {
                    h5055
                }
            };
            let a5057 = 0u64;
            if nat_eq(a5056, a5057) {
                Ok(true)
            } else {
                let a5059 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5058) => {
                        h5058
                    }
                };
                let a5060 = 0u64;
                if nat_eq(a5059, a5060) {
                    Ok(true)
                } else {
                    let a5066 = {
                        let a5063 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5061) => {
                                h5061
                            }
                        };
                        let a5064 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5062, _) => {
                                h5062
                            }
                        };
                        nat_add(a5063, a5064)?
                    };
                    let a5067 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5065) => {
                            h5065
                        }
                    };
                    if nat_le(a5066, a5067) {
                        Ok(true)
                    } else {
                        let a5073 = {
                            let a5070 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5068) => {
                                    h5068
                                }
                            };
                            let a5071 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5069, _) => {
                                    h5069
                                }
                            };
                            nat_add(a5070, a5071)?
                        };
                        let a5074 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5072) => {
                                h5072
                            }
                        };
                        Ok(nat_le(a5073, a5074))
                    }
                }
            }
        }
    }
}

pub fn f194(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f189(v0.clone(), v1.clone()) {
        let a5077 = match v0.clone() {
            Adt14::C0(h5075, _, _, _, _, _, _, _, _, _, _) => {
                h5075
            }
        };
        let a5078 = match v1.clone() {
            Adt14::C0(h5076, _, _, _, _, _, _, _, _, _, _) => {
                h5076
            }
        };
        nat_eq(a5077, a5078)
    } else {
        false
    } {
        let a5081 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5079) => {
                h5079
            }
        };
        let a5082 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5080) => {
                h5080
            }
        };
        nat_eq(a5081, a5082)
    } else {
        false
    } {
        let a5085 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5083, _, _, _, _, _) => {
                h5083
            }
        };
        let a5086 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5084, _, _, _, _, _) => {
                h5084
            }
        };
        nat_eq(a5085, a5086)
    } else {
        false
    } {
        let a5089 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5087, _, _, _, _) => {
                h5087
            }
        };
        let a5090 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5088, _, _, _, _) => {
                h5088
            }
        };
        nat_eq(a5089, a5090)
    } else {
        false
    } {
        let a5093 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5091, _, _, _) => {
                h5091
            }
        };
        let a5094 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5092, _, _, _) => {
                h5092
            }
        };
        nat_eq(a5093, a5094)
    } else {
        false
    } {
        let a5097 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5095, _, _) => {
                h5095
            }
        };
        let a5098 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5096, _, _) => {
                h5096
            }
        };
        nat_eq(a5097, a5098)
    } else {
        false
    }
}

pub fn f195(v0: Adt14) -> R<bool> {
    if f253(match v0.clone() {
        Adt14::C0(_, h5099, _, _, _, _, _, _, _, _, _) => {
            h5099
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5100, _, _, _, _, _) => {
            h5100
        }
    }) {
        f96(f254(match v0.clone() {
            Adt14::C0(_, h5101, _, _, _, _, _, _, _, _, _) => {
                h5101
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5102, _, _, _, _, _) => {
                h5102
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5103, _, _, _, _, _, _, _, _) => {
                h5103
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5104, _, _, _, _, _, _, _) => {
                h5104
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5105, _, _, _, _, _, _) => {
                h5105
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5106, _, _, _, _) => {
                h5106
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5107, _, _, _) => {
                h5107
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5108, _, _) => {
                h5108
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f196(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5109 = v1;
    let a5110 = 0u64;
    if if nat_eq(a5109, a5110) {
        true
    } else {
        let a5111 = v2;
        let a5112 = 0u64;
        nat_eq(a5111, a5112)
    } {
        Ok(0u64)
    } else {
        let a5115 = f97(v1, v2, v3)?;
        let a5116 = {
            let a5113 = v2;
            let a5114 = f255(v0, v1)?;
            nat_mul(a5113, a5114)?
        };
        nat_add(a5115, a5116)
    }
}

pub fn f197(v0: u64, v1: u64) -> bool {
    let a5117 = v0;
    let a5118 = 4u64;
    if nat_eq(a5117, a5118) {
        let a5119 = v1;
        let a5120 = 4u64;
        nat_eq(a5119, a5120)
    } else {
        false
    }
}

pub fn f198(v0: u64) -> R<u64> {
    let a5121 = 6u64;
    let a5122 = v0;
    nat_add(a5121, a5122)
}

pub fn f199(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5123, _, _, _, _, _, _, _, _, _, _) => {
            h5123
        }
    }, match v0.clone() {
        Adt14::C0(_, h5124, _, _, _, _, _, _, _, _, _) => {
            h5124
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5125, _, _, _, _, _, _, _, _) => {
            h5125
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5126, _, _, _, _, _, _, _) => {
            h5126
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5127, _, _, _, _, _, _) => {
            h5127
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5128, _, _, _, _) => {
            h5128
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5129, _, _, _) => {
            h5129
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5130, _, _) => {
            h5130
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5131, _) => {
            h5131
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5132) => {
            h5132
        }
    })
}

pub fn f200(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5133 = v0;
    let a5134 = 2147483647u64;
    if if if nat_le(a5133, a5134) {
        let a5135 = 0u64;
        let a5136 = v3;
        if nat_lt(a5135, a5136) {
            let a5137 = v3;
            let a5138 = 2147483647u64;
            nat_le(a5137, a5138)
        } else {
            false
        }
    } else {
        false
    } {
        let a5143 = {
            let a5141 = {
                let a5139 = v2;
                let a5140 = v3;
                nat_add(a5139, a5140)?
            };
            let a5142 = 1u64;
            nat_sub(a5141, a5142)
        };
        let a5144 = 2147483647u64;
        nat_le(a5143, a5144)
    } else {
        false
    } {
        let a5149 = {
            let a5147 = {
                let a5145 = v0;
                let a5146 = v1;
                nat_mul(a5145, a5146)?
            };
            let a5148 = f256(v2, v3)?;
            nat_mul(a5147, a5148)?
        };
        let a5150 = 9223372036854775807u64;
        Ok(nat_le(a5149, a5150))
    } else {
        Ok(false)
    }
}

pub fn f201(v0: Adt14) -> R<u64> {
    let a5181 = {
        let a5173 = {
            let a5165 = {
                let a5157 = 4u64;
                let a5158 = {
                    let a5155 = {
                        let a5152 = match v0.clone() {
                            Adt14::C0(_, h5151, _, _, _, _, _, _, _, _, _) => {
                                h5151
                            }
                        };
                        let a5153 = 1u64;
                        nat_sub(a5152, a5153)
                    };
                    let a5156 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5154, _, _, _, _, _) => {
                            h5154
                        }
                    };
                    nat_mul(a5155, a5156)?
                };
                nat_add(a5157, a5158)?
            };
            let a5166 = {
                let a5163 = {
                    let a5160 = match v0.clone() {
                        Adt14::C0(_, _, h5159, _, _, _, _, _, _, _, _) => {
                            h5159
                        }
                    };
                    let a5161 = 1u64;
                    nat_sub(a5160, a5161)
                };
                let a5164 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5162, _, _, _, _) => {
                        h5162
                    }
                };
                nat_mul(a5163, a5164)?
            };
            nat_add(a5165, a5166)?
        };
        let a5174 = {
            let a5171 = {
                let a5168 = match v0.clone() {
                    Adt14::C0(_, _, _, h5167, _, _, _, _, _, _, _) => {
                        h5167
                    }
                };
                let a5169 = 1u64;
                nat_sub(a5168, a5169)
            };
            let a5172 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5170, _, _, _) => {
                    h5170
                }
            };
            nat_mul(a5171, a5172)?
        };
        nat_add(a5173, a5174)?
    };
    let a5182 = {
        let a5179 = {
            let a5176 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5175, _, _, _, _, _, _) => {
                    h5175
                }
            };
            let a5177 = 1u64;
            nat_sub(a5176, a5177)
        };
        let a5180 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5178, _, _) => {
                h5178
            }
        };
        nat_mul(a5179, a5180)?
    };
    nat_add(a5181, a5182)
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5183 = v0;
    let a5184 = 0u64;
    if if if if nat_eq(a5183, a5184) {
        true
    } else {
        let a5185 = v1;
        let a5186 = 0u64;
        nat_eq(a5185, a5186)
    } {
        true
    } else {
        let a5187 = v2;
        let a5188 = 0u64;
        nat_eq(a5187, a5188)
    } {
        true
    } else {
        let a5189 = v3;
        let a5190 = 0u64;
        nat_eq(a5189, a5190)
    } {
        Ok(0u64)
    } else {
        let a5191 = f257(v0, v1, v2, v3, v5)?;
        let a5192 = f258(v0, v1, v2, v3, v4)?;
        nat_add(a5191, a5192)
    }
}

pub fn f203(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5193 = v0;
    let a5194 = 0u64;
    if if nat_eq(a5193, a5194) {
        true
    } else {
        let a5195 = v1;
        let a5196 = 0u64;
        nat_eq(a5195, a5196)
    } {
        Ok(0u64)
    } else {
        let a5199 = f97(v0, v1, v2)?;
        let a5200 = {
            let a5197 = v1;
            let a5198 = f259(v0)?;
            nat_mul(a5197, a5198)?
        };
        nat_add(a5199, a5200)
    }
}

pub fn f204(v0: u64, v1: u64, v2: u64) -> bool {
    let a5201 = v0;
    let a5202 = 1u64;
    if nat_le(a5201, a5202) {
        true
    } else {
        let a5206 = {
            let a5203 = v1;
            let a5204 = v2;
            let a5205 = 1u64;
            nat_rem(a5203, a5204, a5205)
        };
        let a5207 = 0u64;
        nat_eq(a5206, a5207)
    }
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5208 = v1;
    let a5209 = 1u64;
    if if if nat_le(a5208, a5209) {
        true
    } else {
        let a5210 = v0;
        let a5211 = v2;
        nat_le(a5210, a5211)
    } {
        let a5212 = v3;
        let a5213 = 1u64;
        if nat_le(a5212, a5213) {
            true
        } else {
            let a5220 = {
                let a5218 = v0;
                let a5219 = {
                    let a5216 = {
                        let a5214 = v1;
                        let a5215 = 1u64;
                        nat_sub(a5214, a5215)
                    };
                    let a5217 = v2;
                    nat_mul(a5216, a5217)?
                };
                nat_add(a5218, a5219)?
            };
            let a5221 = v4;
            nat_le(a5220, a5221)
        }
    } else {
        false
    } {
        let a5222 = v5;
        let a5223 = 1u64;
        if nat_le(a5222, a5223) {
            Ok(true)
        } else {
            let a5236 = {
                let a5234 = {
                    let a5228 = v0;
                    let a5229 = {
                        let a5226 = {
                            let a5224 = v1;
                            let a5225 = 1u64;
                            nat_sub(a5224, a5225)
                        };
                        let a5227 = v2;
                        nat_mul(a5226, a5227)?
                    };
                    nat_add(a5228, a5229)?
                };
                let a5235 = {
                    let a5232 = {
                        let a5230 = v3;
                        let a5231 = 1u64;
                        nat_sub(a5230, a5231)
                    };
                    let a5233 = v4;
                    nat_mul(a5232, a5233)?
                };
                nat_add(a5234, a5235)?
            };
            let a5237 = v6;
            Ok(nat_le(a5236, a5237))
        }
    } else {
        Ok(false)
    }
}

pub fn f206(v0: u64) -> Adt18 {
    let a5238 = v0;
    let a5239 = 0u64;
    if nat_eq(a5238, a5239) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5240 = v0;
        let a5241 = 1u64;
        if nat_eq(a5240, a5241) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5242 = v0;
            let a5243 = 2u64;
            if nat_eq(a5242, a5243) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5244 = v0;
                let a5245 = 3u64;
                if nat_eq(a5244, a5245) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5246 = v0;
                    let a5247 = 6u64;
                    if nat_eq(a5246, a5247) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5248 = v0;
                        let a5249 = 7u64;
                        if nat_eq(a5248, a5249) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5250 = v0;
                            let a5251 = 8u64;
                            if nat_eq(a5250, a5251) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5252 = v0;
                                let a5253 = 9u64;
                                if nat_eq(a5252, a5253) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5254 = v0;
                                    let a5255 = 10u64;
                                    if nat_eq(a5254, a5255) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5256 = v0;
                                        let a5257 = 11u64;
                                        if nat_eq(a5256, a5257) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5258 = v0;
                                            let a5259 = 12u64;
                                            if nat_eq(a5258, a5259) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5260 = v0;
                                                let a5261 = 13u64;
                                                if nat_eq(a5260, a5261) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5262 = v0;
                                                    let a5263 = 14u64;
                                                    if nat_eq(a5262, a5263) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5264 = v0;
                                                        let a5265 = 15u64;
                                                        if nat_eq(a5264, a5265) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5266 = v0;
                                                            let a5267 = 16u64;
                                                            if nat_eq(a5266, a5267) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5268 = v0;
                                                                let a5269 = 17u64;
                                                                if nat_eq(a5268, a5269) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5270 = v0;
                                                                    let a5271 = 18u64;
                                                                    if nat_eq(a5270, a5271) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5272 = v0;
                                                                        let a5273 = 19u64;
                                                                        if nat_eq(a5272, a5273) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5274 = v0;
                                                                            let a5275 = 20u64;
                                                                            if nat_eq(a5274, a5275) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5276 = v0;
                                                                                let a5277 = 21u64;
                                                                                if nat_eq(a5276, a5277) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5278 = v0;
                                                                                    let a5279 = 22u64;
                                                                                    if nat_eq(a5278, a5279) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5280 = v0;
                                                                                        let a5281 = 23u64;
                                                                                        if nat_eq(a5280, a5281) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5282 = v0;
                                                                                            let a5283 = 24u64;
                                                                                            if nat_eq(a5282, a5283) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5284 = v0;
                                                                                                let a5285 = 25u64;
                                                                                                if nat_eq(a5284, a5285) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5286 = v0;
                                                                                                    let a5287 = 26u64;
                                                                                                    if nat_eq(a5286, a5287) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5288 = v0;
                                                                                                        let a5289 = 27u64;
                                                                                                        if nat_eq(a5288, a5289) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5290 = v0;
                                                                                                            let a5291 = 28u64;
                                                                                                            if nat_eq(a5290, a5291) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5292 = v0;
                                                                                                                let a5293 = 29u64;
                                                                                                                if nat_eq(a5292, a5293) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5294 = v0;
                                                                                                                    let a5295 = 30u64;
                                                                                                                    if nat_eq(a5294, a5295) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5296 = v0;
                                                                                                                        let a5297 = 34u64;
                                                                                                                        if nat_eq(a5296, a5297) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5298 = v0;
                                                                                                                            let a5299 = 35u64;
                                                                                                                            if nat_eq(a5298, a5299) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5300 = v0;
                                                                                                                                let a5301 = 39u64;
                                                                                                                                if nat_eq(a5300, a5301) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5302 = v0;
                                                                                                                                    let a5303 = 40u64;
                                                                                                                                    if nat_eq(a5302, a5303) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5304 = v0;
                                                                                                                                        let a5305 = 41u64;
                                                                                                                                        if nat_eq(a5304, a5305) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5306 = v0;
                                                                                                                                            let a5307 = 42u64;
                                                                                                                                            if nat_eq(a5306, a5307) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5308 = v0;
                                                                                                                                                let a5309 = 43u64;
                                                                                                                                                if nat_eq(a5308, a5309) {
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

pub fn f207(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5311 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5310, _, _, _, _, _, _) => {
                    h5310
                }
            };
            let a5312 = 1u64;
            if if if if if nat_le(a5311, a5312) {
                true
            } else {
                let a5314 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5313, _, _) => {
                        h5313
                    }
                };
                let a5315 = v1;
                nat_eq(a5314, a5315)
            } {
                let a5317 = match v0.clone() {
                    Adt2::C0(_, h5316, _, _, _, _, _, _, _, _, _) => {
                        h5316
                    }
                };
                let a5318 = 1u64;
                if nat_le(a5317, a5318) {
                    true
                } else {
                    let a5323 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5319, _, _, _, _, _) => {
                            h5319
                        }
                    };
                    let a5324 = {
                        let a5321 = v1;
                        let a5322 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5320, _, _, _, _, _, _) => {
                                h5320
                            }
                        };
                        nat_mul(a5321, a5322)?
                    };
                    nat_eq(a5323, a5324)
                }
            } else {
                false
            } {
                let a5326 = match v0.clone() {
                    Adt2::C0(_, _, h5325, _, _, _, _, _, _, _, _) => {
                        h5325
                    }
                };
                let a5327 = 1u64;
                if nat_le(a5326, a5327) {
                    true
                } else {
                    let a5335 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5328, _, _, _, _) => {
                            h5328
                        }
                    };
                    let a5336 = {
                        let a5333 = {
                            let a5330 = v1;
                            let a5331 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5329, _, _, _, _, _, _) => {
                                    h5329
                                }
                            };
                            nat_mul(a5330, a5331)?
                        };
                        let a5334 = match v0.clone() {
                            Adt2::C0(_, h5332, _, _, _, _, _, _, _, _, _) => {
                                h5332
                            }
                        };
                        nat_mul(a5333, a5334)?
                    };
                    nat_eq(a5335, a5336)
                }
            } else {
                false
            } {
                let a5338 = match v0.clone() {
                    Adt2::C0(_, _, _, h5337, _, _, _, _, _, _, _) => {
                        h5337
                    }
                };
                let a5339 = 1u64;
                if nat_le(a5338, a5339) {
                    true
                } else {
                    let a5350 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5340, _, _, _) => {
                            h5340
                        }
                    };
                    let a5351 = {
                        let a5348 = {
                            let a5345 = {
                                let a5342 = v1;
                                let a5343 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5341, _, _, _, _, _, _) => {
                                        h5341
                                    }
                                };
                                nat_mul(a5342, a5343)?
                            };
                            let a5346 = match v0.clone() {
                                Adt2::C0(_, h5344, _, _, _, _, _, _, _, _, _) => {
                                    h5344
                                }
                            };
                            nat_mul(a5345, a5346)?
                        };
                        let a5349 = match v0.clone() {
                            Adt2::C0(_, _, h5347, _, _, _, _, _, _, _, _) => {
                                h5347
                            }
                        };
                        nat_mul(a5348, a5349)?
                    };
                    nat_eq(a5350, a5351)
                }
            } else {
                false
            } {
                let a5365 = {
                    let a5362 = {
                        let a5359 = {
                            let a5356 = {
                                let a5353 = v1;
                                let a5354 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5352, _, _, _, _, _, _) => {
                                        h5352
                                    }
                                };
                                nat_mul(a5353, a5354)?
                            };
                            let a5357 = match v0.clone() {
                                Adt2::C0(_, h5355, _, _, _, _, _, _, _, _, _) => {
                                    h5355
                                }
                            };
                            nat_mul(a5356, a5357)?
                        };
                        let a5360 = match v0.clone() {
                            Adt2::C0(_, _, h5358, _, _, _, _, _, _, _, _) => {
                                h5358
                            }
                        };
                        nat_mul(a5359, a5360)?
                    };
                    let a5363 = match v0.clone() {
                        Adt2::C0(_, _, _, h5361, _, _, _, _, _, _, _) => {
                            h5361
                        }
                    };
                    nat_mul(a5362, a5363)?
                };
                let a5366 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5364, _) => {
                        h5364
                    }
                };
                Ok(nat_le(a5365, a5366))
            } else {
                Ok(false)
            }
        } else {
            let a5368 = match v0.clone() {
                Adt2::C0(_, _, _, h5367, _, _, _, _, _, _, _) => {
                    h5367
                }
            };
            let a5369 = 1u64;
            if if if if if nat_le(a5368, a5369) {
                true
            } else {
                let a5371 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5370, _, _, _) => {
                        h5370
                    }
                };
                let a5372 = v1;
                nat_eq(a5371, a5372)
            } {
                let a5374 = match v0.clone() {
                    Adt2::C0(_, h5373, _, _, _, _, _, _, _, _, _) => {
                        h5373
                    }
                };
                let a5375 = 1u64;
                if nat_le(a5374, a5375) {
                    true
                } else {
                    let a5380 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5376, _, _, _, _, _) => {
                            h5376
                        }
                    };
                    let a5381 = {
                        let a5378 = v1;
                        let a5379 = match v0.clone() {
                            Adt2::C0(_, _, _, h5377, _, _, _, _, _, _, _) => {
                                h5377
                            }
                        };
                        nat_mul(a5378, a5379)?
                    };
                    nat_eq(a5380, a5381)
                }
            } else {
                false
            } {
                let a5383 = match v0.clone() {
                    Adt2::C0(_, _, h5382, _, _, _, _, _, _, _, _) => {
                        h5382
                    }
                };
                let a5384 = 1u64;
                if nat_le(a5383, a5384) {
                    true
                } else {
                    let a5392 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5385, _, _, _, _) => {
                            h5385
                        }
                    };
                    let a5393 = {
                        let a5390 = {
                            let a5387 = v1;
                            let a5388 = match v0.clone() {
                                Adt2::C0(_, _, _, h5386, _, _, _, _, _, _, _) => {
                                    h5386
                                }
                            };
                            nat_mul(a5387, a5388)?
                        };
                        let a5391 = match v0.clone() {
                            Adt2::C0(_, h5389, _, _, _, _, _, _, _, _, _) => {
                                h5389
                            }
                        };
                        nat_mul(a5390, a5391)?
                    };
                    nat_eq(a5392, a5393)
                }
            } else {
                false
            } {
                let a5395 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5394, _, _, _, _, _, _) => {
                        h5394
                    }
                };
                let a5396 = 1u64;
                if nat_le(a5395, a5396) {
                    true
                } else {
                    let a5407 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5397, _, _) => {
                            h5397
                        }
                    };
                    let a5408 = {
                        let a5405 = {
                            let a5402 = {
                                let a5399 = v1;
                                let a5400 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5398, _, _, _, _, _, _, _) => {
                                        h5398
                                    }
                                };
                                nat_mul(a5399, a5400)?
                            };
                            let a5403 = match v0.clone() {
                                Adt2::C0(_, h5401, _, _, _, _, _, _, _, _, _) => {
                                    h5401
                                }
                            };
                            nat_mul(a5402, a5403)?
                        };
                        let a5406 = match v0.clone() {
                            Adt2::C0(_, _, h5404, _, _, _, _, _, _, _, _) => {
                                h5404
                            }
                        };
                        nat_mul(a5405, a5406)?
                    };
                    nat_eq(a5407, a5408)
                }
            } else {
                false
            } {
                let a5422 = {
                    let a5419 = {
                        let a5416 = {
                            let a5413 = {
                                let a5410 = v1;
                                let a5411 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5409, _, _, _, _, _, _, _) => {
                                        h5409
                                    }
                                };
                                nat_mul(a5410, a5411)?
                            };
                            let a5414 = match v0.clone() {
                                Adt2::C0(_, h5412, _, _, _, _, _, _, _, _, _) => {
                                    h5412
                                }
                            };
                            nat_mul(a5413, a5414)?
                        };
                        let a5417 = match v0.clone() {
                            Adt2::C0(_, _, h5415, _, _, _, _, _, _, _, _) => {
                                h5415
                            }
                        };
                        nat_mul(a5416, a5417)?
                    };
                    let a5420 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5418, _, _, _, _, _, _) => {
                            h5418
                        }
                    };
                    nat_mul(a5419, a5420)?
                };
                let a5423 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5421, _) => {
                        h5421
                    }
                };
                Ok(nat_le(a5422, a5423))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5425 = match v0.clone() {
            Adt2::C0(_, h5424, _, _, _, _, _, _, _, _, _) => {
                h5424
            }
        };
        let a5426 = 1u64;
        if if if if if nat_le(a5425, a5426) {
            true
        } else {
            let a5428 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5427, _, _, _, _, _) => {
                    h5427
                }
            };
            let a5429 = v1;
            nat_eq(a5428, a5429)
        } {
            let a5431 = match v0.clone() {
                Adt2::C0(_, _, h5430, _, _, _, _, _, _, _, _) => {
                    h5430
                }
            };
            let a5432 = 1u64;
            if nat_le(a5431, a5432) {
                true
            } else {
                let a5437 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5433, _, _, _, _) => {
                        h5433
                    }
                };
                let a5438 = {
                    let a5435 = v1;
                    let a5436 = match v0.clone() {
                        Adt2::C0(_, h5434, _, _, _, _, _, _, _, _, _) => {
                            h5434
                        }
                    };
                    nat_mul(a5435, a5436)?
                };
                nat_eq(a5437, a5438)
            }
        } else {
            false
        } {
            let a5440 = match v0.clone() {
                Adt2::C0(_, _, _, h5439, _, _, _, _, _, _, _) => {
                    h5439
                }
            };
            let a5441 = 1u64;
            if nat_le(a5440, a5441) {
                true
            } else {
                let a5449 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5442, _, _, _) => {
                        h5442
                    }
                };
                let a5450 = {
                    let a5447 = {
                        let a5444 = v1;
                        let a5445 = match v0.clone() {
                            Adt2::C0(_, h5443, _, _, _, _, _, _, _, _, _) => {
                                h5443
                            }
                        };
                        nat_mul(a5444, a5445)?
                    };
                    let a5448 = match v0.clone() {
                        Adt2::C0(_, _, h5446, _, _, _, _, _, _, _, _) => {
                            h5446
                        }
                    };
                    nat_mul(a5447, a5448)?
                };
                nat_eq(a5449, a5450)
            }
        } else {
            false
        } {
            let a5452 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5451, _, _, _, _, _, _) => {
                    h5451
                }
            };
            let a5453 = 1u64;
            if nat_le(a5452, a5453) {
                true
            } else {
                let a5464 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5454, _, _) => {
                        h5454
                    }
                };
                let a5465 = {
                    let a5462 = {
                        let a5459 = {
                            let a5456 = v1;
                            let a5457 = match v0.clone() {
                                Adt2::C0(_, h5455, _, _, _, _, _, _, _, _, _) => {
                                    h5455
                                }
                            };
                            nat_mul(a5456, a5457)?
                        };
                        let a5460 = match v0.clone() {
                            Adt2::C0(_, _, h5458, _, _, _, _, _, _, _, _) => {
                                h5458
                            }
                        };
                        nat_mul(a5459, a5460)?
                    };
                    let a5463 = match v0.clone() {
                        Adt2::C0(_, _, _, h5461, _, _, _, _, _, _, _) => {
                            h5461
                        }
                    };
                    nat_mul(a5462, a5463)?
                };
                nat_eq(a5464, a5465)
            }
        } else {
            false
        } {
            let a5479 = {
                let a5476 = {
                    let a5473 = {
                        let a5470 = {
                            let a5467 = v1;
                            let a5468 = match v0.clone() {
                                Adt2::C0(_, h5466, _, _, _, _, _, _, _, _, _) => {
                                    h5466
                                }
                            };
                            nat_mul(a5467, a5468)?
                        };
                        let a5471 = match v0.clone() {
                            Adt2::C0(_, _, h5469, _, _, _, _, _, _, _, _) => {
                                h5469
                            }
                        };
                        nat_mul(a5470, a5471)?
                    };
                    let a5474 = match v0.clone() {
                        Adt2::C0(_, _, _, h5472, _, _, _, _, _, _, _) => {
                            h5472
                        }
                    };
                    nat_mul(a5473, a5474)?
                };
                let a5477 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5475, _, _, _, _, _, _) => {
                        h5475
                    }
                };
                nat_mul(a5476, a5477)?
            };
            let a5480 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5478, _) => {
                    h5478
                }
            };
            Ok(nat_le(a5479, a5480))
        } else {
            Ok(false)
        }
    }
}

pub fn f208(v0: Adt2, v1: u64) -> R<bool> {
    let a5482 = match v0.clone() {
        Adt2::C0(h5481, _, _, _, _, _, _, _, _, _, _) => {
            h5481
        }
    };
    let a5483 = 0u64;
    if if nat_eq(a5482, a5483) {
        f37(v0.clone())?
    } else {
        false
    } {
        let a5484 = v1;
        let a5485 = 1u64;
        if nat_le(a5484, a5485) {
            Ok(true)
        } else {
            let a5490 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5486, _, _, _, _) => {
                    h5486
                }
            };
            let a5491 = {
                let a5488 = 4u64;
                let a5489 = match v0 {
                    Adt2::C0(_, h5487, _, _, _, _, _, _, _, _, _) => {
                        h5487
                    }
                };
                nat_mul(a5488, a5489)?
            };
            Ok(nat_eq(a5490, a5491))
        }
    } else {
        Ok(false)
    }
}

pub fn f209(v0: Adt2, v1: u64) -> R<bool> {
    let a5493 = match v0.clone() {
        Adt2::C0(_, _, _, h5492, _, _, _, _, _, _, _) => {
            h5492
        }
    };
    let a5494 = 1u64;
    if if if nat_eq(a5493, a5494) {
        let a5496 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5495, _, _, _, _, _, _) => {
                h5495
            }
        };
        let a5497 = 1u64;
        nat_eq(a5496, a5497)
    } else {
        false
    } {
        let a5499 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5498, _, _, _, _, _) => {
                h5498
            }
        };
        let a5500 = v1;
        nat_eq(a5499, a5500)
    } else {
        false
    } {
        let a5502 = match v0.clone() {
            Adt2::C0(_, _, h5501, _, _, _, _, _, _, _, _) => {
                h5501
            }
        };
        let a5503 = 1u64;
        if nat_eq(a5502, a5503) {
            Ok(true)
        } else {
            let a5508 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5504, _, _, _, _) => {
                    h5504
                }
            };
            let a5509 = {
                let a5506 = match v0 {
                    Adt2::C0(_, h5505, _, _, _, _, _, _, _, _, _) => {
                        h5505
                    }
                };
                let a5507 = v1;
                nat_mul(a5506, a5507)?
            };
            Ok(nat_eq(a5508, a5509))
        }
    } else {
        Ok(false)
    }
}

pub fn f210(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5513 = {
            let a5510 = v2;
            let a5511 = 1u64;
            nat_sub(a5510, a5511)
        };
        let a5514 = match v0.clone() {
            Adt6::C0(_, h5512, _, _) => {
                h5512
            }
        };
        nat_mul(a5513, a5514)?
    };
    let a5516 = 0u64;
    let a5517 = match v0.clone() {
        Adt6::C0(h5515, _, _, _) => {
            h5515
        }
    };
    if if if if if if if if nat_lt(a5516, a5517) {
        let a5519 = match v0.clone() {
            Adt6::C0(h5518, _, _, _) => {
                h5518
            }
        };
        let a5520 = 2147483648u64;
        nat_lt(a5519, a5520)
    } else {
        false
    } {
        let a5522 = 0u64;
        let a5523 = match v0.clone() {
            Adt6::C0(_, h5521, _, _) => {
                h5521
            }
        };
        nat_lt(a5522, a5523)
    } else {
        false
    } {
        let a5525 = match v0.clone() {
            Adt6::C0(_, h5524, _, _) => {
                h5524
            }
        };
        let a5526 = 2147483648u64;
        nat_lt(a5525, a5526)
    } else {
        false
    } {
        let a5527 = v1;
        let a5528 = 2147483648u64;
        nat_lt(a5527, a5528)
    } else {
        false
    } {
        let a5529 = 0u64;
        let a5530 = v2;
        nat_lt(a5529, a5530)
    } else {
        false
    } {
        let a5531 = v3;
        let a5532 = 2147483648u64;
        nat_lt(a5531, a5532)
    } else {
        false
    } {
        let a5544 = {
            let a5539 = {
                let a5536 = v3;
                let a5537 = {
                    let a5534 = match v0.clone() {
                        Adt6::C0(h5533, _, _, _) => {
                            h5533
                        }
                    };
                    let a5535 = 1u64;
                    nat_sub(a5534, a5535)
                };
                nat_add(a5536, a5537)?
            };
            let a5540 = f136(match v0.clone() {
                Adt6::C0(_, _, h5538, _) => {
                    h5538
                }
            });
            nat_add(a5539, a5540)?
        };
        let a5545 = {
            let a5542 = 2147483647u64;
            let a5543 = f137(match v0.clone() {
                Adt6::C0(_, _, h5541, _) => {
                    h5541
                }
            });
            nat_add(a5542, a5543)?
        };
        nat_le(a5544, a5545)
    } else {
        false
    } {
        let a5547 = match v0 {
            Adt6::C0(_, _, h5546, _) => {
                h5546
            }
        };
        let a5548 = 2147483648u64;
        Ok(!nat_eq(a5547, a5548))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f260(match v0.clone() {
        Adt2::C0(h5549, _, _, _, _, _, _, _, _, _, _) => {
            h5549
        }
    }, match v0.clone() {
        Adt2::C0(_, h5550, _, _, _, _, _, _, _, _, _) => {
            h5550
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5551, _, _, _, _, _, _, _, _) => {
            h5551
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5552, _, _, _, _, _, _, _) => {
            h5552
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5553, _, _, _, _, _, _) => {
            h5553
        }
    }, match v1.clone() {
        Adt2::C0(h5554, _, _, _, _, _, _, _, _, _, _) => {
            h5554
        }
    }, match v1.clone() {
        Adt2::C0(_, h5555, _, _, _, _, _, _, _, _, _) => {
            h5555
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5556, _, _, _, _, _, _, _, _) => {
            h5556
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5557, _, _, _, _, _, _, _) => {
            h5557
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5558, _, _, _, _, _, _) => {
            h5558
        }
    }, match v2.clone() {
        Adt2::C0(h5559, _, _, _, _, _, _, _, _, _, _) => {
            h5559
        }
    }, match v2.clone() {
        Adt2::C0(_, h5560, _, _, _, _, _, _, _, _, _) => {
            h5560
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5561, _, _, _, _, _, _, _, _) => {
            h5561
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5562, _, _, _, _, _, _, _) => {
            h5562
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5563, _, _, _, _, _, _) => {
            h5563
        }
    }) {
        let m5565 = f206(match v0.clone() {
            Adt2::C0(h5564, _, _, _, _, _, _, _, _, _, _) => {
                h5564
            }
        });
        match m5565 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5571 = {
                    let a5568 = match v0.clone() {
                        Adt2::C0(_, h5566, _, _, _, _, _, _, _, _, _) => {
                            h5566
                        }
                    };
                    let a5569 = match v3.clone() {
                        Adt19::C0(_, h5567, _) => {
                            h5567
                        }
                    };
                    let a5570 = 0u64;
                    nat_rem(a5568, a5569, a5570)
                };
                let a5572 = 0u64;
                if if nat_eq(a5571, a5572) {
                    if f261(match v0.clone() {
                        Adt2::C0(_, h5573, _, _, _, _, _, _, _, _, _) => {
                            h5573
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5574, _, _, _, _, _, _, _, _, _) => {
                            h5574
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5575, _, _, _, _, _, _, _, _) => {
                            h5575
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5576, _, _, _, _, _, _, _) => {
                            h5576
                        }
                    }) {
                        true
                    } else {
                        let a5578 = 4u64;
                        let a5579 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5577, _, _, _, _, _) => {
                                h5577
                            }
                        };
                        if if nat_le(a5578, a5579) {
                            let a5581 = match v3.clone() {
                                Adt19::C0(_, h5580, _) => {
                                    h5580
                                }
                            };
                            let a5582 = 1u64;
                            if nat_eq(a5581, a5582) {
                                true
                            } else {
                                let a5585 = match v3.clone() {
                                    Adt19::C0(_, _, h5583) => {
                                        h5583
                                    }
                                };
                                let a5586 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5584, _, _, _, _, _) => {
                                        h5584
                                    }
                                };
                                nat_le(a5585, a5586)
                            }
                        } else {
                            false
                        } {
                            if f29(v0.clone(), v2.clone())? {
                                f29(v1.clone(), v2.clone())?
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
                        Adt19::C0(_, h5587, _) => {
                            h5587
                        }
                    });
                    Ok(Adt10::C4(if f261(match v0.clone() {
                        Adt2::C0(_, h5588, _, _, _, _, _, _, _, _, _) => {
                            h5588
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5589, _, _, _, _, _, _, _, _, _) => {
                            h5589
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5590, _, _, _, _, _, _, _, _) => {
                            h5590
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5591, _, _, _, _, _, _, _) => {
                            h5591
                        }
                    }) {
                        0u64
                    } else {
                        f170(v4.clone(), f24(v1.clone(), 1u64))?
                    }, 4u64, v4, f24(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5592, _, _, _, _, _, _, _, _, _, _) => {
                            h5592
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

pub fn f212(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f213(match v0.clone() {
        Adt2::C0(h5593, _, _, _, _, _, _, _, _, _, _) => {
            h5593
        }
    }) {
        let a5599 = {
            let a5596 = match v0.clone() {
                Adt2::C0(_, h5594, _, _, _, _, _, _, _, _, _) => {
                    h5594
                }
            };
            let a5597 = f243(match v0.clone() {
                Adt2::C0(h5595, _, _, _, _, _, _, _, _, _, _) => {
                    h5595
                }
            });
            let a5598 = 1u64;
            nat_rem(a5596, a5597, a5598)
        };
        let a5600 = 0u64;
        nat_eq(a5599, a5600)
    } else {
        false
    } {
        let a5602 = match v1.clone() {
            Adt2::C0(h5601, _, _, _, _, _, _, _, _, _, _) => {
                h5601
            }
        };
        let a5603 = 26u64;
        nat_eq(a5602, a5603)
    } else {
        false
    } {
        let a5605 = match v2.clone() {
            Adt2::C0(h5604, _, _, _, _, _, _, _, _, _, _) => {
                h5604
            }
        };
        let a5606 = 0u64;
        nat_eq(a5605, a5606)
    } else {
        false
    } {
        let a5609 = match v0.clone() {
            Adt2::C0(_, h5607, _, _, _, _, _, _, _, _, _) => {
                h5607
            }
        };
        let a5610 = match v2.clone() {
            Adt2::C0(_, h5608, _, _, _, _, _, _, _, _, _) => {
                h5608
            }
        };
        nat_eq(a5609, a5610)
    } else {
        false
    } {
        let a5613 = match v1.clone() {
            Adt2::C0(_, h5611, _, _, _, _, _, _, _, _, _) => {
                h5611
            }
        };
        let a5614 = match v0.clone() {
            Adt2::C0(_, _, h5612, _, _, _, _, _, _, _, _) => {
                h5612
            }
        };
        nat_le(a5613, a5614)
    } else {
        false
    } {
        let a5616 = match v0.clone() {
            Adt2::C0(_, _, _, h5615, _, _, _, _, _, _, _) => {
                h5615
            }
        };
        let a5617 = 1u64;
        nat_eq(a5616, a5617)
    } else {
        false
    } {
        let a5619 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5618, _, _, _, _, _, _) => {
                h5618
            }
        };
        let a5620 = 1u64;
        nat_eq(a5619, a5620)
    } else {
        false
    } {
        let a5622 = match v2.clone() {
            Adt2::C0(_, _, _, h5621, _, _, _, _, _, _, _) => {
                h5621
            }
        };
        let a5623 = 1u64;
        nat_eq(a5622, a5623)
    } else {
        false
    } {
        let a5625 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5624, _, _, _, _, _, _) => {
                h5624
            }
        };
        let a5626 = 1u64;
        nat_eq(a5625, a5626)
    } else {
        false
    } {
        let a5628 = match v1.clone() {
            Adt2::C0(_, _, h5627, _, _, _, _, _, _, _, _) => {
                h5627
            }
        };
        let a5629 = 1u64;
        nat_eq(a5628, a5629)
    } else {
        false
    } {
        let a5631 = match v1.clone() {
            Adt2::C0(_, _, _, h5630, _, _, _, _, _, _, _) => {
                h5630
            }
        };
        let a5632 = 1u64;
        nat_eq(a5631, a5632)
    } else {
        false
    } {
        let a5634 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5633, _, _, _, _, _, _) => {
                h5633
            }
        };
        let a5635 = 1u64;
        nat_eq(a5634, a5635)
    } else {
        false
    } {
        if f47(match v2.clone() {
            Adt2::C0(_, h5636, _, _, _, _, _, _, _, _, _) => {
                h5636
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5637, _, _, _, _, _, _, _, _) => {
                h5637
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5638, _, _, _, _, _, _, _) => {
                h5638
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5639, _, _, _, _, _, _) => {
                h5639
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5641 = match v1.clone() {
                Adt2::C0(_, h5640, _, _, _, _, _, _, _, _, _) => {
                    h5640
                }
            };
            let a5642 = 0u64;
            if nat_eq(a5641, a5642) {
                Ok(Adt10::C4(f22(v2.clone(), 1u64)?, 4u64, f24(v0, 1u64), f24(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5644 = match v0.clone() {
                    Adt2::C0(_, h5643, _, _, _, _, _, _, _, _, _) => {
                        h5643
                    }
                };
                let a5645 = 2147483647u64;
                if if if if if if nat_le(a5644, a5645) {
                    let a5647 = match v1.clone() {
                        Adt2::C0(_, h5646, _, _, _, _, _, _, _, _, _) => {
                            h5646
                        }
                    };
                    let a5648 = 2147483647u64;
                    nat_le(a5647, a5648)
                } else {
                    false
                } {
                    let a5650 = 4u64;
                    let a5651 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5649, _, _, _, _, _) => {
                            h5649
                        }
                    };
                    nat_le(a5650, a5651)
                } else {
                    false
                } {
                    let a5657 = {
                        let a5654 = match v2.clone() {
                            Adt2::C0(_, h5652, _, _, _, _, _, _, _, _, _) => {
                                h5652
                            }
                        };
                        let a5655 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5653, _, _, _, _, _) => {
                                h5653
                            }
                        };
                        nat_mul(a5654, a5655)?
                    };
                    let a5658 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5656, _, _, _, _) => {
                            h5656
                        }
                    };
                    nat_le(a5657, a5658)
                } else {
                    false
                } {
                    f29(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f29(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5671 = {
                            let a5662 = {
                                let a5660 = 2u64;
                                let a5661 = match v1.clone() {
                                    Adt2::C0(_, h5659, _, _, _, _, _, _, _, _, _) => {
                                        h5659
                                    }
                                };
                                nat_mul(a5660, a5661)?
                            };
                            let a5663 = f22(v2.clone(), 1u64)?;
                            nat_add(a5662, a5663)?
                        };
                        let a5672 = {
                            let a5669 = match v1.clone() {
                                Adt2::C0(_, h5664, _, _, _, _, _, _, _, _, _) => {
                                    h5664
                                }
                            };
                            let a5670 = {
                                let a5667 = 1u64;
                                let a5668 = f167(match v0.clone() {
                                    Adt2::C0(h5665, _, _, _, _, _, _, _, _, _, _) => {
                                        h5665
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5666, _, _, _, _, _, _, _, _, _) => {
                                        h5666
                                    }
                                })?;
                                nat_add(a5667, a5668)?
                            };
                            nat_mul(a5669, a5670)?
                        };
                        nat_add(a5671, a5672)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5673, _, _, _, _, _, _, _, _, _) => {
                            h5673
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5674, _, _, _, _, _, _, _, _, _) => {
                            h5674
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5675, _, _, _, _, _) => {
                            h5675
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5676, _, _, _, _) => {
                            h5676
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5677, _, _, _) => {
                            h5677
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5678, _, _) => {
                            h5678
                        }
                    }), f24(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5679, _, _, _, _, _, _, _, _, _, _) => {
                            h5679
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

pub fn f213(v0: u64) -> bool {
    let a5680 = v0;
    let a5681 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5680, a5681) {
        true
    } else {
        let a5682 = v0;
        let a5683 = 1u64;
        nat_eq(a5682, a5683)
    } {
        true
    } else {
        let a5684 = v0;
        let a5685 = 2u64;
        nat_eq(a5684, a5685)
    } {
        true
    } else {
        let a5686 = v0;
        let a5687 = 3u64;
        nat_eq(a5686, a5687)
    } {
        true
    } else {
        let a5688 = v0;
        let a5689 = 6u64;
        nat_eq(a5688, a5689)
    } {
        true
    } else {
        let a5690 = v0;
        let a5691 = 7u64;
        nat_eq(a5690, a5691)
    } {
        true
    } else {
        let a5692 = v0;
        let a5693 = 8u64;
        nat_eq(a5692, a5693)
    } {
        true
    } else {
        let a5694 = v0;
        let a5695 = 10u64;
        nat_eq(a5694, a5695)
    } {
        true
    } else {
        let a5696 = v0;
        let a5697 = 11u64;
        nat_eq(a5696, a5697)
    } {
        true
    } else {
        let a5698 = v0;
        let a5699 = 12u64;
        nat_eq(a5698, a5699)
    } {
        true
    } else {
        let a5700 = v0;
        let a5701 = 13u64;
        nat_eq(a5700, a5701)
    } {
        true
    } else {
        let a5702 = v0;
        let a5703 = 14u64;
        nat_eq(a5702, a5703)
    } {
        true
    } else {
        let a5704 = v0;
        let a5705 = 16u64;
        nat_eq(a5704, a5705)
    } {
        true
    } else {
        let a5706 = v0;
        let a5707 = 17u64;
        nat_eq(a5706, a5707)
    } {
        true
    } else {
        let a5708 = v0;
        let a5709 = 18u64;
        nat_eq(a5708, a5709)
    } {
        true
    } else {
        let a5710 = v0;
        let a5711 = 19u64;
        nat_eq(a5710, a5711)
    } {
        true
    } else {
        let a5712 = v0;
        let a5713 = 20u64;
        nat_eq(a5712, a5713)
    } {
        true
    } else {
        let a5714 = v0;
        let a5715 = 21u64;
        nat_eq(a5714, a5715)
    } {
        true
    } else {
        let a5716 = v0;
        let a5717 = 22u64;
        nat_eq(a5716, a5717)
    } {
        true
    } else {
        let a5718 = v0;
        let a5719 = 23u64;
        nat_eq(a5718, a5719)
    } {
        true
    } else {
        let a5720 = v0;
        let a5721 = 26u64;
        nat_eq(a5720, a5721)
    } {
        true
    } else {
        let a5722 = v0;
        let a5723 = 29u64;
        nat_eq(a5722, a5723)
    } {
        true
    } else {
        let a5724 = v0;
        let a5725 = 30u64;
        nat_eq(a5724, a5725)
    } {
        true
    } else {
        let a5726 = v0;
        let a5727 = 34u64;
        nat_eq(a5726, a5727)
    } {
        true
    } else {
        let a5728 = v0;
        let a5729 = 35u64;
        nat_eq(a5728, a5729)
    } {
        true
    } else {
        let a5730 = v0;
        let a5731 = 39u64;
        nat_eq(a5730, a5731)
    } {
        true
    } else {
        let a5732 = v0;
        let a5733 = 40u64;
        nat_eq(a5732, a5733)
    } {
        true
    } else {
        let a5734 = v0;
        let a5735 = 41u64;
        nat_eq(a5734, a5735)
    } {
        true
    } else {
        let a5736 = v0;
        let a5737 = 42u64;
        nat_eq(a5736, a5737)
    } {
        true
    } else {
        let a5738 = v0;
        let a5739 = 43u64;
        nat_eq(a5738, a5739)
    }
}

pub fn f214(v0: u64) -> bool {
    if f168(v0) {
        true
    } else {
        let a5740 = v0;
        let a5741 = 43u64;
        nat_eq(a5740, a5741)
    }
}

pub fn f215(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5744 = match v1.clone() {
        Adt2::C0(h5742, _, _, _, _, _, _, _, _, _, _) => {
            h5742
        }
    };
    let a5745 = match v3.clone() {
        Adt2::C0(h5743, _, _, _, _, _, _, _, _, _, _) => {
            h5743
        }
    };
    if if nat_eq(a5744, a5745) {
        let a5746 = v0;
        let a5747 = 33u64;
        if nat_eq(a5746, a5747) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5749 = f206(match v1.clone() {
            Adt2::C0(h5748, _, _, _, _, _, _, _, _, _, _) => {
                h5748
            }
        });
        match m5749 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5750, _) => {
                        h5750
                    }
                };
                let v6: u64 = f22(v1.clone(), v5)?;
                let a5755 = {
                    let a5752 = match v1.clone() {
                        Adt2::C0(_, h5751, _, _, _, _, _, _, _, _, _) => {
                            h5751
                        }
                    };
                    let a5753 = v5;
                    let a5754 = 1u64;
                    nat_rem(a5752, a5753, a5754)
                };
                let a5756 = 0u64;
                if if if nat_eq(a5755, a5756) {
                    let a5761 = {
                        let a5758 = match v3.clone() {
                            Adt2::C0(_, h5757, _, _, _, _, _, _, _, _, _) => {
                                h5757
                            }
                        };
                        let a5759 = v5;
                        let a5760 = 1u64;
                        nat_rem(a5758, a5759, a5760)
                    };
                    let a5762 = 0u64;
                    nat_eq(a5761, a5762)
                } else {
                    false
                } {
                    let a5763 = v6;
                    let a5764 = f22(v3.clone(), v5)?;
                    nat_eq(a5763, a5764)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h5765) => {
                            h5765
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

pub fn f216(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f47(v4, v5, v6, v7) {
        f47(v0, v1, v2, v3)
    } else {
        let a5769 = {
            let a5766 = v0;
            let a5767 = v4;
            let a5768 = 1u64;
            nat_rem(a5766, a5767, a5768)
        };
        let a5770 = 0u64;
        if if if nat_eq(a5769, a5770) {
            let a5774 = {
                let a5771 = v1;
                let a5772 = v5;
                let a5773 = 1u64;
                nat_rem(a5771, a5772, a5773)
            };
            let a5775 = 0u64;
            nat_eq(a5774, a5775)
        } else {
            false
        } {
            let a5779 = {
                let a5776 = v2;
                let a5777 = v6;
                let a5778 = 1u64;
                nat_rem(a5776, a5777, a5778)
            };
            let a5780 = 0u64;
            nat_eq(a5779, a5780)
        } else {
            false
        } {
            let a5784 = {
                let a5781 = v3;
                let a5782 = v7;
                let a5783 = 1u64;
                nat_rem(a5781, a5782, a5783)
            };
            let a5785 = 0u64;
            nat_eq(a5784, a5785)
        } else {
            false
        }
    }
}

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5786 = v0;
    let a5787 = v1;
    if nat_eq(a5786, a5787) {
        let a5790 = v4;
        let a5791 = {
            let a5788 = v2;
            let a5789 = v3;
            nat_add(a5788, a5789)?
        };
        Ok(nat_eq(a5790, a5791))
    } else {
        let a5792 = v2;
        let a5793 = v3;
        if nat_eq(a5792, a5793) {
            let a5794 = v4;
            let a5795 = v2;
            Ok(nat_eq(a5794, a5795))
        } else {
            Ok(false)
        }
    }
}

pub fn f218(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5798 = {
            let a5796 = v0;
            let a5797 = f137(v1);
            nat_add(a5796, a5797)?
        };
        let a5799 = f137(v2);
        nat_add(a5798, a5799)?
    };
    let v5: u64 = {
        let a5800 = f136(v1);
        let a5801 = f136(v2);
        nat_add(a5800, a5801)?
    };
    let a5802 = v1;
    let a5803 = 4294967295u64;
    if if nat_le(a5802, a5803) {
        let a5804 = v2;
        let a5805 = 4294967295u64;
        nat_le(a5804, a5805)
    } else {
        false
    } {
        let a5806 = v5;
        let a5807 = v4;
        if nat_le(a5806, a5807) {
            let a5810 = v3;
            let a5811 = {
                let a5808 = v4;
                let a5809 = v5;
                nat_sub(a5808, a5809)
            };
            Ok(nat_eq(a5810, a5811))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f219(v0: u64, v1: u64) -> R<bool> {
    let a5812 = v1;
    let a5813 = 4294967295u64;
    if nat_le(a5812, a5813) {
        let a5816 = {
            let a5814 = f137(v1);
            let a5815 = f136(v1);
            nat_add(a5814, a5815)?
        };
        let a5817 = v0;
        Ok(nat_lt(a5816, a5817))
    } else {
        Ok(false)
    }
}

pub fn f220(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f262(v0.clone(), false)?;
    let v6: u64 = f262(v0, true)?;
    let a5818 = v6;
    let a5819 = v1;
    if if nat_le(a5818, a5819) {
        let a5826 = {
            let a5824 = {
                let a5820 = v1;
                let a5821 = v5;
                nat_add(a5820, a5821)?
            };
            let a5825 = {
                let a5822 = v3;
                let a5823 = 0u64;
                if nat_eq(a5822, a5823) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5824, a5825)?
        };
        let a5827 = v2;
        nat_le(a5826, a5827)
    } else {
        false
    } {
        if v4 {
            let a5832 = {
                let a5830 = {
                    let a5828 = v1;
                    let a5829 = v5;
                    nat_add(a5828, a5829)?
                };
                let a5831 = v6;
                nat_sub(a5830, a5831)
            };
            let a5833 = v2;
            Ok(nat_lt(a5832, a5833))
        } else {
            let a5838 = {
                let a5836 = {
                    let a5834 = v1;
                    let a5835 = v5;
                    nat_add(a5834, a5835)?
                };
                let a5837 = v6;
                nat_sub(a5836, a5837)
            };
            let a5839 = v2;
            Ok(nat_le(a5838, a5839))
        }
    } else {
        Ok(false)
    }
}

pub fn f221(v0: u64, v1: u64) -> bool {
    let a5840 = v0;
    let a5841 = 0u64;
    if if nat_eq(a5840, a5841) {
        true
    } else {
        let a5842 = v0;
        let a5843 = 1u64;
        nat_eq(a5842, a5843)
    } {
        let a5844 = v1;
        let a5845 = 4294967296u64;
        nat_lt(a5844, a5845)
    } else {
        false
    }
}

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let a5850 = {
        let a5848 = v0;
        let a5849 = {
            let a5846 = v1;
            let a5847 = 1u64;
            nat_sub(a5846, a5847)
        };
        nat_add(a5848, a5849)?
    };
    let a5851 = v1;
    let a5852 = 0u64;
    Ok(nat_quot(a5850, a5851, a5852))
}

pub fn f223(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f47(match v0.clone() {
        Adt2::C0(_, h5853, _, _, _, _, _, _, _, _, _) => {
            h5853
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5854, _, _, _, _, _, _, _, _) => {
            h5854
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5855, _, _, _, _, _, _, _) => {
            h5855
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5856, _, _, _, _, _, _) => {
            h5856
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a5859 = match v1.clone() {
                Adt2::C0(_, h5857, _, _, _, _, _, _, _, _, _) => {
                    h5857
                }
            };
            let a5860 = match v1.clone() {
                Adt2::C0(_, _, h5858, _, _, _, _, _, _, _, _) => {
                    h5858
                }
            };
            nat_mul(a5859, a5860)?
        };
        let v5: u64 = {
            let a5862 = v4;
            let a5863 = match v1.clone() {
                Adt2::C0(_, _, _, h5861, _, _, _, _, _, _, _) => {
                    h5861
                }
            };
            nat_mul(a5862, a5863)?
        };
        let a5876 = {
            let a5874 = {
                let a5865 = match v0.clone() {
                    Adt2::C0(_, h5864, _, _, _, _, _, _, _, _, _) => {
                        h5864
                    }
                };
                let a5866 = 1u64;
                nat_sub(a5865, a5866)
            };
            let a5875 = {
                let a5872 = f263(match v0.clone() {
                    Adt2::C0(_, _, h5867, _, _, _, _, _, _, _, _) => {
                        h5867
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h5868, _, _, _, _, _, _, _, _, _) => {
                        h5868
                    }
                })?;
                let a5873 = f263(match v0 {
                    Adt2::C0(_, _, _, h5869, _, _, _, _, _, _, _) => {
                        h5869
                    }
                }, v2, {
                    let a5870 = v3;
                    let a5871 = v5;
                    nat_mul(a5870, a5871)?
                }, v4)?;
                nat_add(a5872, a5873)?
            };
            nat_add(a5874, a5875)?
        };
        let a5877 = f22(v1, 1u64)?;
        Ok(nat_lt(a5876, a5877))
    }
}

pub fn f224(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5878 = v0;
    let a5879 = 2147483648u64;
    if if if if nat_lt(a5878, a5879) {
        let a5880 = v1;
        let a5881 = 2147483648u64;
        nat_lt(a5880, a5881)
    } else {
        false
    } {
        let a5884 = {
            let a5882 = v0;
            let a5883 = v1;
            nat_mul(a5882, a5883)?
        };
        let a5885 = 2147483648u64;
        nat_lt(a5884, a5885)
    } else {
        false
    } {
        let a5890 = {
            let a5888 = {
                let a5886 = v0;
                let a5887 = 1u64;
                nat_sub(a5886, a5887)
            };
            let a5889 = v2;
            nat_mul(a5888, a5889)?
        };
        let a5891 = 2147483648u64;
        nat_lt(a5890, a5891)
    } else {
        false
    } {
        let a5896 = {
            let a5894 = {
                let a5892 = v1;
                let a5893 = 1u64;
                nat_sub(a5892, a5893)
            };
            let a5895 = v2;
            nat_mul(a5894, a5895)?
        };
        let a5897 = 2147483648u64;
        Ok(nat_lt(a5896, a5897))
    } else {
        Ok(false)
    }
}

pub fn f225(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a5898 = v1;
    let a5899 = 13u64;
    if if nat_eq(a5898, a5899) {
        f47(match v0.clone() {
            Adt3::C0(h5900, _, _, _, _, _, _, _) => {
                h5900
            }
        }, match v0.clone() {
            Adt3::C0(_, h5901, _, _, _, _, _, _) => {
                h5901
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h5902, _, _, _, _, _) => {
                h5902
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h5903, _, _, _, _) => {
                h5903
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a5909 = match v0.clone() {
                Adt3::C0(_, h5904, _, _, _, _, _, _) => {
                    h5904
                }
            };
            let a5910 = {
                let a5907 = match v0.clone() {
                    Adt3::C0(_, _, h5905, _, _, _, _, _) => {
                        h5905
                    }
                };
                let a5908 = match v0.clone() {
                    Adt3::C0(_, _, _, h5906, _, _, _, _) => {
                        h5906
                    }
                };
                nat_mul(a5907, a5908)?
            };
            nat_mul(a5909, a5910)?
        };
        let a5911 = v1;
        let a5912 = 13u64;
        if nat_eq(a5911, a5912) {
            let a5921 = 2u64;
            let a5922 = if f264(v0.clone(), v2) {
                let a5914 = v3;
                let a5915 = f237(match v0 {
                    Adt3::C0(h5913, _, _, _, _, _, _, _) => {
                        h5913
                    }
                }, false)?;
                nat_mul(a5914, a5915)?
            } else {
                let a5919 = v3;
                let a5920 = {
                    let a5917 = match v0 {
                        Adt3::C0(h5916, _, _, _, _, _, _, _) => {
                            h5916
                        }
                    };
                    let a5918 = 2u64;
                    nat_mul(a5917, a5918)?
                };
                nat_mul(a5919, a5920)?
            };
            nat_add(a5921, a5922)
        } else {
            let a5926 = v3;
            let a5927 = f237(match v0 {
                Adt3::C0(h5923, _, _, _, _, _, _, _) => {
                    h5923
                }
            }, {
                let a5924 = v1;
                let a5925 = 16u64;
                nat_eq(a5924, a5925)
            })?;
            nat_mul(a5926, a5927)
        }
    }
}

pub fn f226(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a5928 = v0;
    let a5929 = 30u64;
    if if nat_eq(a5928, a5929) {
        let a5932 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h5930, _, _, _, _) => {
                h5930
            }
        };
        let a5933 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h5931, _, _, _, _, _) => {
                h5931
            }
        };
        nat_lt(a5932, a5933)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m5934 = v3;
        match m5934 {
            None => {
                f265(v0, v1, match v2.clone() {
                    Adt2::C0(h5935, _, _, _, _, _, _, _, _, _, _) => {
                        h5935
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h5936, _, _, _, _, _, _, _, _, _, _) => {
                        h5936
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h5937, _, _, _, _, _, _, _, _, _) => {
                        h5937
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5938, _, _, _, _, _, _, _, _) => {
                        h5938
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5939, _, _, _, _, _, _, _) => {
                        h5939
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5940, _, _, _, _, _, _) => {
                        h5940
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h5941, _, _, _, _, _, _, _, _, _) => {
                        h5941
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5942, _, _, _, _, _, _, _, _) => {
                        h5942
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5943, _, _, _, _, _, _, _) => {
                        h5943
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5944, _, _, _, _, _, _) => {
                        h5944
                    }
                })
            }
            Some(v5) => {
                f265(v0, v1, match v2.clone() {
                    Adt2::C0(h5945, _, _, _, _, _, _, _, _, _, _) => {
                        h5945
                    }
                }, match v5.clone() {
                    Adt2::C0(h5946, _, _, _, _, _, _, _, _, _, _) => {
                        h5946
                    }
                }, match v4.clone() {
                    Adt2::C0(h5947, _, _, _, _, _, _, _, _, _, _) => {
                        h5947
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h5948, _, _, _, _, _, _, _, _, _) => {
                        h5948
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5949, _, _, _, _, _, _, _, _) => {
                        h5949
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5950, _, _, _, _, _, _, _) => {
                        h5950
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5951, _, _, _, _, _, _) => {
                        h5951
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h5952, _, _, _, _, _, _, _, _, _) => {
                        h5952
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h5953, _, _, _, _, _, _, _, _) => {
                        h5953
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h5954, _, _, _, _, _, _, _) => {
                        h5954
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h5955, _, _, _, _, _, _) => {
                        h5955
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h5956, _, _, _, _, _, _, _, _, _) => {
                        h5956
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5957, _, _, _, _, _, _, _, _) => {
                        h5957
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5958, _, _, _, _, _, _, _) => {
                        h5958
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5959, _, _, _, _, _, _) => {
                        h5959
                    }
                })
            }
        }
    }
}

pub fn f227(v0: u64, v1: u64) -> u64 {
    let a5960 = v0;
    let a5961 = 23u64;
    if nat_eq(a5960, a5961) {
        let a5962 = v1;
        let a5963 = 0u64;
        if nat_eq(a5962, a5963) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5968 = {
        let a5966 = {
            let a5964 = v0;
            let a5965 = v1;
            nat_sub(a5964, a5965)
        };
        let a5967 = 1u64;
        nat_sub(a5966, a5967)
    };
    let a5969 = v2;
    nat_add(a5968, a5969)
}

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f135({
        let a5970 = v0;
        let a5971 = v4;
        let a5972 = 0u64;
        nat_rem(a5970, a5971, a5972)
    }, {
        let a5973 = v1;
        let a5974 = v5;
        let a5975 = 0u64;
        nat_rem(a5973, a5974, a5975)
    }, {
        let a5976 = v2;
        let a5977 = v6;
        let a5978 = 0u64;
        nat_rem(a5976, a5977, a5978)
    }, {
        let a5979 = v3;
        let a5980 = v7;
        let a5981 = 0u64;
        nat_rem(a5979, a5980, a5981)
    }, v8, v9, v10, v11)
}

pub fn f230(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5982 = v1;
    let a5983 = 2147483648u64;
    if nat_lt(a5982, a5983) {
        let a5994 = {
            let a5992 = {
                let a5987 = {
                    let a5984 = v0;
                    let a5985 = v2;
                    let a5986 = 0u64;
                    nat_rem(a5984, a5985, a5986)
                };
                let a5988 = v2;
                nat_add(a5987, a5988)?
            };
            let a5993 = {
                let a5989 = v1;
                let a5990 = v2;
                let a5991 = 0u64;
                nat_rem(a5989, a5990, a5991)
            };
            nat_sub(a5992, a5993)
        };
        let a5995 = v2;
        let a5996 = 0u64;
        Ok(nat_rem(a5994, a5995, a5996))
    } else {
        let a5999 = {
            let a5997 = v0;
            let a5998 = f136(v1);
            nat_add(a5997, a5998)?
        };
        let a6000 = v2;
        let a6001 = 0u64;
        Ok(nat_rem(a5999, a6000, a6001))
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6004 = f137(v1);
    let a6005 = {
        let a6002 = v0;
        let a6003 = f136(v1);
        nat_add(a6002, a6003)?
    };
    if nat_le(a6004, a6005) {
        let a6006 = f232(v0, v1)?;
        let a6007 = v2;
        Ok(nat_lt(a6006, a6007))
    } else {
        Ok(false)
    }
}

pub fn f232(v0: u64, v1: u64) -> R<u64> {
    let a6010 = {
        let a6008 = v0;
        let a6009 = f136(v1);
        nat_add(a6008, a6009)?
    };
    let a6011 = f137(v1);
    Ok(nat_sub(a6010, a6011))
}

pub fn f233(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6012 = v0;
    let a6013 = v1;
    if nat_lt(a6012, a6013) {
        let a6014 = v1;
        let a6015 = v0;
        Ok(nat_sub(a6014, a6015))
    } else {
        let a6018 = v0;
        let a6019 = {
            let a6016 = v1;
            let a6017 = v2;
            nat_add(a6016, a6017)?
        };
        if nat_lt(a6018, a6019) {
            let a6020 = v0;
            let a6021 = v1;
            Ok(nat_sub(a6020, a6021))
        } else {
            let a6028 = {
                let a6024 = 2u64;
                let a6025 = {
                    let a6022 = v2;
                    let a6023 = 1u64;
                    nat_sub(a6022, a6023)
                };
                nat_mul(a6024, a6025)?
            };
            let a6029 = {
                let a6026 = v0;
                let a6027 = v1;
                nat_sub(a6026, a6027)
            };
            Ok(nat_sub(a6028, a6029))
        }
    }
}

pub fn f234(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6032 = {
        let a6030 = v2;
        let a6031 = f266(v1.clone(), f57(v0, v1.clone(), 0u64), f57(v0, v1.clone(), 1u64), f57(v0, v1.clone(), 2u64), f57(v0, v1.clone(), 3u64), false)?;
        nat_add(a6030, a6031)?
    };
    let a6033 = f266(v1.clone(), f57(v0, v1.clone(), 0u64), f57(v0, v1.clone(), 1u64), f57(v0, v1.clone(), 2u64), f57(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6032, a6033))
}

pub fn f235(v0: u64, v1: u64, v2: u64) -> bool {
    let a6034 = v0;
    let a6035 = 0u64;
    if nat_eq(a6034, a6035) {
        let a6036 = v2;
        let a6037 = v1;
        nat_le(a6036, a6037)
    } else {
        let a6038 = v0;
        let a6039 = 1u64;
        if nat_eq(a6038, a6039) {
            let a6040 = v2;
            let a6041 = v1;
            nat_lt(a6040, a6041)
        } else {
            let a6042 = v0;
            let a6043 = 2u64;
            if nat_eq(a6042, a6043) {
                let a6044 = v1;
                let a6045 = v2;
                nat_le(a6044, a6045)
            } else {
                let a6046 = v0;
                let a6047 = 3u64;
                if nat_eq(a6046, a6047) {
                    let a6048 = v1;
                    let a6049 = v2;
                    nat_lt(a6048, a6049)
                } else {
                    let a6050 = v1;
                    let a6051 = v2;
                    nat_eq(a6050, a6051)
                }
            }
        }
    }
}

pub fn f236(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6052 = v0;
    let a6053 = 0u64;
    if nat_eq(a6052, a6053) {
        Ok(f249(0u64))
    } else {
        let a6056 = {
            let a6054 = v0;
            let a6055 = 1u64;
            nat_add(a6054, a6055)?
        };
        let a6057 = f225(v1.clone(), 13u64, v2)?;
        if nat_eq(a6056, a6057) {
            Ok(Adt9::C4({
                let a6058 = v2;
                let a6059 = 0u64;
                if nat_eq(a6058, a6059) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6060 = v0;
                let a6061 = 1u64;
                nat_sub(a6060, a6061)
            };
            if f264(v1.clone(), v2) {
                let v4: u64 = f237(match v1.clone() {
                    Adt3::C0(h6062, _, _, _, _, _, _, _) => {
                        h6062
                    }
                }, false)?;
                let v5: u64 = {
                    let a6063 = v3;
                    let a6064 = v4;
                    let a6065 = 0u64;
                    nat_quot(a6063, a6064, a6065)
                };
                let v6: u64 = {
                    let a6066 = v3;
                    let a6067 = v4;
                    let a6068 = 0u64;
                    nat_rem(a6066, a6067, a6068)
                };
                let a6071 = {
                    let a6069 = v6;
                    let a6070 = 1u64;
                    nat_add(a6069, a6070)?
                };
                let a6072 = v4;
                if nat_eq(a6071, a6072) {
                    Ok(Adt9::C4({
                        let a6073 = v2;
                        let a6074 = 0u64;
                        if nat_eq(a6073, a6074) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f238(v6, v5, v1, v2)
                }
            } else {
                f267({
                    let a6075 = v3;
                    let a6076 = 2u64;
                    let a6077 = 0u64;
                    nat_quot(a6075, a6076, a6077)
                }, {
                    let a6078 = v3;
                    let a6079 = 2u64;
                    let a6080 = 0u64;
                    nat_rem(a6078, a6079, a6080)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f237(v0: u64, v1: bool) -> R<u64> {
    let a6083 = {
        let a6081 = v0;
        let a6082 = 2u64;
        nat_mul(a6081, a6082)?
    };
    let a6084 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6083, a6084)
}

pub fn f238(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6085 = v0;
    let a6086 = 0u64;
    if nat_eq(a6085, a6086) {
        Ok(f249(1u64))
    } else {
        f267({
            let a6095 = {
                let a6088 = v1;
                let a6089 = match v2.clone() {
                    Adt3::C0(h6087, _, _, _, _, _, _, _) => {
                        h6087
                    }
                };
                nat_mul(a6088, a6089)?
            };
            let a6096 = {
                let a6092 = {
                    let a6090 = v0;
                    let a6091 = 1u64;
                    nat_sub(a6090, a6091)
                };
                let a6093 = 2u64;
                let a6094 = 0u64;
                nat_quot(a6092, a6093, a6094)
            };
            nat_add(a6095, a6096)?
        }, {
            let a6099 = {
                let a6097 = v0;
                let a6098 = 1u64;
                nat_sub(a6097, a6098)
            };
            let a6100 = 2u64;
            let a6101 = 0u64;
            nat_rem(a6099, a6100, a6101)
        }, v2, v3, 1u64)
    }
}

pub fn f239(v0: u64, v1: u64) -> R<u64> {
    let a6102 = v0;
    let a6103 = f91(v1)?;
    let a6104 = 0u64;
    Ok(nat_rem(a6102, a6103, a6104))
}

pub fn f240(v0: u64, v1: u64) -> R<u64> {
    let a6105 = 4u64;
    let a6106 = f268(v0, v1)?;
    nat_mul(a6105, a6106)
}

pub fn f241(v0: u64, v1: u64) -> R<u64> {
    let a6107 = 4u64;
    let a6108 = f269(v0, v1)?;
    nat_mul(a6107, a6108)
}

pub fn f242(v0: u64, v1: u64) -> R<bool> {
    let a6114 = {
        let a6111 = {
            let a6109 = f239(v0, v1)?;
            let a6110 = 2u64;
            nat_sub(a6109, a6110)
        };
        let a6112 = 2u64;
        let a6113 = 0u64;
        nat_rem(a6111, a6112, a6113)
    };
    let a6115 = 0u64;
    Ok(nat_eq(a6114, a6115))
}

pub fn f243(v0: u64) -> u64 {
    let m6116 = f206(v0);
    match m6116 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6117, _) => {
                    h6117
                }
            }
        }
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6118 = f206(v0);
    match m6118 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f213(v0) {
                let a6119 = v1;
                let a6120 = v7;
                nat_lt(a6119, a6120)
            } else {
                false
            } {
                f173(v8, v2)
            } else {
                false
            } {
                let a6121 = v3;
                let a6122 = f270(v4, v5, v6)?;
                nat_lt(a6121, a6122)
            } else {
                false
            } {
                let a6127 = {
                    let a6124 = v7;
                    let a6125 = match v13.clone() {
                        Adt19::C0(_, h6123, _) => {
                            h6123
                        }
                    };
                    let a6126 = 0u64;
                    nat_rem(a6124, a6125, a6126)
                };
                let a6128 = 0u64;
                nat_eq(a6127, a6128)
            } else {
                false
            } {
                Ok(Some({
                    let a6157 = {
                        let a6147 = {
                            let a6137 = {
                                let a6133 = {
                                    let a6130 = v1;
                                    let a6131 = match v13 {
                                        Adt19::C0(_, h6129, _) => {
                                            h6129
                                        }
                                    };
                                    let a6132 = 0u64;
                                    nat_quot(a6130, a6131, a6132)
                                };
                                let a6134 = v9;
                                nat_mul(a6133, a6134)?
                            };
                            let a6138 = {
                                let a6135 = v2;
                                let a6136 = v10;
                                nat_mul(a6135, a6136)?
                            };
                            nat_add(a6137, a6138)?
                        };
                        let a6148 = {
                            let a6145 = {
                                let a6142 = {
                                    let a6139 = v3;
                                    let a6140 = v4;
                                    let a6141 = 0u64;
                                    nat_quot(a6139, a6140, a6141)
                                };
                                let a6143 = v5;
                                let a6144 = 0u64;
                                nat_rem(a6142, a6143, a6144)
                            };
                            let a6146 = v11;
                            nat_mul(a6145, a6146)?
                        };
                        nat_add(a6147, a6148)?
                    };
                    let a6158 = {
                        let a6155 = {
                            let a6152 = {
                                let a6149 = v3;
                                let a6150 = v4;
                                let a6151 = 0u64;
                                nat_quot(a6149, a6150, a6151)
                            };
                            let a6153 = v5;
                            let a6154 = 0u64;
                            nat_quot(a6152, a6153, a6154)
                        };
                        let a6156 = v12;
                        nat_mul(a6155, a6156)?
                    };
                    nat_add(a6157, a6158)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f245(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f271(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f246(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f272(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f247(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6159 = v0;
    let a6160 = 64u64;
    if nat_eq(a6159, a6160) {
        f273(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6161 = v0;
        let a6162 = 65u64;
        if nat_eq(a6161, a6162) {
            f274(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f248(v0: u64) -> u64 {
    let a6163 = v0;
    let a6164 = 30u64;
    if nat_eq(a6163, a6164) {
        4u64
    } else {
        3u64
    }
}

pub fn f249(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f250(v0: u64) -> bool {
    let a6165 = 0u64;
    let a6166 = v0;
    if nat_lt(a6165, a6166) {
        let a6167 = v0;
        let a6168 = 2139095040u64;
        nat_le(a6167, a6168)
    } else {
        false
    }
}

pub fn f251(v0: u64) -> u64 {
    let a6169 = v0;
    let a6170 = 40u64;
    if nat_lt(a6169, a6170) {
        let a6171 = v0;
        let a6172 = 20u64;
        if nat_lt(a6171, a6172) {
            let a6173 = v0;
            let a6174 = 10u64;
            if nat_lt(a6173, a6174) {
                let a6175 = v0;
                let a6176 = 5u64;
                if nat_lt(a6175, a6176) {
                    let a6177 = v0;
                    let a6178 = 2u64;
                    if nat_lt(a6177, a6178) {
                        let a6179 = v0;
                        let a6180 = 1u64;
                        if nat_lt(a6179, a6180) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6181 = v0;
                        let a6182 = 3u64;
                        if nat_lt(a6181, a6182) {
                            4u64
                        } else {
                            let a6183 = v0;
                            let a6184 = 4u64;
                            if nat_lt(a6183, a6184) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6185 = v0;
                    let a6186 = 7u64;
                    if nat_lt(a6185, a6186) {
                        let a6187 = v0;
                        let a6188 = 6u64;
                        if nat_lt(a6187, a6188) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6189 = v0;
                        let a6190 = 8u64;
                        if nat_lt(a6189, a6190) {
                            128u64
                        } else {
                            let a6191 = v0;
                            let a6192 = 9u64;
                            if nat_lt(a6191, a6192) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6193 = v0;
                let a6194 = 15u64;
                if nat_lt(a6193, a6194) {
                    let a6195 = v0;
                    let a6196 = 12u64;
                    if nat_lt(a6195, a6196) {
                        let a6197 = v0;
                        let a6198 = 11u64;
                        if nat_lt(a6197, a6198) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6199 = v0;
                        let a6200 = 13u64;
                        if nat_lt(a6199, a6200) {
                            4096u64
                        } else {
                            let a6201 = v0;
                            let a6202 = 14u64;
                            if nat_lt(a6201, a6202) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6203 = v0;
                    let a6204 = 17u64;
                    if nat_lt(a6203, a6204) {
                        let a6205 = v0;
                        let a6206 = 16u64;
                        if nat_lt(a6205, a6206) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6207 = v0;
                        let a6208 = 18u64;
                        if nat_lt(a6207, a6208) {
                            131072u64
                        } else {
                            let a6209 = v0;
                            let a6210 = 19u64;
                            if nat_lt(a6209, a6210) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6211 = v0;
            let a6212 = 30u64;
            if nat_lt(a6211, a6212) {
                let a6213 = v0;
                let a6214 = 25u64;
                if nat_lt(a6213, a6214) {
                    let a6215 = v0;
                    let a6216 = 22u64;
                    if nat_lt(a6215, a6216) {
                        let a6217 = v0;
                        let a6218 = 21u64;
                        if nat_lt(a6217, a6218) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6219 = v0;
                        let a6220 = 23u64;
                        if nat_lt(a6219, a6220) {
                            4194304u64
                        } else {
                            let a6221 = v0;
                            let a6222 = 24u64;
                            if nat_lt(a6221, a6222) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6223 = v0;
                    let a6224 = 27u64;
                    if nat_lt(a6223, a6224) {
                        let a6225 = v0;
                        let a6226 = 26u64;
                        if nat_lt(a6225, a6226) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6227 = v0;
                        let a6228 = 28u64;
                        if nat_lt(a6227, a6228) {
                            134217728u64
                        } else {
                            let a6229 = v0;
                            let a6230 = 29u64;
                            if nat_lt(a6229, a6230) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6231 = v0;
                let a6232 = 35u64;
                if nat_lt(a6231, a6232) {
                    let a6233 = v0;
                    let a6234 = 32u64;
                    if nat_lt(a6233, a6234) {
                        let a6235 = v0;
                        let a6236 = 31u64;
                        if nat_lt(a6235, a6236) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6237 = v0;
                        let a6238 = 33u64;
                        if nat_lt(a6237, a6238) {
                            4294967296u64
                        } else {
                            let a6239 = v0;
                            let a6240 = 34u64;
                            if nat_lt(a6239, a6240) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6241 = v0;
                    let a6242 = 37u64;
                    if nat_lt(a6241, a6242) {
                        let a6243 = v0;
                        let a6244 = 36u64;
                        if nat_lt(a6243, a6244) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6245 = v0;
                        let a6246 = 38u64;
                        if nat_lt(a6245, a6246) {
                            137438953472u64
                        } else {
                            let a6247 = v0;
                            let a6248 = 39u64;
                            if nat_lt(a6247, a6248) {
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

pub fn f252(v0: u64, v1: u64) -> bool {
    let a6249 = v0;
    let a6250 = 1u64;
    if nat_le(a6249, a6250) {
        true
    } else {
        let a6254 = {
            let a6251 = v1;
            let a6252 = 4u64;
            let a6253 = 0u64;
            nat_rem(a6251, a6252, a6253)
        };
        let a6255 = 0u64;
        nat_eq(a6254, a6255)
    }
}

pub fn f253(v0: u64, v1: u64) -> bool {
    let a6256 = v0;
    let a6257 = 1u64;
    if nat_le(a6256, a6257) {
        true
    } else {
        let a6258 = 4u64;
        let a6259 = v1;
        nat_le(a6258, a6259)
    }
}

pub fn f254(v0: u64, v1: u64) -> R<u64> {
    let a6260 = v0;
    let a6261 = 0u64;
    if nat_eq(a6260, a6261) {
        Ok(0u64)
    } else {
        let a6266 = 4u64;
        let a6267 = {
            let a6264 = {
                let a6262 = v0;
                let a6263 = 1u64;
                nat_sub(a6262, a6263)
            };
            let a6265 = v1;
            nat_mul(a6264, a6265)?
        };
        nat_add(a6266, a6267)
    }
}

pub fn f255(v0: bool, v1: u64) -> R<u64> {
    let a6272 = {
        let a6270 = {
            let a6268 = f275(v0, v1)?;
            let a6269 = f276(v0, v1)?;
            nat_add(a6268, a6269)?
        };
        let a6271 = f277(v0);
        nat_add(a6270, a6271)?
    };
    let a6273 = v1;
    nat_add(a6272, a6273)
}

pub fn f256(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f278(v0, v1)?;
    let v3: u64 = {
        let a6278 = {
            let a6276 = {
                let a6274 = v1;
                let a6275 = 1u64;
                nat_sub(a6274, a6275)
            };
            let a6277 = v2;
            nat_mul(a6276, a6277)?
        };
        let a6279 = v0;
        nat_sub(a6278, a6279)
    };
    let a6280 = v2;
    let a6281 = v3;
    if nat_lt(a6280, a6281) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f257(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6282 = v4;
    let a6283 = 0u64;
    if nat_eq(a6282, a6283) {
        Ok(0u64)
    } else {
        let a6290 = 2u64;
        let a6291 = {
            let a6288 = {
                let a6284 = v0;
                let a6285 = v1;
                nat_mul(a6284, a6285)?
            };
            let a6289 = {
                let a6286 = v2;
                let a6287 = v3;
                nat_mul(a6286, a6287)?
            };
            nat_mul(a6288, a6289)?
        };
        nat_mul(a6290, a6291)
    }
}

pub fn f258(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6296 = v3;
    let a6297 = {
        let a6294 = {
            let a6292 = f279(v2, v4)?;
            let a6293 = f280(v0, v1, f278(v2, v4)?)?;
            nat_mul(a6292, a6293)?
        };
        let a6295 = f280(v0, v1, f281(v2, v4)?)?;
        nat_add(a6294, a6295)?
    };
    nat_mul(a6296, a6297)
}

pub fn f259(v0: u64) -> R<u64> {
    let a6300 = 13u64;
    let a6301 = {
        let a6298 = 9u64;
        let a6299 = v0;
        nat_mul(a6298, a6299)?
    };
    nat_add(a6300, a6301)
}

pub fn f260(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f282(v0, v1, v6, v7, v8) {
        let a6302 = v5;
        let a6303 = 26u64;
        nat_eq(a6302, a6303)
    } else {
        false
    } {
        let a6304 = v3;
        let a6305 = v7;
        nat_eq(a6304, a6305)
    } else {
        false
    } {
        let a6306 = v4;
        let a6307 = v8;
        nat_eq(a6306, a6307)
    } else {
        false
    } {
        let a6308 = v9;
        let a6309 = 1u64;
        nat_eq(a6308, a6309)
    } else {
        false
    } {
        let a6310 = v10;
        let a6311 = f283(v0);
        nat_eq(a6310, a6311)
    } else {
        false
    } {
        let a6312 = v11;
        let a6313 = v1;
        nat_eq(a6312, a6313)
    } else {
        false
    } {
        let a6314 = v12;
        let a6315 = v6;
        nat_eq(a6314, a6315)
    } else {
        false
    } {
        let a6316 = v13;
        let a6317 = v7;
        nat_eq(a6316, a6317)
    } else {
        false
    } {
        let a6318 = v14;
        let a6319 = v8;
        nat_eq(a6318, a6319)
    } else {
        false
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6320 = v0;
    let a6321 = 0u64;
    if if if nat_eq(a6320, a6321) {
        true
    } else {
        let a6322 = v1;
        let a6323 = 0u64;
        nat_eq(a6322, a6323)
    } {
        true
    } else {
        let a6324 = v2;
        let a6325 = 0u64;
        nat_eq(a6324, a6325)
    } {
        true
    } else {
        let a6326 = v3;
        let a6327 = 0u64;
        nat_eq(a6326, a6327)
    }
}

pub fn f262(v0: Adt3, v1: bool) -> R<u64> {
    f266(v0.clone(), {
        let a6329 = match v0.clone() {
            Adt3::C0(h6328, _, _, _, _, _, _, _) => {
                h6328
            }
        };
        let a6330 = 1u64;
        nat_sub(a6329, a6330)
    }, {
        let a6332 = match v0.clone() {
            Adt3::C0(_, h6331, _, _, _, _, _, _) => {
                h6331
            }
        };
        let a6333 = 1u64;
        nat_sub(a6332, a6333)
    }, {
        let a6335 = match v0.clone() {
            Adt3::C0(_, _, h6334, _, _, _, _, _) => {
                h6334
            }
        };
        let a6336 = 1u64;
        nat_sub(a6335, a6336)
    }, {
        let a6338 = match v0 {
            Adt3::C0(_, _, _, h6337, _, _, _, _) => {
                h6337
            }
        };
        let a6339 = 1u64;
        nat_sub(a6338, a6339)
    }, v1)
}

pub fn f263(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6340 = v0;
    let a6341 = 0u64;
    if nat_eq(a6340, a6341) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6342 = v0;
            let a6343 = 1u64;
            nat_sub(a6342, a6343)
        };
        let v5: u64 = {
            let a6344 = v4;
            let a6345 = v1;
            let a6346 = 0u64;
            nat_quot(a6344, a6345, a6346)
        };
        let v6: u64 = {
            let a6354 = {
                let a6347 = v5;
                let a6348 = v2;
                nat_mul(a6347, a6348)?
            };
            let a6355 = {
                let a6352 = {
                    let a6349 = v4;
                    let a6350 = v1;
                    let a6351 = 0u64;
                    nat_rem(a6349, a6350, a6351)
                };
                let a6353 = v3;
                nat_mul(a6352, a6353)?
            };
            nat_add(a6354, a6355)?
        };
        let a6356 = v5;
        let a6357 = 0u64;
        if nat_eq(a6356, a6357) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6366 = {
                    let a6360 = {
                        let a6358 = v5;
                        let a6359 = 1u64;
                        nat_sub(a6358, a6359)
                    };
                    let a6361 = v2;
                    nat_mul(a6360, a6361)?
                };
                let a6367 = {
                    let a6364 = {
                        let a6362 = v1;
                        let a6363 = 1u64;
                        nat_sub(a6362, a6363)
                    };
                    let a6365 = v3;
                    nat_mul(a6364, a6365)?
                };
                nat_add(a6366, a6367)?
            };
            let a6368 = v6;
            let a6369 = v7;
            if nat_lt(a6368, a6369) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f264(v0: Adt3, v1: u64) -> bool {
    let a6370 = v1;
    let a6371 = 0u64;
    if !nat_eq(a6370, a6371) {
        true
    } else {
        let a6373 = match v0 {
            Adt3::C0(_, _, _, _, h6372, _, _, _) => {
                h6372
            }
        };
        let a6374 = 4u64;
        nat_eq(a6373, a6374)
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6375 = f284(v0, v1);
    match m6375 {
        None => {
            if v5 {
                let m6376 = f285(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6376 {
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
                let a6377 = 4u64;
                let a6378 = v28;
                nat_le(a6377, a6378)
            };
            let a6379 = v2;
            let a6380 = 0u64;
            if if if if if nat_eq(a6379, a6380) {
                let a6381 = v4;
                let a6382 = 0u64;
                nat_eq(a6381, a6382)
            } else {
                false
            } {
                let a6383 = v6;
                let a6384 = v14;
                if nat_eq(a6383, a6384) {
                    let a6385 = v7;
                    let a6386 = v15;
                    nat_eq(a6385, a6386)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6387 = v8;
                let a6388 = v16;
                if nat_eq(a6387, a6388) {
                    let a6389 = v9;
                    let a6390 = v17;
                    nat_eq(a6389, a6390)
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
                        let a6391 = v3;
                        let a6392 = 0u64;
                        nat_eq(a6391, a6392)
                    } else {
                        false
                    } {
                        f216(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6397 = {
                        let a6393 = v6;
                        let a6394 = v7;
                        nat_mul(a6393, a6394)?
                    };
                    let a6398 = {
                        let a6395 = v8;
                        let a6396 = v9;
                        nat_mul(a6395, a6396)?
                    };
                    nat_mul(a6397, a6398)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f266(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6419 = {
        let a6413 = {
            let a6407 = {
                let a6401 = v1;
                let a6402 = if v5 {
                    f136(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6399, _, _, _) => {
                            h6399
                        }
                    })
                } else {
                    f137(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6400, _, _, _) => {
                            h6400
                        }
                    })
                };
                nat_mul(a6401, a6402)?
            };
            let a6408 = {
                let a6405 = v2;
                let a6406 = if v5 {
                    f136(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6403, _, _) => {
                            h6403
                        }
                    })
                } else {
                    f137(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6404, _, _) => {
                            h6404
                        }
                    })
                };
                nat_mul(a6405, a6406)?
            };
            nat_add(a6407, a6408)?
        };
        let a6414 = {
            let a6411 = v3;
            let a6412 = if v5 {
                f136(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6409, _) => {
                        h6409
                    }
                })
            } else {
                f137(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6410, _) => {
                        h6410
                    }
                })
            };
            nat_mul(a6411, a6412)?
        };
        nat_add(a6413, a6414)?
    };
    let a6420 = {
        let a6417 = v4;
        let a6418 = if v5 {
            f136(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6415) => {
                    h6415
                }
            })
        } else {
            f137(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6416) => {
                    h6416
                }
            })
        };
        nat_mul(a6417, a6418)?
    };
    nat_add(a6419, a6420)
}

pub fn f267(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6421 = v1;
    let a6422 = 0u64;
    if nat_eq(a6421, a6422) {
        Ok(Adt9::C4({
            let a6423 = v3;
            let a6424 = 0u64;
            if nat_eq(a6423, a6424) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f88(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6425 = v3;
            let a6426 = 0u64;
            if nat_eq(a6425, a6426) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f268(v0: u64, v1: u64) -> R<u64> {
    let a6427 = v0;
    let a6428 = f91(v1)?;
    let a6429 = 0u64;
    Ok(nat_quot(a6427, a6428, a6429))
}

pub fn f269(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f268(v0, v1)?;
    let v3: u64 = {
        let a6432 = {
            let a6430 = f239(v0, v1)?;
            let a6431 = 2u64;
            nat_sub(a6430, a6431)
        };
        let a6433 = 2u64;
        let a6434 = 0u64;
        nat_quot(a6432, a6433, a6434)
    };
    let v4: u64 = {
        let a6435 = v2;
        let a6436 = v1;
        nat_mul(a6435, a6436)?
    };
    if f242(v0, v1)? {
        let a6437 = v4;
        let a6438 = v3;
        nat_add(a6437, a6438)
    } else {
        let a6450 = {
            let a6446 = v4;
            let a6447 = {
                let a6444 = {
                    let a6439 = v1;
                    let a6440 = 1u64;
                    nat_sub(a6439, a6440)
                };
                let a6445 = {
                    let a6441 = v2;
                    let a6442 = v1;
                    let a6443 = 0u64;
                    nat_rem(a6441, a6442, a6443)
                };
                nat_mul(a6444, a6445)?
            };
            nat_sub(a6446, a6447)
        };
        let a6451 = {
            let a6448 = v3;
            let a6449 = v1;
            nat_mul(a6448, a6449)?
        };
        nat_add(a6450, a6451)
    }
}

pub fn f270(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6452 = v0;
    let a6453 = 0u64;
    if if if nat_eq(a6452, a6453) {
        true
    } else {
        let a6454 = v1;
        let a6455 = 0u64;
        nat_eq(a6454, a6455)
    } {
        true
    } else {
        let a6456 = v2;
        let a6457 = 0u64;
        nat_eq(a6456, a6457)
    } {
        Ok(0u64)
    } else {
        let a6460 = {
            let a6458 = v0;
            let a6459 = v1;
            nat_mul(a6458, a6459)?
        };
        let a6461 = v2;
        nat_mul(a6460, a6461)
    }
}

pub fn f271(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f100(match v1.clone() {
        Adt20::C0(h6462, _, _, _, _, _, _, _) => {
            h6462
        }
    })?;
    let v6: u64 = {
        let a6468 = match v1.clone() {
            Adt20::C0(_, h6463, _, _, _, _, _, _) => {
                h6463
            }
        };
        let a6469 = {
            let a6466 = match v1.clone() {
                Adt20::C0(_, _, h6464, _, _, _, _, _) => {
                    h6464
                }
            };
            let a6467 = match v1.clone() {
                Adt20::C0(_, _, _, h6465, _, _, _, _) => {
                    h6465
                }
            };
            nat_mul(a6466, a6467)?
        };
        nat_mul(a6468, a6469)?
    };
    let a6471 = v0;
    let a6472 = f52(match v1.clone() {
        Adt20::C0(h6470, _, _, _, _, _, _, _) => {
            h6470
        }
    }, v6)?;
    if nat_lt(a6471, a6472) {
        f286({
            let a6473 = v0;
            let a6474 = v5;
            let a6475 = 0u64;
            nat_rem(a6473, a6474, a6475)
        }, {
            let a6476 = v0;
            let a6477 = v5;
            let a6478 = 0u64;
            nat_quot(a6476, a6477, a6478)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f272(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6479 = v6;
        let a6480 = 6u64;
        nat_sub(a6479, a6480)
    };
    let v9: Adt20 = f287(v3);
    let a6482 = v0;
    let a6483 = f203(match v1.clone() {
        Adt20::C0(h6481, _, _, _, _, _, _, _) => {
            h6481
        }
    }, v5, v8)?;
    if nat_lt(a6482, a6483) {
        let v10: u64 = f97(match v1.clone() {
            Adt20::C0(h6484, _, _, _, _, _, _, _) => {
                h6484
            }
        }, v5, v8)?;
        let a6485 = v0;
        let a6486 = v10;
        if nat_lt(a6485, a6486) {
            f288(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6487 = v0;
                let a6488 = v10;
                nat_sub(a6487, a6488)
            };
            let v12: u64 = f259(match v1.clone() {
                Adt20::C0(h6489, _, _, _, _, _, _, _) => {
                    h6489
                }
            })?;
            f289({
                let a6490 = v11;
                let a6491 = v12;
                let a6492 = 0u64;
                nat_rem(a6490, a6491, a6492)
            }, {
                let a6493 = v11;
                let a6494 = v12;
                let a6495 = 0u64;
                nat_quot(a6493, a6494, a6495)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f273(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6496 = v5;
        let a6497 = 6u64;
        nat_sub(a6496, a6497)
    };
    let v8: bool = f197(match v1.clone() {
        Adt20::C0(_, _, _, _, h6498, _, _, _) => {
            h6498
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6499, _, _, _) => {
            h6499
        }
    });
    let a6501 = v0;
    let a6502 = f196(v8, match v1.clone() {
        Adt20::C0(h6500, _, _, _, _, _, _, _) => {
            h6500
        }
    }, v4, v7)?;
    if nat_lt(a6501, a6502) {
        let v9: u64 = f97(match v1.clone() {
            Adt20::C0(h6503, _, _, _, _, _, _, _) => {
                h6503
            }
        }, v4, v7)?;
        let a6504 = v0;
        let a6505 = v9;
        if nat_lt(a6504, a6505) {
            f288(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6506 = v0;
                let a6507 = v9;
                nat_sub(a6506, a6507)
            };
            let v11: u64 = f255(v8, match v1.clone() {
                Adt20::C0(h6508, _, _, _, _, _, _, _) => {
                    h6508
                }
            })?;
            f290({
                let a6509 = v10;
                let a6510 = v11;
                let a6511 = 0u64;
                nat_rem(a6509, a6510, a6511)
            }, {
                let a6512 = v10;
                let a6513 = v11;
                let a6514 = 0u64;
                nat_quot(a6512, a6513, a6514)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f274(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6515 = v5;
        let a6516 = 6u64;
        nat_sub(a6515, a6516)
    };
    let v8: Adt20 = f287(v2);
    let v9: u64 = f257(match v1.clone() {
        Adt20::C0(h6517, _, _, _, _, _, _, _) => {
            h6517
        }
    }, match v1.clone() {
        Adt20::C0(_, h6518, _, _, _, _, _, _) => {
            h6518
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6519, _, _, _, _, _) => {
            h6519
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6520, _, _, _, _) => {
            h6520
        }
    }, v7)?;
    let a6525 = v0;
    let a6526 = f202(match v1.clone() {
        Adt20::C0(h6521, _, _, _, _, _, _, _) => {
            h6521
        }
    }, match v1.clone() {
        Adt20::C0(_, h6522, _, _, _, _, _, _) => {
            h6522
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6523, _, _, _, _, _) => {
            h6523
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6524, _, _, _, _) => {
            h6524
        }
    }, v4, v7)?;
    if nat_lt(a6525, a6526) {
        let a6527 = v0;
        let a6528 = v9;
        if nat_lt(a6527, a6528) {
            f288(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f291({
                let a6529 = v0;
                let a6530 = v9;
                nat_sub(a6529, a6530)
            }, match v1.clone() {
                Adt20::C0(h6531, _, _, _, _, _, _, _) => {
                    h6531
                }
            }, match v1.clone() {
                Adt20::C0(_, h6532, _, _, _, _, _, _) => {
                    h6532
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6533, _, _, _, _, _) => {
                    h6533
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6534, _, _, _, _) => {
                    h6534
                }
            }, v4)?;
            f292(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f275(v0: bool, v1: u64) -> R<u64> {
    let a6537 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6538 = {
        let a6535 = f293(v0);
        let a6536 = v1;
        nat_mul(a6535, a6536)?
    };
    nat_add(a6537, a6538)
}

pub fn f276(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6546 = {
            let a6539 = 13u64;
            let a6540 = f294(v1);
            nat_mul(a6539, a6540)?
        };
        let a6547 = {
            let a6544 = 4u64;
            let a6545 = {
                let a6541 = v1;
                let a6542 = 4u64;
                let a6543 = 0u64;
                nat_rem(a6541, a6542, a6543)
            };
            nat_mul(a6544, a6545)?
        };
        nat_add(a6546, a6547)
    } else {
        let a6548 = 3u64;
        let a6549 = v1;
        nat_mul(a6548, a6549)
    }
}

pub fn f277(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f278(v0: u64, v1: u64) -> R<u64> {
    let a6554 = {
        let a6552 = {
            let a6550 = v0;
            let a6551 = v1;
            nat_add(a6550, a6551)?
        };
        let a6553 = 1u64;
        nat_sub(a6552, a6553)
    };
    let a6555 = v1;
    let a6556 = 0u64;
    Ok(nat_quot(a6554, a6555, a6556))
}

pub fn f279(v0: u64, v1: u64) -> R<u64> {
    let a6557 = v0;
    let a6558 = f278(v0, v1)?;
    let a6559 = 0u64;
    Ok(nat_quot(a6557, a6558, a6559))
}

pub fn f280(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6560 = v2;
    let a6561 = 0u64;
    if nat_eq(a6560, a6561) {
        Ok(0u64)
    } else {
        let a6566 = 11u64;
        let a6567 = {
            let a6564 = {
                let a6562 = v1;
                let a6563 = v2;
                nat_mul(a6562, a6563)?
            };
            let a6565 = f295(v0)?;
            nat_mul(a6564, a6565)?
        };
        nat_add(a6566, a6567)
    }
}

pub fn f281(v0: u64, v1: u64) -> R<u64> {
    let a6568 = v0;
    let a6569 = f278(v0, v1)?;
    let a6570 = 0u64;
    Ok(nat_rem(a6568, a6569, a6570))
}

pub fn f282(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f261(v1, v2, v3, v4) {
        f296(v0)
    } else {
        f213(v0)
    }
}

pub fn f283(v0: u64) -> u64 {
    let a6571 = v0;
    let a6572 = 26u64;
    if nat_eq(a6571, a6572) {
        26u64
    } else {
        0u64
    }
}

pub fn f284(v0: u64, v1: u64) -> Option<u64> {
    let a6573 = v0;
    let a6574 = 2u64;
    if nat_eq(a6573, a6574) {
        Some(0u64)
    } else {
        let a6575 = v0;
        let a6576 = 5u64;
        if nat_eq(a6575, a6576) {
            Some(1u64)
        } else {
            let a6577 = v0;
            let a6578 = 6u64;
            if nat_eq(a6577, a6578) {
                Some(2u64)
            } else {
                let a6579 = v0;
                let a6580 = 7u64;
                if nat_eq(a6579, a6580) {
                    Some(3u64)
                } else {
                    let a6581 = v0;
                    let a6582 = 9u64;
                    if nat_eq(a6581, a6582) {
                        Some(4u64)
                    } else {
                        let a6583 = v0;
                        let a6584 = 8u64;
                        if nat_eq(a6583, a6584) {
                            Some(5u64)
                        } else {
                            let a6585 = v0;
                            let a6586 = 89u64;
                            if nat_eq(a6585, a6586) {
                                let a6587 = v1;
                                let a6588 = 2u64;
                                if nat_eq(a6587, a6588) {
                                    Some(6u64)
                                } else {
                                    let a6589 = v1;
                                    let a6590 = 0u64;
                                    if nat_eq(a6589, a6590) {
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

pub fn f285(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6591 = v0;
    let a6592 = 30u64;
    if nat_eq(a6591, a6592) {
        let a6593 = v2;
        let a6594 = 0u64;
        if if if nat_eq(a6593, a6594) {
            true
        } else {
            let a6595 = v2;
            let a6596 = 1u64;
            nat_eq(a6595, a6596)
        } {
            let a6597 = v1;
            let a6598 = 0u64;
            if nat_eq(a6597, a6598) {
                true
            } else {
                f297(v1, v4)
            }
        } else {
            false
        } {
            f298(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f299(v1) {
            f299(v2)
        } else {
            false
        } {
            let a6599 = v1;
            let a6600 = 0u64;
            if if nat_eq(a6599, a6600) {
                true
            } else {
                let a6601 = v2;
                let a6602 = 0u64;
                nat_eq(a6601, a6602)
            } {
                true
            } else {
                let a6603 = v1;
                let a6604 = v2;
                nat_eq(a6603, a6604)
            }
        } else {
            false
        } {
            true
        } else {
            let a6605 = v1;
            let a6606 = 43u64;
            if nat_eq(a6605, a6606) {
                f299(v2)
            } else {
                false
            }
        } {
            f298(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f286(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6607, _, _, _, _, _, _, _) => {
            h6607
        }
    };
    let v7: u64 = {
        let a6610 = 1u64;
        let a6611 = {
            let a6608 = 3u64;
            let a6609 = v6;
            nat_mul(a6608, a6609)?
        };
        nat_add(a6610, a6611)?
    };
    let a6612 = v0;
    let a6613 = 0u64;
    if nat_eq(a6612, a6613) {
        Ok(f300(0u64))
    } else {
        let a6614 = v0;
        let a6615 = v7;
        if nat_lt(a6614, a6615) {
            let v8: u64 = {
                let a6616 = v0;
                let a6617 = 1u64;
                nat_sub(a6616, a6617)
            };
            let v9: u64 = {
                let a6623 = {
                    let a6618 = v1;
                    let a6619 = v6;
                    nat_mul(a6618, a6619)?
                };
                let a6624 = {
                    let a6620 = v8;
                    let a6621 = 3u64;
                    let a6622 = 0u64;
                    nat_quot(a6620, a6621, a6622)
                };
                nat_add(a6623, a6624)?
            };
            let v10: u64 = f301(v9, v2)?;
            let v11: u64 = {
                let a6625 = v8;
                let a6626 = 3u64;
                let a6627 = 0u64;
                nat_rem(a6625, a6626, a6627)
            };
            let a6628 = v11;
            let a6629 = 0u64;
            if nat_eq(a6628, a6629) {
                Ok(f302(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6630 = v11;
                let a6631 = 1u64;
                if nat_eq(a6630, a6631) {
                    Ok(f302(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f302(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6632 = v0;
            let a6633 = v7;
            if nat_eq(a6632, a6633) {
                Ok(f302(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6636 = v0;
                let a6637 = {
                    let a6634 = v7;
                    let a6635 = 1u64;
                    nat_add(a6634, a6635)?
                };
                if nat_eq(a6636, a6637) {
                    Ok(f302(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6640 = v0;
                    let a6641 = {
                        let a6638 = v7;
                        let a6639 = 2u64;
                        nat_add(a6638, a6639)?
                    };
                    if nat_eq(a6640, a6641) {
                        Ok(f302(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6644 = v0;
                        let a6645 = {
                            let a6642 = v7;
                            let a6643 = 3u64;
                            nat_add(a6642, a6643)?
                        };
                        if nat_eq(a6644, a6645) {
                            let a6646 = v5;
                            let a6647 = 1u64;
                            if if nat_eq(a6646, a6647) {
                                true
                            } else {
                                let a6648 = v5;
                                let a6649 = 2u64;
                                nat_eq(a6648, a6649)
                            } {
                                Ok(f302(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f302(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6656 = {
                                    let a6650 = v1;
                                    let a6651 = v6;
                                    nat_mul(a6650, a6651)?
                                };
                                let a6657 = {
                                    let a6654 = v0;
                                    let a6655 = {
                                        let a6652 = v7;
                                        let a6653 = 4u64;
                                        nat_add(a6652, a6653)?
                                    };
                                    nat_sub(a6654, a6655)
                                };
                                nat_add(a6656, a6657)?
                            };
                            Ok(f302(10u64, 0u64, f301(v12, v2)?, 1u64, 2u64, 3u64, f301(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f287(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6658, _, _, _, _, _, _, _) => {
            h6658
        }
    }, match v0.clone() {
        Adt20::C0(_, h6659, _, _, _, _, _, _) => {
            h6659
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6660, _, _, _, _, _) => {
            h6660
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6661, _, _, _, _) => {
            h6661
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6662, _, _) => {
            h6662
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6663, _) => {
            h6663
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6664) => {
            h6664
        }
    })
}

pub fn f288(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6671 = v3;
        let a6672 = {
            let a6668 = f301({
                let a6665 = v0;
                let a6666 = 2u64;
                let a6667 = 0u64;
                nat_quot(a6665, a6666, a6667)
            }, v1)?;
            let a6669 = v2;
            let a6670 = 0u64;
            nat_quot(a6668, a6669, a6670)
        };
        nat_add(a6671, a6672)?
    };
    let a6676 = {
        let a6673 = v0;
        let a6674 = 2u64;
        let a6675 = 0u64;
        nat_rem(a6673, a6674, a6675)
    };
    let a6677 = 0u64;
    if nat_eq(a6676, a6677) {
        Ok(f302(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6678 = v4;
        let a6679 = 0u64;
        if nat_eq(a6678, a6679) {
            Ok(f302(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f289(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f303(match v2.clone() {
        Adt20::C0(h6680, _, _, _, _, _, _, _) => {
            h6680
        }
    })?;
    let v7: u64 = f304(match v2.clone() {
        Adt20::C0(h6681, _, _, _, _, _, _, _) => {
            h6681
        }
    })?;
    let a6682 = v0;
    let a6683 = 2u64;
    if nat_lt(a6682, a6683) {
        Ok(f300(v0))
    } else {
        let a6684 = v0;
        let a6685 = v6;
        if nat_lt(a6684, a6685) {
            f305({
                let a6686 = v0;
                let a6687 = 2u64;
                nat_sub(a6686, a6687)
            }, v1, v2, v3)
        } else {
            let a6688 = v0;
            let a6689 = v7;
            if nat_lt(a6688, a6689) {
                Ok(f306({
                    let a6690 = v0;
                    let a6691 = v6;
                    nat_sub(a6690, a6691)
                }, match v2 {
                    Adt20::C0(h6692, _, _, _, _, _, _, _) => {
                        h6692
                    }
                }, v5))
            } else {
                let a6694 = v0;
                let a6695 = f259(match v2.clone() {
                    Adt20::C0(h6693, _, _, _, _, _, _, _) => {
                        h6693
                    }
                })?;
                if nat_lt(a6694, a6695) {
                    f307({
                        let a6696 = v0;
                        let a6697 = v7;
                        nat_sub(a6696, a6697)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f290(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f275(v5, match v2.clone() {
        Adt20::C0(h6698, _, _, _, _, _, _, _) => {
            h6698
        }
    })?;
    let v7: u64 = {
        let a6700 = v6;
        let a6701 = f276(v5, match v2.clone() {
            Adt20::C0(h6699, _, _, _, _, _, _, _) => {
                h6699
            }
        })?;
        nat_add(a6700, a6701)?
    };
    let v8: u64 = {
        let a6702 = v7;
        let a6703 = f277(v5);
        nat_add(a6702, a6703)?
    };
    let a6704 = v0;
    let a6705 = v6;
    if nat_lt(a6704, a6705) {
        f308(v0, v1, v2, v5)
    } else {
        let a6706 = v0;
        let a6707 = v7;
        if nat_lt(a6706, a6707) {
            f309({
                let a6708 = v0;
                let a6709 = v6;
                nat_sub(a6708, a6709)
            }, v1, v2, v3, v5)
        } else {
            let a6710 = v0;
            let a6711 = v8;
            if nat_lt(a6710, a6711) {
                Ok(f310({
                    let a6712 = v0;
                    let a6713 = v7;
                    nat_sub(a6712, a6713)
                }, match v2 {
                    Adt20::C0(h6714, _, _, _, _, _, _, _) => {
                        h6714
                    }
                }, v4, v5))
            } else {
                let a6718 = v0;
                let a6719 = {
                    let a6716 = v8;
                    let a6717 = match v2.clone() {
                        Adt20::C0(h6715, _, _, _, _, _, _, _) => {
                            h6715
                        }
                    };
                    nat_add(a6716, a6717)?
                };
                if nat_lt(a6718, a6719) {
                    let v9: u64 = f301({
                        let a6725 = {
                            let a6721 = v1;
                            let a6722 = match v2 {
                                Adt20::C0(h6720, _, _, _, _, _, _, _) => {
                                    h6720
                                }
                            };
                            nat_mul(a6721, a6722)?
                        };
                        let a6726 = {
                            let a6723 = v0;
                            let a6724 = v8;
                            nat_sub(a6723, a6724)
                        };
                        nat_add(a6725, a6726)?
                    }, v3)?;
                    Ok(f302(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f291(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f278(v3, v5)?;
    let v7: u64 = f280(v1, v2, v6)?;
    let v8: u64 = {
        let a6729 = {
            let a6727 = f279(v3, v5)?;
            let a6728 = v4;
            nat_mul(a6727, a6728)?
        };
        let a6730 = v7;
        nat_mul(a6729, a6730)?
    };
    let a6731 = v0;
    let a6732 = v8;
    if nat_lt(a6731, a6732) {
        let v9: u64 = {
            let a6733 = v0;
            let a6734 = v7;
            let a6735 = 0u64;
            nat_quot(a6733, a6734, a6735)
        };
        Ok(Adt21::C0({
            let a6739 = {
                let a6736 = v9;
                let a6737 = v4;
                let a6738 = 0u64;
                nat_quot(a6736, a6737, a6738)
            };
            let a6740 = v6;
            nat_mul(a6739, a6740)?
        }, v6, {
            let a6741 = v9;
            let a6742 = v4;
            let a6743 = 0u64;
            nat_rem(a6741, a6742, a6743)
        }, {
            let a6744 = v0;
            let a6745 = v7;
            let a6746 = 0u64;
            nat_rem(a6744, a6745, a6746)
        }))
    } else {
        let v10: u64 = {
            let a6747 = v0;
            let a6748 = v8;
            nat_sub(a6747, a6748)
        };
        let v11: u64 = f280(v1, v2, f281(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6749 = f279(v3, v5)?;
            let a6750 = v6;
            nat_mul(a6749, a6750)?
        }, f281(v3, v5)?, {
            let a6751 = v10;
            let a6752 = v11;
            let a6753 = 0u64;
            nat_quot(a6751, a6752, a6753)
        }, {
            let a6754 = v10;
            let a6755 = v11;
            let a6756 = 0u64;
            nat_rem(a6754, a6755, a6756)
        }))
    }
}

pub fn f292(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h6757) => {
            h6757
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h6758, _, _, _, _, _, _, _) => {
            h6758
        }
    };
    let v6: u64 = {
        let a6761 = match v0.clone() {
            Adt21::C0(_, h6759, _, _) => {
                h6759
            }
        };
        let a6762 = match v1.clone() {
            Adt20::C0(_, h6760, _, _, _, _, _, _) => {
                h6760
            }
        };
        nat_mul(a6761, a6762)?
    };
    let v7: u64 = {
        let a6763 = v6;
        let a6764 = v5;
        nat_mul(a6763, a6764)?
    };
    let v8: u64 = {
        let a6767 = {
            let a6765 = 2u64;
            let a6766 = v5;
            nat_mul(a6765, a6766)?
        };
        let a6768 = 2u64;
        nat_add(a6767, a6768)?
    };
    let v9: u64 = {
        let a6771 = {
            let a6769 = 4u64;
            let a6770 = v5;
            nat_mul(a6769, a6770)?
        };
        let a6772 = 2u64;
        nat_add(a6771, a6772)?
    };
    let v10: u64 = {
        let a6775 = 1u64;
        let a6776 = {
            let a6773 = v6;
            let a6774 = v8;
            nat_mul(a6773, a6774)?
        };
        nat_add(a6775, a6776)?
    };
    let v11: u64 = {
        let a6777 = v10;
        let a6778 = 4u64;
        nat_add(a6777, a6778)?
    };
    let v12: u64 = {
        let a6781 = v11;
        let a6782 = {
            let a6779 = v6;
            let a6780 = v9;
            nat_mul(a6779, a6780)?
        };
        nat_add(a6781, a6782)?
    };
    let v13: u64 = {
        let a6783 = v12;
        let a6784 = 6u64;
        nat_add(a6783, a6784)?
    };
    let a6785 = v4;
    let a6786 = 0u64;
    if nat_eq(a6785, a6786) {
        Ok(f300(0u64))
    } else {
        let a6787 = v4;
        let a6788 = v10;
        if nat_lt(a6787, a6788) {
            let v14: u64 = {
                let a6789 = v4;
                let a6790 = 1u64;
                nat_sub(a6789, a6790)
            };
            f311(false, {
                let a6791 = v14;
                let a6792 = v8;
                let a6793 = 0u64;
                nat_rem(a6791, a6792, a6793)
            }, {
                let a6794 = v14;
                let a6795 = v8;
                let a6796 = 0u64;
                nat_quot(a6794, a6795, a6796)
            }, v0, v1, v2)
        } else {
            let a6797 = v4;
            let a6798 = v11;
            if nat_lt(a6797, a6798) {
                Ok(f312({
                    let a6799 = v4;
                    let a6800 = v10;
                    nat_sub(a6799, a6800)
                }, v7))
            } else {
                let a6801 = v4;
                let a6802 = v12;
                if nat_lt(a6801, a6802) {
                    let v15: u64 = {
                        let a6803 = v4;
                        let a6804 = v11;
                        nat_sub(a6803, a6804)
                    };
                    f311(true, {
                        let a6805 = v15;
                        let a6806 = v9;
                        let a6807 = 0u64;
                        nat_rem(a6805, a6806, a6807)
                    }, {
                        let a6808 = v15;
                        let a6809 = v9;
                        let a6810 = 0u64;
                        nat_quot(a6808, a6809, a6810)
                    }, v0, v1, v2)
                } else {
                    let a6811 = v4;
                    let a6812 = v13;
                    if nat_lt(a6811, a6812) {
                        Ok(f313({
                            let a6813 = v4;
                            let a6814 = v12;
                            nat_sub(a6813, a6814)
                        }, v7, v3))
                    } else {
                        let a6817 = v4;
                        let a6818 = {
                            let a6815 = v13;
                            let a6816 = v7;
                            nat_add(a6815, a6816)?
                        };
                        if nat_lt(a6817, a6818) {
                            let v16: u64 = {
                                let a6819 = v4;
                                let a6820 = v13;
                                nat_sub(a6819, a6820)
                            };
                            let v17: u64 = {
                                let a6821 = v16;
                                let a6822 = v5;
                                let a6823 = 0u64;
                                nat_quot(a6821, a6822, a6823)
                            };
                            let v18: u64 = {
                                let a6829 = {
                                    let a6824 = f314(v17, v0, v1)?;
                                    let a6825 = v5;
                                    nat_mul(a6824, a6825)?
                                };
                                let a6830 = {
                                    let a6826 = v16;
                                    let a6827 = v5;
                                    let a6828 = 0u64;
                                    nat_rem(a6826, a6827, a6828)
                                };
                                nat_add(a6829, a6830)?
                            };
                            let v19: u64 = f301(v18, v2)?;
                            Ok(f302(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f293(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f294(v0: u64) -> u64 {
    let a6831 = v0;
    let a6832 = 4u64;
    let a6833 = 0u64;
    nat_quot(a6831, a6832, a6833)
}

pub fn f295(v0: u64) -> R<u64> {
    let a6836 = {
        let a6834 = 7u64;
        let a6835 = v0;
        nat_mul(a6834, a6835)?
    };
    let a6837 = 4u64;
    nat_add(a6836, a6837)
}

pub fn f296(v0: u64) -> bool {
    let m6838 = f206(v0);
    match m6838 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f297(v0: u64, v1: u64) -> bool {
    let a6839 = v0;
    let a6840 = 1u64;
    if nat_eq(a6839, a6840) {
        true
    } else {
        let v2: u64 = f315(v0);
        let a6841 = 0u64;
        let a6842 = v2;
        if nat_lt(a6841, a6842) {
            let a6846 = {
                let a6843 = v1;
                let a6844 = v2;
                let a6845 = 1u64;
                nat_rem(a6843, a6844, a6845)
            };
            let a6847 = 0u64;
            nat_eq(a6846, a6847)
        } else {
            false
        }
    }
}

pub fn f298(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6848 = v0;
    let a6849 = 28u64;
    if if if if nat_eq(a6848, a6849) {
        let a6850 = v1;
        let a6851 = 0u64;
        nat_eq(a6850, a6851)
    } else {
        false
    } {
        let a6852 = v2;
        let a6853 = 0u64;
        nat_eq(a6852, a6853)
    } else {
        false
    } {
        let a6854 = v3;
        let a6855 = 0u64;
        nat_eq(a6854, a6855)
    } else {
        false
    } {
        let a6856 = v4;
        let a6857 = v8;
        if if if if if if if if nat_eq(a6856, a6857) {
            let a6858 = v5;
            let a6859 = v12;
            nat_eq(a6858, a6859)
        } else {
            false
        } {
            let a6860 = v9;
            let a6861 = v13;
            nat_eq(a6860, a6861)
        } else {
            false
        } {
            let a6862 = v10;
            let a6863 = v14;
            nat_eq(a6862, a6863)
        } else {
            false
        } {
            let a6864 = v11;
            let a6865 = v15;
            nat_eq(a6864, a6865)
        } else {
            false
        } {
            let a6866 = 0u64;
            let a6867 = v6;
            nat_lt(a6866, a6867)
        } else {
            false
        } {
            let a6868 = 0u64;
            let a6869 = v7;
            nat_lt(a6868, a6869)
        } else {
            false
        } {
            let a6873 = {
                let a6870 = v10;
                let a6871 = v6;
                let a6872 = 1u64;
                nat_rem(a6870, a6871, a6872)
            };
            let a6874 = 0u64;
            if nat_eq(a6873, a6874) {
                let a6878 = {
                    let a6875 = v11;
                    let a6876 = v7;
                    let a6877 = 1u64;
                    nat_rem(a6875, a6876, a6877)
                };
                let a6879 = 0u64;
                nat_eq(a6878, a6879)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a6880 = v10;
                let a6881 = v6;
                let a6882 = 0u64;
                nat_quot(a6880, a6881, a6882)
            }, {
                let a6883 = v11;
                let a6884 = v7;
                let a6885 = 0u64;
                nat_quot(a6883, a6884, a6885)
            }, {
                let a6886 = v5;
                let a6887 = v4;
                nat_mul(a6886, a6887)?
            }, {
                let a6888 = v4;
                let a6889 = v9;
                nat_mul(a6888, a6889)?
            }, {
                let a6890 = v5;
                let a6891 = v9;
                nat_mul(a6890, a6891)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f299(v0: u64) -> bool {
    let a6892 = v0;
    let a6893 = 0u64;
    if if nat_eq(a6892, a6893) {
        true
    } else {
        let a6894 = v0;
        let a6895 = 1u64;
        nat_eq(a6894, a6895)
    } {
        true
    } else {
        let a6896 = v0;
        let a6897 = 30u64;
        nat_eq(a6896, a6897)
    }
}

pub fn f300(v0: u64) -> Adt17 {
    f302(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f301(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6899 = v0;
        let a6900 = match v1.clone() {
            Adt20::C0(h6898, _, _, _, _, _, _, _) => {
                h6898
            }
        };
        let a6901 = 0u64;
        nat_quot(a6899, a6900, a6901)
    };
    let v3: u64 = {
        let a6903 = v2;
        let a6904 = match v1.clone() {
            Adt20::C0(_, h6902, _, _, _, _, _, _) => {
                h6902
            }
        };
        let a6905 = 0u64;
        nat_quot(a6903, a6904, a6905)
    };
    f135({
        let a6907 = v0;
        let a6908 = match v1.clone() {
            Adt20::C0(h6906, _, _, _, _, _, _, _) => {
                h6906
            }
        };
        let a6909 = 0u64;
        nat_rem(a6907, a6908, a6909)
    }, {
        let a6911 = v2;
        let a6912 = match v1.clone() {
            Adt20::C0(_, h6910, _, _, _, _, _, _) => {
                h6910
            }
        };
        let a6913 = 0u64;
        nat_rem(a6911, a6912, a6913)
    }, {
        let a6915 = v3;
        let a6916 = match v1.clone() {
            Adt20::C0(_, _, h6914, _, _, _, _, _) => {
                h6914
            }
        };
        let a6917 = 0u64;
        nat_rem(a6915, a6916, a6917)
    }, {
        let a6919 = v3;
        let a6920 = match v1.clone() {
            Adt20::C0(_, _, h6918, _, _, _, _, _) => {
                h6918
            }
        };
        let a6921 = 0u64;
        nat_quot(a6919, a6920, a6921)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6922, _, _, _) => {
            h6922
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6923, _, _) => {
            h6923
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6924, _) => {
            h6924
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6925) => {
            h6925
        }
    })
}

pub fn f302(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a6926 = v0;
    let a6927 = 4u64;
    if if if nat_le(a6926, a6927) {
        true
    } else {
        let a6928 = 9u64;
        let a6929 = v0;
        if nat_le(a6928, a6929) {
            let a6930 = v0;
            let a6931 = 12u64;
            nat_le(a6930, a6931)
        } else {
            false
        }
    } {
        true
    } else {
        let a6932 = v0;
        let a6933 = 16u64;
        if if nat_eq(a6932, a6933) {
            true
        } else {
            let a6934 = v0;
            let a6935 = 19u64;
            nat_eq(a6934, a6935)
        } {
            true
        } else {
            let a6936 = v0;
            let a6937 = 21u64;
            if nat_eq(a6936, a6937) {
                true
            } else {
                let a6938 = v0;
                let a6939 = 22u64;
                nat_eq(a6938, a6939)
            }
        }
    } {
        Adt17::C1({
            let a6940 = v0;
            let a6941 = 4u64;
            if if nat_lt(a6940, a6941) {
                true
            } else {
                let a6942 = 21u64;
                let a6943 = v0;
                nat_le(a6942, a6943)
            } {
                0u64
            } else {
                let a6944 = v0;
                let a6945 = 16u64;
                if if nat_eq(a6944, a6945) {
                    true
                } else {
                    let a6946 = v0;
                    let a6947 = 19u64;
                    nat_eq(a6946, a6947)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6948 = v0;
            let a6949 = 4u64;
            if nat_lt(a6948, a6949) {
                v0
            } else {
                let a6950 = v0;
                let a6951 = 4u64;
                if if nat_eq(a6950, a6951) {
                    true
                } else {
                    let a6952 = v0;
                    let a6953 = 16u64;
                    nat_eq(a6952, a6953)
                } {
                    0u64
                } else {
                    let a6954 = v0;
                    let a6955 = 19u64;
                    if nat_eq(a6954, a6955) {
                        3u64
                    } else {
                        let a6956 = 21u64;
                        let a6957 = v0;
                        if nat_le(a6956, a6957) {
                            let a6958 = v0;
                            let a6959 = 17u64;
                            nat_sub(a6958, a6959)
                        } else {
                            let a6960 = v0;
                            let a6961 = 8u64;
                            nat_sub(a6960, a6961)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f303(v0: u64) -> R<u64> {
    let a6964 = 2u64;
    let a6965 = {
        let a6962 = 6u64;
        let a6963 = v0;
        nat_mul(a6962, a6963)?
    };
    nat_add(a6964, a6965)
}

pub fn f304(v0: u64) -> R<u64> {
    let a6966 = f303(v0)?;
    let a6967 = 11u64;
    nat_add(a6966, a6967)
}

pub fn f305(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a6968 = v0;
        let a6969 = 6u64;
        let a6970 = 0u64;
        nat_rem(a6968, a6969, a6970)
    };
    let v5: u64 = {
        let a6977 = {
            let a6972 = v1;
            let a6973 = match v2.clone() {
                Adt20::C0(h6971, _, _, _, _, _, _, _) => {
                    h6971
                }
            };
            nat_mul(a6972, a6973)?
        };
        let a6978 = {
            let a6974 = v0;
            let a6975 = 6u64;
            let a6976 = 0u64;
            nat_quot(a6974, a6975, a6976)
        };
        nat_add(a6977, a6978)?
    };
    let v6: u64 = f301(v5, v3)?;
    let v7: u64 = f301(v5, v2)?;
    let a6979 = v4;
    let a6980 = 0u64;
    if nat_eq(a6979, a6980) {
        Ok(f302(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6981 = v4;
        let a6982 = 1u64;
        if nat_eq(a6981, a6982) {
            Ok(f302(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6983 = v4;
            let a6984 = 2u64;
            if nat_eq(a6983, a6984) {
                Ok(f302(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6985 = v4;
                let a6986 = 3u64;
                if nat_eq(a6985, a6986) {
                    Ok(f302(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6987 = v4;
                    let a6988 = 4u64;
                    if nat_eq(a6987, a6988) {
                        Ok(f302(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f302(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f306(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6989 = v0;
    let a6990 = 0u64;
    if nat_eq(a6989, a6990) {
        f302(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6991 = v0;
        let a6992 = 1u64;
        if nat_eq(a6991, a6992) {
            f302(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6993 = v0;
            let a6994 = 2u64;
            if nat_eq(a6993, a6994) {
                f302(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6995 = v0;
                let a6996 = 3u64;
                if nat_eq(a6995, a6996) {
                    f302(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6997 = v0;
                    let a6998 = 4u64;
                    if nat_eq(a6997, a6998) {
                        f302(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6999 = v0;
                        let a7000 = 5u64;
                        if nat_eq(a6999, a7000) {
                            f302(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7001 = v0;
                            let a7002 = 6u64;
                            if nat_eq(a7001, a7002) {
                                f302(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7003 = v0;
                                let a7004 = 7u64;
                                if nat_eq(a7003, a7004) {
                                    f302(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7005 = v0;
                                    let a7006 = 8u64;
                                    if nat_eq(a7005, a7006) {
                                        f302(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7007 = v0;
                                        let a7008 = 9u64;
                                        if nat_eq(a7007, a7008) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7009 = v0;
                                            let a7010 = 10u64;
                                            if nat_eq(a7009, a7010) {
                                                f302(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f307(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7011 = v0;
        let a7012 = 3u64;
        let a7013 = 0u64;
        nat_rem(a7011, a7012, a7013)
    };
    let v6: u64 = {
        let a7020 = {
            let a7015 = v1;
            let a7016 = match v2.clone() {
                Adt20::C0(h7014, _, _, _, _, _, _, _) => {
                    h7014
                }
            };
            nat_mul(a7015, a7016)?
        };
        let a7021 = {
            let a7017 = v0;
            let a7018 = 3u64;
            let a7019 = 0u64;
            nat_quot(a7017, a7018, a7019)
        };
        nat_add(a7020, a7021)?
    };
    let v7: u64 = f301(v6, v3)?;
    let v8: u64 = f301(v6, v2)?;
    let v9: u64 = f301(v6, v4)?;
    let a7022 = v5;
    let a7023 = 0u64;
    if nat_eq(a7022, a7023) {
        Ok(f302(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7024 = v5;
        let a7025 = 1u64;
        if nat_eq(a7024, a7025) {
            Ok(f302(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f302(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f308(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7026, _, _, _, _, _, _, _) => {
            h7026
        }
    };
    let v5: u64 = {
        let a7029 = 1u64;
        let a7030 = {
            let a7027 = v4;
            let a7028 = f293(v3);
            nat_mul(a7027, a7028)?
        };
        nat_add(a7029, a7030)?
    };
    let a7031 = v0;
    let a7032 = 0u64;
    if nat_eq(a7031, a7032) {
        Ok(f300(0u64))
    } else {
        let a7033 = v0;
        let a7034 = v5;
        if nat_lt(a7033, a7034) {
            let v6: u64 = {
                let a7035 = v0;
                let a7036 = 1u64;
                nat_sub(a7035, a7036)
            };
            let v7: u64 = f301({
                let a7042 = {
                    let a7037 = v1;
                    let a7038 = v4;
                    nat_mul(a7037, a7038)?
                };
                let a7043 = {
                    let a7039 = v6;
                    let a7040 = f293(v3);
                    let a7041 = 0u64;
                    nat_quot(a7039, a7040, a7041)
                };
                nat_add(a7042, a7043)?
            }, v2)?;
            if v3 {
                let a7047 = {
                    let a7044 = v6;
                    let a7045 = 2u64;
                    let a7046 = 0u64;
                    nat_rem(a7044, a7045, a7046)
                };
                let a7048 = 0u64;
                if nat_eq(a7047, a7048) {
                    Ok(f302(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f302(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f302(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7049 = v0;
                let a7050 = v5;
                nat_sub(a7049, a7050)
            };
            let a7051 = v8;
            let a7052 = 0u64;
            if nat_eq(a7051, a7052) {
                Ok(f302(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7053 = v8;
                    let a7054 = 1u64;
                    nat_eq(a7053, a7054)
                } else {
                    false
                } {
                    Ok(f302(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7055 = v8;
                    let a7056 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7055, a7056) {
                        Ok(f302(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7057 = v8;
                        let a7058 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7057, a7058) {
                            Ok(f300(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f309(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7060 = 13u64;
        let a7061 = f294(match v2.clone() {
            Adt20::C0(h7059, _, _, _, _, _, _, _) => {
                h7059
            }
        });
        nat_mul(a7060, a7061)?
    } else {
        0u64
    };
    let a7062 = v0;
    let a7063 = v5;
    if nat_lt(a7062, a7063) {
        f316({
            let a7064 = v0;
            let a7065 = 13u64;
            let a7066 = 0u64;
            nat_rem(a7064, a7065, a7066)
        }, v1, {
            let a7067 = v0;
            let a7068 = 13u64;
            let a7069 = 0u64;
            nat_quot(a7067, a7068, a7069)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7070 = v0;
            let a7071 = v5;
            nat_sub(a7070, a7071)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f317({
            let a7072 = v6;
            let a7073 = v7;
            let a7074 = 0u64;
            nat_rem(a7072, a7073, a7074)
        }, {
            let a7084 = {
                let a7079 = {
                    let a7076 = v1;
                    let a7077 = match v2.clone() {
                        Adt20::C0(h7075, _, _, _, _, _, _, _) => {
                            h7075
                        }
                    };
                    nat_mul(a7076, a7077)?
                };
                let a7080 = if v4 {
                    f318(match v2.clone() {
                        Adt20::C0(h7078, _, _, _, _, _, _, _) => {
                            h7078
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7079, a7080)?
            };
            let a7085 = {
                let a7081 = v6;
                let a7082 = v7;
                let a7083 = 0u64;
                nat_quot(a7081, a7082, a7083)
            };
            nat_add(a7084, a7085)?
        }, v2, v3, v4)
    }
}

pub fn f310(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7086 = v0;
    let a7087 = 0u64;
    if nat_eq(a7086, a7087) {
        f302(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7088 = v0;
        let a7089 = 1u64;
        if nat_eq(a7088, a7089) {
            f302(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7090 = v0;
                let a7091 = 2u64;
                nat_eq(a7090, a7091)
            } else {
                false
            } {
                f302(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7092 = v0;
                let a7093 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7092, a7093) {
                    f302(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7094 = v0;
                    let a7095 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7094, a7095) {
                        f302(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7096 = v0;
                        let a7097 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7096, a7097) {
                            f302(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f311(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7101 = 1u64;
        let a7102 = {
            let a7099 = match v4.clone() {
                Adt20::C0(h7098, _, _, _, _, _, _, _) => {
                    h7098
                }
            };
            let a7100 = v6;
            nat_mul(a7099, a7100)?
        };
        nat_add(a7101, a7102)?
    };
    let a7103 = v1;
    let a7104 = 0u64;
    if nat_eq(a7103, a7104) {
        Ok(f300(1u64))
    } else {
        let a7105 = v1;
        let a7106 = v7;
        if nat_lt(a7105, a7106) {
            let v8: u64 = {
                let a7107 = v1;
                let a7108 = 1u64;
                nat_sub(a7107, a7108)
            };
            let v9: u64 = {
                let a7115 = {
                    let a7110 = f314(v2, v3, v4.clone())?;
                    let a7111 = match v4.clone() {
                        Adt20::C0(h7109, _, _, _, _, _, _, _) => {
                            h7109
                        }
                    };
                    nat_mul(a7110, a7111)?
                };
                let a7116 = {
                    let a7112 = v8;
                    let a7113 = v6;
                    let a7114 = 0u64;
                    nat_quot(a7112, a7113, a7114)
                };
                nat_add(a7115, a7116)?
            };
            let v10: u64 = f301(v9, v4)?;
            let v11: u64 = f301(v9, v5)?;
            let v12: u64 = {
                let a7117 = v8;
                let a7118 = v6;
                let a7119 = 0u64;
                nat_rem(a7117, a7118, a7119)
            };
            if v0 {
                let a7120 = v12;
                let a7121 = 0u64;
                if nat_eq(a7120, a7121) {
                    Ok(f302(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7122 = v12;
                    let a7123 = 1u64;
                    if nat_eq(a7122, a7123) {
                        Ok(f302(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7124 = v12;
                        let a7125 = 2u64;
                        if nat_eq(a7124, a7125) {
                            Ok(f302(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f302(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7126 = v12;
                let a7127 = 0u64;
                if nat_eq(a7126, a7127) {
                    Ok(f302(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f302(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7128 = v1;
            let a7129 = v7;
            if nat_eq(a7128, a7129) {
                Ok(f302(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f312(v0: u64, v1: u64) -> Adt17 {
    let a7130 = v0;
    let a7131 = 0u64;
    if nat_eq(a7130, a7131) {
        f302(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7132 = v0;
        let a7133 = 1u64;
        if nat_eq(a7132, a7133) {
            f302(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7134 = v0;
            let a7135 = 2u64;
            if nat_eq(a7134, a7135) {
                f302(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7136 = v0;
                let a7137 = 3u64;
                if nat_eq(a7136, a7137) {
                    f300(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f313(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7138 = v0;
    let a7139 = 0u64;
    if nat_eq(a7138, a7139) {
        f302(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7140 = v0;
        let a7141 = 1u64;
        if nat_eq(a7140, a7141) {
            f302(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7142 = v0;
            let a7143 = 2u64;
            if nat_eq(a7142, a7143) {
                f302(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7144 = v0;
                let a7145 = 3u64;
                if nat_eq(a7144, a7145) {
                    f302(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7146 = v0;
                    let a7147 = 4u64;
                    if nat_eq(a7146, a7147) {
                        f302(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7148 = v0;
                        let a7149 = 5u64;
                        if nat_eq(a7148, a7149) {
                            f302(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f314(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7160 = {
        let a7158 = {
            let a7155 = {
                let a7152 = match v1.clone() {
                    Adt21::C0(_, _, h7150, _) => {
                        h7150
                    }
                };
                let a7153 = match v2.clone() {
                    Adt20::C0(_, _, h7151, _, _, _, _, _) => {
                        h7151
                    }
                };
                nat_mul(a7152, a7153)?
            };
            let a7156 = match v1 {
                Adt21::C0(h7154, _, _, _) => {
                    h7154
                }
            };
            nat_add(a7155, a7156)?
        };
        let a7159 = match v2 {
            Adt20::C0(_, h7157, _, _, _, _, _, _) => {
                h7157
            }
        };
        nat_mul(a7158, a7159)?
    };
    let a7161 = v0;
    nat_add(a7160, a7161)
}

pub fn f315(v0: u64) -> u64 {
    if f213(v0) {
        let m7162 = f206(v0);
        match m7162 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7164 = 1u64;
                let a7165 = match v1.clone() {
                    Adt19::C0(_, h7163, _) => {
                        h7163
                    }
                };
                if if nat_lt(a7164, a7165) {
                    true
                } else {
                    let a7166 = v0;
                    let a7167 = 43u64;
                    nat_eq(a7166, a7167)
                } {
                    match v1 {
                        Adt19::C0(_, h7168, _) => {
                            h7168
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

pub fn f316(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7176 = {
            let a7174 = {
                let a7170 = v1;
                let a7171 = match v3.clone() {
                    Adt20::C0(h7169, _, _, _, _, _, _, _) => {
                        h7169
                    }
                };
                nat_mul(a7170, a7171)?
            };
            let a7175 = {
                let a7172 = v2;
                let a7173 = 4u64;
                nat_mul(a7172, a7173)?
            };
            nat_add(a7174, a7175)?
        };
        let a7177 = f319(v0);
        nat_add(a7176, a7177)?
    };
    let v6: u64 = f301(v5, v3)?;
    let v7: u64 = f301(v5, v4)?;
    let a7178 = v0;
    let a7179 = 0u64;
    if if if if nat_eq(a7178, a7179) {
        true
    } else {
        let a7180 = v0;
        let a7181 = 2u64;
        nat_eq(a7180, a7181)
    } {
        true
    } else {
        let a7182 = v0;
        let a7183 = 5u64;
        nat_eq(a7182, a7183)
    } {
        true
    } else {
        let a7184 = v0;
        let a7185 = 7u64;
        nat_eq(a7184, a7185)
    } {
        Ok(f302(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7186 = v0;
        let a7187 = 1u64;
        if if if if nat_eq(a7186, a7187) {
            true
        } else {
            let a7188 = v0;
            let a7189 = 3u64;
            nat_eq(a7188, a7189)
        } {
            true
        } else {
            let a7190 = v0;
            let a7191 = 6u64;
            nat_eq(a7190, a7191)
        } {
            true
        } else {
            let a7192 = v0;
            let a7193 = 8u64;
            nat_eq(a7192, a7193)
        } {
            Ok(f302(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7194 = v0;
                let a7195 = 1u64;
                if nat_eq(a7194, a7195) {
                    4u64
                } else {
                    let a7196 = v0;
                    let a7197 = 6u64;
                    if nat_eq(a7196, a7197) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7198 = v0;
            let a7199 = 4u64;
            if nat_eq(a7198, a7199) {
                Ok(f302(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7200 = v0;
                let a7201 = 9u64;
                if nat_eq(a7200, a7201) {
                    Ok(f302(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7202 = v0;
                    let a7203 = 10u64;
                    if nat_eq(a7202, a7203) {
                        Ok(f302(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7204 = v0;
                        let a7205 = 11u64;
                        if nat_eq(a7204, a7205) {
                            Ok(f302(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7206 = v0;
                            let a7207 = 12u64;
                            if nat_eq(a7206, a7207) {
                                Ok(f302(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f317(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f301(v1, v2)?;
    let v6: u64 = f301(v1, v3)?;
    let a7208 = v0;
    let a7209 = 0u64;
    if nat_eq(a7208, a7209) {
        Ok(f302(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7210 = v0;
        let a7211 = 1u64;
        if nat_eq(a7210, a7211) {
            Ok(f302(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7212 = v0;
                let a7213 = 2u64;
                if nat_eq(a7212, a7213) {
                    Ok(f302(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7214 = v0;
                    let a7215 = 3u64;
                    if nat_eq(a7214, a7215) {
                        Ok(f302(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7216 = v0;
                let a7217 = 2u64;
                if nat_eq(a7216, a7217) {
                    Ok(f302(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f318(v0: u64) -> R<u64> {
    let a7218 = 4u64;
    let a7219 = f294(v0);
    nat_mul(a7218, a7219)
}

pub fn f319(v0: u64) -> u64 {
    let a7220 = v0;
    let a7221 = 5u64;
    if nat_lt(a7220, a7221) {
        let a7222 = v0;
        let a7223 = 2u64;
        if nat_lt(a7222, a7223) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7224 = v0;
        let a7225 = 7u64;
        if nat_lt(a7224, a7225) {
            2u64
        } else {
            3u64
        }
    }
}
