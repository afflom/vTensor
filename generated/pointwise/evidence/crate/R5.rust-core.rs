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
            if if nat_eq(a2, a3) {
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
                let a4 = v1;
                let a5 = 94u64;
                if if nat_eq(a4, a5) {
                    true
                } else {
                    let a6 = v1;
                    let a7 = 95u64;
                    nat_eq(a6, a7)
                } {
                    Ok(Adt9::C0(f2({
                        let a8 = v1;
                        let a9 = 95u64;
                        nat_eq(a8, a9)
                    }, v3, v4, v5, {
                        let m10 = v6;
                        match m10 {
                            None => {
                                let m11 = v7;
                                match m11 {
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
                    let a12 = v1;
                    let a13 = 96u64;
                    if if nat_eq(a12, a13) {
                        true
                    } else {
                        let a14 = v1;
                        let a15 = 97u64;
                        nat_eq(a14, a15)
                    } {
                        Ok(Adt9::C0(f3({
                            let a16 = v1;
                            let a17 = 96u64;
                            nat_eq(a16, a17)
                        }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                    } else {
                        let v15: bool = {
                            let m18 = v7;
                            match m18 {
                                None => {
                                    v8
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        };
                        let a19 = v1;
                        let a20 = 85u64;
                        if nat_eq(a19, a20) {
                            Ok(Adt9::C0(f4(v2, v3, v4, v5, {
                                let m21 = v6;
                                match m21 {
                                    None => {
                                        v15
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v11)?))
                        } else {
                            let a22 = v1;
                            let a23 = 87u64;
                            if nat_eq(a22, a23) {
                                Ok(Adt9::C0(f5(v3, v4, v5, v6, v15, v11)?))
                            } else {
                                let a24 = v1;
                                let a25 = 64u64;
                                if nat_eq(a24, a25) {
                                    Ok(Adt9::C0(f6(v2, v11)?))
                                } else {
                                    let m26 = v3;
                                    match m26 {
                                        None => {
                                            Ok(Adt9::C0(Adt10::C1))
                                        }
                                        Some(v17) => {
                                            let a27 = v1;
                                            let a28 = 48u64;
                                            if if nat_eq(a27, a28) {
                                                true
                                            } else {
                                                let a29 = v1;
                                                let a30 = 56u64;
                                                nat_eq(a29, a30)
                                            } {
                                                Ok(Adt9::C0(f7({
                                                    let a31 = v1;
                                                    let a32 = 56u64;
                                                    nat_eq(a31, a32)
                                                }, v2, v17, v4, v11)?))
                                            } else {
                                                let a33 = v1;
                                                let a34 = 53u64;
                                                if if nat_eq(a33, a34) {
                                                    true
                                                } else {
                                                    let a35 = v1;
                                                    let a36 = 54u64;
                                                    nat_eq(a35, a36)
                                                } {
                                                    Ok(Adt9::C0(f8({
                                                        let a37 = v1;
                                                        let a38 = 54u64;
                                                        nat_eq(a37, a38)
                                                    }, v2, v17, v4, v11)?))
                                                } else {
                                                    let a39 = v1;
                                                    let a40 = 78u64;
                                                    if nat_eq(a39, a40) {
                                                        Ok(Adt9::C0(f9(v2, v17, v4, v5, v11)?))
                                                    } else {
                                                        let a41 = v1;
                                                        let a42 = 23u64;
                                                        if if if nat_eq(a41, a42) {
                                                            true
                                                        } else {
                                                            let a43 = v1;
                                                            let a44 = 26u64;
                                                            nat_eq(a43, a44)
                                                        } {
                                                            true
                                                        } else {
                                                            let a45 = v1;
                                                            let a46 = 25u64;
                                                            nat_eq(a45, a46)
                                                        } {
                                                            Ok(Adt9::C0(f10(v1, v2, v17, v4, v11)?))
                                                        } else {
                                                            let a47 = v1;
                                                            let a48 = 24u64;
                                                            if nat_eq(a47, a48) {
                                                                Ok(Adt9::C0(f11(v2, v17, v11)?))
                                                            } else {
                                                                let a49 = v1;
                                                                let a50 = 27u64;
                                                                if nat_eq(a49, a50) {
                                                                    Ok(Adt9::C0(f12(v2, v17, v11)?))
                                                                } else {
                                                                    let a51 = v1;
                                                                    let a52 = 3u64;
                                                                    if nat_eq(a51, a52) {
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
            let a54 = 0u64;
            let a55 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h53, _, _, _, _, _, _, _) => {
                    h53
                }
            };
            if nat_lt(a54, a55) {
                f15(v18, v19, v20, v21)
            } else {
                f16(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a56 = 3240099840u64;
                let a57 = v22;
                if if nat_le(a56, a57) {
                    let a58 = v22;
                    let a59 = 4286578688u64;
                    nat_le(a58, a59)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a60 = 1092616192u64;
                    let a61 = v22;
                    if if nat_le(a60, a61) {
                        let a62 = v22;
                        let a63 = 2139095040u64;
                        nat_le(a62, a63)
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
    let m64 = v0;
    match m64 {
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
        let m65 = v1.clone();
        match m65 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m66 = v2.clone();
        match m66 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m67 = v1.clone();
        match m67 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m68 = v3.clone();
        match m68 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m69 = v2.clone();
        match m69 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a75 = {
            let a72 = match v10.clone() {
                Adt2::C0(_, _, h70, _, _, _, _, _, _, _, _) => {
                    h70
                }
            };
            let a73 = match v10.clone() {
                Adt2::C0(_, _, _, h71, _, _, _, _, _, _, _) => {
                    h71
                }
            };
            nat_mul(a72, a73)?
        };
        let a76 = match v10.clone() {
            Adt2::C0(_, _, _, _, h74, _, _, _, _, _, _) => {
                h74
            }
        };
        nat_mul(a75, a76)?
    };
    let m77 = v1;
    if if if if if if if if if if match m77 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m78 = v2;
        match m78 {
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
            let m79 = v3;
            match m79 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m80 = v3;
            !match m80 {
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
        let a81 = 0u64;
        let a82 = v14;
        nat_lt(a81, a82)
    } else {
        false
    } {
        let a84 = 0u64;
        let a85 = match v10.clone() {
            Adt2::C0(_, h83, _, _, _, _, _, _, _, _, _) => {
                h83
            }
        };
        nat_lt(a84, a85)
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
            let a87 = match v7.clone() {
                Adt2::C0(h86, _, _, _, _, _, _, _, _, _, _) => {
                    h86
                }
            };
            let a88 = 0u64;
            if if if if nat_eq(a87, a88) {
                let a89 = f23(v7.clone(), 1u64)?;
                let a90 = 1u64;
                nat_eq(a89, a90)
            } else {
                false
            } {
                let a92 = 4u64;
                let a93 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h91, _) => {
                        h91
                    }
                };
                nat_le(a92, a93)
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
            let a95 = match v5.clone() {
                Adt2::C0(h94, _, _, _, _, _, _, _, _, _, _) => {
                    h94
                }
            };
            let a96 = 0u64;
            if if nat_eq(a95, a96) {
                let a97 = f23(v5.clone(), 1u64)?;
                let a98 = 1u64;
                nat_eq(a97, a98)
            } else {
                false
            } {
                let a100 = 4u64;
                let a101 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h99, _) => {
                        h99
                    }
                };
                nat_le(a100, a101)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a107 = {
                let a105 = 2u64;
                let a106 = {
                    let a103 = v14;
                    let a104 = f24(v0, match v10.clone() {
                        Adt2::C0(_, h102, _, _, _, _, _, _, _, _, _) => {
                            h102
                        }
                    })?;
                    nat_mul(a103, a104)?
                };
                nat_add(a105, a106)?
            };
            let a108 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a107, a108)?
        }, 4u64, f25(v10, 1u64), f25(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h109, _, _, _, _) => {
                h109
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m110 = v3.clone();
        match m110 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m111 = v4.clone();
        match m111 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m112 = v5.clone();
        match m112 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m113 = v6.clone();
        match m113 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m114 = v7.clone();
        match m114 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m115 = v5.clone();
        match m115 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m116 = v3;
    if if if if if if if if if if if match m116 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m117 = v4;
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
    } {
        let m118 = v5;
        match m118 {
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
            let m119 = v6;
            if match m119 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m120 = v7;
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
            }
        } else {
            let m121 = v6;
            if !match m121 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m122 = v7;
                !match m122 {
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
            let a129 = {
                let a126 = {
                    let a123 = v1;
                    let a124 = 4u64;
                    let a125 = 0u64;
                    nat_quot(a123, a124, a125)
                };
                let a127 = 2u64;
                let a128 = 0u64;
                nat_rem(a126, a127, a128)
            };
            let a130 = 1u64;
            nat_eq(a129, a130)
        } else {
            false
        } {
            let a132 = match v11.clone() {
                Adt2::C0(h131, _, _, _, _, _, _, _, _, _, _) => {
                    h131
                }
            };
            let a133 = 0u64;
            nat_eq(a132, a133)
        } else {
            false
        } {
            let a135 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h134, _, _, _, _, _) => {
                    h134
                }
            };
            let a136 = 4u64;
            nat_eq(a135, a136)
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
            let a138 = match v11.clone() {
                Adt2::C0(_, h137, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            };
            let a139 = 1u64;
            if nat_le(a138, a139) {
                true
            } else {
                let a142 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                        h140
                    }
                };
                let a143 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h141, _, _, _, _, _) => {
                        h141
                    }
                };
                nat_eq(a142, a143)
            }
        } else {
            false
        } {
            let a145 = match v11.clone() {
                Adt2::C0(_, _, h144, _, _, _, _, _, _, _, _) => {
                    h144
                }
            };
            let a146 = 1u64;
            if nat_le(a145, a146) {
                true
            } else {
                let a149 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h147, _, _, _, _) => {
                        h147
                    }
                };
                let a150 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h148, _, _, _, _) => {
                        h148
                    }
                };
                nat_eq(a149, a150)
            }
        } else {
            false
        } {
            let a152 = match v11.clone() {
                Adt2::C0(_, _, _, h151, _, _, _, _, _, _, _) => {
                    h151
                }
            };
            let a153 = 1u64;
            if nat_le(a152, a153) {
                true
            } else {
                let a156 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h154, _, _, _) => {
                        h154
                    }
                };
                let a157 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h155, _, _, _) => {
                        h155
                    }
                };
                nat_eq(a156, a157)
            }
        } else {
            false
        } {
            let a159 = match v11.clone() {
                Adt2::C0(_, _, _, _, h158, _, _, _, _, _, _) => {
                    h158
                }
            };
            let a160 = 1u64;
            if nat_le(a159, a160) {
                true
            } else {
                let a163 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h161, _, _) => {
                        h161
                    }
                };
                let a164 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h162, _, _) => {
                        h162
                    }
                };
                nat_eq(a163, a164)
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
        let a166 = match v20.clone() {
            Adt2::C0(h165, _, _, _, _, _, _, _, _, _, _) => {
                h165
            }
        };
        let a167 = 0u64;
        nat_eq(a166, a167)
    } else {
        false
    } {
        let a168 = f23(v20.clone(), 1u64)?;
        let a169 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a168, a169)
    } else {
        false
    } {
        let a171 = if v0 {
            28u64
        } else {
            8u64
        };
        let a172 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h170, _) => {
                h170
            }
        };
        nat_le(a171, a172)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a175 = f28(v0);
            let a176 = {
                let a173 = f23(v11.clone(), 1u64)?;
                let a174 = f29(v0);
                nat_mul(a173, a174)?
            };
            nat_add(a175, a176)?
        }, 4u64, f25(v11, 1u64), f25(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m177 = v1.clone();
        match m177 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m178 = v2.clone();
        match m178 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m179 = v3.clone();
        match m179 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h180, _, _, _, _, _, _, _, _, _, _) => {
            h180
        }
    };
    let m181 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m181 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m182 = v2;
        match m182 {
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
        let m183 = v3;
        match m183 {
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
        let a185 = match v7.clone() {
            Adt2::C0(h184, _, _, _, _, _, _, _, _, _, _) => {
                h184
            }
        };
        let a186 = 0u64;
        if if nat_eq(a185, a186) {
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
        let a188 = match v9.clone() {
            Adt2::C0(h187, _, _, _, _, _, _, _, _, _, _) => {
                h187
            }
        };
        let a189 = 0u64;
        if if nat_eq(a188, a189) {
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
        let a191 = match v11.clone() {
            Adt2::C0(h190, _, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        let a192 = 0u64;
        if if nat_eq(a191, a192) {
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
        let a194 = match v7.clone() {
            Adt2::C0(_, h193, _, _, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a195 = 24u64;
        nat_eq(a194, a195)
    } else {
        false
    } {
        let a197 = match v7.clone() {
            Adt2::C0(_, _, _, h196, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = 1u64;
        nat_eq(a197, a198)
    } else {
        false
    } {
        let a200 = match v7.clone() {
            Adt2::C0(_, _, _, _, h199, _, _, _, _, _, _) => {
                h199
            }
        };
        let a201 = 1u64;
        nat_eq(a200, a201)
    } else {
        false
    } {
        let a203 = match v11.clone() {
            Adt2::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                h202
            }
        };
        let a204 = 24u64;
        nat_eq(a203, a204)
    } else {
        false
    } {
        let a206 = match v11.clone() {
            Adt2::C0(_, _, h205, _, _, _, _, _, _, _, _) => {
                h205
            }
        };
        let a207 = 1u64;
        nat_eq(a206, a207)
    } else {
        false
    } {
        let a209 = match v11.clone() {
            Adt2::C0(_, _, _, h208, _, _, _, _, _, _, _) => {
                h208
            }
        };
        let a210 = 1u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a212 = match v11.clone() {
            Adt2::C0(_, _, _, _, h211, _, _, _, _, _, _) => {
                h211
            }
        };
        let a213 = 1u64;
        nat_eq(a212, a213)
    } else {
        false
    } {
        let a215 = match v9.clone() {
            Adt2::C0(_, _, h214, _, _, _, _, _, _, _, _) => {
                h214
            }
        };
        let a216 = 1u64;
        nat_eq(a215, a216)
    } else {
        false
    } {
        let a218 = match v9.clone() {
            Adt2::C0(_, _, _, h217, _, _, _, _, _, _, _) => {
                h217
            }
        };
        let a219 = 1u64;
        nat_eq(a218, a219)
    } else {
        false
    } {
        let a221 = match v9.clone() {
            Adt2::C0(_, _, _, _, h220, _, _, _, _, _, _) => {
                h220
            }
        };
        let a222 = 1u64;
        nat_eq(a221, a222)
    } else {
        false
    } {
        let a224 = 3u64;
        let a225 = match v9.clone() {
            Adt2::C0(_, h223, _, _, _, _, _, _, _, _, _) => {
                h223
            }
        };
        nat_le(a224, a225)
    } else {
        false
    } {
        let a227 = match v5.clone() {
            Adt2::C0(_, h226, _, _, _, _, _, _, _, _, _) => {
                h226
            }
        };
        let a228 = 4u64;
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a230 = match v5.clone() {
            Adt2::C0(_, _, h229, _, _, _, _, _, _, _, _) => {
                h229
            }
        };
        let a231 = 4u64;
        nat_eq(a230, a231)
    } else {
        false
    } {
        let a234 = match v5.clone() {
            Adt2::C0(_, _, _, h232, _, _, _, _, _, _, _) => {
                h232
            }
        };
        let a235 = match v7.clone() {
            Adt2::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        nat_eq(a234, a235)
    } else {
        false
    } {
        let a237 = match v5.clone() {
            Adt2::C0(_, _, _, _, h236, _, _, _, _, _, _) => {
                h236
            }
        };
        let a238 = 1u64;
        nat_eq(a237, a238)
    } else {
        false
    } {
        f31(v5.clone())?
    } else {
        false
    } {
        let a239 = 0u64;
        let a240 = v12;
        nat_lt(a239, a240)
    } else {
        false
    } {
        let a241 = v12;
        let a242 = 2147483648u64;
        nat_lt(a241, a242)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a244 = match v7.clone() {
                Adt2::C0(_, _, h243, _, _, _, _, _, _, _, _) => {
                    h243
                }
            };
            let a245 = f32(v12)?;
            nat_mul(a244, a245)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C38(f25(v9, 1u64), f25(v11, 1u64), match v0 {
            Adt1::C0(h246, _, _, _, _, _, _, _, _, _, _, _) => {
                h246
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m247 = v0.clone();
        match m247 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m248 = v1.clone();
        match m248 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m249 = v2.clone();
        match m249 {
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
        let m250 = v0;
        match m250 {
            None => {
                false
            }
            Some(_) => {
                let m251 = v1;
                match m251 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m252 = v2;
                        match m252 {
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
        let a254 = match v7.clone() {
            Adt2::C0(h253, _, _, _, _, _, _, _, _, _, _) => {
                h253
            }
        };
        let a255 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a254, a255) {
            let a257 = match v7.clone() {
                Adt2::C0(_, _, _, _, h256, _, _, _, _, _, _) => {
                    h256
                }
            };
            let a258 = 1u64;
            nat_eq(a257, a258)
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
            let a260 = match v9.clone() {
                Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                    h259
                }
            };
            let a261 = 0u64;
            if if if nat_eq(a260, a261) {
                let a263 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h262, _, _, _, _, _, _) => {
                        h262
                    }
                };
                let a264 = 1u64;
                nat_eq(a263, a264)
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
            let a266 = match v11.clone() {
                Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                    h265
                }
            };
            let a267 = 0u64;
            if if if nat_eq(a266, a267) {
                let a269 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h268, _, _, _, _, _, _) => {
                        h268
                    }
                };
                let a270 = 1u64;
                nat_eq(a269, a270)
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
            let a272 = match v7.clone() {
                Adt2::C0(_, _, _, h271, _, _, _, _, _, _, _) => {
                    h271
                }
            };
            let a273 = 1u64;
            nat_eq(a272, a273)
        } else {
            false
        } {
            let a275 = match v11.clone() {
                Adt2::C0(_, _, _, h274, _, _, _, _, _, _, _) => {
                    h274
                }
            };
            let a276 = 1u64;
            nat_eq(a275, a276)
        } else {
            false
        } {
            let a279 = match v9.clone() {
                Adt2::C0(_, h277, _, _, _, _, _, _, _, _, _) => {
                    h277
                }
            };
            let a280 = match v7.clone() {
                Adt2::C0(_, h278, _, _, _, _, _, _, _, _, _) => {
                    h278
                }
            };
            nat_eq(a279, a280)
        } else {
            false
        } {
            let a283 = match v9.clone() {
                Adt2::C0(_, _, _, h281, _, _, _, _, _, _, _) => {
                    h281
                }
            };
            let a284 = match v7.clone() {
                Adt2::C0(_, _, h282, _, _, _, _, _, _, _, _) => {
                    h282
                }
            };
            nat_eq(a283, a284)
        } else {
            false
        } {
            let a287 = match v11.clone() {
                Adt2::C0(_, h285, _, _, _, _, _, _, _, _, _) => {
                    h285
                }
            };
            let a288 = match v9.clone() {
                Adt2::C0(_, _, h286, _, _, _, _, _, _, _, _) => {
                    h286
                }
            };
            nat_eq(a287, a288)
        } else {
            false
        } {
            let a291 = match v11.clone() {
                Adt2::C0(_, _, h289, _, _, _, _, _, _, _, _) => {
                    h289
                }
            };
            let a292 = match v7.clone() {
                Adt2::C0(_, _, h290, _, _, _, _, _, _, _, _) => {
                    h290
                }
            };
            nat_eq(a291, a292)
        } else {
            false
        } {
            let a294 = 0u64;
            let a295 = match v9.clone() {
                Adt2::C0(_, _, h293, _, _, _, _, _, _, _, _) => {
                    h293
                }
            };
            if nat_lt(a294, a295) {
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
            let m296 = v3.clone();
            match m296 {
                None => {
                    true
                }
                Some(v15) => {
                    let a298 = match v15.clone() {
                        Adt2::C0(h297, _, _, _, _, _, _, _, _, _, _) => {
                            h297
                        }
                    };
                    let a299 = 0u64;
                    if if if if if if nat_eq(a298, a299) {
                        let a301 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h300, _, _, _, _, _, _) => {
                                h300
                            }
                        };
                        let a302 = 1u64;
                        nat_eq(a301, a302)
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
                        let a305 = match v15.clone() {
                            Adt2::C0(_, h303, _, _, _, _, _, _, _, _, _) => {
                                h303
                            }
                        };
                        let a306 = match v9.clone() {
                            Adt2::C0(_, _, h304, _, _, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        nat_eq(a305, a306)
                    } else {
                        false
                    } {
                        let a309 = match v15.clone() {
                            Adt2::C0(_, _, h307, _, _, _, _, _, _, _, _) => {
                                h307
                            }
                        };
                        let a310 = match v9.clone() {
                            Adt2::C0(_, _, h308, _, _, _, _, _, _, _, _) => {
                                h308
                            }
                        };
                        nat_eq(a309, a310)
                    } else {
                        false
                    } {
                        let a313 = match v15 {
                            Adt2::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                                h311
                            }
                        };
                        let a314 = match v7.clone() {
                            Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                                h312
                            }
                        };
                        nat_eq(a313, a314)
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
            let m315 = v3;
            match m315 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f25(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a317 = f23(v5.clone(), 1u64)?;
            let a318 = f33(match v9.clone() {
                Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                    h316
                }
            }, v17.clone())?;
            nat_mul(a317, a318)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C37(f25(v9, 1u64), f25(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f23(v1.clone(), 1u64)?;
    let a320 = match v1.clone() {
        Adt2::C0(h319, _, _, _, _, _, _, _, _, _, _) => {
            h319
        }
    };
    let a321 = 0u64;
    if if if if if if if if nat_eq(a320, a321) {
        let a323 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h322, _, _, _, _, _) => {
                h322
            }
        };
        let a324 = 4u64;
        nat_eq(a323, a324)
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(h325, _, _, _, _, _, _, _, _, _, _, _) => {
                h325
            }
        })
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(_, h326, _, _, _, _, _, _, _, _, _, _) => {
                h326
            }
        })
    } else {
        false
    } {
        let a328 = 0u64;
        let a329 = match v0.clone() {
            Adt1::C0(_, _, h327, _, _, _, _, _, _, _, _, _) => {
                h327
            }
        };
        nat_lt(a328, a329)
    } else {
        false
    } {
        let a331 = match v0.clone() {
            Adt1::C0(_, _, h330, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        };
        let a332 = 2139095040u64;
        nat_lt(a331, a332)
    } else {
        false
    } {
        let a333 = v2;
        let a334 = 9223372036854775808u64;
        nat_lt(a333, a334)
    } else {
        false
    } {
        let a336 = f35(v2)?;
        let a337 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h335, _) => {
                h335
            }
        };
        nat_le(a336, a337)
    } else {
        false
    } {
        Ok(Adt10::C4(f36(v2)?, 4u64, f25(v1.clone(), 1u64), f25(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h338, _, _, _, _, _, _, _, _, _, _, _) => {
                h338
            }
        }, match v0.clone() {
            Adt1::C0(_, h339, _, _, _, _, _, _, _, _, _, _) => {
                h339
            }
        }, match v0 {
            Adt1::C0(_, _, h340, _, _, _, _, _, _, _, _, _) => {
                h340
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m341 = v3;
    match m341 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f37(v0, v2.clone());
            let v7: Adt3 = f37(v0, v5.clone());
            let v8: Adt3 = f37(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h342, _, _, _, _, _, _, _) => {
                    h342
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h343, _, _, _, _, _, _) => {
                    h343
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h344, _, _, _, _, _) => {
                    h344
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h345, _, _, _, _) => {
                    h345
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h346, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h347, _, _, _, _, _, _) => {
                    h347
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h348, _, _, _, _, _) => {
                    h348
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h349, _, _, _, _, _, _, _, _, _, _, _) => {
                    h349
                }
            };
            let v17: u64 = {
                let a354 = v13;
                let a355 = {
                    let a352 = {
                        let a350 = v9;
                        let a351 = 1u64;
                        nat_sub(a350, a351)
                    };
                    let a353 = v16;
                    nat_mul(a352, a353)?
                };
                nat_add(a354, a355)?
            };
            let v18: u64 = {
                let a360 = v14;
                let a361 = {
                    let a358 = {
                        let a356 = v10;
                        let a357 = 1u64;
                        nat_sub(a356, a357)
                    };
                    let a359 = v16;
                    nat_mul(a358, a359)?
                };
                nat_add(a360, a361)?
            };
            let v19: u64 = {
                let a364 = {
                    let a362 = v9;
                    let a363 = v10;
                    nat_mul(a362, a363)?
                };
                let a365 = v12;
                nat_mul(a364, a365)?
            };
            let v20: u64 = {
                let a368 = {
                    let a366 = v13;
                    let a367 = v14;
                    nat_mul(a366, a367)?
                };
                let a369 = v15;
                nat_mul(a368, a369)?
            };
            let v21: u64 = {
                let a370 = v19;
                let a371 = 32u64;
                if nat_lt(a370, a371) {
                    v19
                } else {
                    32u64
                }
            };
            let a373 = match v2.clone() {
                Adt2::C0(h372, _, _, _, _, _, _, _, _, _, _) => {
                    h372
                }
            };
            let a374 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a373, a374) {
                true
            } else {
                let a376 = match v2.clone() {
                    Adt2::C0(h375, _, _, _, _, _, _, _, _, _, _) => {
                        h375
                    }
                };
                let a377 = 1u64;
                nat_eq(a376, a377)
            } {
                let a379 = match v5.clone() {
                    Adt2::C0(h378, _, _, _, _, _, _, _, _, _, _) => {
                        h378
                    }
                };
                let a380 = 0u64;
                nat_eq(a379, a380)
            } else {
                false
            } {
                let a382 = match v4.clone() {
                    Adt2::C0(h381, _, _, _, _, _, _, _, _, _, _) => {
                        h381
                    }
                };
                let a383 = 0u64;
                nat_eq(a382, a383)
            } else {
                false
            } {
                let a384 = v16;
                let a385 = 2147483648u64;
                nat_lt(a384, a385)
            } else {
                false
            } {
                let a386 = 0u64;
                let a387 = v9;
                nat_lt(a386, a387)
            } else {
                false
            } {
                let a388 = 0u64;
                let a389 = v10;
                nat_lt(a388, a389)
            } else {
                false
            } {
                let a390 = 0u64;
                let a391 = v11;
                nat_lt(a390, a391)
            } else {
                false
            } {
                let a392 = 0u64;
                let a393 = v12;
                nat_lt(a392, a393)
            } else {
                false
            } {
                let a394 = 0u64;
                let a395 = v13;
                nat_lt(a394, a395)
            } else {
                false
            } {
                let a396 = 0u64;
                let a397 = v14;
                nat_lt(a396, a397)
            } else {
                false
            } {
                let a398 = 0u64;
                let a399 = v15;
                nat_lt(a398, a399)
            } else {
                false
            } {
                let a400 = 0u64;
                let a401 = v16;
                nat_lt(a400, a401)
            } else {
                false
            } {
                let a403 = match v6.clone() {
                    Adt3::C0(_, _, _, h402, _, _, _, _) => {
                        h402
                    }
                };
                let a404 = v11;
                nat_eq(a403, a404)
            } else {
                false
            } {
                let a406 = match v8.clone() {
                    Adt3::C0(h405, _, _, _, _, _, _, _) => {
                        h405
                    }
                };
                let a407 = v17;
                nat_eq(a406, a407)
            } else {
                false
            } {
                let a409 = match v8.clone() {
                    Adt3::C0(_, h408, _, _, _, _, _, _) => {
                        h408
                    }
                };
                let a410 = v18;
                nat_eq(a409, a410)
            } else {
                false
            } {
                let a412 = match v8.clone() {
                    Adt3::C0(_, _, h411, _, _, _, _, _) => {
                        h411
                    }
                };
                let a413 = v15;
                nat_eq(a412, a413)
            } else {
                false
            } {
                let a415 = match v8.clone() {
                    Adt3::C0(_, _, _, h414, _, _, _, _) => {
                        h414
                    }
                };
                let a416 = v12;
                nat_eq(a415, a416)
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a421 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h417, _, _, _, _, _) => {
                        h417
                    }
                };
                let a422 = {
                    let a419 = match v2.clone() {
                        Adt2::C0(h418, _, _, _, _, _, _, _, _, _, _) => {
                            h418
                        }
                    };
                    let a420 = 1u64;
                    if nat_eq(a419, a420) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a421, a422)
            } else {
                false
            } {
                let a424 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h423, _, _, _, _, _) => {
                        h423
                    }
                };
                let a425 = 4u64;
                nat_eq(a424, a425)
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
                    let a430 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h426, _, _, _, _) => {
                            h426
                        }
                    };
                    let a431 = {
                        let a428 = v13;
                        let a429 = match v2 {
                            Adt2::C0(_, _, _, _, _, h427, _, _, _, _, _) => {
                                h427
                            }
                        };
                        nat_mul(a428, a429)?
                    };
                    nat_eq(a430, a431)
                } else {
                    let a433 = match v1.clone() {
                        Adt1::C0(_, h432, _, _, _, _, _, _, _, _, _, _) => {
                            h432
                        }
                    };
                    let a434 = 0u64;
                    if if if if if if nat_eq(a433, a434) {
                        let a436 = match v1 {
                            Adt1::C0(_, _, h435, _, _, _, _, _, _, _, _, _) => {
                                h435
                            }
                        };
                        let a437 = 1u64;
                        nat_eq(a436, a437)
                    } else {
                        false
                    } {
                        let a439 = match v2 {
                            Adt2::C0(_, _, _, _, h438, _, _, _, _, _, _) => {
                                h438
                            }
                        };
                        let a440 = 1u64;
                        nat_eq(a439, a440)
                    } else {
                        false
                    } {
                        let a442 = match v5.clone() {
                            Adt2::C0(_, _, _, h441, _, _, _, _, _, _, _) => {
                                h441
                            }
                        };
                        let a443 = 1u64;
                        nat_eq(a442, a443)
                    } else {
                        false
                    } {
                        let a445 = match v5 {
                            Adt2::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                                h444
                            }
                        };
                        let a446 = 1u64;
                        nat_eq(a445, a446)
                    } else {
                        false
                    } {
                        let a448 = match v4.clone() {
                            Adt2::C0(_, _, _, h447, _, _, _, _, _, _, _) => {
                                h447
                            }
                        };
                        let a449 = 1u64;
                        nat_eq(a448, a449)
                    } else {
                        false
                    } {
                        let a451 = match v4 {
                            Adt2::C0(_, _, _, _, h450, _, _, _, _, _, _) => {
                                h450
                            }
                        };
                        let a452 = 1u64;
                        nat_eq(a451, a452)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a453 = v11;
                    let a454 = v20;
                    nat_mul(a453, a454)?
                }, v21, {
                    let a455 = v21;
                    let a456 = v11;
                    nat_mul(a455, a456)?
                }, {
                    let a457 = v21;
                    let a458 = v20;
                    nat_mul(a457, a458)?
                }, {
                    let a463 = {
                        let a459 = v17;
                        let a460 = v18;
                        nat_mul(a459, a460)?
                    };
                    let a464 = {
                        let a461 = v15;
                        let a462 = v12;
                        nat_mul(a461, a462)?
                    };
                    nat_mul(a463, a464)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f8(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m465 = v3;
    match m465 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h466, _, _, _, _, _, _, _, _, _) => {
                    h466
                }
            }, match v1.clone() {
                Adt1::C0(h467, _, _, _, _, _, _, _, _, _, _, _) => {
                    h467
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h468, _, _, _, _, _, _, _, _) => {
                        h468
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h469, _, _, _, _, _, _, _, _, _) => {
                        h469
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h470, _, _, _, _, _) => {
                        h470
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h471, _, _, _, _, _, _, _) => {
                        h471
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h472, _, _, _, _, _, _, _, _) => {
                    h472
                }
            }, match v1.clone() {
                Adt1::C0(_, h473, _, _, _, _, _, _, _, _, _, _) => {
                    h473
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h474, _, _, _, _, _, _, _) => {
                        h474
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h475, _, _, _, _, _, _, _, _) => {
                        h475
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h476, _, _, _, _) => {
                        h476
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h477, _, _, _, _, _, _) => {
                        h477
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h478, _, _, _, _, _, _, _) => {
                        h478
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h479, _, _, _, _, _, _, _, _, _) => {
                        h479
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h480, _, _, _, _, _, _) => {
                        h480
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h481, _, _, _) => {
                        h481
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f39(v6.clone(), match v5.clone() {
                Adt2::C0(_, h482, _, _, _, _, _, _, _, _, _) => {
                    h482
                }
            })?;
            let v10: u64 = f39(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h483, _, _, _, _, _, _, _, _) => {
                    h483
                }
            })?;
            let v11: u64 = if v0 {
                f39(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                        h484
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h485, _, _) => {
                        h485
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h486, _, _, _, _, _, _, _) => {
                        h486
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h487, _) => {
                        h487
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h488, _, _, _, _, _, _) => {
                        h488
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h489) => {
                        h489
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h490, _, _, _, _, _, _) => {
                        h490
                    }
                }
            };
            let v15: u64 = {
                let a495 = {
                    let a491 = v9;
                    let a492 = v10;
                    nat_mul(a491, a492)?
                };
                let a496 = {
                    let a493 = v11;
                    let a494 = v13;
                    nat_mul(a493, a494)?
                };
                nat_mul(a495, a496)?
            };
            let v16: u64 = {
                let a504 = {
                    let a499 = match v6.clone() {
                        Adt6::C0(h497, _, _, _) => {
                            h497
                        }
                    };
                    let a500 = match v7.clone() {
                        Adt6::C0(h498, _, _, _) => {
                            h498
                        }
                    };
                    nat_mul(a499, a500)?
                };
                let a505 = {
                    let a502 = match v8.clone() {
                        Adt6::C0(h501, _, _, _) => {
                            h501
                        }
                    };
                    let a503 = v12;
                    nat_mul(a502, a503)?
                };
                nat_mul(a504, a505)?
            };
            let v17: u64 = {
                let a506 = v15;
                let a507 = 32u64;
                if nat_lt(a506, a507) {
                    v15
                } else {
                    32u64
                }
            };
            let a509 = match v2.clone() {
                Adt2::C0(h508, _, _, _, _, _, _, _, _, _, _) => {
                    h508
                }
            };
            let a510 = 0u64;
            if if if if if if if if if if if nat_eq(a509, a510) {
                true
            } else {
                let a512 = match v2.clone() {
                    Adt2::C0(h511, _, _, _, _, _, _, _, _, _, _) => {
                        h511
                    }
                };
                let a513 = 1u64;
                nat_eq(a512, a513)
            } {
                let a515 = match v5.clone() {
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
                    Adt2::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                        h517
                    }
                };
                let a519 = 0u64;
                nat_eq(a518, a519)
            } else {
                false
            } {
                let a521 = match v4.clone() {
                    Adt2::C0(_, h520, _, _, _, _, _, _, _, _, _) => {
                        h520
                    }
                };
                let a522 = v9;
                nat_eq(a521, a522)
            } else {
                false
            } {
                let a524 = match v4.clone() {
                    Adt2::C0(_, _, h523, _, _, _, _, _, _, _, _) => {
                        h523
                    }
                };
                let a525 = v10;
                nat_eq(a524, a525)
            } else {
                false
            } {
                if v0 {
                    let a526 = 0u64;
                    let a527 = v12;
                    if if if if if if if nat_lt(a526, a527) {
                        let a528 = v12;
                        let a529 = 2147483648u64;
                        nat_lt(a528, a529)
                    } else {
                        false
                    } {
                        let a530 = v14;
                        let a531 = 2147483648u64;
                        nat_lt(a530, a531)
                    } else {
                        false
                    } {
                        let a532 = v13;
                        let a533 = 2147483648u64;
                        nat_lt(a532, a533)
                    } else {
                        false
                    } {
                        let a537 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h534, _, _, _, _, _, _) => {
                                h534
                            }
                        };
                        let a538 = {
                            let a535 = v12;
                            let a536 = v14;
                            nat_mul(a535, a536)?
                        };
                        nat_eq(a537, a538)
                    } else {
                        false
                    } {
                        let a542 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h539, _, _, _, _, _, _) => {
                                h539
                            }
                        };
                        let a543 = {
                            let a540 = v12;
                            let a541 = v13;
                            nat_mul(a540, a541)?
                        };
                        nat_eq(a542, a543)
                    } else {
                        false
                    } {
                        let a545 = match v4.clone() {
                            Adt2::C0(_, _, _, h544, _, _, _, _, _, _, _) => {
                                h544
                            }
                        };
                        let a546 = v11;
                        nat_eq(a545, a546)
                    } else {
                        false
                    } {
                        let a550 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h547, _, _, _, _, _, _) => {
                                h547
                            }
                        };
                        let a551 = {
                            let a548 = v14;
                            let a549 = v13;
                            nat_mul(a548, a549)?
                        };
                        nat_eq(a550, a551)
                    } else {
                        false
                    }
                } else {
                    let a553 = match v2.clone() {
                        Adt2::C0(_, _, _, h552, _, _, _, _, _, _, _) => {
                            h552
                        }
                    };
                    let a554 = v12;
                    if if if nat_eq(a553, a554) {
                        let a556 = match v4.clone() {
                            Adt2::C0(_, _, _, h555, _, _, _, _, _, _, _) => {
                                h555
                            }
                        };
                        let a557 = v14;
                        nat_eq(a556, a557)
                    } else {
                        false
                    } {
                        let a559 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h558, _, _, _, _, _, _) => {
                                h558
                            }
                        };
                        let a560 = v13;
                        nat_eq(a559, a560)
                    } else {
                        false
                    } {
                        let a561 = 0u64;
                        let a562 = v16;
                        nat_lt(a561, a562)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a563 = 0u64;
                let a564 = v14;
                nat_lt(a563, a564)
            } else {
                false
            } {
                let a565 = 0u64;
                let a566 = v15;
                nat_lt(a565, a566)
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
                    let a567 = v16;
                    let a568 = v14;
                    nat_mul(a567, a568)?
                }, v17, {
                    let a569 = v17;
                    let a570 = v16;
                    nat_mul(a569, a570)?
                }, {
                    let a571 = v17;
                    let a572 = v14;
                    nat_mul(a571, a572)?
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
    let m573 = v2;
    match m573 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m574 = v3;
            match m574 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a576 = match v1.clone() {
                        Adt2::C0(h575, _, _, _, _, _, _, _, _, _, _) => {
                            h575
                        }
                    };
                    let a577 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a576, a577) {
                        let a579 = match v5.clone() {
                            Adt2::C0(h578, _, _, _, _, _, _, _, _, _, _) => {
                                h578
                            }
                        };
                        let a580 = 0u64;
                        nat_eq(a579, a580)
                    } else {
                        false
                    } {
                        let a582 = match v6.clone() {
                            Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                                h581
                            }
                        };
                        let a583 = 0u64;
                        nat_eq(a582, a583)
                    } else {
                        false
                    } {
                        let a585 = match v4.clone() {
                            Adt2::C0(h584, _, _, _, _, _, _, _, _, _, _) => {
                                h584
                            }
                        };
                        let a586 = 0u64;
                        nat_eq(a585, a586)
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
                        let a592 = match v1.clone() {
                            Adt2::C0(_, h587, _, _, _, _, _, _, _, _, _) => {
                                h587
                            }
                        };
                        let a593 = {
                            let a590 = match v5.clone() {
                                Adt2::C0(_, h588, _, _, _, _, _, _, _, _, _) => {
                                    h588
                                }
                            };
                            let a591 = match v5.clone() {
                                Adt2::C0(_, h589, _, _, _, _, _, _, _, _, _) => {
                                    h589
                                }
                            };
                            nat_mul(a590, a591)?
                        };
                        nat_eq(a592, a593)
                    } else {
                        false
                    } {
                        let a599 = match v1.clone() {
                            Adt2::C0(_, _, h594, _, _, _, _, _, _, _, _) => {
                                h594
                            }
                        };
                        let a600 = {
                            let a597 = match v5.clone() {
                                Adt2::C0(_, _, h595, _, _, _, _, _, _, _, _) => {
                                    h595
                                }
                            };
                            let a598 = match v5.clone() {
                                Adt2::C0(_, _, _, h596, _, _, _, _, _, _, _) => {
                                    h596
                                }
                            };
                            nat_mul(a597, a598)?
                        };
                        nat_eq(a599, a600)
                    } else {
                        false
                    } {
                        let a603 = match v1.clone() {
                            Adt2::C0(_, _, _, h601, _, _, _, _, _, _, _) => {
                                h601
                            }
                        };
                        let a604 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h602, _, _, _, _, _, _) => {
                                h602
                            }
                        };
                        nat_eq(a603, a604)
                    } else {
                        false
                    } {
                        let a606 = match v0.clone() {
                            Adt1::C0(h605, _, _, _, _, _, _, _, _, _, _, _) => {
                                h605
                            }
                        };
                        let a607 = 4294967296u64;
                        nat_lt(a606, a607)
                    } else {
                        false
                    } {
                        let a609 = match v0.clone() {
                            Adt1::C0(h608, _, _, _, _, _, _, _, _, _, _, _) => {
                                h608
                            }
                        };
                        let a610 = 0u64;
                        if nat_eq(a609, a610) {
                            if f30(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a613 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h611) => {
                                        h611
                                    }
                                };
                                let a614 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h612) => {
                                        h612
                                    }
                                };
                                nat_eq(a613, a614)
                            }
                        } else {
                            let a617 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h615) => {
                                    h615
                                }
                            };
                            let a618 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h616) => {
                                    h616
                                }
                            };
                            nat_eq(a617, a618)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a620 = match v4.clone() {
                                Adt2::C0(_, h619, _, _, _, _, _, _, _, _, _) => {
                                    h619
                                }
                            };
                            let a621 = 0u64;
                            if if if nat_eq(a620, a621) {
                                true
                            } else {
                                let a623 = match v4.clone() {
                                    Adt2::C0(_, _, h622, _, _, _, _, _, _, _, _) => {
                                        h622
                                    }
                                };
                                let a624 = 0u64;
                                nat_eq(a623, a624)
                            } {
                                true
                            } else {
                                let a626 = match v4.clone() {
                                    Adt2::C0(_, _, _, h625, _, _, _, _, _, _, _) => {
                                        h625
                                    }
                                };
                                let a627 = 0u64;
                                nat_eq(a626, a627)
                            } {
                                true
                            } else {
                                let a629 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h628, _, _, _, _, _, _) => {
                                        h628
                                    }
                                };
                                let a630 = 0u64;
                                nat_eq(a629, a630)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h631, _, _, _, _, _, _, _, _, _) => {
                                h631
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a633 = match v0 {
                                Adt1::C0(h632, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h632
                                }
                            };
                            let a634 = 0u64;
                            if nat_eq(a633, a634) {
                                f23(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f40(v8, match v5.clone() {
                                Adt2::C0(_, _, h635, _, _, _, _, _, _, _, _) => {
                                    h635
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h636, _, _, _, _, _, _, _) => {
                                    h636
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h637, _, _, _, _, _, _) => {
                                    h637
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
        let m638 = v3;
        match m638 {
            None => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h639, _, _, _, _, _, _, _, _, _, _, _) => {
                        h639
                    }
                }, match v1 {
                    Adt1::C0(_, h640, _, _, _, _, _, _, _, _, _, _) => {
                        h640
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h641, _, _, _, _, _, _, _, _, _, _, _) => {
                        h641
                    }
                }, match v1 {
                    Adt1::C0(_, h642, _, _, _, _, _, _, _, _, _, _) => {
                        h642
                    }
                }, true, v5, f42(v7), v6)?
            }
        }
    };
    let m643 = v8;
    match m643 {
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
        Adt2::C0(h644, _, _, _, _, _, _, _, _, _, _) => {
            h644
        }
    };
    let v4: u64 = f44(v3);
    let a646 = match v1.clone() {
        Adt2::C0(h645, _, _, _, _, _, _, _, _, _, _) => {
            h645
        }
    };
    let a647 = 0u64;
    if if if if if if if if nat_eq(a646, a647) {
        true
    } else {
        let a649 = match v1.clone() {
            Adt2::C0(h648, _, _, _, _, _, _, _, _, _, _) => {
                h648
            }
        };
        let a650 = 30u64;
        nat_eq(a649, a650)
    } {
        let a653 = match v1.clone() {
            Adt2::C0(h651, _, _, _, _, _, _, _, _, _, _) => {
                h651
            }
        };
        let a654 = match v2.clone() {
            Adt2::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        };
        nat_eq(a653, a654)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f45(match v0.clone() {
            Adt1::C0(h655, _, _, _, _, _, _, _, _, _, _, _) => {
                h655
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
            Adt2::C0(_, h656, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h657, _, _, _, _, _, _, _, _) => {
                h657
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h658, _, _, _, _, _, _, _) => {
                h658
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h659, _, _, _, _, _, _) => {
                h659
            }
        }) {
            0u64
        } else {
            let a665 = match v1.clone() {
                Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                    h660
                }
            };
            let a666 = {
                let a663 = match v1.clone() {
                    Adt2::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
                        h661
                    }
                };
                let a664 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h662, _, _, _, _, _, _) => {
                        h662
                    }
                };
                nat_mul(a663, a664)?
            };
            nat_mul(a665, a666)?
        };
        let v6: u64 = {
            let a667 = v5;
            let a668 = 0u64;
            if if nat_eq(a667, a668) {
                true
            } else {
                f49(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a670 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h669, _) => {
                        h669
                    }
                };
                let a671 = v4;
                let a672 = 0u64;
                nat_quot(a670, a671, a672)
            }
        };
        Ok(Adt10::C4(f50(v3, match v1.clone() {
            Adt2::C0(_, h673, _, _, _, _, _, _, _, _, _) => {
                h673
            }
        }, v5, v6)?, v4, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h674, _, _, _, _, _, _, _, _, _, _, _) => {
                h674
            }
        }, v5, f51(match v1 {
            Adt2::C0(_, h675, _, _, _, _, _, _, _, _, _) => {
                h675
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a677 = match v1.clone() {
        Adt2::C0(h676, _, _, _, _, _, _, _, _, _, _) => {
            h676
        }
    };
    let a678 = 0u64;
    if if if if if nat_eq(a677, a678) {
        let a680 = match v2.clone() {
            Adt2::C0(h679, _, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        };
        let a681 = 0u64;
        nat_eq(a680, a681)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f52(match v0.clone() {
            Adt1::C0(h682, _, _, _, _, _, _, _, _, _, _, _) => {
                h682
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
            let a688 = match v3.clone() {
                Adt3::C0(_, h683, _, _, _, _, _, _) => {
                    h683
                }
            };
            let a689 = {
                let a686 = match v3.clone() {
                    Adt3::C0(_, _, h684, _, _, _, _, _) => {
                        h684
                    }
                };
                let a687 = match v3.clone() {
                    Adt3::C0(_, _, _, h685, _, _, _, _) => {
                        h685
                    }
                };
                nat_mul(a686, a687)?
            };
            nat_mul(a688, a689)?
        };
        Ok(Adt10::C4(f53(match v3.clone() {
            Adt3::C0(h690, _, _, _, _, _, _, _) => {
                h690
            }
        }, v4)?, 4u64, v3, f25(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h691, _, _, _, _, _, _, _, _, _, _, _) => {
                h691
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f13(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m692 = v1;
    match m692 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m693 = v2;
            match m693 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a695 = match v0.clone() {
                        Adt2::C0(h694, _, _, _, _, _, _, _, _, _, _) => {
                            h694
                        }
                    };
                    let a696 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a695, a696) {
                        let a698 = match v4.clone() {
                            Adt2::C0(h697, _, _, _, _, _, _, _, _, _, _) => {
                                h697
                            }
                        };
                        let a699 = 0u64;
                        nat_eq(a698, a699)
                    } else {
                        false
                    } {
                        let a701 = match v5.clone() {
                            Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                                h700
                            }
                        };
                        let a702 = 26u64;
                        nat_eq(a701, a702)
                    } else {
                        false
                    } {
                        let a704 = match v3.clone() {
                            Adt2::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                                h703
                            }
                        };
                        let a705 = 0u64;
                        nat_eq(a704, a705)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a708 = match v0.clone() {
                            Adt2::C0(_, h706, _, _, _, _, _, _, _, _, _) => {
                                h706
                            }
                        };
                        let a709 = match v4.clone() {
                            Adt2::C0(_, h707, _, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        };
                        nat_eq(a708, a709)
                    } else {
                        false
                    } {
                        let a712 = match v0.clone() {
                            Adt2::C0(_, _, h710, _, _, _, _, _, _, _, _) => {
                                h710
                            }
                        };
                        let a713 = match v5.clone() {
                            Adt2::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        nat_eq(a712, a713)
                    } else {
                        false
                    } {
                        let a716 = match v0.clone() {
                            Adt2::C0(_, _, _, h714, _, _, _, _, _, _, _) => {
                                h714
                            }
                        };
                        let a717 = match v5.clone() {
                            Adt2::C0(_, _, h715, _, _, _, _, _, _, _, _) => {
                                h715
                            }
                        };
                        nat_eq(a716, a717)
                    } else {
                        false
                    } {
                        let a719 = 0u64;
                        let a720 = match v5.clone() {
                            Adt2::C0(_, _, _, h718, _, _, _, _, _, _, _) => {
                                h718
                            }
                        };
                        nat_lt(a719, a720)
                    } else {
                        false
                    } {
                        let a722 = 0u64;
                        let a723 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h721, _, _, _, _, _, _) => {
                                h721
                            }
                        };
                        nat_lt(a722, a723)
                    } else {
                        false
                    } {
                        let a725 = 4u64;
                        let a726 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h724, _, _, _, _, _) => {
                                h724
                            }
                        };
                        nat_le(a725, a726)
                    } else {
                        false
                    } {
                        let a728 = 4u64;
                        let a729 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h727, _, _, _, _, _) => {
                                h727
                            }
                        };
                        nat_le(a728, a729)
                    } else {
                        false
                    } {
                        let a731 = 4u64;
                        let a732 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h730, _, _, _, _, _) => {
                                h730
                            }
                        };
                        nat_le(a731, a732)
                    } else {
                        false
                    } {
                        let a734 = 4u64;
                        let a735 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h733, _, _, _, _, _) => {
                                h733
                            }
                        };
                        nat_le(a734, a735)
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
                            Adt2::C0(_, h736, _, _, _, _, _, _, _, _, _) => {
                                h736
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h737, _, _, _, _, _, _, _, _) => {
                                h737
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h738, _, _, _, _, _, _, _) => {
                                h738
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h739, _, _, _, _, _, _) => {
                                h739
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
    let a740 = v0;
    let a741 = 28u64;
    if if if nat_eq(a740, a741) {
        true
    } else {
        let a742 = v0;
        let a743 = 29u64;
        nat_eq(a742, a743)
    } {
        let a745 = 0u64;
        let a746 = match v1.clone() {
            Adt1::C0(h744, _, _, _, _, _, _, _, _, _, _, _) => {
                h744
            }
        };
        if nat_lt(a745, a746) {
            true
        } else {
            let a748 = 0u64;
            let a749 = match v1.clone() {
                Adt1::C0(_, _, _, h747, _, _, _, _, _, _, _, _) => {
                    h747
                }
            };
            nat_lt(a748, a749)
        }
    } else {
        false
    } {
        let m753 = f55(match v2.clone() {
            Adt2::C0(h750, _, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        }, match v1.clone() {
            Adt1::C0(h751, _, _, _, _, _, _, _, _, _, _, _) => {
                h751
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h752, _, _, _, _, _, _, _, _) => {
                h752
            }
        });
        match m753 {
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
        Adt8::C0(_, h754, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h754
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h755, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h755
        }
    };
    let v6: u64 = {
        let a756 = v3;
        let a757 = v5;
        let a758 = 0u64;
        nat_rem(a756, a757, a758)
    };
    let a759 = v1;
    let a760 = 0u64;
    if nat_eq(a759, a760) {
        let a762 = v3;
        let a763 = match v0.clone() {
            Adt8::C0(_, _, _, h761, _, _, _, _, _, _, _, _, _, _, _) => {
                h761
            }
        };
        if nat_lt(a762, a763) {
            Ok(Adt9::C8(Some(f57(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h764, _, _, _, _, _, _) => {
                    h764
                }
            }, {
                let a768 = v6;
                let a769 = {
                    let m767 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h766, _, _, _, _, _, _) => {
                            h766
                        }
                    };
                    match m767 {
                        Adt3::C0(h765, _, _, _, _, _, _, _) => {
                            h765
                        }
                    }
                };
                let a770 = 0u64;
                nat_rem(a768, a769, a770)
            }, {
                let a780 = {
                    let a774 = v6;
                    let a775 = {
                        let m773 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h772, _, _, _, _, _, _) => {
                                h772
                            }
                        };
                        match m773 {
                            Adt3::C0(h771, _, _, _, _, _, _, _) => {
                                h771
                            }
                        }
                    };
                    let a776 = 0u64;
                    nat_quot(a774, a775, a776)
                };
                let a781 = {
                    let m779 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h778, _, _, _, _, _, _) => {
                            h778
                        }
                    };
                    match m779 {
                        Adt3::C0(_, h777, _, _, _, _, _, _) => {
                            h777
                        }
                    }
                };
                let a782 = 0u64;
                nat_rem(a780, a781, a782)
            }, {
                let a792 = {
                    let a786 = v6;
                    let a787 = {
                        let m785 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h784, _, _, _, _, _, _) => {
                                h784
                            }
                        };
                        match m785 {
                            Adt3::C0(h783, _, _, _, _, _, _, _) => {
                                h783
                            }
                        }
                    };
                    let a788 = 0u64;
                    nat_quot(a786, a787, a788)
                };
                let a793 = {
                    let m791 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h790, _, _, _, _, _, _) => {
                            h790
                        }
                    };
                    match m791 {
                        Adt3::C0(_, h789, _, _, _, _, _, _) => {
                            h789
                        }
                    }
                };
                let a794 = 0u64;
                nat_quot(a792, a793, a794)
            }, {
                let a795 = v3;
                let a796 = v5;
                let a797 = 0u64;
                nat_quot(a795, a796, a797)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a803 = v2;
            let a804 = {
                let a800 = v3;
                let a801 = {
                    let a798 = v1;
                    let a799 = 1u64;
                    if nat_eq(a798, a799) {
                        v4
                    } else {
                        v5
                    }
                };
                let a802 = 0u64;
                nat_quot(a800, a801, a802)
            };
            nat_add(a803, a804)?
        };
        let a806 = v7;
        let a807 = match v0.clone() {
            Adt8::C0(h805, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h805
            }
        };
        if nat_lt(a806, a807) {
            let v8: u64 = {
                let a811 = v7;
                let a812 = {
                    let m810 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h809, _, _, _, _, _) => {
                            h809
                        }
                    };
                    match m810 {
                        Adt3::C0(h808, _, _, _, _, _, _, _) => {
                            h808
                        }
                    }
                };
                let a813 = 0u64;
                nat_rem(a811, a812, a813)
            };
            let v9: u64 = {
                let a817 = v7;
                let a818 = {
                    let m816 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h815, _, _, _, _, _) => {
                            h815
                        }
                    };
                    match m816 {
                        Adt3::C0(h814, _, _, _, _, _, _, _) => {
                            h814
                        }
                    }
                };
                let a819 = 0u64;
                nat_quot(a817, a818, a819)
            };
            let v10: u64 = {
                let a823 = v9;
                let a824 = {
                    let m822 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h821, _, _, _, _, _) => {
                            h821
                        }
                    };
                    match m822 {
                        Adt3::C0(_, h820, _, _, _, _, _, _) => {
                            h820
                        }
                    }
                };
                let a825 = 0u64;
                nat_rem(a823, a824, a825)
            };
            let v11: u64 = {
                let a829 = v9;
                let a830 = {
                    let m828 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h827, _, _, _, _, _) => {
                            h827
                        }
                    };
                    match m828 {
                        Adt3::C0(_, h826, _, _, _, _, _, _) => {
                            h826
                        }
                    }
                };
                let a831 = 0u64;
                nat_quot(a829, a830, a831)
            };
            let a832 = v1;
            let a833 = 1u64;
            if nat_eq(a832, a833) {
                Ok(Adt9::C8(Some(f57(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h834, _, _, _, _, _) => {
                        h834
                    }
                }, v8, v10, {
                    let a835 = v3;
                    let a836 = v4;
                    let a837 = 0u64;
                    nat_rem(a835, a836, a837)
                }, v11)?)))
            } else {
                let a838 = v1;
                let a839 = 2u64;
                if nat_eq(a838, a839) {
                    Ok(Adt9::C8(Some({
                        let a885 = f57(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h840, _, _, _, _) => {
                                h840
                            }
                        }, {
                            let a852 = {
                                let a844 = v8;
                                let a845 = {
                                    let m843 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h842, _, _, _) => {
                                            h842
                                        }
                                    };
                                    match m843 {
                                        Adt6::C0(_, h841, _, _) => {
                                            h841
                                        }
                                    }
                                };
                                nat_mul(a844, a845)?
                            };
                            let a853 = {
                                let a849 = v6;
                                let a850 = {
                                    let m848 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h847, _, _, _, _, _, _) => {
                                            h847
                                        }
                                    };
                                    match m848 {
                                        Adt3::C0(h846, _, _, _, _, _, _, _) => {
                                            h846
                                        }
                                    }
                                };
                                let a851 = 0u64;
                                nat_rem(a849, a850, a851)
                            };
                            nat_add(a852, a853)?
                        }, {
                            let a871 = {
                                let a857 = v10;
                                let a858 = {
                                    let m856 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h855, _, _, _) => {
                                            h855
                                        }
                                    };
                                    match m856 {
                                        Adt6::C0(_, h854, _, _) => {
                                            h854
                                        }
                                    }
                                };
                                nat_mul(a857, a858)?
                            };
                            let a872 = {
                                let a868 = {
                                    let a862 = v6;
                                    let a863 = {
                                        let m861 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h860, _, _, _, _, _, _) => {
                                                h860
                                            }
                                        };
                                        match m861 {
                                            Adt3::C0(h859, _, _, _, _, _, _, _) => {
                                                h859
                                            }
                                        }
                                    };
                                    let a864 = 0u64;
                                    nat_quot(a862, a863, a864)
                                };
                                let a869 = {
                                    let m867 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h866, _, _, _, _, _, _) => {
                                            h866
                                        }
                                    };
                                    match m867 {
                                        Adt3::C0(_, h865, _, _, _, _, _, _) => {
                                            h865
                                        }
                                    }
                                };
                                let a870 = 0u64;
                                nat_rem(a868, a869, a870)
                            };
                            nat_add(a871, a872)?
                        }, {
                            let a882 = {
                                let a876 = v6;
                                let a877 = {
                                    let m875 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h874, _, _, _, _, _, _) => {
                                            h874
                                        }
                                    };
                                    match m875 {
                                        Adt3::C0(h873, _, _, _, _, _, _, _) => {
                                            h873
                                        }
                                    }
                                };
                                let a878 = 0u64;
                                nat_quot(a876, a877, a878)
                            };
                            let a883 = {
                                let m881 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h880, _, _, _, _, _, _) => {
                                        h880
                                    }
                                };
                                match m881 {
                                    Adt3::C0(_, h879, _, _, _, _, _, _) => {
                                        h879
                                    }
                                }
                            };
                            let a884 = 0u64;
                            nat_quot(a882, a883, a884)
                        }, v11)?;
                        let a886 = 4u64;
                        let a887 = 0u64;
                        nat_quot(a885, a886, a887)
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
    let a888 = v1;
    let a889 = 0u64;
    if nat_eq(a888, a889) {
        let a891 = v3;
        let a892 = match v0.clone() {
            Adt8::C0(_, _, _, h890, _, _, _, _, _, _, _, _, _, _, _) => {
                h890
            }
        };
        if nat_lt(a891, a892) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h893, _, _, _, _, _, _) => {
                    h893
                }
            };
            let v5: u64 = {
                let a895 = v3;
                let a896 = match v0.clone() {
                    Adt8::C0(_, _, h894, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h894
                    }
                };
                let a897 = 0u64;
                nat_quot(a895, a896, a897)
            };
            Ok(Adt9::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), f59(v0.clone(), v5, {
                let a899 = v3;
                let a900 = match v0 {
                    Adt8::C0(_, _, h898, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h898
                    }
                };
                let a901 = 0u64;
                nat_rem(a899, a900, a901)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h902, _, _, _, _) => {
                h902
            }
        };
        let v7: u64 = {
            let a903 = v1;
            let a904 = 1u64;
            if nat_eq(a903, a904) {
                match v0.clone() {
                    Adt8::C0(_, h905, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h905
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h906, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h906
                    }
                }
            }
        };
        let v8: u64 = {
            let a910 = v2;
            let a911 = {
                let a907 = v3;
                let a908 = v7;
                let a909 = 0u64;
                nat_quot(a907, a908, a909)
            };
            nat_add(a910, a911)?
        };
        let v9: u64 = {
            let a912 = v3;
            let a913 = v7;
            let a914 = 0u64;
            nat_rem(a912, a913, a914)
        };
        let v10: u64 = {
            let a916 = v8;
            let a917 = match v6.clone() {
                Adt3::C0(h915, _, _, _, _, _, _, _) => {
                    h915
                }
            };
            let a918 = 0u64;
            nat_rem(a916, a917, a918)
        };
        let v11: u64 = {
            let a920 = v8;
            let a921 = match v6.clone() {
                Adt3::C0(h919, _, _, _, _, _, _, _) => {
                    h919
                }
            };
            let a922 = 0u64;
            nat_quot(a920, a921, a922)
        };
        let v12: u64 = {
            let a924 = v11;
            let a925 = match v6.clone() {
                Adt3::C0(_, h923, _, _, _, _, _, _) => {
                    h923
                }
            };
            let a926 = 0u64;
            nat_rem(a924, a925, a926)
        };
        let v13: u64 = {
            let a928 = v11;
            let a929 = match v6.clone() {
                Adt3::C0(_, h927, _, _, _, _, _, _) => {
                    h927
                }
            };
            let a930 = 0u64;
            nat_quot(a928, a929, a930)
        };
        let a932 = v8;
        let a933 = match v0.clone() {
            Adt8::C0(h931, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h931
            }
        };
        if nat_lt(a932, a933) {
            let a934 = v1;
            let a935 = 1u64;
            if nat_eq(a934, a935) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h936, _, _, _, _, _, _) => {
                        h936
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h937, _, _, _, _, _) => {
                        h937
                    }
                };
                let m940 = f60(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h938, _, _, _) => {
                        h938
                    }
                }, match v15.clone() {
                    Adt3::C0(h939, _, _, _, _, _, _, _) => {
                        h939
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m940 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m943 = f60(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h941, _, _) => {
                                h941
                            }
                        }, match v15 {
                            Adt3::C0(_, h942, _, _, _, _, _, _) => {
                                h942
                            }
                        }, v12, f58(v9, v14, 1u64))?;
                        match m943 {
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
                let a944 = v1;
                let a945 = 2u64;
                if nat_eq(a944, a945) {
                    Ok(Adt9::C8(Some({
                        let a947 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h946) => {
                                h946
                            }
                        };
                        let a948 = 0u64;
                        if nat_eq(a947, a948) {
                            f57(v6, v10, v12, v9, v13)?
                        } else {
                            f57(v6.clone(), v10, v12, {
                                let a950 = v13;
                                let a951 = match v6.clone() {
                                    Adt3::C0(_, _, h949, _, _, _, _, _) => {
                                        h949
                                    }
                                };
                                let a952 = 0u64;
                                nat_rem(a950, a951, a952)
                            }, {
                                let a960 = {
                                    let a958 = {
                                        let a954 = v13;
                                        let a955 = match v6 {
                                            Adt3::C0(_, _, h953, _, _, _, _, _) => {
                                                h953
                                            }
                                        };
                                        let a956 = 0u64;
                                        nat_quot(a954, a955, a956)
                                    };
                                    let a959 = match v0 {
                                        Adt8::C0(_, _, h957, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h957
                                        }
                                    };
                                    nat_mul(a958, a959)?
                                };
                                let a961 = v9;
                                nat_add(a960, a961)?
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
        let m962 = v3.clone();
        match m962 {
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
                f75(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C19(v72, v73) => {
                f76(v72, v0, v1, v2, v73, v4)?
            }
            Adt4::C20(v74, v75) => {
                f77(v74, v0, v1, v2, v75, v4)?
            }
            Adt4::C21 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C22(v76, v77) => {
                f78(v0, v1, v2, v76, v77, v4)?
            }
            Adt4::C23(v91, v92, v93, v94) => {
                f85(v0, v94, v91, v92, v93, v4)?
            }
            Adt4::C24(v90) => {
                if v5 {
                    f83(Adt16::C0(v90), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C26(v86, v87, v88, v89) => {
                let a964 = v86;
                let a965 = 64u64;
                if if if nat_eq(a964, a965) {
                    true
                } else {
                    let a966 = v86;
                    let a967 = 65u64;
                    nat_eq(a966, a967)
                } {
                    true
                } else {
                    let a968 = v86;
                    let a969 = 66u64;
                    nat_eq(a968, a969)
                } {
                    if v5 {
                        f83(Adt16::C1(v86, v87, v88, v89), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a970 = v86;
                    let a971 = 0u64;
                    if if nat_eq(a970, a971) {
                        true
                    } else {
                        let a972 = v86;
                        let a973 = 30u64;
                        nat_eq(a972, a973)
                    } {
                        f84(v0, v1, v2, v86, v87, v88, v89, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v85) => {
                f82(v0, v1, v2, v85)?
            }
            Adt4::C28(v81) => {
                f79(v0, v1, v2, v81, v4)?
            }
            Adt4::C29(v82) => {
                f80(v0, v1, v2, v82)?
            }
            Adt4::C30(v83, _) => {
                f81(v0, v1, v2, v83)?
            }
            Adt4::C31(v33, v34) => {
                f64(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f65(v0, match v1 {
                    Adt3::C0(h963, _, _, _, _, _, _, _) => {
                        h963
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f66(v0, v1, v36, v2)?
            }
            Adt4::C34(v52, v53, v54, v55) => {
                f74(v0, v1, v52, v2, v53, v54, v55)?
            }
            Adt4::C35(v51) => {
                f73(v0, v51, v1, v2)?
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
                f72(v0, v1, v2, v4)?
            }
        }
    }))
}

pub fn f19(v0: Adt4) -> R<Adt9> {
    let m974 = v0;
    match m974 {
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
                let a980 = {
                    let a976 = 0u64;
                    let a977 = v69;
                    if nat_lt(a976, a977) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a981 = {
                    let a978 = 0u64;
                    let a979 = v71;
                    if nat_lt(a978, a979) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a980, a981)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v77, _, _, v80) => {
            Ok(Adt9::C6({
                let a982 = v77;
                let a983 = 66u64;
                if nat_eq(a982, a983) {
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
                let m975 = v32;
                match m975 {
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
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a984 = v1;
    let a985 = 5u64;
    if if if if if nat_lt(a984, a985) {
        true
    } else {
        f86(v0, v1, v2, v3, v4)
    } {
        let a986 = v5;
        let a987 = 5u64;
        nat_lt(a986, a987)
    } else {
        false
    } {
        let a988 = v7;
        let a989 = 5u64;
        nat_lt(a988, a989)
    } else {
        false
    } {
        let a990 = v9;
        let a991 = 2u64;
        if nat_lt(a990, a991) {
            true
        } else {
            let a992 = v9;
            let a993 = 3u64;
            nat_eq(a992, a993)
        }
    } else {
        false
    } {
        Adt9::C7(f87(v0, v1, v2, v3, v4), f88(v5, v6), f88(v7, v8), f89({
            let a994 = v9;
            let a995 = 0u64;
            if nat_eq(a994, a995) {
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
    let a998 = match v0.clone() {
        Adt2::C0(_, h996, _, _, _, _, _, _, _, _, _) => {
            h996
        }
    };
    let a999 = match v1.clone() {
        Adt2::C0(_, h997, _, _, _, _, _, _, _, _, _) => {
            h997
        }
    };
    if if if nat_eq(a998, a999) {
        let a1002 = match v0.clone() {
            Adt2::C0(_, _, h1000, _, _, _, _, _, _, _, _) => {
                h1000
            }
        };
        let a1003 = match v1.clone() {
            Adt2::C0(_, _, h1001, _, _, _, _, _, _, _, _) => {
                h1001
            }
        };
        nat_eq(a1002, a1003)
    } else {
        false
    } {
        let a1006 = match v0.clone() {
            Adt2::C0(_, _, _, h1004, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = match v1.clone() {
            Adt2::C0(_, _, _, h1005, _, _, _, _, _, _, _) => {
                h1005
            }
        };
        nat_eq(a1006, a1007)
    } else {
        false
    } {
        let a1010 = match v0 {
            Adt2::C0(_, _, _, _, h1008, _, _, _, _, _, _) => {
                h1008
            }
        };
        let a1011 = match v1 {
            Adt2::C0(_, _, _, _, h1009, _, _, _, _, _, _) => {
                h1009
            }
        };
        nat_eq(a1010, a1011)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1013 = match v0.clone() {
        Adt2::C0(h1012, _, _, _, _, _, _, _, _, _, _) => {
            h1012
        }
    };
    let a1014 = 0u64;
    if if if nat_eq(a1013, a1014) {
        let a1016 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1015, _, _, _, _, _) => {
                h1015
            }
        };
        let a1017 = 4u64;
        nat_eq(a1016, a1017)
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
        let a1029 = {
            let a1026 = {
                let a1021 = {
                    let a1018 = v1;
                    let a1019 = 1u64;
                    nat_sub(a1018, a1019)
                };
                let a1022 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1020, _, _, _, _) => {
                        h1020
                    }
                };
                nat_mul(a1021, a1022)?
            };
            let a1027 = {
                let a1024 = 4u64;
                let a1025 = match v0.clone() {
                    Adt2::C0(_, h1023, _, _, _, _, _, _, _, _, _) => {
                        h1023
                    }
                };
                nat_mul(a1024, a1025)?
            };
            nat_add(a1026, a1027)?
        };
        let a1030 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1028, _) => {
                h1028
            }
        };
        Ok(nat_le(a1029, a1030))
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a1042 = {
        let a1036 = {
            let a1032 = match v0.clone() {
                Adt2::C0(_, h1031, _, _, _, _, _, _, _, _, _) => {
                    h1031
                }
            };
            let a1033 = v1;
            let a1034 = 0u64;
            nat_quot(a1032, a1033, a1034)
        };
        let a1037 = match v0.clone() {
            Adt2::C0(_, _, h1035, _, _, _, _, _, _, _, _) => {
                h1035
            }
        };
        nat_mul(a1036, a1037)?
    };
    let a1043 = {
        let a1040 = match v0.clone() {
            Adt2::C0(_, _, _, h1038, _, _, _, _, _, _, _) => {
                h1038
            }
        };
        let a1041 = match v0 {
            Adt2::C0(_, _, _, _, h1039, _, _, _, _, _, _) => {
                h1039
            }
        };
        nat_mul(a1040, a1041)?
    };
    nat_mul(a1042, a1043)
}

pub fn f24(v0: bool, v1: u64) -> R<u64> {
    let a1046 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1047 = {
        let a1044 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1045 = v1;
        nat_mul(a1044, a1045)?
    };
    nat_add(a1046, a1047)
}

pub fn f25(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1049 = match v0.clone() {
            Adt2::C0(_, h1048, _, _, _, _, _, _, _, _, _) => {
                h1048
            }
        };
        let a1050 = v1;
        let a1051 = 0u64;
        nat_quot(a1049, a1050, a1051)
    }, match v0.clone() {
        Adt2::C0(_, _, h1052, _, _, _, _, _, _, _, _) => {
            h1052
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1053, _, _, _, _, _, _, _) => {
            h1053
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1054, _, _, _, _, _, _) => {
            h1054
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1055, _, _, _, _, _) => {
            h1055
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1056, _, _, _, _) => {
            h1056
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1057, _, _, _) => {
            h1057
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1058, _, _) => {
            h1058
        }
    })
}

pub fn f26(v0: Adt2, v1: u64) -> R<bool> {
    let a1059 = v1;
    let a1060 = 0u64;
    if nat_eq(a1059, a1060) {
        Ok(true)
    } else {
        let a1066 = {
            let a1063 = f90({
                let a1061 = v1;
                let a1062 = 1u64;
                nat_sub(a1061, a1062)
            }, f25(v0.clone(), 1u64))?;
            let a1064 = 4u64;
            nat_add(a1063, a1064)?
        };
        let a1067 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1065, _) => {
                h1065
            }
        };
        Ok(nat_le(a1066, a1067))
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1069 = match v1.clone() {
        Adt2::C0(h1068, _, _, _, _, _, _, _, _, _, _) => {
            h1068
        }
    };
    let a1070 = 0u64;
    if if nat_eq(a1069, a1070) {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1071 = f23(v0.clone(), 1u64)?;
        let a1072 = 0u64;
        if nat_eq(a1071, a1072) {
            Ok(true)
        } else {
            let a1078 = {
                let a1075 = f90({
                    let a1073 = f23(v0.clone(), 1u64)?;
                    let a1074 = 1u64;
                    nat_sub(a1073, a1074)
                }, f25(v0, 1u64))?;
                let a1076 = 4u64;
                nat_add(a1075, a1076)?
            };
            let a1079 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1077, _) => {
                    h1077
                }
            };
            Ok(nat_le(a1078, a1079))
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
    let a1081 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1080, _) => {
            h1080
        }
    };
    let a1082 = 0u64;
    if nat_eq(a1081, a1082) {
        Ok(true)
    } else {
        let a1084 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1083, _) => {
                h1083
            }
        };
        let a1085 = 0u64;
        if nat_eq(a1084, a1085) {
            Ok(true)
        } else {
            let a1087 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1086) => {
                    h1086
                }
            };
            let a1088 = 0u64;
            if nat_eq(a1087, a1088) {
                Ok(true)
            } else {
                let a1090 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1089) => {
                        h1089
                    }
                };
                let a1091 = 0u64;
                if nat_eq(a1090, a1091) {
                    Ok(true)
                } else {
                    let a1097 = {
                        let a1094 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1092) => {
                                h1092
                            }
                        };
                        let a1095 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1093, _) => {
                                h1093
                            }
                        };
                        nat_add(a1094, a1095)?
                    };
                    let a1098 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1096) => {
                            h1096
                        }
                    };
                    if nat_le(a1097, a1098) {
                        Ok(true)
                    } else {
                        let a1104 = {
                            let a1101 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1099) => {
                                    h1099
                                }
                            };
                            let a1102 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1100, _) => {
                                    h1100
                                }
                            };
                            nat_add(a1101, a1102)?
                        };
                        let a1105 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1103) => {
                                h1103
                            }
                        };
                        Ok(nat_le(a1104, a1105))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1111 = match v0.clone() {
            Adt2::C0(_, _, h1106, _, _, _, _, _, _, _, _) => {
                h1106
            }
        };
        let a1112 = {
            let a1109 = match v0.clone() {
                Adt2::C0(_, _, _, h1107, _, _, _, _, _, _, _) => {
                    h1107
                }
            };
            let a1110 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1108, _, _, _, _, _, _) => {
                    h1108
                }
            };
            nat_mul(a1109, a1110)?
        };
        nat_mul(a1111, a1112)?
    };
    let v2: u64 = f23(v0.clone(), 1u64)?;
    let a1114 = match v0.clone() {
        Adt2::C0(h1113, _, _, _, _, _, _, _, _, _, _) => {
            h1113
        }
    };
    let a1115 = 0u64;
    if if if if if if if nat_eq(a1114, a1115) {
        let a1117 = match v0.clone() {
            Adt2::C0(_, h1116, _, _, _, _, _, _, _, _, _) => {
                h1116
            }
        };
        let a1118 = 2147483647u64;
        nat_le(a1117, a1118)
    } else {
        false
    } {
        let a1119 = v1;
        let a1120 = 2147483647u64;
        nat_le(a1119, a1120)
    } else {
        false
    } {
        let a1122 = match v0.clone() {
            Adt2::C0(_, h1121, _, _, _, _, _, _, _, _, _) => {
                h1121
            }
        };
        let a1123 = 1u64;
        if nat_eq(a1122, a1123) {
            true
        } else {
            let a1125 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1124, _, _, _, _, _) => {
                    h1124
                }
            };
            let a1126 = 4u64;
            nat_eq(a1125, a1126)
        }
    } else {
        false
    } {
        let a1127 = v1;
        let a1128 = 1u64;
        if nat_le(a1127, a1128) {
            true
        } else {
            let a1133 = {
                let a1130 = match v0.clone() {
                    Adt2::C0(_, h1129, _, _, _, _, _, _, _, _, _) => {
                        h1129
                    }
                };
                let a1131 = 4u64;
                nat_mul(a1130, a1131)?
            };
            let a1134 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1132, _, _, _, _) => {
                    h1132
                }
            };
            nat_le(a1133, a1134)
        }
    } else {
        false
    } {
        let a1136 = match v0.clone() {
            Adt2::C0(_, _, _, h1135, _, _, _, _, _, _, _) => {
                h1135
            }
        };
        let a1137 = 1u64;
        if nat_eq(a1136, a1137) {
            true
        } else {
            let a1143 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1138, _, _, _) => {
                    h1138
                }
            };
            let a1144 = {
                let a1141 = match v0.clone() {
                    Adt2::C0(_, _, h1139, _, _, _, _, _, _, _, _) => {
                        h1139
                    }
                };
                let a1142 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1140, _, _, _, _) => {
                        h1140
                    }
                };
                nat_mul(a1141, a1142)?
            };
            nat_eq(a1143, a1144)
        }
    } else {
        false
    } {
        let a1146 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1145, _, _, _, _, _, _) => {
                h1145
            }
        };
        let a1147 = 1u64;
        if nat_eq(a1146, a1147) {
            true
        } else {
            let a1156 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1148, _, _) => {
                    h1148
                }
            };
            let a1157 = {
                let a1154 = match v0.clone() {
                    Adt2::C0(_, _, _, h1149, _, _, _, _, _, _, _) => {
                        h1149
                    }
                };
                let a1155 = {
                    let a1152 = match v0.clone() {
                        Adt2::C0(_, _, h1150, _, _, _, _, _, _, _, _) => {
                            h1150
                        }
                    };
                    let a1153 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1151, _, _, _, _) => {
                            h1151
                        }
                    };
                    nat_mul(a1152, a1153)?
                };
                nat_mul(a1154, a1155)?
            };
            nat_eq(a1156, a1157)
        }
    } else {
        false
    } {
        let a1158 = v2;
        let a1159 = 0u64;
        if nat_eq(a1158, a1159) {
            Ok(true)
        } else {
            let a1165 = {
                let a1162 = f90({
                    let a1160 = v2;
                    let a1161 = 1u64;
                    nat_sub(a1160, a1161)
                }, f25(v0.clone(), 1u64))?;
                let a1163 = 4u64;
                nat_add(a1162, a1163)?
            };
            let a1166 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1164, _) => {
                    h1164
                }
            };
            Ok(nat_le(a1165, a1166))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1173 = 148u64;
    let a1174 = {
        let a1171 = 36u64;
        let a1172 = {
            let a1169 = {
                let a1167 = 2u64;
                let a1168 = v0;
                nat_mul(a1167, a1168)?
            };
            let a1170 = 1u64;
            nat_sub(a1169, a1170)
        };
        nat_mul(a1171, a1172)?
    };
    nat_add(a1173, a1174)
}

pub fn f33(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1175 = v1;
    match m1175 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1178 = 1u64;
            let a1179 = {
                let a1176 = 2u64;
                let a1177 = v0;
                nat_mul(a1176, a1177)?
            };
            nat_add(a1178, a1179)
        }
    }
}

pub fn f34(v0: u64) -> bool {
    let a1180 = v0;
    let a1181 = 4294967296u64;
    if nat_lt(a1180, a1181) {
        let a1185 = {
            let a1182 = v0;
            let a1183 = 2147483648u64;
            let a1184 = 0u64;
            nat_rem(a1182, a1183, a1184)
        };
        let a1186 = 2139095040u64;
        nat_lt(a1185, a1186)
    } else {
        false
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a1187 = 4u64;
    let a1188 = v0;
    nat_mul(a1187, a1188)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1189 = v0;
    let a1190 = 0u64;
    if nat_eq(a1189, a1190) {
        Ok(0u64)
    } else {
        let a1193 = 4u64;
        let a1194 = {
            let a1191 = 3u64;
            let a1192 = v0;
            nat_mul(a1191, a1192)?
        };
        nat_add(a1193, a1194)
    }
}

pub fn f37(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f25(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1195, _, _, _, _, _, _, _, _, _) => {
                h1195
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1196, _, _, _, _, _, _, _, _) => {
                h1196
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1197, _, _, _, _, _, _, _) => {
                h1197
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1198, _, _, _, _, _) => {
                h1198
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1199, _, _, _, _) => {
                h1199
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1200, _, _, _) => {
                h1200
            }
        })
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let a1202 = match v0.clone() {
        Adt2::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
            h1201
        }
    };
    let a1203 = 1u64;
    if if if if nat_eq(a1202, a1203) {
        true
    } else {
        let a1205 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1204, _, _, _, _, _) => {
                h1204
            }
        };
        let a1206 = 4u64;
        nat_eq(a1205, a1206)
    } {
        let a1208 = match v0.clone() {
            Adt2::C0(_, _, h1207, _, _, _, _, _, _, _, _) => {
                h1207
            }
        };
        let a1209 = 1u64;
        if nat_eq(a1208, a1209) {
            true
        } else {
            let a1214 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1210, _, _, _, _) => {
                    h1210
                }
            };
            let a1215 = {
                let a1212 = match v0.clone() {
                    Adt2::C0(_, h1211, _, _, _, _, _, _, _, _, _) => {
                        h1211
                    }
                };
                let a1213 = 4u64;
                nat_mul(a1212, a1213)?
            };
            nat_eq(a1214, a1215)
        }
    } else {
        false
    } {
        let a1217 = match v0.clone() {
            Adt2::C0(_, _, _, h1216, _, _, _, _, _, _, _) => {
                h1216
            }
        };
        let a1218 = 1u64;
        if nat_eq(a1217, a1218) {
            true
        } else {
            let a1226 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1219, _, _, _) => {
                    h1219
                }
            };
            let a1227 = {
                let a1224 = {
                    let a1222 = match v0.clone() {
                        Adt2::C0(_, h1220, _, _, _, _, _, _, _, _, _) => {
                            h1220
                        }
                    };
                    let a1223 = match v0.clone() {
                        Adt2::C0(_, _, h1221, _, _, _, _, _, _, _, _) => {
                            h1221
                        }
                    };
                    nat_mul(a1222, a1223)?
                };
                let a1225 = 4u64;
                nat_mul(a1224, a1225)?
            };
            nat_eq(a1226, a1227)
        }
    } else {
        false
    } {
        let a1229 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1228, _, _, _, _, _, _) => {
                h1228
            }
        };
        let a1230 = 1u64;
        if nat_eq(a1229, a1230) {
            Ok(true)
        } else {
            let a1241 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1231, _, _) => {
                    h1231
                }
            };
            let a1242 = {
                let a1239 = {
                    let a1237 = {
                        let a1234 = match v0.clone() {
                            Adt2::C0(_, h1232, _, _, _, _, _, _, _, _, _) => {
                                h1232
                            }
                        };
                        let a1235 = match v0.clone() {
                            Adt2::C0(_, _, h1233, _, _, _, _, _, _, _, _) => {
                                h1233
                            }
                        };
                        nat_mul(a1234, a1235)?
                    };
                    let a1238 = match v0 {
                        Adt2::C0(_, _, _, h1236, _, _, _, _, _, _, _) => {
                            h1236
                        }
                    };
                    nat_mul(a1237, a1238)?
                };
                let a1240 = 4u64;
                nat_mul(a1239, a1240)?
            };
            Ok(nat_eq(a1241, a1242))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt6, v1: u64) -> R<u64> {
    let a1244 = match v0.clone() {
        Adt6::C0(h1243, _, _, _) => {
            h1243
        }
    };
    let a1245 = 0u64;
    if nat_eq(a1244, a1245) {
        f91(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1246, _, _) => {
                h1246
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1247, _) => {
                h1247
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1248) => {
                h1248
            }
        }), {
            let a1250 = v1;
            let a1251 = match v0 {
                Adt6::C0(_, _, _, h1249) => {
                    h1249
                }
            };
            nat_add(a1250, a1251)?
        })
    } else {
        f91(v0, v1)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1256 = {
        let a1252 = v0;
        let a1253 = v1;
        nat_mul(a1252, a1253)?
    };
    let a1257 = {
        let a1254 = v2;
        let a1255 = f92(v3);
        nat_mul(a1254, a1255)?
    };
    nat_mul(a1256, a1257)
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1260 = v0;
    let a1261 = {
        let a1258 = f93(v1)?;
        let a1259 = v2;
        nat_mul(a1258, a1259)?
    };
    nat_add(a1260, a1261)
}

pub fn f42(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1262, _, _, _, _, _, _, _, _, _, _) => {
            h1262
        }
    }, match v0.clone() {
        Adt2::C0(_, h1263, _, _, _, _, _, _, _, _, _) => {
            h1263
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1264, _, _, _, _, _, _, _, _) => {
            h1264
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1265, _, _, _, _, _, _, _) => {
            h1265
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1266, _, _, _, _, _, _) => {
            h1266
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1267, _, _, _, _, _) => {
            h1267
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1268, _, _, _, _) => {
            h1268
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1269, _, _, _) => {
            h1269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1270, _, _) => {
            h1270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1271, _) => {
            h1271
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1272) => {
            h1272
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1273 = v0;
    let a1274 = 23u64;
    if nat_eq(a1273, a1274) {
        f94(v1, v4, v6)
    } else {
        let a1275 = v0;
        let a1276 = 26u64;
        if nat_eq(a1275, a1276) {
            f95(v1, v2, v4, v6)
        } else {
            let a1277 = v0;
            let a1278 = 25u64;
            if nat_eq(a1277, a1278) {
                f96(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    let a1279 = v0;
    let a1280 = 30u64;
    if nat_eq(a1279, a1280) {
        2u64
    } else {
        4u64
    }
}

pub fn f45(v0: u64) -> bool {
    let a1281 = v0;
    let a1282 = 2139095040u64;
    if nat_le(a1281, a1282) {
        true
    } else {
        let a1283 = v0;
        let a1284 = 2147483648u64;
        nat_eq(a1283, a1284)
    }
}

pub fn f46(v0: Adt2, v1: u64) -> bool {
    let a1286 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1285, _, _, _, _, _) => {
            h1285
        }
    };
    let a1287 = v1;
    if if nat_eq(a1286, a1287) {
        let a1292 = {
            let a1289 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1288) => {
                    h1288
                }
            };
            let a1290 = v1;
            let a1291 = 0u64;
            nat_rem(a1289, a1290, a1291)
        };
        let a1293 = 0u64;
        nat_eq(a1292, a1293)
    } else {
        false
    } {
        f97(match v0.clone() {
            Adt2::C0(_, _, h1294, _, _, _, _, _, _, _, _) => {
                h1294
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1295, _, _, _, _, _, _, _) => {
                h1295
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1296, _, _, _, _, _, _) => {
                h1296
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1297, _, _, _, _) => {
                h1297
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1298, _, _, _) => {
                h1298
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1299, _, _) => {
                h1299
            }
        }, v1)
    } else {
        false
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a1302 = match v0.clone() {
            Adt2::C0(h1300, _, _, _, _, _, _, _, _, _, _) => {
                h1300
            }
        };
        let a1303 = match v1.clone() {
            Adt2::C0(h1301, _, _, _, _, _, _, _, _, _, _) => {
                h1301
            }
        };
        nat_eq(a1302, a1303)
    } else {
        false
    } {
        let a1306 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1304) => {
                h1304
            }
        };
        let a1307 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1305) => {
                h1305
            }
        };
        nat_eq(a1306, a1307)
    } else {
        false
    } {
        let a1310 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1308, _, _, _, _, _) => {
                h1308
            }
        };
        let a1311 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1309, _, _, _, _, _) => {
                h1309
            }
        };
        nat_eq(a1310, a1311)
    } else {
        false
    } {
        let a1314 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1312, _, _, _, _) => {
                h1312
            }
        };
        let a1315 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1313, _, _, _, _) => {
                h1313
            }
        };
        nat_eq(a1314, a1315)
    } else {
        false
    } {
        let a1318 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1316, _, _, _) => {
                h1316
            }
        };
        let a1319 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1317, _, _, _) => {
                h1317
            }
        };
        nat_eq(a1318, a1319)
    } else {
        false
    } {
        let a1322 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1320, _, _) => {
                h1320
            }
        };
        let a1323 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1321, _, _) => {
                h1321
            }
        };
        nat_eq(a1322, a1323)
    } else {
        false
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1324 = v0;
    let a1325 = 0u64;
    if if if nat_eq(a1324, a1325) {
        true
    } else {
        let a1326 = v1;
        let a1327 = 0u64;
        nat_eq(a1326, a1327)
    } {
        true
    } else {
        let a1328 = v2;
        let a1329 = 0u64;
        nat_eq(a1328, a1329)
    } {
        true
    } else {
        let a1330 = v3;
        let a1331 = 0u64;
        nat_eq(a1330, a1331)
    }
}

pub fn f49(v0: Adt2, v1: u64) -> R<bool> {
    f98({
        let a1333 = match v0.clone() {
            Adt2::C0(_, h1332, _, _, _, _, _, _, _, _, _) => {
                h1332
            }
        };
        let a1334 = v1;
        nat_mul(a1333, a1334)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1335, _, _, _, _, _, _, _, _) => {
            h1335
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1336, _, _, _, _, _, _, _) => {
            h1336
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1337, _, _, _, _, _, _) => {
            h1337
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1338, _, _, _, _) => {
            h1338
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1339, _, _, _) => {
            h1339
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1340, _, _) => {
            h1340
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1341 = v1;
    let a1342 = 0u64;
    if if nat_eq(a1341, a1342) {
        true
    } else {
        let a1343 = v2;
        let a1344 = 0u64;
        nat_eq(a1343, a1344)
    } {
        Ok(0u64)
    } else {
        let a1351 = {
            let a1345 = f99(v1, v2, v3)?;
            let a1346 = f100(v0, v1, v2)?;
            nat_add(a1345, a1346)?
        };
        let a1352 = {
            let a1349 = f101(v0);
            let a1350 = {
                let a1347 = v1;
                let a1348 = v2;
                nat_mul(a1347, a1348)?
            };
            nat_mul(a1349, a1350)?
        };
        nat_add(a1351, a1352)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1353 = v0;
    let a1354 = 0u64;
    if if nat_eq(a1353, a1354) {
        true
    } else {
        let a1355 = v1;
        let a1356 = 0u64;
        nat_eq(a1355, a1356)
    } {
        Ok(3u64)
    } else {
        let a1359 = {
            let a1357 = 3u64;
            let a1358 = v1;
            nat_add(a1357, a1358)?
        };
        let a1360 = v2;
        nat_add(a1359, a1360)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1361 = v0;
    let a1362 = 2139095040u64;
    if nat_le(a1361, a1362) {
        true
    } else {
        let a1363 = v0;
        let a1364 = 2147483648u64;
        nat_eq(a1363, a1364)
    }
}

pub fn f53(v0: u64, v1: u64) -> R<u64> {
    let a1365 = v0;
    let a1366 = 0u64;
    if nat_eq(a1365, a1366) {
        Ok(0u64)
    } else {
        let a1367 = v1;
        let a1368 = f102(v0)?;
        nat_mul(a1367, a1368)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1369 = v0;
    let a1370 = 0u64;
    if if if if nat_eq(a1369, a1370) {
        true
    } else {
        let a1371 = v1;
        let a1372 = 0u64;
        nat_eq(a1371, a1372)
    } {
        true
    } else {
        let a1373 = v2;
        let a1374 = 0u64;
        nat_eq(a1373, a1374)
    } {
        true
    } else {
        let a1375 = v3;
        let a1376 = 0u64;
        nat_eq(a1375, a1376)
    } {
        Ok(0u64)
    } else {
        let a1389 = {
            let a1379 = 2u64;
            let a1380 = {
                let a1377 = v1;
                let a1378 = v2;
                nat_mul(a1377, a1378)?
            };
            nat_mul(a1379, a1380)?
        };
        let a1390 = {
            let a1387 = {
                let a1383 = {
                    let a1381 = v1;
                    let a1382 = v2;
                    nat_mul(a1381, a1382)?
                };
                let a1384 = v3;
                nat_mul(a1383, a1384)?
            };
            let a1388 = {
                let a1385 = 1u64;
                let a1386 = v0;
                nat_add(a1385, a1386)?
            };
            nat_mul(a1387, a1388)?
        };
        nat_add(a1389, a1390)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1391 = v1;
        let a1392 = 0u64;
        if if if nat_eq(a1391, a1392) {
            true
        } else {
            let a1393 = v1;
            let a1394 = 10u64;
            nat_eq(a1393, a1394)
        } {
            true
        } else {
            let a1395 = v1;
            let a1396 = 15u64;
            nat_eq(a1395, a1396)
        } {
            true
        } else {
            let a1397 = v1;
            let a1398 = 20u64;
            nat_eq(a1397, a1398)
        }
    };
    let v4: bool = {
        let a1399 = v2;
        let a1400 = 0u64;
        if if if if if nat_eq(a1399, a1400) {
            true
        } else {
            let a1401 = v2;
            let a1402 = 10u64;
            nat_eq(a1401, a1402)
        } {
            true
        } else {
            let a1403 = v2;
            let a1404 = 15u64;
            nat_eq(a1403, a1404)
        } {
            true
        } else {
            let a1405 = v2;
            let a1406 = 20u64;
            nat_eq(a1405, a1406)
        } {
            true
        } else {
            let a1407 = v2;
            let a1408 = 30u64;
            nat_eq(a1407, a1408)
        } {
            true
        } else {
            let a1409 = v2;
            let a1410 = 40u64;
            nat_eq(a1409, a1410)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1411 = v0;
            let a1412 = 30u64;
            if if nat_eq(a1411, a1412) {
                let a1413 = v2;
                let a1414 = 10u64;
                nat_eq(a1413, a1414)
            } else {
                false
            } {
                true
            } else {
                if f103(v0) {
                    let a1415 = v2;
                    let a1416 = 10u64;
                    if if nat_eq(a1415, a1416) {
                        true
                    } else {
                        let a1417 = v2;
                        let a1418 = 15u64;
                        nat_eq(a1417, a1418)
                    } {
                        true
                    } else {
                        let a1419 = v2;
                        let a1420 = 20u64;
                        nat_eq(a1419, a1420)
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
        Adt1::C0(h1421, _, _, _, _, _, _, _, _, _, _, _) => {
            h1421
        }
    };
    Ok(Adt9::C0({
        let a1422 = v0;
        let a1423 = 82u64;
        if nat_eq(a1422, a1423) {
            let m1424 = v3;
            match m1424 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f104(v2, v6, v4)?
                }
            }
        } else {
            let a1425 = v0;
            let a1426 = 86u64;
            if nat_eq(a1425, a1426) {
                let m1427 = v3;
                match m1427 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f105(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1428 = v0;
                let a1429 = 55u64;
                if nat_eq(a1428, a1429) {
                    let m1430 = v3;
                    match m1430 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f106(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1431 = v0;
                    let a1432 = 22u64;
                    if nat_eq(a1431, a1432) {
                        let m1433 = v3;
                        match m1433 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f107(v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1434 = v0;
                        let a1435 = 44u64;
                        if nat_eq(a1434, a1435) {
                            f108(v1, v2, v3, v4)?
                        } else {
                            let a1436 = v0;
                            let a1437 = 52u64;
                            if nat_eq(a1436, a1437) {
                                f109(v1, v2, v4)?
                            } else {
                                let a1438 = v0;
                                let a1439 = 73u64;
                                if nat_eq(a1438, a1439) {
                                    f110(v2, v3, v4)?
                                } else {
                                    let a1440 = v0;
                                    let a1441 = 57u64;
                                    if if nat_eq(a1440, a1441) {
                                        true
                                    } else {
                                        let a1442 = v0;
                                        let a1443 = 58u64;
                                        nat_eq(a1442, a1443)
                                    } {
                                        let m1444 = v3;
                                        match m1444 {
                                            None => {
                                                f111({
                                                    let a1445 = v0;
                                                    let a1446 = 58u64;
                                                    nat_eq(a1445, a1446)
                                                }, v1, v2, v4)?
                                            }
                                            Some(_) => {
                                                Adt10::C1
                                            }
                                        }
                                    } else {
                                        let a1447 = v0;
                                        let a1448 = 49u64;
                                        if nat_eq(a1447, a1448) {
                                            f112(v1, v2, v3, v4)?
                                        } else {
                                            let a1449 = v0;
                                            let a1450 = 51u64;
                                            if nat_eq(a1449, a1450) {
                                                f113(v1, v2, v3, v4)?
                                            } else {
                                                let a1451 = v0;
                                                let a1452 = 39u64;
                                                if nat_eq(a1451, a1452) {
                                                    f114(v2, v3, v4)?
                                                } else {
                                                    let a1453 = v0;
                                                    let a1454 = 77u64;
                                                    if nat_eq(a1453, a1454) {
                                                        f115(v2, v4)?
                                                    } else {
                                                        let a1455 = v0;
                                                        let a1456 = 40u64;
                                                        if nat_eq(a1455, a1456) {
                                                            f116(v2, v3, v4)?
                                                        } else {
                                                            if f117(v0) {
                                                                let a1457 = v0;
                                                                let a1458 = 33u64;
                                                                if if if nat_eq(a1457, a1458) {
                                                                    let a1460 = match v2.clone() {
                                                                        Adt2::C0(h1459, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1459
                                                                        }
                                                                    };
                                                                    let a1461 = 0u64;
                                                                    nat_eq(a1460, a1461)
                                                                } else {
                                                                    false
                                                                } {
                                                                    let a1463 = match v4.clone() {
                                                                        Adt2::C0(h1462, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1462
                                                                        }
                                                                    };
                                                                    let a1464 = 26u64;
                                                                    nat_eq(a1463, a1464)
                                                                } else {
                                                                    false
                                                                } {
                                                                    let m1465 = v3;
                                                                    match m1465 {
                                                                        None => {
                                                                            Adt10::C1
                                                                        }
                                                                        Some(v11) => {
                                                                            f118(v2, v11, v4)?
                                                                        }
                                                                    }
                                                                } else {
                                                                    let a1466 = v0;
                                                                    let a1467 = 33u64;
                                                                    if if if nat_eq(a1466, a1467) {
                                                                        let a1469 = match v4.clone() {
                                                                            Adt2::C0(h1468, _, _, _, _, _, _, _, _, _, _) => {
                                                                                h1468
                                                                            }
                                                                        };
                                                                        let a1470 = 0u64;
                                                                        nat_eq(a1469, a1470)
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f119(match v2.clone() {
                                                                            Adt2::C0(h1471, _, _, _, _, _, _, _, _, _, _) => {
                                                                                h1471
                                                                            }
                                                                        })
                                                                    } else {
                                                                        false
                                                                    } {
                                                                        f120(v2, v3, v4)?
                                                                    } else {
                                                                        f121(v0, v2, {
                                                                            let m1472 = v3;
                                                                            match m1472 {
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
                                                                let a1473 = v0;
                                                                let a1474 = 19u64;
                                                                if nat_eq(a1473, a1474) {
                                                                    f122(v2, v4)?
                                                                } else {
                                                                    let a1475 = v0;
                                                                    let a1476 = 21u64;
                                                                    if nat_eq(a1475, a1476) {
                                                                        f123(v5, v2, v3, v4)?
                                                                    } else {
                                                                        let a1477 = v0;
                                                                        let a1478 = 61u64;
                                                                        if if if nat_eq(a1477, a1478) {
                                                                            true
                                                                        } else {
                                                                            let a1479 = v0;
                                                                            let a1480 = 62u64;
                                                                            nat_eq(a1479, a1480)
                                                                        } {
                                                                            true
                                                                        } else {
                                                                            let a1481 = v0;
                                                                            let a1482 = 63u64;
                                                                            nat_eq(a1481, a1482)
                                                                        } {
                                                                            f124(v0, v1, v2, v4)?
                                                                        } else {
                                                                            let a1483 = v0;
                                                                            let a1484 = 4u64;
                                                                            if if nat_eq(a1483, a1484) {
                                                                                true
                                                                            } else {
                                                                                let a1485 = v0;
                                                                                let a1486 = 32u64;
                                                                                nat_eq(a1485, a1486)
                                                                            } {
                                                                                f125(v0, v1, v2, v3, v4)?
                                                                            } else {
                                                                                let a1487 = v0;
                                                                                let a1488 = 20u64;
                                                                                if nat_eq(a1487, a1488) {
                                                                                    f126(v2, v4)?
                                                                                } else {
                                                                                    let a1489 = v0;
                                                                                    let a1490 = 42u64;
                                                                                    if nat_eq(a1489, a1490) {
                                                                                        f127(4u64, v2, v4)?
                                                                                    } else {
                                                                                        let a1491 = v0;
                                                                                        let a1492 = 69u64;
                                                                                        if nat_eq(a1491, a1492) {
                                                                                            let a1493 = v5;
                                                                                            let a1494 = 4u64;
                                                                                            if nat_lt(a1493, a1494) {
                                                                                                f127(v5, v2, v4)?
                                                                                            } else {
                                                                                                Adt10::C1
                                                                                            }
                                                                                        } else {
                                                                                            let a1495 = v0;
                                                                                            let a1496 = 70u64;
                                                                                            if nat_eq(a1495, a1496) {
                                                                                                f128(v5, v2, v4)?
                                                                                            } else {
                                                                                                let a1497 = v0;
                                                                                                let a1498 = 75u64;
                                                                                                if if nat_eq(a1497, a1498) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1499 = v0;
                                                                                                    let a1500 = 76u64;
                                                                                                    nat_eq(a1499, a1500)
                                                                                                } {
                                                                                                    f129({
                                                                                                        let a1501 = v0;
                                                                                                        let a1502 = 76u64;
                                                                                                        nat_eq(a1501, a1502)
                                                                                                    }, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1503 = v0;
                                                                                                    let a1504 = 31u64;
                                                                                                    if if if nat_eq(a1503, a1504) {
                                                                                                        true
                                                                                                    } else {
                                                                                                        let a1505 = v0;
                                                                                                        let a1506 = 47u64;
                                                                                                        nat_eq(a1505, a1506)
                                                                                                    } {
                                                                                                        true
                                                                                                    } else {
                                                                                                        let a1507 = v0;
                                                                                                        let a1508 = 68u64;
                                                                                                        nat_eq(a1507, a1508)
                                                                                                    } {
                                                                                                        f130(v0, v1, v2, v4)?
                                                                                                    } else {
                                                                                                        let a1509 = v0;
                                                                                                        let a1510 = 15u64;
                                                                                                        if nat_eq(a1509, a1510) {
                                                                                                            f131(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1511 = v0;
                                                                                                            let a1512 = 17u64;
                                                                                                            if nat_eq(a1511, a1512) {
                                                                                                                f132(v2, v4)?
                                                                                                            } else {
                                                                                                                let a1513 = v0;
                                                                                                                let a1514 = 18u64;
                                                                                                                if nat_eq(a1513, a1514) {
                                                                                                                    f133(v2, v3, v4)?
                                                                                                                } else {
                                                                                                                    let a1515 = v0;
                                                                                                                    let a1516 = 13u64;
                                                                                                                    if if if nat_eq(a1515, a1516) {
                                                                                                                        true
                                                                                                                    } else {
                                                                                                                        let a1517 = v0;
                                                                                                                        let a1518 = 14u64;
                                                                                                                        nat_eq(a1517, a1518)
                                                                                                                    } {
                                                                                                                        true
                                                                                                                    } else {
                                                                                                                        let a1519 = v0;
                                                                                                                        let a1520 = 16u64;
                                                                                                                        nat_eq(a1519, a1520)
                                                                                                                    } {
                                                                                                                        f134(v0, v2, v4)?
                                                                                                                    } else {
                                                                                                                        let m1521 = f135(v0, v5)?;
                                                                                                                        match m1521 {
                                                                                                                            None => {
                                                                                                                                f136(v0, v5, v2, v3, v4)?
                                                                                                                            }
                                                                                                                            Some(v13) => {
                                                                                                                                f137(v13, v2, v4)?
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    f138(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1522, _, _, _) => {
            h1522
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1523, _, _) => {
            h1523
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1524, _) => {
            h1524
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1525) => {
            h1525
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1527 = v0;
        let a1528 = match v1.clone() {
            Adt3::C0(h1526, _, _, _, _, _, _, _) => {
                h1526
            }
        };
        let a1529 = 0u64;
        nat_quot(a1527, a1528, a1529)
    };
    let v4: u64 = {
        let a1531 = v3;
        let a1532 = match v1.clone() {
            Adt3::C0(_, h1530, _, _, _, _, _, _) => {
                h1530
            }
        };
        let a1533 = 0u64;
        nat_quot(a1531, a1532, a1533)
    };
    let a1534 = v2;
    let a1535 = 0u64;
    if nat_eq(a1534, a1535) {
        let a1537 = v0;
        let a1538 = match v1 {
            Adt3::C0(h1536, _, _, _, _, _, _, _) => {
                h1536
            }
        };
        let a1539 = 0u64;
        nat_rem(a1537, a1538, a1539)
    } else {
        let a1540 = v2;
        let a1541 = 1u64;
        if nat_eq(a1540, a1541) {
            let a1543 = v3;
            let a1544 = match v1 {
                Adt3::C0(_, h1542, _, _, _, _, _, _) => {
                    h1542
                }
            };
            let a1545 = 0u64;
            nat_rem(a1543, a1544, a1545)
        } else {
            let a1546 = v2;
            let a1547 = 2u64;
            if nat_eq(a1546, a1547) {
                let a1549 = v4;
                let a1550 = match v1 {
                    Adt3::C0(_, _, h1548, _, _, _, _, _) => {
                        h1548
                    }
                };
                let a1551 = 0u64;
                nat_rem(a1549, a1550, a1551)
            } else {
                let a1553 = v4;
                let a1554 = match v1 {
                    Adt3::C0(_, _, h1552, _, _, _, _, _) => {
                        h1552
                    }
                };
                let a1555 = 0u64;
                nat_quot(a1553, a1554, a1555)
            }
        }
    }
}

pub fn f59(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1557 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1556) => {
            h1556
        }
    };
    let a1558 = 0u64;
    if nat_eq(a1557, a1558) {
        Ok(v2)
    } else {
        let a1563 = {
            let a1560 = v2;
            let a1561 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1559) => {
                    h1559
                }
            };
            nat_mul(a1560, a1561)?
        };
        let a1564 = f58(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1562, _, _, _, _, _, _) => {
                h1562
            }
        }, 3u64);
        nat_add(a1563, a1564)
    }
}

pub fn f60(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1574 = {
            let a1571 = {
                let a1566 = v2;
                let a1567 = match v0.clone() {
                    Adt6::C0(_, h1565, _, _) => {
                        h1565
                    }
                };
                nat_mul(a1566, a1567)?
            };
            let a1572 = {
                let a1569 = v3;
                let a1570 = match v0.clone() {
                    Adt6::C0(_, _, _, h1568) => {
                        h1568
                    }
                };
                nat_mul(a1569, a1570)?
            };
            nat_add(a1571, a1572)?
        };
        let a1575 = f139(match v0.clone() {
            Adt6::C0(_, _, h1573, _) => {
                h1573
            }
        });
        nat_add(a1574, a1575)?
    };
    let v5: u64 = f140(match v0 {
        Adt6::C0(_, _, h1576, _) => {
            h1576
        }
    });
    let a1577 = v5;
    let a1578 = v4;
    if if nat_le(a1577, a1578) {
        let a1581 = {
            let a1579 = v4;
            let a1580 = v5;
            nat_sub(a1579, a1580)
        };
        let a1582 = v1;
        nat_lt(a1581, a1582)
    } else {
        false
    } {
        Ok(Some({
            let a1583 = v4;
            let a1584 = v5;
            nat_sub(a1583, a1584)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1585, _, _, _, _, _) => {
            h1585
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1586, _, _, _, _, _, _) => {
            h1586
        }
    };
    let v7: u64 = f58(v1, v6.clone(), 2u64);
    let a1588 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1587) => {
            h1587
        }
    };
    let a1589 = 0u64;
    if nat_eq(a1588, a1589) {
        Ok(Adt9::C8(Some(f57(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1592 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1591, _, _, _, _) => {
                    h1591
                }
            };
            match m1592 {
                Adt3::C0(_, _, h1590, _, _, _, _, _) => {
                    h1590
                }
            }
        };
        let m1598 = f60(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1593, _) => {
                h1593
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1594, _, _, _, _, _) => {
                h1594
            }
        }, {
            let a1595 = v2;
            let a1596 = v8;
            let a1597 = 0u64;
            nat_rem(a1595, a1596, a1597)
        }, v7)?;
        match m1598 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1605 = {
                        let a1603 = {
                            let a1599 = v2;
                            let a1600 = v8;
                            let a1601 = 0u64;
                            nat_quot(a1599, a1600, a1601)
                        };
                        let a1604 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1602) => {
                                h1602
                            }
                        };
                        nat_mul(a1603, a1604)?
                    };
                    let a1606 = f58(v1, v6, 3u64);
                    nat_add(a1605, a1606)?
                };
                Ok(Adt9::C8(Some(f57(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1607 = v1;
    match m1607 {
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
    let m1608 = v3;
    match m1608 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f147(v0, v1, v2.clone())?, f90(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f148(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f149(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f150(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f149(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f151(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f152(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f153(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f90(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f154(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v51, v52) => {
            f155(v0, v1, v2, v51, v52)
        }
        Adt4::C12(v53, v54, v55) => {
            f156(v0, v1, v2, v53, v54, v55)
        }
        Adt4::C13(_, v57, _, _) => {
            Ok(Adt9::C3(v57, v57, f90(v0, v1)?, f90(v0, v2)?))
        }
        Adt4::C14(v60, v61, v62) => {
            let a1611 = v60;
            let a1612 = 23u64;
            if nat_eq(a1611, a1612) {
                f157(v0, v1, v2, v61, v62)
            } else {
                Ok(Adt9::C3(v61, v62, f90(v0, v1)?, f90(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f158(v0, v1, v2)
        }
        Adt4::C16(v63, v64) => {
            f159(v0, v1, v2, v63, v64)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v65, v66) => {
            f160(v0, v1, v66, v65)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f90(f146(v0, match v2.clone() {
                Adt3::C0(h1609, _, _, _, _, _, _, _) => {
                    h1609
                }
            }, match v2.clone() {
                Adt3::C0(_, h1610, _, _, _, _, _, _) => {
                    h1610
                }
            })?, v1)?, f90(v0, v2)?))
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
            f161(v0, v71, v72, v73)
        }
        Adt4::C26(v79, _, _, _) => {
            let a1613 = v79;
            let a1614 = 64u64;
            if if if nat_eq(a1613, a1614) {
                true
            } else {
                let a1615 = v79;
                let a1616 = 65u64;
                nat_eq(a1615, a1616)
            } {
                true
            } else {
                let a1617 = v79;
                let a1618 = 66u64;
                nat_eq(a1617, a1618)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v78) => {
            f82(v0, v1, v2, v78)
        }
        Adt4::C28(v74) => {
            f79(v0, v1, v2, v74, 0u64)
        }
        Adt4::C29(v75) => {
            f80(v0, v1, v2, v75)
        }
        Adt4::C30(v76, _) => {
            f81(v0, v1, v2, v76)
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
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f162(v3, v4);
    let v6: u64 = {
        let a1619 = v0;
        let a1620 = v5;
        let a1621 = 0u64;
        nat_quot(a1619, a1620, a1621)
    };
    let a1622 = v5;
    let a1623 = 1u64;
    if nat_eq(a1622, a1623) {
        Ok(f141(f163(v3, v4), v3, v4, 0u64, f90(v6, v1)?, 2u64, 0u64, 3u64, f90(v6, v2)?))
    } else {
        let a1627 = {
            let a1624 = v0;
            let a1625 = 2u64;
            let a1626 = 0u64;
            nat_rem(a1624, a1625, a1626)
        };
        let a1628 = 0u64;
        if nat_eq(a1627, a1628) {
            Ok(f141(f163(v3, 0u64), v3, 0u64, 0u64, f90(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f141(f163(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f90(v6, v2)?))
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f164(v1)?;
    let v4: u64 = {
        let a1629 = v0;
        let a1630 = v3;
        let a1631 = 0u64;
        nat_rem(a1629, a1630, a1631)
    };
    let v5: u64 = {
        let a1634 = {
            let a1632 = v1;
            let a1633 = 3u64;
            nat_mul(a1632, a1633)?
        };
        let a1635 = 1u64;
        nat_add(a1634, a1635)?
    };
    let a1636 = v4;
    let a1637 = 0u64;
    if nat_eq(a1636, a1637) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1638 = v4;
        let a1639 = v5;
        if nat_eq(a1638, a1639) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1644 = v4;
                let a1645 = {
                    let a1640 = v4;
                    let a1641 = v5;
                    if nat_lt(a1640, a1641) {
                        1u64
                    } else {
                        let a1642 = v5;
                        let a1643 = 1u64;
                        nat_add(a1642, a1643)?
                    }
                };
                nat_sub(a1644, a1645)
            };
            let v7: u64 = {
                let a1646 = v6;
                let a1647 = 3u64;
                let a1648 = 0u64;
                nat_rem(a1646, a1647, a1648)
            };
            let v8: u64 = {
                let a1659 = 4u64;
                let a1660 = {
                    let a1657 = {
                        let a1652 = {
                            let a1649 = v0;
                            let a1650 = v3;
                            let a1651 = 0u64;
                            nat_quot(a1649, a1650, a1651)
                        };
                        let a1653 = v1;
                        nat_mul(a1652, a1653)?
                    };
                    let a1658 = {
                        let a1654 = v6;
                        let a1655 = 3u64;
                        let a1656 = 0u64;
                        nat_quot(a1654, a1655, a1656)
                    };
                    nat_add(a1657, a1658)?
                };
                nat_mul(a1659, a1660)?
            };
            let a1661 = v4;
            let a1662 = v5;
            if nat_lt(a1661, a1662) {
                let a1663 = v7;
                let a1664 = 0u64;
                if nat_eq(a1663, a1664) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1665 = v7;
                    let a1666 = 1u64;
                    if nat_eq(a1665, a1666) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1667 = v7;
                let a1668 = 0u64;
                if nat_eq(a1667, a1668) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1669 = v7;
                    let a1670 = 1u64;
                    if nat_eq(a1669, a1670) {
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
        let a1671 = v0;
        let a1672 = 6u64;
        let a1673 = 0u64;
        nat_quot(a1671, a1672, a1673)
    };
    let v5: u64 = {
        let a1674 = v0;
        let a1675 = 6u64;
        let a1676 = 0u64;
        nat_rem(a1674, a1675, a1676)
    };
    let a1677 = v5;
    let a1678 = 0u64;
    if nat_eq(a1677, a1678) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f90(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1679 = v5;
        let a1680 = 1u64;
        if nat_eq(a1679, a1680) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1681 = v5;
            let a1682 = 2u64;
            if nat_eq(a1681, a1682) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f90(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1683 = v5;
                let a1684 = 3u64;
                if nat_eq(a1683, a1684) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1685 = v5;
                    let a1686 = 4u64;
                    if nat_eq(a1685, a1686) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f90(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f90(v4, v3)?), 4u64, 4u64))
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
    let v7: u64 = f165(match v1.clone() {
        Adt3::C0(_, h1687, _, _, _, _, _, _) => {
            h1687
        }
    }, v4)?;
    let v8: u64 = {
        let a1688 = v0;
        let a1689 = v7;
        let a1690 = 0u64;
        nat_rem(a1688, a1689, a1690)
    };
    let v9: u64 = {
        let a1691 = v0;
        let a1692 = v7;
        let a1693 = 0u64;
        nat_quot(a1691, a1692, a1693)
    };
    let a1696 = {
        let a1694 = v8;
        let a1695 = 1u64;
        nat_add(a1694, a1695)?
    };
    let a1697 = v7;
    if nat_eq(a1696, a1697) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f90(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1698 = v8;
            let a1699 = v6;
            let a1700 = 0u64;
            nat_quot(a1698, a1699, a1700)
        };
        let v11: u64 = {
            let a1701 = v8;
            let a1702 = v6;
            let a1703 = 0u64;
            nat_rem(a1701, a1702, a1703)
        };
        let v12: u64 = {
            let a1705 = v9;
            let a1706 = match v1.clone() {
                Adt3::C0(h1704, _, _, _, _, _, _, _) => {
                    h1704
                }
            };
            let a1707 = 0u64;
            nat_rem(a1705, a1706, a1707)
        };
        let v13: u64 = {
            let a1709 = v9;
            let a1710 = match v1.clone() {
                Adt3::C0(h1708, _, _, _, _, _, _, _) => {
                    h1708
                }
            };
            let a1711 = 0u64;
            nat_quot(a1709, a1710, a1711)
        };
        let v14: u64 = if v4 {
            let a1723 = {
                let a1718 = {
                    let a1713 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1712, _, _, _) => {
                            h1712
                        }
                    };
                    let a1714 = v12;
                    nat_mul(a1713, a1714)?
                };
                let a1719 = {
                    let a1716 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1715, _, _) => {
                            h1715
                        }
                    };
                    let a1717 = v10;
                    nat_mul(a1716, a1717)?
                };
                nat_add(a1718, a1719)?
            };
            let a1724 = {
                let a1721 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1720, _) => {
                        h1720
                    }
                };
                let a1722 = v13;
                nat_mul(a1721, a1722)?
            };
            nat_add(a1723, a1724)?
        } else {
            let a1731 = {
                let a1726 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1725, _, _, _) => {
                        h1725
                    }
                };
                let a1727 = v10;
                nat_mul(a1726, a1727)?
            };
            let a1732 = {
                let a1729 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1728, _, _) => {
                        h1728
                    }
                };
                let a1730 = v13;
                nat_mul(a1729, a1730)?
            };
            nat_add(a1731, a1732)?
        };
        if if v4 {
            let a1733 = v11;
            let a1734 = 0u64;
            nat_eq(a1733, a1734)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1737 = {
                let a1735 = v11;
                let a1736 = 1u64;
                nat_add(a1735, a1736)?
            };
            let a1738 = v6;
            if nat_eq(a1737, a1738) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1739 = v10;
                    let a1740 = 0u64;
                    if nat_eq(a1739, a1740) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1752 = {
                        let a1747 = {
                            let a1742 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1741, _, _, _) => {
                                    h1741
                                }
                            };
                            let a1743 = v12;
                            nat_mul(a1742, a1743)?
                        };
                        let a1748 = {
                            let a1745 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1744, _, _) => {
                                    h1744
                                }
                            };
                            let a1746 = v10;
                            nat_mul(a1745, a1746)?
                        };
                        nat_add(a1747, a1748)?
                    };
                    let a1753 = {
                        let a1750 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1749, _) => {
                                h1749
                            }
                        };
                        let a1751 = v13;
                        nat_mul(a1750, a1751)?
                    };
                    nat_add(a1752, a1753)?
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
        Adt3::C0(_, h1754, _, _, _, _, _, _) => {
            h1754
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1755 = v0;
        let a1756 = v6;
        let a1757 = 0u64;
        nat_rem(a1755, a1756, a1757)
    };
    let v8: u64 = {
        let a1758 = v0;
        let a1759 = v6;
        let a1760 = 0u64;
        nat_quot(a1758, a1759, a1760)
    };
    let v9: u64 = {
        let a1762 = v8;
        let a1763 = match v5.clone() {
            Adt3::C0(h1761, _, _, _, _, _, _, _) => {
                h1761
            }
        };
        let a1764 = 0u64;
        nat_rem(a1762, a1763, a1764)
    };
    let v10: u64 = {
        let a1766 = v8;
        let a1767 = match v5.clone() {
            Adt3::C0(h1765, _, _, _, _, _, _, _) => {
                h1765
            }
        };
        let a1768 = 0u64;
        nat_quot(a1766, a1767, a1768)
    };
    let v11: u64 = {
        let a1770 = v10;
        let a1771 = match v2.clone() {
            Adt3::C0(_, h1769, _, _, _, _, _, _) => {
                h1769
            }
        };
        let a1772 = 0u64;
        nat_rem(a1770, a1771, a1772)
    };
    let v12: u64 = {
        let a1774 = v10;
        let a1775 = match v2.clone() {
            Adt3::C0(_, h1773, _, _, _, _, _, _) => {
                h1773
            }
        };
        let a1776 = 0u64;
        nat_quot(a1774, a1775, a1776)
    };
    let a1777 = v7;
    let a1778 = 0u64;
    if nat_eq(a1777, a1778) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1785 = {
                let a1780 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1779, _, _, _) => {
                        h1779
                    }
                };
                let a1781 = v9;
                nat_mul(a1780, a1781)?
            };
            let a1786 = {
                let a1783 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1782, _, _) => {
                        h1782
                    }
                };
                let a1784 = v12;
                nat_mul(a1783, a1784)?
            };
            nat_add(a1785, a1786)?
        }), Adt12::C0(2u64, {
            let a1793 = {
                let a1788 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1787, _, _, _) => {
                        h1787
                    }
                };
                let a1789 = v11;
                nat_mul(a1788, a1789)?
            };
            let a1794 = {
                let a1791 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1790, _, _) => {
                        h1790
                    }
                };
                let a1792 = v12;
                nat_mul(a1791, a1792)?
            };
            nat_add(a1793, a1794)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1795 = v4;
        match m1795 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1807 = {
                        let a1802 = {
                            let a1797 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1796, _, _, _) => {
                                    h1796
                                }
                            };
                            let a1798 = v9;
                            nat_mul(a1797, a1798)?
                        };
                        let a1803 = {
                            let a1800 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1799, _, _) => {
                                    h1799
                                }
                            };
                            let a1801 = v11;
                            nat_mul(a1800, a1801)?
                        };
                        nat_add(a1802, a1803)?
                    };
                    let a1808 = {
                        let a1805 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1804, _) => {
                                h1804
                            }
                        };
                        let a1806 = v12;
                        nat_mul(a1805, a1806)?
                    };
                    nat_add(a1807, a1808)?
                }), Adt13::C1(f90(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1809 = v7;
                    let a1810 = 1u64;
                    nat_sub(a1809, a1810)
                };
                let v15: u64 = {
                    let a1811 = v14;
                    let a1812 = 2u64;
                    let a1813 = 0u64;
                    nat_quot(a1811, a1812, a1813)
                };
                let a1817 = {
                    let a1814 = v14;
                    let a1815 = 2u64;
                    let a1816 = 0u64;
                    nat_rem(a1814, a1815, a1816)
                };
                let a1818 = 0u64;
                if nat_eq(a1817, a1818) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1830 = {
                            let a1825 = {
                                let a1820 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1819, _, _, _) => {
                                        h1819
                                    }
                                };
                                let a1821 = v9;
                                nat_mul(a1820, a1821)?
                            };
                            let a1826 = {
                                let a1823 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1822, _, _) => {
                                        h1822
                                    }
                                };
                                let a1824 = v15;
                                nat_mul(a1823, a1824)?
                            };
                            nat_add(a1825, a1826)?
                        };
                        let a1831 = {
                            let a1828 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1827, _) => {
                                    h1827
                                }
                            };
                            let a1829 = v12;
                            nat_mul(a1828, a1829)?
                        };
                        nat_add(a1830, a1831)?
                    }), Adt12::C0(3u64, {
                        let a1843 = {
                            let a1838 = {
                                let a1833 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1832, _, _, _) => {
                                        h1832
                                    }
                                };
                                let a1834 = v11;
                                nat_mul(a1833, a1834)?
                            };
                            let a1839 = {
                                let a1836 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1835, _, _) => {
                                        h1835
                                    }
                                };
                                let a1837 = v15;
                                nat_mul(a1836, a1837)?
                            };
                            nat_add(a1838, a1839)?
                        };
                        let a1844 = {
                            let a1841 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1840, _) => {
                                    h1840
                                }
                            };
                            let a1842 = v12;
                            nat_mul(a1841, a1842)?
                        };
                        nat_add(a1843, a1844)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1847 = {
                            let a1845 = v7;
                            let a1846 = 1u64;
                            nat_add(a1845, a1846)?
                        };
                        let a1848 = v6;
                        if nat_eq(a1847, a1848) {
                            Adt13::C1(f90(v8, v5)?)
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
        let a1849 = v0;
        let a1850 = v8;
        let a1851 = 0u64;
        nat_rem(a1849, a1850, a1851)
    };
    let v10: u64 = {
        let a1852 = v0;
        let a1853 = v8;
        let a1854 = 0u64;
        nat_quot(a1852, a1853, a1854)
    };
    let a1855 = v9;
    let a1856 = 148u64;
    if nat_lt(a1855, a1856) {
        f166({
            let a1857 = v9;
            let a1858 = 37u64;
            let a1859 = 0u64;
            nat_rem(a1857, a1858, a1859)
        }, {
            let a1860 = v9;
            let a1861 = 37u64;
            let a1862 = 0u64;
            nat_quot(a1860, a1861, a1862)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f167({
            let a1863 = v9;
            let a1864 = 148u64;
            nat_sub(a1863, a1864)
        }, v10, v4, v5)
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f28(v2);
    let a1865 = v0;
    let a1866 = v3;
    if nat_lt(a1865, a1866) {
        let a1867 = v0;
        let a1868 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1867, a1868) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1869 = 4u64;
                let a1870 = v0;
                nat_mul(a1869, a1870)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1871 = v0;
            let a1872 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1871, a1872) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1875 = v0;
                let a1876 = {
                    let a1873 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1874 = 1u64;
                    nat_add(a1873, a1874)?
                };
                if nat_eq(a1875, a1876) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1877 = v0;
                    let a1878 = 9u64;
                    if nat_eq(a1877, a1878) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1879 = v0;
            let a1880 = v3;
            nat_sub(a1879, a1880)
        };
        let v5: u64 = f29(v2);
        let v6: u64 = {
            let a1881 = v4;
            let a1882 = v5;
            let a1883 = 0u64;
            nat_rem(a1881, a1882, a1883)
        };
        let v7: u64 = f90({
            let a1884 = v4;
            let a1885 = v5;
            let a1886 = 0u64;
            nat_quot(a1884, a1885, a1886)
        }, v1)?;
        if v2 {
            let a1887 = v6;
            let a1888 = 7u64;
            if nat_lt(a1887, a1888) {
                let a1889 = v6;
                let a1890 = 3u64;
                if nat_lt(a1889, a1890) {
                    let a1891 = v6;
                    let a1892 = 1u64;
                    if nat_lt(a1891, a1892) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1893 = v6;
                        let a1894 = 2u64;
                        if nat_lt(a1893, a1894) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1895 = v6;
                    let a1896 = 5u64;
                    if nat_lt(a1895, a1896) {
                        let a1897 = v6;
                        let a1898 = 4u64;
                        if nat_lt(a1897, a1898) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1899 = v6;
                        let a1900 = 6u64;
                        if nat_lt(a1899, a1900) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1901 = v6;
                let a1902 = 11u64;
                if nat_lt(a1901, a1902) {
                    let a1903 = v6;
                    let a1904 = 9u64;
                    if nat_lt(a1903, a1904) {
                        let a1905 = v6;
                        let a1906 = 8u64;
                        if nat_lt(a1905, a1906) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1907 = v6;
                        let a1908 = 10u64;
                        if nat_lt(a1907, a1908) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1909 = v6;
                    let a1910 = 13u64;
                    if nat_lt(a1909, a1910) {
                        let a1911 = v6;
                        let a1912 = 12u64;
                        if nat_lt(a1911, a1912) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1913 = v6;
                        let a1914 = 14u64;
                        if nat_lt(a1913, a1914) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1915 = v6;
            let a1916 = 1u64;
            if nat_lt(a1915, a1916) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1917 = v6;
                let a1918 = 2u64;
                if nat_lt(a1917, a1918) {
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
        Adt3::C0(h1919, _, _, _, _, _, _, _) => {
            h1919
        }
    };
    let v8: u64 = f24(v3, v7)?;
    let v9: u64 = {
        let a1920 = v0;
        let a1921 = 2u64;
        nat_sub(a1920, a1921)
    };
    let v10: u64 = {
        let a1922 = v9;
        let a1923 = v8;
        let a1924 = 0u64;
        nat_quot(a1922, a1923, a1924)
    };
    let v11: u64 = {
        let a1925 = v9;
        let a1926 = v8;
        let a1927 = 0u64;
        nat_rem(a1925, a1926, a1927)
    };
    let a1928 = v0;
    let a1929 = 0u64;
    if nat_eq(a1928, a1929) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a1930 = v0;
        let a1931 = 1u64;
        if nat_eq(a1930, a1931) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a1932 = v10;
            let a1933 = v5;
            if nat_eq(a1932, a1933) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a1936 = v11;
                let a1937 = {
                    let a1934 = 2u64;
                    let a1935 = v7;
                    nat_mul(a1934, a1935)?
                };
                if nat_lt(a1936, a1937) {
                    let v12: u64 = {
                        let a1938 = v11;
                        let a1939 = 2u64;
                        let a1940 = 0u64;
                        nat_quot(a1938, a1939, a1940)
                    };
                    let a1944 = {
                        let a1941 = v11;
                        let a1942 = 2u64;
                        let a1943 = 0u64;
                        nat_rem(a1941, a1942, a1943)
                    };
                    let a1945 = 0u64;
                    if nat_eq(a1944, a1945) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a1946 = v12;
                            let a1947 = 0u64;
                            if nat_eq(a1946, a1947) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a1953 = {
                                let a1949 = v10;
                                let a1950 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h1948, _, _) => {
                                        h1948
                                    }
                                };
                                nat_mul(a1949, a1950)?
                            };
                            let a1954 = {
                                let a1951 = 4u64;
                                let a1952 = v12;
                                nat_mul(a1951, a1952)?
                            };
                            nat_add(a1953, a1954)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a1955 = v6;
                            let a1956 = 2u64;
                            if nat_eq(a1955, a1956) {
                                let a1957 = v12;
                                let a1958 = 0u64;
                                if nat_eq(a1957, a1958) {
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
                                    let a1964 = {
                                        let a1960 = v10;
                                        let a1961 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1959, _, _) => {
                                                h1959
                                            }
                                        };
                                        nat_mul(a1960, a1961)?
                                    };
                                    let a1965 = {
                                        let a1962 = 4u64;
                                        let a1963 = v12;
                                        nat_mul(a1962, a1963)?
                                    };
                                    nat_add(a1964, a1965)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a1968 = v11;
                    let a1969 = {
                        let a1966 = 6u64;
                        let a1967 = v7;
                        nat_mul(a1966, a1967)?
                    };
                    if nat_lt(a1968, a1969) {
                        let v13: u64 = {
                            let a1972 = v11;
                            let a1973 = {
                                let a1970 = 2u64;
                                let a1971 = v7;
                                nat_mul(a1970, a1971)?
                            };
                            nat_sub(a1972, a1973)
                        };
                        let v14: u64 = {
                            let a1974 = v13;
                            let a1975 = 4u64;
                            let a1976 = 0u64;
                            nat_quot(a1974, a1975, a1976)
                        };
                        let a1980 = {
                            let a1977 = v13;
                            let a1978 = 4u64;
                            let a1979 = 0u64;
                            nat_rem(a1977, a1978, a1979)
                        };
                        let a1981 = 2u64;
                        if nat_lt(a1980, a1981) {
                            let a1985 = {
                                let a1982 = v13;
                                let a1983 = 4u64;
                                let a1984 = 0u64;
                                nat_rem(a1982, a1983, a1984)
                            };
                            let a1986 = 1u64;
                            if nat_lt(a1985, a1986) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a1992 = {
                                        let a1988 = v10;
                                        let a1989 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1987, _, _) => {
                                                h1987
                                            }
                                        };
                                        nat_mul(a1988, a1989)?
                                    };
                                    let a1993 = {
                                        let a1990 = 4u64;
                                        let a1991 = v14;
                                        nat_mul(a1990, a1991)?
                                    };
                                    nat_add(a1992, a1993)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a1999 = {
                                            let a1995 = v10;
                                            let a1996 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h1994, _, _) => {
                                                    h1994
                                                }
                                            };
                                            nat_mul(a1995, a1996)?
                                        };
                                        let a2000 = {
                                            let a1997 = 4u64;
                                            let a1998 = v14;
                                            nat_mul(a1997, a1998)?
                                        };
                                        nat_add(a1999, a2000)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2004 = {
                                let a2001 = v13;
                                let a2002 = 4u64;
                                let a2003 = 0u64;
                                nat_rem(a2001, a2002, a2003)
                            };
                            let a2005 = 3u64;
                            if nat_lt(a2004, a2005) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2011 = {
                                            let a2007 = v10;
                                            let a2008 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2006, _, _) => {
                                                    h2006
                                                }
                                            };
                                            nat_mul(a2007, a2008)?
                                        };
                                        let a2012 = {
                                            let a2009 = 4u64;
                                            let a2010 = v14;
                                            nat_mul(a2009, a2010)?
                                        };
                                        nat_add(a2011, a2012)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2013 = v14;
                                    let a2014 = 0u64;
                                    if nat_eq(a2013, a2014) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2019 = v11;
                        let a2020 = {
                            let a2017 = {
                                let a2015 = 6u64;
                                let a2016 = v7;
                                nat_mul(a2015, a2016)?
                            };
                            let a2018 = 2u64;
                            nat_add(a2017, a2018)?
                        };
                        if nat_lt(a2019, a2020) {
                            let a2023 = v11;
                            let a2024 = {
                                let a2021 = 6u64;
                                let a2022 = v7;
                                nat_mul(a2021, a2022)?
                            };
                            if nat_eq(a2023, a2024) {
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
                                    let a2029 = v11;
                                    let a2030 = {
                                        let a2027 = {
                                            let a2025 = 6u64;
                                            let a2026 = v7;
                                            nat_mul(a2025, a2026)?
                                        };
                                        let a2028 = 2u64;
                                        nat_add(a2027, a2028)?
                                    };
                                    nat_sub(a2029, a2030)
                                };
                                let v16: u64 = {
                                    let a2031 = v15;
                                    let a2032 = 3u64;
                                    let a2033 = 0u64;
                                    nat_quot(a2031, a2032, a2033)
                                };
                                let a2037 = {
                                    let a2034 = v15;
                                    let a2035 = 3u64;
                                    let a2036 = 0u64;
                                    nat_rem(a2034, a2035, a2036)
                                };
                                let a2038 = 1u64;
                                if nat_lt(a2037, a2038) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2044 = {
                                            let a2040 = v10;
                                            let a2041 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2039, _, _) => {
                                                    h2039
                                                }
                                            };
                                            nat_mul(a2040, a2041)?
                                        };
                                        let a2045 = {
                                            let a2042 = 4u64;
                                            let a2043 = v16;
                                            nat_mul(a2042, a2043)?
                                        };
                                        nat_add(a2044, a2045)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2049 = {
                                        let a2046 = v15;
                                        let a2047 = 3u64;
                                        let a2048 = 0u64;
                                        nat_rem(a2046, a2047, a2048)
                                    };
                                    let a2050 = 2u64;
                                    if nat_lt(a2049, a2050) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2055 = {
                                                let a2051 = v10;
                                                let a2052 = v4;
                                                nat_mul(a2051, a2052)?
                                            };
                                            let a2056 = {
                                                let a2053 = 4u64;
                                                let a2054 = v16;
                                                nat_mul(a2053, a2054)?
                                            };
                                            nat_add(a2055, a2056)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2062 = {
                                                let a2058 = v10;
                                                let a2059 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2057, _, _) => {
                                                        h2057
                                                    }
                                                };
                                                nat_mul(a2058, a2059)?
                                            };
                                            let a2063 = {
                                                let a2060 = 4u64;
                                                let a2061 = v16;
                                                nat_mul(a2060, a2061)?
                                            };
                                            nat_add(a2062, a2063)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2068 = v11;
                                let a2069 = {
                                    let a2066 = {
                                        let a2064 = 11u64;
                                        let a2065 = v7;
                                        nat_mul(a2064, a2065)?
                                    };
                                    let a2067 = 2u64;
                                    nat_add(a2066, a2067)?
                                };
                                if nat_lt(a2068, a2069) {
                                    let v17: u64 = {
                                        let a2074 = v11;
                                        let a2075 = {
                                            let a2072 = {
                                                let a2070 = 6u64;
                                                let a2071 = v7;
                                                nat_mul(a2070, a2071)?
                                            };
                                            let a2073 = 2u64;
                                            nat_add(a2072, a2073)?
                                        };
                                        nat_sub(a2074, a2075)
                                    };
                                    let v18: u64 = {
                                        let a2076 = v17;
                                        let a2077 = 5u64;
                                        let a2078 = 0u64;
                                        nat_quot(a2076, a2077, a2078)
                                    };
                                    let a2082 = {
                                        let a2079 = v17;
                                        let a2080 = 5u64;
                                        let a2081 = 0u64;
                                        nat_rem(a2079, a2080, a2081)
                                    };
                                    let a2083 = 2u64;
                                    if nat_lt(a2082, a2083) {
                                        let a2087 = {
                                            let a2084 = v17;
                                            let a2085 = 5u64;
                                            let a2086 = 0u64;
                                            nat_rem(a2084, a2085, a2086)
                                        };
                                        let a2088 = 1u64;
                                        if nat_lt(a2087, a2088) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2094 = {
                                                    let a2090 = v10;
                                                    let a2091 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2089, _, _) => {
                                                            h2089
                                                        }
                                                    };
                                                    nat_mul(a2090, a2091)?
                                                };
                                                let a2095 = {
                                                    let a2092 = 4u64;
                                                    let a2093 = v18;
                                                    nat_mul(a2092, a2093)?
                                                };
                                                nat_add(a2094, a2095)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2099 = {
                                            let a2096 = v17;
                                            let a2097 = 5u64;
                                            let a2098 = 0u64;
                                            nat_rem(a2096, a2097, a2098)
                                        };
                                        let a2100 = 3u64;
                                        if nat_lt(a2099, a2100) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2105 = {
                                                    let a2101 = v10;
                                                    let a2102 = v4;
                                                    nat_mul(a2101, a2102)?
                                                };
                                                let a2106 = {
                                                    let a2103 = 4u64;
                                                    let a2104 = v18;
                                                    nat_mul(a2103, a2104)?
                                                };
                                                nat_add(a2105, a2106)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2110 = {
                                                let a2107 = v17;
                                                let a2108 = 5u64;
                                                let a2109 = 0u64;
                                                nat_rem(a2107, a2108, a2109)
                                            };
                                            let a2111 = 4u64;
                                            if nat_lt(a2110, a2111) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2112 = v18;
                                                    let a2113 = 0u64;
                                                    if nat_eq(a2112, a2113) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2118 = v11;
                                    let a2119 = {
                                        let a2116 = {
                                            let a2114 = 11u64;
                                            let a2115 = v7;
                                            nat_mul(a2114, a2115)?
                                        };
                                        let a2117 = 2u64;
                                        nat_add(a2116, a2117)?
                                    };
                                    if nat_eq(a2118, a2119) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2120 = v10;
                                            let a2121 = 0u64;
                                            if nat_eq(a2120, a2121) {
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

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2122, _, _, _, _, _, _, _) => {
            h2122
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2123, _, _, _, _, _, _, _) => {
            h2123
        }
    };
    let v6: u64 = f168(v4)?;
    let v7: u64 = {
        let a2124 = v0;
        let a2125 = v6;
        let a2126 = 0u64;
        nat_quot(a2124, a2125, a2126)
    };
    let v8: u64 = {
        let a2127 = v7;
        let a2128 = v5;
        let a2129 = 0u64;
        nat_quot(a2127, a2128, a2129)
    };
    let v9: u64 = {
        let a2130 = v7;
        let a2131 = v5;
        let a2132 = 0u64;
        nat_rem(a2130, a2131, a2132)
    };
    let v10: u64 = {
        let a2133 = v3;
        let a2134 = v4;
        let a2135 = 0u64;
        nat_rem(a2133, a2134, a2135)
    };
    let v11: u64 = {
        let a2140 = 4u64;
        let a2141 = {
            let a2138 = v8;
            let a2139 = {
                let a2136 = v4;
                let a2137 = v4;
                nat_mul(a2136, a2137)?
            };
            nat_mul(a2138, a2139)?
        };
        nat_mul(a2140, a2141)?
    };
    let v12: u64 = {
        let a2146 = 4u64;
        let a2147 = {
            let a2144 = v8;
            let a2145 = {
                let a2142 = v4;
                let a2143 = v5;
                nat_mul(a2142, a2143)?
            };
            nat_mul(a2144, a2145)?
        };
        nat_mul(a2146, a2147)?
    };
    let v13: u64 = {
        let a2151 = {
            let a2148 = v0;
            let a2149 = v6;
            let a2150 = 0u64;
            nat_rem(a2148, a2149, a2150)
        };
        let a2152 = f168(v10)?;
        nat_sub(a2151, a2152)
    };
    let a2155 = v13;
    let a2156 = {
        let a2153 = v10;
        let a2154 = 2u64;
        nat_mul(a2153, a2154)?
    };
    if nat_lt(a2155, a2156) {
        let v14: u64 = {
            let a2157 = v13;
            let a2158 = 2u64;
            let a2159 = 0u64;
            nat_quot(a2157, a2158, a2159)
        };
        let a2163 = {
            let a2160 = v13;
            let a2161 = 2u64;
            let a2162 = 0u64;
            nat_rem(a2160, a2161, a2162)
        };
        let a2164 = 0u64;
        if nat_eq(a2163, a2164) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2171 = v11;
                let a2172 = {
                    let a2169 = 4u64;
                    let a2170 = {
                        let a2167 = {
                            let a2165 = v10;
                            let a2166 = v4;
                            nat_mul(a2165, a2166)?
                        };
                        let a2168 = v14;
                        nat_add(a2167, a2168)?
                    };
                    nat_mul(a2169, a2170)?
                };
                nat_add(a2171, a2172)?
            }), Adt12::C3({
                let a2179 = v12;
                let a2180 = {
                    let a2177 = 4u64;
                    let a2178 = {
                        let a2175 = {
                            let a2173 = v14;
                            let a2174 = v5;
                            nat_mul(a2173, a2174)?
                        };
                        let a2176 = v9;
                        nat_add(a2175, a2176)?
                    };
                    nat_mul(a2177, a2178)?
                };
                nat_add(a2179, a2180)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2181 = v14;
                let a2182 = 0u64;
                if nat_eq(a2181, a2182) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2185 = v13;
        let a2186 = {
            let a2183 = v10;
            let a2184 = 2u64;
            nat_mul(a2183, a2184)?
        };
        if nat_eq(a2185, a2186) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2193 = v12;
                let a2194 = {
                    let a2191 = 4u64;
                    let a2192 = {
                        let a2189 = {
                            let a2187 = v10;
                            let a2188 = v5;
                            nat_mul(a2187, a2188)?
                        };
                        let a2190 = v9;
                        nat_add(a2189, a2190)?
                    };
                    nat_mul(a2191, a2192)?
                };
                nat_add(a2193, a2194)?
            }), {
                let a2195 = v10;
                let a2196 = 0u64;
                if nat_eq(a2195, a2196) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2201 = v13;
            let a2202 = {
                let a2199 = {
                    let a2197 = v10;
                    let a2198 = 2u64;
                    nat_mul(a2197, a2198)?
                };
                let a2200 = 1u64;
                nat_add(a2199, a2200)?
            };
            if nat_eq(a2201, a2202) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2209 = v11;
                    let a2210 = {
                        let a2207 = 4u64;
                        let a2208 = {
                            let a2205 = {
                                let a2203 = v10;
                                let a2204 = v4;
                                nat_mul(a2203, a2204)?
                            };
                            let a2206 = v10;
                            nat_add(a2205, a2206)?
                        };
                        nat_mul(a2207, a2208)?
                    };
                    nat_add(a2209, a2210)?
                }), Adt13::C1({
                    let a2217 = v12;
                    let a2218 = {
                        let a2215 = 4u64;
                        let a2216 = {
                            let a2213 = {
                                let a2211 = v10;
                                let a2212 = v5;
                                nat_mul(a2211, a2212)?
                            };
                            let a2214 = v9;
                            nat_add(a2213, a2214)?
                        };
                        nat_mul(a2215, a2216)?
                    };
                    nat_add(a2217, a2218)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2221 = {
                        let a2219 = v10;
                        let a2220 = 1u64;
                        nat_add(a2219, a2220)?
                    };
                    let a2222 = v4;
                    if nat_eq(a2221, a2222) {
                        0u64
                    } else {
                        let a2223 = v10;
                        let a2224 = 1u64;
                        nat_add(a2223, a2224)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f73(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2225 = v0;
    let a2226 = v1;
    if nat_lt(a2225, a2226) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f90(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2227 = v0;
            let a2228 = v1;
            nat_sub(a2227, a2228)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f90(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2235 = {
            let a2233 = 3u64;
            let a2234 = {
                let a2231 = match v4.clone() {
                    Adt6::C0(h2229, _, _, _) => {
                        h2229
                    }
                };
                let a2232 = match v5.clone() {
                    Adt6::C0(h2230, _, _, _) => {
                        h2230
                    }
                };
                nat_mul(a2231, a2232)?
            };
            nat_mul(a2233, a2234)?
        };
        let a2236 = 2u64;
        nat_add(a2235, a2236)?
    };
    let v8: u64 = {
        let a2237 = v0;
        let a2238 = v7;
        let a2239 = 0u64;
        nat_quot(a2237, a2238, a2239)
    };
    let v9: u64 = {
        let a2240 = v0;
        let a2241 = v7;
        let a2242 = 0u64;
        nat_rem(a2240, a2241, a2242)
    };
    let a2243 = v9;
    let a2244 = 0u64;
    if nat_eq(a2243, a2244) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2247 = {
            let a2245 = v9;
            let a2246 = 1u64;
            nat_add(a2245, a2246)?
        };
        let a2248 = v7;
        if nat_eq(a2247, a2248) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f90(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2249 = v9;
                let a2250 = 1u64;
                nat_sub(a2249, a2250)
            };
            let v11: u64 = {
                let a2251 = v10;
                let a2252 = 3u64;
                let a2253 = 0u64;
                nat_quot(a2251, a2252, a2253)
            };
            let v12: u64 = {
                let a2255 = v11;
                let a2256 = match v4.clone() {
                    Adt6::C0(h2254, _, _, _) => {
                        h2254
                    }
                };
                let a2257 = 0u64;
                nat_rem(a2255, a2256, a2257)
            };
            let v13: u64 = {
                let a2259 = v11;
                let a2260 = match v4.clone() {
                    Adt6::C0(h2258, _, _, _) => {
                        h2258
                    }
                };
                let a2261 = 0u64;
                nat_quot(a2259, a2260, a2261)
            };
            let v14: u64 = {
                let a2262 = v10;
                let a2263 = 3u64;
                let a2264 = 0u64;
                nat_rem(a2262, a2263, a2264)
            };
            let v15: u64 = f58(v8, v3.clone(), 2u64);
            let m2266 = f60(v4, match v2.clone() {
                Adt3::C0(h2265, _, _, _, _, _, _, _) => {
                    h2265
                }
            }, f58(v8, v3.clone(), 0u64), v12)?;
            match m2266 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2268 = f60(v5, match v2.clone() {
                        Adt3::C0(_, h2267, _, _, _, _, _, _) => {
                            h2267
                        }
                    }, f58(v8, v3.clone(), 1u64), v13)?;
                    match m2268 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2269 = v14;
                            let a2270 = 0u64;
                            if nat_eq(a2269, a2270) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f57(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f169(v6), 4u64))
                            } else {
                                let a2271 = v14;
                                let a2272 = 1u64;
                                if nat_eq(a2271, a2272) {
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

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f170(match v1.clone() {
        Adt3::C0(h2273, _, _, _, _, _, _, _) => {
            h2273
        }
    })?;
    let v5: u64 = {
        let a2274 = v0;
        let a2275 = v4;
        let a2276 = 0u64;
        nat_quot(a2274, a2275, a2276)
    };
    let v6: u64 = {
        let a2277 = v0;
        let a2278 = v4;
        let a2279 = 0u64;
        nat_rem(a2277, a2278, a2279)
    };
    let a2280 = v6;
    let a2281 = 0u64;
    if nat_eq(a2280, a2281) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2282 = v6;
        let a2283 = 1u64;
        if nat_eq(a2282, a2283) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2286 = {
                let a2284 = v6;
                let a2285 = 1u64;
                nat_add(a2284, a2285)?
            };
            let a2287 = v4;
            if nat_eq(a2286, a2287) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f90(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2288 = v6;
                    let a2289 = 2u64;
                    nat_sub(a2288, a2289)
                };
                let v8: u64 = {
                    let a2290 = v7;
                    let a2291 = 3u64;
                    let a2292 = 0u64;
                    nat_quot(a2290, a2291, a2292)
                };
                let v9: u64 = {
                    let a2293 = v7;
                    let a2294 = 3u64;
                    let a2295 = 0u64;
                    nat_rem(a2293, a2294, a2295)
                };
                let v10: u64 = f90({
                    let a2299 = {
                        let a2297 = v5;
                        let a2298 = match v1.clone() {
                            Adt3::C0(h2296, _, _, _, _, _, _, _) => {
                                h2296
                            }
                        };
                        nat_mul(a2297, a2298)?
                    };
                    let a2300 = v8;
                    nat_add(a2299, a2300)?
                }, v1)?;
                let a2301 = v9;
                let a2302 = 0u64;
                if nat_eq(a2301, a2302) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2303 = v9;
                    let a2304 = 1u64;
                    if nat_eq(a2303, a2304) {
                        let a2305 = v3;
                        let a2306 = 2u64;
                        if nat_eq(a2305, a2306) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2307 = v3;
                        let a2308 = 2u64;
                        if if nat_eq(a2307, a2308) {
                            true
                        } else {
                            let a2309 = v3;
                            let a2310 = 3u64;
                            nat_eq(a2309, a2310)
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

pub fn f76(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2312 = 2u64;
        let a2313 = match v2.clone() {
            Adt3::C0(_, h2311, _, _, _, _, _, _) => {
                h2311
            }
        };
        nat_mul(a2312, a2313)?
    };
    let a2314 = v1;
    let a2315 = v6;
    if nat_lt(a2314, a2315) {
        let a2319 = {
            let a2316 = v1;
            let a2317 = 2u64;
            let a2318 = 0u64;
            nat_rem(a2316, a2317, a2318)
        };
        let a2320 = 0u64;
        if nat_eq(a2319, a2320) {
            Ok(Adt9::C4(19u64, 4u64, f90({
                let a2321 = v1;
                let a2322 = 2u64;
                let a2323 = 0u64;
                nat_quot(a2321, a2322, a2323)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2324 = v5;
            let a2325 = 2147483648u64;
            if if nat_lt(a2324, a2325) {
                let a2327 = v5;
                let a2328 = match v3 {
                    Adt3::C0(_, h2326, _, _, _, _, _, _) => {
                        h2326
                    }
                };
                nat_lt(a2327, a2328)
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
            let a2331 = match v3.clone() {
                Adt3::C0(h2329, _, _, _, _, _, _, _) => {
                    h2329
                }
            };
            let a2332 = match v3.clone() {
                Adt3::C0(_, h2330, _, _, _, _, _, _) => {
                    h2330
                }
            };
            nat_mul(a2331, a2332)?
        };
        let a2335 = {
            let a2333 = v1;
            let a2334 = v6;
            nat_sub(a2333, a2334)
        };
        let a2336 = v7;
        if nat_lt(a2335, a2336) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f90({
                let a2337 = v1;
                let a2338 = v6;
                nat_sub(a2337, a2338)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2341 = {
                    let a2339 = v1;
                    let a2340 = v6;
                    nat_sub(a2339, a2340)
                };
                let a2342 = v7;
                nat_sub(a2341, a2342)
            };
            let v9: u64 = {
                let a2344 = 1u64;
                let a2345 = f171(v0, match v2.clone() {
                    Adt3::C0(h2343, _, _, _, _, _, _, _) => {
                        h2343
                    }
                })?;
                nat_add(a2344, a2345)?
            };
            let v10: u64 = {
                let a2346 = v8;
                let a2347 = v9;
                let a2348 = 0u64;
                nat_quot(a2346, a2347, a2348)
            };
            let v11: u64 = {
                let a2349 = v8;
                let a2350 = v9;
                let a2351 = 0u64;
                nat_rem(a2349, a2350, a2351)
            };
            let a2352 = v11;
            let a2353 = 0u64;
            if nat_eq(a2352, a2353) {
                Ok(Adt9::C4(19u64, 4u64, f90(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f172(v0) {
                    let a2357 = {
                        let a2354 = v11;
                        let a2355 = 2u64;
                        let a2356 = 0u64;
                        nat_rem(a2354, a2355, a2356)
                    };
                    let a2358 = 1u64;
                    if nat_eq(a2357, a2358) {
                        Ok(Adt9::C4(6u64, 0u64, f90({
                            let a2367 = {
                                let a2360 = v10;
                                let a2361 = match v2.clone() {
                                    Adt3::C0(h2359, _, _, _, _, _, _, _) => {
                                        h2359
                                    }
                                };
                                nat_mul(a2360, a2361)?
                            };
                            let a2368 = {
                                let a2364 = {
                                    let a2362 = v11;
                                    let a2363 = 1u64;
                                    nat_sub(a2362, a2363)
                                };
                                let a2365 = 2u64;
                                let a2366 = 0u64;
                                nat_quot(a2364, a2365, a2366)
                            };
                            nat_add(a2367, a2368)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f90({
                            let a2377 = {
                                let a2370 = v5;
                                let a2371 = match v2.clone() {
                                    Adt3::C0(h2369, _, _, _, _, _, _, _) => {
                                        h2369
                                    }
                                };
                                nat_mul(a2370, a2371)?
                            };
                            let a2378 = {
                                let a2374 = {
                                    let a2372 = v11;
                                    let a2373 = 1u64;
                                    nat_sub(a2372, a2373)
                                };
                                let a2375 = 2u64;
                                let a2376 = 0u64;
                                nat_quot(a2374, a2375, a2376)
                            };
                            nat_add(a2377, a2378)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f90({
                            let a2387 = {
                                let a2380 = v5;
                                let a2381 = match v2 {
                                    Adt3::C0(h2379, _, _, _, _, _, _, _) => {
                                        h2379
                                    }
                                };
                                nat_mul(a2380, a2381)?
                            };
                            let a2388 = {
                                let a2384 = {
                                    let a2382 = v11;
                                    let a2383 = 1u64;
                                    nat_sub(a2382, a2383)
                                };
                                let a2385 = 2u64;
                                let a2386 = 0u64;
                                nat_quot(a2384, a2385, a2386)
                            };
                            nat_add(a2387, a2388)?
                        }, v3)?))
                    }
                } else {
                    f173(v0, v10, {
                        let a2389 = v11;
                        let a2390 = 1u64;
                        nat_sub(a2389, a2390)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f77(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2391 = v1;
    let a2392 = f174(v2.clone(), v4.clone())?;
    if nat_lt(a2391, a2392) {
        let a2395 = v1;
        let a2396 = {
            let a2393 = 2u64;
            let a2394 = f175(v4.clone())?;
            nat_mul(a2393, a2394)?
        };
        if nat_lt(a2395, a2396) {
            let a2400 = {
                let a2397 = v1;
                let a2398 = 2u64;
                let a2399 = 0u64;
                nat_rem(a2397, a2398, a2399)
            };
            let a2401 = 0u64;
            if nat_eq(a2400, a2401) {
                f176({
                    let a2402 = v1;
                    let a2403 = 2u64;
                    let a2404 = 0u64;
                    nat_quot(a2402, a2403, a2404)
                }, v4)
            } else {
                if f177(match v2 {
                    Adt3::C0(_, h2405, _, _, _, _, _, _) => {
                        h2405
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2408 = v1;
                let a2409 = {
                    let a2406 = 2u64;
                    let a2407 = f175(v4.clone())?;
                    nat_mul(a2406, a2407)?
                };
                nat_sub(a2408, a2409)
            };
            let v7: u64 = {
                let a2413 = v6;
                let a2414 = {
                    let a2411 = 1u64;
                    let a2412 = match v2.clone() {
                        Adt3::C0(h2410, _, _, _, _, _, _, _) => {
                            h2410
                        }
                    };
                    nat_add(a2411, a2412)?
                };
                let a2415 = 0u64;
                nat_quot(a2413, a2414, a2415)
            };
            let v8: u64 = {
                let a2419 = v6;
                let a2420 = {
                    let a2417 = 1u64;
                    let a2418 = match v2.clone() {
                        Adt3::C0(h2416, _, _, _, _, _, _, _) => {
                            h2416
                        }
                    };
                    nat_add(a2417, a2418)?
                };
                let a2421 = 0u64;
                nat_rem(a2419, a2420, a2421)
            };
            let a2422 = v8;
            let a2423 = 0u64;
            if nat_eq(a2422, a2423) {
                f176(v7, v4)
            } else {
                f178(v0, v7, {
                    let a2424 = v8;
                    let a2425 = 1u64;
                    nat_sub(a2424, a2425)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2428 = match v4.clone() {
            Adt3::C0(h2426, _, _, _, _, _, _, _) => {
                h2426
            }
        };
        let a2429 = match v4.clone() {
            Adt3::C0(_, h2427, _, _, _, _, _, _) => {
                h2427
            }
        };
        nat_mul(a2428, a2429)?
    };
    let v7: u64 = {
        let a2430 = 2u64;
        let a2431 = v6;
        nat_mul(a2430, a2431)?
    };
    let v8: u64 = f179(match v3.clone() {
        Adt3::C0(_, h2432, _, _, _, _, _, _) => {
            h2432
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2433, _, _, _, _, _) => {
            h2433
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2434, _, _, _, _) => {
            h2434
        }
    });
    let a2435 = v0;
    let a2436 = v7;
    if nat_lt(a2435, a2436) {
        let a2440 = {
            let a2437 = v0;
            let a2438 = 2u64;
            let a2439 = 0u64;
            nat_rem(a2437, a2438, a2439)
        };
        let a2441 = 0u64;
        if nat_eq(a2440, a2441) {
            Ok(Adt9::C4(19u64, 5u64, f90({
                let a2442 = v0;
                let a2443 = 2u64;
                let a2444 = 0u64;
                nat_quot(a2442, a2443, a2444)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f180(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2445 = v0;
            let a2446 = v7;
            nat_sub(a2445, a2446)
        };
        let v10: u64 = {
            let a2450 = v9;
            let a2451 = {
                let a2448 = 1u64;
                let a2449 = match v1.clone() {
                    Adt3::C0(h2447, _, _, _, _, _, _, _) => {
                        h2447
                    }
                };
                nat_add(a2448, a2449)?
            };
            let a2452 = 0u64;
            nat_quot(a2450, a2451, a2452)
        };
        let v11: u64 = {
            let a2456 = v9;
            let a2457 = {
                let a2454 = 1u64;
                let a2455 = match v1.clone() {
                    Adt3::C0(h2453, _, _, _, _, _, _, _) => {
                        h2453
                    }
                };
                nat_add(a2454, a2455)?
            };
            let a2458 = 0u64;
            nat_rem(a2456, a2457, a2458)
        };
        let a2459 = v11;
        let a2460 = 0u64;
        if nat_eq(a2459, a2460) {
            Ok(Adt9::C4(19u64, 5u64, f90({
                let a2461 = v10;
                let a2462 = v6;
                let a2463 = 0u64;
                nat_rem(a2461, a2462, a2463)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f180(v5, v8) {
                let v12: u64 = {
                    let a2469 = {
                        let a2465 = v10;
                        let a2466 = match v1.clone() {
                            Adt3::C0(h2464, _, _, _, _, _, _, _) => {
                                h2464
                            }
                        };
                        nat_mul(a2465, a2466)?
                    };
                    let a2470 = {
                        let a2467 = v11;
                        let a2468 = 1u64;
                        nat_sub(a2467, a2468)
                    };
                    nat_add(a2469, a2470)?
                };
                let a2471 = v5;
                let a2472 = 4294967295u64;
                if nat_eq(a2471, a2472) {
                    Ok(Adt9::C1(false, f90(v12, v1)?, f90(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f90(v12, v1)?, 4u64, f90({
                        let a2478 = {
                            let a2474 = v5;
                            let a2475 = match v3.clone() {
                                Adt3::C0(h2473, _, _, _, _, _, _, _) => {
                                    h2473
                                }
                            };
                            nat_mul(a2474, a2475)?
                        };
                        let a2479 = {
                            let a2476 = v11;
                            let a2477 = 1u64;
                            nat_sub(a2476, a2477)
                        };
                        nat_add(a2478, a2479)?
                    }, v3)?, 3u64, f90(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f181(v3.clone())?;
    let v6: u64 = {
        let a2480 = v0;
        let a2481 = v5;
        let a2482 = 0u64;
        nat_quot(a2480, a2481, a2482)
    };
    let v7: u64 = {
        let a2483 = v0;
        let a2484 = v5;
        let a2485 = 0u64;
        nat_rem(a2483, a2484, a2485)
    };
    let v8: u64 = f182(v3.clone())?;
    let a2486 = v7;
    let a2487 = 0u64;
    if nat_eq(a2486, a2487) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2488, _) => {
                h2488
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2492 = v5;
        let a2493 = {
            let a2490 = v7;
            let a2491 = if match v3.clone() {
                Adt7::C0(_, _, _, h2489, _) => {
                    h2489
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2490, a2491)?
        };
        if nat_le(a2492, a2493) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2494, _) => {
                    h2494
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f90(v6, v2)?))
            } else {
                let a2497 = {
                    let a2495 = v7;
                    let a2496 = 2u64;
                    nat_add(a2495, a2496)?
                };
                let a2498 = v5;
                if nat_eq(a2497, a2498) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2499, _, _) => {
                            h2499
                        }
                    } {
                        let a2506 = {
                            let m2502 = match v3.clone() {
                                Adt7::C0(h2501, _, _, _, _) => {
                                    h2501
                                }
                            };
                            match m2502 {
                                Adt6::C0(h2500, _, _, _) => {
                                    h2500
                                }
                            }
                        };
                        let a2507 = {
                            let m2505 = match v3 {
                                Adt7::C0(_, h2504, _, _, _) => {
                                    h2504
                                }
                            };
                            match m2505 {
                                Adt6::C0(h2503, _, _, _) => {
                                    h2503
                                }
                            }
                        };
                        nat_mul(a2506, a2507)?
                    } else {
                        f183(match v3 {
                            Adt7::C0(h2508, _, _, _, _) => {
                                h2508
                            }
                        }, match v1 {
                            Adt3::C0(h2509, _, _, _, _, _, _, _) => {
                                h2509
                            }
                        }, f58(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2510 = v9;
                        let a2511 = 0u64;
                        if nat_eq(a2510, a2511) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f90(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2514 = {
                    let a2512 = v7;
                    let a2513 = 1u64;
                    nat_sub(a2512, a2513)
                };
                let a2515 = v8;
                let a2516 = 0u64;
                nat_rem(a2514, a2515, a2516)
            };
            let m2522 = f184(v3.clone(), v1, v2, v6, {
                let a2519 = {
                    let a2517 = v7;
                    let a2518 = 1u64;
                    nat_sub(a2517, a2518)
                };
                let a2520 = v8;
                let a2521 = 0u64;
                nat_quot(a2519, a2520, a2521)
            })?;
            match m2522 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2524 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2523) => {
                            h2523
                        }
                    };
                    let a2525 = 1u64;
                    if if nat_eq(a2524, a2525) {
                        let a2526 = v10;
                        let a2527 = 0u64;
                        nat_eq(a2526, a2527)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2528, _) => {
                                h2528
                            }
                        } {
                            let a2531 = {
                                let a2529 = v10;
                                let a2530 = 1u64;
                                nat_add(a2529, a2530)?
                            };
                            let a2532 = v8;
                            if nat_eq(a2531, a2532) {
                                let a2533 = v4;
                                let a2534 = 2u64;
                                if nat_eq(a2533, a2534) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2536 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2535) => {
                                                h2535
                                            }
                                        };
                                        let a2537 = 1u64;
                                        if nat_eq(a2536, a2537) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2539 = match v3 {
                                            Adt7::C0(_, _, _, _, h2538) => {
                                                h2538
                                            }
                                        };
                                        let a2540 = 1u64;
                                        if nat_eq(a2539, a2540) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2542 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2541) => {
                                            h2541
                                        }
                                    };
                                    let a2543 = 1u64;
                                    if nat_eq(a2542, a2543) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2545 = match v3 {
                                        Adt7::C0(_, _, _, _, h2544) => {
                                            h2544
                                        }
                                    };
                                    let a2546 = 1u64;
                                    if nat_eq(a2545, a2546) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2548 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2547) => {
                                        h2547
                                    }
                                };
                                let a2549 = 1u64;
                                if nat_eq(a2548, a2549) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2551 = match v3 {
                                    Adt7::C0(_, _, _, _, h2550) => {
                                        h2550
                                    }
                                };
                                let a2552 = 1u64;
                                if nat_eq(a2551, a2552) {
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

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f185(match v3.clone() {
        Adt3::C0(h2553, _, _, _, _, _, _, _) => {
            h2553
        }
    })?;
    let v5: u64 = {
        let a2554 = v0;
        let a2555 = v4;
        let a2556 = 0u64;
        nat_rem(a2554, a2555, a2556)
    };
    let v6: u64 = {
        let a2557 = v0;
        let a2558 = v4;
        let a2559 = 0u64;
        nat_quot(a2557, a2558, a2559)
    };
    let a2560 = v5;
    let a2561 = 0u64;
    if nat_eq(a2560, a2561) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2564 = {
            let a2562 = v5;
            let a2563 = 1u64;
            nat_add(a2562, a2563)?
        };
        let a2565 = v4;
        if nat_eq(a2564, a2565) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f90(v6, v2)?))
        } else {
            let a2569 = {
                let a2566 = v5;
                let a2567 = 2u64;
                let a2568 = 0u64;
                nat_rem(a2566, a2567, a2568)
            };
            let a2570 = 0u64;
            if nat_eq(a2569, a2570) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2573 = {
                        let a2571 = v5;
                        let a2572 = 1u64;
                        nat_sub(a2571, a2572)
                    };
                    let a2574 = 2u64;
                    let a2575 = 0u64;
                    nat_quot(a2573, a2574, a2575)
                };
                let v8: u64 = f58(v6, v2.clone(), 0u64);
                let v9: u64 = f58(v6, v2.clone(), 1u64);
                let v10: u64 = f58(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f57(v1, {
                    let a2576 = v9;
                    let a2577 = v7;
                    nat_add(a2576, a2577)?
                }, v8, v10, 0u64)?, 4u64, f57(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f186(v3.clone())?;
    let v5: u64 = {
        let a2578 = v0;
        let a2579 = v4;
        let a2580 = 0u64;
        nat_rem(a2578, a2579, a2580)
    };
    let v6: u64 = {
        let a2581 = v0;
        let a2582 = v4;
        let a2583 = 0u64;
        nat_quot(a2581, a2582, a2583)
    };
    let a2584 = v5;
    let a2585 = 0u64;
    if nat_eq(a2584, a2585) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2588 = {
            let a2586 = v5;
            let a2587 = 1u64;
            nat_add(a2586, a2587)?
        };
        let a2589 = v4;
        if nat_eq(a2588, a2589) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f90(v6, v2)?))
        } else {
            let a2593 = {
                let a2590 = v5;
                let a2591 = 2u64;
                let a2592 = 0u64;
                nat_rem(a2590, a2591, a2592)
            };
            let a2594 = 0u64;
            if nat_eq(a2593, a2594) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2596 = f58(v6, v2.clone(), 0u64);
                    let a2597 = match v3.clone() {
                        Adt6::C0(_, _, h2595, _) => {
                            h2595
                        }
                    };
                    nat_add(a2596, a2597)?
                };
                let v8: u64 = {
                    let a2605 = {
                        let a2602 = {
                            let a2599 = v7;
                            let a2600 = match v3.clone() {
                                Adt6::C0(_, h2598, _, _) => {
                                    h2598
                                }
                            };
                            nat_add(a2599, a2600)?
                        };
                        let a2603 = match v3.clone() {
                            Adt6::C0(h2601, _, _, _) => {
                                h2601
                            }
                        };
                        nat_sub(a2602, a2603)
                    };
                    let a2606 = match v3.clone() {
                        Adt6::C0(_, h2604, _, _) => {
                            h2604
                        }
                    };
                    let a2607 = 0u64;
                    nat_quot(a2605, a2606, a2607)
                };
                let v9: u64 = {
                    let a2613 = v8;
                    let a2614 = {
                        let a2610 = {
                            let a2608 = v5;
                            let a2609 = 1u64;
                            nat_sub(a2608, a2609)
                        };
                        let a2611 = 2u64;
                        let a2612 = 0u64;
                        nat_quot(a2610, a2611, a2612)
                    };
                    nat_add(a2613, a2614)?
                };
                let v10: u64 = {
                    let a2616 = v9;
                    let a2617 = match v3.clone() {
                        Adt6::C0(_, h2615, _, _) => {
                            h2615
                        }
                    };
                    nat_mul(a2616, a2617)?
                };
                let v11: u64 = {
                    let a2618 = v7;
                    let a2619 = v10;
                    nat_sub(a2618, a2619)
                };
                let a2621 = v9;
                let a2622 = match v1.clone() {
                    Adt3::C0(_, h2620, _, _, _, _, _, _) => {
                        h2620
                    }
                };
                if if if nat_lt(a2621, a2622) {
                    let a2623 = v10;
                    let a2624 = v7;
                    nat_le(a2623, a2624)
                } else {
                    false
                } {
                    let a2626 = v11;
                    let a2627 = match v3.clone() {
                        Adt6::C0(h2625, _, _, _) => {
                            h2625
                        }
                    };
                    nat_lt(a2626, a2627)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f57(v1, {
                        let a2631 = {
                            let a2629 = f58(v6, v2, 1u64);
                            let a2630 = match v3 {
                                Adt6::C0(h2628, _, _, _) => {
                                    h2628
                                }
                            };
                            nat_mul(a2629, a2630)?
                        };
                        let a2632 = v11;
                        nat_add(a2631, a2632)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2634 = v5;
        let a2635 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2633, _) => {
                h2633
            }
        };
        let a2636 = 0u64;
        nat_rem(a2634, a2635, a2636)
    };
    let v7: u64 = {
        let a2638 = v5;
        let a2639 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2637, _) => {
                h2637
            }
        };
        let a2640 = 0u64;
        nat_quot(a2638, a2639, a2640)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2641, _, _, _, _, _, _, _) => {
            h2641
        }
    }, match v1.clone() {
        Adt3::C0(h2642, _, _, _, _, _, _, _) => {
            h2642
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a2646 = v4;
        let a2647 = {
            let m2645 = match v3.clone() {
                Adt5::C0(h2644, _, _, _, _, _, _, _) => {
                    h2644
                }
            };
            match m2645 {
                Adt6::C0(h2643, _, _, _) => {
                    h2643
                }
            }
        };
        let a2648 = 0u64;
        nat_rem(a2646, a2647, a2648)
    })?;
    let v9: u64 = {
        let a2652 = v4;
        let a2653 = {
            let m2651 = match v3.clone() {
                Adt5::C0(h2650, _, _, _, _, _, _, _) => {
                    h2650
                }
            };
            match m2651 {
                Adt6::C0(h2649, _, _, _) => {
                    h2649
                }
            }
        };
        let a2654 = 0u64;
        nat_quot(a2652, a2653, a2654)
    };
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2655, _, _, _, _, _, _) => {
            h2655
        }
    }, match v1.clone() {
        Adt3::C0(_, h2656, _, _, _, _, _, _) => {
            h2656
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a2660 = v9;
        let a2661 = {
            let m2659 = match v3.clone() {
                Adt5::C0(_, h2658, _, _, _, _, _, _) => {
                    h2658
                }
            };
            match m2659 {
                Adt6::C0(h2657, _, _, _) => {
                    h2657
                }
            }
        };
        let a2662 = 0u64;
        nat_rem(a2660, a2661, a2662)
    })?;
    let v11: u64 = {
        let a2666 = v9;
        let a2667 = {
            let m2665 = match v3.clone() {
                Adt5::C0(_, h2664, _, _, _, _, _, _) => {
                    h2664
                }
            };
            match m2665 {
                Adt6::C0(h2663, _, _, _) => {
                    h2663
                }
            }
        };
        let a2668 = 0u64;
        nat_quot(a2666, a2667, a2668)
    };
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2669, _, _, _, _, _) => {
            h2669
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2670, _, _, _, _, _) => {
            h2670
        }
    }, v6, {
        let a2674 = v11;
        let a2675 = {
            let m2673 = match v3.clone() {
                Adt5::C0(_, _, h2672, _, _, _, _, _) => {
                    h2672
                }
            };
            match m2673 {
                Adt6::C0(h2671, _, _, _) => {
                    h2671
                }
            }
        };
        let a2676 = 0u64;
        nat_rem(a2674, a2675, a2676)
    })?;
    let v13: u64 = {
        let a2680 = v11;
        let a2681 = {
            let m2679 = match v3.clone() {
                Adt5::C0(_, _, h2678, _, _, _, _, _) => {
                    h2678
                }
            };
            match m2679 {
                Adt6::C0(h2677, _, _, _) => {
                    h2677
                }
            }
        };
        let a2682 = 0u64;
        nat_quot(a2680, a2681, a2682)
    };
    let v17: Adt12 = {
        let m2683 = v8;
        match m2683 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2684 = v10;
                match m2684 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2685 = v12;
                        match m2685 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2694 = {
                                        let a2689 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a2690 = {
                                            let a2687 = v7;
                                            let a2688 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2686, _, _, _) => {
                                                    h2686
                                                }
                                            };
                                            nat_mul(a2687, a2688)?
                                        };
                                        nat_add(a2689, a2690)?
                                    };
                                    let a2695 = {
                                        let a2692 = v13;
                                        let a2693 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2691, _, _, _, _) => {
                                                h2691
                                            }
                                        };
                                        nat_mul(a2692, a2693)?
                                    };
                                    nat_add(a2694, a2695)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2696, _, _) => {
            h2696
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2697) => {
            h2697
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f90(v0, v2)?), {
        let a2699 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2698, _, _) => {
                h2698
            }
        };
        let a2700 = 0u64;
        if nat_eq(a2699, a2700) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2702 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2701) => {
                h2701
            }
        };
        let a2703 = 0u64;
        if nat_eq(a2702, a2703) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f83(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2716 = f187(v0, v1, match v2.clone() {
        Adt3::C0(h2704, _, _, _, _, _, _, _) => {
            h2704
        }
    }, match v2.clone() {
        Adt3::C0(_, h2705, _, _, _, _, _, _) => {
            h2705
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2706, _, _, _, _, _) => {
            h2706
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2707, _, _, _, _) => {
            h2707
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2708, _, _, _) => {
            h2708
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2709, _, _) => {
            h2709
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2710, _) => {
            h2710
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2711) => {
            h2711
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2712, _, _, _) => {
            h2712
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2713, _, _) => {
            h2713
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2714, _) => {
            h2714
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2715) => {
            h2715
        }
    }, v4)?;
    match m2716 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f20(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2719 = v6;
        let a2720 = {
            let a2717 = 3u64;
            let a2718 = v5;
            nat_add(a2717, a2718)?
        };
        nat_sub(a2719, a2720)
    };
    let a2722 = v0;
    let a2723 = f50(v3, match v1.clone() {
        Adt3::C0(h2721, _, _, _, _, _, _, _) => {
            h2721
        }
    }, v5, v8)?;
    if nat_lt(a2722, a2723) {
        let v9: u64 = f99(match v1.clone() {
            Adt3::C0(h2724, _, _, _, _, _, _, _) => {
                h2724
            }
        }, v5, v8)?;
        let v10: u64 = f100(v3, match v1.clone() {
            Adt3::C0(h2725, _, _, _, _, _, _, _) => {
                h2725
            }
        }, v5)?;
        let a2726 = v0;
        let a2727 = v9;
        if nat_lt(a2726, a2727) {
            f188(v0, v2, f44(v3), {
                let a2728 = 3u64;
                let a2729 = v5;
                nat_add(a2728, a2729)?
            }, v7)
        } else {
            let a2732 = v0;
            let a2733 = {
                let a2730 = v9;
                let a2731 = v10;
                nat_add(a2730, a2731)?
            };
            if nat_lt(a2732, a2733) {
                let v11: u64 = {
                    let a2734 = v0;
                    let a2735 = v9;
                    nat_sub(a2734, a2735)
                };
                let v12: u64 = f189(v3, match v1.clone() {
                    Adt3::C0(h2736, _, _, _, _, _, _, _) => {
                        h2736
                    }
                })?;
                f190({
                    let a2737 = v11;
                    let a2738 = v12;
                    let a2739 = 0u64;
                    nat_rem(a2737, a2738, a2739)
                }, {
                    let a2740 = v11;
                    let a2741 = v12;
                    let a2742 = 0u64;
                    nat_quot(a2740, a2741, a2742)
                }, v1, v3, v4, v7)
            } else {
                f191({
                    let a2745 = v0;
                    let a2746 = {
                        let a2743 = v9;
                        let a2744 = v10;
                        nat_add(a2743, a2744)?
                    };
                    nat_sub(a2745, a2746)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f85(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2747 = v0;
    let a2748 = f36(v1)?;
    if nat_lt(a2747, a2748) {
        let a2749 = v0;
        let a2750 = 4u64;
        if nat_lt(a2749, a2750) {
            let a2751 = v0;
            let a2752 = 0u64;
            if nat_eq(a2751, a2752) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2753 = v0;
                let a2754 = 1u64;
                if nat_eq(a2753, a2754) {
                    let a2755 = v5;
                    let a2756 = 0u64;
                    if nat_eq(a2755, a2756) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2757 = v0;
                    let a2758 = 2u64;
                    if nat_eq(a2757, a2758) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2759 = v0;
            let a2760 = 4u64;
            if if nat_eq(a2759, a2760) {
                let a2761 = f192(v5)?;
                let a2762 = v1;
                !nat_eq(a2761, a2762)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2765 = {
                        let a2763 = v0;
                        let a2764 = 4u64;
                        nat_sub(a2763, a2764)
                    };
                    let a2766 = 3u64;
                    let a2767 = 0u64;
                    nat_quot(a2765, a2766, a2767)
                };
                let v7: u64 = {
                    let a2770 = {
                        let a2768 = v0;
                        let a2769 = 4u64;
                        nat_sub(a2768, a2769)
                    };
                    let a2771 = 3u64;
                    let a2772 = 0u64;
                    nat_rem(a2770, a2771, a2772)
                };
                let a2773 = v7;
                let a2774 = 0u64;
                if nat_eq(a2773, a2774) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2775 = v7;
                    let a2776 = 1u64;
                    if nat_eq(a2775, a2776) {
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

pub fn f86(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2777 = v1;
        let a2778 = 5u64;
        nat_eq(a2777, a2778)
    } else {
        false
    } {
        let a2779 = v2;
        let a2780 = 16u64;
        nat_eq(a2779, a2780)
    } else {
        false
    } {
        let a2781 = v3;
        let a2782 = 0u64;
        nat_eq(a2781, a2782)
    } else {
        false
    } {
        let a2783 = v4;
        let a2784 = 0u64;
        nat_eq(a2783, a2784)
    } else {
        false
    }
}

pub fn f87(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2785 = v1;
    let a2786 = 0u64;
    if nat_eq(a2785, a2786) {
        if v0 {
            let a2787 = v3;
            let a2788 = 0u64;
            if if if nat_eq(a2787, a2788) {
                let a2789 = v4;
                let a2790 = 43u64;
                nat_eq(a2789, a2790)
            } else {
                false
            } {
                true
            } else {
                let a2791 = v3;
                let a2792 = 43u64;
                if nat_eq(a2791, a2792) {
                    let a2793 = v4;
                    let a2794 = 0u64;
                    nat_eq(a2793, a2794)
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
        let a2795 = v1;
        let a2796 = 1u64;
        if nat_eq(a2795, a2796) {
            Adt11::C1(v2)
        } else {
            let a2797 = v1;
            let a2798 = 2u64;
            if nat_eq(a2797, a2798) {
                Adt11::C5(v2)
            } else {
                let a2799 = v1;
                let a2800 = 3u64;
                if nat_eq(a2799, a2800) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2801 = v1;
                    let a2802 = 4u64;
                    if nat_eq(a2801, a2802) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f86(v0, v1, v2, v3, v4) {
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

pub fn f88(v0: u64, v1: u64) -> Adt12 {
    let a2803 = v0;
    let a2804 = 0u64;
    if nat_eq(a2803, a2804) {
        Adt12::C0(0u64, v1)
    } else {
        let a2805 = v0;
        let a2806 = 1u64;
        if nat_eq(a2805, a2806) {
            Adt12::C1(v1)
        } else {
            let a2807 = v0;
            let a2808 = 2u64;
            if nat_eq(a2807, a2808) {
                Adt12::C2(v1)
            } else {
                let a2809 = v0;
                let a2810 = 3u64;
                if nat_eq(a2809, a2810) {
                    Adt12::C3(v1)
                } else {
                    let a2811 = v0;
                    let a2812 = 13u64;
                    if nat_lt(a2811, a2812) {
                        Adt12::C0({
                            let a2813 = v0;
                            let a2814 = 3u64;
                            nat_sub(a2813, a2814)
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
    let a2815 = v0;
    let a2816 = 1u64;
    if nat_eq(a2815, a2816) {
        Adt13::C0(v1)
    } else {
        let a2817 = v0;
        let a2818 = 3u64;
        if if nat_eq(a2817, a2818) {
            true
        } else {
            let a2819 = v0;
            let a2820 = 6u64;
            nat_eq(a2819, a2820)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f90(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2822 = v0;
        let a2823 = match v1.clone() {
            Adt3::C0(h2821, _, _, _, _, _, _, _) => {
                h2821
            }
        };
        let a2824 = 0u64;
        nat_quot(a2822, a2823, a2824)
    };
    let v3: u64 = {
        let a2826 = v2;
        let a2827 = match v1.clone() {
            Adt3::C0(_, h2825, _, _, _, _, _, _) => {
                h2825
            }
        };
        let a2828 = 0u64;
        nat_quot(a2826, a2827, a2828)
    };
    f138({
        let a2830 = v0;
        let a2831 = match v1.clone() {
            Adt3::C0(h2829, _, _, _, _, _, _, _) => {
                h2829
            }
        };
        let a2832 = 0u64;
        nat_rem(a2830, a2831, a2832)
    }, {
        let a2834 = v2;
        let a2835 = match v1.clone() {
            Adt3::C0(_, h2833, _, _, _, _, _, _) => {
                h2833
            }
        };
        let a2836 = 0u64;
        nat_rem(a2834, a2835, a2836)
    }, {
        let a2838 = v3;
        let a2839 = match v1.clone() {
            Adt3::C0(_, _, h2837, _, _, _, _, _) => {
                h2837
            }
        };
        let a2840 = 0u64;
        nat_rem(a2838, a2839, a2840)
    }, {
        let a2842 = v3;
        let a2843 = match v1.clone() {
            Adt3::C0(_, _, h2841, _, _, _, _, _) => {
                h2841
            }
        };
        let a2844 = 0u64;
        nat_quot(a2842, a2843, a2844)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2845, _, _, _) => {
            h2845
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2846, _, _) => {
            h2846
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2847, _) => {
            h2847
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2848) => {
            h2848
        }
    })
}

pub fn f91(v0: Adt6, v1: u64) -> R<u64> {
    let a2850 = 0u64;
    let a2851 = match v0.clone() {
        Adt6::C0(h2849, _, _, _) => {
            h2849
        }
    };
    if if if if if nat_lt(a2850, a2851) {
        let a2853 = 0u64;
        let a2854 = match v0.clone() {
            Adt6::C0(_, h2852, _, _) => {
                h2852
            }
        };
        nat_lt(a2853, a2854)
    } else {
        false
    } {
        let a2856 = 0u64;
        let a2857 = match v0.clone() {
            Adt6::C0(_, _, _, h2855) => {
                h2855
            }
        };
        nat_lt(a2856, a2857)
    } else {
        false
    } {
        let a2859 = match v0.clone() {
            Adt6::C0(_, h2858, _, _) => {
                h2858
            }
        };
        let a2860 = 2147483648u64;
        nat_lt(a2859, a2860)
    } else {
        false
    } {
        let a2862 = match v0.clone() {
            Adt6::C0(_, _, _, h2861) => {
                h2861
            }
        };
        let a2863 = 2147483648u64;
        nat_lt(a2862, a2863)
    } else {
        false
    } {
        let v2: u64 = {
            let a2867 = v1;
            let a2868 = {
                let a2865 = 2u64;
                let a2866 = f140(match v0.clone() {
                    Adt6::C0(_, _, h2864, _) => {
                        h2864
                    }
                });
                nat_mul(a2865, a2866)?
            };
            nat_add(a2867, a2868)?
        };
        let v3: u64 = {
            let a2880 = {
                let a2875 = 1u64;
                let a2876 = {
                    let a2873 = match v0.clone() {
                        Adt6::C0(_, _, _, h2869) => {
                            h2869
                        }
                    };
                    let a2874 = {
                        let a2871 = match v0.clone() {
                            Adt6::C0(h2870, _, _, _) => {
                                h2870
                            }
                        };
                        let a2872 = 1u64;
                        nat_sub(a2871, a2872)
                    };
                    nat_mul(a2873, a2874)?
                };
                nat_add(a2875, a2876)?
            };
            let a2881 = {
                let a2878 = 2u64;
                let a2879 = f139(match v0.clone() {
                    Adt6::C0(_, _, h2877, _) => {
                        h2877
                    }
                });
                nat_mul(a2878, a2879)?
            };
            nat_add(a2880, a2881)?
        };
        let a2882 = v3;
        let a2883 = v2;
        if nat_le(a2882, a2883) {
            let a2890 = 1u64;
            let a2891 = {
                let a2887 = {
                    let a2884 = v2;
                    let a2885 = v3;
                    nat_sub(a2884, a2885)
                };
                let a2888 = match v0 {
                    Adt6::C0(_, h2886, _, _) => {
                        h2886
                    }
                };
                let a2889 = 0u64;
                nat_quot(a2887, a2888, a2889)
            };
            nat_add(a2890, a2891)
        } else {
            let a2895 = {
                let a2892 = v3;
                let a2893 = v2;
                nat_sub(a2892, a2893)
            };
            let a2896 = match v0 {
                Adt6::C0(_, h2894, _, _) => {
                    h2894
                }
            };
            if nat_lt(a2895, a2896) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f92(v0: u64) -> u64 {
    let v1: u64 = {
        let a2897 = v0;
        let a2898 = 4294967296u64;
        let a2899 = 0u64;
        nat_rem(a2897, a2898, a2899)
    };
    let a2900 = v1;
    let a2901 = 2147483648u64;
    if nat_lt(a2900, a2901) {
        v1
    } else {
        0u64
    }
}

pub fn f93(v0: u64) -> R<u64> {
    let a2904 = 2u64;
    let a2905 = {
        let a2902 = 2u64;
        let a2903 = v0;
        nat_mul(a2902, a2903)?
    };
    nat_add(a2904, a2905)
}

pub fn f94(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2907 = match v1.clone() {
        Adt14::C0(h2906, _, _, _, _, _, _, _, _, _, _) => {
            h2906
        }
    };
    let a2908 = 0u64;
    if if if if nat_eq(a2907, a2908) {
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
        f193(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f194(match v1.clone() {
            Adt14::C0(_, h2912, _, _, _, _, _, _, _, _, _) => {
                h2912
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2913, _, _, _, _, _, _, _, _) => {
                h2913
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2914, _, _, _, _, _, _, _) => {
                h2914
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2915, _, _, _, _, _, _) => {
                h2915
            }
        }) {
            true
        } else {
            if if if if f45(v0) {
                f195(v1.clone(), v2.clone())
            } else {
                false
            } {
                f196(v1.clone())
            } else {
                false
            } {
                f196(v2.clone())
            } else {
                false
            } {
                if f197(v1.clone(), v2.clone())? {
                    true
                } else {
                    f198(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f194(match v1.clone() {
            Adt14::C0(_, h2916, _, _, _, _, _, _, _, _, _) => {
                h2916
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2917, _, _, _, _, _, _, _, _) => {
                h2917
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2918, _, _, _, _, _, _, _) => {
                h2918
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2919, _, _, _, _, _, _) => {
                h2919
            }
        }) {
            0u64
        } else {
            let a2925 = match v1.clone() {
                Adt14::C0(_, _, h2920, _, _, _, _, _, _, _, _) => {
                    h2920
                }
            };
            let a2926 = {
                let a2923 = match v1.clone() {
                    Adt14::C0(_, _, _, h2921, _, _, _, _, _, _, _) => {
                        h2921
                    }
                };
                let a2924 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2922, _, _, _, _, _, _) => {
                        h2922
                    }
                };
                nat_mul(a2923, a2924)?
            };
            nat_mul(a2925, a2926)?
        };
        let v4: u64 = {
            let a2927 = v3;
            let a2928 = 0u64;
            if if nat_eq(a2927, a2928) {
                true
            } else {
                f199(v2.clone())?
            } {
                0u64
            } else {
                let a2930 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2929, _) => {
                        h2929
                    }
                };
                let a2931 = 4u64;
                let a2932 = 0u64;
                nat_quot(a2930, a2931, a2932)
            }
        };
        Ok(Adt15::C1(f200(f201(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2933, _, _, _, _, _) => {
                h2933
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2934, _, _, _, _, _) => {
                h2934
            }
        }), match v1 {
            Adt14::C0(_, h2935, _, _, _, _, _, _, _, _, _) => {
                h2935
            }
        }, v3, v4)?, 4u64, 64u64, v3, f202(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f95(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f203(v3.clone());
    let a2937 = match v2.clone() {
        Adt14::C0(h2936, _, _, _, _, _, _, _, _, _, _) => {
            h2936
        }
    };
    let a2938 = 0u64;
    if if if if nat_eq(a2937, a2938) {
        let a2940 = match v3.clone() {
            Adt14::C0(h2939, _, _, _, _, _, _, _, _, _, _) => {
                h2939
            }
        };
        let a2941 = 0u64;
        nat_eq(a2940, a2941)
    } else {
        false
    } {
        f193(v2.clone(), v3)
    } else {
        false
    } {
        if f194(match v2.clone() {
            Adt14::C0(_, h2942, _, _, _, _, _, _, _, _, _) => {
                h2942
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2943, _, _, _, _, _, _, _, _) => {
                h2943
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2944, _, _, _, _, _, _, _) => {
                h2944
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2945, _, _, _, _, _, _) => {
                h2945
            }
        }) {
            true
        } else {
            let a2947 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2946, _, _, _, _, _) => {
                    h2946
                }
            };
            let a2948 = 4u64;
            if if if if if nat_eq(a2947, a2948) {
                f204(match v2.clone() {
                    Adt14::C0(_, h2949, _, _, _, _, _, _, _, _, _) => {
                        h2949
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2950, _, _, _, _, _, _, _, _) => {
                        h2950
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2951, _, _, _, _, _, _, _) => {
                        h2951
                    }
                }, v0)?
            } else {
                false
            } {
                f196(v2.clone())
            } else {
                false
            } {
                f196(v4.clone())
            } else {
                false
            } {
                let a2953 = f205(v4.clone())?;
                let a2954 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2952, _) => {
                        h2952
                    }
                };
                nat_le(a2953, a2954)
            } else {
                false
            } {
                if f197(v2.clone(), v4.clone())? {
                    true
                } else {
                    f198(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f194(match v2.clone() {
            Adt14::C0(_, h2955, _, _, _, _, _, _, _, _, _) => {
                h2955
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2956, _, _, _, _, _, _, _, _) => {
                h2956
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2957, _, _, _, _, _, _, _) => {
                h2957
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2958, _, _, _, _, _, _) => {
                h2958
            }
        }) {
            true
        } else {
            f199(v4.clone())?
        } {
            0u64
        } else {
            let a2960 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2959, _) => {
                    h2959
                }
            };
            let a2961 = 4u64;
            let a2962 = 0u64;
            nat_quot(a2960, a2961, a2962)
        };
        Ok(Adt15::C1(f206(match v2.clone() {
            Adt14::C0(_, h2963, _, _, _, _, _, _, _, _, _) => {
                h2963
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2964, _, _, _, _, _, _, _, _) => {
                h2964
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2965, _, _, _, _, _, _, _) => {
                h2965
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2966, _, _, _, _, _, _) => {
                h2966
            }
        }, v0, v5)?, 4u64, 65u64, v0, f202(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f96(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f203(v4.clone());
    if if if if if if if v1 {
        let a2968 = match v2.clone() {
            Adt14::C0(h2967, _, _, _, _, _, _, _, _, _, _) => {
                h2967
            }
        };
        let a2969 = 0u64;
        nat_eq(a2968, a2969)
    } else {
        false
    } {
        let a2971 = match v3.clone() {
            Adt14::C0(h2970, _, _, _, _, _, _, _, _, _, _) => {
                h2970
            }
        };
        let a2972 = 0u64;
        nat_eq(a2971, a2972)
    } else {
        false
    } {
        let a2974 = match v4.clone() {
            Adt14::C0(h2973, _, _, _, _, _, _, _, _, _, _) => {
                h2973
            }
        };
        let a2975 = 0u64;
        nat_eq(a2974, a2975)
    } else {
        false
    } {
        f193(v2.clone(), v3.clone())
    } else {
        false
    } {
        f193(v2.clone(), v4)
    } else {
        false
    } {
        if f194(match v2.clone() {
            Adt14::C0(_, h2976, _, _, _, _, _, _, _, _, _) => {
                h2976
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2977, _, _, _, _, _, _, _, _) => {
                h2977
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2978, _, _, _, _, _, _, _) => {
                h2978
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2979, _, _, _, _, _, _) => {
                h2979
            }
        }) {
            true
        } else {
            let a2981 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2980, _, _, _, _, _) => {
                    h2980
                }
            };
            let a2982 = 4u64;
            if if if if if if if nat_eq(a2981, a2982) {
                let a2984 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2983, _, _, _, _, _) => {
                        h2983
                    }
                };
                let a2985 = 4u64;
                nat_eq(a2984, a2985)
            } else {
                false
            } {
                f196(v2.clone())
            } else {
                false
            } {
                f196(v3.clone())
            } else {
                false
            } {
                f196(v5.clone())
            } else {
                false
            } {
                let a2987 = f205(v5.clone())?;
                let a2988 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2986, _) => {
                        h2986
                    }
                };
                nat_le(a2987, a2988)
            } else {
                false
            } {
                if f197(v2.clone(), v5.clone())? {
                    true
                } else {
                    f198(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f197(v3.clone(), v5.clone())? {
                    true
                } else {
                    f198(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f194(match v2.clone() {
            Adt14::C0(_, h2989, _, _, _, _, _, _, _, _, _) => {
                h2989
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2990, _, _, _, _, _, _, _, _) => {
                h2990
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2991, _, _, _, _, _, _, _) => {
                h2991
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2992, _, _, _, _, _, _) => {
                h2992
            }
        }) {
            0u64
        } else {
            let a2998 = match v2.clone() {
                Adt14::C0(_, _, h2993, _, _, _, _, _, _, _, _) => {
                    h2993
                }
            };
            let a2999 = {
                let a2996 = match v2.clone() {
                    Adt14::C0(_, _, _, h2994, _, _, _, _, _, _, _) => {
                        h2994
                    }
                };
                let a2997 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2995, _, _, _, _, _, _) => {
                        h2995
                    }
                };
                nat_mul(a2996, a2997)?
            };
            nat_mul(a2998, a2999)?
        };
        let v7: u64 = {
            let a3000 = v6;
            let a3001 = 0u64;
            if if nat_eq(a3000, a3001) {
                true
            } else {
                f199(v5.clone())?
            } {
                0u64
            } else {
                let a3003 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3002, _) => {
                        h3002
                    }
                };
                let a3004 = 4u64;
                let a3005 = 0u64;
                nat_quot(a3003, a3004, a3005)
            }
        };
        Ok(Adt15::C1(f207(match v2 {
            Adt14::C0(_, h3006, _, _, _, _, _, _, _, _, _) => {
                h3006
            }
        }, v6, v7)?, 4u64, 66u64, v6, f202(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f208(v0, v3, v6) {
        f208(v1, v4, v6)
    } else {
        false
    } {
        f208(v2, v5, v6)
    } else {
        false
    }
}

pub fn f98(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f209(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f209(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f209(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f209(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f209(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f209(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3007 = v2;
    let a3008 = 0u64;
    if nat_eq(a3007, a3008) {
        Ok(0u64)
    } else {
        let a3011 = 2u64;
        let a3012 = {
            let a3009 = v0;
            let a3010 = v1;
            nat_mul(a3009, a3010)?
        };
        nat_mul(a3011, a3012)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3013 = v2;
    let a3014 = f189(v0, v1)?;
    nat_mul(a3013, a3014)
}

pub fn f101(v0: u64) -> u64 {
    let a3015 = v0;
    let a3016 = 30u64;
    if nat_eq(a3015, a3016) {
        3u64
    } else {
        1u64
    }
}

pub fn f102(v0: u64) -> R<u64> {
    let a3019 = 5u64;
    let a3020 = {
        let a3017 = 4u64;
        let a3018 = v0;
        nat_mul(a3017, a3018)?
    };
    nat_add(a3019, a3020)
}

pub fn f103(v0: u64) -> bool {
    let m3021 = f210(v0);
    match m3021 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3023 = 1u64;
            let a3024 = match v1 {
                Adt19::C0(_, h3022, _) => {
                    h3022
                }
            };
            nat_lt(a3023, a3024)
        }
    }
}

pub fn f104(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3026 = match v0.clone() {
        Adt2::C0(h3025, _, _, _, _, _, _, _, _, _, _) => {
            h3025
        }
    };
    let a3027 = 0u64;
    if if if if if if if if if if if if nat_eq(a3026, a3027) {
        f38(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3029 = match v1.clone() {
            Adt2::C0(h3028, _, _, _, _, _, _, _, _, _, _) => {
                h3028
            }
        };
        let a3030 = 0u64;
        if if nat_eq(a3029, a3030) {
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
        let a3032 = match v2.clone() {
            Adt2::C0(h3031, _, _, _, _, _, _, _, _, _, _) => {
                h3031
            }
        };
        let a3033 = 0u64;
        if if nat_eq(a3032, a3033) {
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
        let a3036 = match v0.clone() {
            Adt2::C0(_, h3034, _, _, _, _, _, _, _, _, _) => {
                h3034
            }
        };
        let a3037 = match v0.clone() {
            Adt2::C0(_, _, h3035, _, _, _, _, _, _, _, _) => {
                h3035
            }
        };
        nat_eq(a3036, a3037)
    } else {
        false
    } {
        let a3040 = match v0.clone() {
            Adt2::C0(_, _, h3038, _, _, _, _, _, _, _, _) => {
                h3038
            }
        };
        let a3041 = match v1.clone() {
            Adt2::C0(_, _, h3039, _, _, _, _, _, _, _, _) => {
                h3039
            }
        };
        nat_eq(a3040, a3041)
    } else {
        false
    } {
        let a3044 = match v0.clone() {
            Adt2::C0(_, _, _, h3042, _, _, _, _, _, _, _) => {
                h3042
            }
        };
        let a3045 = match v1.clone() {
            Adt2::C0(_, _, _, h3043, _, _, _, _, _, _, _) => {
                h3043
            }
        };
        nat_eq(a3044, a3045)
    } else {
        false
    } {
        let a3048 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3046, _, _, _, _, _, _) => {
                h3046
            }
        };
        let a3049 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3047, _, _, _, _, _, _) => {
                h3047
            }
        };
        nat_eq(a3048, a3049)
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
            let a3058 = {
                let a3055 = {
                    let a3052 = match v1.clone() {
                        Adt2::C0(_, _, _, h3050, _, _, _, _, _, _, _) => {
                            h3050
                        }
                    };
                    let a3053 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3051, _, _, _, _, _, _) => {
                            h3051
                        }
                    };
                    nat_mul(a3052, a3053)?
                };
                let a3056 = match v1 {
                    Adt2::C0(_, h3054, _, _, _, _, _, _, _, _, _) => {
                        h3054
                    }
                };
                nat_mul(a3055, a3056)?
            };
            let a3059 = f168(match v0.clone() {
                Adt2::C0(_, h3057, _, _, _, _, _, _, _, _, _) => {
                    h3057
                }
            })?;
            nat_mul(a3058, a3059)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f105(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3061 = 0u64;
        let a3062 = match v0.clone() {
            Adt1::C0(_, h3060, _, _, _, _, _, _, _, _, _, _) => {
                h3060
            }
        };
        nat_lt(a3061, a3062)
    };
    let a3064 = match v1.clone() {
        Adt2::C0(h3063, _, _, _, _, _, _, _, _, _, _) => {
            h3063
        }
    };
    let a3065 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3064, a3065) {
        let a3067 = match v2.clone() {
            Adt2::C0(h3066, _, _, _, _, _, _, _, _, _, _) => {
                h3066
            }
        };
        let a3068 = 0u64;
        nat_eq(a3067, a3068)
    } else {
        false
    } {
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
        let a3073 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3072, _, _, _, _, _, _) => {
                h3072
            }
        };
        let a3074 = 1u64;
        nat_eq(a3073, a3074)
    } else {
        false
    } {
        let a3076 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3075, _, _, _, _, _, _) => {
                h3075
            }
        };
        let a3077 = 1u64;
        nat_eq(a3076, a3077)
    } else {
        false
    } {
        let a3079 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3078, _, _, _, _, _, _) => {
                h3078
            }
        };
        let a3080 = 1u64;
        nat_eq(a3079, a3080)
    } else {
        false
    } {
        let a3082 = 0u64;
        let a3083 = match v1.clone() {
            Adt2::C0(_, _, h3081, _, _, _, _, _, _, _, _) => {
                h3081
            }
        };
        nat_lt(a3082, a3083)
    } else {
        false
    } {
        let a3085 = match v3.clone() {
            Adt2::C0(_, _, _, h3084, _, _, _, _, _, _, _) => {
                h3084
            }
        };
        let a3086 = 1u64;
        nat_eq(a3085, a3086)
    } else {
        false
    } {
        let a3089 = match v3.clone() {
            Adt2::C0(_, h3087, _, _, _, _, _, _, _, _, _) => {
                h3087
            }
        };
        let a3090 = match v1.clone() {
            Adt2::C0(_, h3088, _, _, _, _, _, _, _, _, _) => {
                h3088
            }
        };
        nat_eq(a3089, a3090)
    } else {
        false
    } {
        let a3093 = match v3.clone() {
            Adt2::C0(_, _, h3091, _, _, _, _, _, _, _, _) => {
                h3091
            }
        };
        let a3094 = match v1.clone() {
            Adt2::C0(_, _, _, h3092, _, _, _, _, _, _, _) => {
                h3092
            }
        };
        nat_eq(a3093, a3094)
    } else {
        false
    } {
        if v4 {
            f21(v1.clone(), v2.clone())
        } else {
            let a3097 = match v2.clone() {
                Adt2::C0(_, h3095, _, _, _, _, _, _, _, _, _) => {
                    h3095
                }
            };
            let a3098 = match v1.clone() {
                Adt2::C0(_, _, h3096, _, _, _, _, _, _, _, _) => {
                    h3096
                }
            };
            if if nat_eq(a3097, a3098) {
                let a3101 = match v2.clone() {
                    Adt2::C0(_, _, h3099, _, _, _, _, _, _, _, _) => {
                        h3099
                    }
                };
                let a3102 = match v1.clone() {
                    Adt2::C0(_, _, _, h3100, _, _, _, _, _, _, _) => {
                        h3100
                    }
                };
                nat_eq(a3101, a3102)
            } else {
                false
            } {
                let a3104 = match v2.clone() {
                    Adt2::C0(_, _, _, h3103, _, _, _, _, _, _, _) => {
                        h3103
                    }
                };
                let a3105 = 1u64;
                nat_eq(a3104, a3105)
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
            let a3107 = f23(v3.clone(), 1u64)?;
            let a3108 = f165(match v1.clone() {
                Adt2::C0(_, _, h3106, _, _, _, _, _, _, _, _) => {
                    h3106
                }
            }, v4)?;
            nat_mul(a3107, a3108)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C36(f25(v2, 1u64), v4, match v0 {
            Adt1::C0(h3109, _, _, _, _, _, _, _, _, _, _, _) => {
                h3109
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f106(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3110, _, _, _, _, _, _, _, _, _) => {
            h3110
        }
    }, match v0.clone() {
        Adt1::C0(h3111, _, _, _, _, _, _, _, _, _, _, _) => {
            h3111
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3112, _, _, _, _, _, _, _, _, _) => {
            h3112
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3113, _, _, _, _, _, _, _) => {
            h3113
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3114, _, _, _, _, _, _, _, _) => {
            h3114
        }
    }, match v0.clone() {
        Adt1::C0(_, h3115, _, _, _, _, _, _, _, _, _, _) => {
            h3115
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3116, _, _, _, _, _, _, _, _) => {
            h3116
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3117, _, _, _, _, _, _) => {
            h3117
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3118, _, _, _, _, _, _, _, _, _, _) => {
            h3118
        }
    };
    let v7: bool = {
        let a3121 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3119, _, _, _) => {
                h3119
            }
        };
        let a3122 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3120, _, _, _, _, _) => {
                h3120
            }
        };
        nat_lt(a3121, a3122)
    };
    let a3123 = v6;
    let a3124 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3123, a3124) {
        true
    } else {
        let a3125 = v6;
        let a3126 = 1u64;
        nat_eq(a3125, a3126)
    } {
        let a3128 = match v2.clone() {
            Adt2::C0(h3127, _, _, _, _, _, _, _, _, _, _) => {
                h3127
            }
        };
        let a3129 = 0u64;
        nat_eq(a3128, a3129)
    } else {
        false
    } {
        let a3131 = match v3.clone() {
            Adt2::C0(h3130, _, _, _, _, _, _, _, _, _, _) => {
                h3130
            }
        };
        let a3132 = 0u64;
        nat_eq(a3131, a3132)
    } else {
        false
    } {
        let a3134 = match v1.clone() {
            Adt2::C0(_, _, _, h3133, _, _, _, _, _, _, _) => {
                h3133
            }
        };
        let a3135 = 1u64;
        nat_eq(a3134, a3135)
    } else {
        false
    } {
        let a3138 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3136, _, _, _, _, _, _) => {
                h3136
            }
        };
        let a3139 = match v2.clone() {
            Adt2::C0(_, _, _, h3137, _, _, _, _, _, _, _) => {
                h3137
            }
        };
        nat_eq(a3138, a3139)
    } else {
        false
    } {
        let a3142 = match v3.clone() {
            Adt2::C0(_, _, _, h3140, _, _, _, _, _, _, _) => {
                h3140
            }
        };
        let a3143 = match v2.clone() {
            Adt2::C0(_, _, _, h3141, _, _, _, _, _, _, _) => {
                h3141
            }
        };
        nat_eq(a3142, a3143)
    } else {
        false
    } {
        let a3146 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3144, _, _, _, _, _, _) => {
                h3144
            }
        };
        let a3147 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3145, _, _, _, _, _, _) => {
                h3145
            }
        };
        nat_eq(a3146, a3147)
    } else {
        false
    } {
        let a3149 = 0u64;
        let a3150 = match v4.clone() {
            Adt6::C0(h3148, _, _, _) => {
                h3148
            }
        };
        if if if if if nat_lt(a3149, a3150) {
            let a3152 = 0u64;
            let a3153 = match v4.clone() {
                Adt6::C0(_, h3151, _, _) => {
                    h3151
                }
            };
            nat_lt(a3152, a3153)
        } else {
            false
        } {
            let a3155 = match v4.clone() {
                Adt6::C0(_, h3154, _, _) => {
                    h3154
                }
            };
            let a3156 = 2147483648u64;
            nat_lt(a3155, a3156)
        } else {
            false
        } {
            let a3158 = 0u64;
            let a3159 = match v4.clone() {
                Adt6::C0(_, _, _, h3157) => {
                    h3157
                }
            };
            nat_lt(a3158, a3159)
        } else {
            false
        } {
            let a3161 = match v4.clone() {
                Adt6::C0(_, _, _, h3160) => {
                    h3160
                }
            };
            let a3162 = 2147483648u64;
            nat_lt(a3161, a3162)
        } else {
            false
        } {
            let a3165 = match v3.clone() {
                Adt2::C0(_, h3163, _, _, _, _, _, _, _, _, _) => {
                    h3163
                }
            };
            let a3166 = f91(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3164, _, _, _, _, _, _, _, _, _) => {
                    h3164
                }
            })?;
            nat_eq(a3165, a3166)
        } else {
            false
        }
    } else {
        false
    } {
        let a3168 = 0u64;
        let a3169 = match v5.clone() {
            Adt6::C0(h3167, _, _, _) => {
                h3167
            }
        };
        if if if if if nat_lt(a3168, a3169) {
            let a3171 = 0u64;
            let a3172 = match v5.clone() {
                Adt6::C0(_, h3170, _, _) => {
                    h3170
                }
            };
            nat_lt(a3171, a3172)
        } else {
            false
        } {
            let a3174 = match v5.clone() {
                Adt6::C0(_, h3173, _, _) => {
                    h3173
                }
            };
            let a3175 = 2147483648u64;
            nat_lt(a3174, a3175)
        } else {
            false
        } {
            let a3177 = 0u64;
            let a3178 = match v5.clone() {
                Adt6::C0(_, _, _, h3176) => {
                    h3176
                }
            };
            nat_lt(a3177, a3178)
        } else {
            false
        } {
            let a3180 = match v5.clone() {
                Adt6::C0(_, _, _, h3179) => {
                    h3179
                }
            };
            let a3181 = 2147483648u64;
            nat_lt(a3180, a3181)
        } else {
            false
        } {
            let a3184 = match v3.clone() {
                Adt2::C0(_, _, h3182, _, _, _, _, _, _, _, _) => {
                    h3182
                }
            };
            let a3185 = f91(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3183, _, _, _, _, _, _, _, _) => {
                    h3183
                }
            })?;
            nat_eq(a3184, a3185)
        } else {
            false
        }
    } else {
        false
    } {
        f211(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f211(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f211(v1.clone(), {
            let a3186 = v6;
            let a3187 = 0u64;
            if nat_eq(a3186, a3187) {
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
            let a3196 = {
                let a3194 = {
                    let a3192 = 3u64;
                    let a3193 = {
                        let a3190 = match v4.clone() {
                            Adt6::C0(h3188, _, _, _) => {
                                h3188
                            }
                        };
                        let a3191 = match v5.clone() {
                            Adt6::C0(h3189, _, _, _) => {
                                h3189
                            }
                        };
                        nat_mul(a3190, a3191)?
                    };
                    nat_mul(a3192, a3193)?
                };
                let a3195 = 2u64;
                nat_add(a3194, a3195)?
            };
            let a3197 = f23(v3.clone(), 1u64)?;
            nat_mul(a3196, a3197)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C34(f25(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
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
            let a3198 = 6u64;
            let a3199 = f23(v0.clone(), 1u64)?;
            nat_mul(a3198, a3199)?
        }, 2u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C33(f25(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3200 = v2;
    match m3200 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3201, _, _, _, _, _, _, _, _, _) => {
                    h3201
                }
            };
            let v6: u64 = {
                let a3207 = match v1.clone() {
                    Adt2::C0(_, _, h3202, _, _, _, _, _, _, _, _) => {
                        h3202
                    }
                };
                let a3208 = {
                    let a3205 = match v1.clone() {
                        Adt2::C0(_, _, _, h3203, _, _, _, _, _, _, _) => {
                            h3203
                        }
                    };
                    let a3206 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3204, _, _, _, _, _, _) => {
                            h3204
                        }
                    };
                    nat_mul(a3205, a3206)?
                };
                nat_mul(a3207, a3208)?
            };
            if if if if if if if if if if f21(v1.clone(), v4.clone()) {
                f21(v1.clone(), v3.clone())
            } else {
                false
            } {
                f212(v1.clone(), v6)?
            } else {
                false
            } {
                f212(v4.clone(), v6)?
            } else {
                false
            } {
                f212(v3.clone(), v6)?
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
                let a3209 = v5;
                let a3210 = 2147483647u64;
                nat_le(a3209, a3210)
            } else {
                false
            } {
                let a3211 = v6;
                let a3212 = 2147483647u64;
                nat_le(a3211, a3212)
            } else {
                false
            } {
                let a3214 = match v0.clone() {
                    Adt1::C0(h3213, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3213
                    }
                };
                let a3215 = 4294967295u64;
                if nat_le(a3214, a3215) {
                    let a3217 = match v0.clone() {
                        Adt1::C0(_, h3216, _, _, _, _, _, _, _, _, _, _) => {
                            h3216
                        }
                    };
                    let a3218 = 4294967295u64;
                    nat_le(a3217, a3218)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3219 = v5;
                    let a3220 = 0u64;
                    if nat_eq(a3219, a3220) {
                        0u64
                    } else {
                        let a3221 = v6;
                        let a3222 = f164(v5)?;
                        nat_mul(a3221, a3222)?
                    }
                }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3223, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3223
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f109(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3224, _, _, _, _, _, _, _, _, _, _, _) => {
            h3224
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3225, _, _, _, _, _, _, _, _, _, _) => {
            h3225
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3226, _, _, _, _, _, _, _, _, _) => {
            h3226
        }
    };
    let v6: u64 = {
        let a3228 = match v1.clone() {
            Adt2::C0(_, h3227, _, _, _, _, _, _, _, _, _) => {
                h3227
            }
        };
        let a3229 = v4;
        let a3230 = 0u64;
        nat_quot(a3228, a3229, a3230)
    };
    let v7: u64 = {
        let a3232 = match v1.clone() {
            Adt2::C0(h3231, _, _, _, _, _, _, _, _, _, _) => {
                h3231
            }
        };
        let a3233 = 0u64;
        if nat_eq(a3232, a3233) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f172(match v1.clone() {
        Adt2::C0(h3234, _, _, _, _, _, _, _, _, _, _) => {
            h3234
        }
    }) {
        let a3237 = match v1.clone() {
            Adt2::C0(h3235, _, _, _, _, _, _, _, _, _, _) => {
                h3235
            }
        };
        let a3238 = match v2.clone() {
            Adt2::C0(h3236, _, _, _, _, _, _, _, _, _, _) => {
                h3236
            }
        };
        nat_eq(a3237, a3238)
    } else {
        false
    } {
        let a3239 = 0u64;
        let a3240 = v3;
        nat_lt(a3239, a3240)
    } else {
        false
    } {
        let a3241 = v3;
        let a3242 = 2147483648u64;
        nat_lt(a3241, a3242)
    } else {
        false
    } {
        let a3243 = 0u64;
        let a3244 = v4;
        nat_lt(a3243, a3244)
    } else {
        false
    } {
        let a3245 = v4;
        let a3246 = 2147483648u64;
        nat_lt(a3245, a3246)
    } else {
        false
    } {
        let a3247 = v5;
        let a3248 = 2147483648u64;
        nat_lt(a3247, a3248)
    } else {
        false
    } {
        let a3249 = 0u64;
        let a3250 = v6;
        nat_lt(a3249, a3250)
    } else {
        false
    } {
        let a3252 = 0u64;
        let a3253 = match v2.clone() {
            Adt2::C0(_, h3251, _, _, _, _, _, _, _, _, _) => {
                h3251
            }
        };
        nat_lt(a3252, a3253)
    } else {
        false
    } {
        let a3257 = match v1.clone() {
            Adt2::C0(_, h3254, _, _, _, _, _, _, _, _, _) => {
                h3254
            }
        };
        let a3258 = {
            let a3255 = v6;
            let a3256 = v4;
            nat_mul(a3255, a3256)?
        };
        nat_eq(a3257, a3258)
    } else {
        false
    } {
        let a3260 = match v2.clone() {
            Adt2::C0(_, _, h3259, _, _, _, _, _, _, _, _) => {
                h3259
            }
        };
        let a3261 = v4;
        nat_eq(a3260, a3261)
    } else {
        false
    } {
        let a3274 = {
            let a3267 = {
                let a3265 = match v2.clone() {
                    Adt2::C0(_, h3262, _, _, _, _, _, _, _, _, _) => {
                        h3262
                    }
                };
                let a3266 = {
                    let a3263 = v5;
                    let a3264 = 2u64;
                    nat_mul(a3263, a3264)?
                };
                nat_add(a3265, a3266)?
            };
            let a3268 = v3;
            nat_add(a3267, a3268)?
        };
        let a3275 = {
            let a3272 = {
                let a3270 = match v1.clone() {
                    Adt2::C0(_, _, h3269, _, _, _, _, _, _, _, _) => {
                        h3269
                    }
                };
                let a3271 = v3;
                nat_mul(a3270, a3271)?
            };
            let a3273 = v6;
            nat_add(a3272, a3273)?
        };
        nat_eq(a3274, a3275)
    } else {
        false
    } {
        f213(v1.clone(), v7)?
    } else {
        false
    } {
        f213(v2.clone(), v7)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3276 = f23(v2.clone(), 1u64)?;
            let a3277 = f186(v8.clone())?;
            nat_mul(a3276, a3277)?
        }, v7, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3278, _, _, _, _, _, _, _, _, _, _) => {
                h3278
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3279 = v1;
    match m3279 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3281 = match v0.clone() {
                Adt2::C0(h3280, _, _, _, _, _, _, _, _, _, _) => {
                    h3280
                }
            };
            let a3282 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3281, a3282) {
                let a3284 = match v3.clone() {
                    Adt2::C0(h3283, _, _, _, _, _, _, _, _, _, _) => {
                        h3283
                    }
                };
                let a3285 = 0u64;
                nat_eq(a3284, a3285)
            } else {
                false
            } {
                let a3287 = match v2.clone() {
                    Adt2::C0(h3286, _, _, _, _, _, _, _, _, _, _) => {
                        h3286
                    }
                };
                let a3288 = 0u64;
                nat_eq(a3287, a3288)
            } else {
                false
            } {
                let a3290 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3289, _, _, _, _, _, _) => {
                        h3289
                    }
                };
                let a3291 = 1u64;
                nat_eq(a3290, a3291)
            } else {
                false
            } {
                let a3293 = match v3.clone() {
                    Adt2::C0(_, _, _, h3292, _, _, _, _, _, _, _) => {
                        h3292
                    }
                };
                let a3294 = 1u64;
                nat_eq(a3293, a3294)
            } else {
                false
            } {
                let a3296 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3295, _, _, _, _, _, _) => {
                        h3295
                    }
                };
                let a3297 = 1u64;
                nat_eq(a3296, a3297)
            } else {
                false
            } {
                let a3300 = match v0.clone() {
                    Adt2::C0(_, _, h3298, _, _, _, _, _, _, _, _) => {
                        h3298
                    }
                };
                let a3301 = match v3.clone() {
                    Adt2::C0(_, _, h3299, _, _, _, _, _, _, _, _) => {
                        h3299
                    }
                };
                nat_eq(a3300, a3301)
            } else {
                false
            } {
                let a3306 = match v3.clone() {
                    Adt2::C0(_, h3302, _, _, _, _, _, _, _, _, _) => {
                        h3302
                    }
                };
                let a3307 = {
                    let a3304 = match v0.clone() {
                        Adt2::C0(_, h3303, _, _, _, _, _, _, _, _, _) => {
                            h3303
                        }
                    };
                    let a3305 = 1u64;
                    nat_add(a3304, a3305)?
                };
                nat_le(a3306, a3307)
            } else {
                false
            } {
                let a3310 = match v2.clone() {
                    Adt2::C0(_, h3308, _, _, _, _, _, _, _, _, _) => {
                        h3308
                    }
                };
                let a3311 = match v0.clone() {
                    Adt2::C0(_, _, h3309, _, _, _, _, _, _, _, _) => {
                        h3309
                    }
                };
                nat_eq(a3310, a3311)
            } else {
                false
            } {
                let a3319 = match v2.clone() {
                    Adt2::C0(_, _, h3312, _, _, _, _, _, _, _, _) => {
                        h3312
                    }
                };
                let a3320 = {
                    let a3317 = {
                        let a3314 = match v0.clone() {
                            Adt2::C0(_, h3313, _, _, _, _, _, _, _, _, _) => {
                                h3313
                            }
                        };
                        let a3315 = 1u64;
                        nat_add(a3314, a3315)?
                    };
                    let a3318 = match v3.clone() {
                        Adt2::C0(_, h3316, _, _, _, _, _, _, _, _, _) => {
                            h3316
                        }
                    };
                    nat_sub(a3317, a3318)
                };
                nat_eq(a3319, a3320)
            } else {
                false
            } {
                let a3323 = match v2.clone() {
                    Adt2::C0(_, _, _, h3321, _, _, _, _, _, _, _) => {
                        h3321
                    }
                };
                let a3324 = match v0.clone() {
                    Adt2::C0(_, _, _, h3322, _, _, _, _, _, _, _) => {
                        h3322
                    }
                };
                nat_eq(a3323, a3324)
            } else {
                false
            } {
                let a3326 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3325, _, _, _, _, _, _) => {
                        h3325
                    }
                };
                let a3327 = 1u64;
                nat_eq(a3326, a3327)
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                let a3329 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3328, _, _, _, _, _) => {
                        h3328
                    }
                };
                let a3330 = 4u64;
                nat_eq(a3329, a3330)
            } else {
                false
            } {
                let a3332 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3331, _, _, _, _, _) => {
                        h3331
                    }
                };
                let a3333 = 4u64;
                nat_eq(a3332, a3333)
            } else {
                false
            } {
                let a3338 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3334, _, _, _, _) => {
                        h3334
                    }
                };
                let a3339 = {
                    let a3336 = match v0.clone() {
                        Adt2::C0(_, h3335, _, _, _, _, _, _, _, _, _) => {
                            h3335
                        }
                    };
                    let a3337 = 4u64;
                    nat_mul(a3336, a3337)?
                };
                nat_eq(a3338, a3339)
            } else {
                false
            } {
                let a3344 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3340, _, _, _, _) => {
                        h3340
                    }
                };
                let a3345 = {
                    let a3342 = match v3.clone() {
                        Adt2::C0(_, h3341, _, _, _, _, _, _, _, _, _) => {
                            h3341
                        }
                    };
                    let a3343 = 4u64;
                    nat_mul(a3342, a3343)?
                };
                nat_eq(a3344, a3345)
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
                    let a3347 = f23(v2.clone(), 1u64)?;
                    let a3348 = f185(match v3.clone() {
                        Adt2::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                            h3346
                        }
                    })?;
                    nat_mul(a3347, a3348)?
                }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C29(f25(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f111(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h3349, _, _, _, _, _, _, _, _, _, _) => {
            h3349
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h3350, _, _, _, _, _, _, _, _) => {
                h3350
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3351, _, _, _, _, _, _, _, _, _) => {
                h3351
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h3352, _, _, _, _, _, _) => {
                h3352
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h3353, _, _, _, _, _, _, _, _) => {
                h3353
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h3354, _, _, _, _, _, _, _, _, _) => {
                h3354
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h3355, _, _, _, _, _, _, _) => {
                h3355
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h3356, _, _, _, _, _) => {
                h3356
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3358 = match v1.clone() {
        Adt1::C0(h3357, _, _, _, _, _, _, _, _, _, _, _) => {
            h3357
        }
    };
    let a3359 = 1u64;
    if if if if if if if if if if if nat_le(a3358, a3359) {
        let a3361 = match v2.clone() {
            Adt2::C0(h3360, _, _, _, _, _, _, _, _, _, _) => {
                h3360
            }
        };
        let a3362 = 1u64;
        nat_le(a3361, a3362)
    } else {
        false
    } {
        let a3364 = match v3.clone() {
            Adt2::C0(h3363, _, _, _, _, _, _, _, _, _, _) => {
                h3363
            }
        };
        let a3365 = 0u64;
        nat_eq(a3364, a3365)
    } else {
        false
    } {
        let a3370 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3366, _, _, _, _, _) => {
                h3366
            }
        };
        let a3371 = {
            let a3368 = match v2.clone() {
                Adt2::C0(h3367, _, _, _, _, _, _, _, _, _, _) => {
                    h3367
                }
            };
            let a3369 = 0u64;
            if nat_eq(a3368, a3369) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3370, a3371)
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
        f214(v4.clone(), match v2.clone() {
            Adt2::C0(_, h3372, _, _, _, _, _, _, _, _, _) => {
                h3372
            }
        }, match v3.clone() {
            Adt2::C0(_, h3373, _, _, _, _, _, _, _, _, _) => {
                h3373
            }
        })?
    } else {
        false
    } {
        if v0 {
            f214(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3374, _, _, _, _, _, _, _, _) => {
                    h3374
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h3375, _, _, _, _, _, _, _, _) => {
                    h3375
                }
            })?
        } else {
            let a3378 = match v2.clone() {
                Adt2::C0(_, _, h3376, _, _, _, _, _, _, _, _) => {
                    h3376
                }
            };
            let a3379 = match v3.clone() {
                Adt2::C0(_, _, h3377, _, _, _, _, _, _, _, _) => {
                    h3377
                }
            };
            nat_eq(a3378, a3379)
        }
    } else {
        false
    } {
        let a3382 = match v2.clone() {
            Adt2::C0(_, _, _, h3380, _, _, _, _, _, _, _) => {
                h3380
            }
        };
        let a3383 = match v3.clone() {
            Adt2::C0(_, _, _, h3381, _, _, _, _, _, _, _) => {
                h3381
            }
        };
        nat_eq(a3382, a3383)
    } else {
        false
    } {
        let a3386 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3384, _, _, _, _, _, _) => {
                h3384
            }
        };
        let a3387 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3385, _, _, _, _, _, _) => {
                h3385
            }
        };
        nat_eq(a3386, a3387)
    } else {
        false
    } {
        let a3392 = {
            let a3390 = match v4.clone() {
                Adt6::C0(h3388, _, _, _) => {
                    h3388
                }
            };
            let a3391 = match v5.clone() {
                Adt6::C0(h3389, _, _, _) => {
                    h3389
                }
            };
            nat_mul(a3390, a3391)?
        };
        let a3393 = 2147483648u64;
        nat_lt(a3392, a3393)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a3395 = match v1 {
                Adt1::C0(h3394, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3394
                }
            };
            let a3396 = 0u64;
            nat_eq(a3395, a3396)
        }, match v2.clone() {
            Adt2::C0(h3397, _, _, _, _, _, _, _, _, _, _) => {
                h3397
            }
        });
        Ok(Adt10::C4({
            let a3398 = f23(v3.clone(), 1u64)?;
            let a3399 = f181(v6.clone())?;
            nat_mul(a3398, a3399)?
        }, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3400 = v2;
    match m3400 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a3402 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3401, _, _, _, _, _) => {
                        h3401
                    }
                };
                let a3403 = 1u64;
                nat_eq(a3402, a3403)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3404, _, _, _, _, _, _, _, _, _) => {
                    h3404
                }
            }, match v0.clone() {
                Adt1::C0(h3405, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3405
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3406, _, _, _, _, _, _, _, _, _) => {
                    h3406
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3407, _, _, _, _, _, _, _) => {
                    h3407
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h3408, _, _, _, _, _, _, _, _) => {
                        h3408
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h3409, _, _, _, _, _, _, _, _, _, _) => {
                        h3409
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h3410, _, _, _, _, _, _, _, _) => {
                        h3410
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h3411, _, _, _, _, _, _) => {
                        h3411
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h3412, _, _, _, _, _, _, _) => {
                        h3412
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h3413, _, _, _, _, _, _, _, _) => {
                        h3413
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h3414, _, _, _, _, _, _) => {
                        h3414
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h3415, _, _, _, _, _, _, _) => {
                        h3415
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h3416, _, _, _, _, _, _, _, _) => {
                        h3416
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h3417, _, _, _, _, _, _, _, _, _, _) => {
                    h3417
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h3418, _, _, _, _, _, _, _, _, _, _) => {
                    h3418
                }
            };
            let v14: u64 = {
                let a3419 = v12;
                let a3420 = 0u64;
                if nat_eq(a3419, a3420) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a3422 = match v4.clone() {
                    Adt2::C0(_, h3421, _, _, _, _, _, _, _, _, _) => {
                        h3421
                    }
                };
                let a3423 = v14;
                nat_mul(a3422, a3423)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3424, _, _, _) => {
                        h3424
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3425, _, _, _, _) => {
                        h3425
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3426, _, _) => {
                        h3426
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3427, _, _, _) => {
                        h3427
                    }
                }
            };
            let a3429 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h3428, _, _, _, _, _) => {
                    h3428
                }
            };
            let a3430 = 1u64;
            if if if if if if if if if if nat_le(a3429, a3430) {
                let a3431 = v12;
                let a3432 = 0u64;
                if nat_eq(a3431, a3432) {
                    true
                } else {
                    let a3433 = v12;
                    let a3434 = 1u64;
                    if nat_eq(a3433, a3434) {
                        let a3435 = v13;
                        let a3436 = 1u64;
                        nat_eq(a3435, a3436)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3437 = v13;
                let a3438 = 0u64;
                if nat_eq(a3437, a3438) {
                    true
                } else {
                    let a3439 = v13;
                    let a3440 = 1u64;
                    nat_eq(a3439, a3440)
                }
            } else {
                false
            } {
                let a3442 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3441, _, _, _, _, _) => {
                        h3441
                    }
                };
                let a3443 = v14;
                nat_eq(a3442, a3443)
            } else {
                false
            } {
                let a3451 = match v3.clone() {
                    Adt2::C0(_, h3444, _, _, _, _, _, _, _, _, _) => {
                        h3444
                    }
                };
                let a3452 = {
                    let a3449 = v9;
                    let a3450 = {
                        let a3447 = match v6.clone() {
                            Adt6::C0(h3445, _, _, _) => {
                                h3445
                            }
                        };
                        let a3448 = match v7.clone() {
                            Adt6::C0(h3446, _, _, _) => {
                                h3446
                            }
                        };
                        nat_mul(a3447, a3448)?
                    };
                    nat_mul(a3449, a3450)?
                };
                nat_eq(a3451, a3452)
            } else {
                false
            } {
                let a3455 = match v3.clone() {
                    Adt2::C0(_, _, h3453, _, _, _, _, _, _, _, _) => {
                        h3453
                    }
                };
                let a3456 = f91(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h3454, _, _, _, _, _, _, _, _, _) => {
                        h3454
                    }
                })?;
                nat_eq(a3455, a3456)
            } else {
                false
            } {
                let a3458 = 0u64;
                let a3459 = match v3.clone() {
                    Adt2::C0(_, _, h3457, _, _, _, _, _, _, _, _) => {
                        h3457
                    }
                };
                nat_lt(a3458, a3459)
            } else {
                false
            } {
                if v5 {
                    let a3461 = match v1 {
                        Adt2::C0(_, _, _, h3460, _, _, _, _, _, _, _) => {
                            h3460
                        }
                    };
                    let a3462 = v9;
                    if if if nat_eq(a3461, a3462) {
                        let a3464 = match v3.clone() {
                            Adt2::C0(_, _, _, h3463, _, _, _, _, _, _, _) => {
                                h3463
                            }
                        };
                        let a3465 = f91(v7.clone(), v11)?;
                        nat_eq(a3464, a3465)
                    } else {
                        false
                    } {
                        let a3467 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3466, _, _, _, _, _, _) => {
                                h3466
                            }
                        };
                        let a3468 = v10;
                        nat_eq(a3467, a3468)
                    } else {
                        false
                    } {
                        let a3470 = 0u64;
                        let a3471 = match v3.clone() {
                            Adt2::C0(_, _, _, h3469, _, _, _, _, _, _, _) => {
                                h3469
                            }
                        };
                        nat_lt(a3470, a3471)
                    } else {
                        false
                    }
                } else {
                    let a3473 = match v1 {
                        Adt2::C0(_, _, h3472, _, _, _, _, _, _, _, _) => {
                            h3472
                        }
                    };
                    let a3474 = v9;
                    if if if nat_eq(a3473, a3474) {
                        let a3476 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3475, _, _, _, _, _, _) => {
                                h3475
                            }
                        };
                        let a3477 = 1u64;
                        nat_eq(a3476, a3477)
                    } else {
                        false
                    } {
                        let a3479 = match v3.clone() {
                            Adt2::C0(_, _, _, h3478, _, _, _, _, _, _, _) => {
                                h3478
                            }
                        };
                        let a3480 = v10;
                        nat_eq(a3479, a3480)
                    } else {
                        false
                    } {
                        let a3482 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3481, _, _, _, _, _, _) => {
                                h3481
                            }
                        };
                        let a3483 = 1u64;
                        nat_eq(a3482, a3483)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f48(match v4.clone() {
                    Adt2::C0(_, h3484, _, _, _, _, _, _, _, _, _) => {
                        h3484
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a3504 = {
                        let a3501 = {
                            let a3499 = {
                                let a3493 = {
                                    let a3487 = {
                                        let a3485 = v10;
                                        let a3486 = 1u64;
                                        nat_sub(a3485, a3486)
                                    };
                                    let a3488 = v17;
                                    nat_mul(a3487, a3488)?
                                };
                                let a3494 = {
                                    let a3491 = {
                                        let a3489 = v9;
                                        let a3490 = 1u64;
                                        nat_sub(a3489, a3490)
                                    };
                                    let a3492 = v16;
                                    nat_mul(a3491, a3492)?
                                };
                                nat_add(a3493, a3494)?
                            };
                            let a3500 = {
                                let a3497 = {
                                    let a3495 = v11;
                                    let a3496 = 1u64;
                                    nat_sub(a3495, a3496)
                                };
                                let a3498 = v15;
                                nat_mul(a3497, a3498)?
                            };
                            nat_add(a3499, a3500)?
                        };
                        let a3502 = v15;
                        nat_add(a3501, a3502)?
                    };
                    let a3505 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3503, _) => {
                            h3503
                        }
                    };
                    nat_le(a3504, a3505)
                }
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3506 = v13;
                    let a3507 = 0u64;
                    if nat_eq(a3506, a3507) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h3508, _, _, _, _, _, _, _, _, _) => {
                        h3508
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f25(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h3509, _, _, _, _, _, _, _, _, _) => {
                            h3509
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3510, _, _, _, _, _, _, _, _) => {
                            h3510
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h3511, _, _, _, _, _, _, _) => {
                            h3511
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h3512, _, _, _, _, _) => {
                            h3512
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3513, _, _, _, _) => {
                            h3513
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h3514, _, _, _) => {
                            h3514
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f113(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3515 = v2;
    match m3515 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3516, _, _, _, _, _, _, _, _, _) => {
                    h3516
                }
            }, match v0.clone() {
                Adt1::C0(h3517, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3517
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3518, _, _, _, _, _, _, _, _) => {
                    h3518
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3519, _, _, _, _, _) => {
                    h3519
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3520, _, _, _, _, _, _, _, _) => {
                    h3520
                }
            }, match v0.clone() {
                Adt1::C0(_, h3521, _, _, _, _, _, _, _, _, _, _) => {
                    h3521
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3522, _, _, _, _, _, _, _) => {
                    h3522
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3523, _, _, _, _) => {
                    h3523
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3524, _, _, _, _, _, _, _) => {
                    h3524
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3525, _, _, _, _, _, _, _, _, _) => {
                    h3525
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3526, _, _, _, _, _, _) => {
                    h3526
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3527, _, _, _) => {
                    h3527
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3528, _, _) => {
                    h3528
                }
            };
            let v9: u64 = f91(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3529, _, _, _, _, _, _, _) => {
                    h3529
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3530, _, _, _, _, _, _, _, _, _, _) => {
                    h3530
                }
            };
            let v11: u64 = {
                let a3538 = v8;
                let a3539 = {
                    let a3536 = match v5.clone() {
                        Adt6::C0(h3531, _, _, _) => {
                            h3531
                        }
                    };
                    let a3537 = {
                        let a3534 = match v6.clone() {
                            Adt6::C0(h3532, _, _, _) => {
                                h3532
                            }
                        };
                        let a3535 = match v7.clone() {
                            Adt6::C0(h3533, _, _, _) => {
                                h3533
                            }
                        };
                        nat_mul(a3534, a3535)?
                    };
                    nat_mul(a3536, a3537)?
                };
                nat_mul(a3538, a3539)?
            };
            let a3541 = match v4.clone() {
                Adt2::C0(h3540, _, _, _, _, _, _, _, _, _, _) => {
                    h3540
                }
            };
            let a3542 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3541, a3542) {
                let a3543 = v10;
                let a3544 = 0u64;
                if nat_eq(a3543, a3544) {
                    true
                } else {
                    let a3545 = v10;
                    let a3546 = 1u64;
                    nat_eq(a3545, a3546)
                }
            } else {
                false
            } {
                let a3547 = 0u64;
                let a3548 = v8;
                nat_lt(a3547, a3548)
            } else {
                false
            } {
                let a3549 = v8;
                let a3550 = 2147483648u64;
                nat_lt(a3549, a3550)
            } else {
                false
            } {
                let a3552 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3551, _, _, _, _, _) => {
                        h3551
                    }
                };
                let a3553 = 4u64;
                nat_eq(a3552, a3553)
            } else {
                false
            } {
                let a3554 = 0u64;
                let a3555 = v9;
                nat_lt(a3554, a3555)
            } else {
                false
            } {
                let a3557 = 0u64;
                let a3558 = match v3.clone() {
                    Adt2::C0(_, _, h3556, _, _, _, _, _, _, _, _) => {
                        h3556
                    }
                };
                nat_lt(a3557, a3558)
            } else {
                false
            } {
                let a3560 = 0u64;
                let a3561 = match v3.clone() {
                    Adt2::C0(_, _, _, h3559, _, _, _, _, _, _, _) => {
                        h3559
                    }
                };
                nat_lt(a3560, a3561)
            } else {
                false
            } {
                let a3563 = match v3.clone() {
                    Adt2::C0(_, h3562, _, _, _, _, _, _, _, _, _) => {
                        h3562
                    }
                };
                let a3564 = v11;
                nat_eq(a3563, a3564)
            } else {
                false
            } {
                let a3567 = match v3.clone() {
                    Adt2::C0(_, _, h3565, _, _, _, _, _, _, _, _) => {
                        h3565
                    }
                };
                let a3568 = f91(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3566, _, _, _, _, _, _, _, _, _) => {
                        h3566
                    }
                })?;
                nat_eq(a3567, a3568)
            } else {
                false
            } {
                let a3571 = match v3.clone() {
                    Adt2::C0(_, _, _, h3569, _, _, _, _, _, _, _) => {
                        h3569
                    }
                };
                let a3572 = f91(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3570, _, _, _, _, _, _, _, _) => {
                        h3570
                    }
                })?;
                nat_eq(a3571, a3572)
            } else {
                false
            } {
                let a3580 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3573, _, _, _, _, _, _) => {
                        h3573
                    }
                };
                let a3581 = {
                    let a3578 = v9;
                    let a3579 = {
                        let a3575 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3574, _, _, _, _, _, _) => {
                                h3574
                            }
                        };
                        let a3576 = v8;
                        let a3577 = 0u64;
                        nat_quot(a3575, a3576, a3577)
                    };
                    nat_mul(a3578, a3579)?
                };
                nat_eq(a3580, a3581)
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3582 = v10;
                    let a3583 = 0u64;
                    if nat_eq(a3582, a3583) {
                        4u64
                    } else {
                        2u64
                    }
                }, f25(v4.clone(), 1u64), f25(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3584, _, _) => {
                        h3584
                    }
                }, {
                    let a3586 = v8;
                    let a3587 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3585, _, _) => {
                            h3585
                        }
                    };
                    nat_mul(a3586, a3587)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f114(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3588 = v1;
    match m3588 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f215(v0, v3, v2)
        }
    }
}

pub fn f115(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3590 = match v0.clone() {
        Adt2::C0(h3589, _, _, _, _, _, _, _, _, _, _) => {
            h3589
        }
    };
    let a3591 = 1u64;
    if if nat_eq(a3590, a3591) {
        true
    } else {
        let a3593 = match v0.clone() {
            Adt2::C0(h3592, _, _, _, _, _, _, _, _, _, _) => {
                h3592
            }
        };
        let a3594 = 30u64;
        nat_eq(a3593, a3594)
    } {
        let a3596 = match v1.clone() {
            Adt2::C0(h3595, _, _, _, _, _, _, _, _, _, _) => {
                h3595
            }
        };
        let a3597 = 1u64;
        if if if if if if if if if if if nat_eq(a3596, a3597) {
            let a3600 = match v0.clone() {
                Adt2::C0(_, h3598, _, _, _, _, _, _, _, _, _) => {
                    h3598
                }
            };
            let a3601 = match v1.clone() {
                Adt2::C0(_, h3599, _, _, _, _, _, _, _, _, _) => {
                    h3599
                }
            };
            nat_eq(a3600, a3601)
        } else {
            false
        } {
            let a3603 = 0u64;
            let a3604 = match v1.clone() {
                Adt2::C0(_, _, h3602, _, _, _, _, _, _, _, _) => {
                    h3602
                }
            };
            nat_lt(a3603, a3604)
        } else {
            false
        } {
            let a3607 = match v1.clone() {
                Adt2::C0(_, _, h3605, _, _, _, _, _, _, _, _) => {
                    h3605
                }
            };
            let a3608 = match v1.clone() {
                Adt2::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                    h3606
                }
            };
            nat_eq(a3607, a3608)
        } else {
            false
        } {
            let a3610 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3609, _, _, _, _, _, _) => {
                    h3609
                }
            };
            let a3611 = 1u64;
            nat_eq(a3610, a3611)
        } else {
            false
        } {
            let a3619 = match v0.clone() {
                Adt2::C0(_, _, h3612, _, _, _, _, _, _, _, _) => {
                    h3612
                }
            };
            let a3620 = {
                let a3617 = {
                    let a3615 = match v1.clone() {
                        Adt2::C0(_, _, h3613, _, _, _, _, _, _, _, _) => {
                            h3613
                        }
                    };
                    let a3616 = match v1.clone() {
                        Adt2::C0(_, _, h3614, _, _, _, _, _, _, _, _) => {
                            h3614
                        }
                    };
                    nat_add(a3615, a3616)?
                };
                let a3618 = 1u64;
                nat_sub(a3617, a3618)
            };
            nat_eq(a3619, a3620)
        } else {
            false
        } {
            let a3622 = 0u64;
            let a3623 = match v0.clone() {
                Adt2::C0(_, _, _, h3621, _, _, _, _, _, _, _) => {
                    h3621
                }
            };
            nat_lt(a3622, a3623)
        } else {
            false
        } {
            let a3625 = 0u64;
            let a3626 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3624, _, _, _, _, _, _) => {
                    h3624
                }
            };
            nat_lt(a3625, a3626)
        } else {
            false
        } {
            let a3628 = 2u64;
            let a3629 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3627, _, _, _, _, _) => {
                    h3627
                }
            };
            nat_le(a3628, a3629)
        } else {
            false
        } {
            let a3631 = 2u64;
            let a3632 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3630, _, _, _, _, _) => {
                    h3630
                }
            };
            nat_le(a3631, a3632)
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

pub fn f116(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3633 = v1;
    match m3633 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f216(v0, v3, v2)
        }
    }
}

pub fn f117(v0: u64) -> bool {
    let a3634 = v0;
    let a3635 = 1u64;
    if if nat_eq(a3634, a3635) {
        true
    } else {
        let a3636 = v0;
        let a3637 = 33u64;
        nat_eq(a3636, a3637)
    } {
        true
    } else {
        let a3638 = v0;
        let a3639 = 34u64;
        nat_eq(a3638, a3639)
    }
}

pub fn f118(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f47(v1, v2.clone()) {
        let a3640 = v3;
        let a3641 = f23(v2.clone(), 1u64)?;
        nat_eq(a3640, a3641)
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
        let a3642 = v3;
        let a3643 = 0u64;
        if nat_eq(a3642, a3643) {
            true
        } else {
            let a3645 = 4u64;
            let a3646 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3644, _, _, _, _, _) => {
                    h3644
                }
            };
            nat_le(a3645, a3646)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3647 = 2u64;
            let a3648 = v3;
            nat_mul(a3647, a3648)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f119(v0: u64) -> bool {
    if f217(v0) {
        !f218(v0)
    } else {
        false
    }
}

pub fn f120(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3649 = v1;
    match m3649 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3651 = f210(match v0.clone() {
                Adt2::C0(h3650, _, _, _, _, _, _, _, _, _, _) => {
                    h3650
                }
            });
            match m3651 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3652, _) => {
                            h3652
                        }
                    };
                    if if if if if if f47(v3, v2.clone()) {
                        f30(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3653 = f23(v0.clone(), 1u64)?;
                        let a3654 = f23(v2.clone(), 1u64)?;
                        nat_eq(a3653, a3654)
                    } else {
                        false
                    } {
                        let a3659 = {
                            let a3656 = match v0.clone() {
                                Adt2::C0(_, h3655, _, _, _, _, _, _, _, _, _) => {
                                    h3655
                                }
                            };
                            let a3657 = v5;
                            let a3658 = 1u64;
                            nat_rem(a3656, a3657, a3658)
                        };
                        let a3660 = 0u64;
                        if nat_eq(a3659, a3660) {
                            let a3663 = match v4 {
                                Adt19::C0(_, _, h3661) => {
                                    h3661
                                }
                            };
                            let a3664 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3662, _, _, _, _, _) => {
                                    h3662
                                }
                            };
                            nat_le(a3663, a3664)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3665 = v5;
                        let a3666 = 1u64;
                        if nat_eq(a3665, a3666) {
                            let a3668 = 4u64;
                            let a3669 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3667, _, _, _, _, _) => {
                                    h3667
                                }
                            };
                            nat_le(a3668, a3669)
                        } else {
                            let a3671 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3670, _, _, _, _, _) => {
                                    h3670
                                }
                            };
                            let a3672 = 4u64;
                            nat_eq(a3671, a3672)
                        }
                    } else {
                        false
                    } {
                        let a3677 = {
                            let a3674 = match v2.clone() {
                                Adt2::C0(_, h3673, _, _, _, _, _, _, _, _, _) => {
                                    h3673
                                }
                            };
                            let a3675 = v5;
                            let a3676 = 1u64;
                            nat_rem(a3674, a3675, a3676)
                        };
                        let a3678 = 0u64;
                        if nat_eq(a3677, a3678) {
                            true
                        } else {
                            f38(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f25(v0.clone(), v5), f25(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3679, _, _, _, _, _, _, _, _, _, _) => {
                                h3679
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

pub fn f121(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3682 = match v1.clone() {
        Adt2::C0(h3680, _, _, _, _, _, _, _, _, _, _) => {
            h3680
        }
    };
    let a3683 = match v3.clone() {
        Adt2::C0(h3681, _, _, _, _, _, _, _, _, _, _) => {
            h3681
        }
    };
    if nat_eq(a3682, a3683) {
        f219(v0, v1, v2, v3)
    } else {
        if if if if f218(match v1.clone() {
            Adt2::C0(h3684, _, _, _, _, _, _, _, _, _, _) => {
                h3684
            }
        }) {
            f218(match v3.clone() {
                Adt2::C0(h3685, _, _, _, _, _, _, _, _, _, _) => {
                    h3685
                }
            })
        } else {
            false
        } {
            let a3686 = v0;
            let a3687 = 33u64;
            if nat_eq(a3686, a3687) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3688 = f23(v1.clone(), 1u64)?;
            let a3689 = f23(v3.clone(), 1u64)?;
            nat_eq(a3688, a3689)
        } else {
            false
        } {
            let m3691 = f210(match v3.clone() {
                Adt2::C0(h3690, _, _, _, _, _, _, _, _, _, _) => {
                    h3690
                }
            });
            match m3691 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3692, _, _, _, _, _, _, _, _, _, _) => {
                            h3692
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3693, _, _, _, _, _, _, _, _, _, _) => {
                            h3693
                        }
                    };
                    let v7: bool = {
                        let a3694 = v5;
                        let a3695 = 43u64;
                        if nat_eq(a3694, a3695) {
                            true
                        } else {
                            let a3696 = v6;
                            let a3697 = 43u64;
                            nat_eq(a3696, a3697)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3698 = f23(v3.clone(), 1u64)?;
                        let a3699 = if v7 {
                            f162(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3698, a3699)?
                    }, match v4 {
                        Adt19::C0(_, _, h3700) => {
                            h3700
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

pub fn f122(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3702 = match v0.clone() {
        Adt2::C0(h3701, _, _, _, _, _, _, _, _, _, _) => {
            h3701
        }
    };
    let a3703 = 0u64;
    if if if if nat_eq(a3702, a3703) {
        true
    } else {
        let a3705 = match v0.clone() {
            Adt2::C0(h3704, _, _, _, _, _, _, _, _, _, _) => {
                h3704
            }
        };
        let a3706 = 1u64;
        nat_eq(a3705, a3706)
    } {
        true
    } else {
        let a3708 = match v0.clone() {
            Adt2::C0(h3707, _, _, _, _, _, _, _, _, _, _) => {
                h3707
            }
        };
        let a3709 = 25u64;
        if nat_eq(a3708, a3709) {
            true
        } else {
            let a3711 = match v0.clone() {
                Adt2::C0(h3710, _, _, _, _, _, _, _, _, _, _) => {
                    h3710
                }
            };
            let a3712 = 26u64;
            nat_eq(a3711, a3712)
        }
    } {
        true
    } else {
        let a3714 = match v0.clone() {
            Adt2::C0(h3713, _, _, _, _, _, _, _, _, _, _) => {
                h3713
            }
        };
        let a3715 = 30u64;
        nat_eq(a3714, a3715)
    } {
        let a3718 = match v0.clone() {
            Adt2::C0(h3716, _, _, _, _, _, _, _, _, _, _) => {
                h3716
            }
        };
        let a3719 = match v1.clone() {
            Adt2::C0(h3717, _, _, _, _, _, _, _, _, _, _) => {
                h3717
            }
        };
        if if nat_eq(a3718, a3719) {
            f220(match v1.clone() {
                Adt2::C0(_, h3720, _, _, _, _, _, _, _, _, _) => {
                    h3720
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3721, _, _, _, _, _, _, _, _) => {
                    h3721
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3722, _, _, _, _, _, _, _) => {
                    h3722
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3723, _, _, _, _, _, _) => {
                    h3723
                }
            }, match v0.clone() {
                Adt2::C0(_, h3724, _, _, _, _, _, _, _, _, _) => {
                    h3724
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3725, _, _, _, _, _, _, _, _) => {
                    h3725
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3726, _, _, _, _, _, _, _) => {
                    h3726
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3727, _, _, _, _, _, _) => {
                    h3727
                }
            })
        } else {
            false
        } {
            let m3729 = f210(match v0.clone() {
                Adt2::C0(h3728, _, _, _, _, _, _, _, _, _, _) => {
                    h3728
                }
            });
            match m3729 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3730) => {
                            h3730
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

pub fn f123(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3731 = v2;
    match m3731 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3732 = v0;
            let a3733 = 4u64;
            if if if nat_lt(a3732, a3733) {
                let a3736 = match v1.clone() {
                    Adt2::C0(h3734, _, _, _, _, _, _, _, _, _, _) => {
                        h3734
                    }
                };
                let a3737 = match v4.clone() {
                    Adt2::C0(h3735, _, _, _, _, _, _, _, _, _, _) => {
                        h3735
                    }
                };
                if nat_eq(a3736, a3737) {
                    let a3740 = match v1.clone() {
                        Adt2::C0(h3738, _, _, _, _, _, _, _, _, _, _) => {
                            h3738
                        }
                    };
                    let a3741 = match v3.clone() {
                        Adt2::C0(h3739, _, _, _, _, _, _, _, _, _, _) => {
                            h3739
                        }
                    };
                    nat_eq(a3740, a3741)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f221(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3742, _, _, _, _, _, _, _, _, _) => {
                        h3742
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3743, _, _, _, _, _, _, _, _, _) => {
                        h3743
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3744, _, _, _, _, _, _, _, _, _) => {
                        h3744
                    }
                })? {
                    f221(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3745, _, _, _, _, _, _, _, _) => {
                            h3745
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3746, _, _, _, _, _, _, _, _) => {
                            h3746
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3747, _, _, _, _, _, _, _, _) => {
                            h3747
                        }
                    })?
                } else {
                    false
                } {
                    f221(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3748, _, _, _, _, _, _, _) => {
                            h3748
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3749, _, _, _, _, _, _, _) => {
                            h3749
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3750, _, _, _, _, _, _, _) => {
                            h3750
                        }
                    })?
                } else {
                    false
                } {
                    f221(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3751, _, _, _, _, _, _) => {
                            h3751
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3752, _, _, _, _, _, _) => {
                            h3752
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3753, _, _, _, _, _, _) => {
                            h3753
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3755 = f210(match v1.clone() {
                    Adt2::C0(h3754, _, _, _, _, _, _, _, _, _, _) => {
                        h3754
                    }
                });
                match m3755 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3756, _) => {
                                h3756
                            }
                        };
                        let a3761 = {
                            let a3758 = match v1.clone() {
                                Adt2::C0(_, h3757, _, _, _, _, _, _, _, _, _) => {
                                    h3757
                                }
                            };
                            let a3759 = v6;
                            let a3760 = 1u64;
                            nat_rem(a3758, a3759, a3760)
                        };
                        let a3762 = 0u64;
                        if if if nat_eq(a3761, a3762) {
                            let a3767 = {
                                let a3764 = match v4.clone() {
                                    Adt2::C0(_, h3763, _, _, _, _, _, _, _, _, _) => {
                                        h3763
                                    }
                                };
                                let a3765 = v6;
                                let a3766 = 1u64;
                                nat_rem(a3764, a3765, a3766)
                            };
                            let a3768 = 0u64;
                            nat_eq(a3767, a3768)
                        } else {
                            false
                        } {
                            let a3773 = {
                                let a3770 = match v3.clone() {
                                    Adt2::C0(_, h3769, _, _, _, _, _, _, _, _, _) => {
                                        h3769
                                    }
                                };
                                let a3771 = v6;
                                let a3772 = 1u64;
                                nat_rem(a3770, a3771, a3772)
                            };
                            let a3774 = 0u64;
                            nat_eq(a3773, a3774)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3775) => {
                                    h3775
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

pub fn f124(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3777 = match v2.clone() {
        Adt2::C0(h3776, _, _, _, _, _, _, _, _, _, _) => {
            h3776
        }
    };
    let a3778 = 0u64;
    if if nat_eq(a3777, a3778) {
        let a3780 = match v3.clone() {
            Adt2::C0(h3779, _, _, _, _, _, _, _, _, _, _) => {
                h3779
            }
        };
        let a3781 = 0u64;
        nat_eq(a3780, a3781)
    } else {
        false
    } {
        let a3782 = v0;
        let a3783 = 61u64;
        if nat_eq(a3782, a3783) {
            let v4: bool = {
                let a3785 = 0u64;
                let a3786 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3784, _, _, _) => {
                        h3784
                    }
                };
                nat_lt(a3785, a3786)
            };
            if if if if if if f222(match v2.clone() {
                Adt2::C0(_, h3787, _, _, _, _, _, _, _, _, _) => {
                    h3787
                }
            }, match v1.clone() {
                Adt1::C0(h3788, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3788
                }
            }, match v1.clone() {
                Adt1::C0(_, h3789, _, _, _, _, _, _, _, _, _, _) => {
                    h3789
                }
            }, match v3.clone() {
                Adt2::C0(_, h3790, _, _, _, _, _, _, _, _, _) => {
                    h3790
                }
            })? {
                f222(match v2.clone() {
                    Adt2::C0(_, _, h3791, _, _, _, _, _, _, _, _) => {
                        h3791
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3792, _, _, _, _, _, _, _, _, _) => {
                        h3792
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3793, _, _, _, _, _, _, _, _) => {
                        h3793
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3794, _, _, _, _, _, _, _, _) => {
                        h3794
                    }
                })?
            } else {
                false
            } {
                f222(match v2.clone() {
                    Adt2::C0(_, _, _, h3795, _, _, _, _, _, _, _) => {
                        h3795
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3796, _, _, _, _, _, _, _) => {
                        h3796
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3797, _, _, _, _, _, _) => {
                        h3797
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3798, _, _, _, _, _, _, _) => {
                        h3798
                    }
                })?
            } else {
                false
            } {
                f222(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3799, _, _, _, _, _, _) => {
                        h3799
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3800, _, _, _, _, _) => {
                        h3800
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3801, _, _, _, _) => {
                        h3801
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3802, _, _, _, _, _, _) => {
                        h3802
                    }
                })?
            } else {
                false
            } {
                let a3804 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3803, _, _, _) => {
                        h3803
                    }
                };
                let a3805 = 4294967295u64;
                nat_le(a3804, a3805)
            } else {
                false
            } {
                if v4 {
                    !f48(match v2.clone() {
                        Adt2::C0(_, h3806, _, _, _, _, _, _, _, _, _) => {
                            h3806
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3807, _, _, _, _, _, _, _, _) => {
                            h3807
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3808, _, _, _, _, _, _, _) => {
                            h3808
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3809, _, _, _, _, _, _) => {
                            h3809
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3810, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3810
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3811, _, _, _, _, _, _, _, _, _) => {
                        h3811
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3812, _, _, _, _, _, _, _) => {
                        h3812
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3813, _, _, _, _, _) => {
                        h3813
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3814 = v0;
            let a3815 = 62u64;
            if nat_eq(a3814, a3815) {
                let a3817 = match v1.clone() {
                    Adt1::C0(h3816, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3816
                    }
                };
                let a3818 = 2147483648u64;
                if if if if if nat_lt(a3817, a3818) {
                    let a3820 = match v1.clone() {
                        Adt1::C0(_, h3819, _, _, _, _, _, _, _, _, _, _) => {
                            h3819
                        }
                    };
                    let a3821 = 2147483648u64;
                    nat_lt(a3820, a3821)
                } else {
                    false
                } {
                    let a3824 = match v1.clone() {
                        Adt1::C0(h3822, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3822
                        }
                    };
                    let a3825 = match v2.clone() {
                        Adt2::C0(_, h3823, _, _, _, _, _, _, _, _, _) => {
                            h3823
                        }
                    };
                    nat_lt(a3824, a3825)
                } else {
                    false
                } {
                    let a3828 = match v1.clone() {
                        Adt1::C0(_, h3826, _, _, _, _, _, _, _, _, _, _) => {
                            h3826
                        }
                    };
                    let a3829 = match v2.clone() {
                        Adt2::C0(_, h3827, _, _, _, _, _, _, _, _, _) => {
                            h3827
                        }
                    };
                    nat_lt(a3828, a3829)
                } else {
                    false
                } {
                    let a3832 = match v2.clone() {
                        Adt2::C0(_, _, h3830, _, _, _, _, _, _, _, _) => {
                            h3830
                        }
                    };
                    let a3833 = match v3.clone() {
                        Adt2::C0(_, _, h3831, _, _, _, _, _, _, _, _) => {
                            h3831
                        }
                    };
                    if if if nat_eq(a3832, a3833) {
                        let a3836 = match v2.clone() {
                            Adt2::C0(_, _, _, h3834, _, _, _, _, _, _, _) => {
                                h3834
                            }
                        };
                        let a3837 = match v3.clone() {
                            Adt2::C0(_, _, _, h3835, _, _, _, _, _, _, _) => {
                                h3835
                            }
                        };
                        nat_eq(a3836, a3837)
                    } else {
                        false
                    } {
                        let a3840 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3838, _, _, _, _, _, _) => {
                                h3838
                            }
                        };
                        let a3841 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3839, _, _, _, _, _, _) => {
                                h3839
                            }
                        };
                        nat_eq(a3840, a3841)
                    } else {
                        false
                    } {
                        let a3850 = match v3.clone() {
                            Adt2::C0(_, h3842, _, _, _, _, _, _, _, _, _) => {
                                h3842
                            }
                        };
                        let a3851 = {
                            let a3848 = {
                                let a3845 = match v2.clone() {
                                    Adt2::C0(_, h3843, _, _, _, _, _, _, _, _, _) => {
                                        h3843
                                    }
                                };
                                let a3846 = match v1.clone() {
                                    Adt1::C0(h3844, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3844
                                    }
                                };
                                nat_add(a3845, a3846)?
                            };
                            let a3849 = match v1.clone() {
                                Adt1::C0(_, h3847, _, _, _, _, _, _, _, _, _, _) => {
                                    h3847
                                }
                            };
                            nat_add(a3848, a3849)?
                        };
                        nat_eq(a3850, a3851)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3852, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3852
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3853 = v0;
                let a3854 = 63u64;
                if nat_eq(a3853, a3854) {
                    if if if if if f223(match v2.clone() {
                        Adt2::C0(_, h3855, _, _, _, _, _, _, _, _, _) => {
                            h3855
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3856, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3856
                        }
                    })? {
                        f223(match v2.clone() {
                            Adt2::C0(_, _, h3857, _, _, _, _, _, _, _, _) => {
                                h3857
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3858, _, _, _, _, _, _, _, _, _, _) => {
                                h3858
                            }
                        })?
                    } else {
                        false
                    } {
                        f223(match v2.clone() {
                            Adt2::C0(_, _, _, h3859, _, _, _, _, _, _, _) => {
                                h3859
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3860, _, _, _, _, _, _, _, _, _) => {
                                h3860
                            }
                        })?
                    } else {
                        false
                    } {
                        f223(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3861, _, _, _, _, _, _) => {
                                h3861
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3862, _, _, _, _, _, _, _, _) => {
                                h3862
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3865 = match v2.clone() {
                            Adt2::C0(_, h3863, _, _, _, _, _, _, _, _, _) => {
                                h3863
                            }
                        };
                        let a3866 = match v3.clone() {
                            Adt2::C0(_, h3864, _, _, _, _, _, _, _, _, _) => {
                                h3864
                            }
                        };
                        if if if nat_eq(a3865, a3866) {
                            let a3869 = match v2.clone() {
                                Adt2::C0(_, _, h3867, _, _, _, _, _, _, _, _) => {
                                    h3867
                                }
                            };
                            let a3870 = match v3.clone() {
                                Adt2::C0(_, _, h3868, _, _, _, _, _, _, _, _) => {
                                    h3868
                                }
                            };
                            nat_eq(a3869, a3870)
                        } else {
                            false
                        } {
                            let a3873 = match v2.clone() {
                                Adt2::C0(_, _, _, h3871, _, _, _, _, _, _, _) => {
                                    h3871
                                }
                            };
                            let a3874 = match v3.clone() {
                                Adt2::C0(_, _, _, h3872, _, _, _, _, _, _, _) => {
                                    h3872
                                }
                            };
                            nat_eq(a3873, a3874)
                        } else {
                            false
                        } {
                            let a3877 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3875, _, _, _, _, _, _) => {
                                    h3875
                                }
                            };
                            let a3878 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3876, _, _, _, _, _, _) => {
                                    h3876
                                }
                            };
                            nat_eq(a3877, a3878)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3879, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3879
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3880, _, _, _, _, _, _, _, _, _, _) => {
                                h3880
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3881, _, _, _, _, _, _, _, _, _) => {
                                h3881
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3882, _, _, _, _, _, _, _, _) => {
                                h3882
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

pub fn f125(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3883 = v3;
    match m3883 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3884 = v0;
                let a3885 = 4u64;
                nat_eq(a3884, a3885)
            };
            let v7: bool = {
                let a3887 = 0u64;
                let a3888 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3886, _, _, _, _, _, _, _) => {
                        h3886
                    }
                };
                nat_lt(a3887, a3888)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3889, _, _, _, _, _, _, _, _, _) => {
                    h3889
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3890, _, _, _, _, _, _, _, _) => {
                    h3890
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3891, _, _, _, _, _, _, _) => {
                    h3891
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3892, _, _, _, _, _, _) => {
                    h3892
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3893, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3893
                }
            }, match v1.clone() {
                Adt1::C0(_, h3894, _, _, _, _, _, _, _, _, _, _) => {
                    h3894
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3895, _, _, _, _, _, _, _, _, _) => {
                    h3895
                }
            });
            if if if if if if if if if if if if v6 {
                let a3897 = match v2.clone() {
                    Adt2::C0(h3896, _, _, _, _, _, _, _, _, _, _) => {
                        h3896
                    }
                };
                let a3898 = 0u64;
                if nat_eq(a3897, a3898) {
                    let a3900 = match v5.clone() {
                        Adt2::C0(h3899, _, _, _, _, _, _, _, _, _, _) => {
                            h3899
                        }
                    };
                    let a3901 = 0u64;
                    nat_eq(a3900, a3901)
                } else {
                    false
                }
            } else {
                let a3903 = match v2.clone() {
                    Adt2::C0(h3902, _, _, _, _, _, _, _, _, _, _) => {
                        h3902
                    }
                };
                let a3904 = 0u64;
                if if nat_eq(a3903, a3904) {
                    true
                } else {
                    let a3906 = match v2.clone() {
                        Adt2::C0(h3905, _, _, _, _, _, _, _, _, _, _) => {
                            h3905
                        }
                    };
                    let a3907 = 26u64;
                    nat_eq(a3906, a3907)
                } {
                    let a3909 = match v5.clone() {
                        Adt2::C0(h3908, _, _, _, _, _, _, _, _, _, _) => {
                            h3908
                        }
                    };
                    let a3910 = 0u64;
                    if nat_eq(a3909, a3910) {
                        true
                    } else {
                        let a3912 = match v5.clone() {
                            Adt2::C0(h3911, _, _, _, _, _, _, _, _, _, _) => {
                                h3911
                            }
                        };
                        let a3913 = 26u64;
                        nat_eq(a3912, a3913)
                    }
                } else {
                    false
                }
            } {
                let a3916 = match v2.clone() {
                    Adt2::C0(h3914, _, _, _, _, _, _, _, _, _, _) => {
                        h3914
                    }
                };
                let a3917 = match v4.clone() {
                    Adt2::C0(h3915, _, _, _, _, _, _, _, _, _, _) => {
                        h3915
                    }
                };
                nat_eq(a3916, a3917)
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
                let a3919 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3918, _, _, _, _, _) => {
                        h3918
                    }
                };
                let a3920 = 4u64;
                nat_eq(a3919, a3920)
            } else {
                false
            } {
                let a3921 = v8;
                let a3922 = f23(v2.clone(), 1u64)?;
                nat_le(a3921, a3922)
            } else {
                false
            } {
                let a3924 = match v1.clone() {
                    Adt1::C0(_, _, _, h3923, _, _, _, _, _, _, _, _) => {
                        h3923
                    }
                };
                let a3925 = 1073741824u64;
                nat_lt(a3924, a3925)
            } else {
                false
            } {
                let a3927 = match v1.clone() {
                    Adt1::C0(h3926, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3926
                    }
                };
                let a3928 = 4294967295u64;
                if if if if nat_le(a3927, a3928) {
                    let a3930 = match v1.clone() {
                        Adt1::C0(_, h3929, _, _, _, _, _, _, _, _, _, _) => {
                            h3929
                        }
                    };
                    let a3931 = 4294967295u64;
                    nat_le(a3930, a3931)
                } else {
                    false
                } {
                    let a3933 = match v1.clone() {
                        Adt1::C0(_, _, h3932, _, _, _, _, _, _, _, _, _) => {
                            h3932
                        }
                    };
                    let a3934 = 4294967295u64;
                    nat_le(a3933, a3934)
                } else {
                    false
                } {
                    let a3936 = match v1.clone() {
                        Adt1::C0(_, _, _, h3935, _, _, _, _, _, _, _, _) => {
                            h3935
                        }
                    };
                    let a3937 = 4294967295u64;
                    nat_le(a3936, a3937)
                } else {
                    false
                } {
                    let a3939 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3938, _, _, _, _, _, _, _) => {
                            h3938
                        }
                    };
                    let a3940 = 4294967295u64;
                    nat_le(a3939, a3940)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a3943 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3941) => {
                            h3941
                        }
                    };
                    let a3944 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3942) => {
                            h3942
                        }
                    };
                    nat_eq(a3943, a3944)
                } else {
                    true
                }
            } else {
                false
            } {
                f224(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3945, _, _, _, _, _, _, _, _) => {
                        h3945
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3946, _) => {
                        h3946
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
                    let a3947 = v10;
                    let a3948 = v8;
                    nat_add(a3947, a3948)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3949, _, _, _, _, _, _, _, _) => {
                        h3949
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f126(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3951 = match v0.clone() {
        Adt2::C0(h3950, _, _, _, _, _, _, _, _, _, _) => {
            h3950
        }
    };
    let a3952 = 0u64;
    if if if nat_eq(a3951, a3952) {
        let a3954 = match v1.clone() {
            Adt2::C0(h3953, _, _, _, _, _, _, _, _, _, _) => {
                h3953
            }
        };
        let a3955 = 0u64;
        nat_eq(a3954, a3955)
    } else {
        false
    } {
        f220(match v0.clone() {
            Adt2::C0(_, h3956, _, _, _, _, _, _, _, _, _) => {
                h3956
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3957, _, _, _, _, _, _, _, _) => {
                h3957
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3958, _, _, _, _, _, _, _) => {
                h3958
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3959, _, _, _, _, _, _) => {
                h3959
            }
        }, match v1.clone() {
            Adt2::C0(_, h3960, _, _, _, _, _, _, _, _, _) => {
                h3960
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3961, _, _, _, _, _, _, _, _) => {
                h3961
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3962, _, _, _, _, _, _, _) => {
                h3962
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3963, _, _, _, _, _, _) => {
                h3963
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3964 = v2;
            let a3965 = f23(v0.clone(), 1u64)?;
            nat_add(a3964, a3965)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f127(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3967 = match v1.clone() {
        Adt2::C0(h3966, _, _, _, _, _, _, _, _, _, _) => {
            h3966
        }
    };
    let a3968 = 0u64;
    if if if nat_eq(a3967, a3968) {
        true
    } else {
        let a3970 = match v1.clone() {
            Adt2::C0(h3969, _, _, _, _, _, _, _, _, _, _) => {
                h3969
            }
        };
        let a3971 = 1u64;
        nat_eq(a3970, a3971)
    } {
        true
    } else {
        let a3973 = match v1.clone() {
            Adt2::C0(h3972, _, _, _, _, _, _, _, _, _, _) => {
                h3972
            }
        };
        let a3974 = 30u64;
        nat_eq(a3973, a3974)
    } {
        let a3977 = match v1.clone() {
            Adt2::C0(h3975, _, _, _, _, _, _, _, _, _, _) => {
                h3975
            }
        };
        let a3978 = match v2.clone() {
            Adt2::C0(h3976, _, _, _, _, _, _, _, _, _, _) => {
                h3976
            }
        };
        if if if if if nat_eq(a3977, a3978) {
            let a3981 = match v1.clone() {
                Adt2::C0(_, h3979, _, _, _, _, _, _, _, _, _) => {
                    h3979
                }
            };
            let a3982 = match v2.clone() {
                Adt2::C0(_, h3980, _, _, _, _, _, _, _, _, _) => {
                    h3980
                }
            };
            nat_eq(a3981, a3982)
        } else {
            false
        } {
            let a3985 = match v1.clone() {
                Adt2::C0(_, _, _, h3983, _, _, _, _, _, _, _) => {
                    h3983
                }
            };
            let a3986 = match v2.clone() {
                Adt2::C0(_, _, _, h3984, _, _, _, _, _, _, _) => {
                    h3984
                }
            };
            nat_eq(a3985, a3986)
        } else {
            false
        } {
            let a3989 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3987, _, _, _, _, _, _) => {
                    h3987
                }
            };
            let a3990 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3988, _, _, _, _, _, _) => {
                    h3988
                }
            };
            nat_eq(a3989, a3990)
        } else {
            false
        } {
            let a3991 = v0;
            let a3992 = 4u64;
            if nat_eq(a3991, a3992) {
                let a3994 = match v1.clone() {
                    Adt2::C0(_, _, h3993, _, _, _, _, _, _, _, _) => {
                        h3993
                    }
                };
                let a3995 = 1u64;
                if nat_eq(a3994, a3995) {
                    let a3998 = match v1.clone() {
                        Adt2::C0(_, h3996, _, _, _, _, _, _, _, _, _) => {
                            h3996
                        }
                    };
                    let a3999 = match v2.clone() {
                        Adt2::C0(_, _, h3997, _, _, _, _, _, _, _, _) => {
                            h3997
                        }
                    };
                    nat_eq(a3998, a3999)
                } else {
                    false
                }
            } else {
                let a4000 = v0;
                let a4001 = 4u64;
                if if nat_lt(a4000, a4001) {
                    let a4004 = match v1.clone() {
                        Adt2::C0(_, h4002, _, _, _, _, _, _, _, _, _) => {
                            h4002
                        }
                    };
                    let a4005 = match v1.clone() {
                        Adt2::C0(_, _, h4003, _, _, _, _, _, _, _, _) => {
                            h4003
                        }
                    };
                    nat_eq(a4004, a4005)
                } else {
                    false
                } {
                    let a4008 = match v1.clone() {
                        Adt2::C0(_, _, h4006, _, _, _, _, _, _, _, _) => {
                            h4006
                        }
                    };
                    let a4009 = match v2.clone() {
                        Adt2::C0(_, _, h4007, _, _, _, _, _, _, _, _) => {
                            h4007
                        }
                    };
                    nat_eq(a4008, a4009)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4011 = f210(match v1.clone() {
                Adt2::C0(h4010, _, _, _, _, _, _, _, _, _, _) => {
                    h4010
                }
            });
            match m4011 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4012) => {
                            h4012
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

pub fn f128(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f225(match v1.clone() {
        Adt2::C0(h4013, _, _, _, _, _, _, _, _, _, _) => {
            h4013
        }
    }, v0) {
        let a4016 = match v1.clone() {
            Adt2::C0(h4014, _, _, _, _, _, _, _, _, _, _) => {
                h4014
            }
        };
        let a4017 = match v2.clone() {
            Adt2::C0(h4015, _, _, _, _, _, _, _, _, _, _) => {
                h4015
            }
        };
        nat_eq(a4016, a4017)
    } else {
        false
    } {
        let a4020 = match v1.clone() {
            Adt2::C0(_, h4018, _, _, _, _, _, _, _, _, _) => {
                h4018
            }
        };
        let a4021 = match v2.clone() {
            Adt2::C0(_, h4019, _, _, _, _, _, _, _, _, _) => {
                h4019
            }
        };
        if if if nat_eq(a4020, a4021) {
            let a4024 = match v1.clone() {
                Adt2::C0(_, _, h4022, _, _, _, _, _, _, _, _) => {
                    h4022
                }
            };
            let a4025 = match v2.clone() {
                Adt2::C0(_, _, h4023, _, _, _, _, _, _, _, _) => {
                    h4023
                }
            };
            nat_eq(a4024, a4025)
        } else {
            false
        } {
            let a4028 = match v1.clone() {
                Adt2::C0(_, _, _, h4026, _, _, _, _, _, _, _) => {
                    h4026
                }
            };
            let a4029 = match v2.clone() {
                Adt2::C0(_, _, _, h4027, _, _, _, _, _, _, _) => {
                    h4027
                }
            };
            nat_eq(a4028, a4029)
        } else {
            false
        } {
            let a4032 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4030, _, _, _, _, _, _) => {
                    h4030
                }
            };
            let a4033 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4031, _, _, _, _, _, _) => {
                    h4031
                }
            };
            nat_eq(a4032, a4033)
        } else {
            false
        }
    } else {
        false
    } {
        let m4035 = f210(match v1.clone() {
            Adt2::C0(h4034, _, _, _, _, _, _, _, _, _, _) => {
                h4034
            }
        });
        match m4035 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4036) => {
                        h4036
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4037, _, _, _, _, _, _, _, _, _, _) => {
                        h4037
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f129(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4038, _, _, _, _, _, _, _, _, _, _, _) => {
                h4038
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4039, _, _, _, _, _, _, _, _, _) => {
                h4039
            }
        }
    };
    let a4040 = 0u64;
    let a4041 = v4;
    if if nat_lt(a4040, a4041) {
        let a4042 = v4;
        let a4043 = 2147483648u64;
        nat_lt(a4042, a4043)
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
        let v7: u64 = f226(match v5.clone() {
            Adt2::C0(_, _, h4044, _, _, _, _, _, _, _, _) => {
                h4044
            }
        }, v4)?;
        let v8: u64 = f226(match v5.clone() {
            Adt2::C0(_, _, _, h4045, _, _, _, _, _, _, _) => {
                h4045
            }
        }, v4)?;
        let a4047 = match v5.clone() {
            Adt2::C0(h4046, _, _, _, _, _, _, _, _, _, _) => {
                h4046
            }
        };
        let a4048 = 0u64;
        if if if if if nat_eq(a4047, a4048) {
            let a4050 = match v6.clone() {
                Adt2::C0(h4049, _, _, _, _, _, _, _, _, _, _) => {
                    h4049
                }
            };
            let a4051 = 0u64;
            nat_eq(a4050, a4051)
        } else {
            false
        } {
            let a4053 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4052, _, _, _, _, _, _) => {
                    h4052
                }
            };
            let a4054 = 1u64;
            nat_eq(a4053, a4054)
        } else {
            false
        } {
            let a4057 = match v5.clone() {
                Adt2::C0(_, h4055, _, _, _, _, _, _, _, _, _) => {
                    h4055
                }
            };
            let a4058 = match v6.clone() {
                Adt2::C0(_, h4056, _, _, _, _, _, _, _, _, _) => {
                    h4056
                }
            };
            nat_eq(a4057, a4058)
        } else {
            false
        } {
            if v0 {
                let a4060 = match v5.clone() {
                    Adt2::C0(_, _, h4059, _, _, _, _, _, _, _, _) => {
                        h4059
                    }
                };
                let a4061 = 2147483648u64;
                if if if nat_lt(a4060, a4061) {
                    let a4063 = match v5.clone() {
                        Adt2::C0(_, _, _, h4062, _, _, _, _, _, _, _) => {
                            h4062
                        }
                    };
                    let a4064 = 2147483648u64;
                    nat_lt(a4063, a4064)
                } else {
                    false
                } {
                    let a4067 = {
                        let a4065 = v7;
                        let a4066 = v8;
                        nat_mul(a4065, a4066)?
                    };
                    let a4068 = 2147483648u64;
                    nat_le(a4067, a4068)
                } else {
                    false
                } {
                    f227(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4070 = match v1.clone() {
                    Adt1::C0(h4069, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4069
                    }
                };
                let a4071 = v7;
                if if if if nat_eq(a4070, a4071) {
                    let a4073 = match v1 {
                        Adt1::C0(_, h4072, _, _, _, _, _, _, _, _, _, _) => {
                            h4072
                        }
                    };
                    let a4074 = v8;
                    nat_eq(a4073, a4074)
                } else {
                    false
                } {
                    f228(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4076 = match v6.clone() {
                        Adt2::C0(_, _, h4075, _, _, _, _, _, _, _, _) => {
                            h4075
                        }
                    };
                    let a4077 = v4;
                    if nat_eq(a4076, a4077) {
                        let a4079 = match v6.clone() {
                            Adt2::C0(_, _, _, h4078, _, _, _, _, _, _, _) => {
                                h4078
                            }
                        };
                        let a4080 = v4;
                        nat_eq(a4079, a4080)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4084 = match v6 {
                        Adt2::C0(_, _, _, _, h4081, _, _, _, _, _, _) => {
                            h4081
                        }
                    };
                    let a4085 = {
                        let a4082 = v7;
                        let a4083 = v8;
                        nat_mul(a4082, a4083)?
                    };
                    nat_eq(a4084, a4085)
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

pub fn f130(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4087 = match v2.clone() {
        Adt2::C0(h4086, _, _, _, _, _, _, _, _, _, _) => {
            h4086
        }
    };
    let a4088 = 0u64;
    if if if if if nat_eq(a4087, a4088) {
        true
    } else {
        let a4090 = match v2.clone() {
            Adt2::C0(h4089, _, _, _, _, _, _, _, _, _, _) => {
                h4089
            }
        };
        let a4091 = 1u64;
        nat_eq(a4090, a4091)
    } {
        true
    } else {
        let a4093 = match v2.clone() {
            Adt2::C0(h4092, _, _, _, _, _, _, _, _, _, _) => {
                h4092
            }
        };
        let a4094 = 30u64;
        nat_eq(a4093, a4094)
    } {
        let a4097 = match v2.clone() {
            Adt2::C0(h4095, _, _, _, _, _, _, _, _, _, _) => {
                h4095
            }
        };
        let a4098 = match v3.clone() {
            Adt2::C0(h4096, _, _, _, _, _, _, _, _, _, _) => {
                h4096
            }
        };
        nat_eq(a4097, a4098)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a4100 = match v1.clone() {
                Adt1::C0(h4099, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4099
                }
            };
            let a4101 = 4294967296u64;
            nat_lt(a4100, a4101)
        } else {
            false
        } {
            let a4103 = match v1.clone() {
                Adt1::C0(_, h4102, _, _, _, _, _, _, _, _, _, _) => {
                    h4102
                }
            };
            let a4104 = 4294967296u64;
            nat_lt(a4103, a4104)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4106 = match v2.clone() {
                Adt2::C0(h4105, _, _, _, _, _, _, _, _, _, _) => {
                    h4105
                }
            };
            let a4107 = 0u64;
            if nat_eq(a4106, a4107) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a4108 = v0;
            let a4109 = 31u64;
            if nat_eq(a4108, a4109) {
                0u64
            } else {
                let a4110 = v0;
                let a4111 = 47u64;
                if nat_eq(a4110, a4111) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4112, _, _, _, _, _, _, _, _, _, _) => {
                h4112
            }
        }, match v1.clone() {
            Adt1::C0(h4113, _, _, _, _, _, _, _, _, _, _, _) => {
                h4113
            }
        }, match v1 {
            Adt1::C0(_, h4114, _, _, _, _, _, _, _, _, _, _) => {
                h4114
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f131(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4116 = match v0.clone() {
        Adt2::C0(h4115, _, _, _, _, _, _, _, _, _, _) => {
            h4115
        }
    };
    let a4117 = 0u64;
    if if if nat_eq(a4116, a4117) {
        let a4119 = match v1.clone() {
            Adt2::C0(h4118, _, _, _, _, _, _, _, _, _, _) => {
                h4118
            }
        };
        let a4120 = 0u64;
        nat_eq(a4119, a4120)
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

pub fn f132(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4122 = match v0.clone() {
        Adt2::C0(h4121, _, _, _, _, _, _, _, _, _, _) => {
            h4121
        }
    };
    let a4123 = 0u64;
    if if nat_eq(a4122, a4123) {
        let a4125 = match v1.clone() {
            Adt2::C0(h4124, _, _, _, _, _, _, _, _, _, _) => {
                h4124
            }
        };
        let a4126 = 26u64;
        nat_eq(a4125, a4126)
    } else {
        false
    } {
        let a4128 = match v0.clone() {
            Adt2::C0(_, _, _, h4127, _, _, _, _, _, _, _) => {
                h4127
            }
        };
        let a4129 = 1u64;
        if if if if if if if nat_eq(a4128, a4129) {
            let a4131 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4130, _, _, _, _, _, _) => {
                    h4130
                }
            };
            let a4132 = 1u64;
            nat_eq(a4131, a4132)
        } else {
            false
        } {
            let a4134 = match v0.clone() {
                Adt2::C0(_, h4133, _, _, _, _, _, _, _, _, _) => {
                    h4133
                }
            };
            let a4135 = 2147483647u64;
            nat_le(a4134, a4135)
        } else {
            false
        } {
            let a4138 = match v1.clone() {
                Adt2::C0(_, h4136, _, _, _, _, _, _, _, _, _) => {
                    h4136
                }
            };
            let a4139 = match v0.clone() {
                Adt2::C0(_, _, h4137, _, _, _, _, _, _, _, _) => {
                    h4137
                }
            };
            nat_eq(a4138, a4139)
        } else {
            false
        } {
            let a4141 = match v1.clone() {
                Adt2::C0(_, _, h4140, _, _, _, _, _, _, _, _) => {
                    h4140
                }
            };
            let a4142 = 1u64;
            nat_eq(a4141, a4142)
        } else {
            false
        } {
            let a4144 = match v1.clone() {
                Adt2::C0(_, _, _, h4143, _, _, _, _, _, _, _) => {
                    h4143
                }
            };
            let a4145 = 1u64;
            nat_eq(a4144, a4145)
        } else {
            false
        } {
            let a4147 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4146, _, _, _, _, _, _) => {
                    h4146
                }
            };
            let a4148 = 1u64;
            nat_eq(a4147, a4148)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4151 = match v0.clone() {
                    Adt2::C0(_, _, h4149, _, _, _, _, _, _, _, _) => {
                        h4149
                    }
                };
                let a4152 = f170(match v0.clone() {
                    Adt2::C0(_, h4150, _, _, _, _, _, _, _, _, _) => {
                        h4150
                    }
                })?;
                nat_mul(a4151, a4152)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f133(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4153 = v1;
    match m4153 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f175(f25(v0.clone(), 1u64))?;
            let a4155 = match v0.clone() {
                Adt2::C0(h4154, _, _, _, _, _, _, _, _, _, _) => {
                    h4154
                }
            };
            let a4156 = 26u64;
            if if if nat_eq(a4155, a4156) {
                let a4158 = match v3.clone() {
                    Adt2::C0(h4157, _, _, _, _, _, _, _, _, _, _) => {
                        h4157
                    }
                };
                let a4159 = 26u64;
                nat_eq(a4158, a4159)
            } else {
                false
            } {
                let a4161 = match v2.clone() {
                    Adt2::C0(h4160, _, _, _, _, _, _, _, _, _, _) => {
                        h4160
                    }
                };
                let a4162 = 27u64;
                nat_eq(a4161, a4162)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a4164 = match v2.clone() {
                        Adt2::C0(_, h4163, _, _, _, _, _, _, _, _, _) => {
                            h4163
                        }
                    };
                    let a4165 = 1u64;
                    nat_eq(a4164, a4165)
                } else {
                    false
                } {
                    let a4167 = match v2.clone() {
                        Adt2::C0(_, _, h4166, _, _, _, _, _, _, _, _) => {
                            h4166
                        }
                    };
                    let a4168 = 1u64;
                    nat_eq(a4167, a4168)
                } else {
                    false
                } {
                    let a4170 = match v2.clone() {
                        Adt2::C0(_, _, _, h4169, _, _, _, _, _, _, _) => {
                            h4169
                        }
                    };
                    let a4171 = 1u64;
                    nat_eq(a4170, a4171)
                } else {
                    false
                } {
                    let a4173 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4172, _, _, _, _, _, _) => {
                            h4172
                        }
                    };
                    let a4174 = 1u64;
                    nat_eq(a4173, a4174)
                } else {
                    false
                } {
                    let a4175 = v4;
                    let a4176 = 9223372036854775808u64;
                    nat_lt(a4175, a4176)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4179 = 2u64;
                        let a4180 = {
                            let a4177 = v4;
                            let a4178 = 2u64;
                            nat_mul(a4177, a4178)?
                        };
                        nat_add(a4179, a4180)?
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

pub fn f134(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4181, _, _, _, _, _, _, _, _, _, _) => {
            h4181
        }
    };
    let a4182 = v3;
    let a4183 = 0u64;
    if if nat_eq(a4182, a4183) {
        true
    } else {
        let a4184 = v0;
        let a4185 = 13u64;
        if nat_eq(a4184, a4185) {
            let a4186 = v3;
            let a4187 = 1u64;
            if nat_eq(a4186, a4187) {
                true
            } else {
                let a4188 = v3;
                let a4189 = 30u64;
                nat_eq(a4188, a4189)
            }
        } else {
            false
        }
    } {
        let a4191 = v3;
        let a4192 = match v2.clone() {
            Adt2::C0(h4190, _, _, _, _, _, _, _, _, _, _) => {
                h4190
            }
        };
        if if nat_eq(a4191, a4192) {
            let a4193 = v0;
            let a4194 = 13u64;
            if nat_eq(a4193, a4194) {
                let a4196 = match v2.clone() {
                    Adt2::C0(_, h4195, _, _, _, _, _, _, _, _, _) => {
                        h4195
                    }
                };
                let a4197 = 1u64;
                if if if nat_eq(a4196, a4197) {
                    let a4199 = match v2.clone() {
                        Adt2::C0(_, _, h4198, _, _, _, _, _, _, _, _) => {
                            h4198
                        }
                    };
                    let a4200 = 1u64;
                    nat_eq(a4199, a4200)
                } else {
                    false
                } {
                    let a4202 = match v2.clone() {
                        Adt2::C0(_, _, _, h4201, _, _, _, _, _, _, _) => {
                            h4201
                        }
                    };
                    let a4203 = 1u64;
                    nat_eq(a4202, a4203)
                } else {
                    false
                } {
                    let a4205 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4204, _, _, _, _, _, _) => {
                            h4204
                        }
                    };
                    let a4206 = 1u64;
                    nat_eq(a4205, a4206)
                } else {
                    false
                }
            } else {
                let a4208 = match v2.clone() {
                    Adt2::C0(_, h4207, _, _, _, _, _, _, _, _, _) => {
                        h4207
                    }
                };
                let a4209 = 1u64;
                if if if nat_eq(a4208, a4209) {
                    let a4212 = match v1.clone() {
                        Adt2::C0(_, _, h4210, _, _, _, _, _, _, _, _) => {
                            h4210
                        }
                    };
                    let a4213 = match v2.clone() {
                        Adt2::C0(_, _, h4211, _, _, _, _, _, _, _, _) => {
                            h4211
                        }
                    };
                    nat_eq(a4212, a4213)
                } else {
                    false
                } {
                    let a4216 = match v1.clone() {
                        Adt2::C0(_, _, _, h4214, _, _, _, _, _, _, _) => {
                            h4214
                        }
                    };
                    let a4217 = match v2.clone() {
                        Adt2::C0(_, _, _, h4215, _, _, _, _, _, _, _) => {
                            h4215
                        }
                    };
                    nat_eq(a4216, a4217)
                } else {
                    false
                } {
                    let a4220 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4218, _, _, _, _, _, _) => {
                            h4218
                        }
                    };
                    let a4221 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4219, _, _, _, _, _, _) => {
                            h4219
                        }
                    };
                    nat_eq(a4220, a4221)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt10::C4(f229(v4.clone(), v0, v3)?, {
                let a4222 = v3;
                let a4223 = 0u64;
                if nat_eq(a4222, a4223) {
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

pub fn f135(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4224 = v0;
    let a4225 = 89u64;
    if if nat_eq(a4224, a4225) {
        let a4226 = v1;
        let a4227 = 16u64;
        nat_eq(a4226, a4227)
    } else {
        false
    } {
        Ok(Some(24u64))
    } else {
        let a4228 = v0;
        let a4229 = 89u64;
        if if nat_eq(a4228, a4229) {
            let a4230 = v1;
            let a4231 = 14u64;
            nat_eq(a4230, a4231)
        } else {
            false
        } {
            Ok(Some(23u64))
        } else {
            let a4232 = v0;
            let a4233 = 89u64;
            if if nat_eq(a4232, a4233) {
                let a4234 = v1;
                let a4235 = 15u64;
                nat_eq(a4234, a4235)
            } else {
                false
            } {
                Ok(Some(22u64))
            } else {
                let a4236 = v0;
                let a4237 = 89u64;
                if if nat_eq(a4236, a4237) {
                    let a4238 = 8u64;
                    let a4239 = v1;
                    if nat_le(a4238, a4239) {
                        let a4240 = v1;
                        let a4241 = 9u64;
                        nat_le(a4240, a4241)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some({
                        let a4242 = v1;
                        let a4243 = 12u64;
                        nat_add(a4242, a4243)?
                    }))
                } else {
                    let a4244 = v0;
                    let a4245 = 89u64;
                    if if nat_eq(a4244, a4245) {
                        let a4246 = v1;
                        let a4247 = 4u64;
                        nat_eq(a4246, a4247)
                    } else {
                        false
                    } {
                        Ok(Some(19u64))
                    } else {
                        let a4248 = 10u64;
                        let a4249 = v0;
                        if if nat_le(a4248, a4249) {
                            let a4250 = v0;
                            let a4251 = 12u64;
                            nat_le(a4250, a4251)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4252 = v0;
                                let a4253 = 3u64;
                                nat_add(a4252, a4253)?
                            }))
                        } else {
                            let a4254 = v0;
                            let a4255 = 89u64;
                            if if nat_eq(a4254, a4255) {
                                let a4256 = v1;
                                let a4257 = 7u64;
                                nat_eq(a4256, a4257)
                            } else {
                                false
                            } {
                                Ok(Some(17u64))
                            } else {
                                let a4258 = v0;
                                let a4259 = 89u64;
                                if if nat_eq(a4258, a4259) {
                                    let a4260 = v1;
                                    let a4261 = 10u64;
                                    nat_eq(a4260, a4261)
                                } else {
                                    false
                                } {
                                    Ok(Some(18u64))
                                } else {
                                    let a4262 = v0;
                                    let a4263 = 89u64;
                                    if if nat_eq(a4262, a4263) {
                                        let a4264 = v1;
                                        let a4265 = 13u64;
                                        nat_eq(a4264, a4265)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(16u64))
                                    } else {
                                        let a4266 = v0;
                                        let a4267 = 8u64;
                                        if nat_eq(a4266, a4267) {
                                            Ok(Some(5u64))
                                        } else {
                                            let a4268 = v0;
                                            let a4269 = 9u64;
                                            if nat_eq(a4268, a4269) {
                                                Ok(Some(6u64))
                                            } else {
                                                let a4270 = v0;
                                                let a4271 = 89u64;
                                                if nat_eq(a4270, a4271) {
                                                    let a4272 = v1;
                                                    let a4273 = 0u64;
                                                    if nat_eq(a4272, a4273) {
                                                        Ok(Some(0u64))
                                                    } else {
                                                        let a4274 = v1;
                                                        let a4275 = 1u64;
                                                        if nat_eq(a4274, a4275) {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            let a4276 = v1;
                                                            let a4277 = 2u64;
                                                            if nat_eq(a4276, a4277) {
                                                                Ok(Some(2u64))
                                                            } else {
                                                                let a4278 = v1;
                                                                let a4279 = 3u64;
                                                                if nat_eq(a4278, a4279) {
                                                                    Ok(Some(3u64))
                                                                } else {
                                                                    let a4280 = v1;
                                                                    let a4281 = 6u64;
                                                                    if nat_eq(a4280, a4281) {
                                                                        Ok(Some(4u64))
                                                                    } else {
                                                                        let a4282 = v1;
                                                                        let a4283 = 12u64;
                                                                        if nat_eq(a4282, a4283) {
                                                                            Ok(Some(7u64))
                                                                        } else {
                                                                            let a4284 = v1;
                                                                            let a4285 = 11u64;
                                                                            if nat_eq(a4284, a4285) {
                                                                                Ok(Some(8u64))
                                                                            } else {
                                                                                let a4286 = v1;
                                                                                let a4287 = 18u64;
                                                                                if nat_eq(a4286, a4287) {
                                                                                    Ok(Some(9u64))
                                                                                } else {
                                                                                    let a4288 = v1;
                                                                                    let a4289 = 19u64;
                                                                                    if nat_eq(a4288, a4289) {
                                                                                        Ok(Some(10u64))
                                                                                    } else {
                                                                                        let a4290 = v1;
                                                                                        let a4291 = 20u64;
                                                                                        if nat_eq(a4290, a4291) {
                                                                                            Ok(Some(11u64))
                                                                                        } else {
                                                                                            let a4292 = v1;
                                                                                            let a4293 = 21u64;
                                                                                            if nat_eq(a4292, a4293) {
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

pub fn f136(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4294 = f230(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4294 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4295 = v0;
            let a4296 = 2u64;
            if if if if nat_eq(a4295, a4296) {
                true
            } else {
                let a4297 = v0;
                let a4298 = 5u64;
                nat_eq(a4297, a4298)
            } {
                true
            } else {
                let a4299 = v0;
                let a4300 = 6u64;
                nat_eq(a4299, a4300)
            } {
                true
            } else {
                let a4301 = v0;
                let a4302 = 7u64;
                nat_eq(a4301, a4302)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4303 = v3;
                    match m4303 {
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

pub fn f137(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4305 = match v1.clone() {
        Adt2::C0(h4304, _, _, _, _, _, _, _, _, _, _) => {
            h4304
        }
    };
    let a4306 = 0u64;
    if if if if if nat_eq(a4305, a4306) {
        true
    } else {
        let a4308 = match v1.clone() {
            Adt2::C0(h4307, _, _, _, _, _, _, _, _, _, _) => {
                h4307
            }
        };
        let a4309 = 1u64;
        nat_eq(a4308, a4309)
    } {
        true
    } else {
        let a4311 = match v1.clone() {
            Adt2::C0(h4310, _, _, _, _, _, _, _, _, _, _) => {
                h4310
            }
        };
        let a4312 = 30u64;
        nat_eq(a4311, a4312)
    } {
        let a4315 = match v1.clone() {
            Adt2::C0(h4313, _, _, _, _, _, _, _, _, _, _) => {
                h4313
            }
        };
        let a4316 = match v2.clone() {
            Adt2::C0(h4314, _, _, _, _, _, _, _, _, _, _) => {
                h4314
            }
        };
        if nat_eq(a4315, a4316) {
            true
        } else {
            let a4318 = match v2.clone() {
                Adt2::C0(h4317, _, _, _, _, _, _, _, _, _, _) => {
                    h4317
                }
            };
            let a4319 = 0u64;
            nat_eq(a4318, a4319)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4321 = f23(v2.clone(), 1u64)?;
            let a4322 = f231(v0, match v2.clone() {
                Adt2::C0(h4320, _, _, _, _, _, _, _, _, _, _) => {
                    h4320
                }
            });
            nat_mul(a4321, a4322)?
        }, {
            let a4324 = match v2.clone() {
                Adt2::C0(h4323, _, _, _, _, _, _, _, _, _, _) => {
                    h4323
                }
            };
            let a4325 = 0u64;
            if nat_eq(a4324, a4325) {
                4u64
            } else {
                2u64
            }
        }, f25(v1.clone(), 1u64), f25(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h4326, _, _, _, _, _, _, _, _, _, _) => {
                h4326
            }
        }, match v2 {
            Adt2::C0(h4327, _, _, _, _, _, _, _, _, _, _) => {
                h4327
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f138(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4340 = {
        let a4336 = {
            let a4332 = {
                let a4328 = v0;
                let a4329 = v4;
                nat_mul(a4328, a4329)?
            };
            let a4333 = {
                let a4330 = v1;
                let a4331 = v5;
                nat_mul(a4330, a4331)?
            };
            nat_add(a4332, a4333)?
        };
        let a4337 = {
            let a4334 = v2;
            let a4335 = v6;
            nat_mul(a4334, a4335)?
        };
        nat_add(a4336, a4337)?
    };
    let a4341 = {
        let a4338 = v3;
        let a4339 = v7;
        nat_mul(a4338, a4339)?
    };
    nat_add(a4340, a4341)
}

pub fn f139(v0: u64) -> u64 {
    let a4342 = v0;
    let a4343 = 2147483648u64;
    if nat_lt(a4342, a4343) {
        0u64
    } else {
        let a4344 = 4294967296u64;
        let a4345 = v0;
        nat_sub(a4344, a4345)
    }
}

pub fn f140(v0: u64) -> u64 {
    let a4346 = v0;
    let a4347 = 2147483648u64;
    if nat_lt(a4346, a4347) {
        v0
    } else {
        0u64
    }
}

pub fn f141(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4348 = v7;
        let a4349 = 6u64;
        if nat_eq(a4348, a4349) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f88(v3, v4), f88(v5, v6), f89(v7, v8), f169(v1), f169(v9))
}

pub fn f142(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4350 = v0;
    match m4350 {
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
            let a4351 = 13u64;
            let a4352 = v57;
            if nat_le(a4351, a4352) {
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
                Adt5::C0(_, _, _, _, _, h4353, _, _) => {
                    h4353
                }
            }, match v77 {
                Adt5::C0(_, _, _, _, _, _, _, h4354) => {
                    h4354
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
    }
}

pub fn f143(v0: Adt4) -> u64 {
    let m4355 = v0;
    match m4355 {
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
        Adt4::C41 => {
            0u64
        }
    }
}

pub fn f144(v0: Adt4, v1: u64) -> u64 {
    let m4356 = v0;
    match m4356 {
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
            let a4357 = v1;
            let a4358 = 6u64;
            if nat_eq(a4357, a4358) {
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
            let a4367 = v1;
            let a4368 = 6u64;
            if if if nat_eq(a4367, a4368) {
                true
            } else {
                let a4369 = v1;
                let a4370 = 7u64;
                nat_eq(a4369, a4370)
            } {
                let a4371 = v77;
                let a4372 = 30u64;
                nat_eq(a4371, a4372)
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
                Adt5::C0(_, _, _, _, _, _, _, h4366) => {
                    h4366
                }
            }
        }
        Adt4::C28(v72) => {
            let a4359 = v1;
            let a4360 = 6u64;
            if nat_eq(a4359, a4360) {
                match v72 {
                    Adt7::C0(_, _, _, _, h4361) => {
                        h4361
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
            let a4362 = v1;
            let a4363 = 6u64;
            if if nat_eq(a4362, a4363) {
                true
            } else {
                let a4364 = v1;
                let a4365 = 7u64;
                nat_eq(a4364, a4365)
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
        Adt4::C41 => {
            0u64
        }
    }
}

pub fn f145(v0: Adt4, v1: u64) -> Adt11 {
    let a4373 = v1;
    let a4374 = 4u64;
    if nat_lt(a4373, a4374) {
        Adt11::C0(v1)
    } else {
        let a4375 = v1;
        let a4376 = 4u64;
        if nat_eq(a4375, a4376) {
            Adt11::C1(0u64)
        } else {
            let a4377 = v1;
            let a4378 = 5u64;
            if nat_eq(a4377, a4378) {
                Adt11::C1(3u64)
            } else {
                let a4379 = v1;
                let a4380 = 6u64;
                if nat_eq(a4379, a4380) {
                    Adt11::C2(f144(v0, v1), 0u64, false)
                } else {
                    let a4381 = v1;
                    let a4382 = 7u64;
                    if nat_eq(a4381, a4382) {
                        Adt11::C2(0u64, f144(v0, v1), false)
                    } else {
                        let a4383 = v1;
                        let a4384 = 16u64;
                        if nat_lt(a4383, a4384) {
                            Adt11::C1({
                                let a4385 = v1;
                                let a4386 = 8u64;
                                nat_sub(a4385, a4386)
                            })
                        } else {
                            let a4387 = v1;
                            let a4388 = 20u64;
                            if nat_lt(a4387, a4388) {
                                Adt11::C5({
                                    let a4389 = v1;
                                    let a4390 = 16u64;
                                    nat_sub(a4389, a4390)
                                })
                            } else {
                                let a4391 = v1;
                                let a4392 = 20u64;
                                if nat_eq(a4391, a4392) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4393 = v1;
                                    let a4394 = 21u64;
                                    if nat_eq(a4393, a4394) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4395 = v1;
                                        let a4396 = 22u64;
                                        if nat_eq(a4395, a4396) {
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

pub fn f146(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4414 = {
        let a4409 = f232(v2, {
            let a4400 = {
                let a4397 = v0;
                let a4398 = v1;
                let a4399 = 0u64;
                nat_quot(a4397, a4398, a4399)
            };
            let a4401 = v2;
            let a4402 = 0u64;
            nat_rem(a4400, a4401, a4402)
        }, {
            let a4406 = {
                let a4403 = v0;
                let a4404 = v1;
                let a4405 = 0u64;
                nat_quot(a4403, a4404, a4405)
            };
            let a4407 = v2;
            let a4408 = 0u64;
            nat_quot(a4406, a4407, a4408)
        })?;
        let a4410 = v1;
        nat_mul(a4409, a4410)?
    };
    let a4415 = {
        let a4411 = v0;
        let a4412 = v1;
        let a4413 = 0u64;
        nat_rem(a4411, a4412, a4413)
    };
    nat_add(a4414, a4415)
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4417 = v0;
        let a4418 = match v2.clone() {
            Adt3::C0(h4416, _, _, _, _, _, _, _) => {
                h4416
            }
        };
        let a4419 = 0u64;
        nat_quot(a4417, a4418, a4419)
    };
    let v4: u64 = {
        let a4421 = v3;
        let a4422 = match v2.clone() {
            Adt3::C0(_, h4420, _, _, _, _, _, _) => {
                h4420
            }
        };
        let a4423 = 0u64;
        nat_quot(a4421, a4422, a4423)
    };
    f233({
        let a4425 = v0;
        let a4426 = match v2.clone() {
            Adt3::C0(h4424, _, _, _, _, _, _, _) => {
                h4424
            }
        };
        let a4427 = 0u64;
        nat_rem(a4425, a4426, a4427)
    }, {
        let a4429 = v3;
        let a4430 = match v2.clone() {
            Adt3::C0(_, h4428, _, _, _, _, _, _) => {
                h4428
            }
        };
        let a4431 = 0u64;
        nat_rem(a4429, a4430, a4431)
    }, {
        let a4433 = v4;
        let a4434 = match v2.clone() {
            Adt3::C0(_, _, h4432, _, _, _, _, _) => {
                h4432
            }
        };
        let a4435 = 0u64;
        nat_rem(a4433, a4434, a4435)
    }, {
        let a4437 = v4;
        let a4438 = match v2 {
            Adt3::C0(_, _, h4436, _, _, _, _, _) => {
                h4436
            }
        };
        let a4439 = 0u64;
        nat_quot(a4437, a4438, a4439)
    }, match v1.clone() {
        Adt3::C0(h4440, _, _, _, _, _, _, _) => {
            h4440
        }
    }, match v1.clone() {
        Adt3::C0(_, h4441, _, _, _, _, _, _) => {
            h4441
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4442, _, _, _, _, _) => {
            h4442
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4443, _, _, _, _) => {
            h4443
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4444, _, _, _) => {
            h4444
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4445, _, _) => {
            h4445
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4446, _) => {
            h4446
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4447) => {
            h4447
        }
    })
}

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4449 = v0;
        let a4450 = match v3.clone() {
            Adt3::C0(h4448, _, _, _, _, _, _, _) => {
                h4448
            }
        };
        let a4451 = 0u64;
        nat_quot(a4449, a4450, a4451)
    };
    let v6: u64 = {
        let a4453 = v5;
        let a4454 = match v3.clone() {
            Adt3::C0(_, h4452, _, _, _, _, _, _) => {
                h4452
            }
        };
        let a4455 = 0u64;
        nat_quot(a4453, a4454, a4455)
    };
    let v7: u64 = {
        let a4457 = v0;
        let a4458 = match v3.clone() {
            Adt3::C0(h4456, _, _, _, _, _, _, _) => {
                h4456
            }
        };
        let a4459 = 0u64;
        nat_rem(a4457, a4458, a4459)
    };
    let v8: u64 = {
        let a4461 = v5;
        let a4462 = match v3.clone() {
            Adt3::C0(_, h4460, _, _, _, _, _, _) => {
                h4460
            }
        };
        let a4463 = 0u64;
        nat_rem(a4461, a4462, a4463)
    };
    let v9: u64 = {
        let a4465 = v6;
        let a4466 = match v3.clone() {
            Adt3::C0(_, _, h4464, _, _, _, _, _) => {
                h4464
            }
        };
        let a4467 = 0u64;
        nat_rem(a4465, a4466, a4467)
    };
    let v10: u64 = {
        let a4469 = v6;
        let a4470 = match v3.clone() {
            Adt3::C0(_, _, h4468, _, _, _, _, _) => {
                h4468
            }
        };
        let a4471 = 0u64;
        nat_quot(a4469, a4470, a4471)
    };
    let v11: u64 = {
        let a4472 = v4;
        let a4473 = 0u64;
        if nat_eq(a4472, a4473) {
            match v1.clone() {
                Adt3::C0(h4474, _, _, _, _, _, _, _) => {
                    h4474
                }
            }
        } else {
            let a4475 = v4;
            let a4476 = 1u64;
            if nat_eq(a4475, a4476) {
                match v1.clone() {
                    Adt3::C0(_, h4477, _, _, _, _, _, _) => {
                        h4477
                    }
                }
            } else {
                let a4478 = v4;
                let a4479 = 2u64;
                if nat_eq(a4478, a4479) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4480, _, _, _, _, _) => {
                            h4480
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4481, _, _, _, _) => {
                            h4481
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4488 = v11;
        let a4489 = {
            let a4482 = v4;
            let a4483 = 0u64;
            if nat_eq(a4482, a4483) {
                v7
            } else {
                let a4484 = v4;
                let a4485 = 1u64;
                if nat_eq(a4484, a4485) {
                    v8
                } else {
                    let a4486 = v4;
                    let a4487 = 2u64;
                    if nat_eq(a4486, a4487) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4488, a4489)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f138(if if v12 {
        let a4490 = v4;
        let a4491 = 0u64;
        nat_eq(a4490, a4491)
    } else {
        false
    } {
        let a4492 = v7;
        let a4493 = v11;
        nat_sub(a4492, a4493)
    } else {
        v7
    }, if if v12 {
        let a4494 = v4;
        let a4495 = 1u64;
        nat_eq(a4494, a4495)
    } else {
        false
    } {
        let a4496 = v8;
        let a4497 = v11;
        nat_sub(a4496, a4497)
    } else {
        v8
    }, if if v12 {
        let a4498 = v4;
        let a4499 = 2u64;
        nat_eq(a4498, a4499)
    } else {
        false
    } {
        let a4500 = v9;
        let a4501 = v11;
        nat_sub(a4500, a4501)
    } else {
        v9
    }, if if v12 {
        let a4502 = v4;
        let a4503 = 3u64;
        nat_eq(a4502, a4503)
    } else {
        false
    } {
        let a4504 = v10;
        let a4505 = v11;
        nat_sub(a4504, a4505)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4506, _, _, _) => {
            h4506
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4507, _, _) => {
            h4507
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4508, _) => {
            h4508
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4509) => {
            h4509
        }
    })?, f90(v0, v3)?))
}

pub fn f149(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f57(v1.clone(), f234(v8, v3, match v1.clone() {
            Adt3::C0(h4510, _, _, _, _, _, _, _) => {
                h4510
            }
        })?, f234(v9, v4, match v1.clone() {
            Adt3::C0(_, h4511, _, _, _, _, _, _) => {
                h4511
            }
        })?, f234(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4512, _, _, _, _, _) => {
                h4512
            }
        })?, f234(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4513, _, _, _, _) => {
                h4513
            }
        })?)?, f90(v0, v2)?))
    } else {
        if if if if f235(v8, v3, match v1.clone() {
            Adt3::C0(h4514, _, _, _, _, _, _, _) => {
                h4514
            }
        })? {
            f235(v9, v4, match v1.clone() {
                Adt3::C0(_, h4515, _, _, _, _, _, _) => {
                    h4515
                }
            })?
        } else {
            false
        } {
            f235(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4516, _, _, _, _, _) => {
                    h4516
                }
            })?
        } else {
            false
        } {
            f235(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4517, _, _, _, _) => {
                    h4517
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f57(v1, f236(v8, v3)?, f236(v9, v4)?, f236(v10, v5)?, f236(v11, v6)?)?, f90(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f90(v0, v2)?))
        }
    }
}

pub fn f150(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f57(v1.clone(), f237(v4, v3, match v1 {
        Adt3::C0(h4518, _, _, _, _, _, _, _) => {
            h4518
        }
    })?, v5, v6, v7)?, f90(v0, v2)?))
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4519 = v0;
    let a4520 = v3;
    if nat_lt(a4519, a4520) {
        Ok(Adt9::C1(false, {
            let a4521 = v0;
            let a4522 = 4u64;
            nat_mul(a4521, a4522)?
        }, {
            let a4523 = v0;
            let a4524 = 4u64;
            nat_mul(a4523, a4524)?
        }))
    } else {
        let v6: u64 = {
            let a4525 = v0;
            let a4526 = v3;
            nat_sub(a4525, a4526)
        };
        let v7: u64 = f90(v6, v1)?;
        let v8: u64 = f238(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4527 = v0;
    let a4528 = v3;
    if nat_lt(a4527, a4528) {
        Ok(Adt9::C1(true, 0u64, f90(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4529 = v0;
            let a4530 = v3;
            nat_sub(a4529, a4530)
        };
        let v5: u64 = f147(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f90(v4, v1)?, 3u64, v5))
    }
}

pub fn f153(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4532 = v1;
        let a4533 = match v3.clone() {
            Adt3::C0(h4531, _, _, _, _, _, _, _) => {
                h4531
            }
        };
        let a4534 = 0u64;
        nat_rem(a4532, a4533, a4534)
    };
    let v5: u64 = {
        let a4540 = {
            let a4536 = v1;
            let a4537 = match v3.clone() {
                Adt3::C0(h4535, _, _, _, _, _, _, _) => {
                    h4535
                }
            };
            let a4538 = 0u64;
            nat_quot(a4536, a4537, a4538)
        };
        let a4541 = match v3.clone() {
            Adt3::C0(_, h4539, _, _, _, _, _, _) => {
                h4539
            }
        };
        let a4542 = 0u64;
        nat_rem(a4540, a4541, a4542)
    };
    let v6: bool = f239(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f147(v1, v2, v3.clone())?
    } else {
        0u64
    }, f90(v1, v3)?))
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4543 = v3;
        let a4544 = 8u64;
        nat_add(a4543, a4544)?
    }, 0u64, f90(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f147(v0, v5, v2.clone())?
    }, 3u64, f90(v0, v2)?))
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4545 = v4;
    let a4546 = 0u64;
    if if nat_eq(a4545, a4546) {
        f119(v3)
    } else {
        false
    } {
        let m4547 = f210(v3);
        match m4547 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4548, _) => {
                        h4548
                    }
                };
                Ok(Adt9::C5(v3, f90(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4549) => {
                        h4549
                    }
                }, f90({
                    let a4550 = v0;
                    let a4551 = v6;
                    nat_mul(a4550, a4551)?
                }, v2.clone())?, {
                    let a4557 = {
                        let a4555 = {
                            let a4552 = v6;
                            let a4553 = 1u64;
                            nat_sub(a4552, a4553)
                        };
                        let a4556 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4554, _, _, _) => {
                                h4554
                            }
                        };
                        nat_mul(a4555, a4556)?
                    };
                    let a4558 = 4u64;
                    nat_add(a4557, a4558)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4559, _, _, _) => {
                        h4559
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f90(v0, v1)?, f90(v0, v2)?))
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4561 = v0;
        let a4562 = match v2.clone() {
            Adt3::C0(h4560, _, _, _, _, _, _, _) => {
                h4560
            }
        };
        let a4563 = 0u64;
        nat_rem(a4561, a4562, a4563)
    };
    let v7: u64 = {
        let a4565 = v0;
        let a4566 = match v2.clone() {
            Adt3::C0(h4564, _, _, _, _, _, _, _) => {
                h4564
            }
        };
        let a4567 = 0u64;
        nat_quot(a4565, a4566, a4567)
    };
    let v8: u64 = {
        let a4569 = v7;
        let a4570 = match v2.clone() {
            Adt3::C0(_, h4568, _, _, _, _, _, _) => {
                h4568
            }
        };
        let a4571 = 0u64;
        nat_rem(a4569, a4570, a4571)
    };
    let v9: u64 = {
        let a4573 = v7;
        let a4574 = match v2.clone() {
            Adt3::C0(_, h4572, _, _, _, _, _, _) => {
                h4572
            }
        };
        let a4575 = 0u64;
        nat_quot(a4573, a4574, a4575)
    };
    let v10: u64 = {
        let a4577 = v9;
        let a4578 = match v2.clone() {
            Adt3::C0(_, _, h4576, _, _, _, _, _) => {
                h4576
            }
        };
        let a4579 = 0u64;
        nat_rem(a4577, a4578, a4579)
    };
    let v11: u64 = {
        let a4581 = v9;
        let a4582 = match v2.clone() {
            Adt3::C0(_, _, h4580, _, _, _, _, _) => {
                h4580
            }
        };
        let a4583 = 0u64;
        nat_quot(a4581, a4582, a4583)
    };
    let v12: u64 = f90(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4592 = {
                let a4587 = {
                    let a4584 = v10;
                    let a4585 = v4;
                    let a4586 = 0u64;
                    nat_quot(a4584, a4585, a4586)
                };
                let a4588 = v5;
                nat_mul(a4587, a4588)?
            };
            let a4593 = {
                let a4589 = v8;
                let a4590 = v4;
                let a4591 = 0u64;
                nat_quot(a4589, a4590, a4591)
            };
            nat_add(a4592, a4593)?
        };
        let v14: u64 = {
            let a4613 = v6;
            let a4614 = {
                let a4611 = match v1.clone() {
                    Adt3::C0(h4594, _, _, _, _, _, _, _) => {
                        h4594
                    }
                };
                let a4612 = {
                    let a4609 = {
                        let a4595 = v8;
                        let a4596 = v4;
                        let a4597 = 0u64;
                        nat_rem(a4595, a4596, a4597)
                    };
                    let a4610 = {
                        let a4607 = match v1.clone() {
                            Adt3::C0(_, h4598, _, _, _, _, _, _) => {
                                h4598
                            }
                        };
                        let a4608 = {
                            let a4605 = {
                                let a4599 = v10;
                                let a4600 = v4;
                                let a4601 = 0u64;
                                nat_rem(a4599, a4600, a4601)
                            };
                            let a4606 = {
                                let a4603 = match v1.clone() {
                                    Adt3::C0(_, _, h4602, _, _, _, _, _) => {
                                        h4602
                                    }
                                };
                                let a4604 = v13;
                                nat_mul(a4603, a4604)?
                            };
                            nat_add(a4605, a4606)?
                        };
                        nat_mul(a4607, a4608)?
                    };
                    nat_add(a4609, a4610)?
                };
                nat_mul(a4611, a4612)?
            };
            nat_add(a4613, a4614)?
        };
        Ok(Adt9::C1(false, f90(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4620 = {
                let a4618 = {
                    let a4615 = v11;
                    let a4616 = v5;
                    let a4617 = 0u64;
                    nat_rem(a4615, a4616, a4617)
                };
                let a4619 = v4;
                nat_mul(a4618, a4619)?
            };
            let a4621 = v8;
            nat_add(a4620, a4621)?
        };
        let v16: u64 = {
            let a4627 = {
                let a4625 = {
                    let a4622 = v11;
                    let a4623 = v5;
                    let a4624 = 0u64;
                    nat_quot(a4622, a4623, a4624)
                };
                let a4626 = v4;
                nat_mul(a4625, a4626)?
            };
            let a4628 = v10;
            nat_add(a4627, a4628)?
        };
        let a4630 = v15;
        let a4631 = match v1.clone() {
            Adt3::C0(_, h4629, _, _, _, _, _, _) => {
                h4629
            }
        };
        if if nat_lt(a4630, a4631) {
            let a4633 = v16;
            let a4634 = match v1.clone() {
                Adt3::C0(_, _, h4632, _, _, _, _, _) => {
                    h4632
                }
            };
            nat_lt(a4633, a4634)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f90({
                let a4643 = v6;
                let a4644 = {
                    let a4641 = match v1.clone() {
                        Adt3::C0(h4635, _, _, _, _, _, _, _) => {
                            h4635
                        }
                    };
                    let a4642 = {
                        let a4639 = v15;
                        let a4640 = {
                            let a4637 = match v1.clone() {
                                Adt3::C0(_, h4636, _, _, _, _, _, _) => {
                                    h4636
                                }
                            };
                            let a4638 = v16;
                            nat_mul(a4637, a4638)?
                        };
                        nat_add(a4639, a4640)?
                    };
                    nat_mul(a4641, a4642)?
                };
                nat_add(a4643, a4644)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f231(23u64, v4);
    let v6: u64 = {
        let a4645 = v0;
        let a4646 = v5;
        let a4647 = 0u64;
        nat_rem(a4645, a4646, a4647)
    };
    let v7: u64 = {
        let a4648 = v0;
        let a4649 = v5;
        let a4650 = 0u64;
        nat_quot(a4648, a4649, a4650)
    };
    let v8: Adt13 = Adt13::C1(f90(v7, v2)?);
    let a4651 = v6;
    let a4652 = 0u64;
    if nat_eq(a4651, a4652) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f90(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f169(v3), 4u64))
    } else {
        let a4653 = v6;
        let a4654 = 1u64;
        if nat_eq(a4653, a4654) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4655 = v4;
                let a4656 = 0u64;
                if nat_eq(a4655, a4656) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f169(v4)))
        }
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f90(v0, v2.clone())?;
    let v4: u64 = f90(v0, v1.clone())?;
    let a4661 = {
        let a4658 = v0;
        let a4659 = match v1 {
            Adt3::C0(h4657, _, _, _, _, _, _, _) => {
                h4657
            }
        };
        let a4660 = 0u64;
        nat_rem(a4658, a4659, a4660)
    };
    let a4662 = 0u64;
    if nat_eq(a4661, a4662) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f90({
            let a4663 = v0;
            let a4664 = 1u64;
            nat_sub(a4663, a4664)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4665 = v3;
    let a4666 = 13u64;
    if nat_eq(a4665, a4666) {
        f240(v0, v1, v4)
    } else {
        let v5: u64 = f241(match v1.clone() {
            Adt3::C0(h4667, _, _, _, _, _, _, _) => {
                h4667
            }
        }, {
            let a4668 = v3;
            let a4669 = 16u64;
            nat_eq(a4668, a4669)
        })?;
        let v6: u64 = {
            let a4670 = v0;
            let a4671 = v5;
            let a4672 = 0u64;
            nat_quot(a4670, a4671, a4672)
        };
        let v7: u64 = {
            let a4673 = v0;
            let a4674 = v5;
            let a4675 = 0u64;
            nat_rem(a4673, a4674, a4675)
        };
        let v8: u64 = {
            let a4679 = 1u64;
            let a4680 = {
                let a4677 = match v1.clone() {
                    Adt3::C0(h4676, _, _, _, _, _, _, _) => {
                        h4676
                    }
                };
                let a4678 = 2u64;
                nat_mul(a4677, a4678)?
            };
            nat_add(a4679, a4680)?
        };
        let a4681 = v7;
        let a4682 = v8;
        if nat_lt(a4681, a4682) {
            f242(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f90(v6, v2)?;
            let a4683 = v3;
            let a4684 = 14u64;
            if nat_eq(a4683, a4684) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4685 = v7;
                let a4686 = v8;
                if nat_eq(a4685, a4686) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4689 = v7;
                    let a4690 = {
                        let a4687 = v8;
                        let a4688 = 1u64;
                        nat_add(a4687, a4688)?
                    };
                    if nat_eq(a4689, a4690) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4691, _, _, _, _, _, _, _) => {
                                h4691
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

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4692 = v0;
    let a4693 = 0u64;
    if nat_eq(a4692, a4693) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4700 = {
            let a4694 = v0;
            let a4695 = 1u64;
            nat_add(a4694, a4695)?
        };
        let a4701 = {
            let a4698 = 2u64;
            let a4699 = {
                let a4696 = v3;
                let a4697 = 2u64;
                nat_mul(a4696, a4697)?
            };
            nat_add(a4698, a4699)?
        };
        if nat_eq(a4700, a4701) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4702 = v0;
                let a4703 = 1u64;
                nat_sub(a4702, a4703)
            };
            let v5: u64 = {
                let a4704 = v4;
                let a4705 = 2u64;
                let a4706 = 0u64;
                nat_quot(a4704, a4705, a4706)
            };
            let a4710 = {
                let a4707 = v4;
                let a4708 = 2u64;
                let a4709 = 0u64;
                nat_rem(a4707, a4708, a4709)
            };
            let a4711 = 0u64;
            if nat_eq(a4710, a4711) {
                Ok(Adt9::C4(17u64, 0u64, f90(v5, v1)?, 4u64, f90(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f161(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4712 = v0;
    let a4713 = f41(v1, v2, v3)?;
    if nat_lt(a4712, a4713) {
        let a4714 = v0;
        let a4715 = v1;
        if nat_lt(a4714, a4715) {
            Ok(Adt9::C1(false, {
                let a4716 = 4u64;
                let a4717 = v0;
                nat_mul(a4716, a4717)?
            }, {
                let a4718 = 4u64;
                let a4719 = v0;
                nat_mul(a4718, a4719)?
            }))
        } else {
            let v4: u64 = {
                let a4720 = v0;
                let a4721 = v1;
                nat_sub(a4720, a4721)
            };
            let v5: u64 = f243(v4, v2)?;
            let a4722 = v5;
            let a4723 = 2u64;
            if nat_lt(a4722, a4723) {
                Ok(Adt9::C4(6u64, {
                    let a4724 = v5;
                    let a4725 = 0u64;
                    if nat_eq(a4724, a4725) {
                        4u64
                    } else {
                        5u64
                    }
                }, f244(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f245(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f246(v4, v2)? {
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

pub fn f162(v0: u64, v1: u64) -> u64 {
    let a4726 = v0;
    let a4727 = 0u64;
    if if nat_eq(a4726, a4727) {
        true
    } else {
        let a4728 = v1;
        let a4729 = 0u64;
        nat_eq(a4728, a4729)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f163(v0: u64, v1: u64) -> Adt11 {
    let a4730 = v0;
    let a4731 = 43u64;
    if nat_eq(a4730, a4731) {
        Adt11::C8(3u64)
    } else {
        let a4732 = v1;
        let a4733 = 43u64;
        if nat_eq(a4732, a4733) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f164(v0: u64) -> R<u64> {
    let a4736 = 2u64;
    let a4737 = {
        let a4734 = 6u64;
        let a4735 = v0;
        nat_mul(a4734, a4735)?
    };
    nat_add(a4736, a4737)
}

pub fn f165(v0: u64, v1: bool) -> R<u64> {
    let a4740 = {
        let a4738 = v0;
        let a4739 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4738, a4739)?
    };
    let a4741 = 1u64;
    nat_add(a4740, a4741)
}

pub fn f166(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4742 = v0;
    let a4743 = 16u64;
    if nat_lt(a4742, a4743) {
        let v9: u64 = {
            let a4744 = v0;
            let a4745 = 4u64;
            let a4746 = 0u64;
            nat_quot(a4744, a4745, a4746)
        };
        let v10: u64 = {
            let a4751 = {
                let a4749 = 8u64;
                let a4750 = {
                    let a4747 = 4u64;
                    let a4748 = v1;
                    nat_mul(a4747, a4748)?
                };
                nat_add(a4749, a4750)?
            };
            let a4752 = v9;
            nat_add(a4751, a4752)?
        };
        let v11: u64 = f57(v6, v9, v1, v2, 0u64)?;
        let a4756 = {
            let a4753 = v0;
            let a4754 = 4u64;
            let a4755 = 0u64;
            nat_rem(a4753, a4754, a4755)
        };
        let a4757 = 0u64;
        if nat_eq(a4756, a4757) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f57(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f57(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4761 = {
                let a4758 = v0;
                let a4759 = 4u64;
                let a4760 = 0u64;
                nat_rem(a4758, a4759, a4760)
            };
            let a4762 = 1u64;
            if nat_eq(a4761, a4762) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f57(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4766 = {
                    let a4763 = v0;
                    let a4764 = 4u64;
                    let a4765 = 0u64;
                    nat_rem(a4763, a4764, a4765)
                };
                let a4767 = 2u64;
                if nat_eq(a4766, a4767) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4768 = v9;
                        let a4769 = 0u64;
                        if nat_eq(a4768, a4769) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4770 = v8;
                        let a4771 = 2u64;
                        if nat_eq(a4770, a4771) {
                            let a4772 = v9;
                            let a4773 = 0u64;
                            if nat_eq(a4772, a4773) {
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
        let a4774 = v0;
        let a4775 = 28u64;
        if nat_lt(a4774, a4775) {
            let v12: u64 = {
                let a4776 = v0;
                let a4777 = 16u64;
                nat_sub(a4776, a4777)
            };
            let v13: u64 = {
                let a4778 = v12;
                let a4779 = 3u64;
                let a4780 = 0u64;
                nat_quot(a4778, a4779, a4780)
            };
            let v14: u64 = f57(v6, v13, v1, v2, 0u64)?;
            let a4784 = {
                let a4781 = v12;
                let a4782 = 3u64;
                let a4783 = 0u64;
                nat_rem(a4781, a4782, a4783)
            };
            let a4785 = 0u64;
            if nat_eq(a4784, a4785) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4789 = {
                    let a4786 = v12;
                    let a4787 = 3u64;
                    let a4788 = 0u64;
                    nat_rem(a4786, a4787, a4788)
                };
                let a4790 = 1u64;
                if nat_eq(a4789, a4790) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4791 = v13;
                        let a4792 = 0u64;
                        if nat_eq(a4791, a4792) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4793 = v0;
            let a4794 = 28u64;
            if nat_eq(a4793, a4794) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4795 = v0;
                    let a4796 = 29u64;
                    nat_sub(a4795, a4796)
                };
                let v16: u64 = {
                    let a4797 = v15;
                    let a4798 = 2u64;
                    let a4799 = 0u64;
                    nat_quot(a4797, a4798, a4799)
                };
                let v17: u64 = f57(v6, v16, v1, v2, 0u64)?;
                let a4803 = {
                    let a4800 = v15;
                    let a4801 = 2u64;
                    let a4802 = 0u64;
                    nat_rem(a4800, a4801, a4802)
                };
                let a4804 = 0u64;
                if nat_eq(a4803, a4804) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f167(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4805 = v0;
        let a4806 = 9u64;
        let a4807 = 0u64;
        nat_rem(a4805, a4806, a4807)
    };
    let v5: u64 = {
        let a4811 = {
            let a4808 = v0;
            let a4809 = 9u64;
            let a4810 = 0u64;
            nat_quot(a4808, a4809, a4810)
        };
        let a4812 = 4u64;
        let a4813 = 0u64;
        nat_rem(a4811, a4812, a4813)
    };
    let v6: bool = {
        let a4820 = {
            let a4817 = {
                let a4814 = v0;
                let a4815 = 36u64;
                let a4816 = 0u64;
                nat_quot(a4814, a4815, a4816)
            };
            let a4818 = 2u64;
            let a4819 = 0u64;
            nat_rem(a4817, a4818, a4819)
        };
        let a4821 = 0u64;
        nat_eq(a4820, a4821)
    };
    let v7: u64 = {
        let a4822 = v4;
        let a4823 = 4u64;
        if nat_lt(a4822, a4823) {
            v4
        } else {
            let a4824 = v4;
            let a4825 = 5u64;
            nat_sub(a4824, a4825)
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
    let a4826 = v4;
    let a4827 = 4u64;
    if nat_lt(a4826, a4827) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4828 = v4;
            let a4829 = 0u64;
            if nat_eq(a4828, a4829) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4830 = v4;
        let a4831 = 4u64;
        if nat_eq(a4830, a4831) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f168(v0: u64) -> R<u64> {
    let a4834 = v0;
    let a4835 = {
        let a4832 = v0;
        let a4833 = 2u64;
        nat_add(a4832, a4833)?
    };
    nat_mul(a4834, a4835)
}

pub fn f169(v0: u64) -> u64 {
    let m4836 = f210(v0);
    match m4836 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4838 = match v1.clone() {
                Adt19::C0(_, h4837, _) => {
                    h4837
                }
            };
            let a4839 = 1u64;
            if if nat_eq(a4838, a4839) {
                let a4841 = match v1.clone() {
                    Adt19::C0(_, _, h4840) => {
                        h4840
                    }
                };
                let a4842 = 1u64;
                if if if nat_eq(a4841, a4842) {
                    true
                } else {
                    let a4844 = match v1.clone() {
                        Adt19::C0(_, _, h4843) => {
                            h4843
                        }
                    };
                    let a4845 = 2u64;
                    nat_eq(a4844, a4845)
                } {
                    true
                } else {
                    let a4847 = match v1.clone() {
                        Adt19::C0(_, _, h4846) => {
                            h4846
                        }
                    };
                    let a4848 = 4u64;
                    nat_eq(a4847, a4848)
                } {
                    true
                } else {
                    let a4850 = match v1.clone() {
                        Adt19::C0(_, _, h4849) => {
                            h4849
                        }
                    };
                    let a4851 = 8u64;
                    nat_eq(a4850, a4851)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4852) => {
                        h4852
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f170(v0: u64) -> R<u64> {
    let a4855 = 3u64;
    let a4856 = {
        let a4853 = v0;
        let a4854 = 3u64;
        nat_mul(a4853, a4854)?
    };
    nat_add(a4855, a4856)
}

pub fn f171(v0: u64, v1: u64) -> R<u64> {
    if f172(v0) {
        let a4857 = 2u64;
        let a4858 = v1;
        nat_mul(a4857, a4858)
    } else {
        let a4859 = v1;
        let a4860 = f247(v0);
        let a4861 = 0u64;
        Ok(nat_quot(a4859, a4860, a4861))
    }
}

pub fn f172(v0: u64) -> bool {
    let a4862 = v0;
    let a4863 = 0u64;
    if if nat_eq(a4862, a4863) {
        true
    } else {
        let a4864 = v0;
        let a4865 = 1u64;
        nat_eq(a4864, a4865)
    } {
        true
    } else {
        let a4866 = v0;
        let a4867 = 30u64;
        nat_eq(a4866, a4867)
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4868 = f210(v0);
    match m4868 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f138(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4869, _, _, _) => {
                    h4869
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4870, _, _) => {
                    h4870
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4871, _) => {
                    h4871
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4872) => {
                    h4872
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4873) => {
                    h4873
                }
            }, f138({
                let a4875 = v2;
                let a4876 = match v6.clone() {
                    Adt19::C0(_, h4874, _) => {
                        h4874
                    }
                };
                nat_mul(a4875, a4876)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4877, _, _, _) => {
                    h4877
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4878, _, _) => {
                    h4878
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4879, _) => {
                    h4879
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4880) => {
                    h4880
                }
            })?, match v6 {
                Adt19::C0(_, h4881, _) => {
                    h4881
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4882, _, _, _) => {
                    h4882
                }
            }))
        }
    }
}

pub fn f174(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4890 = {
        let a4883 = 2u64;
        let a4884 = f175(v1.clone())?;
        nat_mul(a4883, a4884)?
    };
    let a4891 = {
        let a4888 = f175(v1)?;
        let a4889 = {
            let a4886 = 1u64;
            let a4887 = match v0 {
                Adt3::C0(h4885, _, _, _, _, _, _, _) => {
                    h4885
                }
            };
            nat_add(a4886, a4887)?
        };
        nat_mul(a4888, a4889)?
    };
    nat_add(a4890, a4891)
}

pub fn f175(v0: Adt3) -> R<u64> {
    if f48(match v0.clone() {
        Adt3::C0(h4892, _, _, _, _, _, _, _) => {
            h4892
        }
    }, match v0.clone() {
        Adt3::C0(_, h4893, _, _, _, _, _, _) => {
            h4893
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4894, _, _, _, _, _) => {
            h4894
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4895, _, _, _, _) => {
            h4895
        }
    }) {
        Ok(0u64)
    } else {
        let a4904 = match v0.clone() {
            Adt3::C0(h4896, _, _, _, _, _, _, _) => {
                h4896
            }
        };
        let a4905 = {
            let a4902 = match v0.clone() {
                Adt3::C0(_, h4897, _, _, _, _, _, _) => {
                    h4897
                }
            };
            let a4903 = {
                let a4900 = match v0.clone() {
                    Adt3::C0(_, _, h4898, _, _, _, _, _) => {
                        h4898
                    }
                };
                let a4901 = match v0 {
                    Adt3::C0(_, _, _, h4899, _, _, _, _) => {
                        h4899
                    }
                };
                nat_mul(a4900, a4901)?
            };
            nat_mul(a4902, a4903)?
        };
        nat_mul(a4904, a4905)
    }
}

pub fn f176(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f90(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f177(v0: u64, v1: u64) -> bool {
    let a4906 = v1;
    let a4907 = 2147483647u64;
    if nat_le(a4906, a4907) {
        let a4908 = v1;
        let a4909 = v0;
        nat_lt(a4908, a4909)
    } else {
        false
    }
}

pub fn f178(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4910 = f210(v0);
    match m4910 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a4912 = v2;
                let a4913 = match v7.clone() {
                    Adt19::C0(_, h4911, _) => {
                        h4911
                    }
                };
                nat_mul(a4912, a4913)?
            };
            let m4926 = f248(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4914, _, _, _, _, _, _, _) => {
                    h4914
                }
            }, match v5.clone() {
                Adt3::C0(_, h4915, _, _, _, _, _, _) => {
                    h4915
                }
            }, match v5 {
                Adt3::C0(_, _, h4916, _, _, _, _, _) => {
                    h4916
                }
            }, {
                let a4919 = match v3.clone() {
                    Adt3::C0(h4917, _, _, _, _, _, _, _) => {
                        h4917
                    }
                };
                let a4920 = match v7.clone() {
                    Adt19::C0(_, h4918, _) => {
                        h4918
                    }
                };
                nat_mul(a4919, a4920)?
            }, match v3.clone() {
                Adt3::C0(_, h4921, _, _, _, _, _, _) => {
                    h4921
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4922, _, _, _) => {
                    h4922
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4923, _, _) => {
                    h4923
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4924, _) => {
                    h4924
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4925) => {
                    h4925
                }
            })?;
            match m4926 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4927 = v0;
                    let a4928 = 0u64;
                    if if nat_eq(a4927, a4928) {
                        true
                    } else {
                        let a4929 = v0;
                        let a4930 = 26u64;
                        nat_eq(a4929, a4930)
                    } {
                        Ok(Adt9::C1(false, v9, f90({
                            let a4934 = {
                                let a4932 = v1;
                                let a4933 = match v4.clone() {
                                    Adt3::C0(h4931, _, _, _, _, _, _, _) => {
                                        h4931
                                    }
                                };
                                nat_mul(a4932, a4933)?
                            };
                            let a4935 = v8;
                            nat_add(a4934, a4935)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h4936) => {
                                h4936
                            }
                        }, f90({
                            let a4940 = {
                                let a4938 = v1;
                                let a4939 = match v4.clone() {
                                    Adt3::C0(h4937, _, _, _, _, _, _, _) => {
                                        h4937
                                    }
                                };
                                nat_mul(a4938, a4939)?
                            };
                            let a4941 = v8;
                            nat_add(a4940, a4941)?
                        }, v4.clone())?, {
                            let a4948 = {
                                let a4946 = {
                                    let a4943 = match v7 {
                                        Adt19::C0(_, h4942, _) => {
                                            h4942
                                        }
                                    };
                                    let a4944 = 1u64;
                                    nat_sub(a4943, a4944)
                                };
                                let a4947 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4945, _, _, _) => {
                                        h4945
                                    }
                                };
                                nat_mul(a4946, a4947)?
                            };
                            let a4949 = 4u64;
                            nat_add(a4948, a4949)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4950, _, _, _) => {
                                h4950
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f179(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4951 = v1;
    let a4952 = 0u64;
    if if nat_eq(a4951, a4952) {
        true
    } else {
        let a4953 = v2;
        let a4954 = 0u64;
        nat_eq(a4953, a4954)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f180(v0: u64, v1: u64) -> bool {
    let a4955 = v0;
    let a4956 = 4294967295u64;
    if nat_eq(a4955, a4956) {
        true
    } else {
        let a4957 = v0;
        let a4958 = 2147483648u64;
        if nat_lt(a4957, a4958) {
            let a4959 = v0;
            let a4960 = v1;
            nat_lt(a4959, a4960)
        } else {
            false
        }
    }
}

pub fn f181(v0: Adt7) -> R<u64> {
    let a4972 = if match v0.clone() {
        Adt7::C0(_, _, _, h4961, _) => {
            h4961
        }
    } {
        2u64
    } else {
        3u64
    };
    let a4973 = {
        let a4970 = {
            let a4968 = {
                let m4964 = match v0.clone() {
                    Adt7::C0(h4963, _, _, _, _) => {
                        h4963
                    }
                };
                match m4964 {
                    Adt6::C0(h4962, _, _, _) => {
                        h4962
                    }
                }
            };
            let a4969 = {
                let m4967 = match v0.clone() {
                    Adt7::C0(_, h4966, _, _, _) => {
                        h4966
                    }
                };
                match m4967 {
                    Adt6::C0(h4965, _, _, _) => {
                        h4965
                    }
                }
            };
            nat_mul(a4968, a4969)?
        };
        let a4971 = f182(v0)?;
        nat_mul(a4970, a4971)?
    };
    nat_add(a4972, a4973)
}

pub fn f182(v0: Adt7) -> R<u64> {
    let a4978 = if match v0.clone() {
        Adt7::C0(_, _, _, h4974, _) => {
            h4974
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4979 = {
        let a4976 = match v0 {
            Adt7::C0(_, _, _, _, h4975) => {
                h4975
            }
        };
        let a4977 = 1u64;
        if nat_eq(a4976, a4977) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4978, a4979)
}

pub fn f183(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4984 = {
            let a4981 = v2;
            let a4982 = match v0.clone() {
                Adt6::C0(_, h4980, _, _) => {
                    h4980
                }
            };
            nat_mul(a4981, a4982)?
        };
        let a4985 = f139(match v0.clone() {
            Adt6::C0(_, _, h4983, _) => {
                h4983
            }
        });
        nat_add(a4984, a4985)?
    };
    let v4: u64 = f140(match v0.clone() {
        Adt6::C0(_, _, h4986, _) => {
            h4986
        }
    });
    let v5: u64 = {
        let a4990 = {
            let a4988 = v3;
            let a4989 = match v0 {
                Adt6::C0(h4987, _, _, _) => {
                    h4987
                }
            };
            nat_add(a4988, a4989)?
        };
        let a4991 = v4;
        nat_sub(a4990, a4991)
    };
    let a4996 = {
        let a4992 = v5;
        let a4993 = v1;
        if nat_lt(a4992, a4993) {
            v5
        } else {
            v1
        }
    };
    let a4997 = {
        let a4994 = v3;
        let a4995 = v4;
        nat_sub(a4994, a4995)
    };
    Ok(nat_sub(a4996, a4997))
}

pub fn f184(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5006 = f60(match v0.clone() {
        Adt7::C0(h4998, _, _, _, _) => {
            h4998
        }
    }, match v1.clone() {
        Adt3::C0(h4999, _, _, _, _, _, _, _) => {
            h4999
        }
    }, f58(v3, v2.clone(), 0u64), {
        let a5003 = v4;
        let a5004 = {
            let m5002 = match v0.clone() {
                Adt7::C0(h5001, _, _, _, _) => {
                    h5001
                }
            };
            match m5002 {
                Adt6::C0(h5000, _, _, _) => {
                    h5000
                }
            }
        };
        let a5005 = 0u64;
        nat_rem(a5003, a5004, a5005)
    })?;
    match m5006 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5015 = f60(match v0.clone() {
                Adt7::C0(_, h5007, _, _, _) => {
                    h5007
                }
            }, match v1.clone() {
                Adt3::C0(_, h5008, _, _, _, _, _, _) => {
                    h5008
                }
            }, f58(v3, v2.clone(), 1u64), {
                let a5012 = v4;
                let a5013 = {
                    let m5011 = match v0 {
                        Adt7::C0(h5010, _, _, _, _) => {
                            h5010
                        }
                    };
                    match m5011 {
                        Adt6::C0(h5009, _, _, _) => {
                            h5009
                        }
                    }
                };
                let a5014 = 0u64;
                nat_quot(a5012, a5013, a5014)
            })?;
            match m5015 {
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

pub fn f185(v0: u64) -> R<u64> {
    let a5018 = 2u64;
    let a5019 = {
        let a5016 = v0;
        let a5017 = 2u64;
        nat_mul(a5016, a5017)?
    };
    nat_add(a5018, a5019)
}

pub fn f186(v0: Adt6) -> R<u64> {
    let a5032 = 2u64;
    let a5033 = {
        let a5030 = 2u64;
        let a5031 = {
            let a5027 = {
                let a5024 = match v0.clone() {
                    Adt6::C0(h5020, _, _, _) => {
                        h5020
                    }
                };
                let a5025 = {
                    let a5022 = match v0.clone() {
                        Adt6::C0(_, h5021, _, _) => {
                            h5021
                        }
                    };
                    let a5023 = 1u64;
                    nat_sub(a5022, a5023)
                };
                nat_add(a5024, a5025)?
            };
            let a5028 = match v0 {
                Adt6::C0(_, h5026, _, _) => {
                    h5026
                }
            };
            let a5029 = 0u64;
            nat_quot(a5027, a5028, a5029)
        };
        nat_mul(a5030, a5031)?
    };
    nat_add(a5032, a5033)
}

pub fn f187(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5034 = v0;
    match m5034 {
        Adt16::C0(v19) => {
            f249(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5035 = v20;
            let a5036 = 66u64;
            if nat_eq(a5035, a5036) {
                f250(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f251(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f188(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5043 = v3;
        let a5044 = {
            let a5040 = f90({
                let a5037 = v0;
                let a5038 = 2u64;
                let a5039 = 0u64;
                nat_quot(a5037, a5038, a5039)
            }, v1)?;
            let a5041 = v2;
            let a5042 = 0u64;
            nat_quot(a5040, a5041, a5042)
        };
        nat_add(a5043, a5044)?
    };
    let a5048 = {
        let a5045 = v0;
        let a5046 = 2u64;
        let a5047 = 0u64;
        nat_rem(a5045, a5046, a5047)
    };
    let a5049 = 0u64;
    if nat_eq(a5048, a5049) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5050 = v4;
        let a5051 = 0u64;
        if nat_eq(a5050, a5051) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f189(v0: u64, v1: u64) -> R<u64> {
    let a5054 = 8u64;
    let a5055 = {
        let a5052 = f252(v0);
        let a5053 = v1;
        nat_mul(a5052, a5053)?
    };
    nat_add(a5054, a5055)
}

pub fn f190(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5056, _, _, _, _, _, _, _) => {
            h5056
        }
    };
    let v7: u64 = f252(v3);
    let v8: u64 = {
        let a5059 = 1u64;
        let a5060 = {
            let a5057 = v6;
            let a5058 = v7;
            nat_mul(a5057, a5058)?
        };
        nat_add(a5059, a5060)?
    };
    let a5061 = v0;
    let a5062 = 0u64;
    if nat_eq(a5061, a5062) {
        Ok(f253(0u64))
    } else {
        let a5063 = v0;
        let a5064 = v8;
        if nat_lt(a5063, a5064) {
            let v9: u64 = {
                let a5065 = v0;
                let a5066 = 1u64;
                nat_sub(a5065, a5066)
            };
            let v10: u64 = {
                let a5067 = v9;
                let a5068 = v7;
                let a5069 = 0u64;
                nat_rem(a5067, a5068, a5069)
            };
            let v11: u64 = f90({
                let a5075 = {
                    let a5070 = v1;
                    let a5071 = v6;
                    nat_mul(a5070, a5071)?
                };
                let a5076 = {
                    let a5072 = v9;
                    let a5073 = v7;
                    let a5074 = 0u64;
                    nat_quot(a5072, a5073, a5074)
                };
                nat_add(a5075, a5076)?
            }, v2)?;
            let a5077 = v3;
            let a5078 = 30u64;
            if nat_eq(a5077, a5078) {
                let a5079 = v10;
                let a5080 = 0u64;
                if nat_eq(a5079, a5080) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5081 = v10;
                    let a5082 = 1u64;
                    if nat_eq(a5081, a5082) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5083 = v10;
                        let a5084 = 2u64;
                        if nat_eq(a5083, a5084) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5085 = v10;
                let a5086 = 0u64;
                if nat_eq(a5085, a5086) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5087 = v10;
                    let a5088 = 1u64;
                    if nat_eq(a5087, a5088) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5089 = v0;
                let a5090 = v8;
                nat_sub(a5089, a5090)
            };
            let a5091 = v12;
            let a5092 = 0u64;
            if nat_eq(a5091, a5092) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5093 = v12;
                let a5094 = 1u64;
                if nat_eq(a5093, a5094) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5095 = v12;
                    let a5096 = 2u64;
                    if nat_eq(a5095, a5096) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5097 = v12;
                        let a5098 = 3u64;
                        if nat_eq(a5097, a5098) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5099 = v12;
                            let a5100 = 4u64;
                            if nat_eq(a5099, a5100) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5101 = v12;
                                let a5102 = 5u64;
                                if nat_eq(a5101, a5102) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5103 = v12;
                                    let a5104 = 6u64;
                                    if nat_eq(a5103, a5104) {
                                        if f254(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5105 = 3u64;
                                                let a5106 = v1;
                                                nat_add(a5105, a5106)?
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

pub fn f191(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5107 = v0;
        let a5108 = f101(v3);
        let a5109 = 0u64;
        nat_quot(a5107, a5108, a5109)
    };
    let v5: u64 = {
        let a5114 = 3u64;
        let a5115 = {
            let a5111 = v4;
            let a5112 = match v1.clone() {
                Adt3::C0(h5110, _, _, _, _, _, _, _) => {
                    h5110
                }
            };
            let a5113 = 0u64;
            nat_quot(a5111, a5112, a5113)
        };
        nat_add(a5114, a5115)?
    };
    let v6: u64 = f90(v4, v1)?;
    let v7: u64 = f90(v4, v2)?;
    let a5116 = v3;
    let a5117 = 30u64;
    if nat_eq(a5116, a5117) {
        let a5121 = {
            let a5118 = v0;
            let a5119 = 3u64;
            let a5120 = 0u64;
            nat_rem(a5118, a5119, a5120)
        };
        let a5122 = 0u64;
        if nat_eq(a5121, a5122) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5126 = {
                let a5123 = v0;
                let a5124 = 3u64;
                let a5125 = 0u64;
                nat_rem(a5123, a5124, a5125)
            };
            let a5127 = 1u64;
            if nat_eq(a5126, a5127) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f192(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5128 = v0;
        let a5129 = 8388608u64;
        let a5130 = 0u64;
        nat_quot(a5128, a5129, a5130)
    };
    let a5131 = v0;
    let a5132 = 2147483648u64;
    if if if nat_lt(a5131, a5132) {
        let a5133 = 127u64;
        let a5134 = v1;
        nat_le(a5133, a5134)
    } else {
        false
    } {
        let a5135 = v1;
        let a5136 = 190u64;
        nat_lt(a5135, a5136)
    } else {
        false
    } {
        let v2: u64 = {
            let a5140 = 8388608u64;
            let a5141 = {
                let a5137 = v0;
                let a5138 = 8388608u64;
                let a5139 = 0u64;
                nat_rem(a5137, a5138, a5139)
            };
            nat_add(a5140, a5141)?
        };
        let a5142 = v1;
        let a5143 = 150u64;
        if nat_lt(a5142, a5143) {
            let v3: u64 = f255({
                let a5144 = 150u64;
                let a5145 = v1;
                nat_sub(a5144, a5145)
            });
            let a5149 = {
                let a5146 = v2;
                let a5147 = v3;
                let a5148 = 0u64;
                nat_rem(a5146, a5147, a5148)
            };
            let a5150 = 0u64;
            if nat_eq(a5149, a5150) {
                let a5151 = v2;
                let a5152 = v3;
                let a5153 = 0u64;
                Ok(nat_quot(a5151, a5152, a5153))
            } else {
                Ok(0u64)
            }
        } else {
            let a5156 = f255({
                let a5154 = v1;
                let a5155 = 150u64;
                nat_sub(a5154, a5155)
            });
            let a5157 = v2;
            nat_mul(a5156, a5157)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f193(v0: Adt14, v1: Adt14) -> bool {
    let a5160 = match v0.clone() {
        Adt14::C0(_, h5158, _, _, _, _, _, _, _, _, _) => {
            h5158
        }
    };
    let a5161 = match v1.clone() {
        Adt14::C0(_, h5159, _, _, _, _, _, _, _, _, _) => {
            h5159
        }
    };
    if if if nat_eq(a5160, a5161) {
        let a5164 = match v0.clone() {
            Adt14::C0(_, _, h5162, _, _, _, _, _, _, _, _) => {
                h5162
            }
        };
        let a5165 = match v1.clone() {
            Adt14::C0(_, _, h5163, _, _, _, _, _, _, _, _) => {
                h5163
            }
        };
        nat_eq(a5164, a5165)
    } else {
        false
    } {
        let a5168 = match v0.clone() {
            Adt14::C0(_, _, _, h5166, _, _, _, _, _, _, _) => {
                h5166
            }
        };
        let a5169 = match v1.clone() {
            Adt14::C0(_, _, _, h5167, _, _, _, _, _, _, _) => {
                h5167
            }
        };
        nat_eq(a5168, a5169)
    } else {
        false
    } {
        let a5172 = match v0 {
            Adt14::C0(_, _, _, _, h5170, _, _, _, _, _, _) => {
                h5170
            }
        };
        let a5173 = match v1 {
            Adt14::C0(_, _, _, _, h5171, _, _, _, _, _, _) => {
                h5171
            }
        };
        nat_eq(a5172, a5173)
    } else {
        false
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5174 = v0;
    let a5175 = 0u64;
    if if if nat_eq(a5174, a5175) {
        true
    } else {
        let a5176 = v1;
        let a5177 = 0u64;
        nat_eq(a5176, a5177)
    } {
        true
    } else {
        let a5178 = v2;
        let a5179 = 0u64;
        nat_eq(a5178, a5179)
    } {
        true
    } else {
        let a5180 = v3;
        let a5181 = 0u64;
        nat_eq(a5180, a5181)
    }
}

pub fn f195(v0: Adt14, v1: Adt14) -> bool {
    if f201(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5182, _, _, _, _, _) => {
            h5182
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5183, _, _, _, _, _) => {
            h5183
        }
    }) {
        let a5185 = match v0 {
            Adt14::C0(_, h5184, _, _, _, _, _, _, _, _, _) => {
                h5184
            }
        };
        let a5186 = 2147483647u64;
        nat_le(a5185, a5186)
    } else {
        true
    }
}

pub fn f196(v0: Adt14) -> bool {
    let a5191 = {
        let a5188 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5187) => {
                h5187
            }
        };
        let a5189 = 4u64;
        let a5190 = 0u64;
        nat_rem(a5188, a5189, a5190)
    };
    let a5192 = 0u64;
    if if nat_eq(a5191, a5192) {
        f256(match v0.clone() {
            Adt14::C0(_, h5193, _, _, _, _, _, _, _, _, _) => {
                h5193
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5194, _, _, _, _, _) => {
                h5194
            }
        })
    } else {
        false
    } {
        f97(match v0.clone() {
            Adt14::C0(_, _, h5195, _, _, _, _, _, _, _, _) => {
                h5195
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5196, _, _, _, _, _, _, _) => {
                h5196
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5197, _, _, _, _, _, _) => {
                h5197
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5198, _, _, _, _) => {
                h5198
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5199, _, _, _) => {
                h5199
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5200, _, _) => {
                h5200
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f197(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5202 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5201, _) => {
            h5201
        }
    };
    let a5203 = 0u64;
    if nat_eq(a5202, a5203) {
        Ok(true)
    } else {
        let a5205 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5204, _) => {
                h5204
            }
        };
        let a5206 = 0u64;
        if nat_eq(a5205, a5206) {
            Ok(true)
        } else {
            let a5208 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5207) => {
                    h5207
                }
            };
            let a5209 = 0u64;
            if nat_eq(a5208, a5209) {
                Ok(true)
            } else {
                let a5211 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5210) => {
                        h5210
                    }
                };
                let a5212 = 0u64;
                if nat_eq(a5211, a5212) {
                    Ok(true)
                } else {
                    let a5218 = {
                        let a5215 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5213) => {
                                h5213
                            }
                        };
                        let a5216 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5214, _) => {
                                h5214
                            }
                        };
                        nat_add(a5215, a5216)?
                    };
                    let a5219 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5217) => {
                            h5217
                        }
                    };
                    if nat_le(a5218, a5219) {
                        Ok(true)
                    } else {
                        let a5225 = {
                            let a5222 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5220) => {
                                    h5220
                                }
                            };
                            let a5223 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5221, _) => {
                                    h5221
                                }
                            };
                            nat_add(a5222, a5223)?
                        };
                        let a5226 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5224) => {
                                h5224
                            }
                        };
                        Ok(nat_le(a5225, a5226))
                    }
                }
            }
        }
    }
}

pub fn f198(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f193(v0.clone(), v1.clone()) {
        let a5229 = match v0.clone() {
            Adt14::C0(h5227, _, _, _, _, _, _, _, _, _, _) => {
                h5227
            }
        };
        let a5230 = match v1.clone() {
            Adt14::C0(h5228, _, _, _, _, _, _, _, _, _, _) => {
                h5228
            }
        };
        nat_eq(a5229, a5230)
    } else {
        false
    } {
        let a5233 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5231) => {
                h5231
            }
        };
        let a5234 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5232) => {
                h5232
            }
        };
        nat_eq(a5233, a5234)
    } else {
        false
    } {
        let a5237 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5235, _, _, _, _, _) => {
                h5235
            }
        };
        let a5238 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5236, _, _, _, _, _) => {
                h5236
            }
        };
        nat_eq(a5237, a5238)
    } else {
        false
    } {
        let a5241 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5239, _, _, _, _) => {
                h5239
            }
        };
        let a5242 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5240, _, _, _, _) => {
                h5240
            }
        };
        nat_eq(a5241, a5242)
    } else {
        false
    } {
        let a5245 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5243, _, _, _) => {
                h5243
            }
        };
        let a5246 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5244, _, _, _) => {
                h5244
            }
        };
        nat_eq(a5245, a5246)
    } else {
        false
    } {
        let a5249 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5247, _, _) => {
                h5247
            }
        };
        let a5250 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5248, _, _) => {
                h5248
            }
        };
        nat_eq(a5249, a5250)
    } else {
        false
    }
}

pub fn f199(v0: Adt14) -> R<bool> {
    if f257(match v0.clone() {
        Adt14::C0(_, h5251, _, _, _, _, _, _, _, _, _) => {
            h5251
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5252, _, _, _, _, _) => {
            h5252
        }
    }) {
        f98(f258(match v0.clone() {
            Adt14::C0(_, h5253, _, _, _, _, _, _, _, _, _) => {
                h5253
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5254, _, _, _, _, _) => {
                h5254
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5255, _, _, _, _, _, _, _, _) => {
                h5255
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5256, _, _, _, _, _, _, _) => {
                h5256
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5257, _, _, _, _, _, _) => {
                h5257
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5258, _, _, _, _) => {
                h5258
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5259, _, _, _) => {
                h5259
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5260, _, _) => {
                h5260
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f200(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5261 = v1;
    let a5262 = 0u64;
    if if nat_eq(a5261, a5262) {
        true
    } else {
        let a5263 = v2;
        let a5264 = 0u64;
        nat_eq(a5263, a5264)
    } {
        Ok(0u64)
    } else {
        let a5267 = f99(v1, v2, v3)?;
        let a5268 = {
            let a5265 = v2;
            let a5266 = f259(v0, v1)?;
            nat_mul(a5265, a5266)?
        };
        nat_add(a5267, a5268)
    }
}

pub fn f201(v0: u64, v1: u64) -> bool {
    let a5269 = v0;
    let a5270 = 4u64;
    if nat_eq(a5269, a5270) {
        let a5271 = v1;
        let a5272 = 4u64;
        nat_eq(a5271, a5272)
    } else {
        false
    }
}

pub fn f202(v0: u64) -> R<u64> {
    let a5273 = 6u64;
    let a5274 = v0;
    nat_add(a5273, a5274)
}

pub fn f203(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5275, _, _, _, _, _, _, _, _, _, _) => {
            h5275
        }
    }, match v0.clone() {
        Adt14::C0(_, h5276, _, _, _, _, _, _, _, _, _) => {
            h5276
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5277, _, _, _, _, _, _, _, _) => {
            h5277
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5278, _, _, _, _, _, _, _) => {
            h5278
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5279, _, _, _, _, _, _) => {
            h5279
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5280, _, _, _, _) => {
            h5280
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5281, _, _, _) => {
            h5281
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5282, _, _) => {
            h5282
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5283, _) => {
            h5283
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5284) => {
            h5284
        }
    })
}

pub fn f204(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5285 = v0;
    let a5286 = 2147483647u64;
    if if if nat_le(a5285, a5286) {
        let a5287 = 0u64;
        let a5288 = v3;
        if nat_lt(a5287, a5288) {
            let a5289 = v3;
            let a5290 = 2147483647u64;
            nat_le(a5289, a5290)
        } else {
            false
        }
    } else {
        false
    } {
        let a5295 = {
            let a5293 = {
                let a5291 = v2;
                let a5292 = v3;
                nat_add(a5291, a5292)?
            };
            let a5294 = 1u64;
            nat_sub(a5293, a5294)
        };
        let a5296 = 2147483647u64;
        nat_le(a5295, a5296)
    } else {
        false
    } {
        let a5301 = {
            let a5299 = {
                let a5297 = v0;
                let a5298 = v1;
                nat_mul(a5297, a5298)?
            };
            let a5300 = f260(v2, v3)?;
            nat_mul(a5299, a5300)?
        };
        let a5302 = 9223372036854775807u64;
        Ok(nat_le(a5301, a5302))
    } else {
        Ok(false)
    }
}

pub fn f205(v0: Adt14) -> R<u64> {
    let a5333 = {
        let a5325 = {
            let a5317 = {
                let a5309 = 4u64;
                let a5310 = {
                    let a5307 = {
                        let a5304 = match v0.clone() {
                            Adt14::C0(_, h5303, _, _, _, _, _, _, _, _, _) => {
                                h5303
                            }
                        };
                        let a5305 = 1u64;
                        nat_sub(a5304, a5305)
                    };
                    let a5308 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5306, _, _, _, _, _) => {
                            h5306
                        }
                    };
                    nat_mul(a5307, a5308)?
                };
                nat_add(a5309, a5310)?
            };
            let a5318 = {
                let a5315 = {
                    let a5312 = match v0.clone() {
                        Adt14::C0(_, _, h5311, _, _, _, _, _, _, _, _) => {
                            h5311
                        }
                    };
                    let a5313 = 1u64;
                    nat_sub(a5312, a5313)
                };
                let a5316 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5314, _, _, _, _) => {
                        h5314
                    }
                };
                nat_mul(a5315, a5316)?
            };
            nat_add(a5317, a5318)?
        };
        let a5326 = {
            let a5323 = {
                let a5320 = match v0.clone() {
                    Adt14::C0(_, _, _, h5319, _, _, _, _, _, _, _) => {
                        h5319
                    }
                };
                let a5321 = 1u64;
                nat_sub(a5320, a5321)
            };
            let a5324 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5322, _, _, _) => {
                    h5322
                }
            };
            nat_mul(a5323, a5324)?
        };
        nat_add(a5325, a5326)?
    };
    let a5334 = {
        let a5331 = {
            let a5328 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5327, _, _, _, _, _, _) => {
                    h5327
                }
            };
            let a5329 = 1u64;
            nat_sub(a5328, a5329)
        };
        let a5332 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5330, _, _) => {
                h5330
            }
        };
        nat_mul(a5331, a5332)?
    };
    nat_add(a5333, a5334)
}

pub fn f206(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5335 = v0;
    let a5336 = 0u64;
    if if if if nat_eq(a5335, a5336) {
        true
    } else {
        let a5337 = v1;
        let a5338 = 0u64;
        nat_eq(a5337, a5338)
    } {
        true
    } else {
        let a5339 = v2;
        let a5340 = 0u64;
        nat_eq(a5339, a5340)
    } {
        true
    } else {
        let a5341 = v3;
        let a5342 = 0u64;
        nat_eq(a5341, a5342)
    } {
        Ok(0u64)
    } else {
        let a5343 = f261(v0, v1, v2, v3, v5)?;
        let a5344 = f262(v0, v1, v2, v3, v4)?;
        nat_add(a5343, a5344)
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5345 = v0;
    let a5346 = 0u64;
    if if nat_eq(a5345, a5346) {
        true
    } else {
        let a5347 = v1;
        let a5348 = 0u64;
        nat_eq(a5347, a5348)
    } {
        Ok(0u64)
    } else {
        let a5351 = f99(v0, v1, v2)?;
        let a5352 = {
            let a5349 = v1;
            let a5350 = f263(v0)?;
            nat_mul(a5349, a5350)?
        };
        nat_add(a5351, a5352)
    }
}

pub fn f208(v0: u64, v1: u64, v2: u64) -> bool {
    let a5353 = v0;
    let a5354 = 1u64;
    if nat_le(a5353, a5354) {
        true
    } else {
        let a5358 = {
            let a5355 = v1;
            let a5356 = v2;
            let a5357 = 1u64;
            nat_rem(a5355, a5356, a5357)
        };
        let a5359 = 0u64;
        nat_eq(a5358, a5359)
    }
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5360 = v1;
    let a5361 = 1u64;
    if if if nat_le(a5360, a5361) {
        true
    } else {
        let a5362 = v0;
        let a5363 = v2;
        nat_le(a5362, a5363)
    } {
        let a5364 = v3;
        let a5365 = 1u64;
        if nat_le(a5364, a5365) {
            true
        } else {
            let a5372 = {
                let a5370 = v0;
                let a5371 = {
                    let a5368 = {
                        let a5366 = v1;
                        let a5367 = 1u64;
                        nat_sub(a5366, a5367)
                    };
                    let a5369 = v2;
                    nat_mul(a5368, a5369)?
                };
                nat_add(a5370, a5371)?
            };
            let a5373 = v4;
            nat_le(a5372, a5373)
        }
    } else {
        false
    } {
        let a5374 = v5;
        let a5375 = 1u64;
        if nat_le(a5374, a5375) {
            Ok(true)
        } else {
            let a5388 = {
                let a5386 = {
                    let a5380 = v0;
                    let a5381 = {
                        let a5378 = {
                            let a5376 = v1;
                            let a5377 = 1u64;
                            nat_sub(a5376, a5377)
                        };
                        let a5379 = v2;
                        nat_mul(a5378, a5379)?
                    };
                    nat_add(a5380, a5381)?
                };
                let a5387 = {
                    let a5384 = {
                        let a5382 = v3;
                        let a5383 = 1u64;
                        nat_sub(a5382, a5383)
                    };
                    let a5385 = v4;
                    nat_mul(a5384, a5385)?
                };
                nat_add(a5386, a5387)?
            };
            let a5389 = v6;
            Ok(nat_le(a5388, a5389))
        }
    } else {
        Ok(false)
    }
}

pub fn f210(v0: u64) -> Adt18 {
    let a5390 = v0;
    let a5391 = 0u64;
    if nat_eq(a5390, a5391) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5392 = v0;
        let a5393 = 1u64;
        if nat_eq(a5392, a5393) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5394 = v0;
            let a5395 = 2u64;
            if nat_eq(a5394, a5395) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5396 = v0;
                let a5397 = 3u64;
                if nat_eq(a5396, a5397) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5398 = v0;
                    let a5399 = 6u64;
                    if nat_eq(a5398, a5399) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5400 = v0;
                        let a5401 = 7u64;
                        if nat_eq(a5400, a5401) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5402 = v0;
                            let a5403 = 8u64;
                            if nat_eq(a5402, a5403) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5404 = v0;
                                let a5405 = 9u64;
                                if nat_eq(a5404, a5405) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5406 = v0;
                                    let a5407 = 10u64;
                                    if nat_eq(a5406, a5407) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5408 = v0;
                                        let a5409 = 11u64;
                                        if nat_eq(a5408, a5409) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5410 = v0;
                                            let a5411 = 12u64;
                                            if nat_eq(a5410, a5411) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5412 = v0;
                                                let a5413 = 13u64;
                                                if nat_eq(a5412, a5413) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5414 = v0;
                                                    let a5415 = 14u64;
                                                    if nat_eq(a5414, a5415) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5416 = v0;
                                                        let a5417 = 15u64;
                                                        if nat_eq(a5416, a5417) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5418 = v0;
                                                            let a5419 = 16u64;
                                                            if nat_eq(a5418, a5419) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5420 = v0;
                                                                let a5421 = 17u64;
                                                                if nat_eq(a5420, a5421) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5422 = v0;
                                                                    let a5423 = 18u64;
                                                                    if nat_eq(a5422, a5423) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5424 = v0;
                                                                        let a5425 = 19u64;
                                                                        if nat_eq(a5424, a5425) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5426 = v0;
                                                                            let a5427 = 20u64;
                                                                            if nat_eq(a5426, a5427) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5428 = v0;
                                                                                let a5429 = 21u64;
                                                                                if nat_eq(a5428, a5429) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5430 = v0;
                                                                                    let a5431 = 22u64;
                                                                                    if nat_eq(a5430, a5431) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5432 = v0;
                                                                                        let a5433 = 23u64;
                                                                                        if nat_eq(a5432, a5433) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5434 = v0;
                                                                                            let a5435 = 24u64;
                                                                                            if nat_eq(a5434, a5435) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5436 = v0;
                                                                                                let a5437 = 25u64;
                                                                                                if nat_eq(a5436, a5437) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5438 = v0;
                                                                                                    let a5439 = 26u64;
                                                                                                    if nat_eq(a5438, a5439) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5440 = v0;
                                                                                                        let a5441 = 27u64;
                                                                                                        if nat_eq(a5440, a5441) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5442 = v0;
                                                                                                            let a5443 = 28u64;
                                                                                                            if nat_eq(a5442, a5443) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5444 = v0;
                                                                                                                let a5445 = 29u64;
                                                                                                                if nat_eq(a5444, a5445) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5446 = v0;
                                                                                                                    let a5447 = 30u64;
                                                                                                                    if nat_eq(a5446, a5447) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5448 = v0;
                                                                                                                        let a5449 = 34u64;
                                                                                                                        if nat_eq(a5448, a5449) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5450 = v0;
                                                                                                                            let a5451 = 35u64;
                                                                                                                            if nat_eq(a5450, a5451) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5452 = v0;
                                                                                                                                let a5453 = 39u64;
                                                                                                                                if nat_eq(a5452, a5453) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5454 = v0;
                                                                                                                                    let a5455 = 40u64;
                                                                                                                                    if nat_eq(a5454, a5455) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5456 = v0;
                                                                                                                                        let a5457 = 41u64;
                                                                                                                                        if nat_eq(a5456, a5457) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5458 = v0;
                                                                                                                                            let a5459 = 42u64;
                                                                                                                                            if nat_eq(a5458, a5459) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5460 = v0;
                                                                                                                                                let a5461 = 43u64;
                                                                                                                                                if nat_eq(a5460, a5461) {
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

pub fn f211(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5463 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5462, _, _, _, _, _, _) => {
                    h5462
                }
            };
            let a5464 = 1u64;
            if if if if if nat_le(a5463, a5464) {
                true
            } else {
                let a5466 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5465, _, _) => {
                        h5465
                    }
                };
                let a5467 = v1;
                nat_eq(a5466, a5467)
            } {
                let a5469 = match v0.clone() {
                    Adt2::C0(_, h5468, _, _, _, _, _, _, _, _, _) => {
                        h5468
                    }
                };
                let a5470 = 1u64;
                if nat_le(a5469, a5470) {
                    true
                } else {
                    let a5475 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5471, _, _, _, _, _) => {
                            h5471
                        }
                    };
                    let a5476 = {
                        let a5473 = v1;
                        let a5474 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5472, _, _, _, _, _, _) => {
                                h5472
                            }
                        };
                        nat_mul(a5473, a5474)?
                    };
                    nat_eq(a5475, a5476)
                }
            } else {
                false
            } {
                let a5478 = match v0.clone() {
                    Adt2::C0(_, _, h5477, _, _, _, _, _, _, _, _) => {
                        h5477
                    }
                };
                let a5479 = 1u64;
                if nat_le(a5478, a5479) {
                    true
                } else {
                    let a5487 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5480, _, _, _, _) => {
                            h5480
                        }
                    };
                    let a5488 = {
                        let a5485 = {
                            let a5482 = v1;
                            let a5483 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5481, _, _, _, _, _, _) => {
                                    h5481
                                }
                            };
                            nat_mul(a5482, a5483)?
                        };
                        let a5486 = match v0.clone() {
                            Adt2::C0(_, h5484, _, _, _, _, _, _, _, _, _) => {
                                h5484
                            }
                        };
                        nat_mul(a5485, a5486)?
                    };
                    nat_eq(a5487, a5488)
                }
            } else {
                false
            } {
                let a5490 = match v0.clone() {
                    Adt2::C0(_, _, _, h5489, _, _, _, _, _, _, _) => {
                        h5489
                    }
                };
                let a5491 = 1u64;
                if nat_le(a5490, a5491) {
                    true
                } else {
                    let a5502 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5492, _, _, _) => {
                            h5492
                        }
                    };
                    let a5503 = {
                        let a5500 = {
                            let a5497 = {
                                let a5494 = v1;
                                let a5495 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5493, _, _, _, _, _, _) => {
                                        h5493
                                    }
                                };
                                nat_mul(a5494, a5495)?
                            };
                            let a5498 = match v0.clone() {
                                Adt2::C0(_, h5496, _, _, _, _, _, _, _, _, _) => {
                                    h5496
                                }
                            };
                            nat_mul(a5497, a5498)?
                        };
                        let a5501 = match v0.clone() {
                            Adt2::C0(_, _, h5499, _, _, _, _, _, _, _, _) => {
                                h5499
                            }
                        };
                        nat_mul(a5500, a5501)?
                    };
                    nat_eq(a5502, a5503)
                }
            } else {
                false
            } {
                let a5517 = {
                    let a5514 = {
                        let a5511 = {
                            let a5508 = {
                                let a5505 = v1;
                                let a5506 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5504, _, _, _, _, _, _) => {
                                        h5504
                                    }
                                };
                                nat_mul(a5505, a5506)?
                            };
                            let a5509 = match v0.clone() {
                                Adt2::C0(_, h5507, _, _, _, _, _, _, _, _, _) => {
                                    h5507
                                }
                            };
                            nat_mul(a5508, a5509)?
                        };
                        let a5512 = match v0.clone() {
                            Adt2::C0(_, _, h5510, _, _, _, _, _, _, _, _) => {
                                h5510
                            }
                        };
                        nat_mul(a5511, a5512)?
                    };
                    let a5515 = match v0.clone() {
                        Adt2::C0(_, _, _, h5513, _, _, _, _, _, _, _) => {
                            h5513
                        }
                    };
                    nat_mul(a5514, a5515)?
                };
                let a5518 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5516, _) => {
                        h5516
                    }
                };
                Ok(nat_le(a5517, a5518))
            } else {
                Ok(false)
            }
        } else {
            let a5520 = match v0.clone() {
                Adt2::C0(_, _, _, h5519, _, _, _, _, _, _, _) => {
                    h5519
                }
            };
            let a5521 = 1u64;
            if if if if if nat_le(a5520, a5521) {
                true
            } else {
                let a5523 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5522, _, _, _) => {
                        h5522
                    }
                };
                let a5524 = v1;
                nat_eq(a5523, a5524)
            } {
                let a5526 = match v0.clone() {
                    Adt2::C0(_, h5525, _, _, _, _, _, _, _, _, _) => {
                        h5525
                    }
                };
                let a5527 = 1u64;
                if nat_le(a5526, a5527) {
                    true
                } else {
                    let a5532 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5528, _, _, _, _, _) => {
                            h5528
                        }
                    };
                    let a5533 = {
                        let a5530 = v1;
                        let a5531 = match v0.clone() {
                            Adt2::C0(_, _, _, h5529, _, _, _, _, _, _, _) => {
                                h5529
                            }
                        };
                        nat_mul(a5530, a5531)?
                    };
                    nat_eq(a5532, a5533)
                }
            } else {
                false
            } {
                let a5535 = match v0.clone() {
                    Adt2::C0(_, _, h5534, _, _, _, _, _, _, _, _) => {
                        h5534
                    }
                };
                let a5536 = 1u64;
                if nat_le(a5535, a5536) {
                    true
                } else {
                    let a5544 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5537, _, _, _, _) => {
                            h5537
                        }
                    };
                    let a5545 = {
                        let a5542 = {
                            let a5539 = v1;
                            let a5540 = match v0.clone() {
                                Adt2::C0(_, _, _, h5538, _, _, _, _, _, _, _) => {
                                    h5538
                                }
                            };
                            nat_mul(a5539, a5540)?
                        };
                        let a5543 = match v0.clone() {
                            Adt2::C0(_, h5541, _, _, _, _, _, _, _, _, _) => {
                                h5541
                            }
                        };
                        nat_mul(a5542, a5543)?
                    };
                    nat_eq(a5544, a5545)
                }
            } else {
                false
            } {
                let a5547 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5546, _, _, _, _, _, _) => {
                        h5546
                    }
                };
                let a5548 = 1u64;
                if nat_le(a5547, a5548) {
                    true
                } else {
                    let a5559 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5549, _, _) => {
                            h5549
                        }
                    };
                    let a5560 = {
                        let a5557 = {
                            let a5554 = {
                                let a5551 = v1;
                                let a5552 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5550, _, _, _, _, _, _, _) => {
                                        h5550
                                    }
                                };
                                nat_mul(a5551, a5552)?
                            };
                            let a5555 = match v0.clone() {
                                Adt2::C0(_, h5553, _, _, _, _, _, _, _, _, _) => {
                                    h5553
                                }
                            };
                            nat_mul(a5554, a5555)?
                        };
                        let a5558 = match v0.clone() {
                            Adt2::C0(_, _, h5556, _, _, _, _, _, _, _, _) => {
                                h5556
                            }
                        };
                        nat_mul(a5557, a5558)?
                    };
                    nat_eq(a5559, a5560)
                }
            } else {
                false
            } {
                let a5574 = {
                    let a5571 = {
                        let a5568 = {
                            let a5565 = {
                                let a5562 = v1;
                                let a5563 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5561, _, _, _, _, _, _, _) => {
                                        h5561
                                    }
                                };
                                nat_mul(a5562, a5563)?
                            };
                            let a5566 = match v0.clone() {
                                Adt2::C0(_, h5564, _, _, _, _, _, _, _, _, _) => {
                                    h5564
                                }
                            };
                            nat_mul(a5565, a5566)?
                        };
                        let a5569 = match v0.clone() {
                            Adt2::C0(_, _, h5567, _, _, _, _, _, _, _, _) => {
                                h5567
                            }
                        };
                        nat_mul(a5568, a5569)?
                    };
                    let a5572 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5570, _, _, _, _, _, _) => {
                            h5570
                        }
                    };
                    nat_mul(a5571, a5572)?
                };
                let a5575 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5573, _) => {
                        h5573
                    }
                };
                Ok(nat_le(a5574, a5575))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5577 = match v0.clone() {
            Adt2::C0(_, h5576, _, _, _, _, _, _, _, _, _) => {
                h5576
            }
        };
        let a5578 = 1u64;
        if if if if if nat_le(a5577, a5578) {
            true
        } else {
            let a5580 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5579, _, _, _, _, _) => {
                    h5579
                }
            };
            let a5581 = v1;
            nat_eq(a5580, a5581)
        } {
            let a5583 = match v0.clone() {
                Adt2::C0(_, _, h5582, _, _, _, _, _, _, _, _) => {
                    h5582
                }
            };
            let a5584 = 1u64;
            if nat_le(a5583, a5584) {
                true
            } else {
                let a5589 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5585, _, _, _, _) => {
                        h5585
                    }
                };
                let a5590 = {
                    let a5587 = v1;
                    let a5588 = match v0.clone() {
                        Adt2::C0(_, h5586, _, _, _, _, _, _, _, _, _) => {
                            h5586
                        }
                    };
                    nat_mul(a5587, a5588)?
                };
                nat_eq(a5589, a5590)
            }
        } else {
            false
        } {
            let a5592 = match v0.clone() {
                Adt2::C0(_, _, _, h5591, _, _, _, _, _, _, _) => {
                    h5591
                }
            };
            let a5593 = 1u64;
            if nat_le(a5592, a5593) {
                true
            } else {
                let a5601 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5594, _, _, _) => {
                        h5594
                    }
                };
                let a5602 = {
                    let a5599 = {
                        let a5596 = v1;
                        let a5597 = match v0.clone() {
                            Adt2::C0(_, h5595, _, _, _, _, _, _, _, _, _) => {
                                h5595
                            }
                        };
                        nat_mul(a5596, a5597)?
                    };
                    let a5600 = match v0.clone() {
                        Adt2::C0(_, _, h5598, _, _, _, _, _, _, _, _) => {
                            h5598
                        }
                    };
                    nat_mul(a5599, a5600)?
                };
                nat_eq(a5601, a5602)
            }
        } else {
            false
        } {
            let a5604 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5603, _, _, _, _, _, _) => {
                    h5603
                }
            };
            let a5605 = 1u64;
            if nat_le(a5604, a5605) {
                true
            } else {
                let a5616 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5606, _, _) => {
                        h5606
                    }
                };
                let a5617 = {
                    let a5614 = {
                        let a5611 = {
                            let a5608 = v1;
                            let a5609 = match v0.clone() {
                                Adt2::C0(_, h5607, _, _, _, _, _, _, _, _, _) => {
                                    h5607
                                }
                            };
                            nat_mul(a5608, a5609)?
                        };
                        let a5612 = match v0.clone() {
                            Adt2::C0(_, _, h5610, _, _, _, _, _, _, _, _) => {
                                h5610
                            }
                        };
                        nat_mul(a5611, a5612)?
                    };
                    let a5615 = match v0.clone() {
                        Adt2::C0(_, _, _, h5613, _, _, _, _, _, _, _) => {
                            h5613
                        }
                    };
                    nat_mul(a5614, a5615)?
                };
                nat_eq(a5616, a5617)
            }
        } else {
            false
        } {
            let a5631 = {
                let a5628 = {
                    let a5625 = {
                        let a5622 = {
                            let a5619 = v1;
                            let a5620 = match v0.clone() {
                                Adt2::C0(_, h5618, _, _, _, _, _, _, _, _, _) => {
                                    h5618
                                }
                            };
                            nat_mul(a5619, a5620)?
                        };
                        let a5623 = match v0.clone() {
                            Adt2::C0(_, _, h5621, _, _, _, _, _, _, _, _) => {
                                h5621
                            }
                        };
                        nat_mul(a5622, a5623)?
                    };
                    let a5626 = match v0.clone() {
                        Adt2::C0(_, _, _, h5624, _, _, _, _, _, _, _) => {
                            h5624
                        }
                    };
                    nat_mul(a5625, a5626)?
                };
                let a5629 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5627, _, _, _, _, _, _) => {
                        h5627
                    }
                };
                nat_mul(a5628, a5629)?
            };
            let a5632 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5630, _) => {
                    h5630
                }
            };
            Ok(nat_le(a5631, a5632))
        } else {
            Ok(false)
        }
    }
}

pub fn f212(v0: Adt2, v1: u64) -> R<bool> {
    let a5634 = match v0.clone() {
        Adt2::C0(h5633, _, _, _, _, _, _, _, _, _, _) => {
            h5633
        }
    };
    let a5635 = 0u64;
    if if nat_eq(a5634, a5635) {
        f38(v0.clone())?
    } else {
        false
    } {
        let a5636 = v1;
        let a5637 = 1u64;
        if nat_le(a5636, a5637) {
            Ok(true)
        } else {
            let a5642 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5638, _, _, _, _) => {
                    h5638
                }
            };
            let a5643 = {
                let a5640 = 4u64;
                let a5641 = match v0 {
                    Adt2::C0(_, h5639, _, _, _, _, _, _, _, _, _) => {
                        h5639
                    }
                };
                nat_mul(a5640, a5641)?
            };
            Ok(nat_eq(a5642, a5643))
        }
    } else {
        Ok(false)
    }
}

pub fn f213(v0: Adt2, v1: u64) -> R<bool> {
    let a5645 = match v0.clone() {
        Adt2::C0(_, _, _, h5644, _, _, _, _, _, _, _) => {
            h5644
        }
    };
    let a5646 = 1u64;
    if if if nat_eq(a5645, a5646) {
        let a5648 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5647, _, _, _, _, _, _) => {
                h5647
            }
        };
        let a5649 = 1u64;
        nat_eq(a5648, a5649)
    } else {
        false
    } {
        let a5651 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5650, _, _, _, _, _) => {
                h5650
            }
        };
        let a5652 = v1;
        nat_eq(a5651, a5652)
    } else {
        false
    } {
        let a5654 = match v0.clone() {
            Adt2::C0(_, _, h5653, _, _, _, _, _, _, _, _) => {
                h5653
            }
        };
        let a5655 = 1u64;
        if nat_eq(a5654, a5655) {
            Ok(true)
        } else {
            let a5660 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5656, _, _, _, _) => {
                    h5656
                }
            };
            let a5661 = {
                let a5658 = match v0 {
                    Adt2::C0(_, h5657, _, _, _, _, _, _, _, _, _) => {
                        h5657
                    }
                };
                let a5659 = v1;
                nat_mul(a5658, a5659)?
            };
            Ok(nat_eq(a5660, a5661))
        }
    } else {
        Ok(false)
    }
}

pub fn f214(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5665 = {
            let a5662 = v2;
            let a5663 = 1u64;
            nat_sub(a5662, a5663)
        };
        let a5666 = match v0.clone() {
            Adt6::C0(_, h5664, _, _) => {
                h5664
            }
        };
        nat_mul(a5665, a5666)?
    };
    let a5668 = 0u64;
    let a5669 = match v0.clone() {
        Adt6::C0(h5667, _, _, _) => {
            h5667
        }
    };
    if if if if if if if if nat_lt(a5668, a5669) {
        let a5671 = match v0.clone() {
            Adt6::C0(h5670, _, _, _) => {
                h5670
            }
        };
        let a5672 = 2147483648u64;
        nat_lt(a5671, a5672)
    } else {
        false
    } {
        let a5674 = 0u64;
        let a5675 = match v0.clone() {
            Adt6::C0(_, h5673, _, _) => {
                h5673
            }
        };
        nat_lt(a5674, a5675)
    } else {
        false
    } {
        let a5677 = match v0.clone() {
            Adt6::C0(_, h5676, _, _) => {
                h5676
            }
        };
        let a5678 = 2147483648u64;
        nat_lt(a5677, a5678)
    } else {
        false
    } {
        let a5679 = v1;
        let a5680 = 2147483648u64;
        nat_lt(a5679, a5680)
    } else {
        false
    } {
        let a5681 = 0u64;
        let a5682 = v2;
        nat_lt(a5681, a5682)
    } else {
        false
    } {
        let a5683 = v3;
        let a5684 = 2147483648u64;
        nat_lt(a5683, a5684)
    } else {
        false
    } {
        let a5696 = {
            let a5691 = {
                let a5688 = v3;
                let a5689 = {
                    let a5686 = match v0.clone() {
                        Adt6::C0(h5685, _, _, _) => {
                            h5685
                        }
                    };
                    let a5687 = 1u64;
                    nat_sub(a5686, a5687)
                };
                nat_add(a5688, a5689)?
            };
            let a5692 = f139(match v0.clone() {
                Adt6::C0(_, _, h5690, _) => {
                    h5690
                }
            });
            nat_add(a5691, a5692)?
        };
        let a5697 = {
            let a5694 = 2147483647u64;
            let a5695 = f140(match v0.clone() {
                Adt6::C0(_, _, h5693, _) => {
                    h5693
                }
            });
            nat_add(a5694, a5695)?
        };
        nat_le(a5696, a5697)
    } else {
        false
    } {
        let a5699 = match v0 {
            Adt6::C0(_, _, h5698, _) => {
                h5698
            }
        };
        let a5700 = 2147483648u64;
        Ok(!nat_eq(a5699, a5700))
    } else {
        Ok(false)
    }
}

pub fn f215(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f264(match v0.clone() {
        Adt2::C0(h5701, _, _, _, _, _, _, _, _, _, _) => {
            h5701
        }
    }, match v0.clone() {
        Adt2::C0(_, h5702, _, _, _, _, _, _, _, _, _) => {
            h5702
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5703, _, _, _, _, _, _, _, _) => {
            h5703
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5704, _, _, _, _, _, _, _) => {
            h5704
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5705, _, _, _, _, _, _) => {
            h5705
        }
    }, match v1.clone() {
        Adt2::C0(h5706, _, _, _, _, _, _, _, _, _, _) => {
            h5706
        }
    }, match v1.clone() {
        Adt2::C0(_, h5707, _, _, _, _, _, _, _, _, _) => {
            h5707
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5708, _, _, _, _, _, _, _, _) => {
            h5708
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5709, _, _, _, _, _, _, _) => {
            h5709
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5710, _, _, _, _, _, _) => {
            h5710
        }
    }, match v2.clone() {
        Adt2::C0(h5711, _, _, _, _, _, _, _, _, _, _) => {
            h5711
        }
    }, match v2.clone() {
        Adt2::C0(_, h5712, _, _, _, _, _, _, _, _, _) => {
            h5712
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5713, _, _, _, _, _, _, _, _) => {
            h5713
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5714, _, _, _, _, _, _, _) => {
            h5714
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5715, _, _, _, _, _, _) => {
            h5715
        }
    }) {
        let m5717 = f210(match v0.clone() {
            Adt2::C0(h5716, _, _, _, _, _, _, _, _, _, _) => {
                h5716
            }
        });
        match m5717 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5723 = {
                    let a5720 = match v0.clone() {
                        Adt2::C0(_, h5718, _, _, _, _, _, _, _, _, _) => {
                            h5718
                        }
                    };
                    let a5721 = match v3.clone() {
                        Adt19::C0(_, h5719, _) => {
                            h5719
                        }
                    };
                    let a5722 = 0u64;
                    nat_rem(a5720, a5721, a5722)
                };
                let a5724 = 0u64;
                if if nat_eq(a5723, a5724) {
                    if f265(match v0.clone() {
                        Adt2::C0(_, h5725, _, _, _, _, _, _, _, _, _) => {
                            h5725
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5726, _, _, _, _, _, _, _, _, _) => {
                            h5726
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5727, _, _, _, _, _, _, _, _) => {
                            h5727
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5728, _, _, _, _, _, _, _) => {
                            h5728
                        }
                    }) {
                        true
                    } else {
                        let a5730 = 4u64;
                        let a5731 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5729, _, _, _, _, _) => {
                                h5729
                            }
                        };
                        if if nat_le(a5730, a5731) {
                            let a5733 = match v3.clone() {
                                Adt19::C0(_, h5732, _) => {
                                    h5732
                                }
                            };
                            let a5734 = 1u64;
                            if nat_eq(a5733, a5734) {
                                true
                            } else {
                                let a5737 = match v3.clone() {
                                    Adt19::C0(_, _, h5735) => {
                                        h5735
                                    }
                                };
                                let a5738 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5736, _, _, _, _, _) => {
                                        h5736
                                    }
                                };
                                nat_le(a5737, a5738)
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
                        Adt19::C0(_, h5739, _) => {
                            h5739
                        }
                    });
                    Ok(Adt10::C4(if f265(match v0.clone() {
                        Adt2::C0(_, h5740, _, _, _, _, _, _, _, _, _) => {
                            h5740
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5741, _, _, _, _, _, _, _, _, _) => {
                            h5741
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5742, _, _, _, _, _, _, _, _) => {
                            h5742
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5743, _, _, _, _, _, _, _) => {
                            h5743
                        }
                    }) {
                        0u64
                    } else {
                        f174(v4.clone(), f25(v1.clone(), 1u64))?
                    }, 4u64, v4, f25(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5744, _, _, _, _, _, _, _, _, _, _) => {
                            h5744
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

pub fn f216(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f217(match v0.clone() {
        Adt2::C0(h5745, _, _, _, _, _, _, _, _, _, _) => {
            h5745
        }
    }) {
        let a5751 = {
            let a5748 = match v0.clone() {
                Adt2::C0(_, h5746, _, _, _, _, _, _, _, _, _) => {
                    h5746
                }
            };
            let a5749 = f247(match v0.clone() {
                Adt2::C0(h5747, _, _, _, _, _, _, _, _, _, _) => {
                    h5747
                }
            });
            let a5750 = 1u64;
            nat_rem(a5748, a5749, a5750)
        };
        let a5752 = 0u64;
        nat_eq(a5751, a5752)
    } else {
        false
    } {
        let a5754 = match v1.clone() {
            Adt2::C0(h5753, _, _, _, _, _, _, _, _, _, _) => {
                h5753
            }
        };
        let a5755 = 26u64;
        nat_eq(a5754, a5755)
    } else {
        false
    } {
        let a5757 = match v2.clone() {
            Adt2::C0(h5756, _, _, _, _, _, _, _, _, _, _) => {
                h5756
            }
        };
        let a5758 = 0u64;
        nat_eq(a5757, a5758)
    } else {
        false
    } {
        let a5761 = match v0.clone() {
            Adt2::C0(_, h5759, _, _, _, _, _, _, _, _, _) => {
                h5759
            }
        };
        let a5762 = match v2.clone() {
            Adt2::C0(_, h5760, _, _, _, _, _, _, _, _, _) => {
                h5760
            }
        };
        nat_eq(a5761, a5762)
    } else {
        false
    } {
        let a5765 = match v1.clone() {
            Adt2::C0(_, h5763, _, _, _, _, _, _, _, _, _) => {
                h5763
            }
        };
        let a5766 = match v0.clone() {
            Adt2::C0(_, _, h5764, _, _, _, _, _, _, _, _) => {
                h5764
            }
        };
        nat_le(a5765, a5766)
    } else {
        false
    } {
        let a5768 = match v0.clone() {
            Adt2::C0(_, _, _, h5767, _, _, _, _, _, _, _) => {
                h5767
            }
        };
        let a5769 = 1u64;
        nat_eq(a5768, a5769)
    } else {
        false
    } {
        let a5771 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5770, _, _, _, _, _, _) => {
                h5770
            }
        };
        let a5772 = 1u64;
        nat_eq(a5771, a5772)
    } else {
        false
    } {
        let a5774 = match v2.clone() {
            Adt2::C0(_, _, _, h5773, _, _, _, _, _, _, _) => {
                h5773
            }
        };
        let a5775 = 1u64;
        nat_eq(a5774, a5775)
    } else {
        false
    } {
        let a5777 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5776, _, _, _, _, _, _) => {
                h5776
            }
        };
        let a5778 = 1u64;
        nat_eq(a5777, a5778)
    } else {
        false
    } {
        let a5780 = match v1.clone() {
            Adt2::C0(_, _, h5779, _, _, _, _, _, _, _, _) => {
                h5779
            }
        };
        let a5781 = 1u64;
        nat_eq(a5780, a5781)
    } else {
        false
    } {
        let a5783 = match v1.clone() {
            Adt2::C0(_, _, _, h5782, _, _, _, _, _, _, _) => {
                h5782
            }
        };
        let a5784 = 1u64;
        nat_eq(a5783, a5784)
    } else {
        false
    } {
        let a5786 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5785, _, _, _, _, _, _) => {
                h5785
            }
        };
        let a5787 = 1u64;
        nat_eq(a5786, a5787)
    } else {
        false
    } {
        if f48(match v2.clone() {
            Adt2::C0(_, h5788, _, _, _, _, _, _, _, _, _) => {
                h5788
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5789, _, _, _, _, _, _, _, _) => {
                h5789
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5790, _, _, _, _, _, _, _) => {
                h5790
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5791, _, _, _, _, _, _) => {
                h5791
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5793 = match v1.clone() {
                Adt2::C0(_, h5792, _, _, _, _, _, _, _, _, _) => {
                    h5792
                }
            };
            let a5794 = 0u64;
            if nat_eq(a5793, a5794) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5796 = match v0.clone() {
                    Adt2::C0(_, h5795, _, _, _, _, _, _, _, _, _) => {
                        h5795
                    }
                };
                let a5797 = 2147483647u64;
                if if if if if if nat_le(a5796, a5797) {
                    let a5799 = match v1.clone() {
                        Adt2::C0(_, h5798, _, _, _, _, _, _, _, _, _) => {
                            h5798
                        }
                    };
                    let a5800 = 2147483647u64;
                    nat_le(a5799, a5800)
                } else {
                    false
                } {
                    let a5802 = 4u64;
                    let a5803 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5801, _, _, _, _, _) => {
                            h5801
                        }
                    };
                    nat_le(a5802, a5803)
                } else {
                    false
                } {
                    let a5809 = {
                        let a5806 = match v2.clone() {
                            Adt2::C0(_, h5804, _, _, _, _, _, _, _, _, _) => {
                                h5804
                            }
                        };
                        let a5807 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5805, _, _, _, _, _) => {
                                h5805
                            }
                        };
                        nat_mul(a5806, a5807)?
                    };
                    let a5810 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5808, _, _, _, _) => {
                            h5808
                        }
                    };
                    nat_le(a5809, a5810)
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
                        let a5823 = {
                            let a5814 = {
                                let a5812 = 2u64;
                                let a5813 = match v1.clone() {
                                    Adt2::C0(_, h5811, _, _, _, _, _, _, _, _, _) => {
                                        h5811
                                    }
                                };
                                nat_mul(a5812, a5813)?
                            };
                            let a5815 = f23(v2.clone(), 1u64)?;
                            nat_add(a5814, a5815)?
                        };
                        let a5824 = {
                            let a5821 = match v1.clone() {
                                Adt2::C0(_, h5816, _, _, _, _, _, _, _, _, _) => {
                                    h5816
                                }
                            };
                            let a5822 = {
                                let a5819 = 1u64;
                                let a5820 = f171(match v0.clone() {
                                    Adt2::C0(h5817, _, _, _, _, _, _, _, _, _, _) => {
                                        h5817
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5818, _, _, _, _, _, _, _, _, _) => {
                                        h5818
                                    }
                                })?;
                                nat_add(a5819, a5820)?
                            };
                            nat_mul(a5821, a5822)?
                        };
                        nat_add(a5823, a5824)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5825, _, _, _, _, _, _, _, _, _) => {
                            h5825
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5826, _, _, _, _, _, _, _, _, _) => {
                            h5826
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5827, _, _, _, _, _) => {
                            h5827
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5828, _, _, _, _) => {
                            h5828
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5829, _, _, _) => {
                            h5829
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5830, _, _) => {
                            h5830
                        }
                    }), f25(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5831, _, _, _, _, _, _, _, _, _, _) => {
                            h5831
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

pub fn f217(v0: u64) -> bool {
    let a5832 = v0;
    let a5833 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5832, a5833) {
        true
    } else {
        let a5834 = v0;
        let a5835 = 1u64;
        nat_eq(a5834, a5835)
    } {
        true
    } else {
        let a5836 = v0;
        let a5837 = 2u64;
        nat_eq(a5836, a5837)
    } {
        true
    } else {
        let a5838 = v0;
        let a5839 = 3u64;
        nat_eq(a5838, a5839)
    } {
        true
    } else {
        let a5840 = v0;
        let a5841 = 6u64;
        nat_eq(a5840, a5841)
    } {
        true
    } else {
        let a5842 = v0;
        let a5843 = 7u64;
        nat_eq(a5842, a5843)
    } {
        true
    } else {
        let a5844 = v0;
        let a5845 = 8u64;
        nat_eq(a5844, a5845)
    } {
        true
    } else {
        let a5846 = v0;
        let a5847 = 10u64;
        nat_eq(a5846, a5847)
    } {
        true
    } else {
        let a5848 = v0;
        let a5849 = 11u64;
        nat_eq(a5848, a5849)
    } {
        true
    } else {
        let a5850 = v0;
        let a5851 = 12u64;
        nat_eq(a5850, a5851)
    } {
        true
    } else {
        let a5852 = v0;
        let a5853 = 13u64;
        nat_eq(a5852, a5853)
    } {
        true
    } else {
        let a5854 = v0;
        let a5855 = 14u64;
        nat_eq(a5854, a5855)
    } {
        true
    } else {
        let a5856 = v0;
        let a5857 = 16u64;
        nat_eq(a5856, a5857)
    } {
        true
    } else {
        let a5858 = v0;
        let a5859 = 17u64;
        nat_eq(a5858, a5859)
    } {
        true
    } else {
        let a5860 = v0;
        let a5861 = 18u64;
        nat_eq(a5860, a5861)
    } {
        true
    } else {
        let a5862 = v0;
        let a5863 = 19u64;
        nat_eq(a5862, a5863)
    } {
        true
    } else {
        let a5864 = v0;
        let a5865 = 20u64;
        nat_eq(a5864, a5865)
    } {
        true
    } else {
        let a5866 = v0;
        let a5867 = 21u64;
        nat_eq(a5866, a5867)
    } {
        true
    } else {
        let a5868 = v0;
        let a5869 = 22u64;
        nat_eq(a5868, a5869)
    } {
        true
    } else {
        let a5870 = v0;
        let a5871 = 23u64;
        nat_eq(a5870, a5871)
    } {
        true
    } else {
        let a5872 = v0;
        let a5873 = 26u64;
        nat_eq(a5872, a5873)
    } {
        true
    } else {
        let a5874 = v0;
        let a5875 = 29u64;
        nat_eq(a5874, a5875)
    } {
        true
    } else {
        let a5876 = v0;
        let a5877 = 30u64;
        nat_eq(a5876, a5877)
    } {
        true
    } else {
        let a5878 = v0;
        let a5879 = 34u64;
        nat_eq(a5878, a5879)
    } {
        true
    } else {
        let a5880 = v0;
        let a5881 = 35u64;
        nat_eq(a5880, a5881)
    } {
        true
    } else {
        let a5882 = v0;
        let a5883 = 39u64;
        nat_eq(a5882, a5883)
    } {
        true
    } else {
        let a5884 = v0;
        let a5885 = 40u64;
        nat_eq(a5884, a5885)
    } {
        true
    } else {
        let a5886 = v0;
        let a5887 = 41u64;
        nat_eq(a5886, a5887)
    } {
        true
    } else {
        let a5888 = v0;
        let a5889 = 42u64;
        nat_eq(a5888, a5889)
    } {
        true
    } else {
        let a5890 = v0;
        let a5891 = 43u64;
        nat_eq(a5890, a5891)
    }
}

pub fn f218(v0: u64) -> bool {
    if f172(v0) {
        true
    } else {
        let a5892 = v0;
        let a5893 = 43u64;
        nat_eq(a5892, a5893)
    }
}

pub fn f219(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5896 = match v1.clone() {
        Adt2::C0(h5894, _, _, _, _, _, _, _, _, _, _) => {
            h5894
        }
    };
    let a5897 = match v3.clone() {
        Adt2::C0(h5895, _, _, _, _, _, _, _, _, _, _) => {
            h5895
        }
    };
    if if nat_eq(a5896, a5897) {
        let a5898 = v0;
        let a5899 = 33u64;
        if nat_eq(a5898, a5899) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5901 = f210(match v1.clone() {
            Adt2::C0(h5900, _, _, _, _, _, _, _, _, _, _) => {
                h5900
            }
        });
        match m5901 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5902, _) => {
                        h5902
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a5907 = {
                    let a5904 = match v1.clone() {
                        Adt2::C0(_, h5903, _, _, _, _, _, _, _, _, _) => {
                            h5903
                        }
                    };
                    let a5905 = v5;
                    let a5906 = 1u64;
                    nat_rem(a5904, a5905, a5906)
                };
                let a5908 = 0u64;
                if if if nat_eq(a5907, a5908) {
                    let a5913 = {
                        let a5910 = match v3.clone() {
                            Adt2::C0(_, h5909, _, _, _, _, _, _, _, _, _) => {
                                h5909
                            }
                        };
                        let a5911 = v5;
                        let a5912 = 1u64;
                        nat_rem(a5910, a5911, a5912)
                    };
                    let a5914 = 0u64;
                    nat_eq(a5913, a5914)
                } else {
                    false
                } {
                    let a5915 = v6;
                    let a5916 = f23(v3.clone(), v5)?;
                    nat_eq(a5915, a5916)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h5917) => {
                            h5917
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

pub fn f220(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f48(v4, v5, v6, v7) {
        f48(v0, v1, v2, v3)
    } else {
        let a5921 = {
            let a5918 = v0;
            let a5919 = v4;
            let a5920 = 1u64;
            nat_rem(a5918, a5919, a5920)
        };
        let a5922 = 0u64;
        if if if nat_eq(a5921, a5922) {
            let a5926 = {
                let a5923 = v1;
                let a5924 = v5;
                let a5925 = 1u64;
                nat_rem(a5923, a5924, a5925)
            };
            let a5927 = 0u64;
            nat_eq(a5926, a5927)
        } else {
            false
        } {
            let a5931 = {
                let a5928 = v2;
                let a5929 = v6;
                let a5930 = 1u64;
                nat_rem(a5928, a5929, a5930)
            };
            let a5932 = 0u64;
            nat_eq(a5931, a5932)
        } else {
            false
        } {
            let a5936 = {
                let a5933 = v3;
                let a5934 = v7;
                let a5935 = 1u64;
                nat_rem(a5933, a5934, a5935)
            };
            let a5937 = 0u64;
            nat_eq(a5936, a5937)
        } else {
            false
        }
    }
}

pub fn f221(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5938 = v0;
    let a5939 = v1;
    if nat_eq(a5938, a5939) {
        let a5942 = v4;
        let a5943 = {
            let a5940 = v2;
            let a5941 = v3;
            nat_add(a5940, a5941)?
        };
        Ok(nat_eq(a5942, a5943))
    } else {
        let a5944 = v2;
        let a5945 = v3;
        if nat_eq(a5944, a5945) {
            let a5946 = v4;
            let a5947 = v2;
            Ok(nat_eq(a5946, a5947))
        } else {
            Ok(false)
        }
    }
}

pub fn f222(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5950 = {
            let a5948 = v0;
            let a5949 = f140(v1);
            nat_add(a5948, a5949)?
        };
        let a5951 = f140(v2);
        nat_add(a5950, a5951)?
    };
    let v5: u64 = {
        let a5952 = f139(v1);
        let a5953 = f139(v2);
        nat_add(a5952, a5953)?
    };
    let a5954 = v1;
    let a5955 = 4294967295u64;
    if if nat_le(a5954, a5955) {
        let a5956 = v2;
        let a5957 = 4294967295u64;
        nat_le(a5956, a5957)
    } else {
        false
    } {
        let a5958 = v5;
        let a5959 = v4;
        if nat_le(a5958, a5959) {
            let a5962 = v3;
            let a5963 = {
                let a5960 = v4;
                let a5961 = v5;
                nat_sub(a5960, a5961)
            };
            Ok(nat_eq(a5962, a5963))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f223(v0: u64, v1: u64) -> R<bool> {
    let a5964 = v1;
    let a5965 = 4294967295u64;
    if nat_le(a5964, a5965) {
        let a5968 = {
            let a5966 = f140(v1);
            let a5967 = f139(v1);
            nat_add(a5966, a5967)?
        };
        let a5969 = v0;
        Ok(nat_lt(a5968, a5969))
    } else {
        Ok(false)
    }
}

pub fn f224(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f266(v0.clone(), false)?;
    let v6: u64 = f266(v0, true)?;
    let a5970 = v6;
    let a5971 = v1;
    if if nat_le(a5970, a5971) {
        let a5978 = {
            let a5976 = {
                let a5972 = v1;
                let a5973 = v5;
                nat_add(a5972, a5973)?
            };
            let a5977 = {
                let a5974 = v3;
                let a5975 = 0u64;
                if nat_eq(a5974, a5975) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5976, a5977)?
        };
        let a5979 = v2;
        nat_le(a5978, a5979)
    } else {
        false
    } {
        if v4 {
            let a5984 = {
                let a5982 = {
                    let a5980 = v1;
                    let a5981 = v5;
                    nat_add(a5980, a5981)?
                };
                let a5983 = v6;
                nat_sub(a5982, a5983)
            };
            let a5985 = v2;
            Ok(nat_lt(a5984, a5985))
        } else {
            let a5990 = {
                let a5988 = {
                    let a5986 = v1;
                    let a5987 = v5;
                    nat_add(a5986, a5987)?
                };
                let a5989 = v6;
                nat_sub(a5988, a5989)
            };
            let a5991 = v2;
            Ok(nat_le(a5990, a5991))
        }
    } else {
        Ok(false)
    }
}

pub fn f225(v0: u64, v1: u64) -> bool {
    let a5992 = v0;
    let a5993 = 0u64;
    if if nat_eq(a5992, a5993) {
        true
    } else {
        let a5994 = v0;
        let a5995 = 1u64;
        nat_eq(a5994, a5995)
    } {
        let a5996 = v1;
        let a5997 = 4294967296u64;
        nat_lt(a5996, a5997)
    } else {
        false
    }
}

pub fn f226(v0: u64, v1: u64) -> R<u64> {
    let a6002 = {
        let a6000 = v0;
        let a6001 = {
            let a5998 = v1;
            let a5999 = 1u64;
            nat_sub(a5998, a5999)
        };
        nat_add(a6000, a6001)?
    };
    let a6003 = v1;
    let a6004 = 0u64;
    Ok(nat_quot(a6002, a6003, a6004))
}

pub fn f227(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f48(match v0.clone() {
        Adt2::C0(_, h6005, _, _, _, _, _, _, _, _, _) => {
            h6005
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6006, _, _, _, _, _, _, _, _) => {
            h6006
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6007, _, _, _, _, _, _, _) => {
            h6007
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6008, _, _, _, _, _, _) => {
            h6008
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6011 = match v1.clone() {
                Adt2::C0(_, h6009, _, _, _, _, _, _, _, _, _) => {
                    h6009
                }
            };
            let a6012 = match v1.clone() {
                Adt2::C0(_, _, h6010, _, _, _, _, _, _, _, _) => {
                    h6010
                }
            };
            nat_mul(a6011, a6012)?
        };
        let v5: u64 = {
            let a6014 = v4;
            let a6015 = match v1.clone() {
                Adt2::C0(_, _, _, h6013, _, _, _, _, _, _, _) => {
                    h6013
                }
            };
            nat_mul(a6014, a6015)?
        };
        let a6028 = {
            let a6026 = {
                let a6017 = match v0.clone() {
                    Adt2::C0(_, h6016, _, _, _, _, _, _, _, _, _) => {
                        h6016
                    }
                };
                let a6018 = 1u64;
                nat_sub(a6017, a6018)
            };
            let a6027 = {
                let a6024 = f267(match v0.clone() {
                    Adt2::C0(_, _, h6019, _, _, _, _, _, _, _, _) => {
                        h6019
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6020, _, _, _, _, _, _, _, _, _) => {
                        h6020
                    }
                })?;
                let a6025 = f267(match v0 {
                    Adt2::C0(_, _, _, h6021, _, _, _, _, _, _, _) => {
                        h6021
                    }
                }, v2, {
                    let a6022 = v3;
                    let a6023 = v5;
                    nat_mul(a6022, a6023)?
                }, v4)?;
                nat_add(a6024, a6025)?
            };
            nat_add(a6026, a6027)?
        };
        let a6029 = f23(v1, 1u64)?;
        Ok(nat_lt(a6028, a6029))
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6030 = v0;
    let a6031 = 2147483648u64;
    if if if if nat_lt(a6030, a6031) {
        let a6032 = v1;
        let a6033 = 2147483648u64;
        nat_lt(a6032, a6033)
    } else {
        false
    } {
        let a6036 = {
            let a6034 = v0;
            let a6035 = v1;
            nat_mul(a6034, a6035)?
        };
        let a6037 = 2147483648u64;
        nat_lt(a6036, a6037)
    } else {
        false
    } {
        let a6042 = {
            let a6040 = {
                let a6038 = v0;
                let a6039 = 1u64;
                nat_sub(a6038, a6039)
            };
            let a6041 = v2;
            nat_mul(a6040, a6041)?
        };
        let a6043 = 2147483648u64;
        nat_lt(a6042, a6043)
    } else {
        false
    } {
        let a6048 = {
            let a6046 = {
                let a6044 = v1;
                let a6045 = 1u64;
                nat_sub(a6044, a6045)
            };
            let a6047 = v2;
            nat_mul(a6046, a6047)?
        };
        let a6049 = 2147483648u64;
        Ok(nat_lt(a6048, a6049))
    } else {
        Ok(false)
    }
}

pub fn f229(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6050 = v1;
    let a6051 = 13u64;
    if if nat_eq(a6050, a6051) {
        f48(match v0.clone() {
            Adt3::C0(h6052, _, _, _, _, _, _, _) => {
                h6052
            }
        }, match v0.clone() {
            Adt3::C0(_, h6053, _, _, _, _, _, _) => {
                h6053
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6054, _, _, _, _, _) => {
                h6054
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6055, _, _, _, _) => {
                h6055
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6061 = match v0.clone() {
                Adt3::C0(_, h6056, _, _, _, _, _, _) => {
                    h6056
                }
            };
            let a6062 = {
                let a6059 = match v0.clone() {
                    Adt3::C0(_, _, h6057, _, _, _, _, _) => {
                        h6057
                    }
                };
                let a6060 = match v0.clone() {
                    Adt3::C0(_, _, _, h6058, _, _, _, _) => {
                        h6058
                    }
                };
                nat_mul(a6059, a6060)?
            };
            nat_mul(a6061, a6062)?
        };
        let a6063 = v1;
        let a6064 = 13u64;
        if nat_eq(a6063, a6064) {
            let a6073 = 2u64;
            let a6074 = if f268(v0.clone(), v2) {
                let a6066 = v3;
                let a6067 = f241(match v0 {
                    Adt3::C0(h6065, _, _, _, _, _, _, _) => {
                        h6065
                    }
                }, false)?;
                nat_mul(a6066, a6067)?
            } else {
                let a6071 = v3;
                let a6072 = {
                    let a6069 = match v0 {
                        Adt3::C0(h6068, _, _, _, _, _, _, _) => {
                            h6068
                        }
                    };
                    let a6070 = 2u64;
                    nat_mul(a6069, a6070)?
                };
                nat_mul(a6071, a6072)?
            };
            nat_add(a6073, a6074)
        } else {
            let a6078 = v3;
            let a6079 = f241(match v0 {
                Adt3::C0(h6075, _, _, _, _, _, _, _) => {
                    h6075
                }
            }, {
                let a6076 = v1;
                let a6077 = 16u64;
                nat_eq(a6076, a6077)
            })?;
            nat_mul(a6078, a6079)
        }
    }
}

pub fn f230(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6080 = v0;
    let a6081 = 30u64;
    if if nat_eq(a6080, a6081) {
        let a6084 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6082, _, _, _, _) => {
                h6082
            }
        };
        let a6085 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6083, _, _, _, _, _) => {
                h6083
            }
        };
        nat_lt(a6084, a6085)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6086 = v3;
        match m6086 {
            None => {
                f269(v0, v1, match v2.clone() {
                    Adt2::C0(h6087, _, _, _, _, _, _, _, _, _, _) => {
                        h6087
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6088, _, _, _, _, _, _, _, _, _, _) => {
                        h6088
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6089, _, _, _, _, _, _, _, _, _) => {
                        h6089
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6090, _, _, _, _, _, _, _, _) => {
                        h6090
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6091, _, _, _, _, _, _, _) => {
                        h6091
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6092, _, _, _, _, _, _) => {
                        h6092
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6093, _, _, _, _, _, _, _, _, _) => {
                        h6093
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6094, _, _, _, _, _, _, _, _) => {
                        h6094
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6095, _, _, _, _, _, _, _) => {
                        h6095
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6096, _, _, _, _, _, _) => {
                        h6096
                    }
                })
            }
            Some(v5) => {
                f269(v0, v1, match v2.clone() {
                    Adt2::C0(h6097, _, _, _, _, _, _, _, _, _, _) => {
                        h6097
                    }
                }, match v5.clone() {
                    Adt2::C0(h6098, _, _, _, _, _, _, _, _, _, _) => {
                        h6098
                    }
                }, match v4.clone() {
                    Adt2::C0(h6099, _, _, _, _, _, _, _, _, _, _) => {
                        h6099
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6100, _, _, _, _, _, _, _, _, _) => {
                        h6100
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6101, _, _, _, _, _, _, _, _) => {
                        h6101
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6102, _, _, _, _, _, _, _) => {
                        h6102
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6103, _, _, _, _, _, _) => {
                        h6103
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6104, _, _, _, _, _, _, _, _, _) => {
                        h6104
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6105, _, _, _, _, _, _, _, _) => {
                        h6105
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6106, _, _, _, _, _, _, _) => {
                        h6106
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6107, _, _, _, _, _, _) => {
                        h6107
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6108, _, _, _, _, _, _, _, _, _) => {
                        h6108
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6109, _, _, _, _, _, _, _, _) => {
                        h6109
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6110, _, _, _, _, _, _, _) => {
                        h6110
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6111, _, _, _, _, _, _) => {
                        h6111
                    }
                })
            }
        }
    }
}

pub fn f231(v0: u64, v1: u64) -> u64 {
    let a6112 = v0;
    let a6113 = 23u64;
    if nat_eq(a6112, a6113) {
        let a6114 = v1;
        let a6115 = 0u64;
        if nat_eq(a6114, a6115) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f232(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6120 = {
        let a6118 = {
            let a6116 = v0;
            let a6117 = v1;
            nat_sub(a6116, a6117)
        };
        let a6119 = 1u64;
        nat_sub(a6118, a6119)
    };
    let a6121 = v2;
    nat_add(a6120, a6121)
}

pub fn f233(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f138({
        let a6122 = v0;
        let a6123 = v4;
        let a6124 = 0u64;
        nat_rem(a6122, a6123, a6124)
    }, {
        let a6125 = v1;
        let a6126 = v5;
        let a6127 = 0u64;
        nat_rem(a6125, a6126, a6127)
    }, {
        let a6128 = v2;
        let a6129 = v6;
        let a6130 = 0u64;
        nat_rem(a6128, a6129, a6130)
    }, {
        let a6131 = v3;
        let a6132 = v7;
        let a6133 = 0u64;
        nat_rem(a6131, a6132, a6133)
    }, v8, v9, v10, v11)
}

pub fn f234(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6134 = v1;
    let a6135 = 2147483648u64;
    if nat_lt(a6134, a6135) {
        let a6146 = {
            let a6144 = {
                let a6139 = {
                    let a6136 = v0;
                    let a6137 = v2;
                    let a6138 = 0u64;
                    nat_rem(a6136, a6137, a6138)
                };
                let a6140 = v2;
                nat_add(a6139, a6140)?
            };
            let a6145 = {
                let a6141 = v1;
                let a6142 = v2;
                let a6143 = 0u64;
                nat_rem(a6141, a6142, a6143)
            };
            nat_sub(a6144, a6145)
        };
        let a6147 = v2;
        let a6148 = 0u64;
        Ok(nat_rem(a6146, a6147, a6148))
    } else {
        let a6151 = {
            let a6149 = v0;
            let a6150 = f139(v1);
            nat_add(a6149, a6150)?
        };
        let a6152 = v2;
        let a6153 = 0u64;
        Ok(nat_rem(a6151, a6152, a6153))
    }
}

pub fn f235(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6156 = f140(v1);
    let a6157 = {
        let a6154 = v0;
        let a6155 = f139(v1);
        nat_add(a6154, a6155)?
    };
    if nat_le(a6156, a6157) {
        let a6158 = f236(v0, v1)?;
        let a6159 = v2;
        Ok(nat_lt(a6158, a6159))
    } else {
        Ok(false)
    }
}

pub fn f236(v0: u64, v1: u64) -> R<u64> {
    let a6162 = {
        let a6160 = v0;
        let a6161 = f139(v1);
        nat_add(a6160, a6161)?
    };
    let a6163 = f140(v1);
    Ok(nat_sub(a6162, a6163))
}

pub fn f237(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6164 = v0;
    let a6165 = v1;
    if nat_lt(a6164, a6165) {
        let a6166 = v1;
        let a6167 = v0;
        Ok(nat_sub(a6166, a6167))
    } else {
        let a6170 = v0;
        let a6171 = {
            let a6168 = v1;
            let a6169 = v2;
            nat_add(a6168, a6169)?
        };
        if nat_lt(a6170, a6171) {
            let a6172 = v0;
            let a6173 = v1;
            Ok(nat_sub(a6172, a6173))
        } else {
            let a6180 = {
                let a6176 = 2u64;
                let a6177 = {
                    let a6174 = v2;
                    let a6175 = 1u64;
                    nat_sub(a6174, a6175)
                };
                nat_mul(a6176, a6177)?
            };
            let a6181 = {
                let a6178 = v0;
                let a6179 = v1;
                nat_sub(a6178, a6179)
            };
            Ok(nat_sub(a6180, a6181))
        }
    }
}

pub fn f238(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6184 = {
        let a6182 = v2;
        let a6183 = f270(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a6182, a6183)?
    };
    let a6185 = f270(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6184, a6185))
}

pub fn f239(v0: u64, v1: u64, v2: u64) -> bool {
    let a6186 = v0;
    let a6187 = 0u64;
    if nat_eq(a6186, a6187) {
        let a6188 = v2;
        let a6189 = v1;
        nat_le(a6188, a6189)
    } else {
        let a6190 = v0;
        let a6191 = 1u64;
        if nat_eq(a6190, a6191) {
            let a6192 = v2;
            let a6193 = v1;
            nat_lt(a6192, a6193)
        } else {
            let a6194 = v0;
            let a6195 = 2u64;
            if nat_eq(a6194, a6195) {
                let a6196 = v1;
                let a6197 = v2;
                nat_le(a6196, a6197)
            } else {
                let a6198 = v0;
                let a6199 = 3u64;
                if nat_eq(a6198, a6199) {
                    let a6200 = v1;
                    let a6201 = v2;
                    nat_lt(a6200, a6201)
                } else {
                    let a6202 = v1;
                    let a6203 = v2;
                    nat_eq(a6202, a6203)
                }
            }
        }
    }
}

pub fn f240(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6204 = v0;
    let a6205 = 0u64;
    if nat_eq(a6204, a6205) {
        Ok(f253(0u64))
    } else {
        let a6208 = {
            let a6206 = v0;
            let a6207 = 1u64;
            nat_add(a6206, a6207)?
        };
        let a6209 = f229(v1.clone(), 13u64, v2)?;
        if nat_eq(a6208, a6209) {
            Ok(Adt9::C4({
                let a6210 = v2;
                let a6211 = 0u64;
                if nat_eq(a6210, a6211) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6212 = v0;
                let a6213 = 1u64;
                nat_sub(a6212, a6213)
            };
            if f268(v1.clone(), v2) {
                let v4: u64 = f241(match v1.clone() {
                    Adt3::C0(h6214, _, _, _, _, _, _, _) => {
                        h6214
                    }
                }, false)?;
                let v5: u64 = {
                    let a6215 = v3;
                    let a6216 = v4;
                    let a6217 = 0u64;
                    nat_quot(a6215, a6216, a6217)
                };
                let v6: u64 = {
                    let a6218 = v3;
                    let a6219 = v4;
                    let a6220 = 0u64;
                    nat_rem(a6218, a6219, a6220)
                };
                let a6223 = {
                    let a6221 = v6;
                    let a6222 = 1u64;
                    nat_add(a6221, a6222)?
                };
                let a6224 = v4;
                if nat_eq(a6223, a6224) {
                    Ok(Adt9::C4({
                        let a6225 = v2;
                        let a6226 = 0u64;
                        if nat_eq(a6225, a6226) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f242(v6, v5, v1, v2)
                }
            } else {
                f271({
                    let a6227 = v3;
                    let a6228 = 2u64;
                    let a6229 = 0u64;
                    nat_quot(a6227, a6228, a6229)
                }, {
                    let a6230 = v3;
                    let a6231 = 2u64;
                    let a6232 = 0u64;
                    nat_rem(a6230, a6231, a6232)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f241(v0: u64, v1: bool) -> R<u64> {
    let a6235 = {
        let a6233 = v0;
        let a6234 = 2u64;
        nat_mul(a6233, a6234)?
    };
    let a6236 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6235, a6236)
}

pub fn f242(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6237 = v0;
    let a6238 = 0u64;
    if nat_eq(a6237, a6238) {
        Ok(f253(1u64))
    } else {
        f271({
            let a6247 = {
                let a6240 = v1;
                let a6241 = match v2.clone() {
                    Adt3::C0(h6239, _, _, _, _, _, _, _) => {
                        h6239
                    }
                };
                nat_mul(a6240, a6241)?
            };
            let a6248 = {
                let a6244 = {
                    let a6242 = v0;
                    let a6243 = 1u64;
                    nat_sub(a6242, a6243)
                };
                let a6245 = 2u64;
                let a6246 = 0u64;
                nat_quot(a6244, a6245, a6246)
            };
            nat_add(a6247, a6248)?
        }, {
            let a6251 = {
                let a6249 = v0;
                let a6250 = 1u64;
                nat_sub(a6249, a6250)
            };
            let a6252 = 2u64;
            let a6253 = 0u64;
            nat_rem(a6251, a6252, a6253)
        }, v2, v3, 1u64)
    }
}

pub fn f243(v0: u64, v1: u64) -> R<u64> {
    let a6254 = v0;
    let a6255 = f93(v1)?;
    let a6256 = 0u64;
    Ok(nat_rem(a6254, a6255, a6256))
}

pub fn f244(v0: u64, v1: u64) -> R<u64> {
    let a6257 = 4u64;
    let a6258 = f272(v0, v1)?;
    nat_mul(a6257, a6258)
}

pub fn f245(v0: u64, v1: u64) -> R<u64> {
    let a6259 = 4u64;
    let a6260 = f273(v0, v1)?;
    nat_mul(a6259, a6260)
}

pub fn f246(v0: u64, v1: u64) -> R<bool> {
    let a6266 = {
        let a6263 = {
            let a6261 = f243(v0, v1)?;
            let a6262 = 2u64;
            nat_sub(a6261, a6262)
        };
        let a6264 = 2u64;
        let a6265 = 0u64;
        nat_rem(a6263, a6264, a6265)
    };
    let a6267 = 0u64;
    Ok(nat_eq(a6266, a6267))
}

pub fn f247(v0: u64) -> u64 {
    let m6268 = f210(v0);
    match m6268 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6269, _) => {
                    h6269
                }
            }
        }
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6270 = f210(v0);
    match m6270 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f217(v0) {
                let a6271 = v1;
                let a6272 = v7;
                nat_lt(a6271, a6272)
            } else {
                false
            } {
                f177(v8, v2)
            } else {
                false
            } {
                let a6273 = v3;
                let a6274 = f274(v4, v5, v6)?;
                nat_lt(a6273, a6274)
            } else {
                false
            } {
                let a6279 = {
                    let a6276 = v7;
                    let a6277 = match v13.clone() {
                        Adt19::C0(_, h6275, _) => {
                            h6275
                        }
                    };
                    let a6278 = 0u64;
                    nat_rem(a6276, a6277, a6278)
                };
                let a6280 = 0u64;
                nat_eq(a6279, a6280)
            } else {
                false
            } {
                Ok(Some({
                    let a6309 = {
                        let a6299 = {
                            let a6289 = {
                                let a6285 = {
                                    let a6282 = v1;
                                    let a6283 = match v13 {
                                        Adt19::C0(_, h6281, _) => {
                                            h6281
                                        }
                                    };
                                    let a6284 = 0u64;
                                    nat_quot(a6282, a6283, a6284)
                                };
                                let a6286 = v9;
                                nat_mul(a6285, a6286)?
                            };
                            let a6290 = {
                                let a6287 = v2;
                                let a6288 = v10;
                                nat_mul(a6287, a6288)?
                            };
                            nat_add(a6289, a6290)?
                        };
                        let a6300 = {
                            let a6297 = {
                                let a6294 = {
                                    let a6291 = v3;
                                    let a6292 = v4;
                                    let a6293 = 0u64;
                                    nat_quot(a6291, a6292, a6293)
                                };
                                let a6295 = v5;
                                let a6296 = 0u64;
                                nat_rem(a6294, a6295, a6296)
                            };
                            let a6298 = v11;
                            nat_mul(a6297, a6298)?
                        };
                        nat_add(a6299, a6300)?
                    };
                    let a6310 = {
                        let a6307 = {
                            let a6304 = {
                                let a6301 = v3;
                                let a6302 = v4;
                                let a6303 = 0u64;
                                nat_quot(a6301, a6302, a6303)
                            };
                            let a6305 = v5;
                            let a6306 = 0u64;
                            nat_quot(a6304, a6305, a6306)
                        };
                        let a6308 = v12;
                        nat_mul(a6307, a6308)?
                    };
                    nat_add(a6309, a6310)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f249(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f275(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f250(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f276(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f251(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6311 = v0;
    let a6312 = 64u64;
    if nat_eq(a6311, a6312) {
        f277(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6313 = v0;
        let a6314 = 65u64;
        if nat_eq(a6313, a6314) {
            f278(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f252(v0: u64) -> u64 {
    let a6315 = v0;
    let a6316 = 30u64;
    if nat_eq(a6315, a6316) {
        4u64
    } else {
        3u64
    }
}

pub fn f253(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f254(v0: u64) -> bool {
    let a6317 = 0u64;
    let a6318 = v0;
    if nat_lt(a6317, a6318) {
        let a6319 = v0;
        let a6320 = 2139095040u64;
        nat_le(a6319, a6320)
    } else {
        false
    }
}

pub fn f255(v0: u64) -> u64 {
    let a6321 = v0;
    let a6322 = 40u64;
    if nat_lt(a6321, a6322) {
        let a6323 = v0;
        let a6324 = 20u64;
        if nat_lt(a6323, a6324) {
            let a6325 = v0;
            let a6326 = 10u64;
            if nat_lt(a6325, a6326) {
                let a6327 = v0;
                let a6328 = 5u64;
                if nat_lt(a6327, a6328) {
                    let a6329 = v0;
                    let a6330 = 2u64;
                    if nat_lt(a6329, a6330) {
                        let a6331 = v0;
                        let a6332 = 1u64;
                        if nat_lt(a6331, a6332) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6333 = v0;
                        let a6334 = 3u64;
                        if nat_lt(a6333, a6334) {
                            4u64
                        } else {
                            let a6335 = v0;
                            let a6336 = 4u64;
                            if nat_lt(a6335, a6336) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6337 = v0;
                    let a6338 = 7u64;
                    if nat_lt(a6337, a6338) {
                        let a6339 = v0;
                        let a6340 = 6u64;
                        if nat_lt(a6339, a6340) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6341 = v0;
                        let a6342 = 8u64;
                        if nat_lt(a6341, a6342) {
                            128u64
                        } else {
                            let a6343 = v0;
                            let a6344 = 9u64;
                            if nat_lt(a6343, a6344) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6345 = v0;
                let a6346 = 15u64;
                if nat_lt(a6345, a6346) {
                    let a6347 = v0;
                    let a6348 = 12u64;
                    if nat_lt(a6347, a6348) {
                        let a6349 = v0;
                        let a6350 = 11u64;
                        if nat_lt(a6349, a6350) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6351 = v0;
                        let a6352 = 13u64;
                        if nat_lt(a6351, a6352) {
                            4096u64
                        } else {
                            let a6353 = v0;
                            let a6354 = 14u64;
                            if nat_lt(a6353, a6354) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6355 = v0;
                    let a6356 = 17u64;
                    if nat_lt(a6355, a6356) {
                        let a6357 = v0;
                        let a6358 = 16u64;
                        if nat_lt(a6357, a6358) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6359 = v0;
                        let a6360 = 18u64;
                        if nat_lt(a6359, a6360) {
                            131072u64
                        } else {
                            let a6361 = v0;
                            let a6362 = 19u64;
                            if nat_lt(a6361, a6362) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6363 = v0;
            let a6364 = 30u64;
            if nat_lt(a6363, a6364) {
                let a6365 = v0;
                let a6366 = 25u64;
                if nat_lt(a6365, a6366) {
                    let a6367 = v0;
                    let a6368 = 22u64;
                    if nat_lt(a6367, a6368) {
                        let a6369 = v0;
                        let a6370 = 21u64;
                        if nat_lt(a6369, a6370) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6371 = v0;
                        let a6372 = 23u64;
                        if nat_lt(a6371, a6372) {
                            4194304u64
                        } else {
                            let a6373 = v0;
                            let a6374 = 24u64;
                            if nat_lt(a6373, a6374) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6375 = v0;
                    let a6376 = 27u64;
                    if nat_lt(a6375, a6376) {
                        let a6377 = v0;
                        let a6378 = 26u64;
                        if nat_lt(a6377, a6378) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6379 = v0;
                        let a6380 = 28u64;
                        if nat_lt(a6379, a6380) {
                            134217728u64
                        } else {
                            let a6381 = v0;
                            let a6382 = 29u64;
                            if nat_lt(a6381, a6382) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6383 = v0;
                let a6384 = 35u64;
                if nat_lt(a6383, a6384) {
                    let a6385 = v0;
                    let a6386 = 32u64;
                    if nat_lt(a6385, a6386) {
                        let a6387 = v0;
                        let a6388 = 31u64;
                        if nat_lt(a6387, a6388) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6389 = v0;
                        let a6390 = 33u64;
                        if nat_lt(a6389, a6390) {
                            4294967296u64
                        } else {
                            let a6391 = v0;
                            let a6392 = 34u64;
                            if nat_lt(a6391, a6392) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6393 = v0;
                    let a6394 = 37u64;
                    if nat_lt(a6393, a6394) {
                        let a6395 = v0;
                        let a6396 = 36u64;
                        if nat_lt(a6395, a6396) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6397 = v0;
                        let a6398 = 38u64;
                        if nat_lt(a6397, a6398) {
                            137438953472u64
                        } else {
                            let a6399 = v0;
                            let a6400 = 39u64;
                            if nat_lt(a6399, a6400) {
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

pub fn f256(v0: u64, v1: u64) -> bool {
    let a6401 = v0;
    let a6402 = 1u64;
    if nat_le(a6401, a6402) {
        true
    } else {
        let a6406 = {
            let a6403 = v1;
            let a6404 = 4u64;
            let a6405 = 0u64;
            nat_rem(a6403, a6404, a6405)
        };
        let a6407 = 0u64;
        nat_eq(a6406, a6407)
    }
}

pub fn f257(v0: u64, v1: u64) -> bool {
    let a6408 = v0;
    let a6409 = 1u64;
    if nat_le(a6408, a6409) {
        true
    } else {
        let a6410 = 4u64;
        let a6411 = v1;
        nat_le(a6410, a6411)
    }
}

pub fn f258(v0: u64, v1: u64) -> R<u64> {
    let a6412 = v0;
    let a6413 = 0u64;
    if nat_eq(a6412, a6413) {
        Ok(0u64)
    } else {
        let a6418 = 4u64;
        let a6419 = {
            let a6416 = {
                let a6414 = v0;
                let a6415 = 1u64;
                nat_sub(a6414, a6415)
            };
            let a6417 = v1;
            nat_mul(a6416, a6417)?
        };
        nat_add(a6418, a6419)
    }
}

pub fn f259(v0: bool, v1: u64) -> R<u64> {
    let a6424 = {
        let a6422 = {
            let a6420 = f279(v0, v1)?;
            let a6421 = f280(v0, v1)?;
            nat_add(a6420, a6421)?
        };
        let a6423 = f281(v0);
        nat_add(a6422, a6423)?
    };
    let a6425 = v1;
    nat_add(a6424, a6425)
}

pub fn f260(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f282(v0, v1)?;
    let v3: u64 = {
        let a6430 = {
            let a6428 = {
                let a6426 = v1;
                let a6427 = 1u64;
                nat_sub(a6426, a6427)
            };
            let a6429 = v2;
            nat_mul(a6428, a6429)?
        };
        let a6431 = v0;
        nat_sub(a6430, a6431)
    };
    let a6432 = v2;
    let a6433 = v3;
    if nat_lt(a6432, a6433) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6434 = v4;
    let a6435 = 0u64;
    if nat_eq(a6434, a6435) {
        Ok(0u64)
    } else {
        let a6442 = 2u64;
        let a6443 = {
            let a6440 = {
                let a6436 = v0;
                let a6437 = v1;
                nat_mul(a6436, a6437)?
            };
            let a6441 = {
                let a6438 = v2;
                let a6439 = v3;
                nat_mul(a6438, a6439)?
            };
            nat_mul(a6440, a6441)?
        };
        nat_mul(a6442, a6443)
    }
}

pub fn f262(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6448 = v3;
    let a6449 = {
        let a6446 = {
            let a6444 = f283(v2, v4)?;
            let a6445 = f284(v0, v1, f282(v2, v4)?)?;
            nat_mul(a6444, a6445)?
        };
        let a6447 = f284(v0, v1, f285(v2, v4)?)?;
        nat_add(a6446, a6447)?
    };
    nat_mul(a6448, a6449)
}

pub fn f263(v0: u64) -> R<u64> {
    let a6452 = 13u64;
    let a6453 = {
        let a6450 = 9u64;
        let a6451 = v0;
        nat_mul(a6450, a6451)?
    };
    nat_add(a6452, a6453)
}

pub fn f264(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f286(v0, v1, v6, v7, v8) {
        let a6454 = v5;
        let a6455 = 26u64;
        nat_eq(a6454, a6455)
    } else {
        false
    } {
        let a6456 = v3;
        let a6457 = v7;
        nat_eq(a6456, a6457)
    } else {
        false
    } {
        let a6458 = v4;
        let a6459 = v8;
        nat_eq(a6458, a6459)
    } else {
        false
    } {
        let a6460 = v9;
        let a6461 = 1u64;
        nat_eq(a6460, a6461)
    } else {
        false
    } {
        let a6462 = v10;
        let a6463 = f287(v0);
        nat_eq(a6462, a6463)
    } else {
        false
    } {
        let a6464 = v11;
        let a6465 = v1;
        nat_eq(a6464, a6465)
    } else {
        false
    } {
        let a6466 = v12;
        let a6467 = v6;
        nat_eq(a6466, a6467)
    } else {
        false
    } {
        let a6468 = v13;
        let a6469 = v7;
        nat_eq(a6468, a6469)
    } else {
        false
    } {
        let a6470 = v14;
        let a6471 = v8;
        nat_eq(a6470, a6471)
    } else {
        false
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6472 = v0;
    let a6473 = 0u64;
    if if if nat_eq(a6472, a6473) {
        true
    } else {
        let a6474 = v1;
        let a6475 = 0u64;
        nat_eq(a6474, a6475)
    } {
        true
    } else {
        let a6476 = v2;
        let a6477 = 0u64;
        nat_eq(a6476, a6477)
    } {
        true
    } else {
        let a6478 = v3;
        let a6479 = 0u64;
        nat_eq(a6478, a6479)
    }
}

pub fn f266(v0: Adt3, v1: bool) -> R<u64> {
    f270(v0.clone(), {
        let a6481 = match v0.clone() {
            Adt3::C0(h6480, _, _, _, _, _, _, _) => {
                h6480
            }
        };
        let a6482 = 1u64;
        nat_sub(a6481, a6482)
    }, {
        let a6484 = match v0.clone() {
            Adt3::C0(_, h6483, _, _, _, _, _, _) => {
                h6483
            }
        };
        let a6485 = 1u64;
        nat_sub(a6484, a6485)
    }, {
        let a6487 = match v0.clone() {
            Adt3::C0(_, _, h6486, _, _, _, _, _) => {
                h6486
            }
        };
        let a6488 = 1u64;
        nat_sub(a6487, a6488)
    }, {
        let a6490 = match v0 {
            Adt3::C0(_, _, _, h6489, _, _, _, _) => {
                h6489
            }
        };
        let a6491 = 1u64;
        nat_sub(a6490, a6491)
    }, v1)
}

pub fn f267(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6492 = v0;
    let a6493 = 0u64;
    if nat_eq(a6492, a6493) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6494 = v0;
            let a6495 = 1u64;
            nat_sub(a6494, a6495)
        };
        let v5: u64 = {
            let a6496 = v4;
            let a6497 = v1;
            let a6498 = 0u64;
            nat_quot(a6496, a6497, a6498)
        };
        let v6: u64 = {
            let a6506 = {
                let a6499 = v5;
                let a6500 = v2;
                nat_mul(a6499, a6500)?
            };
            let a6507 = {
                let a6504 = {
                    let a6501 = v4;
                    let a6502 = v1;
                    let a6503 = 0u64;
                    nat_rem(a6501, a6502, a6503)
                };
                let a6505 = v3;
                nat_mul(a6504, a6505)?
            };
            nat_add(a6506, a6507)?
        };
        let a6508 = v5;
        let a6509 = 0u64;
        if nat_eq(a6508, a6509) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6518 = {
                    let a6512 = {
                        let a6510 = v5;
                        let a6511 = 1u64;
                        nat_sub(a6510, a6511)
                    };
                    let a6513 = v2;
                    nat_mul(a6512, a6513)?
                };
                let a6519 = {
                    let a6516 = {
                        let a6514 = v1;
                        let a6515 = 1u64;
                        nat_sub(a6514, a6515)
                    };
                    let a6517 = v3;
                    nat_mul(a6516, a6517)?
                };
                nat_add(a6518, a6519)?
            };
            let a6520 = v6;
            let a6521 = v7;
            if nat_lt(a6520, a6521) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f268(v0: Adt3, v1: u64) -> bool {
    let a6522 = v1;
    let a6523 = 0u64;
    if !nat_eq(a6522, a6523) {
        true
    } else {
        let a6525 = match v0 {
            Adt3::C0(_, _, _, _, h6524, _, _, _) => {
                h6524
            }
        };
        let a6526 = 4u64;
        nat_eq(a6525, a6526)
    }
}

pub fn f269(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6527 = f288(v0, v1);
    match m6527 {
        None => {
            if v5 {
                let m6528 = f289(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6528 {
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
                let a6529 = 4u64;
                let a6530 = v28;
                nat_le(a6529, a6530)
            };
            let a6531 = v2;
            let a6532 = 0u64;
            if if if if if nat_eq(a6531, a6532) {
                let a6533 = v4;
                let a6534 = 0u64;
                nat_eq(a6533, a6534)
            } else {
                false
            } {
                let a6535 = v6;
                let a6536 = v14;
                if nat_eq(a6535, a6536) {
                    let a6537 = v7;
                    let a6538 = v15;
                    nat_eq(a6537, a6538)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6539 = v8;
                let a6540 = v16;
                if nat_eq(a6539, a6540) {
                    let a6541 = v9;
                    let a6542 = v17;
                    nat_eq(a6541, a6542)
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
                        let a6543 = v3;
                        let a6544 = 0u64;
                        nat_eq(a6543, a6544)
                    } else {
                        false
                    } {
                        f220(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6549 = {
                        let a6545 = v6;
                        let a6546 = v7;
                        nat_mul(a6545, a6546)?
                    };
                    let a6550 = {
                        let a6547 = v8;
                        let a6548 = v9;
                        nat_mul(a6547, a6548)?
                    };
                    nat_mul(a6549, a6550)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f270(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6571 = {
        let a6565 = {
            let a6559 = {
                let a6553 = v1;
                let a6554 = if v5 {
                    f139(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6551, _, _, _) => {
                            h6551
                        }
                    })
                } else {
                    f140(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6552, _, _, _) => {
                            h6552
                        }
                    })
                };
                nat_mul(a6553, a6554)?
            };
            let a6560 = {
                let a6557 = v2;
                let a6558 = if v5 {
                    f139(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6555, _, _) => {
                            h6555
                        }
                    })
                } else {
                    f140(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6556, _, _) => {
                            h6556
                        }
                    })
                };
                nat_mul(a6557, a6558)?
            };
            nat_add(a6559, a6560)?
        };
        let a6566 = {
            let a6563 = v3;
            let a6564 = if v5 {
                f139(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6561, _) => {
                        h6561
                    }
                })
            } else {
                f140(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6562, _) => {
                        h6562
                    }
                })
            };
            nat_mul(a6563, a6564)?
        };
        nat_add(a6565, a6566)?
    };
    let a6572 = {
        let a6569 = v4;
        let a6570 = if v5 {
            f139(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6567) => {
                    h6567
                }
            })
        } else {
            f140(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6568) => {
                    h6568
                }
            })
        };
        nat_mul(a6569, a6570)?
    };
    nat_add(a6571, a6572)
}

pub fn f271(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6573 = v1;
    let a6574 = 0u64;
    if nat_eq(a6573, a6574) {
        Ok(Adt9::C4({
            let a6575 = v3;
            let a6576 = 0u64;
            if nat_eq(a6575, a6576) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f90(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6577 = v3;
            let a6578 = 0u64;
            if nat_eq(a6577, a6578) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f272(v0: u64, v1: u64) -> R<u64> {
    let a6579 = v0;
    let a6580 = f93(v1)?;
    let a6581 = 0u64;
    Ok(nat_quot(a6579, a6580, a6581))
}

pub fn f273(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f272(v0, v1)?;
    let v3: u64 = {
        let a6584 = {
            let a6582 = f243(v0, v1)?;
            let a6583 = 2u64;
            nat_sub(a6582, a6583)
        };
        let a6585 = 2u64;
        let a6586 = 0u64;
        nat_quot(a6584, a6585, a6586)
    };
    let v4: u64 = {
        let a6587 = v2;
        let a6588 = v1;
        nat_mul(a6587, a6588)?
    };
    if f246(v0, v1)? {
        let a6589 = v4;
        let a6590 = v3;
        nat_add(a6589, a6590)
    } else {
        let a6602 = {
            let a6598 = v4;
            let a6599 = {
                let a6596 = {
                    let a6591 = v1;
                    let a6592 = 1u64;
                    nat_sub(a6591, a6592)
                };
                let a6597 = {
                    let a6593 = v2;
                    let a6594 = v1;
                    let a6595 = 0u64;
                    nat_rem(a6593, a6594, a6595)
                };
                nat_mul(a6596, a6597)?
            };
            nat_sub(a6598, a6599)
        };
        let a6603 = {
            let a6600 = v3;
            let a6601 = v1;
            nat_mul(a6600, a6601)?
        };
        nat_add(a6602, a6603)
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6604 = v0;
    let a6605 = 0u64;
    if if if nat_eq(a6604, a6605) {
        true
    } else {
        let a6606 = v1;
        let a6607 = 0u64;
        nat_eq(a6606, a6607)
    } {
        true
    } else {
        let a6608 = v2;
        let a6609 = 0u64;
        nat_eq(a6608, a6609)
    } {
        Ok(0u64)
    } else {
        let a6612 = {
            let a6610 = v0;
            let a6611 = v1;
            nat_mul(a6610, a6611)?
        };
        let a6613 = v2;
        nat_mul(a6612, a6613)
    }
}

pub fn f275(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f102(match v1.clone() {
        Adt20::C0(h6614, _, _, _, _, _, _, _) => {
            h6614
        }
    })?;
    let v6: u64 = {
        let a6620 = match v1.clone() {
            Adt20::C0(_, h6615, _, _, _, _, _, _) => {
                h6615
            }
        };
        let a6621 = {
            let a6618 = match v1.clone() {
                Adt20::C0(_, _, h6616, _, _, _, _, _) => {
                    h6616
                }
            };
            let a6619 = match v1.clone() {
                Adt20::C0(_, _, _, h6617, _, _, _, _) => {
                    h6617
                }
            };
            nat_mul(a6618, a6619)?
        };
        nat_mul(a6620, a6621)?
    };
    let a6623 = v0;
    let a6624 = f53(match v1.clone() {
        Adt20::C0(h6622, _, _, _, _, _, _, _) => {
            h6622
        }
    }, v6)?;
    if nat_lt(a6623, a6624) {
        f290({
            let a6625 = v0;
            let a6626 = v5;
            let a6627 = 0u64;
            nat_rem(a6625, a6626, a6627)
        }, {
            let a6628 = v0;
            let a6629 = v5;
            let a6630 = 0u64;
            nat_quot(a6628, a6629, a6630)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f276(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6631 = v6;
        let a6632 = 6u64;
        nat_sub(a6631, a6632)
    };
    let v9: Adt20 = f291(v3);
    let a6634 = v0;
    let a6635 = f207(match v1.clone() {
        Adt20::C0(h6633, _, _, _, _, _, _, _) => {
            h6633
        }
    }, v5, v8)?;
    if nat_lt(a6634, a6635) {
        let v10: u64 = f99(match v1.clone() {
            Adt20::C0(h6636, _, _, _, _, _, _, _) => {
                h6636
            }
        }, v5, v8)?;
        let a6637 = v0;
        let a6638 = v10;
        if nat_lt(a6637, a6638) {
            f292(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6639 = v0;
                let a6640 = v10;
                nat_sub(a6639, a6640)
            };
            let v12: u64 = f263(match v1.clone() {
                Adt20::C0(h6641, _, _, _, _, _, _, _) => {
                    h6641
                }
            })?;
            f293({
                let a6642 = v11;
                let a6643 = v12;
                let a6644 = 0u64;
                nat_rem(a6642, a6643, a6644)
            }, {
                let a6645 = v11;
                let a6646 = v12;
                let a6647 = 0u64;
                nat_quot(a6645, a6646, a6647)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f277(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6648 = v5;
        let a6649 = 6u64;
        nat_sub(a6648, a6649)
    };
    let v8: bool = f201(match v1.clone() {
        Adt20::C0(_, _, _, _, h6650, _, _, _) => {
            h6650
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6651, _, _, _) => {
            h6651
        }
    });
    let a6653 = v0;
    let a6654 = f200(v8, match v1.clone() {
        Adt20::C0(h6652, _, _, _, _, _, _, _) => {
            h6652
        }
    }, v4, v7)?;
    if nat_lt(a6653, a6654) {
        let v9: u64 = f99(match v1.clone() {
            Adt20::C0(h6655, _, _, _, _, _, _, _) => {
                h6655
            }
        }, v4, v7)?;
        let a6656 = v0;
        let a6657 = v9;
        if nat_lt(a6656, a6657) {
            f292(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6658 = v0;
                let a6659 = v9;
                nat_sub(a6658, a6659)
            };
            let v11: u64 = f259(v8, match v1.clone() {
                Adt20::C0(h6660, _, _, _, _, _, _, _) => {
                    h6660
                }
            })?;
            f294({
                let a6661 = v10;
                let a6662 = v11;
                let a6663 = 0u64;
                nat_rem(a6661, a6662, a6663)
            }, {
                let a6664 = v10;
                let a6665 = v11;
                let a6666 = 0u64;
                nat_quot(a6664, a6665, a6666)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f278(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6667 = v5;
        let a6668 = 6u64;
        nat_sub(a6667, a6668)
    };
    let v8: Adt20 = f291(v2);
    let v9: u64 = f261(match v1.clone() {
        Adt20::C0(h6669, _, _, _, _, _, _, _) => {
            h6669
        }
    }, match v1.clone() {
        Adt20::C0(_, h6670, _, _, _, _, _, _) => {
            h6670
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6671, _, _, _, _, _) => {
            h6671
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6672, _, _, _, _) => {
            h6672
        }
    }, v7)?;
    let a6677 = v0;
    let a6678 = f206(match v1.clone() {
        Adt20::C0(h6673, _, _, _, _, _, _, _) => {
            h6673
        }
    }, match v1.clone() {
        Adt20::C0(_, h6674, _, _, _, _, _, _) => {
            h6674
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6675, _, _, _, _, _) => {
            h6675
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6676, _, _, _, _) => {
            h6676
        }
    }, v4, v7)?;
    if nat_lt(a6677, a6678) {
        let a6679 = v0;
        let a6680 = v9;
        if nat_lt(a6679, a6680) {
            f292(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f295({
                let a6681 = v0;
                let a6682 = v9;
                nat_sub(a6681, a6682)
            }, match v1.clone() {
                Adt20::C0(h6683, _, _, _, _, _, _, _) => {
                    h6683
                }
            }, match v1.clone() {
                Adt20::C0(_, h6684, _, _, _, _, _, _) => {
                    h6684
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6685, _, _, _, _, _) => {
                    h6685
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6686, _, _, _, _) => {
                    h6686
                }
            }, v4)?;
            f296(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f279(v0: bool, v1: u64) -> R<u64> {
    let a6689 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6690 = {
        let a6687 = f297(v0);
        let a6688 = v1;
        nat_mul(a6687, a6688)?
    };
    nat_add(a6689, a6690)
}

pub fn f280(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6698 = {
            let a6691 = 13u64;
            let a6692 = f298(v1);
            nat_mul(a6691, a6692)?
        };
        let a6699 = {
            let a6696 = 4u64;
            let a6697 = {
                let a6693 = v1;
                let a6694 = 4u64;
                let a6695 = 0u64;
                nat_rem(a6693, a6694, a6695)
            };
            nat_mul(a6696, a6697)?
        };
        nat_add(a6698, a6699)
    } else {
        let a6700 = 3u64;
        let a6701 = v1;
        nat_mul(a6700, a6701)
    }
}

pub fn f281(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f282(v0: u64, v1: u64) -> R<u64> {
    let a6706 = {
        let a6704 = {
            let a6702 = v0;
            let a6703 = v1;
            nat_add(a6702, a6703)?
        };
        let a6705 = 1u64;
        nat_sub(a6704, a6705)
    };
    let a6707 = v1;
    let a6708 = 0u64;
    Ok(nat_quot(a6706, a6707, a6708))
}

pub fn f283(v0: u64, v1: u64) -> R<u64> {
    let a6709 = v0;
    let a6710 = f282(v0, v1)?;
    let a6711 = 0u64;
    Ok(nat_quot(a6709, a6710, a6711))
}

pub fn f284(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6712 = v2;
    let a6713 = 0u64;
    if nat_eq(a6712, a6713) {
        Ok(0u64)
    } else {
        let a6718 = 11u64;
        let a6719 = {
            let a6716 = {
                let a6714 = v1;
                let a6715 = v2;
                nat_mul(a6714, a6715)?
            };
            let a6717 = f299(v0)?;
            nat_mul(a6716, a6717)?
        };
        nat_add(a6718, a6719)
    }
}

pub fn f285(v0: u64, v1: u64) -> R<u64> {
    let a6720 = v0;
    let a6721 = f282(v0, v1)?;
    let a6722 = 0u64;
    Ok(nat_rem(a6720, a6721, a6722))
}

pub fn f286(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f265(v1, v2, v3, v4) {
        f300(v0)
    } else {
        f217(v0)
    }
}

pub fn f287(v0: u64) -> u64 {
    let a6723 = v0;
    let a6724 = 26u64;
    if nat_eq(a6723, a6724) {
        26u64
    } else {
        0u64
    }
}

pub fn f288(v0: u64, v1: u64) -> Option<u64> {
    let a6725 = v0;
    let a6726 = 2u64;
    if nat_eq(a6725, a6726) {
        Some(0u64)
    } else {
        let a6727 = v0;
        let a6728 = 5u64;
        if nat_eq(a6727, a6728) {
            Some(1u64)
        } else {
            let a6729 = v0;
            let a6730 = 6u64;
            if nat_eq(a6729, a6730) {
                Some(2u64)
            } else {
                let a6731 = v0;
                let a6732 = 7u64;
                if nat_eq(a6731, a6732) {
                    Some(3u64)
                } else {
                    let a6733 = v0;
                    let a6734 = 9u64;
                    if nat_eq(a6733, a6734) {
                        Some(4u64)
                    } else {
                        let a6735 = v0;
                        let a6736 = 8u64;
                        if nat_eq(a6735, a6736) {
                            Some(5u64)
                        } else {
                            let a6737 = v0;
                            let a6738 = 89u64;
                            if nat_eq(a6737, a6738) {
                                let a6739 = v1;
                                let a6740 = 2u64;
                                if nat_eq(a6739, a6740) {
                                    Some(6u64)
                                } else {
                                    let a6741 = v1;
                                    let a6742 = 0u64;
                                    if nat_eq(a6741, a6742) {
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

pub fn f289(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6743 = v0;
    let a6744 = 30u64;
    if nat_eq(a6743, a6744) {
        let a6745 = v2;
        let a6746 = 0u64;
        if if if nat_eq(a6745, a6746) {
            true
        } else {
            let a6747 = v2;
            let a6748 = 1u64;
            nat_eq(a6747, a6748)
        } {
            let a6749 = v1;
            let a6750 = 0u64;
            if nat_eq(a6749, a6750) {
                true
            } else {
                f301(v1, v4)
            }
        } else {
            false
        } {
            f302(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f303(v1) {
            f303(v2)
        } else {
            false
        } {
            let a6751 = v1;
            let a6752 = 0u64;
            if if nat_eq(a6751, a6752) {
                true
            } else {
                let a6753 = v2;
                let a6754 = 0u64;
                nat_eq(a6753, a6754)
            } {
                true
            } else {
                let a6755 = v1;
                let a6756 = v2;
                nat_eq(a6755, a6756)
            }
        } else {
            false
        } {
            true
        } else {
            let a6757 = v1;
            let a6758 = 43u64;
            if nat_eq(a6757, a6758) {
                f303(v2)
            } else {
                false
            }
        } {
            f302(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f290(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6759, _, _, _, _, _, _, _) => {
            h6759
        }
    };
    let v7: u64 = {
        let a6762 = 1u64;
        let a6763 = {
            let a6760 = 3u64;
            let a6761 = v6;
            nat_mul(a6760, a6761)?
        };
        nat_add(a6762, a6763)?
    };
    let a6764 = v0;
    let a6765 = 0u64;
    if nat_eq(a6764, a6765) {
        Ok(f304(0u64))
    } else {
        let a6766 = v0;
        let a6767 = v7;
        if nat_lt(a6766, a6767) {
            let v8: u64 = {
                let a6768 = v0;
                let a6769 = 1u64;
                nat_sub(a6768, a6769)
            };
            let v9: u64 = {
                let a6775 = {
                    let a6770 = v1;
                    let a6771 = v6;
                    nat_mul(a6770, a6771)?
                };
                let a6776 = {
                    let a6772 = v8;
                    let a6773 = 3u64;
                    let a6774 = 0u64;
                    nat_quot(a6772, a6773, a6774)
                };
                nat_add(a6775, a6776)?
            };
            let v10: u64 = f305(v9, v2)?;
            let v11: u64 = {
                let a6777 = v8;
                let a6778 = 3u64;
                let a6779 = 0u64;
                nat_rem(a6777, a6778, a6779)
            };
            let a6780 = v11;
            let a6781 = 0u64;
            if nat_eq(a6780, a6781) {
                Ok(f306(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6782 = v11;
                let a6783 = 1u64;
                if nat_eq(a6782, a6783) {
                    Ok(f306(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f306(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6784 = v0;
            let a6785 = v7;
            if nat_eq(a6784, a6785) {
                Ok(f306(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6788 = v0;
                let a6789 = {
                    let a6786 = v7;
                    let a6787 = 1u64;
                    nat_add(a6786, a6787)?
                };
                if nat_eq(a6788, a6789) {
                    Ok(f306(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6792 = v0;
                    let a6793 = {
                        let a6790 = v7;
                        let a6791 = 2u64;
                        nat_add(a6790, a6791)?
                    };
                    if nat_eq(a6792, a6793) {
                        Ok(f306(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6796 = v0;
                        let a6797 = {
                            let a6794 = v7;
                            let a6795 = 3u64;
                            nat_add(a6794, a6795)?
                        };
                        if nat_eq(a6796, a6797) {
                            let a6798 = v5;
                            let a6799 = 1u64;
                            if if nat_eq(a6798, a6799) {
                                true
                            } else {
                                let a6800 = v5;
                                let a6801 = 2u64;
                                nat_eq(a6800, a6801)
                            } {
                                Ok(f306(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f306(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6808 = {
                                    let a6802 = v1;
                                    let a6803 = v6;
                                    nat_mul(a6802, a6803)?
                                };
                                let a6809 = {
                                    let a6806 = v0;
                                    let a6807 = {
                                        let a6804 = v7;
                                        let a6805 = 4u64;
                                        nat_add(a6804, a6805)?
                                    };
                                    nat_sub(a6806, a6807)
                                };
                                nat_add(a6808, a6809)?
                            };
                            Ok(f306(10u64, 0u64, f305(v12, v2)?, 1u64, 2u64, 3u64, f305(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f291(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6810, _, _, _, _, _, _, _) => {
            h6810
        }
    }, match v0.clone() {
        Adt20::C0(_, h6811, _, _, _, _, _, _) => {
            h6811
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6812, _, _, _, _, _) => {
            h6812
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6813, _, _, _, _) => {
            h6813
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6814, _, _) => {
            h6814
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6815, _) => {
            h6815
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6816) => {
            h6816
        }
    })
}

pub fn f292(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6823 = v3;
        let a6824 = {
            let a6820 = f305({
                let a6817 = v0;
                let a6818 = 2u64;
                let a6819 = 0u64;
                nat_quot(a6817, a6818, a6819)
            }, v1)?;
            let a6821 = v2;
            let a6822 = 0u64;
            nat_quot(a6820, a6821, a6822)
        };
        nat_add(a6823, a6824)?
    };
    let a6828 = {
        let a6825 = v0;
        let a6826 = 2u64;
        let a6827 = 0u64;
        nat_rem(a6825, a6826, a6827)
    };
    let a6829 = 0u64;
    if nat_eq(a6828, a6829) {
        Ok(f306(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6830 = v4;
        let a6831 = 0u64;
        if nat_eq(a6830, a6831) {
            Ok(f306(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f293(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f307(match v2.clone() {
        Adt20::C0(h6832, _, _, _, _, _, _, _) => {
            h6832
        }
    })?;
    let v7: u64 = f308(match v2.clone() {
        Adt20::C0(h6833, _, _, _, _, _, _, _) => {
            h6833
        }
    })?;
    let a6834 = v0;
    let a6835 = 2u64;
    if nat_lt(a6834, a6835) {
        Ok(f304(v0))
    } else {
        let a6836 = v0;
        let a6837 = v6;
        if nat_lt(a6836, a6837) {
            f309({
                let a6838 = v0;
                let a6839 = 2u64;
                nat_sub(a6838, a6839)
            }, v1, v2, v3)
        } else {
            let a6840 = v0;
            let a6841 = v7;
            if nat_lt(a6840, a6841) {
                Ok(f310({
                    let a6842 = v0;
                    let a6843 = v6;
                    nat_sub(a6842, a6843)
                }, match v2 {
                    Adt20::C0(h6844, _, _, _, _, _, _, _) => {
                        h6844
                    }
                }, v5))
            } else {
                let a6846 = v0;
                let a6847 = f263(match v2.clone() {
                    Adt20::C0(h6845, _, _, _, _, _, _, _) => {
                        h6845
                    }
                })?;
                if nat_lt(a6846, a6847) {
                    f311({
                        let a6848 = v0;
                        let a6849 = v7;
                        nat_sub(a6848, a6849)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f294(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f279(v5, match v2.clone() {
        Adt20::C0(h6850, _, _, _, _, _, _, _) => {
            h6850
        }
    })?;
    let v7: u64 = {
        let a6852 = v6;
        let a6853 = f280(v5, match v2.clone() {
            Adt20::C0(h6851, _, _, _, _, _, _, _) => {
                h6851
            }
        })?;
        nat_add(a6852, a6853)?
    };
    let v8: u64 = {
        let a6854 = v7;
        let a6855 = f281(v5);
        nat_add(a6854, a6855)?
    };
    let a6856 = v0;
    let a6857 = v6;
    if nat_lt(a6856, a6857) {
        f312(v0, v1, v2, v5)
    } else {
        let a6858 = v0;
        let a6859 = v7;
        if nat_lt(a6858, a6859) {
            f313({
                let a6860 = v0;
                let a6861 = v6;
                nat_sub(a6860, a6861)
            }, v1, v2, v3, v5)
        } else {
            let a6862 = v0;
            let a6863 = v8;
            if nat_lt(a6862, a6863) {
                Ok(f314({
                    let a6864 = v0;
                    let a6865 = v7;
                    nat_sub(a6864, a6865)
                }, match v2 {
                    Adt20::C0(h6866, _, _, _, _, _, _, _) => {
                        h6866
                    }
                }, v4, v5))
            } else {
                let a6870 = v0;
                let a6871 = {
                    let a6868 = v8;
                    let a6869 = match v2.clone() {
                        Adt20::C0(h6867, _, _, _, _, _, _, _) => {
                            h6867
                        }
                    };
                    nat_add(a6868, a6869)?
                };
                if nat_lt(a6870, a6871) {
                    let v9: u64 = f305({
                        let a6877 = {
                            let a6873 = v1;
                            let a6874 = match v2 {
                                Adt20::C0(h6872, _, _, _, _, _, _, _) => {
                                    h6872
                                }
                            };
                            nat_mul(a6873, a6874)?
                        };
                        let a6878 = {
                            let a6875 = v0;
                            let a6876 = v8;
                            nat_sub(a6875, a6876)
                        };
                        nat_add(a6877, a6878)?
                    }, v3)?;
                    Ok(f306(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f295(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f282(v3, v5)?;
    let v7: u64 = f284(v1, v2, v6)?;
    let v8: u64 = {
        let a6881 = {
            let a6879 = f283(v3, v5)?;
            let a6880 = v4;
            nat_mul(a6879, a6880)?
        };
        let a6882 = v7;
        nat_mul(a6881, a6882)?
    };
    let a6883 = v0;
    let a6884 = v8;
    if nat_lt(a6883, a6884) {
        let v9: u64 = {
            let a6885 = v0;
            let a6886 = v7;
            let a6887 = 0u64;
            nat_quot(a6885, a6886, a6887)
        };
        Ok(Adt21::C0({
            let a6891 = {
                let a6888 = v9;
                let a6889 = v4;
                let a6890 = 0u64;
                nat_quot(a6888, a6889, a6890)
            };
            let a6892 = v6;
            nat_mul(a6891, a6892)?
        }, v6, {
            let a6893 = v9;
            let a6894 = v4;
            let a6895 = 0u64;
            nat_rem(a6893, a6894, a6895)
        }, {
            let a6896 = v0;
            let a6897 = v7;
            let a6898 = 0u64;
            nat_rem(a6896, a6897, a6898)
        }))
    } else {
        let v10: u64 = {
            let a6899 = v0;
            let a6900 = v8;
            nat_sub(a6899, a6900)
        };
        let v11: u64 = f284(v1, v2, f285(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6901 = f283(v3, v5)?;
            let a6902 = v6;
            nat_mul(a6901, a6902)?
        }, f285(v3, v5)?, {
            let a6903 = v10;
            let a6904 = v11;
            let a6905 = 0u64;
            nat_quot(a6903, a6904, a6905)
        }, {
            let a6906 = v10;
            let a6907 = v11;
            let a6908 = 0u64;
            nat_rem(a6906, a6907, a6908)
        }))
    }
}

pub fn f296(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h6909) => {
            h6909
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h6910, _, _, _, _, _, _, _) => {
            h6910
        }
    };
    let v6: u64 = {
        let a6913 = match v0.clone() {
            Adt21::C0(_, h6911, _, _) => {
                h6911
            }
        };
        let a6914 = match v1.clone() {
            Adt20::C0(_, h6912, _, _, _, _, _, _) => {
                h6912
            }
        };
        nat_mul(a6913, a6914)?
    };
    let v7: u64 = {
        let a6915 = v6;
        let a6916 = v5;
        nat_mul(a6915, a6916)?
    };
    let v8: u64 = {
        let a6919 = {
            let a6917 = 2u64;
            let a6918 = v5;
            nat_mul(a6917, a6918)?
        };
        let a6920 = 2u64;
        nat_add(a6919, a6920)?
    };
    let v9: u64 = {
        let a6923 = {
            let a6921 = 4u64;
            let a6922 = v5;
            nat_mul(a6921, a6922)?
        };
        let a6924 = 2u64;
        nat_add(a6923, a6924)?
    };
    let v10: u64 = {
        let a6927 = 1u64;
        let a6928 = {
            let a6925 = v6;
            let a6926 = v8;
            nat_mul(a6925, a6926)?
        };
        nat_add(a6927, a6928)?
    };
    let v11: u64 = {
        let a6929 = v10;
        let a6930 = 4u64;
        nat_add(a6929, a6930)?
    };
    let v12: u64 = {
        let a6933 = v11;
        let a6934 = {
            let a6931 = v6;
            let a6932 = v9;
            nat_mul(a6931, a6932)?
        };
        nat_add(a6933, a6934)?
    };
    let v13: u64 = {
        let a6935 = v12;
        let a6936 = 6u64;
        nat_add(a6935, a6936)?
    };
    let a6937 = v4;
    let a6938 = 0u64;
    if nat_eq(a6937, a6938) {
        Ok(f304(0u64))
    } else {
        let a6939 = v4;
        let a6940 = v10;
        if nat_lt(a6939, a6940) {
            let v14: u64 = {
                let a6941 = v4;
                let a6942 = 1u64;
                nat_sub(a6941, a6942)
            };
            f315(false, {
                let a6943 = v14;
                let a6944 = v8;
                let a6945 = 0u64;
                nat_rem(a6943, a6944, a6945)
            }, {
                let a6946 = v14;
                let a6947 = v8;
                let a6948 = 0u64;
                nat_quot(a6946, a6947, a6948)
            }, v0, v1, v2)
        } else {
            let a6949 = v4;
            let a6950 = v11;
            if nat_lt(a6949, a6950) {
                Ok(f316({
                    let a6951 = v4;
                    let a6952 = v10;
                    nat_sub(a6951, a6952)
                }, v7))
            } else {
                let a6953 = v4;
                let a6954 = v12;
                if nat_lt(a6953, a6954) {
                    let v15: u64 = {
                        let a6955 = v4;
                        let a6956 = v11;
                        nat_sub(a6955, a6956)
                    };
                    f315(true, {
                        let a6957 = v15;
                        let a6958 = v9;
                        let a6959 = 0u64;
                        nat_rem(a6957, a6958, a6959)
                    }, {
                        let a6960 = v15;
                        let a6961 = v9;
                        let a6962 = 0u64;
                        nat_quot(a6960, a6961, a6962)
                    }, v0, v1, v2)
                } else {
                    let a6963 = v4;
                    let a6964 = v13;
                    if nat_lt(a6963, a6964) {
                        Ok(f317({
                            let a6965 = v4;
                            let a6966 = v12;
                            nat_sub(a6965, a6966)
                        }, v7, v3))
                    } else {
                        let a6969 = v4;
                        let a6970 = {
                            let a6967 = v13;
                            let a6968 = v7;
                            nat_add(a6967, a6968)?
                        };
                        if nat_lt(a6969, a6970) {
                            let v16: u64 = {
                                let a6971 = v4;
                                let a6972 = v13;
                                nat_sub(a6971, a6972)
                            };
                            let v17: u64 = {
                                let a6973 = v16;
                                let a6974 = v5;
                                let a6975 = 0u64;
                                nat_quot(a6973, a6974, a6975)
                            };
                            let v18: u64 = {
                                let a6981 = {
                                    let a6976 = f318(v17, v0, v1)?;
                                    let a6977 = v5;
                                    nat_mul(a6976, a6977)?
                                };
                                let a6982 = {
                                    let a6978 = v16;
                                    let a6979 = v5;
                                    let a6980 = 0u64;
                                    nat_rem(a6978, a6979, a6980)
                                };
                                nat_add(a6981, a6982)?
                            };
                            let v19: u64 = f305(v18, v2)?;
                            Ok(f306(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f297(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f298(v0: u64) -> u64 {
    let a6983 = v0;
    let a6984 = 4u64;
    let a6985 = 0u64;
    nat_quot(a6983, a6984, a6985)
}

pub fn f299(v0: u64) -> R<u64> {
    let a6988 = {
        let a6986 = 7u64;
        let a6987 = v0;
        nat_mul(a6986, a6987)?
    };
    let a6989 = 4u64;
    nat_add(a6988, a6989)
}

pub fn f300(v0: u64) -> bool {
    let m6990 = f210(v0);
    match m6990 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f301(v0: u64, v1: u64) -> bool {
    let a6991 = v0;
    let a6992 = 1u64;
    if nat_eq(a6991, a6992) {
        true
    } else {
        let v2: u64 = f319(v0);
        let a6993 = 0u64;
        let a6994 = v2;
        if nat_lt(a6993, a6994) {
            let a6998 = {
                let a6995 = v1;
                let a6996 = v2;
                let a6997 = 1u64;
                nat_rem(a6995, a6996, a6997)
            };
            let a6999 = 0u64;
            nat_eq(a6998, a6999)
        } else {
            false
        }
    }
}

pub fn f302(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7000 = v0;
    let a7001 = 28u64;
    if if if if nat_eq(a7000, a7001) {
        let a7002 = v1;
        let a7003 = 0u64;
        nat_eq(a7002, a7003)
    } else {
        false
    } {
        let a7004 = v2;
        let a7005 = 0u64;
        nat_eq(a7004, a7005)
    } else {
        false
    } {
        let a7006 = v3;
        let a7007 = 0u64;
        nat_eq(a7006, a7007)
    } else {
        false
    } {
        let a7008 = v4;
        let a7009 = v8;
        if if if if if if if if nat_eq(a7008, a7009) {
            let a7010 = v5;
            let a7011 = v12;
            nat_eq(a7010, a7011)
        } else {
            false
        } {
            let a7012 = v9;
            let a7013 = v13;
            nat_eq(a7012, a7013)
        } else {
            false
        } {
            let a7014 = v10;
            let a7015 = v14;
            nat_eq(a7014, a7015)
        } else {
            false
        } {
            let a7016 = v11;
            let a7017 = v15;
            nat_eq(a7016, a7017)
        } else {
            false
        } {
            let a7018 = 0u64;
            let a7019 = v6;
            nat_lt(a7018, a7019)
        } else {
            false
        } {
            let a7020 = 0u64;
            let a7021 = v7;
            nat_lt(a7020, a7021)
        } else {
            false
        } {
            let a7025 = {
                let a7022 = v10;
                let a7023 = v6;
                let a7024 = 1u64;
                nat_rem(a7022, a7023, a7024)
            };
            let a7026 = 0u64;
            if nat_eq(a7025, a7026) {
                let a7030 = {
                    let a7027 = v11;
                    let a7028 = v7;
                    let a7029 = 1u64;
                    nat_rem(a7027, a7028, a7029)
                };
                let a7031 = 0u64;
                nat_eq(a7030, a7031)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7032 = v10;
                let a7033 = v6;
                let a7034 = 0u64;
                nat_quot(a7032, a7033, a7034)
            }, {
                let a7035 = v11;
                let a7036 = v7;
                let a7037 = 0u64;
                nat_quot(a7035, a7036, a7037)
            }, {
                let a7038 = v5;
                let a7039 = v4;
                nat_mul(a7038, a7039)?
            }, {
                let a7040 = v4;
                let a7041 = v9;
                nat_mul(a7040, a7041)?
            }, {
                let a7042 = v5;
                let a7043 = v9;
                nat_mul(a7042, a7043)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f303(v0: u64) -> bool {
    let a7044 = v0;
    let a7045 = 0u64;
    if if nat_eq(a7044, a7045) {
        true
    } else {
        let a7046 = v0;
        let a7047 = 1u64;
        nat_eq(a7046, a7047)
    } {
        true
    } else {
        let a7048 = v0;
        let a7049 = 30u64;
        nat_eq(a7048, a7049)
    }
}

pub fn f304(v0: u64) -> Adt17 {
    f306(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f305(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7051 = v0;
        let a7052 = match v1.clone() {
            Adt20::C0(h7050, _, _, _, _, _, _, _) => {
                h7050
            }
        };
        let a7053 = 0u64;
        nat_quot(a7051, a7052, a7053)
    };
    let v3: u64 = {
        let a7055 = v2;
        let a7056 = match v1.clone() {
            Adt20::C0(_, h7054, _, _, _, _, _, _) => {
                h7054
            }
        };
        let a7057 = 0u64;
        nat_quot(a7055, a7056, a7057)
    };
    f138({
        let a7059 = v0;
        let a7060 = match v1.clone() {
            Adt20::C0(h7058, _, _, _, _, _, _, _) => {
                h7058
            }
        };
        let a7061 = 0u64;
        nat_rem(a7059, a7060, a7061)
    }, {
        let a7063 = v2;
        let a7064 = match v1.clone() {
            Adt20::C0(_, h7062, _, _, _, _, _, _) => {
                h7062
            }
        };
        let a7065 = 0u64;
        nat_rem(a7063, a7064, a7065)
    }, {
        let a7067 = v3;
        let a7068 = match v1.clone() {
            Adt20::C0(_, _, h7066, _, _, _, _, _) => {
                h7066
            }
        };
        let a7069 = 0u64;
        nat_rem(a7067, a7068, a7069)
    }, {
        let a7071 = v3;
        let a7072 = match v1.clone() {
            Adt20::C0(_, _, h7070, _, _, _, _, _) => {
                h7070
            }
        };
        let a7073 = 0u64;
        nat_quot(a7071, a7072, a7073)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7074, _, _, _) => {
            h7074
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7075, _, _) => {
            h7075
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7076, _) => {
            h7076
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7077) => {
            h7077
        }
    })
}

pub fn f306(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7078 = v0;
    let a7079 = 4u64;
    if if if nat_le(a7078, a7079) {
        true
    } else {
        let a7080 = 9u64;
        let a7081 = v0;
        if nat_le(a7080, a7081) {
            let a7082 = v0;
            let a7083 = 12u64;
            nat_le(a7082, a7083)
        } else {
            false
        }
    } {
        true
    } else {
        let a7084 = v0;
        let a7085 = 16u64;
        if if nat_eq(a7084, a7085) {
            true
        } else {
            let a7086 = v0;
            let a7087 = 19u64;
            nat_eq(a7086, a7087)
        } {
            true
        } else {
            let a7088 = v0;
            let a7089 = 21u64;
            if nat_eq(a7088, a7089) {
                true
            } else {
                let a7090 = v0;
                let a7091 = 22u64;
                nat_eq(a7090, a7091)
            }
        }
    } {
        Adt17::C1({
            let a7092 = v0;
            let a7093 = 4u64;
            if if nat_lt(a7092, a7093) {
                true
            } else {
                let a7094 = 21u64;
                let a7095 = v0;
                nat_le(a7094, a7095)
            } {
                0u64
            } else {
                let a7096 = v0;
                let a7097 = 16u64;
                if if nat_eq(a7096, a7097) {
                    true
                } else {
                    let a7098 = v0;
                    let a7099 = 19u64;
                    nat_eq(a7098, a7099)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7100 = v0;
            let a7101 = 4u64;
            if nat_lt(a7100, a7101) {
                v0
            } else {
                let a7102 = v0;
                let a7103 = 4u64;
                if if nat_eq(a7102, a7103) {
                    true
                } else {
                    let a7104 = v0;
                    let a7105 = 16u64;
                    nat_eq(a7104, a7105)
                } {
                    0u64
                } else {
                    let a7106 = v0;
                    let a7107 = 19u64;
                    if nat_eq(a7106, a7107) {
                        3u64
                    } else {
                        let a7108 = 21u64;
                        let a7109 = v0;
                        if nat_le(a7108, a7109) {
                            let a7110 = v0;
                            let a7111 = 17u64;
                            nat_sub(a7110, a7111)
                        } else {
                            let a7112 = v0;
                            let a7113 = 8u64;
                            nat_sub(a7112, a7113)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f307(v0: u64) -> R<u64> {
    let a7116 = 2u64;
    let a7117 = {
        let a7114 = 6u64;
        let a7115 = v0;
        nat_mul(a7114, a7115)?
    };
    nat_add(a7116, a7117)
}

pub fn f308(v0: u64) -> R<u64> {
    let a7118 = f307(v0)?;
    let a7119 = 11u64;
    nat_add(a7118, a7119)
}

pub fn f309(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7120 = v0;
        let a7121 = 6u64;
        let a7122 = 0u64;
        nat_rem(a7120, a7121, a7122)
    };
    let v5: u64 = {
        let a7129 = {
            let a7124 = v1;
            let a7125 = match v2.clone() {
                Adt20::C0(h7123, _, _, _, _, _, _, _) => {
                    h7123
                }
            };
            nat_mul(a7124, a7125)?
        };
        let a7130 = {
            let a7126 = v0;
            let a7127 = 6u64;
            let a7128 = 0u64;
            nat_quot(a7126, a7127, a7128)
        };
        nat_add(a7129, a7130)?
    };
    let v6: u64 = f305(v5, v3)?;
    let v7: u64 = f305(v5, v2)?;
    let a7131 = v4;
    let a7132 = 0u64;
    if nat_eq(a7131, a7132) {
        Ok(f306(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7133 = v4;
        let a7134 = 1u64;
        if nat_eq(a7133, a7134) {
            Ok(f306(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7135 = v4;
            let a7136 = 2u64;
            if nat_eq(a7135, a7136) {
                Ok(f306(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7137 = v4;
                let a7138 = 3u64;
                if nat_eq(a7137, a7138) {
                    Ok(f306(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7139 = v4;
                    let a7140 = 4u64;
                    if nat_eq(a7139, a7140) {
                        Ok(f306(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f306(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f310(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7141 = v0;
    let a7142 = 0u64;
    if nat_eq(a7141, a7142) {
        f306(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7143 = v0;
        let a7144 = 1u64;
        if nat_eq(a7143, a7144) {
            f306(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7145 = v0;
            let a7146 = 2u64;
            if nat_eq(a7145, a7146) {
                f306(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7147 = v0;
                let a7148 = 3u64;
                if nat_eq(a7147, a7148) {
                    f306(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7149 = v0;
                    let a7150 = 4u64;
                    if nat_eq(a7149, a7150) {
                        f306(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7151 = v0;
                        let a7152 = 5u64;
                        if nat_eq(a7151, a7152) {
                            f306(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7153 = v0;
                            let a7154 = 6u64;
                            if nat_eq(a7153, a7154) {
                                f306(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7155 = v0;
                                let a7156 = 7u64;
                                if nat_eq(a7155, a7156) {
                                    f306(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7157 = v0;
                                    let a7158 = 8u64;
                                    if nat_eq(a7157, a7158) {
                                        f306(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7159 = v0;
                                        let a7160 = 9u64;
                                        if nat_eq(a7159, a7160) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7161 = v0;
                                            let a7162 = 10u64;
                                            if nat_eq(a7161, a7162) {
                                                f306(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f311(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7163 = v0;
        let a7164 = 3u64;
        let a7165 = 0u64;
        nat_rem(a7163, a7164, a7165)
    };
    let v6: u64 = {
        let a7172 = {
            let a7167 = v1;
            let a7168 = match v2.clone() {
                Adt20::C0(h7166, _, _, _, _, _, _, _) => {
                    h7166
                }
            };
            nat_mul(a7167, a7168)?
        };
        let a7173 = {
            let a7169 = v0;
            let a7170 = 3u64;
            let a7171 = 0u64;
            nat_quot(a7169, a7170, a7171)
        };
        nat_add(a7172, a7173)?
    };
    let v7: u64 = f305(v6, v3)?;
    let v8: u64 = f305(v6, v2)?;
    let v9: u64 = f305(v6, v4)?;
    let a7174 = v5;
    let a7175 = 0u64;
    if nat_eq(a7174, a7175) {
        Ok(f306(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7176 = v5;
        let a7177 = 1u64;
        if nat_eq(a7176, a7177) {
            Ok(f306(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f306(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f312(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7178, _, _, _, _, _, _, _) => {
            h7178
        }
    };
    let v5: u64 = {
        let a7181 = 1u64;
        let a7182 = {
            let a7179 = v4;
            let a7180 = f297(v3);
            nat_mul(a7179, a7180)?
        };
        nat_add(a7181, a7182)?
    };
    let a7183 = v0;
    let a7184 = 0u64;
    if nat_eq(a7183, a7184) {
        Ok(f304(0u64))
    } else {
        let a7185 = v0;
        let a7186 = v5;
        if nat_lt(a7185, a7186) {
            let v6: u64 = {
                let a7187 = v0;
                let a7188 = 1u64;
                nat_sub(a7187, a7188)
            };
            let v7: u64 = f305({
                let a7194 = {
                    let a7189 = v1;
                    let a7190 = v4;
                    nat_mul(a7189, a7190)?
                };
                let a7195 = {
                    let a7191 = v6;
                    let a7192 = f297(v3);
                    let a7193 = 0u64;
                    nat_quot(a7191, a7192, a7193)
                };
                nat_add(a7194, a7195)?
            }, v2)?;
            if v3 {
                let a7199 = {
                    let a7196 = v6;
                    let a7197 = 2u64;
                    let a7198 = 0u64;
                    nat_rem(a7196, a7197, a7198)
                };
                let a7200 = 0u64;
                if nat_eq(a7199, a7200) {
                    Ok(f306(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f306(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f306(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7201 = v0;
                let a7202 = v5;
                nat_sub(a7201, a7202)
            };
            let a7203 = v8;
            let a7204 = 0u64;
            if nat_eq(a7203, a7204) {
                Ok(f306(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7205 = v8;
                    let a7206 = 1u64;
                    nat_eq(a7205, a7206)
                } else {
                    false
                } {
                    Ok(f306(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7207 = v8;
                    let a7208 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7207, a7208) {
                        Ok(f306(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7209 = v8;
                        let a7210 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7209, a7210) {
                            Ok(f304(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f313(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7212 = 13u64;
        let a7213 = f298(match v2.clone() {
            Adt20::C0(h7211, _, _, _, _, _, _, _) => {
                h7211
            }
        });
        nat_mul(a7212, a7213)?
    } else {
        0u64
    };
    let a7214 = v0;
    let a7215 = v5;
    if nat_lt(a7214, a7215) {
        f320({
            let a7216 = v0;
            let a7217 = 13u64;
            let a7218 = 0u64;
            nat_rem(a7216, a7217, a7218)
        }, v1, {
            let a7219 = v0;
            let a7220 = 13u64;
            let a7221 = 0u64;
            nat_quot(a7219, a7220, a7221)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7222 = v0;
            let a7223 = v5;
            nat_sub(a7222, a7223)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f321({
            let a7224 = v6;
            let a7225 = v7;
            let a7226 = 0u64;
            nat_rem(a7224, a7225, a7226)
        }, {
            let a7236 = {
                let a7231 = {
                    let a7228 = v1;
                    let a7229 = match v2.clone() {
                        Adt20::C0(h7227, _, _, _, _, _, _, _) => {
                            h7227
                        }
                    };
                    nat_mul(a7228, a7229)?
                };
                let a7232 = if v4 {
                    f322(match v2.clone() {
                        Adt20::C0(h7230, _, _, _, _, _, _, _) => {
                            h7230
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7231, a7232)?
            };
            let a7237 = {
                let a7233 = v6;
                let a7234 = v7;
                let a7235 = 0u64;
                nat_quot(a7233, a7234, a7235)
            };
            nat_add(a7236, a7237)?
        }, v2, v3, v4)
    }
}

pub fn f314(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7238 = v0;
    let a7239 = 0u64;
    if nat_eq(a7238, a7239) {
        f306(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7240 = v0;
        let a7241 = 1u64;
        if nat_eq(a7240, a7241) {
            f306(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7242 = v0;
                let a7243 = 2u64;
                nat_eq(a7242, a7243)
            } else {
                false
            } {
                f306(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7244 = v0;
                let a7245 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7244, a7245) {
                    f306(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7246 = v0;
                    let a7247 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7246, a7247) {
                        f306(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7248 = v0;
                        let a7249 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7248, a7249) {
                            f306(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f315(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7253 = 1u64;
        let a7254 = {
            let a7251 = match v4.clone() {
                Adt20::C0(h7250, _, _, _, _, _, _, _) => {
                    h7250
                }
            };
            let a7252 = v6;
            nat_mul(a7251, a7252)?
        };
        nat_add(a7253, a7254)?
    };
    let a7255 = v1;
    let a7256 = 0u64;
    if nat_eq(a7255, a7256) {
        Ok(f304(1u64))
    } else {
        let a7257 = v1;
        let a7258 = v7;
        if nat_lt(a7257, a7258) {
            let v8: u64 = {
                let a7259 = v1;
                let a7260 = 1u64;
                nat_sub(a7259, a7260)
            };
            let v9: u64 = {
                let a7267 = {
                    let a7262 = f318(v2, v3, v4.clone())?;
                    let a7263 = match v4.clone() {
                        Adt20::C0(h7261, _, _, _, _, _, _, _) => {
                            h7261
                        }
                    };
                    nat_mul(a7262, a7263)?
                };
                let a7268 = {
                    let a7264 = v8;
                    let a7265 = v6;
                    let a7266 = 0u64;
                    nat_quot(a7264, a7265, a7266)
                };
                nat_add(a7267, a7268)?
            };
            let v10: u64 = f305(v9, v4)?;
            let v11: u64 = f305(v9, v5)?;
            let v12: u64 = {
                let a7269 = v8;
                let a7270 = v6;
                let a7271 = 0u64;
                nat_rem(a7269, a7270, a7271)
            };
            if v0 {
                let a7272 = v12;
                let a7273 = 0u64;
                if nat_eq(a7272, a7273) {
                    Ok(f306(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7274 = v12;
                    let a7275 = 1u64;
                    if nat_eq(a7274, a7275) {
                        Ok(f306(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7276 = v12;
                        let a7277 = 2u64;
                        if nat_eq(a7276, a7277) {
                            Ok(f306(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f306(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7278 = v12;
                let a7279 = 0u64;
                if nat_eq(a7278, a7279) {
                    Ok(f306(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f306(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7280 = v1;
            let a7281 = v7;
            if nat_eq(a7280, a7281) {
                Ok(f306(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f316(v0: u64, v1: u64) -> Adt17 {
    let a7282 = v0;
    let a7283 = 0u64;
    if nat_eq(a7282, a7283) {
        f306(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7284 = v0;
        let a7285 = 1u64;
        if nat_eq(a7284, a7285) {
            f306(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7286 = v0;
            let a7287 = 2u64;
            if nat_eq(a7286, a7287) {
                f306(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7288 = v0;
                let a7289 = 3u64;
                if nat_eq(a7288, a7289) {
                    f304(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f317(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7290 = v0;
    let a7291 = 0u64;
    if nat_eq(a7290, a7291) {
        f306(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7292 = v0;
        let a7293 = 1u64;
        if nat_eq(a7292, a7293) {
            f306(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7294 = v0;
            let a7295 = 2u64;
            if nat_eq(a7294, a7295) {
                f306(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7296 = v0;
                let a7297 = 3u64;
                if nat_eq(a7296, a7297) {
                    f306(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7298 = v0;
                    let a7299 = 4u64;
                    if nat_eq(a7298, a7299) {
                        f306(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7300 = v0;
                        let a7301 = 5u64;
                        if nat_eq(a7300, a7301) {
                            f306(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f318(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7312 = {
        let a7310 = {
            let a7307 = {
                let a7304 = match v1.clone() {
                    Adt21::C0(_, _, h7302, _) => {
                        h7302
                    }
                };
                let a7305 = match v2.clone() {
                    Adt20::C0(_, _, h7303, _, _, _, _, _) => {
                        h7303
                    }
                };
                nat_mul(a7304, a7305)?
            };
            let a7308 = match v1 {
                Adt21::C0(h7306, _, _, _) => {
                    h7306
                }
            };
            nat_add(a7307, a7308)?
        };
        let a7311 = match v2 {
            Adt20::C0(_, h7309, _, _, _, _, _, _) => {
                h7309
            }
        };
        nat_mul(a7310, a7311)?
    };
    let a7313 = v0;
    nat_add(a7312, a7313)
}

pub fn f319(v0: u64) -> u64 {
    if f217(v0) {
        let m7314 = f210(v0);
        match m7314 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7316 = 1u64;
                let a7317 = match v1.clone() {
                    Adt19::C0(_, h7315, _) => {
                        h7315
                    }
                };
                if if nat_lt(a7316, a7317) {
                    true
                } else {
                    let a7318 = v0;
                    let a7319 = 43u64;
                    nat_eq(a7318, a7319)
                } {
                    match v1 {
                        Adt19::C0(_, h7320, _) => {
                            h7320
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

pub fn f320(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7328 = {
            let a7326 = {
                let a7322 = v1;
                let a7323 = match v3.clone() {
                    Adt20::C0(h7321, _, _, _, _, _, _, _) => {
                        h7321
                    }
                };
                nat_mul(a7322, a7323)?
            };
            let a7327 = {
                let a7324 = v2;
                let a7325 = 4u64;
                nat_mul(a7324, a7325)?
            };
            nat_add(a7326, a7327)?
        };
        let a7329 = f323(v0);
        nat_add(a7328, a7329)?
    };
    let v6: u64 = f305(v5, v3)?;
    let v7: u64 = f305(v5, v4)?;
    let a7330 = v0;
    let a7331 = 0u64;
    if if if if nat_eq(a7330, a7331) {
        true
    } else {
        let a7332 = v0;
        let a7333 = 2u64;
        nat_eq(a7332, a7333)
    } {
        true
    } else {
        let a7334 = v0;
        let a7335 = 5u64;
        nat_eq(a7334, a7335)
    } {
        true
    } else {
        let a7336 = v0;
        let a7337 = 7u64;
        nat_eq(a7336, a7337)
    } {
        Ok(f306(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7338 = v0;
        let a7339 = 1u64;
        if if if if nat_eq(a7338, a7339) {
            true
        } else {
            let a7340 = v0;
            let a7341 = 3u64;
            nat_eq(a7340, a7341)
        } {
            true
        } else {
            let a7342 = v0;
            let a7343 = 6u64;
            nat_eq(a7342, a7343)
        } {
            true
        } else {
            let a7344 = v0;
            let a7345 = 8u64;
            nat_eq(a7344, a7345)
        } {
            Ok(f306(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7346 = v0;
                let a7347 = 1u64;
                if nat_eq(a7346, a7347) {
                    4u64
                } else {
                    let a7348 = v0;
                    let a7349 = 6u64;
                    if nat_eq(a7348, a7349) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7350 = v0;
            let a7351 = 4u64;
            if nat_eq(a7350, a7351) {
                Ok(f306(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7352 = v0;
                let a7353 = 9u64;
                if nat_eq(a7352, a7353) {
                    Ok(f306(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7354 = v0;
                    let a7355 = 10u64;
                    if nat_eq(a7354, a7355) {
                        Ok(f306(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7356 = v0;
                        let a7357 = 11u64;
                        if nat_eq(a7356, a7357) {
                            Ok(f306(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7358 = v0;
                            let a7359 = 12u64;
                            if nat_eq(a7358, a7359) {
                                Ok(f306(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f321(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f305(v1, v2)?;
    let v6: u64 = f305(v1, v3)?;
    let a7360 = v0;
    let a7361 = 0u64;
    if nat_eq(a7360, a7361) {
        Ok(f306(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7362 = v0;
        let a7363 = 1u64;
        if nat_eq(a7362, a7363) {
            Ok(f306(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7364 = v0;
                let a7365 = 2u64;
                if nat_eq(a7364, a7365) {
                    Ok(f306(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7366 = v0;
                    let a7367 = 3u64;
                    if nat_eq(a7366, a7367) {
                        Ok(f306(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7368 = v0;
                let a7369 = 2u64;
                if nat_eq(a7368, a7369) {
                    Ok(f306(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f322(v0: u64) -> R<u64> {
    let a7370 = 4u64;
    let a7371 = f298(v0);
    nat_mul(a7370, a7371)
}

pub fn f323(v0: u64) -> u64 {
    let a7372 = v0;
    let a7373 = 5u64;
    if nat_lt(a7372, a7373) {
        let a7374 = v0;
        let a7375 = 2u64;
        if nat_lt(a7374, a7375) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7376 = v0;
        let a7377 = 7u64;
        if nat_lt(a7376, a7377) {
            2u64
        } else {
            3u64
        }
    }
}
