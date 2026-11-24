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
    C43(Adt7, u64),
    C44(u64),
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
            if if if if if nat_eq(a2, a3) {
                true
            } else {
                let a4 = v1;
                let a5 = 50u64;
                nat_eq(a4, a5)
            } {
                true
            } else {
                let a6 = v1;
                let a7 = 59u64;
                nat_eq(a6, a7)
            } {
                true
            } else {
                let a8 = v1;
                let a9 = 65u64;
                nat_eq(a8, a9)
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
                let a10 = v1;
                let a11 = 94u64;
                if if nat_eq(a10, a11) {
                    true
                } else {
                    let a12 = v1;
                    let a13 = 95u64;
                    nat_eq(a12, a13)
                } {
                    Ok(Adt9::C0(f2({
                        let a14 = v1;
                        let a15 = 95u64;
                        nat_eq(a14, a15)
                    }, v3, v4, v5, {
                        let m16 = v6;
                        match m16 {
                            None => {
                                let m17 = v7;
                                match m17 {
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
                    let a18 = v1;
                    let a19 = 96u64;
                    if if nat_eq(a18, a19) {
                        true
                    } else {
                        let a20 = v1;
                        let a21 = 97u64;
                        nat_eq(a20, a21)
                    } {
                        Ok(Adt9::C0(f3({
                            let a22 = v1;
                            let a23 = 96u64;
                            nat_eq(a22, a23)
                        }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                    } else {
                        let v15: bool = {
                            let m24 = v7;
                            match m24 {
                                None => {
                                    v8
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        };
                        let a25 = v1;
                        let a26 = 85u64;
                        if nat_eq(a25, a26) {
                            Ok(Adt9::C0(f4(v2, v3, v4, v5, {
                                let m27 = v6;
                                match m27 {
                                    None => {
                                        v15
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v11)?))
                        } else {
                            let a28 = v1;
                            let a29 = 87u64;
                            if nat_eq(a28, a29) {
                                Ok(Adt9::C0(f5(v3, v4, v5, v6, v15, v11)?))
                            } else {
                                let a30 = v1;
                                let a31 = 64u64;
                                if nat_eq(a30, a31) {
                                    Ok(Adt9::C0(f6(v2, v11)?))
                                } else {
                                    let m32 = v3;
                                    match m32 {
                                        None => {
                                            Ok(Adt9::C0(Adt10::C1))
                                        }
                                        Some(v17) => {
                                            let a33 = v1;
                                            let a34 = 48u64;
                                            if if nat_eq(a33, a34) {
                                                true
                                            } else {
                                                let a35 = v1;
                                                let a36 = 56u64;
                                                nat_eq(a35, a36)
                                            } {
                                                Ok(Adt9::C0(f7({
                                                    let a37 = v1;
                                                    let a38 = 56u64;
                                                    nat_eq(a37, a38)
                                                }, v2, v17, v4, v11)?))
                                            } else {
                                                let a39 = v1;
                                                let a40 = 53u64;
                                                if if nat_eq(a39, a40) {
                                                    true
                                                } else {
                                                    let a41 = v1;
                                                    let a42 = 54u64;
                                                    nat_eq(a41, a42)
                                                } {
                                                    Ok(Adt9::C0(f8({
                                                        let a43 = v1;
                                                        let a44 = 54u64;
                                                        nat_eq(a43, a44)
                                                    }, v2, v17, v4, v11)?))
                                                } else {
                                                    let a45 = v1;
                                                    let a46 = 78u64;
                                                    if nat_eq(a45, a46) {
                                                        Ok(Adt9::C0(f9(v2, v17, v4, v5, v11)?))
                                                    } else {
                                                        let a47 = v1;
                                                        let a48 = 23u64;
                                                        if if if nat_eq(a47, a48) {
                                                            true
                                                        } else {
                                                            let a49 = v1;
                                                            let a50 = 26u64;
                                                            nat_eq(a49, a50)
                                                        } {
                                                            true
                                                        } else {
                                                            let a51 = v1;
                                                            let a52 = 25u64;
                                                            nat_eq(a51, a52)
                                                        } {
                                                            Ok(Adt9::C0(f10(v1, v2, v17, v4, v11)?))
                                                        } else {
                                                            let a53 = v1;
                                                            let a54 = 24u64;
                                                            if nat_eq(a53, a54) {
                                                                Ok(Adt9::C0(f11(v2, v17, v11)?))
                                                            } else {
                                                                let a55 = v1;
                                                                let a56 = 27u64;
                                                                if nat_eq(a55, a56) {
                                                                    Ok(Adt9::C0(f12(v2, v17, v11)?))
                                                                } else {
                                                                    let a57 = v1;
                                                                    let a58 = 3u64;
                                                                    if nat_eq(a57, a58) {
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
            let a60 = 0u64;
            let a61 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h59, _, _, _, _, _, _, _) => {
                    h59
                }
            };
            if nat_lt(a60, a61) {
                f15(v18, v19, v20, v21)
            } else {
                f16(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a62 = 3240099840u64;
                let a63 = v22;
                if if nat_le(a62, a63) {
                    let a64 = v22;
                    let a65 = 4286578688u64;
                    nat_le(a64, a65)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a66 = 1092616192u64;
                    let a67 = v22;
                    if if nat_le(a66, a67) {
                        let a68 = v22;
                        let a69 = 2139095040u64;
                        nat_le(a68, a69)
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
    let m70 = v0;
    match m70 {
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
        let m71 = v1.clone();
        match m71 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m72 = v2.clone();
        match m72 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m73 = v1.clone();
        match m73 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m74 = v3.clone();
        match m74 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m75 = v2.clone();
        match m75 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a81 = {
            let a78 = match v10.clone() {
                Adt2::C0(_, _, h76, _, _, _, _, _, _, _, _) => {
                    h76
                }
            };
            let a79 = match v10.clone() {
                Adt2::C0(_, _, _, h77, _, _, _, _, _, _, _) => {
                    h77
                }
            };
            nat_mul(a78, a79)?
        };
        let a82 = match v10.clone() {
            Adt2::C0(_, _, _, _, h80, _, _, _, _, _, _) => {
                h80
            }
        };
        nat_mul(a81, a82)?
    };
    let m83 = v1;
    if if if if if if if if if if match m83 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m84 = v2;
        match m84 {
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
            let m85 = v3;
            match m85 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m86 = v3;
            !match m86 {
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
        let a87 = 0u64;
        let a88 = v14;
        nat_lt(a87, a88)
    } else {
        false
    } {
        let a90 = 0u64;
        let a91 = match v10.clone() {
            Adt2::C0(_, h89, _, _, _, _, _, _, _, _, _) => {
                h89
            }
        };
        nat_lt(a90, a91)
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
            let a93 = match v7.clone() {
                Adt2::C0(h92, _, _, _, _, _, _, _, _, _, _) => {
                    h92
                }
            };
            let a94 = 0u64;
            if if if if nat_eq(a93, a94) {
                let a95 = f23(v7.clone(), 1u64)?;
                let a96 = 1u64;
                nat_eq(a95, a96)
            } else {
                false
            } {
                let a98 = 4u64;
                let a99 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h97, _) => {
                        h97
                    }
                };
                nat_le(a98, a99)
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
            let a101 = match v5.clone() {
                Adt2::C0(h100, _, _, _, _, _, _, _, _, _, _) => {
                    h100
                }
            };
            let a102 = 0u64;
            if if nat_eq(a101, a102) {
                let a103 = f23(v5.clone(), 1u64)?;
                let a104 = 1u64;
                nat_eq(a103, a104)
            } else {
                false
            } {
                let a106 = 4u64;
                let a107 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h105, _) => {
                        h105
                    }
                };
                nat_le(a106, a107)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a113 = {
                let a111 = 2u64;
                let a112 = {
                    let a109 = v14;
                    let a110 = f24(v0, match v10.clone() {
                        Adt2::C0(_, h108, _, _, _, _, _, _, _, _, _) => {
                            h108
                        }
                    })?;
                    nat_mul(a109, a110)?
                };
                nat_add(a111, a112)?
            };
            let a114 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a113, a114)?
        }, 4u64, f25(v10, 1u64), f25(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h115, _, _, _, _) => {
                h115
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m116 = v3.clone();
        match m116 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m117 = v4.clone();
        match m117 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m118 = v5.clone();
        match m118 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m119 = v6.clone();
        match m119 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m120 = v7.clone();
        match m120 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m121 = v5.clone();
        match m121 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m122 = v3;
    if if if if if if if if if if if match m122 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m123 = v4;
        match m123 {
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
        let m124 = v5;
        match m124 {
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
            let m125 = v6;
            if match m125 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m126 = v7;
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
            }
        } else {
            let m127 = v6;
            if !match m127 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m128 = v7;
                !match m128 {
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
            let a135 = {
                let a132 = {
                    let a129 = v1;
                    let a130 = 4u64;
                    let a131 = 0u64;
                    nat_quot(a129, a130, a131)
                };
                let a133 = 2u64;
                let a134 = 0u64;
                nat_rem(a132, a133, a134)
            };
            let a136 = 1u64;
            nat_eq(a135, a136)
        } else {
            false
        } {
            let a138 = match v11.clone() {
                Adt2::C0(h137, _, _, _, _, _, _, _, _, _, _) => {
                    h137
                }
            };
            let a139 = 0u64;
            nat_eq(a138, a139)
        } else {
            false
        } {
            let a141 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h140, _, _, _, _, _) => {
                    h140
                }
            };
            let a142 = 4u64;
            nat_eq(a141, a142)
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
            let a144 = match v11.clone() {
                Adt2::C0(_, h143, _, _, _, _, _, _, _, _, _) => {
                    h143
                }
            };
            let a145 = 1u64;
            if nat_le(a144, a145) {
                true
            } else {
                let a148 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h146, _, _, _, _, _) => {
                        h146
                    }
                };
                let a149 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h147, _, _, _, _, _) => {
                        h147
                    }
                };
                nat_eq(a148, a149)
            }
        } else {
            false
        } {
            let a151 = match v11.clone() {
                Adt2::C0(_, _, h150, _, _, _, _, _, _, _, _) => {
                    h150
                }
            };
            let a152 = 1u64;
            if nat_le(a151, a152) {
                true
            } else {
                let a155 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h153, _, _, _, _) => {
                        h153
                    }
                };
                let a156 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h154, _, _, _, _) => {
                        h154
                    }
                };
                nat_eq(a155, a156)
            }
        } else {
            false
        } {
            let a158 = match v11.clone() {
                Adt2::C0(_, _, _, h157, _, _, _, _, _, _, _) => {
                    h157
                }
            };
            let a159 = 1u64;
            if nat_le(a158, a159) {
                true
            } else {
                let a162 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h160, _, _, _) => {
                        h160
                    }
                };
                let a163 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h161, _, _, _) => {
                        h161
                    }
                };
                nat_eq(a162, a163)
            }
        } else {
            false
        } {
            let a165 = match v11.clone() {
                Adt2::C0(_, _, _, _, h164, _, _, _, _, _, _) => {
                    h164
                }
            };
            let a166 = 1u64;
            if nat_le(a165, a166) {
                true
            } else {
                let a169 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h167, _, _) => {
                        h167
                    }
                };
                let a170 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h168, _, _) => {
                        h168
                    }
                };
                nat_eq(a169, a170)
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
        let a172 = match v20.clone() {
            Adt2::C0(h171, _, _, _, _, _, _, _, _, _, _) => {
                h171
            }
        };
        let a173 = 0u64;
        nat_eq(a172, a173)
    } else {
        false
    } {
        let a174 = f23(v20.clone(), 1u64)?;
        let a175 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a174, a175)
    } else {
        false
    } {
        let a177 = if v0 {
            28u64
        } else {
            8u64
        };
        let a178 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h176, _) => {
                h176
            }
        };
        nat_le(a177, a178)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a181 = f28(v0);
            let a182 = {
                let a179 = f23(v11.clone(), 1u64)?;
                let a180 = f29(v0);
                nat_mul(a179, a180)?
            };
            nat_add(a181, a182)?
        }, 4u64, f25(v11, 1u64), f25(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m183 = v1.clone();
        match m183 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m184 = v2.clone();
        match m184 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m185 = v3.clone();
        match m185 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h186, _, _, _, _, _, _, _, _, _, _) => {
            h186
        }
    };
    let m187 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m187 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m188 = v2;
        match m188 {
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
        let m189 = v3;
        match m189 {
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
        let a191 = match v7.clone() {
            Adt2::C0(h190, _, _, _, _, _, _, _, _, _, _) => {
                h190
            }
        };
        let a192 = 0u64;
        if if nat_eq(a191, a192) {
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
        let a194 = match v9.clone() {
            Adt2::C0(h193, _, _, _, _, _, _, _, _, _, _) => {
                h193
            }
        };
        let a195 = 0u64;
        if if nat_eq(a194, a195) {
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
        let a197 = match v11.clone() {
            Adt2::C0(h196, _, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        };
        let a198 = 0u64;
        if if nat_eq(a197, a198) {
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
        let a200 = match v7.clone() {
            Adt2::C0(_, h199, _, _, _, _, _, _, _, _, _) => {
                h199
            }
        };
        let a201 = 24u64;
        nat_eq(a200, a201)
    } else {
        false
    } {
        let a203 = match v7.clone() {
            Adt2::C0(_, _, _, h202, _, _, _, _, _, _, _) => {
                h202
            }
        };
        let a204 = 1u64;
        nat_eq(a203, a204)
    } else {
        false
    } {
        let a206 = match v7.clone() {
            Adt2::C0(_, _, _, _, h205, _, _, _, _, _, _) => {
                h205
            }
        };
        let a207 = 1u64;
        nat_eq(a206, a207)
    } else {
        false
    } {
        let a209 = match v11.clone() {
            Adt2::C0(_, h208, _, _, _, _, _, _, _, _, _) => {
                h208
            }
        };
        let a210 = 24u64;
        nat_eq(a209, a210)
    } else {
        false
    } {
        let a212 = match v11.clone() {
            Adt2::C0(_, _, h211, _, _, _, _, _, _, _, _) => {
                h211
            }
        };
        let a213 = 1u64;
        nat_eq(a212, a213)
    } else {
        false
    } {
        let a215 = match v11.clone() {
            Adt2::C0(_, _, _, h214, _, _, _, _, _, _, _) => {
                h214
            }
        };
        let a216 = 1u64;
        nat_eq(a215, a216)
    } else {
        false
    } {
        let a218 = match v11.clone() {
            Adt2::C0(_, _, _, _, h217, _, _, _, _, _, _) => {
                h217
            }
        };
        let a219 = 1u64;
        nat_eq(a218, a219)
    } else {
        false
    } {
        let a221 = match v9.clone() {
            Adt2::C0(_, _, h220, _, _, _, _, _, _, _, _) => {
                h220
            }
        };
        let a222 = 1u64;
        nat_eq(a221, a222)
    } else {
        false
    } {
        let a224 = match v9.clone() {
            Adt2::C0(_, _, _, h223, _, _, _, _, _, _, _) => {
                h223
            }
        };
        let a225 = 1u64;
        nat_eq(a224, a225)
    } else {
        false
    } {
        let a227 = match v9.clone() {
            Adt2::C0(_, _, _, _, h226, _, _, _, _, _, _) => {
                h226
            }
        };
        let a228 = 1u64;
        nat_eq(a227, a228)
    } else {
        false
    } {
        let a230 = 3u64;
        let a231 = match v9.clone() {
            Adt2::C0(_, h229, _, _, _, _, _, _, _, _, _) => {
                h229
            }
        };
        nat_le(a230, a231)
    } else {
        false
    } {
        let a233 = match v5.clone() {
            Adt2::C0(_, h232, _, _, _, _, _, _, _, _, _) => {
                h232
            }
        };
        let a234 = 4u64;
        nat_eq(a233, a234)
    } else {
        false
    } {
        let a236 = match v5.clone() {
            Adt2::C0(_, _, h235, _, _, _, _, _, _, _, _) => {
                h235
            }
        };
        let a237 = 4u64;
        nat_eq(a236, a237)
    } else {
        false
    } {
        let a240 = match v5.clone() {
            Adt2::C0(_, _, _, h238, _, _, _, _, _, _, _) => {
                h238
            }
        };
        let a241 = match v7.clone() {
            Adt2::C0(_, _, h239, _, _, _, _, _, _, _, _) => {
                h239
            }
        };
        nat_eq(a240, a241)
    } else {
        false
    } {
        let a243 = match v5.clone() {
            Adt2::C0(_, _, _, _, h242, _, _, _, _, _, _) => {
                h242
            }
        };
        let a244 = 1u64;
        nat_eq(a243, a244)
    } else {
        false
    } {
        f31(v5.clone())?
    } else {
        false
    } {
        let a245 = 0u64;
        let a246 = v12;
        nat_lt(a245, a246)
    } else {
        false
    } {
        let a247 = v12;
        let a248 = 2147483648u64;
        nat_lt(a247, a248)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a250 = match v7.clone() {
                Adt2::C0(_, _, h249, _, _, _, _, _, _, _, _) => {
                    h249
                }
            };
            let a251 = f32(v12)?;
            nat_mul(a250, a251)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C38(f25(v9, 1u64), f25(v11, 1u64), match v0 {
            Adt1::C0(h252, _, _, _, _, _, _, _, _, _, _, _) => {
                h252
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m253 = v0.clone();
        match m253 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m254 = v1.clone();
        match m254 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m255 = v2.clone();
        match m255 {
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
        let m256 = v0;
        match m256 {
            None => {
                false
            }
            Some(_) => {
                let m257 = v1;
                match m257 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m258 = v2;
                        match m258 {
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
        let a260 = match v7.clone() {
            Adt2::C0(h259, _, _, _, _, _, _, _, _, _, _) => {
                h259
            }
        };
        let a261 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a260, a261) {
            let a263 = match v7.clone() {
                Adt2::C0(_, _, _, _, h262, _, _, _, _, _, _) => {
                    h262
                }
            };
            let a264 = 1u64;
            nat_eq(a263, a264)
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
            let a266 = match v9.clone() {
                Adt2::C0(h265, _, _, _, _, _, _, _, _, _, _) => {
                    h265
                }
            };
            let a267 = 0u64;
            if if if nat_eq(a266, a267) {
                let a269 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h268, _, _, _, _, _, _) => {
                        h268
                    }
                };
                let a270 = 1u64;
                nat_eq(a269, a270)
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
            let a272 = match v11.clone() {
                Adt2::C0(h271, _, _, _, _, _, _, _, _, _, _) => {
                    h271
                }
            };
            let a273 = 0u64;
            if if if nat_eq(a272, a273) {
                let a275 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h274, _, _, _, _, _, _) => {
                        h274
                    }
                };
                let a276 = 1u64;
                nat_eq(a275, a276)
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
            let a278 = match v7.clone() {
                Adt2::C0(_, _, _, h277, _, _, _, _, _, _, _) => {
                    h277
                }
            };
            let a279 = 1u64;
            nat_eq(a278, a279)
        } else {
            false
        } {
            let a281 = match v11.clone() {
                Adt2::C0(_, _, _, h280, _, _, _, _, _, _, _) => {
                    h280
                }
            };
            let a282 = 1u64;
            nat_eq(a281, a282)
        } else {
            false
        } {
            let a285 = match v9.clone() {
                Adt2::C0(_, h283, _, _, _, _, _, _, _, _, _) => {
                    h283
                }
            };
            let a286 = match v7.clone() {
                Adt2::C0(_, h284, _, _, _, _, _, _, _, _, _) => {
                    h284
                }
            };
            nat_eq(a285, a286)
        } else {
            false
        } {
            let a289 = match v9.clone() {
                Adt2::C0(_, _, _, h287, _, _, _, _, _, _, _) => {
                    h287
                }
            };
            let a290 = match v7.clone() {
                Adt2::C0(_, _, h288, _, _, _, _, _, _, _, _) => {
                    h288
                }
            };
            nat_eq(a289, a290)
        } else {
            false
        } {
            let a293 = match v11.clone() {
                Adt2::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                    h291
                }
            };
            let a294 = match v9.clone() {
                Adt2::C0(_, _, h292, _, _, _, _, _, _, _, _) => {
                    h292
                }
            };
            nat_eq(a293, a294)
        } else {
            false
        } {
            let a297 = match v11.clone() {
                Adt2::C0(_, _, h295, _, _, _, _, _, _, _, _) => {
                    h295
                }
            };
            let a298 = match v7.clone() {
                Adt2::C0(_, _, h296, _, _, _, _, _, _, _, _) => {
                    h296
                }
            };
            nat_eq(a297, a298)
        } else {
            false
        } {
            let a300 = 0u64;
            let a301 = match v9.clone() {
                Adt2::C0(_, _, h299, _, _, _, _, _, _, _, _) => {
                    h299
                }
            };
            if nat_lt(a300, a301) {
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
            let m302 = v3.clone();
            match m302 {
                None => {
                    true
                }
                Some(v15) => {
                    let a304 = match v15.clone() {
                        Adt2::C0(h303, _, _, _, _, _, _, _, _, _, _) => {
                            h303
                        }
                    };
                    let a305 = 0u64;
                    if if if if if if nat_eq(a304, a305) {
                        let a307 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h306, _, _, _, _, _, _) => {
                                h306
                            }
                        };
                        let a308 = 1u64;
                        nat_eq(a307, a308)
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
                        let a311 = match v15.clone() {
                            Adt2::C0(_, h309, _, _, _, _, _, _, _, _, _) => {
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
                        let a315 = match v15.clone() {
                            Adt2::C0(_, _, h313, _, _, _, _, _, _, _, _) => {
                                h313
                            }
                        };
                        let a316 = match v9.clone() {
                            Adt2::C0(_, _, h314, _, _, _, _, _, _, _, _) => {
                                h314
                            }
                        };
                        nat_eq(a315, a316)
                    } else {
                        false
                    } {
                        let a319 = match v15 {
                            Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                                h317
                            }
                        };
                        let a320 = match v7.clone() {
                            Adt2::C0(_, _, h318, _, _, _, _, _, _, _, _) => {
                                h318
                            }
                        };
                        nat_eq(a319, a320)
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
            let m321 = v3;
            match m321 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f25(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a323 = f23(v5.clone(), 1u64)?;
            let a324 = f33(match v9.clone() {
                Adt2::C0(_, _, h322, _, _, _, _, _, _, _, _) => {
                    h322
                }
            }, v17.clone())?;
            nat_mul(a323, a324)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C37(f25(v9, 1u64), f25(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f23(v1.clone(), 1u64)?;
    let a326 = match v1.clone() {
        Adt2::C0(h325, _, _, _, _, _, _, _, _, _, _) => {
            h325
        }
    };
    let a327 = 0u64;
    if if if if if if if if nat_eq(a326, a327) {
        let a329 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h328, _, _, _, _, _) => {
                h328
            }
        };
        let a330 = 4u64;
        nat_eq(a329, a330)
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(h331, _, _, _, _, _, _, _, _, _, _, _) => {
                h331
            }
        })
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(_, h332, _, _, _, _, _, _, _, _, _, _) => {
                h332
            }
        })
    } else {
        false
    } {
        let a334 = 0u64;
        let a335 = match v0.clone() {
            Adt1::C0(_, _, h333, _, _, _, _, _, _, _, _, _) => {
                h333
            }
        };
        nat_lt(a334, a335)
    } else {
        false
    } {
        let a337 = match v0.clone() {
            Adt1::C0(_, _, h336, _, _, _, _, _, _, _, _, _) => {
                h336
            }
        };
        let a338 = 2139095040u64;
        nat_lt(a337, a338)
    } else {
        false
    } {
        let a339 = v2;
        let a340 = 9223372036854775808u64;
        nat_lt(a339, a340)
    } else {
        false
    } {
        let a342 = f35(v2)?;
        let a343 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h341, _) => {
                h341
            }
        };
        nat_le(a342, a343)
    } else {
        false
    } {
        Ok(Adt10::C4(f36(v2)?, 4u64, f25(v1.clone(), 1u64), f25(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h344, _, _, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        }, match v0.clone() {
            Adt1::C0(_, h345, _, _, _, _, _, _, _, _, _, _) => {
                h345
            }
        }, match v0 {
            Adt1::C0(_, _, h346, _, _, _, _, _, _, _, _, _) => {
                h346
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m347 = v3;
    match m347 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f37(v0, v2.clone());
            let v7: Adt3 = f37(v0, v5.clone());
            let v8: Adt3 = f37(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h348, _, _, _, _, _, _, _) => {
                    h348
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h349, _, _, _, _, _, _) => {
                    h349
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h350, _, _, _, _, _) => {
                    h350
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h351, _, _, _, _) => {
                    h351
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h352, _, _, _, _, _, _, _) => {
                    h352
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h353, _, _, _, _, _, _) => {
                    h353
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h354, _, _, _, _, _) => {
                    h354
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h355, _, _, _, _, _, _, _, _, _, _, _) => {
                    h355
                }
            };
            let v17: u64 = {
                let a360 = v13;
                let a361 = {
                    let a358 = {
                        let a356 = v9;
                        let a357 = 1u64;
                        nat_sub(a356, a357)
                    };
                    let a359 = v16;
                    nat_mul(a358, a359)?
                };
                nat_add(a360, a361)?
            };
            let v18: u64 = {
                let a366 = v14;
                let a367 = {
                    let a364 = {
                        let a362 = v10;
                        let a363 = 1u64;
                        nat_sub(a362, a363)
                    };
                    let a365 = v16;
                    nat_mul(a364, a365)?
                };
                nat_add(a366, a367)?
            };
            let v19: u64 = {
                let a370 = {
                    let a368 = v9;
                    let a369 = v10;
                    nat_mul(a368, a369)?
                };
                let a371 = v12;
                nat_mul(a370, a371)?
            };
            let v20: u64 = {
                let a374 = {
                    let a372 = v13;
                    let a373 = v14;
                    nat_mul(a372, a373)?
                };
                let a375 = v15;
                nat_mul(a374, a375)?
            };
            let v21: u64 = {
                let a376 = v19;
                let a377 = 32u64;
                if nat_lt(a376, a377) {
                    v19
                } else {
                    32u64
                }
            };
            let a379 = match v2.clone() {
                Adt2::C0(h378, _, _, _, _, _, _, _, _, _, _) => {
                    h378
                }
            };
            let a380 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a379, a380) {
                true
            } else {
                let a382 = match v2.clone() {
                    Adt2::C0(h381, _, _, _, _, _, _, _, _, _, _) => {
                        h381
                    }
                };
                let a383 = 1u64;
                nat_eq(a382, a383)
            } {
                let a385 = match v5.clone() {
                    Adt2::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
                        h384
                    }
                };
                let a386 = 0u64;
                nat_eq(a385, a386)
            } else {
                false
            } {
                let a388 = match v4.clone() {
                    Adt2::C0(h387, _, _, _, _, _, _, _, _, _, _) => {
                        h387
                    }
                };
                let a389 = 0u64;
                nat_eq(a388, a389)
            } else {
                false
            } {
                let a390 = v16;
                let a391 = 2147483648u64;
                nat_lt(a390, a391)
            } else {
                false
            } {
                let a392 = 0u64;
                let a393 = v9;
                nat_lt(a392, a393)
            } else {
                false
            } {
                let a394 = 0u64;
                let a395 = v10;
                nat_lt(a394, a395)
            } else {
                false
            } {
                let a396 = 0u64;
                let a397 = v11;
                nat_lt(a396, a397)
            } else {
                false
            } {
                let a398 = 0u64;
                let a399 = v12;
                nat_lt(a398, a399)
            } else {
                false
            } {
                let a400 = 0u64;
                let a401 = v13;
                nat_lt(a400, a401)
            } else {
                false
            } {
                let a402 = 0u64;
                let a403 = v14;
                nat_lt(a402, a403)
            } else {
                false
            } {
                let a404 = 0u64;
                let a405 = v15;
                nat_lt(a404, a405)
            } else {
                false
            } {
                let a406 = 0u64;
                let a407 = v16;
                nat_lt(a406, a407)
            } else {
                false
            } {
                let a409 = match v6.clone() {
                    Adt3::C0(_, _, _, h408, _, _, _, _) => {
                        h408
                    }
                };
                let a410 = v11;
                nat_eq(a409, a410)
            } else {
                false
            } {
                let a412 = match v8.clone() {
                    Adt3::C0(h411, _, _, _, _, _, _, _) => {
                        h411
                    }
                };
                let a413 = v17;
                nat_eq(a412, a413)
            } else {
                false
            } {
                let a415 = match v8.clone() {
                    Adt3::C0(_, h414, _, _, _, _, _, _) => {
                        h414
                    }
                };
                let a416 = v18;
                nat_eq(a415, a416)
            } else {
                false
            } {
                let a418 = match v8.clone() {
                    Adt3::C0(_, _, h417, _, _, _, _, _) => {
                        h417
                    }
                };
                let a419 = v15;
                nat_eq(a418, a419)
            } else {
                false
            } {
                let a421 = match v8.clone() {
                    Adt3::C0(_, _, _, h420, _, _, _, _) => {
                        h420
                    }
                };
                let a422 = v12;
                nat_eq(a421, a422)
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a427 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h423, _, _, _, _, _) => {
                        h423
                    }
                };
                let a428 = {
                    let a425 = match v2.clone() {
                        Adt2::C0(h424, _, _, _, _, _, _, _, _, _, _) => {
                            h424
                        }
                    };
                    let a426 = 1u64;
                    if nat_eq(a425, a426) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a427, a428)
            } else {
                false
            } {
                let a430 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h429, _, _, _, _, _) => {
                        h429
                    }
                };
                let a431 = 4u64;
                nat_eq(a430, a431)
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
                    let a436 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h432, _, _, _, _) => {
                            h432
                        }
                    };
                    let a437 = {
                        let a434 = v13;
                        let a435 = match v2 {
                            Adt2::C0(_, _, _, _, _, h433, _, _, _, _, _) => {
                                h433
                            }
                        };
                        nat_mul(a434, a435)?
                    };
                    nat_eq(a436, a437)
                } else {
                    let a439 = match v1.clone() {
                        Adt1::C0(_, h438, _, _, _, _, _, _, _, _, _, _) => {
                            h438
                        }
                    };
                    let a440 = 0u64;
                    if if if if if if nat_eq(a439, a440) {
                        let a442 = match v1 {
                            Adt1::C0(_, _, h441, _, _, _, _, _, _, _, _, _) => {
                                h441
                            }
                        };
                        let a443 = 1u64;
                        nat_eq(a442, a443)
                    } else {
                        false
                    } {
                        let a445 = match v2 {
                            Adt2::C0(_, _, _, _, h444, _, _, _, _, _, _) => {
                                h444
                            }
                        };
                        let a446 = 1u64;
                        nat_eq(a445, a446)
                    } else {
                        false
                    } {
                        let a448 = match v5.clone() {
                            Adt2::C0(_, _, _, h447, _, _, _, _, _, _, _) => {
                                h447
                            }
                        };
                        let a449 = 1u64;
                        nat_eq(a448, a449)
                    } else {
                        false
                    } {
                        let a451 = match v5 {
                            Adt2::C0(_, _, _, _, h450, _, _, _, _, _, _) => {
                                h450
                            }
                        };
                        let a452 = 1u64;
                        nat_eq(a451, a452)
                    } else {
                        false
                    } {
                        let a454 = match v4.clone() {
                            Adt2::C0(_, _, _, h453, _, _, _, _, _, _, _) => {
                                h453
                            }
                        };
                        let a455 = 1u64;
                        nat_eq(a454, a455)
                    } else {
                        false
                    } {
                        let a457 = match v4 {
                            Adt2::C0(_, _, _, _, h456, _, _, _, _, _, _) => {
                                h456
                            }
                        };
                        let a458 = 1u64;
                        nat_eq(a457, a458)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a459 = v11;
                    let a460 = v20;
                    nat_mul(a459, a460)?
                }, v21, {
                    let a461 = v21;
                    let a462 = v11;
                    nat_mul(a461, a462)?
                }, {
                    let a463 = v21;
                    let a464 = v20;
                    nat_mul(a463, a464)?
                }, {
                    let a469 = {
                        let a465 = v17;
                        let a466 = v18;
                        nat_mul(a465, a466)?
                    };
                    let a470 = {
                        let a467 = v15;
                        let a468 = v12;
                        nat_mul(a467, a468)?
                    };
                    nat_mul(a469, a470)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f8(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m471 = v3;
    match m471 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h472, _, _, _, _, _, _, _, _, _) => {
                    h472
                }
            }, match v1.clone() {
                Adt1::C0(h473, _, _, _, _, _, _, _, _, _, _, _) => {
                    h473
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h474, _, _, _, _, _, _, _, _) => {
                        h474
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h475, _, _, _, _, _, _, _, _, _) => {
                        h475
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h476, _, _, _, _, _) => {
                        h476
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h477, _, _, _, _, _, _, _) => {
                        h477
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h478, _, _, _, _, _, _, _, _) => {
                    h478
                }
            }, match v1.clone() {
                Adt1::C0(_, h479, _, _, _, _, _, _, _, _, _, _) => {
                    h479
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h480, _, _, _, _, _, _, _) => {
                        h480
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h481, _, _, _, _, _, _, _, _) => {
                        h481
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h482, _, _, _, _) => {
                        h482
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h483, _, _, _, _, _, _) => {
                        h483
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h484, _, _, _, _, _, _, _) => {
                        h484
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h485, _, _, _, _, _, _, _, _, _) => {
                        h485
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h486, _, _, _, _, _, _) => {
                        h486
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h487, _, _, _) => {
                        h487
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f39(v6.clone(), match v5.clone() {
                Adt2::C0(_, h488, _, _, _, _, _, _, _, _, _) => {
                    h488
                }
            })?;
            let v10: u64 = f39(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h489, _, _, _, _, _, _, _, _) => {
                    h489
                }
            })?;
            let v11: u64 = if v0 {
                f39(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h490, _, _, _, _, _, _, _) => {
                        h490
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h491, _, _) => {
                        h491
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h492, _, _, _, _, _, _, _) => {
                        h492
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h493, _) => {
                        h493
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h494, _, _, _, _, _, _) => {
                        h494
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h495) => {
                        h495
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h496, _, _, _, _, _, _) => {
                        h496
                    }
                }
            };
            let v15: u64 = {
                let a501 = {
                    let a497 = v9;
                    let a498 = v10;
                    nat_mul(a497, a498)?
                };
                let a502 = {
                    let a499 = v11;
                    let a500 = v13;
                    nat_mul(a499, a500)?
                };
                nat_mul(a501, a502)?
            };
            let v16: u64 = {
                let a510 = {
                    let a505 = match v6.clone() {
                        Adt6::C0(h503, _, _, _) => {
                            h503
                        }
                    };
                    let a506 = match v7.clone() {
                        Adt6::C0(h504, _, _, _) => {
                            h504
                        }
                    };
                    nat_mul(a505, a506)?
                };
                let a511 = {
                    let a508 = match v8.clone() {
                        Adt6::C0(h507, _, _, _) => {
                            h507
                        }
                    };
                    let a509 = v12;
                    nat_mul(a508, a509)?
                };
                nat_mul(a510, a511)?
            };
            let v17: u64 = {
                let a512 = v15;
                let a513 = 32u64;
                if nat_lt(a512, a513) {
                    v15
                } else {
                    32u64
                }
            };
            let a515 = match v2.clone() {
                Adt2::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
                    h514
                }
            };
            let a516 = 0u64;
            if if if if if if if if if if if nat_eq(a515, a516) {
                true
            } else {
                let a518 = match v2.clone() {
                    Adt2::C0(h517, _, _, _, _, _, _, _, _, _, _) => {
                        h517
                    }
                };
                let a519 = 1u64;
                nat_eq(a518, a519)
            } {
                let a521 = match v5.clone() {
                    Adt2::C0(h520, _, _, _, _, _, _, _, _, _, _) => {
                        h520
                    }
                };
                let a522 = 0u64;
                nat_eq(a521, a522)
            } else {
                false
            } {
                let a524 = match v4.clone() {
                    Adt2::C0(h523, _, _, _, _, _, _, _, _, _, _) => {
                        h523
                    }
                };
                let a525 = 0u64;
                nat_eq(a524, a525)
            } else {
                false
            } {
                let a527 = match v4.clone() {
                    Adt2::C0(_, h526, _, _, _, _, _, _, _, _, _) => {
                        h526
                    }
                };
                let a528 = v9;
                nat_eq(a527, a528)
            } else {
                false
            } {
                let a530 = match v4.clone() {
                    Adt2::C0(_, _, h529, _, _, _, _, _, _, _, _) => {
                        h529
                    }
                };
                let a531 = v10;
                nat_eq(a530, a531)
            } else {
                false
            } {
                if v0 {
                    let a532 = 0u64;
                    let a533 = v12;
                    if if if if if if if nat_lt(a532, a533) {
                        let a534 = v12;
                        let a535 = 2147483648u64;
                        nat_lt(a534, a535)
                    } else {
                        false
                    } {
                        let a536 = v14;
                        let a537 = 2147483648u64;
                        nat_lt(a536, a537)
                    } else {
                        false
                    } {
                        let a538 = v13;
                        let a539 = 2147483648u64;
                        nat_lt(a538, a539)
                    } else {
                        false
                    } {
                        let a543 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h540, _, _, _, _, _, _) => {
                                h540
                            }
                        };
                        let a544 = {
                            let a541 = v12;
                            let a542 = v14;
                            nat_mul(a541, a542)?
                        };
                        nat_eq(a543, a544)
                    } else {
                        false
                    } {
                        let a548 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h545, _, _, _, _, _, _) => {
                                h545
                            }
                        };
                        let a549 = {
                            let a546 = v12;
                            let a547 = v13;
                            nat_mul(a546, a547)?
                        };
                        nat_eq(a548, a549)
                    } else {
                        false
                    } {
                        let a551 = match v4.clone() {
                            Adt2::C0(_, _, _, h550, _, _, _, _, _, _, _) => {
                                h550
                            }
                        };
                        let a552 = v11;
                        nat_eq(a551, a552)
                    } else {
                        false
                    } {
                        let a556 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h553, _, _, _, _, _, _) => {
                                h553
                            }
                        };
                        let a557 = {
                            let a554 = v14;
                            let a555 = v13;
                            nat_mul(a554, a555)?
                        };
                        nat_eq(a556, a557)
                    } else {
                        false
                    }
                } else {
                    let a559 = match v2.clone() {
                        Adt2::C0(_, _, _, h558, _, _, _, _, _, _, _) => {
                            h558
                        }
                    };
                    let a560 = v12;
                    if if if nat_eq(a559, a560) {
                        let a562 = match v4.clone() {
                            Adt2::C0(_, _, _, h561, _, _, _, _, _, _, _) => {
                                h561
                            }
                        };
                        let a563 = v14;
                        nat_eq(a562, a563)
                    } else {
                        false
                    } {
                        let a565 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h564, _, _, _, _, _, _) => {
                                h564
                            }
                        };
                        let a566 = v13;
                        nat_eq(a565, a566)
                    } else {
                        false
                    } {
                        let a567 = 0u64;
                        let a568 = v16;
                        nat_lt(a567, a568)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a569 = 0u64;
                let a570 = v14;
                nat_lt(a569, a570)
            } else {
                false
            } {
                let a571 = 0u64;
                let a572 = v15;
                nat_lt(a571, a572)
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
                    let a573 = v16;
                    let a574 = v14;
                    nat_mul(a573, a574)?
                }, v17, {
                    let a575 = v17;
                    let a576 = v16;
                    nat_mul(a575, a576)?
                }, {
                    let a577 = v17;
                    let a578 = v14;
                    nat_mul(a577, a578)?
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
    let m579 = v2;
    match m579 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m580 = v3;
            match m580 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a582 = match v1.clone() {
                        Adt2::C0(h581, _, _, _, _, _, _, _, _, _, _) => {
                            h581
                        }
                    };
                    let a583 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a582, a583) {
                        let a585 = match v5.clone() {
                            Adt2::C0(h584, _, _, _, _, _, _, _, _, _, _) => {
                                h584
                            }
                        };
                        let a586 = 0u64;
                        nat_eq(a585, a586)
                    } else {
                        false
                    } {
                        let a588 = match v6.clone() {
                            Adt2::C0(h587, _, _, _, _, _, _, _, _, _, _) => {
                                h587
                            }
                        };
                        let a589 = 0u64;
                        nat_eq(a588, a589)
                    } else {
                        false
                    } {
                        let a591 = match v4.clone() {
                            Adt2::C0(h590, _, _, _, _, _, _, _, _, _, _) => {
                                h590
                            }
                        };
                        let a592 = 0u64;
                        nat_eq(a591, a592)
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
                        let a598 = match v1.clone() {
                            Adt2::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
                                h593
                            }
                        };
                        let a599 = {
                            let a596 = match v5.clone() {
                                Adt2::C0(_, h594, _, _, _, _, _, _, _, _, _) => {
                                    h594
                                }
                            };
                            let a597 = match v5.clone() {
                                Adt2::C0(_, h595, _, _, _, _, _, _, _, _, _) => {
                                    h595
                                }
                            };
                            nat_mul(a596, a597)?
                        };
                        nat_eq(a598, a599)
                    } else {
                        false
                    } {
                        let a605 = match v1.clone() {
                            Adt2::C0(_, _, h600, _, _, _, _, _, _, _, _) => {
                                h600
                            }
                        };
                        let a606 = {
                            let a603 = match v5.clone() {
                                Adt2::C0(_, _, h601, _, _, _, _, _, _, _, _) => {
                                    h601
                                }
                            };
                            let a604 = match v5.clone() {
                                Adt2::C0(_, _, _, h602, _, _, _, _, _, _, _) => {
                                    h602
                                }
                            };
                            nat_mul(a603, a604)?
                        };
                        nat_eq(a605, a606)
                    } else {
                        false
                    } {
                        let a609 = match v1.clone() {
                            Adt2::C0(_, _, _, h607, _, _, _, _, _, _, _) => {
                                h607
                            }
                        };
                        let a610 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h608, _, _, _, _, _, _) => {
                                h608
                            }
                        };
                        nat_eq(a609, a610)
                    } else {
                        false
                    } {
                        let a612 = match v0.clone() {
                            Adt1::C0(h611, _, _, _, _, _, _, _, _, _, _, _) => {
                                h611
                            }
                        };
                        let a613 = 4294967296u64;
                        nat_lt(a612, a613)
                    } else {
                        false
                    } {
                        let a615 = match v0.clone() {
                            Adt1::C0(h614, _, _, _, _, _, _, _, _, _, _, _) => {
                                h614
                            }
                        };
                        let a616 = 0u64;
                        if nat_eq(a615, a616) {
                            if f30(v1.clone(), v4.clone())? {
                                true
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
                            let a623 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h621) => {
                                    h621
                                }
                            };
                            let a624 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h622) => {
                                    h622
                                }
                            };
                            nat_eq(a623, a624)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a626 = match v4.clone() {
                                Adt2::C0(_, h625, _, _, _, _, _, _, _, _, _) => {
                                    h625
                                }
                            };
                            let a627 = 0u64;
                            if if if nat_eq(a626, a627) {
                                true
                            } else {
                                let a629 = match v4.clone() {
                                    Adt2::C0(_, _, h628, _, _, _, _, _, _, _, _) => {
                                        h628
                                    }
                                };
                                let a630 = 0u64;
                                nat_eq(a629, a630)
                            } {
                                true
                            } else {
                                let a632 = match v4.clone() {
                                    Adt2::C0(_, _, _, h631, _, _, _, _, _, _, _) => {
                                        h631
                                    }
                                };
                                let a633 = 0u64;
                                nat_eq(a632, a633)
                            } {
                                true
                            } else {
                                let a635 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h634, _, _, _, _, _, _) => {
                                        h634
                                    }
                                };
                                let a636 = 0u64;
                                nat_eq(a635, a636)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h637, _, _, _, _, _, _, _, _, _) => {
                                h637
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a639 = match v0 {
                                Adt1::C0(h638, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h638
                                }
                            };
                            let a640 = 0u64;
                            if nat_eq(a639, a640) {
                                f23(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f40(v8, match v5.clone() {
                                Adt2::C0(_, _, h641, _, _, _, _, _, _, _, _) => {
                                    h641
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h642, _, _, _, _, _, _, _) => {
                                    h642
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h643, _, _, _, _, _, _) => {
                                    h643
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
        let m644 = v3;
        match m644 {
            None => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h645, _, _, _, _, _, _, _, _, _, _, _) => {
                        h645
                    }
                }, match v1 {
                    Adt1::C0(_, h646, _, _, _, _, _, _, _, _, _, _) => {
                        h646
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h647, _, _, _, _, _, _, _, _, _, _, _) => {
                        h647
                    }
                }, match v1 {
                    Adt1::C0(_, h648, _, _, _, _, _, _, _, _, _, _) => {
                        h648
                    }
                }, true, v5, f42(v7), v6)?
            }
        }
    };
    let m649 = v8;
    match m649 {
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
        Adt2::C0(h650, _, _, _, _, _, _, _, _, _, _) => {
            h650
        }
    };
    let v4: u64 = f44(v3);
    let a652 = match v1.clone() {
        Adt2::C0(h651, _, _, _, _, _, _, _, _, _, _) => {
            h651
        }
    };
    let a653 = 0u64;
    if if if if if if if if nat_eq(a652, a653) {
        true
    } else {
        let a655 = match v1.clone() {
            Adt2::C0(h654, _, _, _, _, _, _, _, _, _, _) => {
                h654
            }
        };
        let a656 = 30u64;
        nat_eq(a655, a656)
    } {
        let a659 = match v1.clone() {
            Adt2::C0(h657, _, _, _, _, _, _, _, _, _, _) => {
                h657
            }
        };
        let a660 = match v2.clone() {
            Adt2::C0(h658, _, _, _, _, _, _, _, _, _, _) => {
                h658
            }
        };
        nat_eq(a659, a660)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f45(match v0.clone() {
            Adt1::C0(h661, _, _, _, _, _, _, _, _, _, _, _) => {
                h661
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
            Adt2::C0(_, h662, _, _, _, _, _, _, _, _, _) => {
                h662
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h663, _, _, _, _, _, _, _, _) => {
                h663
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h664, _, _, _, _, _, _, _) => {
                h664
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h665, _, _, _, _, _, _) => {
                h665
            }
        }) {
            0u64
        } else {
            let a671 = match v1.clone() {
                Adt2::C0(_, _, h666, _, _, _, _, _, _, _, _) => {
                    h666
                }
            };
            let a672 = {
                let a669 = match v1.clone() {
                    Adt2::C0(_, _, _, h667, _, _, _, _, _, _, _) => {
                        h667
                    }
                };
                let a670 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h668, _, _, _, _, _, _) => {
                        h668
                    }
                };
                nat_mul(a669, a670)?
            };
            nat_mul(a671, a672)?
        };
        let v6: u64 = {
            let a673 = v5;
            let a674 = 0u64;
            if if nat_eq(a673, a674) {
                true
            } else {
                f49(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a676 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h675, _) => {
                        h675
                    }
                };
                let a677 = v4;
                let a678 = 0u64;
                nat_quot(a676, a677, a678)
            }
        };
        Ok(Adt10::C4(f50(v3, match v1.clone() {
            Adt2::C0(_, h679, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        }, v5, v6)?, v4, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h680, _, _, _, _, _, _, _, _, _, _, _) => {
                h680
            }
        }, v5, f51(match v1 {
            Adt2::C0(_, h681, _, _, _, _, _, _, _, _, _) => {
                h681
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a683 = match v1.clone() {
        Adt2::C0(h682, _, _, _, _, _, _, _, _, _, _) => {
            h682
        }
    };
    let a684 = 0u64;
    if if if if if nat_eq(a683, a684) {
        let a686 = match v2.clone() {
            Adt2::C0(h685, _, _, _, _, _, _, _, _, _, _) => {
                h685
            }
        };
        let a687 = 0u64;
        nat_eq(a686, a687)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f52(match v0.clone() {
            Adt1::C0(h688, _, _, _, _, _, _, _, _, _, _, _) => {
                h688
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
            let a694 = match v3.clone() {
                Adt3::C0(_, h689, _, _, _, _, _, _) => {
                    h689
                }
            };
            let a695 = {
                let a692 = match v3.clone() {
                    Adt3::C0(_, _, h690, _, _, _, _, _) => {
                        h690
                    }
                };
                let a693 = match v3.clone() {
                    Adt3::C0(_, _, _, h691, _, _, _, _) => {
                        h691
                    }
                };
                nat_mul(a692, a693)?
            };
            nat_mul(a694, a695)?
        };
        Ok(Adt10::C4(f53(match v3.clone() {
            Adt3::C0(h696, _, _, _, _, _, _, _) => {
                h696
            }
        }, v4)?, 4u64, v3, f25(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h697, _, _, _, _, _, _, _, _, _, _, _) => {
                h697
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f13(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m698 = v1;
    match m698 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m699 = v2;
            match m699 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a701 = match v0.clone() {
                        Adt2::C0(h700, _, _, _, _, _, _, _, _, _, _) => {
                            h700
                        }
                    };
                    let a702 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a701, a702) {
                        let a704 = match v4.clone() {
                            Adt2::C0(h703, _, _, _, _, _, _, _, _, _, _) => {
                                h703
                            }
                        };
                        let a705 = 0u64;
                        nat_eq(a704, a705)
                    } else {
                        false
                    } {
                        let a707 = match v5.clone() {
                            Adt2::C0(h706, _, _, _, _, _, _, _, _, _, _) => {
                                h706
                            }
                        };
                        let a708 = 26u64;
                        nat_eq(a707, a708)
                    } else {
                        false
                    } {
                        let a710 = match v3.clone() {
                            Adt2::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
                                h709
                            }
                        };
                        let a711 = 0u64;
                        nat_eq(a710, a711)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a714 = match v0.clone() {
                            Adt2::C0(_, h712, _, _, _, _, _, _, _, _, _) => {
                                h712
                            }
                        };
                        let a715 = match v4.clone() {
                            Adt2::C0(_, h713, _, _, _, _, _, _, _, _, _) => {
                                h713
                            }
                        };
                        nat_eq(a714, a715)
                    } else {
                        false
                    } {
                        let a718 = match v0.clone() {
                            Adt2::C0(_, _, h716, _, _, _, _, _, _, _, _) => {
                                h716
                            }
                        };
                        let a719 = match v5.clone() {
                            Adt2::C0(_, h717, _, _, _, _, _, _, _, _, _) => {
                                h717
                            }
                        };
                        nat_eq(a718, a719)
                    } else {
                        false
                    } {
                        let a722 = match v0.clone() {
                            Adt2::C0(_, _, _, h720, _, _, _, _, _, _, _) => {
                                h720
                            }
                        };
                        let a723 = match v5.clone() {
                            Adt2::C0(_, _, h721, _, _, _, _, _, _, _, _) => {
                                h721
                            }
                        };
                        nat_eq(a722, a723)
                    } else {
                        false
                    } {
                        let a725 = 0u64;
                        let a726 = match v5.clone() {
                            Adt2::C0(_, _, _, h724, _, _, _, _, _, _, _) => {
                                h724
                            }
                        };
                        nat_lt(a725, a726)
                    } else {
                        false
                    } {
                        let a728 = 0u64;
                        let a729 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h727, _, _, _, _, _, _) => {
                                h727
                            }
                        };
                        nat_lt(a728, a729)
                    } else {
                        false
                    } {
                        let a731 = 4u64;
                        let a732 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h730, _, _, _, _, _) => {
                                h730
                            }
                        };
                        nat_le(a731, a732)
                    } else {
                        false
                    } {
                        let a734 = 4u64;
                        let a735 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h733, _, _, _, _, _) => {
                                h733
                            }
                        };
                        nat_le(a734, a735)
                    } else {
                        false
                    } {
                        let a737 = 4u64;
                        let a738 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h736, _, _, _, _, _) => {
                                h736
                            }
                        };
                        nat_le(a737, a738)
                    } else {
                        false
                    } {
                        let a740 = 4u64;
                        let a741 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h739, _, _, _, _, _) => {
                                h739
                            }
                        };
                        nat_le(a740, a741)
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
                            Adt2::C0(_, h742, _, _, _, _, _, _, _, _, _) => {
                                h742
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h743, _, _, _, _, _, _, _, _) => {
                                h743
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h744, _, _, _, _, _, _, _) => {
                                h744
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h745, _, _, _, _, _, _) => {
                                h745
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
    let a746 = v0;
    let a747 = 28u64;
    if if if nat_eq(a746, a747) {
        true
    } else {
        let a748 = v0;
        let a749 = 29u64;
        nat_eq(a748, a749)
    } {
        let a751 = 0u64;
        let a752 = match v1.clone() {
            Adt1::C0(h750, _, _, _, _, _, _, _, _, _, _, _) => {
                h750
            }
        };
        if nat_lt(a751, a752) {
            true
        } else {
            let a754 = 0u64;
            let a755 = match v1.clone() {
                Adt1::C0(_, _, _, h753, _, _, _, _, _, _, _, _) => {
                    h753
                }
            };
            nat_lt(a754, a755)
        }
    } else {
        false
    } {
        let m759 = f55(match v2.clone() {
            Adt2::C0(h756, _, _, _, _, _, _, _, _, _, _) => {
                h756
            }
        }, match v1.clone() {
            Adt1::C0(h757, _, _, _, _, _, _, _, _, _, _, _) => {
                h757
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h758, _, _, _, _, _, _, _, _) => {
                h758
            }
        });
        match m759 {
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
        Adt8::C0(_, h760, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h760
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h761, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h761
        }
    };
    let v6: u64 = {
        let a762 = v3;
        let a763 = v5;
        let a764 = 0u64;
        nat_rem(a762, a763, a764)
    };
    let a765 = v1;
    let a766 = 0u64;
    if nat_eq(a765, a766) {
        let a768 = v3;
        let a769 = match v0.clone() {
            Adt8::C0(_, _, _, h767, _, _, _, _, _, _, _, _, _, _, _) => {
                h767
            }
        };
        if nat_lt(a768, a769) {
            Ok(Adt9::C8(Some(f57(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h770, _, _, _, _, _, _) => {
                    h770
                }
            }, {
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
                nat_rem(a774, a775, a776)
            }, {
                let a786 = {
                    let a780 = v6;
                    let a781 = {
                        let m779 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h778, _, _, _, _, _, _) => {
                                h778
                            }
                        };
                        match m779 {
                            Adt3::C0(h777, _, _, _, _, _, _, _) => {
                                h777
                            }
                        }
                    };
                    let a782 = 0u64;
                    nat_quot(a780, a781, a782)
                };
                let a787 = {
                    let m785 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h784, _, _, _, _, _, _) => {
                            h784
                        }
                    };
                    match m785 {
                        Adt3::C0(_, h783, _, _, _, _, _, _) => {
                            h783
                        }
                    }
                };
                let a788 = 0u64;
                nat_rem(a786, a787, a788)
            }, {
                let a798 = {
                    let a792 = v6;
                    let a793 = {
                        let m791 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h790, _, _, _, _, _, _) => {
                                h790
                            }
                        };
                        match m791 {
                            Adt3::C0(h789, _, _, _, _, _, _, _) => {
                                h789
                            }
                        }
                    };
                    let a794 = 0u64;
                    nat_quot(a792, a793, a794)
                };
                let a799 = {
                    let m797 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h796, _, _, _, _, _, _) => {
                            h796
                        }
                    };
                    match m797 {
                        Adt3::C0(_, h795, _, _, _, _, _, _) => {
                            h795
                        }
                    }
                };
                let a800 = 0u64;
                nat_quot(a798, a799, a800)
            }, {
                let a801 = v3;
                let a802 = v5;
                let a803 = 0u64;
                nat_quot(a801, a802, a803)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a809 = v2;
            let a810 = {
                let a806 = v3;
                let a807 = {
                    let a804 = v1;
                    let a805 = 1u64;
                    if nat_eq(a804, a805) {
                        v4
                    } else {
                        v5
                    }
                };
                let a808 = 0u64;
                nat_quot(a806, a807, a808)
            };
            nat_add(a809, a810)?
        };
        let a812 = v7;
        let a813 = match v0.clone() {
            Adt8::C0(h811, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h811
            }
        };
        if nat_lt(a812, a813) {
            let v8: u64 = {
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
                nat_rem(a817, a818, a819)
            };
            let v9: u64 = {
                let a823 = v7;
                let a824 = {
                    let m822 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h821, _, _, _, _, _) => {
                            h821
                        }
                    };
                    match m822 {
                        Adt3::C0(h820, _, _, _, _, _, _, _) => {
                            h820
                        }
                    }
                };
                let a825 = 0u64;
                nat_quot(a823, a824, a825)
            };
            let v10: u64 = {
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
                nat_rem(a829, a830, a831)
            };
            let v11: u64 = {
                let a835 = v9;
                let a836 = {
                    let m834 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h833, _, _, _, _, _) => {
                            h833
                        }
                    };
                    match m834 {
                        Adt3::C0(_, h832, _, _, _, _, _, _) => {
                            h832
                        }
                    }
                };
                let a837 = 0u64;
                nat_quot(a835, a836, a837)
            };
            let a838 = v1;
            let a839 = 1u64;
            if nat_eq(a838, a839) {
                Ok(Adt9::C8(Some(f57(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h840, _, _, _, _, _) => {
                        h840
                    }
                }, v8, v10, {
                    let a841 = v3;
                    let a842 = v4;
                    let a843 = 0u64;
                    nat_rem(a841, a842, a843)
                }, v11)?)))
            } else {
                let a844 = v1;
                let a845 = 2u64;
                if nat_eq(a844, a845) {
                    Ok(Adt9::C8(Some({
                        let a891 = f57(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h846, _, _, _, _) => {
                                h846
                            }
                        }, {
                            let a858 = {
                                let a850 = v8;
                                let a851 = {
                                    let m849 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h848, _, _, _) => {
                                            h848
                                        }
                                    };
                                    match m849 {
                                        Adt6::C0(_, h847, _, _) => {
                                            h847
                                        }
                                    }
                                };
                                nat_mul(a850, a851)?
                            };
                            let a859 = {
                                let a855 = v6;
                                let a856 = {
                                    let m854 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h853, _, _, _, _, _, _) => {
                                            h853
                                        }
                                    };
                                    match m854 {
                                        Adt3::C0(h852, _, _, _, _, _, _, _) => {
                                            h852
                                        }
                                    }
                                };
                                let a857 = 0u64;
                                nat_rem(a855, a856, a857)
                            };
                            nat_add(a858, a859)?
                        }, {
                            let a877 = {
                                let a863 = v10;
                                let a864 = {
                                    let m862 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h861, _, _, _) => {
                                            h861
                                        }
                                    };
                                    match m862 {
                                        Adt6::C0(_, h860, _, _) => {
                                            h860
                                        }
                                    }
                                };
                                nat_mul(a863, a864)?
                            };
                            let a878 = {
                                let a874 = {
                                    let a868 = v6;
                                    let a869 = {
                                        let m867 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h866, _, _, _, _, _, _) => {
                                                h866
                                            }
                                        };
                                        match m867 {
                                            Adt3::C0(h865, _, _, _, _, _, _, _) => {
                                                h865
                                            }
                                        }
                                    };
                                    let a870 = 0u64;
                                    nat_quot(a868, a869, a870)
                                };
                                let a875 = {
                                    let m873 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h872, _, _, _, _, _, _) => {
                                            h872
                                        }
                                    };
                                    match m873 {
                                        Adt3::C0(_, h871, _, _, _, _, _, _) => {
                                            h871
                                        }
                                    }
                                };
                                let a876 = 0u64;
                                nat_rem(a874, a875, a876)
                            };
                            nat_add(a877, a878)?
                        }, {
                            let a888 = {
                                let a882 = v6;
                                let a883 = {
                                    let m881 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h880, _, _, _, _, _, _) => {
                                            h880
                                        }
                                    };
                                    match m881 {
                                        Adt3::C0(h879, _, _, _, _, _, _, _) => {
                                            h879
                                        }
                                    }
                                };
                                let a884 = 0u64;
                                nat_quot(a882, a883, a884)
                            };
                            let a889 = {
                                let m887 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h886, _, _, _, _, _, _) => {
                                        h886
                                    }
                                };
                                match m887 {
                                    Adt3::C0(_, h885, _, _, _, _, _, _) => {
                                        h885
                                    }
                                }
                            };
                            let a890 = 0u64;
                            nat_quot(a888, a889, a890)
                        }, v11)?;
                        let a892 = 4u64;
                        let a893 = 0u64;
                        nat_quot(a891, a892, a893)
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
    let a894 = v1;
    let a895 = 0u64;
    if nat_eq(a894, a895) {
        let a897 = v3;
        let a898 = match v0.clone() {
            Adt8::C0(_, _, _, h896, _, _, _, _, _, _, _, _, _, _, _) => {
                h896
            }
        };
        if nat_lt(a897, a898) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h899, _, _, _, _, _, _) => {
                    h899
                }
            };
            let v5: u64 = {
                let a901 = v3;
                let a902 = match v0.clone() {
                    Adt8::C0(_, _, h900, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h900
                    }
                };
                let a903 = 0u64;
                nat_quot(a901, a902, a903)
            };
            Ok(Adt9::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), f59(v0.clone(), v5, {
                let a905 = v3;
                let a906 = match v0 {
                    Adt8::C0(_, _, h904, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h904
                    }
                };
                let a907 = 0u64;
                nat_rem(a905, a906, a907)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h908, _, _, _, _) => {
                h908
            }
        };
        let v7: u64 = {
            let a909 = v1;
            let a910 = 1u64;
            if nat_eq(a909, a910) {
                match v0.clone() {
                    Adt8::C0(_, h911, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h911
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h912, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h912
                    }
                }
            }
        };
        let v8: u64 = {
            let a916 = v2;
            let a917 = {
                let a913 = v3;
                let a914 = v7;
                let a915 = 0u64;
                nat_quot(a913, a914, a915)
            };
            nat_add(a916, a917)?
        };
        let v9: u64 = {
            let a918 = v3;
            let a919 = v7;
            let a920 = 0u64;
            nat_rem(a918, a919, a920)
        };
        let v10: u64 = {
            let a922 = v8;
            let a923 = match v6.clone() {
                Adt3::C0(h921, _, _, _, _, _, _, _) => {
                    h921
                }
            };
            let a924 = 0u64;
            nat_rem(a922, a923, a924)
        };
        let v11: u64 = {
            let a926 = v8;
            let a927 = match v6.clone() {
                Adt3::C0(h925, _, _, _, _, _, _, _) => {
                    h925
                }
            };
            let a928 = 0u64;
            nat_quot(a926, a927, a928)
        };
        let v12: u64 = {
            let a930 = v11;
            let a931 = match v6.clone() {
                Adt3::C0(_, h929, _, _, _, _, _, _) => {
                    h929
                }
            };
            let a932 = 0u64;
            nat_rem(a930, a931, a932)
        };
        let v13: u64 = {
            let a934 = v11;
            let a935 = match v6.clone() {
                Adt3::C0(_, h933, _, _, _, _, _, _) => {
                    h933
                }
            };
            let a936 = 0u64;
            nat_quot(a934, a935, a936)
        };
        let a938 = v8;
        let a939 = match v0.clone() {
            Adt8::C0(h937, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h937
            }
        };
        if nat_lt(a938, a939) {
            let a940 = v1;
            let a941 = 1u64;
            if nat_eq(a940, a941) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h942, _, _, _, _, _, _) => {
                        h942
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h943, _, _, _, _, _) => {
                        h943
                    }
                };
                let m946 = f60(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h944, _, _, _) => {
                        h944
                    }
                }, match v15.clone() {
                    Adt3::C0(h945, _, _, _, _, _, _, _) => {
                        h945
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m946 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m949 = f60(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h947, _, _) => {
                                h947
                            }
                        }, match v15 {
                            Adt3::C0(_, h948, _, _, _, _, _, _) => {
                                h948
                            }
                        }, v12, f58(v9, v14, 1u64))?;
                        match m949 {
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
                let a950 = v1;
                let a951 = 2u64;
                if nat_eq(a950, a951) {
                    Ok(Adt9::C8(Some({
                        let a953 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h952) => {
                                h952
                            }
                        };
                        let a954 = 0u64;
                        if nat_eq(a953, a954) {
                            f57(v6, v10, v12, v9, v13)?
                        } else {
                            f57(v6.clone(), v10, v12, {
                                let a956 = v13;
                                let a957 = match v6.clone() {
                                    Adt3::C0(_, _, h955, _, _, _, _, _) => {
                                        h955
                                    }
                                };
                                let a958 = 0u64;
                                nat_rem(a956, a957, a958)
                            }, {
                                let a966 = {
                                    let a964 = {
                                        let a960 = v13;
                                        let a961 = match v6 {
                                            Adt3::C0(_, _, h959, _, _, _, _, _) => {
                                                h959
                                            }
                                        };
                                        let a962 = 0u64;
                                        nat_quot(a960, a961, a962)
                                    };
                                    let a965 = match v0 {
                                        Adt8::C0(_, _, h963, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h963
                                        }
                                    };
                                    nat_mul(a964, a965)?
                                };
                                let a967 = v9;
                                nat_add(a966, a967)?
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
        let m968 = v3.clone();
        match m968 {
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
                f78(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C19(v77, v78) => {
                f79(v77, v0, v1, v2, v78, v4)?
            }
            Adt4::C20(v79, v80) => {
                f80(v79, v0, v1, v2, v80, v4)?
            }
            Adt4::C21 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C22(v81, v82) => {
                f81(v0, v1, v2, v81, v82, v4)?
            }
            Adt4::C23(v96, v97, v98, v99) => {
                f88(v0, v99, v96, v97, v98, v4)?
            }
            Adt4::C24(v95) => {
                if v5 {
                    f86(Adt16::C0(v95), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C26(v91, v92, v93, v94) => {
                let a970 = v91;
                let a971 = 64u64;
                if if if nat_eq(a970, a971) {
                    true
                } else {
                    let a972 = v91;
                    let a973 = 65u64;
                    nat_eq(a972, a973)
                } {
                    true
                } else {
                    let a974 = v91;
                    let a975 = 66u64;
                    nat_eq(a974, a975)
                } {
                    if v5 {
                        f86(Adt16::C1(v91, v92, v93, v94), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a976 = v91;
                    let a977 = 0u64;
                    if if nat_eq(a976, a977) {
                        true
                    } else {
                        let a978 = v91;
                        let a979 = 30u64;
                        nat_eq(a978, a979)
                    } {
                        f87(v0, v1, v2, v91, v92, v93, v94, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v90) => {
                f85(v0, v1, v2, v90)?
            }
            Adt4::C28(v86) => {
                f82(v0, v1, v2, v86, v4)?
            }
            Adt4::C29(v87) => {
                f83(v0, v1, v2, v87)?
            }
            Adt4::C30(v88, _) => {
                f84(v0, v1, v2, v88)?
            }
            Adt4::C31(v33, v34) => {
                f64(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f65(v0, match v1 {
                    Adt3::C0(h969, _, _, _, _, _, _, _) => {
                        h969
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f66(v0, v1, v36, v2)?
            }
            Adt4::C34(v57, v58, v59, v60) => {
                f77(v0, v1, v57, v2, v58, v59, v60)?
            }
            Adt4::C35(v56) => {
                f76(v0, v56, v1, v2)?
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
                f75(v0, v1, v2, v4)?
            }
            Adt4::C42(v54, v55) => {
                f74(v0, v1, v2, v54, v55)?
            }
            Adt4::C43(v52, v53) => {
                f73(v0, v1, v2, v52, v53, v4)?
            }
            Adt4::C44(v51) => {
                f72(v0, v2, v51)?
            }
        }
    }))
}

pub fn f19(v0: Adt4) -> R<Adt9> {
    let m980 = v0;
    match m980 {
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
        Adt4::C25(v74, _, v76) => {
            Ok(Adt9::C6({
                let a986 = {
                    let a982 = 0u64;
                    let a983 = v74;
                    if nat_lt(a982, a983) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a987 = {
                    let a984 = 0u64;
                    let a985 = v76;
                    if nat_lt(a984, a985) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a986, a987)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v82, _, _, v85) => {
            Ok(Adt9::C6({
                let a988 = v82;
                let a989 = 66u64;
                if nat_eq(a988, a989) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v85))
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
        Adt4::C35(v47) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v47))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m981 = v32;
                match m981 {
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
        Adt4::C43(_, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C44(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 5u64))
        }
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a990 = v1;
    let a991 = 5u64;
    if if if if if nat_lt(a990, a991) {
        true
    } else {
        f89(v0, v1, v2, v3, v4)
    } {
        let a992 = v5;
        let a993 = 5u64;
        nat_lt(a992, a993)
    } else {
        false
    } {
        let a994 = v7;
        let a995 = 5u64;
        nat_lt(a994, a995)
    } else {
        false
    } {
        let a996 = v9;
        let a997 = 2u64;
        if nat_lt(a996, a997) {
            true
        } else {
            let a998 = v9;
            let a999 = 3u64;
            nat_eq(a998, a999)
        }
    } else {
        false
    } {
        Adt9::C7(f90(v0, v1, v2, v3, v4), f91(v5, v6), f91(v7, v8), f92({
            let a1000 = v9;
            let a1001 = 0u64;
            if nat_eq(a1000, a1001) {
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
    let a1004 = match v0.clone() {
        Adt2::C0(_, h1002, _, _, _, _, _, _, _, _, _) => {
            h1002
        }
    };
    let a1005 = match v1.clone() {
        Adt2::C0(_, h1003, _, _, _, _, _, _, _, _, _) => {
            h1003
        }
    };
    if if if nat_eq(a1004, a1005) {
        let a1008 = match v0.clone() {
            Adt2::C0(_, _, h1006, _, _, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = match v1.clone() {
            Adt2::C0(_, _, h1007, _, _, _, _, _, _, _, _) => {
                h1007
            }
        };
        nat_eq(a1008, a1009)
    } else {
        false
    } {
        let a1012 = match v0.clone() {
            Adt2::C0(_, _, _, h1010, _, _, _, _, _, _, _) => {
                h1010
            }
        };
        let a1013 = match v1.clone() {
            Adt2::C0(_, _, _, h1011, _, _, _, _, _, _, _) => {
                h1011
            }
        };
        nat_eq(a1012, a1013)
    } else {
        false
    } {
        let a1016 = match v0 {
            Adt2::C0(_, _, _, _, h1014, _, _, _, _, _, _) => {
                h1014
            }
        };
        let a1017 = match v1 {
            Adt2::C0(_, _, _, _, h1015, _, _, _, _, _, _) => {
                h1015
            }
        };
        nat_eq(a1016, a1017)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1019 = match v0.clone() {
        Adt2::C0(h1018, _, _, _, _, _, _, _, _, _, _) => {
            h1018
        }
    };
    let a1020 = 0u64;
    if if if nat_eq(a1019, a1020) {
        let a1022 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1021, _, _, _, _, _) => {
                h1021
            }
        };
        let a1023 = 4u64;
        nat_eq(a1022, a1023)
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
        let a1035 = {
            let a1032 = {
                let a1027 = {
                    let a1024 = v1;
                    let a1025 = 1u64;
                    nat_sub(a1024, a1025)
                };
                let a1028 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1026, _, _, _, _) => {
                        h1026
                    }
                };
                nat_mul(a1027, a1028)?
            };
            let a1033 = {
                let a1030 = 4u64;
                let a1031 = match v0.clone() {
                    Adt2::C0(_, h1029, _, _, _, _, _, _, _, _, _) => {
                        h1029
                    }
                };
                nat_mul(a1030, a1031)?
            };
            nat_add(a1032, a1033)?
        };
        let a1036 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1034, _) => {
                h1034
            }
        };
        Ok(nat_le(a1035, a1036))
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a1048 = {
        let a1042 = {
            let a1038 = match v0.clone() {
                Adt2::C0(_, h1037, _, _, _, _, _, _, _, _, _) => {
                    h1037
                }
            };
            let a1039 = v1;
            let a1040 = 0u64;
            nat_quot(a1038, a1039, a1040)
        };
        let a1043 = match v0.clone() {
            Adt2::C0(_, _, h1041, _, _, _, _, _, _, _, _) => {
                h1041
            }
        };
        nat_mul(a1042, a1043)?
    };
    let a1049 = {
        let a1046 = match v0.clone() {
            Adt2::C0(_, _, _, h1044, _, _, _, _, _, _, _) => {
                h1044
            }
        };
        let a1047 = match v0 {
            Adt2::C0(_, _, _, _, h1045, _, _, _, _, _, _) => {
                h1045
            }
        };
        nat_mul(a1046, a1047)?
    };
    nat_mul(a1048, a1049)
}

pub fn f24(v0: bool, v1: u64) -> R<u64> {
    let a1052 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1053 = {
        let a1050 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1051 = v1;
        nat_mul(a1050, a1051)?
    };
    nat_add(a1052, a1053)
}

pub fn f25(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1055 = match v0.clone() {
            Adt2::C0(_, h1054, _, _, _, _, _, _, _, _, _) => {
                h1054
            }
        };
        let a1056 = v1;
        let a1057 = 0u64;
        nat_quot(a1055, a1056, a1057)
    }, match v0.clone() {
        Adt2::C0(_, _, h1058, _, _, _, _, _, _, _, _) => {
            h1058
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1059, _, _, _, _, _, _, _) => {
            h1059
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1060, _, _, _, _, _, _) => {
            h1060
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1061, _, _, _, _, _) => {
            h1061
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1062, _, _, _, _) => {
            h1062
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1063, _, _, _) => {
            h1063
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1064, _, _) => {
            h1064
        }
    })
}

pub fn f26(v0: Adt2, v1: u64) -> R<bool> {
    let a1065 = v1;
    let a1066 = 0u64;
    if nat_eq(a1065, a1066) {
        Ok(true)
    } else {
        let a1072 = {
            let a1069 = f93({
                let a1067 = v1;
                let a1068 = 1u64;
                nat_sub(a1067, a1068)
            }, f25(v0.clone(), 1u64))?;
            let a1070 = 4u64;
            nat_add(a1069, a1070)?
        };
        let a1073 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1071, _) => {
                h1071
            }
        };
        Ok(nat_le(a1072, a1073))
    }
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1075 = match v1.clone() {
        Adt2::C0(h1074, _, _, _, _, _, _, _, _, _, _) => {
            h1074
        }
    };
    let a1076 = 0u64;
    if if nat_eq(a1075, a1076) {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1077 = f23(v0.clone(), 1u64)?;
        let a1078 = 0u64;
        if nat_eq(a1077, a1078) {
            Ok(true)
        } else {
            let a1084 = {
                let a1081 = f93({
                    let a1079 = f23(v0.clone(), 1u64)?;
                    let a1080 = 1u64;
                    nat_sub(a1079, a1080)
                }, f25(v0, 1u64))?;
                let a1082 = 4u64;
                nat_add(a1081, a1082)?
            };
            let a1085 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1083, _) => {
                    h1083
                }
            };
            Ok(nat_le(a1084, a1085))
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
    let a1087 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1086, _) => {
            h1086
        }
    };
    let a1088 = 0u64;
    if nat_eq(a1087, a1088) {
        Ok(true)
    } else {
        let a1090 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1089, _) => {
                h1089
            }
        };
        let a1091 = 0u64;
        if nat_eq(a1090, a1091) {
            Ok(true)
        } else {
            let a1093 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1092) => {
                    h1092
                }
            };
            let a1094 = 0u64;
            if nat_eq(a1093, a1094) {
                Ok(true)
            } else {
                let a1096 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1095) => {
                        h1095
                    }
                };
                let a1097 = 0u64;
                if nat_eq(a1096, a1097) {
                    Ok(true)
                } else {
                    let a1103 = {
                        let a1100 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1098) => {
                                h1098
                            }
                        };
                        let a1101 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1099, _) => {
                                h1099
                            }
                        };
                        nat_add(a1100, a1101)?
                    };
                    let a1104 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1102) => {
                            h1102
                        }
                    };
                    if nat_le(a1103, a1104) {
                        Ok(true)
                    } else {
                        let a1110 = {
                            let a1107 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1105) => {
                                    h1105
                                }
                            };
                            let a1108 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1106, _) => {
                                    h1106
                                }
                            };
                            nat_add(a1107, a1108)?
                        };
                        let a1111 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1109) => {
                                h1109
                            }
                        };
                        Ok(nat_le(a1110, a1111))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1117 = match v0.clone() {
            Adt2::C0(_, _, h1112, _, _, _, _, _, _, _, _) => {
                h1112
            }
        };
        let a1118 = {
            let a1115 = match v0.clone() {
                Adt2::C0(_, _, _, h1113, _, _, _, _, _, _, _) => {
                    h1113
                }
            };
            let a1116 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1114, _, _, _, _, _, _) => {
                    h1114
                }
            };
            nat_mul(a1115, a1116)?
        };
        nat_mul(a1117, a1118)?
    };
    let v2: u64 = f23(v0.clone(), 1u64)?;
    let a1120 = match v0.clone() {
        Adt2::C0(h1119, _, _, _, _, _, _, _, _, _, _) => {
            h1119
        }
    };
    let a1121 = 0u64;
    if if if if if if if nat_eq(a1120, a1121) {
        let a1123 = match v0.clone() {
            Adt2::C0(_, h1122, _, _, _, _, _, _, _, _, _) => {
                h1122
            }
        };
        let a1124 = 2147483647u64;
        nat_le(a1123, a1124)
    } else {
        false
    } {
        let a1125 = v1;
        let a1126 = 2147483647u64;
        nat_le(a1125, a1126)
    } else {
        false
    } {
        let a1128 = match v0.clone() {
            Adt2::C0(_, h1127, _, _, _, _, _, _, _, _, _) => {
                h1127
            }
        };
        let a1129 = 1u64;
        if nat_eq(a1128, a1129) {
            true
        } else {
            let a1131 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1130, _, _, _, _, _) => {
                    h1130
                }
            };
            let a1132 = 4u64;
            nat_eq(a1131, a1132)
        }
    } else {
        false
    } {
        let a1133 = v1;
        let a1134 = 1u64;
        if nat_le(a1133, a1134) {
            true
        } else {
            let a1139 = {
                let a1136 = match v0.clone() {
                    Adt2::C0(_, h1135, _, _, _, _, _, _, _, _, _) => {
                        h1135
                    }
                };
                let a1137 = 4u64;
                nat_mul(a1136, a1137)?
            };
            let a1140 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1138, _, _, _, _) => {
                    h1138
                }
            };
            nat_le(a1139, a1140)
        }
    } else {
        false
    } {
        let a1142 = match v0.clone() {
            Adt2::C0(_, _, _, h1141, _, _, _, _, _, _, _) => {
                h1141
            }
        };
        let a1143 = 1u64;
        if nat_eq(a1142, a1143) {
            true
        } else {
            let a1149 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1144, _, _, _) => {
                    h1144
                }
            };
            let a1150 = {
                let a1147 = match v0.clone() {
                    Adt2::C0(_, _, h1145, _, _, _, _, _, _, _, _) => {
                        h1145
                    }
                };
                let a1148 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1146, _, _, _, _) => {
                        h1146
                    }
                };
                nat_mul(a1147, a1148)?
            };
            nat_eq(a1149, a1150)
        }
    } else {
        false
    } {
        let a1152 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1151, _, _, _, _, _, _) => {
                h1151
            }
        };
        let a1153 = 1u64;
        if nat_eq(a1152, a1153) {
            true
        } else {
            let a1162 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1154, _, _) => {
                    h1154
                }
            };
            let a1163 = {
                let a1160 = match v0.clone() {
                    Adt2::C0(_, _, _, h1155, _, _, _, _, _, _, _) => {
                        h1155
                    }
                };
                let a1161 = {
                    let a1158 = match v0.clone() {
                        Adt2::C0(_, _, h1156, _, _, _, _, _, _, _, _) => {
                            h1156
                        }
                    };
                    let a1159 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1157, _, _, _, _) => {
                            h1157
                        }
                    };
                    nat_mul(a1158, a1159)?
                };
                nat_mul(a1160, a1161)?
            };
            nat_eq(a1162, a1163)
        }
    } else {
        false
    } {
        let a1164 = v2;
        let a1165 = 0u64;
        if nat_eq(a1164, a1165) {
            Ok(true)
        } else {
            let a1171 = {
                let a1168 = f93({
                    let a1166 = v2;
                    let a1167 = 1u64;
                    nat_sub(a1166, a1167)
                }, f25(v0.clone(), 1u64))?;
                let a1169 = 4u64;
                nat_add(a1168, a1169)?
            };
            let a1172 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1170, _) => {
                    h1170
                }
            };
            Ok(nat_le(a1171, a1172))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1179 = 148u64;
    let a1180 = {
        let a1177 = 36u64;
        let a1178 = {
            let a1175 = {
                let a1173 = 2u64;
                let a1174 = v0;
                nat_mul(a1173, a1174)?
            };
            let a1176 = 1u64;
            nat_sub(a1175, a1176)
        };
        nat_mul(a1177, a1178)?
    };
    nat_add(a1179, a1180)
}

pub fn f33(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1181 = v1;
    match m1181 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1184 = 1u64;
            let a1185 = {
                let a1182 = 2u64;
                let a1183 = v0;
                nat_mul(a1182, a1183)?
            };
            nat_add(a1184, a1185)
        }
    }
}

pub fn f34(v0: u64) -> bool {
    let a1186 = v0;
    let a1187 = 4294967296u64;
    if nat_lt(a1186, a1187) {
        let a1191 = {
            let a1188 = v0;
            let a1189 = 2147483648u64;
            let a1190 = 0u64;
            nat_rem(a1188, a1189, a1190)
        };
        let a1192 = 2139095040u64;
        nat_lt(a1191, a1192)
    } else {
        false
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a1193 = 4u64;
    let a1194 = v0;
    nat_mul(a1193, a1194)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1195 = v0;
    let a1196 = 0u64;
    if nat_eq(a1195, a1196) {
        Ok(0u64)
    } else {
        let a1199 = 4u64;
        let a1200 = {
            let a1197 = 3u64;
            let a1198 = v0;
            nat_mul(a1197, a1198)?
        };
        nat_add(a1199, a1200)
    }
}

pub fn f37(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f25(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1201, _, _, _, _, _, _, _, _, _) => {
                h1201
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1202, _, _, _, _, _, _, _, _) => {
                h1202
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1203, _, _, _, _, _, _, _) => {
                h1203
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1204, _, _, _, _, _) => {
                h1204
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1205, _, _, _, _) => {
                h1205
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1206, _, _, _) => {
                h1206
            }
        })
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let a1208 = match v0.clone() {
        Adt2::C0(_, h1207, _, _, _, _, _, _, _, _, _) => {
            h1207
        }
    };
    let a1209 = 1u64;
    if if if if nat_eq(a1208, a1209) {
        true
    } else {
        let a1211 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1210, _, _, _, _, _) => {
                h1210
            }
        };
        let a1212 = 4u64;
        nat_eq(a1211, a1212)
    } {
        let a1214 = match v0.clone() {
            Adt2::C0(_, _, h1213, _, _, _, _, _, _, _, _) => {
                h1213
            }
        };
        let a1215 = 1u64;
        if nat_eq(a1214, a1215) {
            true
        } else {
            let a1220 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1216, _, _, _, _) => {
                    h1216
                }
            };
            let a1221 = {
                let a1218 = match v0.clone() {
                    Adt2::C0(_, h1217, _, _, _, _, _, _, _, _, _) => {
                        h1217
                    }
                };
                let a1219 = 4u64;
                nat_mul(a1218, a1219)?
            };
            nat_eq(a1220, a1221)
        }
    } else {
        false
    } {
        let a1223 = match v0.clone() {
            Adt2::C0(_, _, _, h1222, _, _, _, _, _, _, _) => {
                h1222
            }
        };
        let a1224 = 1u64;
        if nat_eq(a1223, a1224) {
            true
        } else {
            let a1232 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1225, _, _, _) => {
                    h1225
                }
            };
            let a1233 = {
                let a1230 = {
                    let a1228 = match v0.clone() {
                        Adt2::C0(_, h1226, _, _, _, _, _, _, _, _, _) => {
                            h1226
                        }
                    };
                    let a1229 = match v0.clone() {
                        Adt2::C0(_, _, h1227, _, _, _, _, _, _, _, _) => {
                            h1227
                        }
                    };
                    nat_mul(a1228, a1229)?
                };
                let a1231 = 4u64;
                nat_mul(a1230, a1231)?
            };
            nat_eq(a1232, a1233)
        }
    } else {
        false
    } {
        let a1235 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1234, _, _, _, _, _, _) => {
                h1234
            }
        };
        let a1236 = 1u64;
        if nat_eq(a1235, a1236) {
            Ok(true)
        } else {
            let a1247 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1237, _, _) => {
                    h1237
                }
            };
            let a1248 = {
                let a1245 = {
                    let a1243 = {
                        let a1240 = match v0.clone() {
                            Adt2::C0(_, h1238, _, _, _, _, _, _, _, _, _) => {
                                h1238
                            }
                        };
                        let a1241 = match v0.clone() {
                            Adt2::C0(_, _, h1239, _, _, _, _, _, _, _, _) => {
                                h1239
                            }
                        };
                        nat_mul(a1240, a1241)?
                    };
                    let a1244 = match v0 {
                        Adt2::C0(_, _, _, h1242, _, _, _, _, _, _, _) => {
                            h1242
                        }
                    };
                    nat_mul(a1243, a1244)?
                };
                let a1246 = 4u64;
                nat_mul(a1245, a1246)?
            };
            Ok(nat_eq(a1247, a1248))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt6, v1: u64) -> R<u64> {
    let a1250 = match v0.clone() {
        Adt6::C0(h1249, _, _, _) => {
            h1249
        }
    };
    let a1251 = 0u64;
    if nat_eq(a1250, a1251) {
        f94(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1252, _, _) => {
                h1252
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1253, _) => {
                h1253
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1254) => {
                h1254
            }
        }), {
            let a1256 = v1;
            let a1257 = match v0 {
                Adt6::C0(_, _, _, h1255) => {
                    h1255
                }
            };
            nat_add(a1256, a1257)?
        })
    } else {
        f94(v0, v1)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1262 = {
        let a1258 = v0;
        let a1259 = v1;
        nat_mul(a1258, a1259)?
    };
    let a1263 = {
        let a1260 = v2;
        let a1261 = f95(v3);
        nat_mul(a1260, a1261)?
    };
    nat_mul(a1262, a1263)
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1266 = v0;
    let a1267 = {
        let a1264 = f96(v1)?;
        let a1265 = v2;
        nat_mul(a1264, a1265)?
    };
    nat_add(a1266, a1267)
}

pub fn f42(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1268, _, _, _, _, _, _, _, _, _, _) => {
            h1268
        }
    }, match v0.clone() {
        Adt2::C0(_, h1269, _, _, _, _, _, _, _, _, _) => {
            h1269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1270, _, _, _, _, _, _, _, _) => {
            h1270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1271, _, _, _, _, _, _, _) => {
            h1271
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1272, _, _, _, _, _, _) => {
            h1272
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1273, _, _, _, _, _) => {
            h1273
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1274, _, _, _, _) => {
            h1274
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1275, _, _, _) => {
            h1275
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1276, _, _) => {
            h1276
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1277, _) => {
            h1277
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1278) => {
            h1278
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1279 = v0;
    let a1280 = 23u64;
    if nat_eq(a1279, a1280) {
        f97(v1, v4, v6)
    } else {
        let a1281 = v0;
        let a1282 = 26u64;
        if nat_eq(a1281, a1282) {
            f98(v1, v2, v4, v6)
        } else {
            let a1283 = v0;
            let a1284 = 25u64;
            if nat_eq(a1283, a1284) {
                f99(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    let a1285 = v0;
    let a1286 = 30u64;
    if nat_eq(a1285, a1286) {
        2u64
    } else {
        4u64
    }
}

pub fn f45(v0: u64) -> bool {
    let a1287 = v0;
    let a1288 = 2139095040u64;
    if nat_le(a1287, a1288) {
        true
    } else {
        let a1289 = v0;
        let a1290 = 2147483648u64;
        nat_eq(a1289, a1290)
    }
}

pub fn f46(v0: Adt2, v1: u64) -> bool {
    let a1292 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1291, _, _, _, _, _) => {
            h1291
        }
    };
    let a1293 = v1;
    if if nat_eq(a1292, a1293) {
        let a1298 = {
            let a1295 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1294) => {
                    h1294
                }
            };
            let a1296 = v1;
            let a1297 = 0u64;
            nat_rem(a1295, a1296, a1297)
        };
        let a1299 = 0u64;
        nat_eq(a1298, a1299)
    } else {
        false
    } {
        f100(match v0.clone() {
            Adt2::C0(_, _, h1300, _, _, _, _, _, _, _, _) => {
                h1300
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1301, _, _, _, _, _, _, _) => {
                h1301
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1302, _, _, _, _, _, _) => {
                h1302
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1303, _, _, _, _) => {
                h1303
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1304, _, _, _) => {
                h1304
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1305, _, _) => {
                h1305
            }
        }, v1)
    } else {
        false
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a1308 = match v0.clone() {
            Adt2::C0(h1306, _, _, _, _, _, _, _, _, _, _) => {
                h1306
            }
        };
        let a1309 = match v1.clone() {
            Adt2::C0(h1307, _, _, _, _, _, _, _, _, _, _) => {
                h1307
            }
        };
        nat_eq(a1308, a1309)
    } else {
        false
    } {
        let a1312 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1310) => {
                h1310
            }
        };
        let a1313 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1311) => {
                h1311
            }
        };
        nat_eq(a1312, a1313)
    } else {
        false
    } {
        let a1316 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1314, _, _, _, _, _) => {
                h1314
            }
        };
        let a1317 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1315, _, _, _, _, _) => {
                h1315
            }
        };
        nat_eq(a1316, a1317)
    } else {
        false
    } {
        let a1320 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1318, _, _, _, _) => {
                h1318
            }
        };
        let a1321 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1319, _, _, _, _) => {
                h1319
            }
        };
        nat_eq(a1320, a1321)
    } else {
        false
    } {
        let a1324 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1322, _, _, _) => {
                h1322
            }
        };
        let a1325 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1323, _, _, _) => {
                h1323
            }
        };
        nat_eq(a1324, a1325)
    } else {
        false
    } {
        let a1328 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1326, _, _) => {
                h1326
            }
        };
        let a1329 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1327, _, _) => {
                h1327
            }
        };
        nat_eq(a1328, a1329)
    } else {
        false
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1330 = v0;
    let a1331 = 0u64;
    if if if nat_eq(a1330, a1331) {
        true
    } else {
        let a1332 = v1;
        let a1333 = 0u64;
        nat_eq(a1332, a1333)
    } {
        true
    } else {
        let a1334 = v2;
        let a1335 = 0u64;
        nat_eq(a1334, a1335)
    } {
        true
    } else {
        let a1336 = v3;
        let a1337 = 0u64;
        nat_eq(a1336, a1337)
    }
}

pub fn f49(v0: Adt2, v1: u64) -> R<bool> {
    f101({
        let a1339 = match v0.clone() {
            Adt2::C0(_, h1338, _, _, _, _, _, _, _, _, _) => {
                h1338
            }
        };
        let a1340 = v1;
        nat_mul(a1339, a1340)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1341, _, _, _, _, _, _, _, _) => {
            h1341
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1342, _, _, _, _, _, _, _) => {
            h1342
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1343, _, _, _, _, _, _) => {
            h1343
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1344, _, _, _, _) => {
            h1344
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1345, _, _, _) => {
            h1345
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1346, _, _) => {
            h1346
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1347 = v1;
    let a1348 = 0u64;
    if if nat_eq(a1347, a1348) {
        true
    } else {
        let a1349 = v2;
        let a1350 = 0u64;
        nat_eq(a1349, a1350)
    } {
        Ok(0u64)
    } else {
        let a1357 = {
            let a1351 = f102(v1, v2, v3)?;
            let a1352 = f103(v0, v1, v2)?;
            nat_add(a1351, a1352)?
        };
        let a1358 = {
            let a1355 = f104(v0);
            let a1356 = {
                let a1353 = v1;
                let a1354 = v2;
                nat_mul(a1353, a1354)?
            };
            nat_mul(a1355, a1356)?
        };
        nat_add(a1357, a1358)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1359 = v0;
    let a1360 = 0u64;
    if if nat_eq(a1359, a1360) {
        true
    } else {
        let a1361 = v1;
        let a1362 = 0u64;
        nat_eq(a1361, a1362)
    } {
        Ok(3u64)
    } else {
        let a1365 = {
            let a1363 = 3u64;
            let a1364 = v1;
            nat_add(a1363, a1364)?
        };
        let a1366 = v2;
        nat_add(a1365, a1366)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1367 = v0;
    let a1368 = 2139095040u64;
    if nat_le(a1367, a1368) {
        true
    } else {
        let a1369 = v0;
        let a1370 = 2147483648u64;
        nat_eq(a1369, a1370)
    }
}

pub fn f53(v0: u64, v1: u64) -> R<u64> {
    let a1371 = v0;
    let a1372 = 0u64;
    if nat_eq(a1371, a1372) {
        Ok(0u64)
    } else {
        let a1373 = v1;
        let a1374 = f105(v0)?;
        nat_mul(a1373, a1374)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1375 = v0;
    let a1376 = 0u64;
    if if if if nat_eq(a1375, a1376) {
        true
    } else {
        let a1377 = v1;
        let a1378 = 0u64;
        nat_eq(a1377, a1378)
    } {
        true
    } else {
        let a1379 = v2;
        let a1380 = 0u64;
        nat_eq(a1379, a1380)
    } {
        true
    } else {
        let a1381 = v3;
        let a1382 = 0u64;
        nat_eq(a1381, a1382)
    } {
        Ok(0u64)
    } else {
        let a1395 = {
            let a1385 = 2u64;
            let a1386 = {
                let a1383 = v1;
                let a1384 = v2;
                nat_mul(a1383, a1384)?
            };
            nat_mul(a1385, a1386)?
        };
        let a1396 = {
            let a1393 = {
                let a1389 = {
                    let a1387 = v1;
                    let a1388 = v2;
                    nat_mul(a1387, a1388)?
                };
                let a1390 = v3;
                nat_mul(a1389, a1390)?
            };
            let a1394 = {
                let a1391 = 1u64;
                let a1392 = v0;
                nat_add(a1391, a1392)?
            };
            nat_mul(a1393, a1394)?
        };
        nat_add(a1395, a1396)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1397 = v1;
        let a1398 = 0u64;
        if if if nat_eq(a1397, a1398) {
            true
        } else {
            let a1399 = v1;
            let a1400 = 10u64;
            nat_eq(a1399, a1400)
        } {
            true
        } else {
            let a1401 = v1;
            let a1402 = 15u64;
            nat_eq(a1401, a1402)
        } {
            true
        } else {
            let a1403 = v1;
            let a1404 = 20u64;
            nat_eq(a1403, a1404)
        }
    };
    let v4: bool = {
        let a1405 = v2;
        let a1406 = 0u64;
        if if if if if nat_eq(a1405, a1406) {
            true
        } else {
            let a1407 = v2;
            let a1408 = 10u64;
            nat_eq(a1407, a1408)
        } {
            true
        } else {
            let a1409 = v2;
            let a1410 = 15u64;
            nat_eq(a1409, a1410)
        } {
            true
        } else {
            let a1411 = v2;
            let a1412 = 20u64;
            nat_eq(a1411, a1412)
        } {
            true
        } else {
            let a1413 = v2;
            let a1414 = 30u64;
            nat_eq(a1413, a1414)
        } {
            true
        } else {
            let a1415 = v2;
            let a1416 = 40u64;
            nat_eq(a1415, a1416)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1417 = v0;
            let a1418 = 30u64;
            if if nat_eq(a1417, a1418) {
                let a1419 = v2;
                let a1420 = 10u64;
                nat_eq(a1419, a1420)
            } else {
                false
            } {
                true
            } else {
                if f106(v0) {
                    let a1421 = v2;
                    let a1422 = 10u64;
                    if if nat_eq(a1421, a1422) {
                        true
                    } else {
                        let a1423 = v2;
                        let a1424 = 15u64;
                        nat_eq(a1423, a1424)
                    } {
                        true
                    } else {
                        let a1425 = v2;
                        let a1426 = 20u64;
                        nat_eq(a1425, a1426)
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
        Adt1::C0(h1427, _, _, _, _, _, _, _, _, _, _, _) => {
            h1427
        }
    };
    Ok(Adt9::C0({
        let a1428 = v0;
        let a1429 = 65u64;
        if nat_eq(a1428, a1429) {
            let m1430 = v3;
            match m1430 {
                None => {
                    f107(v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a1431 = v0;
            let a1432 = 59u64;
            if nat_eq(a1431, a1432) {
                let m1433 = v3;
                match m1433 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f108(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1434 = v0;
                let a1435 = 50u64;
                if nat_eq(a1434, a1435) {
                    let m1436 = v3;
                    match m1436 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f109(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1437 = v0;
                    let a1438 = 82u64;
                    if nat_eq(a1437, a1438) {
                        let m1439 = v3;
                        match m1439 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f110(v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1440 = v0;
                        let a1441 = 86u64;
                        if nat_eq(a1440, a1441) {
                            let m1442 = v3;
                            match m1442 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f111(v1, v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1443 = v0;
                            let a1444 = 55u64;
                            if nat_eq(a1443, a1444) {
                                let m1445 = v3;
                                match m1445 {
                                    None => {
                                        Adt10::C1
                                    }
                                    Some(v11) => {
                                        f112(v1, v2, v11, v4)?
                                    }
                                }
                            } else {
                                let a1446 = v0;
                                let a1447 = 22u64;
                                if nat_eq(a1446, a1447) {
                                    let m1448 = v3;
                                    match m1448 {
                                        None => {
                                            Adt10::C1
                                        }
                                        Some(v12) => {
                                            f113(v2, v12, v4)?
                                        }
                                    }
                                } else {
                                    let a1449 = v0;
                                    let a1450 = 44u64;
                                    if nat_eq(a1449, a1450) {
                                        f114(v1, v2, v3, v4)?
                                    } else {
                                        let a1451 = v0;
                                        let a1452 = 52u64;
                                        if nat_eq(a1451, a1452) {
                                            f115(v1, v2, v4)?
                                        } else {
                                            let a1453 = v0;
                                            let a1454 = 73u64;
                                            if nat_eq(a1453, a1454) {
                                                f116(v2, v3, v4)?
                                            } else {
                                                let a1455 = v0;
                                                let a1456 = 57u64;
                                                if if nat_eq(a1455, a1456) {
                                                    true
                                                } else {
                                                    let a1457 = v0;
                                                    let a1458 = 58u64;
                                                    nat_eq(a1457, a1458)
                                                } {
                                                    let m1459 = v3;
                                                    match m1459 {
                                                        None => {
                                                            f117(false, {
                                                                let a1460 = v0;
                                                                let a1461 = 58u64;
                                                                nat_eq(a1460, a1461)
                                                            }, v1, v2, v4)?
                                                        }
                                                        Some(_) => {
                                                            Adt10::C1
                                                        }
                                                    }
                                                } else {
                                                    let a1462 = v0;
                                                    let a1463 = 49u64;
                                                    if nat_eq(a1462, a1463) {
                                                        f118(false, v1, v2, v3, v4)?
                                                    } else {
                                                        let a1464 = v0;
                                                        let a1465 = 51u64;
                                                        if nat_eq(a1464, a1465) {
                                                            f119(v1, v2, v3, v4)?
                                                        } else {
                                                            let a1466 = v0;
                                                            let a1467 = 39u64;
                                                            if nat_eq(a1466, a1467) {
                                                                f120(v2, v3, v4)?
                                                            } else {
                                                                let a1468 = v0;
                                                                let a1469 = 77u64;
                                                                if nat_eq(a1468, a1469) {
                                                                    f121(v2, v4)?
                                                                } else {
                                                                    let a1470 = v0;
                                                                    let a1471 = 40u64;
                                                                    if nat_eq(a1470, a1471) {
                                                                        f122(v2, v3, v4)?
                                                                    } else {
                                                                        if f123(v0) {
                                                                            let a1472 = v0;
                                                                            let a1473 = 33u64;
                                                                            if if if nat_eq(a1472, a1473) {
                                                                                let a1475 = match v2.clone() {
                                                                                    Adt2::C0(h1474, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1474
                                                                                    }
                                                                                };
                                                                                let a1476 = 0u64;
                                                                                nat_eq(a1475, a1476)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let a1478 = match v4.clone() {
                                                                                    Adt2::C0(h1477, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1477
                                                                                    }
                                                                                };
                                                                                let a1479 = 26u64;
                                                                                nat_eq(a1478, a1479)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let m1480 = v3;
                                                                                match m1480 {
                                                                                    None => {
                                                                                        Adt10::C1
                                                                                    }
                                                                                    Some(v14) => {
                                                                                        f124(v2, v14, v4)?
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a1481 = v0;
                                                                                let a1482 = 33u64;
                                                                                if if if nat_eq(a1481, a1482) {
                                                                                    let a1484 = match v4.clone() {
                                                                                        Adt2::C0(h1483, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1483
                                                                                        }
                                                                                    };
                                                                                    let a1485 = 0u64;
                                                                                    nat_eq(a1484, a1485)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f125(match v2.clone() {
                                                                                        Adt2::C0(h1486, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1486
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f126(v2, v3, v4)?
                                                                                } else {
                                                                                    f127(v0, v2, {
                                                                                        let m1487 = v3;
                                                                                        match m1487 {
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
                                                                            let a1488 = v0;
                                                                            let a1489 = 19u64;
                                                                            if nat_eq(a1488, a1489) {
                                                                                f128(v2, v4)?
                                                                            } else {
                                                                                let a1490 = v0;
                                                                                let a1491 = 21u64;
                                                                                if nat_eq(a1490, a1491) {
                                                                                    f129(v5, v2, v3, v4)?
                                                                                } else {
                                                                                    let a1492 = v0;
                                                                                    let a1493 = 61u64;
                                                                                    if if if nat_eq(a1492, a1493) {
                                                                                        true
                                                                                    } else {
                                                                                        let a1494 = v0;
                                                                                        let a1495 = 62u64;
                                                                                        nat_eq(a1494, a1495)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a1496 = v0;
                                                                                        let a1497 = 63u64;
                                                                                        nat_eq(a1496, a1497)
                                                                                    } {
                                                                                        f130(v0, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a1498 = v0;
                                                                                        let a1499 = 4u64;
                                                                                        if if nat_eq(a1498, a1499) {
                                                                                            true
                                                                                        } else {
                                                                                            let a1500 = v0;
                                                                                            let a1501 = 32u64;
                                                                                            nat_eq(a1500, a1501)
                                                                                        } {
                                                                                            f131(v0, v1, v2, v3, v4)?
                                                                                        } else {
                                                                                            let a1502 = v0;
                                                                                            let a1503 = 20u64;
                                                                                            if nat_eq(a1502, a1503) {
                                                                                                f132(v2, v4)?
                                                                                            } else {
                                                                                                let a1504 = v0;
                                                                                                let a1505 = 42u64;
                                                                                                if nat_eq(a1504, a1505) {
                                                                                                    f133(4u64, v2, v4)?
                                                                                                } else {
                                                                                                    let a1506 = v0;
                                                                                                    let a1507 = 69u64;
                                                                                                    if nat_eq(a1506, a1507) {
                                                                                                        let a1508 = v5;
                                                                                                        let a1509 = 4u64;
                                                                                                        if nat_lt(a1508, a1509) {
                                                                                                            f133(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            Adt10::C1
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1510 = v0;
                                                                                                        let a1511 = 70u64;
                                                                                                        if nat_eq(a1510, a1511) {
                                                                                                            f134(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1512 = v0;
                                                                                                            let a1513 = 75u64;
                                                                                                            if if nat_eq(a1512, a1513) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1514 = v0;
                                                                                                                let a1515 = 76u64;
                                                                                                                nat_eq(a1514, a1515)
                                                                                                            } {
                                                                                                                f135({
                                                                                                                    let a1516 = v0;
                                                                                                                    let a1517 = 76u64;
                                                                                                                    nat_eq(a1516, a1517)
                                                                                                                }, v1, v2, v4)?
                                                                                                            } else {
                                                                                                                let a1518 = v0;
                                                                                                                let a1519 = 31u64;
                                                                                                                if if if nat_eq(a1518, a1519) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1520 = v0;
                                                                                                                    let a1521 = 47u64;
                                                                                                                    nat_eq(a1520, a1521)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1522 = v0;
                                                                                                                    let a1523 = 68u64;
                                                                                                                    nat_eq(a1522, a1523)
                                                                                                                } {
                                                                                                                    f136(v0, v1, v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1524 = v0;
                                                                                                                    let a1525 = 15u64;
                                                                                                                    if nat_eq(a1524, a1525) {
                                                                                                                        f137(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1526 = v0;
                                                                                                                        let a1527 = 17u64;
                                                                                                                        if nat_eq(a1526, a1527) {
                                                                                                                            f138(v2, v4)?
                                                                                                                        } else {
                                                                                                                            let a1528 = v0;
                                                                                                                            let a1529 = 18u64;
                                                                                                                            if nat_eq(a1528, a1529) {
                                                                                                                                f139(v2, v3, v4)?
                                                                                                                            } else {
                                                                                                                                let a1530 = v0;
                                                                                                                                let a1531 = 13u64;
                                                                                                                                if if if nat_eq(a1530, a1531) {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1532 = v0;
                                                                                                                                    let a1533 = 14u64;
                                                                                                                                    nat_eq(a1532, a1533)
                                                                                                                                } {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1534 = v0;
                                                                                                                                    let a1535 = 16u64;
                                                                                                                                    nat_eq(a1534, a1535)
                                                                                                                                } {
                                                                                                                                    f140(v0, v2, v4)?
                                                                                                                                } else {
                                                                                                                                    let m1536 = f141(v0, v5)?;
                                                                                                                                    match m1536 {
                                                                                                                                        None => {
                                                                                                                                            f142(v0, v5, v2, v3, v4)?
                                                                                                                                        }
                                                                                                                                        Some(v16) => {
                                                                                                                                            f143(v16, v2, v4)?
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    f144(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1537, _, _, _) => {
            h1537
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1538, _, _) => {
            h1538
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1539, _) => {
            h1539
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1540) => {
            h1540
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1542 = v0;
        let a1543 = match v1.clone() {
            Adt3::C0(h1541, _, _, _, _, _, _, _) => {
                h1541
            }
        };
        let a1544 = 0u64;
        nat_quot(a1542, a1543, a1544)
    };
    let v4: u64 = {
        let a1546 = v3;
        let a1547 = match v1.clone() {
            Adt3::C0(_, h1545, _, _, _, _, _, _) => {
                h1545
            }
        };
        let a1548 = 0u64;
        nat_quot(a1546, a1547, a1548)
    };
    let a1549 = v2;
    let a1550 = 0u64;
    if nat_eq(a1549, a1550) {
        let a1552 = v0;
        let a1553 = match v1 {
            Adt3::C0(h1551, _, _, _, _, _, _, _) => {
                h1551
            }
        };
        let a1554 = 0u64;
        nat_rem(a1552, a1553, a1554)
    } else {
        let a1555 = v2;
        let a1556 = 1u64;
        if nat_eq(a1555, a1556) {
            let a1558 = v3;
            let a1559 = match v1 {
                Adt3::C0(_, h1557, _, _, _, _, _, _) => {
                    h1557
                }
            };
            let a1560 = 0u64;
            nat_rem(a1558, a1559, a1560)
        } else {
            let a1561 = v2;
            let a1562 = 2u64;
            if nat_eq(a1561, a1562) {
                let a1564 = v4;
                let a1565 = match v1 {
                    Adt3::C0(_, _, h1563, _, _, _, _, _) => {
                        h1563
                    }
                };
                let a1566 = 0u64;
                nat_rem(a1564, a1565, a1566)
            } else {
                let a1568 = v4;
                let a1569 = match v1 {
                    Adt3::C0(_, _, h1567, _, _, _, _, _) => {
                        h1567
                    }
                };
                let a1570 = 0u64;
                nat_quot(a1568, a1569, a1570)
            }
        }
    }
}

pub fn f59(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1572 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1571) => {
            h1571
        }
    };
    let a1573 = 0u64;
    if nat_eq(a1572, a1573) {
        Ok(v2)
    } else {
        let a1578 = {
            let a1575 = v2;
            let a1576 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1574) => {
                    h1574
                }
            };
            nat_mul(a1575, a1576)?
        };
        let a1579 = f58(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1577, _, _, _, _, _, _) => {
                h1577
            }
        }, 3u64);
        nat_add(a1578, a1579)
    }
}

pub fn f60(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1589 = {
            let a1586 = {
                let a1581 = v2;
                let a1582 = match v0.clone() {
                    Adt6::C0(_, h1580, _, _) => {
                        h1580
                    }
                };
                nat_mul(a1581, a1582)?
            };
            let a1587 = {
                let a1584 = v3;
                let a1585 = match v0.clone() {
                    Adt6::C0(_, _, _, h1583) => {
                        h1583
                    }
                };
                nat_mul(a1584, a1585)?
            };
            nat_add(a1586, a1587)?
        };
        let a1590 = f145(match v0.clone() {
            Adt6::C0(_, _, h1588, _) => {
                h1588
            }
        });
        nat_add(a1589, a1590)?
    };
    let v5: u64 = f146(match v0 {
        Adt6::C0(_, _, h1591, _) => {
            h1591
        }
    });
    let a1592 = v5;
    let a1593 = v4;
    if if nat_le(a1592, a1593) {
        let a1596 = {
            let a1594 = v4;
            let a1595 = v5;
            nat_sub(a1594, a1595)
        };
        let a1597 = v1;
        nat_lt(a1596, a1597)
    } else {
        false
    } {
        Ok(Some({
            let a1598 = v4;
            let a1599 = v5;
            nat_sub(a1598, a1599)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1600, _, _, _, _, _) => {
            h1600
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1601, _, _, _, _, _, _) => {
            h1601
        }
    };
    let v7: u64 = f58(v1, v6.clone(), 2u64);
    let a1603 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1602) => {
            h1602
        }
    };
    let a1604 = 0u64;
    if nat_eq(a1603, a1604) {
        Ok(Adt9::C8(Some(f57(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1607 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1606, _, _, _, _) => {
                    h1606
                }
            };
            match m1607 {
                Adt3::C0(_, _, h1605, _, _, _, _, _) => {
                    h1605
                }
            }
        };
        let m1613 = f60(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1608, _) => {
                h1608
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1609, _, _, _, _, _) => {
                h1609
            }
        }, {
            let a1610 = v2;
            let a1611 = v8;
            let a1612 = 0u64;
            nat_rem(a1610, a1611, a1612)
        }, v7)?;
        match m1613 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1620 = {
                        let a1618 = {
                            let a1614 = v2;
                            let a1615 = v8;
                            let a1616 = 0u64;
                            nat_quot(a1614, a1615, a1616)
                        };
                        let a1619 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1617) => {
                                h1617
                            }
                        };
                        nat_mul(a1618, a1619)?
                    };
                    let a1621 = f58(v1, v6, 3u64);
                    nat_add(a1620, a1621)?
                };
                Ok(Adt9::C8(Some(f57(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1622 = v1;
    match m1622 {
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
            f147(f148(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f149(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f150(v0.clone(), v24);
            f147(f151(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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
    let m1623 = v3;
    match m1623 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f93(v0, v1)?, f93(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f153(v0, v1, v2.clone())?, f93(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f154(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f155(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f156(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f155(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f157(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f158(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f159(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f93(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f160(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v56, v57) => {
            f161(v0, v1, v2, v56, v57)
        }
        Adt4::C12(v58, v59, v60) => {
            f162(v0, v1, v2, v58, v59, v60)
        }
        Adt4::C13(_, v62, _, _) => {
            Ok(Adt9::C3(v62, v62, f93(v0, v1)?, f93(v0, v2)?))
        }
        Adt4::C14(v65, v66, v67) => {
            let a1626 = v65;
            let a1627 = 23u64;
            if nat_eq(a1626, a1627) {
                f163(v0, v1, v2, v66, v67)
            } else {
                Ok(Adt9::C3(v66, v67, f93(v0, v1)?, f93(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f164(v0, v1, v2)
        }
        Adt4::C16(v68, v69) => {
            f165(v0, v1, v2, v68, v69)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v70, v71) => {
            f166(v0, v1, v71, v70)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f93(f152(v0, match v2.clone() {
                Adt3::C0(h1624, _, _, _, _, _, _, _) => {
                    h1624
                }
            }, match v2.clone() {
                Adt3::C0(_, h1625, _, _, _, _, _, _) => {
                    h1625
                }
            })?, v1)?, f93(v0, v2)?))
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
        Adt4::C25(v76, v77, v78) => {
            f167(v0, v76, v77, v78)
        }
        Adt4::C26(v84, _, _, _) => {
            let a1628 = v84;
            let a1629 = 64u64;
            if if if nat_eq(a1628, a1629) {
                true
            } else {
                let a1630 = v84;
                let a1631 = 65u64;
                nat_eq(a1630, a1631)
            } {
                true
            } else {
                let a1632 = v84;
                let a1633 = 66u64;
                nat_eq(a1632, a1633)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v83) => {
            f85(v0, v1, v2, v83)
        }
        Adt4::C28(v79) => {
            f82(v0, v1, v2, v79, 0u64)
        }
        Adt4::C29(v80) => {
            f83(v0, v1, v2, v80)
        }
        Adt4::C30(v81, _) => {
            f84(v0, v1, v2, v81)
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
        Adt4::C43(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C44(_) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f168(v3, v4);
    let v6: u64 = {
        let a1634 = v0;
        let a1635 = v5;
        let a1636 = 0u64;
        nat_quot(a1634, a1635, a1636)
    };
    let a1637 = v5;
    let a1638 = 1u64;
    if nat_eq(a1637, a1638) {
        Ok(f147(f169(v3, v4), v3, v4, 0u64, f93(v6, v1)?, 2u64, 0u64, 3u64, f93(v6, v2)?))
    } else {
        let a1642 = {
            let a1639 = v0;
            let a1640 = 2u64;
            let a1641 = 0u64;
            nat_rem(a1639, a1640, a1641)
        };
        let a1643 = 0u64;
        if nat_eq(a1642, a1643) {
            Ok(f147(f169(v3, 0u64), v3, 0u64, 0u64, f93(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f147(f169(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f93(v6, v2)?))
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f170(v1)?;
    let v4: u64 = {
        let a1644 = v0;
        let a1645 = v3;
        let a1646 = 0u64;
        nat_rem(a1644, a1645, a1646)
    };
    let v5: u64 = {
        let a1649 = {
            let a1647 = v1;
            let a1648 = 3u64;
            nat_mul(a1647, a1648)?
        };
        let a1650 = 1u64;
        nat_add(a1649, a1650)?
    };
    let a1651 = v4;
    let a1652 = 0u64;
    if nat_eq(a1651, a1652) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1653 = v4;
        let a1654 = v5;
        if nat_eq(a1653, a1654) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1659 = v4;
                let a1660 = {
                    let a1655 = v4;
                    let a1656 = v5;
                    if nat_lt(a1655, a1656) {
                        1u64
                    } else {
                        let a1657 = v5;
                        let a1658 = 1u64;
                        nat_add(a1657, a1658)?
                    }
                };
                nat_sub(a1659, a1660)
            };
            let v7: u64 = {
                let a1661 = v6;
                let a1662 = 3u64;
                let a1663 = 0u64;
                nat_rem(a1661, a1662, a1663)
            };
            let v8: u64 = {
                let a1674 = 4u64;
                let a1675 = {
                    let a1672 = {
                        let a1667 = {
                            let a1664 = v0;
                            let a1665 = v3;
                            let a1666 = 0u64;
                            nat_quot(a1664, a1665, a1666)
                        };
                        let a1668 = v1;
                        nat_mul(a1667, a1668)?
                    };
                    let a1673 = {
                        let a1669 = v6;
                        let a1670 = 3u64;
                        let a1671 = 0u64;
                        nat_quot(a1669, a1670, a1671)
                    };
                    nat_add(a1672, a1673)?
                };
                nat_mul(a1674, a1675)?
            };
            let a1676 = v4;
            let a1677 = v5;
            if nat_lt(a1676, a1677) {
                let a1678 = v7;
                let a1679 = 0u64;
                if nat_eq(a1678, a1679) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1680 = v7;
                    let a1681 = 1u64;
                    if nat_eq(a1680, a1681) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1682 = v7;
                let a1683 = 0u64;
                if nat_eq(a1682, a1683) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1684 = v7;
                    let a1685 = 1u64;
                    if nat_eq(a1684, a1685) {
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
        let a1686 = v0;
        let a1687 = 6u64;
        let a1688 = 0u64;
        nat_quot(a1686, a1687, a1688)
    };
    let v5: u64 = {
        let a1689 = v0;
        let a1690 = 6u64;
        let a1691 = 0u64;
        nat_rem(a1689, a1690, a1691)
    };
    let a1692 = v5;
    let a1693 = 0u64;
    if nat_eq(a1692, a1693) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f93(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1694 = v5;
        let a1695 = 1u64;
        if nat_eq(a1694, a1695) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1696 = v5;
            let a1697 = 2u64;
            if nat_eq(a1696, a1697) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f93(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1698 = v5;
                let a1699 = 3u64;
                if nat_eq(a1698, a1699) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1700 = v5;
                    let a1701 = 4u64;
                    if nat_eq(a1700, a1701) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f93(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f93(v4, v3)?), 4u64, 4u64))
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
    let v7: u64 = f171(match v1.clone() {
        Adt3::C0(_, h1702, _, _, _, _, _, _) => {
            h1702
        }
    }, v4)?;
    let v8: u64 = {
        let a1703 = v0;
        let a1704 = v7;
        let a1705 = 0u64;
        nat_rem(a1703, a1704, a1705)
    };
    let v9: u64 = {
        let a1706 = v0;
        let a1707 = v7;
        let a1708 = 0u64;
        nat_quot(a1706, a1707, a1708)
    };
    let a1711 = {
        let a1709 = v8;
        let a1710 = 1u64;
        nat_add(a1709, a1710)?
    };
    let a1712 = v7;
    if nat_eq(a1711, a1712) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f93(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1713 = v8;
            let a1714 = v6;
            let a1715 = 0u64;
            nat_quot(a1713, a1714, a1715)
        };
        let v11: u64 = {
            let a1716 = v8;
            let a1717 = v6;
            let a1718 = 0u64;
            nat_rem(a1716, a1717, a1718)
        };
        let v12: u64 = {
            let a1720 = v9;
            let a1721 = match v1.clone() {
                Adt3::C0(h1719, _, _, _, _, _, _, _) => {
                    h1719
                }
            };
            let a1722 = 0u64;
            nat_rem(a1720, a1721, a1722)
        };
        let v13: u64 = {
            let a1724 = v9;
            let a1725 = match v1.clone() {
                Adt3::C0(h1723, _, _, _, _, _, _, _) => {
                    h1723
                }
            };
            let a1726 = 0u64;
            nat_quot(a1724, a1725, a1726)
        };
        let v14: u64 = if v4 {
            let a1738 = {
                let a1733 = {
                    let a1728 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1727, _, _, _) => {
                            h1727
                        }
                    };
                    let a1729 = v12;
                    nat_mul(a1728, a1729)?
                };
                let a1734 = {
                    let a1731 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1730, _, _) => {
                            h1730
                        }
                    };
                    let a1732 = v10;
                    nat_mul(a1731, a1732)?
                };
                nat_add(a1733, a1734)?
            };
            let a1739 = {
                let a1736 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1735, _) => {
                        h1735
                    }
                };
                let a1737 = v13;
                nat_mul(a1736, a1737)?
            };
            nat_add(a1738, a1739)?
        } else {
            let a1746 = {
                let a1741 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1740, _, _, _) => {
                        h1740
                    }
                };
                let a1742 = v10;
                nat_mul(a1741, a1742)?
            };
            let a1747 = {
                let a1744 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1743, _, _) => {
                        h1743
                    }
                };
                let a1745 = v13;
                nat_mul(a1744, a1745)?
            };
            nat_add(a1746, a1747)?
        };
        if if v4 {
            let a1748 = v11;
            let a1749 = 0u64;
            nat_eq(a1748, a1749)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1752 = {
                let a1750 = v11;
                let a1751 = 1u64;
                nat_add(a1750, a1751)?
            };
            let a1753 = v6;
            if nat_eq(a1752, a1753) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1754 = v10;
                    let a1755 = 0u64;
                    if nat_eq(a1754, a1755) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1767 = {
                        let a1762 = {
                            let a1757 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1756, _, _, _) => {
                                    h1756
                                }
                            };
                            let a1758 = v12;
                            nat_mul(a1757, a1758)?
                        };
                        let a1763 = {
                            let a1760 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1759, _, _) => {
                                    h1759
                                }
                            };
                            let a1761 = v10;
                            nat_mul(a1760, a1761)?
                        };
                        nat_add(a1762, a1763)?
                    };
                    let a1768 = {
                        let a1765 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1764, _) => {
                                h1764
                            }
                        };
                        let a1766 = v13;
                        nat_mul(a1765, a1766)?
                    };
                    nat_add(a1767, a1768)?
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
        Adt3::C0(_, h1769, _, _, _, _, _, _) => {
            h1769
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1770 = v0;
        let a1771 = v6;
        let a1772 = 0u64;
        nat_rem(a1770, a1771, a1772)
    };
    let v8: u64 = {
        let a1773 = v0;
        let a1774 = v6;
        let a1775 = 0u64;
        nat_quot(a1773, a1774, a1775)
    };
    let v9: u64 = {
        let a1777 = v8;
        let a1778 = match v5.clone() {
            Adt3::C0(h1776, _, _, _, _, _, _, _) => {
                h1776
            }
        };
        let a1779 = 0u64;
        nat_rem(a1777, a1778, a1779)
    };
    let v10: u64 = {
        let a1781 = v8;
        let a1782 = match v5.clone() {
            Adt3::C0(h1780, _, _, _, _, _, _, _) => {
                h1780
            }
        };
        let a1783 = 0u64;
        nat_quot(a1781, a1782, a1783)
    };
    let v11: u64 = {
        let a1785 = v10;
        let a1786 = match v2.clone() {
            Adt3::C0(_, h1784, _, _, _, _, _, _) => {
                h1784
            }
        };
        let a1787 = 0u64;
        nat_rem(a1785, a1786, a1787)
    };
    let v12: u64 = {
        let a1789 = v10;
        let a1790 = match v2.clone() {
            Adt3::C0(_, h1788, _, _, _, _, _, _) => {
                h1788
            }
        };
        let a1791 = 0u64;
        nat_quot(a1789, a1790, a1791)
    };
    let a1792 = v7;
    let a1793 = 0u64;
    if nat_eq(a1792, a1793) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1800 = {
                let a1795 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1794, _, _, _) => {
                        h1794
                    }
                };
                let a1796 = v9;
                nat_mul(a1795, a1796)?
            };
            let a1801 = {
                let a1798 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1797, _, _) => {
                        h1797
                    }
                };
                let a1799 = v12;
                nat_mul(a1798, a1799)?
            };
            nat_add(a1800, a1801)?
        }), Adt12::C0(2u64, {
            let a1808 = {
                let a1803 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1802, _, _, _) => {
                        h1802
                    }
                };
                let a1804 = v11;
                nat_mul(a1803, a1804)?
            };
            let a1809 = {
                let a1806 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1805, _, _) => {
                        h1805
                    }
                };
                let a1807 = v12;
                nat_mul(a1806, a1807)?
            };
            nat_add(a1808, a1809)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1810 = v4;
        match m1810 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1822 = {
                        let a1817 = {
                            let a1812 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1811, _, _, _) => {
                                    h1811
                                }
                            };
                            let a1813 = v9;
                            nat_mul(a1812, a1813)?
                        };
                        let a1818 = {
                            let a1815 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1814, _, _) => {
                                    h1814
                                }
                            };
                            let a1816 = v11;
                            nat_mul(a1815, a1816)?
                        };
                        nat_add(a1817, a1818)?
                    };
                    let a1823 = {
                        let a1820 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1819, _) => {
                                h1819
                            }
                        };
                        let a1821 = v12;
                        nat_mul(a1820, a1821)?
                    };
                    nat_add(a1822, a1823)?
                }), Adt13::C1(f93(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1824 = v7;
                    let a1825 = 1u64;
                    nat_sub(a1824, a1825)
                };
                let v15: u64 = {
                    let a1826 = v14;
                    let a1827 = 2u64;
                    let a1828 = 0u64;
                    nat_quot(a1826, a1827, a1828)
                };
                let a1832 = {
                    let a1829 = v14;
                    let a1830 = 2u64;
                    let a1831 = 0u64;
                    nat_rem(a1829, a1830, a1831)
                };
                let a1833 = 0u64;
                if nat_eq(a1832, a1833) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1845 = {
                            let a1840 = {
                                let a1835 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1834, _, _, _) => {
                                        h1834
                                    }
                                };
                                let a1836 = v9;
                                nat_mul(a1835, a1836)?
                            };
                            let a1841 = {
                                let a1838 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1837, _, _) => {
                                        h1837
                                    }
                                };
                                let a1839 = v15;
                                nat_mul(a1838, a1839)?
                            };
                            nat_add(a1840, a1841)?
                        };
                        let a1846 = {
                            let a1843 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1842, _) => {
                                    h1842
                                }
                            };
                            let a1844 = v12;
                            nat_mul(a1843, a1844)?
                        };
                        nat_add(a1845, a1846)?
                    }), Adt12::C0(3u64, {
                        let a1858 = {
                            let a1853 = {
                                let a1848 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1847, _, _, _) => {
                                        h1847
                                    }
                                };
                                let a1849 = v11;
                                nat_mul(a1848, a1849)?
                            };
                            let a1854 = {
                                let a1851 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1850, _, _) => {
                                        h1850
                                    }
                                };
                                let a1852 = v15;
                                nat_mul(a1851, a1852)?
                            };
                            nat_add(a1853, a1854)?
                        };
                        let a1859 = {
                            let a1856 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1855, _) => {
                                    h1855
                                }
                            };
                            let a1857 = v12;
                            nat_mul(a1856, a1857)?
                        };
                        nat_add(a1858, a1859)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1862 = {
                            let a1860 = v7;
                            let a1861 = 1u64;
                            nat_add(a1860, a1861)?
                        };
                        let a1863 = v6;
                        if nat_eq(a1862, a1863) {
                            Adt13::C1(f93(v8, v5)?)
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
        let a1864 = v0;
        let a1865 = v8;
        let a1866 = 0u64;
        nat_rem(a1864, a1865, a1866)
    };
    let v10: u64 = {
        let a1867 = v0;
        let a1868 = v8;
        let a1869 = 0u64;
        nat_quot(a1867, a1868, a1869)
    };
    let a1870 = v9;
    let a1871 = 148u64;
    if nat_lt(a1870, a1871) {
        f172({
            let a1872 = v9;
            let a1873 = 37u64;
            let a1874 = 0u64;
            nat_rem(a1872, a1873, a1874)
        }, {
            let a1875 = v9;
            let a1876 = 37u64;
            let a1877 = 0u64;
            nat_quot(a1875, a1876, a1877)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f173({
            let a1878 = v9;
            let a1879 = 148u64;
            nat_sub(a1878, a1879)
        }, v10, v4, v5)
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f28(v2);
    let a1880 = v0;
    let a1881 = v3;
    if nat_lt(a1880, a1881) {
        let a1882 = v0;
        let a1883 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1882, a1883) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1884 = 4u64;
                let a1885 = v0;
                nat_mul(a1884, a1885)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1886 = v0;
            let a1887 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1886, a1887) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1890 = v0;
                let a1891 = {
                    let a1888 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1889 = 1u64;
                    nat_add(a1888, a1889)?
                };
                if nat_eq(a1890, a1891) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1892 = v0;
                    let a1893 = 9u64;
                    if nat_eq(a1892, a1893) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1894 = v0;
            let a1895 = v3;
            nat_sub(a1894, a1895)
        };
        let v5: u64 = f29(v2);
        let v6: u64 = {
            let a1896 = v4;
            let a1897 = v5;
            let a1898 = 0u64;
            nat_rem(a1896, a1897, a1898)
        };
        let v7: u64 = f93({
            let a1899 = v4;
            let a1900 = v5;
            let a1901 = 0u64;
            nat_quot(a1899, a1900, a1901)
        }, v1)?;
        if v2 {
            let a1902 = v6;
            let a1903 = 7u64;
            if nat_lt(a1902, a1903) {
                let a1904 = v6;
                let a1905 = 3u64;
                if nat_lt(a1904, a1905) {
                    let a1906 = v6;
                    let a1907 = 1u64;
                    if nat_lt(a1906, a1907) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1908 = v6;
                        let a1909 = 2u64;
                        if nat_lt(a1908, a1909) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1910 = v6;
                    let a1911 = 5u64;
                    if nat_lt(a1910, a1911) {
                        let a1912 = v6;
                        let a1913 = 4u64;
                        if nat_lt(a1912, a1913) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1914 = v6;
                        let a1915 = 6u64;
                        if nat_lt(a1914, a1915) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1916 = v6;
                let a1917 = 11u64;
                if nat_lt(a1916, a1917) {
                    let a1918 = v6;
                    let a1919 = 9u64;
                    if nat_lt(a1918, a1919) {
                        let a1920 = v6;
                        let a1921 = 8u64;
                        if nat_lt(a1920, a1921) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1922 = v6;
                        let a1923 = 10u64;
                        if nat_lt(a1922, a1923) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1924 = v6;
                    let a1925 = 13u64;
                    if nat_lt(a1924, a1925) {
                        let a1926 = v6;
                        let a1927 = 12u64;
                        if nat_lt(a1926, a1927) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1928 = v6;
                        let a1929 = 14u64;
                        if nat_lt(a1928, a1929) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1930 = v6;
            let a1931 = 1u64;
            if nat_lt(a1930, a1931) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1932 = v6;
                let a1933 = 2u64;
                if nat_lt(a1932, a1933) {
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
        Adt3::C0(h1934, _, _, _, _, _, _, _) => {
            h1934
        }
    };
    let v8: u64 = f24(v3, v7)?;
    let v9: u64 = {
        let a1935 = v0;
        let a1936 = 2u64;
        nat_sub(a1935, a1936)
    };
    let v10: u64 = {
        let a1937 = v9;
        let a1938 = v8;
        let a1939 = 0u64;
        nat_quot(a1937, a1938, a1939)
    };
    let v11: u64 = {
        let a1940 = v9;
        let a1941 = v8;
        let a1942 = 0u64;
        nat_rem(a1940, a1941, a1942)
    };
    let a1943 = v0;
    let a1944 = 0u64;
    if nat_eq(a1943, a1944) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a1945 = v0;
        let a1946 = 1u64;
        if nat_eq(a1945, a1946) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a1947 = v10;
            let a1948 = v5;
            if nat_eq(a1947, a1948) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a1951 = v11;
                let a1952 = {
                    let a1949 = 2u64;
                    let a1950 = v7;
                    nat_mul(a1949, a1950)?
                };
                if nat_lt(a1951, a1952) {
                    let v12: u64 = {
                        let a1953 = v11;
                        let a1954 = 2u64;
                        let a1955 = 0u64;
                        nat_quot(a1953, a1954, a1955)
                    };
                    let a1959 = {
                        let a1956 = v11;
                        let a1957 = 2u64;
                        let a1958 = 0u64;
                        nat_rem(a1956, a1957, a1958)
                    };
                    let a1960 = 0u64;
                    if nat_eq(a1959, a1960) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a1961 = v12;
                            let a1962 = 0u64;
                            if nat_eq(a1961, a1962) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a1968 = {
                                let a1964 = v10;
                                let a1965 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h1963, _, _) => {
                                        h1963
                                    }
                                };
                                nat_mul(a1964, a1965)?
                            };
                            let a1969 = {
                                let a1966 = 4u64;
                                let a1967 = v12;
                                nat_mul(a1966, a1967)?
                            };
                            nat_add(a1968, a1969)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a1970 = v6;
                            let a1971 = 2u64;
                            if nat_eq(a1970, a1971) {
                                let a1972 = v12;
                                let a1973 = 0u64;
                                if nat_eq(a1972, a1973) {
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
                                    let a1979 = {
                                        let a1975 = v10;
                                        let a1976 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1974, _, _) => {
                                                h1974
                                            }
                                        };
                                        nat_mul(a1975, a1976)?
                                    };
                                    let a1980 = {
                                        let a1977 = 4u64;
                                        let a1978 = v12;
                                        nat_mul(a1977, a1978)?
                                    };
                                    nat_add(a1979, a1980)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a1983 = v11;
                    let a1984 = {
                        let a1981 = 6u64;
                        let a1982 = v7;
                        nat_mul(a1981, a1982)?
                    };
                    if nat_lt(a1983, a1984) {
                        let v13: u64 = {
                            let a1987 = v11;
                            let a1988 = {
                                let a1985 = 2u64;
                                let a1986 = v7;
                                nat_mul(a1985, a1986)?
                            };
                            nat_sub(a1987, a1988)
                        };
                        let v14: u64 = {
                            let a1989 = v13;
                            let a1990 = 4u64;
                            let a1991 = 0u64;
                            nat_quot(a1989, a1990, a1991)
                        };
                        let a1995 = {
                            let a1992 = v13;
                            let a1993 = 4u64;
                            let a1994 = 0u64;
                            nat_rem(a1992, a1993, a1994)
                        };
                        let a1996 = 2u64;
                        if nat_lt(a1995, a1996) {
                            let a2000 = {
                                let a1997 = v13;
                                let a1998 = 4u64;
                                let a1999 = 0u64;
                                nat_rem(a1997, a1998, a1999)
                            };
                            let a2001 = 1u64;
                            if nat_lt(a2000, a2001) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a2007 = {
                                        let a2003 = v10;
                                        let a2004 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2002, _, _) => {
                                                h2002
                                            }
                                        };
                                        nat_mul(a2003, a2004)?
                                    };
                                    let a2008 = {
                                        let a2005 = 4u64;
                                        let a2006 = v14;
                                        nat_mul(a2005, a2006)?
                                    };
                                    nat_add(a2007, a2008)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2014 = {
                                            let a2010 = v10;
                                            let a2011 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2009, _, _) => {
                                                    h2009
                                                }
                                            };
                                            nat_mul(a2010, a2011)?
                                        };
                                        let a2015 = {
                                            let a2012 = 4u64;
                                            let a2013 = v14;
                                            nat_mul(a2012, a2013)?
                                        };
                                        nat_add(a2014, a2015)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2019 = {
                                let a2016 = v13;
                                let a2017 = 4u64;
                                let a2018 = 0u64;
                                nat_rem(a2016, a2017, a2018)
                            };
                            let a2020 = 3u64;
                            if nat_lt(a2019, a2020) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2026 = {
                                            let a2022 = v10;
                                            let a2023 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2021, _, _) => {
                                                    h2021
                                                }
                                            };
                                            nat_mul(a2022, a2023)?
                                        };
                                        let a2027 = {
                                            let a2024 = 4u64;
                                            let a2025 = v14;
                                            nat_mul(a2024, a2025)?
                                        };
                                        nat_add(a2026, a2027)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2028 = v14;
                                    let a2029 = 0u64;
                                    if nat_eq(a2028, a2029) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
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
                        if nat_lt(a2034, a2035) {
                            let a2038 = v11;
                            let a2039 = {
                                let a2036 = 6u64;
                                let a2037 = v7;
                                nat_mul(a2036, a2037)?
                            };
                            if nat_eq(a2038, a2039) {
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
                                    let a2044 = v11;
                                    let a2045 = {
                                        let a2042 = {
                                            let a2040 = 6u64;
                                            let a2041 = v7;
                                            nat_mul(a2040, a2041)?
                                        };
                                        let a2043 = 2u64;
                                        nat_add(a2042, a2043)?
                                    };
                                    nat_sub(a2044, a2045)
                                };
                                let v16: u64 = {
                                    let a2046 = v15;
                                    let a2047 = 3u64;
                                    let a2048 = 0u64;
                                    nat_quot(a2046, a2047, a2048)
                                };
                                let a2052 = {
                                    let a2049 = v15;
                                    let a2050 = 3u64;
                                    let a2051 = 0u64;
                                    nat_rem(a2049, a2050, a2051)
                                };
                                let a2053 = 1u64;
                                if nat_lt(a2052, a2053) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2059 = {
                                            let a2055 = v10;
                                            let a2056 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2054, _, _) => {
                                                    h2054
                                                }
                                            };
                                            nat_mul(a2055, a2056)?
                                        };
                                        let a2060 = {
                                            let a2057 = 4u64;
                                            let a2058 = v16;
                                            nat_mul(a2057, a2058)?
                                        };
                                        nat_add(a2059, a2060)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2064 = {
                                        let a2061 = v15;
                                        let a2062 = 3u64;
                                        let a2063 = 0u64;
                                        nat_rem(a2061, a2062, a2063)
                                    };
                                    let a2065 = 2u64;
                                    if nat_lt(a2064, a2065) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2070 = {
                                                let a2066 = v10;
                                                let a2067 = v4;
                                                nat_mul(a2066, a2067)?
                                            };
                                            let a2071 = {
                                                let a2068 = 4u64;
                                                let a2069 = v16;
                                                nat_mul(a2068, a2069)?
                                            };
                                            nat_add(a2070, a2071)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2077 = {
                                                let a2073 = v10;
                                                let a2074 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2072, _, _) => {
                                                        h2072
                                                    }
                                                };
                                                nat_mul(a2073, a2074)?
                                            };
                                            let a2078 = {
                                                let a2075 = 4u64;
                                                let a2076 = v16;
                                                nat_mul(a2075, a2076)?
                                            };
                                            nat_add(a2077, a2078)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2083 = v11;
                                let a2084 = {
                                    let a2081 = {
                                        let a2079 = 11u64;
                                        let a2080 = v7;
                                        nat_mul(a2079, a2080)?
                                    };
                                    let a2082 = 2u64;
                                    nat_add(a2081, a2082)?
                                };
                                if nat_lt(a2083, a2084) {
                                    let v17: u64 = {
                                        let a2089 = v11;
                                        let a2090 = {
                                            let a2087 = {
                                                let a2085 = 6u64;
                                                let a2086 = v7;
                                                nat_mul(a2085, a2086)?
                                            };
                                            let a2088 = 2u64;
                                            nat_add(a2087, a2088)?
                                        };
                                        nat_sub(a2089, a2090)
                                    };
                                    let v18: u64 = {
                                        let a2091 = v17;
                                        let a2092 = 5u64;
                                        let a2093 = 0u64;
                                        nat_quot(a2091, a2092, a2093)
                                    };
                                    let a2097 = {
                                        let a2094 = v17;
                                        let a2095 = 5u64;
                                        let a2096 = 0u64;
                                        nat_rem(a2094, a2095, a2096)
                                    };
                                    let a2098 = 2u64;
                                    if nat_lt(a2097, a2098) {
                                        let a2102 = {
                                            let a2099 = v17;
                                            let a2100 = 5u64;
                                            let a2101 = 0u64;
                                            nat_rem(a2099, a2100, a2101)
                                        };
                                        let a2103 = 1u64;
                                        if nat_lt(a2102, a2103) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2109 = {
                                                    let a2105 = v10;
                                                    let a2106 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2104, _, _) => {
                                                            h2104
                                                        }
                                                    };
                                                    nat_mul(a2105, a2106)?
                                                };
                                                let a2110 = {
                                                    let a2107 = 4u64;
                                                    let a2108 = v18;
                                                    nat_mul(a2107, a2108)?
                                                };
                                                nat_add(a2109, a2110)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2114 = {
                                            let a2111 = v17;
                                            let a2112 = 5u64;
                                            let a2113 = 0u64;
                                            nat_rem(a2111, a2112, a2113)
                                        };
                                        let a2115 = 3u64;
                                        if nat_lt(a2114, a2115) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2120 = {
                                                    let a2116 = v10;
                                                    let a2117 = v4;
                                                    nat_mul(a2116, a2117)?
                                                };
                                                let a2121 = {
                                                    let a2118 = 4u64;
                                                    let a2119 = v18;
                                                    nat_mul(a2118, a2119)?
                                                };
                                                nat_add(a2120, a2121)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2125 = {
                                                let a2122 = v17;
                                                let a2123 = 5u64;
                                                let a2124 = 0u64;
                                                nat_rem(a2122, a2123, a2124)
                                            };
                                            let a2126 = 4u64;
                                            if nat_lt(a2125, a2126) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2127 = v18;
                                                    let a2128 = 0u64;
                                                    if nat_eq(a2127, a2128) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2133 = v11;
                                    let a2134 = {
                                        let a2131 = {
                                            let a2129 = 11u64;
                                            let a2130 = v7;
                                            nat_mul(a2129, a2130)?
                                        };
                                        let a2132 = 2u64;
                                        nat_add(a2131, a2132)?
                                    };
                                    if nat_eq(a2133, a2134) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2135 = v10;
                                            let a2136 = 0u64;
                                            if nat_eq(a2135, a2136) {
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

pub fn f72(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt3::C0(h2137, _, _, _, _, _, _, _) => {
            h2137
        }
    };
    let v4: u64 = match v1 {
        Adt3::C0(_, h2138, _, _, _, _, _, _) => {
            h2138
        }
    };
    let v5: u64 = {
        let a2139 = v3;
        let a2140 = 2u64;
        let a2141 = 0u64;
        nat_quot(a2139, a2140, a2141)
    };
    let v6: u64 = {
        let a2144 = 5u64;
        let a2145 = {
            let a2142 = 3u64;
            let a2143 = v4;
            nat_mul(a2142, a2143)?
        };
        nat_add(a2144, a2145)?
    };
    let v7: u64 = {
        let a2146 = v5;
        let a2147 = 0u64;
        if nat_eq(a2146, a2147) {
            0u64
        } else {
            let a2150 = 7u64;
            let a2151 = {
                let a2148 = v5;
                let a2149 = v6;
                nat_mul(a2148, a2149)?
            };
            nat_add(a2150, a2151)?
        }
    };
    let a2152 = v7;
    let a2153 = v0;
    if nat_le(a2152, a2153) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2162 = 4u64;
            let a2163 = {
                let a2160 = {
                    let a2156 = {
                        let a2154 = v0;
                        let a2155 = v7;
                        nat_sub(a2154, a2155)
                    };
                    let a2157 = v3;
                    nat_mul(a2156, a2157)?
                };
                let a2161 = {
                    let a2158 = 2u64;
                    let a2159 = v5;
                    nat_mul(a2158, a2159)?
                };
                nat_add(a2160, a2161)?
            };
            nat_mul(a2162, a2163)?
        }), 4u64, 4u64))
    } else {
        let a2164 = v0;
        let a2165 = 7u64;
        if nat_lt(a2164, a2165) {
            let a2166 = v0;
            let a2167 = 0u64;
            if nat_eq(a2166, a2167) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2({
                    let a2168 = f146(v2);
                    let a2169 = f145(v2);
                    nat_add(a2168, a2169)?
                }), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2170 = v0;
                let a2171 = 1u64;
                if nat_eq(a2170, a2171) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2172 = v0;
                    let a2173 = 2u64;
                    if nat_eq(a2172, a2173) {
                        let a2174 = v2;
                        let a2175 = 2147483648u64;
                        if nat_lt(a2174, a2175) {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    } else {
                        let a2176 = v0;
                        let a2177 = 3u64;
                        if nat_eq(a2176, a2177) {
                            Ok(Adt9::C7(Adt11::C7(13u64, 0u64, 0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            let a2178 = v0;
                            let a2179 = 4u64;
                            if nat_eq(a2178, a2179) {
                                Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                let a2180 = v0;
                                let a2181 = 5u64;
                                if nat_eq(a2180, a2181) {
                                    Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                } else {
                                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let v8: u64 = {
                let a2182 = v0;
                let a2183 = 7u64;
                nat_sub(a2182, a2183)
            };
            let v9: u64 = {
                let a2184 = v8;
                let a2185 = v6;
                let a2186 = 0u64;
                nat_quot(a2184, a2185, a2186)
            };
            let v10: u64 = {
                let a2187 = v8;
                let a2188 = v6;
                let a2189 = 0u64;
                nat_rem(a2187, a2188, a2189)
            };
            let a2190 = v10;
            let a2191 = 5u64;
            if nat_lt(a2190, a2191) {
                let a2192 = v10;
                let a2193 = 0u64;
                if nat_eq(a2192, a2193) {
                    Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v9), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                } else {
                    let a2194 = v10;
                    let a2195 = 1u64;
                    if nat_eq(a2194, a2195) {
                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                    } else {
                        let a2196 = v10;
                        let a2197 = 2u64;
                        if nat_eq(a2196, a2197) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(3u64), 4u64, 4u64))
                        } else {
                            let a2198 = v10;
                            let a2199 = 3u64;
                            if nat_eq(a2198, a2199) {
                                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            }
                        }
                    }
                }
            } else {
                let v11: u64 = {
                    let a2200 = v10;
                    let a2201 = 5u64;
                    nat_sub(a2200, a2201)
                };
                let v12: u64 = {
                    let a2202 = v11;
                    let a2203 = 3u64;
                    let a2204 = 0u64;
                    nat_quot(a2202, a2203, a2204)
                };
                let v13: u64 = {
                    let a2205 = v11;
                    let a2206 = 3u64;
                    let a2207 = 0u64;
                    nat_rem(a2205, a2206, a2207)
                };
                let a2208 = v13;
                let a2209 = 0u64;
                if nat_eq(a2208, a2209) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                        let a2210 = 4u64;
                        let a2211 = v12;
                        nat_mul(a2210, a2211)?
                    }), Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                } else {
                    let a2212 = v13;
                    let a2213 = 1u64;
                    if nat_eq(a2212, a2213) {
                        Ok(Adt9::C7(Adt11::C7(15u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2218 = 4u64;
                            let a2219 = {
                                let a2216 = {
                                    let a2214 = v12;
                                    let a2215 = v3;
                                    nat_mul(a2214, a2215)?
                                };
                                let a2217 = v9;
                                nat_add(a2216, a2217)?
                            };
                            nat_mul(a2218, a2219)?
                        }), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C7(14u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2226 = 4u64;
                            let a2227 = {
                                let a2224 = {
                                    let a2220 = v12;
                                    let a2221 = v3;
                                    nat_mul(a2220, a2221)?
                                };
                                let a2225 = {
                                    let a2222 = v5;
                                    let a2223 = v9;
                                    nat_add(a2222, a2223)?
                                };
                                nat_add(a2224, a2225)?
                            };
                            nat_mul(a2226, a2227)?
                        }), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64, v5: u64) -> R<Adt9> {
    let a2228 = v0;
    let a2229 = v4;
    if nat_lt(a2228, a2229) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f93(v0, v2)?), 4u64, 4u64))
    } else {
        let v6: u64 = {
            let a2236 = {
                let m2232 = match v3.clone() {
                    Adt7::C0(h2231, _, _, _, _) => {
                        h2231
                    }
                };
                match m2232 {
                    Adt6::C0(h2230, _, _, _) => {
                        h2230
                    }
                }
            };
            let a2237 = {
                let m2235 = match v3.clone() {
                    Adt7::C0(_, h2234, _, _, _) => {
                        h2234
                    }
                };
                match m2235 {
                    Adt6::C0(h2233, _, _, _) => {
                        h2233
                    }
                }
            };
            nat_mul(a2236, a2237)?
        };
        let v7: u64 = f174(match v3.clone() {
            Adt7::C0(_, _, _, h2238, _) => {
                h2238
            }
        }, v6)?;
        let v8: u64 = {
            let a2239 = v0;
            let a2240 = v4;
            nat_sub(a2239, a2240)
        };
        let v9: u64 = {
            let a2241 = v8;
            let a2242 = v7;
            let a2243 = 0u64;
            nat_quot(a2241, a2242, a2243)
        };
        let v10: u64 = {
            let a2244 = v8;
            let a2245 = v7;
            let a2246 = 0u64;
            nat_rem(a2244, a2245, a2246)
        };
        if match v3.clone() {
            Adt7::C0(_, _, _, h2247, _) => {
                h2247
            }
        } {
            let a2248 = v10;
            let a2249 = 0u64;
            if nat_eq(a2248, a2249) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(4286578687u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2250 = v10;
                let a2251 = 1u64;
                if nat_eq(a2250, a2251) {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2254 = {
                        let a2252 = v10;
                        let a2253 = 2u64;
                        nat_add(a2252, a2253)?
                    };
                    let a2255 = v7;
                    if nat_eq(a2254, a2255) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a2258 = {
                            let a2256 = v10;
                            let a2257 = 1u64;
                            nat_add(a2256, a2257)?
                        };
                        let a2259 = v7;
                        if nat_eq(a2258, a2259) {
                            let a2260 = v5;
                            let a2261 = 0u64;
                            if nat_eq(a2260, a2261) {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3({
                                    let a2262 = v5;
                                    let a2263 = 1u64;
                                    nat_sub(a2262, a2263)
                                }), Adt12::C0(0u64, f93(v9, v1)?), Adt13::C1({
                                    let a2264 = v5;
                                    let a2265 = 1u64;
                                    nat_sub(a2264, a2265)
                                }), 4u64, 4u64))
                            }
                        } else {
                            let v11: u64 = {
                                let a2266 = v10;
                                let a2267 = 2u64;
                                nat_sub(a2266, a2267)
                            };
                            let v12: u64 = {
                                let a2268 = v11;
                                let a2269 = 3u64;
                                let a2270 = 0u64;
                                nat_rem(a2268, a2269, a2270)
                            };
                            let m2274 = f175(v3, v2, v1, v9, {
                                let a2271 = v11;
                                let a2272 = 3u64;
                                let a2273 = 0u64;
                                nat_quot(a2271, a2272, a2273)
                            })?;
                            match m2274 {
                                None => {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                                Some(v13) => {
                                    let a2275 = v12;
                                    let a2276 = 0u64;
                                    if nat_eq(a2275, a2276) {
                                        Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C0(1u64, v13), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                                    } else {
                                        let a2277 = 2u64;
                                        let a2278 = v5;
                                        if nat_le(a2277, a2278) {
                                            let a2279 = v12;
                                            let a2280 = 1u64;
                                            if nat_eq(a2279, a2280) {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(1u64, v13), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                                    let a2281 = v13;
                                                    let a2282 = 1u64;
                                                    nat_add(a2281, a2282)?
                                                }), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                                            }
                                        } else {
                                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let a2283 = v10;
            let a2284 = 0u64;
            if nat_eq(a2283, a2284) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v6), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2285 = v10;
                let a2286 = 1u64;
                if nat_eq(a2285, a2286) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2287 = v10;
                    let a2288 = 2u64;
                    if nat_eq(a2287, a2288) {
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f93(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2291 = f175(v3, v2, v1, v9, {
                            let a2289 = v10;
                            let a2290 = 3u64;
                            nat_sub(a2289, a2290)
                        })?;
                        match m2291 {
                            None => {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            }
                            Some(v14) => {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C1(v14), 4u64, 4u64))
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2294 = match v3.clone() {
            Adt6::C0(h2292, _, _, _) => {
                h2292
            }
        };
        let a2295 = match v4.clone() {
            Adt6::C0(h2293, _, _, _) => {
                h2293
            }
        };
        nat_mul(a2294, a2295)?
    };
    let v6: u64 = {
        let a2296 = v5;
        let a2297 = 2u64;
        nat_add(a2296, a2297)?
    };
    let v7: u64 = {
        let a2298 = v0;
        let a2299 = v6;
        let a2300 = 0u64;
        nat_quot(a2298, a2299, a2300)
    };
    let v8: u64 = {
        let a2301 = v0;
        let a2302 = v6;
        let a2303 = 0u64;
        nat_rem(a2301, a2302, a2303)
    };
    let a2304 = v8;
    let a2305 = 0u64;
    if nat_eq(a2304, a2305) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2308 = {
            let a2306 = v8;
            let a2307 = 1u64;
            nat_add(a2306, a2307)?
        };
        let a2309 = v6;
        if nat_eq(a2308, a2309) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f93(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2310 = v8;
                let a2311 = 1u64;
                nat_sub(a2310, a2311)
            };
            let m2317 = f176(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2312, _, _, _, _, _, _) => {
                    h2312
                }
            }, f58(v7, v2.clone(), 0u64), {
                let a2314 = v9;
                let a2315 = match v3.clone() {
                    Adt6::C0(h2313, _, _, _) => {
                        h2313
                    }
                };
                let a2316 = 0u64;
                nat_rem(a2314, a2315, a2316)
            })?;
            match m2317 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2323 = f176(v4, match v1.clone() {
                        Adt3::C0(_, _, h2318, _, _, _, _, _) => {
                            h2318
                        }
                    }, f58(v7, v2.clone(), 1u64), {
                        let a2320 = v9;
                        let a2321 = match v3 {
                            Adt6::C0(h2319, _, _, _) => {
                                h2319
                            }
                        };
                        let a2322 = 0u64;
                        nat_quot(a2320, a2321, a2322)
                    })?;
                    match m2323 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f57(v1, {
                                let a2326 = {
                                    let a2324 = f58(v7, v2.clone(), 2u64);
                                    let a2325 = v5;
                                    nat_mul(a2324, a2325)?
                                };
                                let a2327 = v9;
                                nat_add(a2326, a2327)?
                            }, v10, v11, f58(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2328, _, _, _, _, _, _, _) => {
            h2328
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2329, _, _, _, _, _, _, _) => {
            h2329
        }
    };
    let v6: u64 = f177(v4)?;
    let v7: u64 = {
        let a2330 = v0;
        let a2331 = v6;
        let a2332 = 0u64;
        nat_quot(a2330, a2331, a2332)
    };
    let v8: u64 = {
        let a2333 = v7;
        let a2334 = v5;
        let a2335 = 0u64;
        nat_quot(a2333, a2334, a2335)
    };
    let v9: u64 = {
        let a2336 = v7;
        let a2337 = v5;
        let a2338 = 0u64;
        nat_rem(a2336, a2337, a2338)
    };
    let v10: u64 = {
        let a2339 = v3;
        let a2340 = v4;
        let a2341 = 0u64;
        nat_rem(a2339, a2340, a2341)
    };
    let v11: u64 = {
        let a2346 = 4u64;
        let a2347 = {
            let a2344 = v8;
            let a2345 = {
                let a2342 = v4;
                let a2343 = v4;
                nat_mul(a2342, a2343)?
            };
            nat_mul(a2344, a2345)?
        };
        nat_mul(a2346, a2347)?
    };
    let v12: u64 = {
        let a2352 = 4u64;
        let a2353 = {
            let a2350 = v8;
            let a2351 = {
                let a2348 = v4;
                let a2349 = v5;
                nat_mul(a2348, a2349)?
            };
            nat_mul(a2350, a2351)?
        };
        nat_mul(a2352, a2353)?
    };
    let v13: u64 = {
        let a2357 = {
            let a2354 = v0;
            let a2355 = v6;
            let a2356 = 0u64;
            nat_rem(a2354, a2355, a2356)
        };
        let a2358 = f177(v10)?;
        nat_sub(a2357, a2358)
    };
    let a2361 = v13;
    let a2362 = {
        let a2359 = v10;
        let a2360 = 2u64;
        nat_mul(a2359, a2360)?
    };
    if nat_lt(a2361, a2362) {
        let v14: u64 = {
            let a2363 = v13;
            let a2364 = 2u64;
            let a2365 = 0u64;
            nat_quot(a2363, a2364, a2365)
        };
        let a2369 = {
            let a2366 = v13;
            let a2367 = 2u64;
            let a2368 = 0u64;
            nat_rem(a2366, a2367, a2368)
        };
        let a2370 = 0u64;
        if nat_eq(a2369, a2370) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2377 = v11;
                let a2378 = {
                    let a2375 = 4u64;
                    let a2376 = {
                        let a2373 = {
                            let a2371 = v10;
                            let a2372 = v4;
                            nat_mul(a2371, a2372)?
                        };
                        let a2374 = v14;
                        nat_add(a2373, a2374)?
                    };
                    nat_mul(a2375, a2376)?
                };
                nat_add(a2377, a2378)?
            }), Adt12::C3({
                let a2385 = v12;
                let a2386 = {
                    let a2383 = 4u64;
                    let a2384 = {
                        let a2381 = {
                            let a2379 = v14;
                            let a2380 = v5;
                            nat_mul(a2379, a2380)?
                        };
                        let a2382 = v9;
                        nat_add(a2381, a2382)?
                    };
                    nat_mul(a2383, a2384)?
                };
                nat_add(a2385, a2386)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2387 = v14;
                let a2388 = 0u64;
                if nat_eq(a2387, a2388) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2391 = v13;
        let a2392 = {
            let a2389 = v10;
            let a2390 = 2u64;
            nat_mul(a2389, a2390)?
        };
        if nat_eq(a2391, a2392) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2399 = v12;
                let a2400 = {
                    let a2397 = 4u64;
                    let a2398 = {
                        let a2395 = {
                            let a2393 = v10;
                            let a2394 = v5;
                            nat_mul(a2393, a2394)?
                        };
                        let a2396 = v9;
                        nat_add(a2395, a2396)?
                    };
                    nat_mul(a2397, a2398)?
                };
                nat_add(a2399, a2400)?
            }), {
                let a2401 = v10;
                let a2402 = 0u64;
                if nat_eq(a2401, a2402) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2407 = v13;
            let a2408 = {
                let a2405 = {
                    let a2403 = v10;
                    let a2404 = 2u64;
                    nat_mul(a2403, a2404)?
                };
                let a2406 = 1u64;
                nat_add(a2405, a2406)?
            };
            if nat_eq(a2407, a2408) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2415 = v11;
                    let a2416 = {
                        let a2413 = 4u64;
                        let a2414 = {
                            let a2411 = {
                                let a2409 = v10;
                                let a2410 = v4;
                                nat_mul(a2409, a2410)?
                            };
                            let a2412 = v10;
                            nat_add(a2411, a2412)?
                        };
                        nat_mul(a2413, a2414)?
                    };
                    nat_add(a2415, a2416)?
                }), Adt13::C1({
                    let a2423 = v12;
                    let a2424 = {
                        let a2421 = 4u64;
                        let a2422 = {
                            let a2419 = {
                                let a2417 = v10;
                                let a2418 = v5;
                                nat_mul(a2417, a2418)?
                            };
                            let a2420 = v9;
                            nat_add(a2419, a2420)?
                        };
                        nat_mul(a2421, a2422)?
                    };
                    nat_add(a2423, a2424)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2427 = {
                        let a2425 = v10;
                        let a2426 = 1u64;
                        nat_add(a2425, a2426)?
                    };
                    let a2428 = v4;
                    if nat_eq(a2427, a2428) {
                        0u64
                    } else {
                        let a2429 = v10;
                        let a2430 = 1u64;
                        nat_add(a2429, a2430)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f76(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2431 = v0;
    let a2432 = v1;
    if nat_lt(a2431, a2432) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f93(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2433 = v0;
            let a2434 = v1;
            nat_sub(a2433, a2434)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f93(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2441 = {
            let a2439 = 3u64;
            let a2440 = {
                let a2437 = match v4.clone() {
                    Adt6::C0(h2435, _, _, _) => {
                        h2435
                    }
                };
                let a2438 = match v5.clone() {
                    Adt6::C0(h2436, _, _, _) => {
                        h2436
                    }
                };
                nat_mul(a2437, a2438)?
            };
            nat_mul(a2439, a2440)?
        };
        let a2442 = 2u64;
        nat_add(a2441, a2442)?
    };
    let v8: u64 = {
        let a2443 = v0;
        let a2444 = v7;
        let a2445 = 0u64;
        nat_quot(a2443, a2444, a2445)
    };
    let v9: u64 = {
        let a2446 = v0;
        let a2447 = v7;
        let a2448 = 0u64;
        nat_rem(a2446, a2447, a2448)
    };
    let a2449 = v9;
    let a2450 = 0u64;
    if nat_eq(a2449, a2450) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2453 = {
            let a2451 = v9;
            let a2452 = 1u64;
            nat_add(a2451, a2452)?
        };
        let a2454 = v7;
        if nat_eq(a2453, a2454) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f93(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2455 = v9;
                let a2456 = 1u64;
                nat_sub(a2455, a2456)
            };
            let v11: u64 = {
                let a2457 = v10;
                let a2458 = 3u64;
                let a2459 = 0u64;
                nat_quot(a2457, a2458, a2459)
            };
            let v12: u64 = {
                let a2461 = v11;
                let a2462 = match v4.clone() {
                    Adt6::C0(h2460, _, _, _) => {
                        h2460
                    }
                };
                let a2463 = 0u64;
                nat_rem(a2461, a2462, a2463)
            };
            let v13: u64 = {
                let a2465 = v11;
                let a2466 = match v4.clone() {
                    Adt6::C0(h2464, _, _, _) => {
                        h2464
                    }
                };
                let a2467 = 0u64;
                nat_quot(a2465, a2466, a2467)
            };
            let v14: u64 = {
                let a2468 = v10;
                let a2469 = 3u64;
                let a2470 = 0u64;
                nat_rem(a2468, a2469, a2470)
            };
            let v15: u64 = f58(v8, v3.clone(), 2u64);
            let m2472 = f60(v4, match v2.clone() {
                Adt3::C0(h2471, _, _, _, _, _, _, _) => {
                    h2471
                }
            }, f58(v8, v3.clone(), 0u64), v12)?;
            match m2472 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2474 = f60(v5, match v2.clone() {
                        Adt3::C0(_, h2473, _, _, _, _, _, _) => {
                            h2473
                        }
                    }, f58(v8, v3.clone(), 1u64), v13)?;
                    match m2474 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2475 = v14;
                            let a2476 = 0u64;
                            if nat_eq(a2475, a2476) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f57(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f178(v6), 4u64))
                            } else {
                                let a2477 = v14;
                                let a2478 = 1u64;
                                if nat_eq(a2477, a2478) {
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

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f179(match v1.clone() {
        Adt3::C0(h2479, _, _, _, _, _, _, _) => {
            h2479
        }
    })?;
    let v5: u64 = {
        let a2480 = v0;
        let a2481 = v4;
        let a2482 = 0u64;
        nat_quot(a2480, a2481, a2482)
    };
    let v6: u64 = {
        let a2483 = v0;
        let a2484 = v4;
        let a2485 = 0u64;
        nat_rem(a2483, a2484, a2485)
    };
    let a2486 = v6;
    let a2487 = 0u64;
    if nat_eq(a2486, a2487) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2488 = v6;
        let a2489 = 1u64;
        if nat_eq(a2488, a2489) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2492 = {
                let a2490 = v6;
                let a2491 = 1u64;
                nat_add(a2490, a2491)?
            };
            let a2493 = v4;
            if nat_eq(a2492, a2493) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f93(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2494 = v6;
                    let a2495 = 2u64;
                    nat_sub(a2494, a2495)
                };
                let v8: u64 = {
                    let a2496 = v7;
                    let a2497 = 3u64;
                    let a2498 = 0u64;
                    nat_quot(a2496, a2497, a2498)
                };
                let v9: u64 = {
                    let a2499 = v7;
                    let a2500 = 3u64;
                    let a2501 = 0u64;
                    nat_rem(a2499, a2500, a2501)
                };
                let v10: u64 = f93({
                    let a2505 = {
                        let a2503 = v5;
                        let a2504 = match v1.clone() {
                            Adt3::C0(h2502, _, _, _, _, _, _, _) => {
                                h2502
                            }
                        };
                        nat_mul(a2503, a2504)?
                    };
                    let a2506 = v8;
                    nat_add(a2505, a2506)?
                }, v1)?;
                let a2507 = v9;
                let a2508 = 0u64;
                if nat_eq(a2507, a2508) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2509 = v9;
                    let a2510 = 1u64;
                    if nat_eq(a2509, a2510) {
                        let a2511 = v3;
                        let a2512 = 2u64;
                        if nat_eq(a2511, a2512) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2513 = v3;
                        let a2514 = 2u64;
                        if if nat_eq(a2513, a2514) {
                            true
                        } else {
                            let a2515 = v3;
                            let a2516 = 3u64;
                            nat_eq(a2515, a2516)
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

pub fn f79(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2518 = 2u64;
        let a2519 = match v2.clone() {
            Adt3::C0(_, h2517, _, _, _, _, _, _) => {
                h2517
            }
        };
        nat_mul(a2518, a2519)?
    };
    let a2520 = v1;
    let a2521 = v6;
    if nat_lt(a2520, a2521) {
        let a2525 = {
            let a2522 = v1;
            let a2523 = 2u64;
            let a2524 = 0u64;
            nat_rem(a2522, a2523, a2524)
        };
        let a2526 = 0u64;
        if nat_eq(a2525, a2526) {
            Ok(Adt9::C4(19u64, 4u64, f93({
                let a2527 = v1;
                let a2528 = 2u64;
                let a2529 = 0u64;
                nat_quot(a2527, a2528, a2529)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2530 = v5;
            let a2531 = 2147483648u64;
            if if nat_lt(a2530, a2531) {
                let a2533 = v5;
                let a2534 = match v3 {
                    Adt3::C0(_, h2532, _, _, _, _, _, _) => {
                        h2532
                    }
                };
                nat_lt(a2533, a2534)
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
            let a2537 = match v3.clone() {
                Adt3::C0(h2535, _, _, _, _, _, _, _) => {
                    h2535
                }
            };
            let a2538 = match v3.clone() {
                Adt3::C0(_, h2536, _, _, _, _, _, _) => {
                    h2536
                }
            };
            nat_mul(a2537, a2538)?
        };
        let a2541 = {
            let a2539 = v1;
            let a2540 = v6;
            nat_sub(a2539, a2540)
        };
        let a2542 = v7;
        if nat_lt(a2541, a2542) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f93({
                let a2543 = v1;
                let a2544 = v6;
                nat_sub(a2543, a2544)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2547 = {
                    let a2545 = v1;
                    let a2546 = v6;
                    nat_sub(a2545, a2546)
                };
                let a2548 = v7;
                nat_sub(a2547, a2548)
            };
            let v9: u64 = {
                let a2550 = 1u64;
                let a2551 = f180(v0, match v2.clone() {
                    Adt3::C0(h2549, _, _, _, _, _, _, _) => {
                        h2549
                    }
                })?;
                nat_add(a2550, a2551)?
            };
            let v10: u64 = {
                let a2552 = v8;
                let a2553 = v9;
                let a2554 = 0u64;
                nat_quot(a2552, a2553, a2554)
            };
            let v11: u64 = {
                let a2555 = v8;
                let a2556 = v9;
                let a2557 = 0u64;
                nat_rem(a2555, a2556, a2557)
            };
            let a2558 = v11;
            let a2559 = 0u64;
            if nat_eq(a2558, a2559) {
                Ok(Adt9::C4(19u64, 4u64, f93(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f181(v0) {
                    let a2563 = {
                        let a2560 = v11;
                        let a2561 = 2u64;
                        let a2562 = 0u64;
                        nat_rem(a2560, a2561, a2562)
                    };
                    let a2564 = 1u64;
                    if nat_eq(a2563, a2564) {
                        Ok(Adt9::C4(6u64, 0u64, f93({
                            let a2573 = {
                                let a2566 = v10;
                                let a2567 = match v2.clone() {
                                    Adt3::C0(h2565, _, _, _, _, _, _, _) => {
                                        h2565
                                    }
                                };
                                nat_mul(a2566, a2567)?
                            };
                            let a2574 = {
                                let a2570 = {
                                    let a2568 = v11;
                                    let a2569 = 1u64;
                                    nat_sub(a2568, a2569)
                                };
                                let a2571 = 2u64;
                                let a2572 = 0u64;
                                nat_quot(a2570, a2571, a2572)
                            };
                            nat_add(a2573, a2574)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f93({
                            let a2583 = {
                                let a2576 = v5;
                                let a2577 = match v2.clone() {
                                    Adt3::C0(h2575, _, _, _, _, _, _, _) => {
                                        h2575
                                    }
                                };
                                nat_mul(a2576, a2577)?
                            };
                            let a2584 = {
                                let a2580 = {
                                    let a2578 = v11;
                                    let a2579 = 1u64;
                                    nat_sub(a2578, a2579)
                                };
                                let a2581 = 2u64;
                                let a2582 = 0u64;
                                nat_quot(a2580, a2581, a2582)
                            };
                            nat_add(a2583, a2584)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f93({
                            let a2593 = {
                                let a2586 = v5;
                                let a2587 = match v2 {
                                    Adt3::C0(h2585, _, _, _, _, _, _, _) => {
                                        h2585
                                    }
                                };
                                nat_mul(a2586, a2587)?
                            };
                            let a2594 = {
                                let a2590 = {
                                    let a2588 = v11;
                                    let a2589 = 1u64;
                                    nat_sub(a2588, a2589)
                                };
                                let a2591 = 2u64;
                                let a2592 = 0u64;
                                nat_quot(a2590, a2591, a2592)
                            };
                            nat_add(a2593, a2594)?
                        }, v3)?))
                    }
                } else {
                    f182(v0, v10, {
                        let a2595 = v11;
                        let a2596 = 1u64;
                        nat_sub(a2595, a2596)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2597 = v1;
    let a2598 = f183(v2.clone(), v4.clone())?;
    if nat_lt(a2597, a2598) {
        let a2601 = v1;
        let a2602 = {
            let a2599 = 2u64;
            let a2600 = f184(v4.clone())?;
            nat_mul(a2599, a2600)?
        };
        if nat_lt(a2601, a2602) {
            let a2606 = {
                let a2603 = v1;
                let a2604 = 2u64;
                let a2605 = 0u64;
                nat_rem(a2603, a2604, a2605)
            };
            let a2607 = 0u64;
            if nat_eq(a2606, a2607) {
                f185({
                    let a2608 = v1;
                    let a2609 = 2u64;
                    let a2610 = 0u64;
                    nat_quot(a2608, a2609, a2610)
                }, v4)
            } else {
                if f186(match v2 {
                    Adt3::C0(_, h2611, _, _, _, _, _, _) => {
                        h2611
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2614 = v1;
                let a2615 = {
                    let a2612 = 2u64;
                    let a2613 = f184(v4.clone())?;
                    nat_mul(a2612, a2613)?
                };
                nat_sub(a2614, a2615)
            };
            let v7: u64 = {
                let a2619 = v6;
                let a2620 = {
                    let a2617 = 1u64;
                    let a2618 = match v2.clone() {
                        Adt3::C0(h2616, _, _, _, _, _, _, _) => {
                            h2616
                        }
                    };
                    nat_add(a2617, a2618)?
                };
                let a2621 = 0u64;
                nat_quot(a2619, a2620, a2621)
            };
            let v8: u64 = {
                let a2625 = v6;
                let a2626 = {
                    let a2623 = 1u64;
                    let a2624 = match v2.clone() {
                        Adt3::C0(h2622, _, _, _, _, _, _, _) => {
                            h2622
                        }
                    };
                    nat_add(a2623, a2624)?
                };
                let a2627 = 0u64;
                nat_rem(a2625, a2626, a2627)
            };
            let a2628 = v8;
            let a2629 = 0u64;
            if nat_eq(a2628, a2629) {
                f185(v7, v4)
            } else {
                f187(v0, v7, {
                    let a2630 = v8;
                    let a2631 = 1u64;
                    nat_sub(a2630, a2631)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2634 = match v4.clone() {
            Adt3::C0(h2632, _, _, _, _, _, _, _) => {
                h2632
            }
        };
        let a2635 = match v4.clone() {
            Adt3::C0(_, h2633, _, _, _, _, _, _) => {
                h2633
            }
        };
        nat_mul(a2634, a2635)?
    };
    let v7: u64 = {
        let a2636 = 2u64;
        let a2637 = v6;
        nat_mul(a2636, a2637)?
    };
    let v8: u64 = f188(match v3.clone() {
        Adt3::C0(_, h2638, _, _, _, _, _, _) => {
            h2638
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2639, _, _, _, _, _) => {
            h2639
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2640, _, _, _, _) => {
            h2640
        }
    });
    let a2641 = v0;
    let a2642 = v7;
    if nat_lt(a2641, a2642) {
        let a2646 = {
            let a2643 = v0;
            let a2644 = 2u64;
            let a2645 = 0u64;
            nat_rem(a2643, a2644, a2645)
        };
        let a2647 = 0u64;
        if nat_eq(a2646, a2647) {
            Ok(Adt9::C4(19u64, 5u64, f93({
                let a2648 = v0;
                let a2649 = 2u64;
                let a2650 = 0u64;
                nat_quot(a2648, a2649, a2650)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f189(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2651 = v0;
            let a2652 = v7;
            nat_sub(a2651, a2652)
        };
        let v10: u64 = {
            let a2656 = v9;
            let a2657 = {
                let a2654 = 1u64;
                let a2655 = match v1.clone() {
                    Adt3::C0(h2653, _, _, _, _, _, _, _) => {
                        h2653
                    }
                };
                nat_add(a2654, a2655)?
            };
            let a2658 = 0u64;
            nat_quot(a2656, a2657, a2658)
        };
        let v11: u64 = {
            let a2662 = v9;
            let a2663 = {
                let a2660 = 1u64;
                let a2661 = match v1.clone() {
                    Adt3::C0(h2659, _, _, _, _, _, _, _) => {
                        h2659
                    }
                };
                nat_add(a2660, a2661)?
            };
            let a2664 = 0u64;
            nat_rem(a2662, a2663, a2664)
        };
        let a2665 = v11;
        let a2666 = 0u64;
        if nat_eq(a2665, a2666) {
            Ok(Adt9::C4(19u64, 5u64, f93({
                let a2667 = v10;
                let a2668 = v6;
                let a2669 = 0u64;
                nat_rem(a2667, a2668, a2669)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f189(v5, v8) {
                let v12: u64 = {
                    let a2675 = {
                        let a2671 = v10;
                        let a2672 = match v1.clone() {
                            Adt3::C0(h2670, _, _, _, _, _, _, _) => {
                                h2670
                            }
                        };
                        nat_mul(a2671, a2672)?
                    };
                    let a2676 = {
                        let a2673 = v11;
                        let a2674 = 1u64;
                        nat_sub(a2673, a2674)
                    };
                    nat_add(a2675, a2676)?
                };
                let a2677 = v5;
                let a2678 = 4294967295u64;
                if nat_eq(a2677, a2678) {
                    Ok(Adt9::C1(false, f93(v12, v1)?, f93(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f93(v12, v1)?, 4u64, f93({
                        let a2684 = {
                            let a2680 = v5;
                            let a2681 = match v3.clone() {
                                Adt3::C0(h2679, _, _, _, _, _, _, _) => {
                                    h2679
                                }
                            };
                            nat_mul(a2680, a2681)?
                        };
                        let a2685 = {
                            let a2682 = v11;
                            let a2683 = 1u64;
                            nat_sub(a2682, a2683)
                        };
                        nat_add(a2684, a2685)?
                    }, v3)?, 3u64, f93(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f190(v3.clone())?;
    let v6: u64 = {
        let a2686 = v0;
        let a2687 = v5;
        let a2688 = 0u64;
        nat_quot(a2686, a2687, a2688)
    };
    let v7: u64 = {
        let a2689 = v0;
        let a2690 = v5;
        let a2691 = 0u64;
        nat_rem(a2689, a2690, a2691)
    };
    let v8: u64 = f191(v3.clone())?;
    let a2692 = v7;
    let a2693 = 0u64;
    if nat_eq(a2692, a2693) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2694, _) => {
                h2694
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2698 = v5;
        let a2699 = {
            let a2696 = v7;
            let a2697 = if match v3.clone() {
                Adt7::C0(_, _, _, h2695, _) => {
                    h2695
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2696, a2697)?
        };
        if nat_le(a2698, a2699) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2700, _) => {
                    h2700
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f93(v6, v2)?))
            } else {
                let a2703 = {
                    let a2701 = v7;
                    let a2702 = 2u64;
                    nat_add(a2701, a2702)?
                };
                let a2704 = v5;
                if nat_eq(a2703, a2704) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2705, _, _) => {
                            h2705
                        }
                    } {
                        let a2712 = {
                            let m2708 = match v3.clone() {
                                Adt7::C0(h2707, _, _, _, _) => {
                                    h2707
                                }
                            };
                            match m2708 {
                                Adt6::C0(h2706, _, _, _) => {
                                    h2706
                                }
                            }
                        };
                        let a2713 = {
                            let m2711 = match v3 {
                                Adt7::C0(_, h2710, _, _, _) => {
                                    h2710
                                }
                            };
                            match m2711 {
                                Adt6::C0(h2709, _, _, _) => {
                                    h2709
                                }
                            }
                        };
                        nat_mul(a2712, a2713)?
                    } else {
                        f192(match v3 {
                            Adt7::C0(h2714, _, _, _, _) => {
                                h2714
                            }
                        }, match v1 {
                            Adt3::C0(h2715, _, _, _, _, _, _, _) => {
                                h2715
                            }
                        }, f58(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2716 = v9;
                        let a2717 = 0u64;
                        if nat_eq(a2716, a2717) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f93(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2720 = {
                    let a2718 = v7;
                    let a2719 = 1u64;
                    nat_sub(a2718, a2719)
                };
                let a2721 = v8;
                let a2722 = 0u64;
                nat_rem(a2720, a2721, a2722)
            };
            let m2728 = f175(v3.clone(), v1, v2, v6, {
                let a2725 = {
                    let a2723 = v7;
                    let a2724 = 1u64;
                    nat_sub(a2723, a2724)
                };
                let a2726 = v8;
                let a2727 = 0u64;
                nat_quot(a2725, a2726, a2727)
            })?;
            match m2728 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2730 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2729) => {
                            h2729
                        }
                    };
                    let a2731 = 1u64;
                    if if nat_eq(a2730, a2731) {
                        let a2732 = v10;
                        let a2733 = 0u64;
                        nat_eq(a2732, a2733)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2734, _) => {
                                h2734
                            }
                        } {
                            let a2737 = {
                                let a2735 = v10;
                                let a2736 = 1u64;
                                nat_add(a2735, a2736)?
                            };
                            let a2738 = v8;
                            if nat_eq(a2737, a2738) {
                                let a2739 = v4;
                                let a2740 = 2u64;
                                if nat_eq(a2739, a2740) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2742 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2741) => {
                                                h2741
                                            }
                                        };
                                        let a2743 = 1u64;
                                        if nat_eq(a2742, a2743) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2745 = match v3 {
                                            Adt7::C0(_, _, _, _, h2744) => {
                                                h2744
                                            }
                                        };
                                        let a2746 = 1u64;
                                        if nat_eq(a2745, a2746) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2748 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2747) => {
                                            h2747
                                        }
                                    };
                                    let a2749 = 1u64;
                                    if nat_eq(a2748, a2749) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2751 = match v3 {
                                        Adt7::C0(_, _, _, _, h2750) => {
                                            h2750
                                        }
                                    };
                                    let a2752 = 1u64;
                                    if nat_eq(a2751, a2752) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2754 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2753) => {
                                        h2753
                                    }
                                };
                                let a2755 = 1u64;
                                if nat_eq(a2754, a2755) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2757 = match v3 {
                                    Adt7::C0(_, _, _, _, h2756) => {
                                        h2756
                                    }
                                };
                                let a2758 = 1u64;
                                if nat_eq(a2757, a2758) {
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

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f193(match v3.clone() {
        Adt3::C0(h2759, _, _, _, _, _, _, _) => {
            h2759
        }
    })?;
    let v5: u64 = {
        let a2760 = v0;
        let a2761 = v4;
        let a2762 = 0u64;
        nat_rem(a2760, a2761, a2762)
    };
    let v6: u64 = {
        let a2763 = v0;
        let a2764 = v4;
        let a2765 = 0u64;
        nat_quot(a2763, a2764, a2765)
    };
    let a2766 = v5;
    let a2767 = 0u64;
    if nat_eq(a2766, a2767) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2770 = {
            let a2768 = v5;
            let a2769 = 1u64;
            nat_add(a2768, a2769)?
        };
        let a2771 = v4;
        if nat_eq(a2770, a2771) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f93(v6, v2)?))
        } else {
            let a2775 = {
                let a2772 = v5;
                let a2773 = 2u64;
                let a2774 = 0u64;
                nat_rem(a2772, a2773, a2774)
            };
            let a2776 = 0u64;
            if nat_eq(a2775, a2776) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2779 = {
                        let a2777 = v5;
                        let a2778 = 1u64;
                        nat_sub(a2777, a2778)
                    };
                    let a2780 = 2u64;
                    let a2781 = 0u64;
                    nat_quot(a2779, a2780, a2781)
                };
                let v8: u64 = f58(v6, v2.clone(), 0u64);
                let v9: u64 = f58(v6, v2.clone(), 1u64);
                let v10: u64 = f58(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f57(v1, {
                    let a2782 = v9;
                    let a2783 = v7;
                    nat_add(a2782, a2783)?
                }, v8, v10, 0u64)?, 4u64, f57(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f194(v3.clone())?;
    let v5: u64 = {
        let a2784 = v0;
        let a2785 = v4;
        let a2786 = 0u64;
        nat_rem(a2784, a2785, a2786)
    };
    let v6: u64 = {
        let a2787 = v0;
        let a2788 = v4;
        let a2789 = 0u64;
        nat_quot(a2787, a2788, a2789)
    };
    let a2790 = v5;
    let a2791 = 0u64;
    if nat_eq(a2790, a2791) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2794 = {
            let a2792 = v5;
            let a2793 = 1u64;
            nat_add(a2792, a2793)?
        };
        let a2795 = v4;
        if nat_eq(a2794, a2795) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f93(v6, v2)?))
        } else {
            let a2799 = {
                let a2796 = v5;
                let a2797 = 2u64;
                let a2798 = 0u64;
                nat_rem(a2796, a2797, a2798)
            };
            let a2800 = 0u64;
            if nat_eq(a2799, a2800) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2802 = f58(v6, v2.clone(), 0u64);
                    let a2803 = match v3.clone() {
                        Adt6::C0(_, _, h2801, _) => {
                            h2801
                        }
                    };
                    nat_add(a2802, a2803)?
                };
                let v8: u64 = {
                    let a2811 = {
                        let a2808 = {
                            let a2805 = v7;
                            let a2806 = match v3.clone() {
                                Adt6::C0(_, h2804, _, _) => {
                                    h2804
                                }
                            };
                            nat_add(a2805, a2806)?
                        };
                        let a2809 = match v3.clone() {
                            Adt6::C0(h2807, _, _, _) => {
                                h2807
                            }
                        };
                        nat_sub(a2808, a2809)
                    };
                    let a2812 = match v3.clone() {
                        Adt6::C0(_, h2810, _, _) => {
                            h2810
                        }
                    };
                    let a2813 = 0u64;
                    nat_quot(a2811, a2812, a2813)
                };
                let v9: u64 = {
                    let a2819 = v8;
                    let a2820 = {
                        let a2816 = {
                            let a2814 = v5;
                            let a2815 = 1u64;
                            nat_sub(a2814, a2815)
                        };
                        let a2817 = 2u64;
                        let a2818 = 0u64;
                        nat_quot(a2816, a2817, a2818)
                    };
                    nat_add(a2819, a2820)?
                };
                let v10: u64 = {
                    let a2822 = v9;
                    let a2823 = match v3.clone() {
                        Adt6::C0(_, h2821, _, _) => {
                            h2821
                        }
                    };
                    nat_mul(a2822, a2823)?
                };
                let v11: u64 = {
                    let a2824 = v7;
                    let a2825 = v10;
                    nat_sub(a2824, a2825)
                };
                let a2827 = v9;
                let a2828 = match v1.clone() {
                    Adt3::C0(_, h2826, _, _, _, _, _, _) => {
                        h2826
                    }
                };
                if if if nat_lt(a2827, a2828) {
                    let a2829 = v10;
                    let a2830 = v7;
                    nat_le(a2829, a2830)
                } else {
                    false
                } {
                    let a2832 = v11;
                    let a2833 = match v3.clone() {
                        Adt6::C0(h2831, _, _, _) => {
                            h2831
                        }
                    };
                    nat_lt(a2832, a2833)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f57(v1, {
                        let a2837 = {
                            let a2835 = f58(v6, v2, 1u64);
                            let a2836 = match v3 {
                                Adt6::C0(h2834, _, _, _) => {
                                    h2834
                                }
                            };
                            nat_mul(a2835, a2836)?
                        };
                        let a2838 = v11;
                        nat_add(a2837, a2838)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2840 = v5;
        let a2841 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2839, _) => {
                h2839
            }
        };
        let a2842 = 0u64;
        nat_rem(a2840, a2841, a2842)
    };
    let v7: u64 = {
        let a2844 = v5;
        let a2845 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2843, _) => {
                h2843
            }
        };
        let a2846 = 0u64;
        nat_quot(a2844, a2845, a2846)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2847, _, _, _, _, _, _, _) => {
            h2847
        }
    }, match v1.clone() {
        Adt3::C0(h2848, _, _, _, _, _, _, _) => {
            h2848
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a2852 = v4;
        let a2853 = {
            let m2851 = match v3.clone() {
                Adt5::C0(h2850, _, _, _, _, _, _, _) => {
                    h2850
                }
            };
            match m2851 {
                Adt6::C0(h2849, _, _, _) => {
                    h2849
                }
            }
        };
        let a2854 = 0u64;
        nat_rem(a2852, a2853, a2854)
    })?;
    let v9: u64 = {
        let a2858 = v4;
        let a2859 = {
            let m2857 = match v3.clone() {
                Adt5::C0(h2856, _, _, _, _, _, _, _) => {
                    h2856
                }
            };
            match m2857 {
                Adt6::C0(h2855, _, _, _) => {
                    h2855
                }
            }
        };
        let a2860 = 0u64;
        nat_quot(a2858, a2859, a2860)
    };
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2861, _, _, _, _, _, _) => {
            h2861
        }
    }, match v1.clone() {
        Adt3::C0(_, h2862, _, _, _, _, _, _) => {
            h2862
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a2866 = v9;
        let a2867 = {
            let m2865 = match v3.clone() {
                Adt5::C0(_, h2864, _, _, _, _, _, _) => {
                    h2864
                }
            };
            match m2865 {
                Adt6::C0(h2863, _, _, _) => {
                    h2863
                }
            }
        };
        let a2868 = 0u64;
        nat_rem(a2866, a2867, a2868)
    })?;
    let v11: u64 = {
        let a2872 = v9;
        let a2873 = {
            let m2871 = match v3.clone() {
                Adt5::C0(_, h2870, _, _, _, _, _, _) => {
                    h2870
                }
            };
            match m2871 {
                Adt6::C0(h2869, _, _, _) => {
                    h2869
                }
            }
        };
        let a2874 = 0u64;
        nat_quot(a2872, a2873, a2874)
    };
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2875, _, _, _, _, _) => {
            h2875
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2876, _, _, _, _, _) => {
            h2876
        }
    }, v6, {
        let a2880 = v11;
        let a2881 = {
            let m2879 = match v3.clone() {
                Adt5::C0(_, _, h2878, _, _, _, _, _) => {
                    h2878
                }
            };
            match m2879 {
                Adt6::C0(h2877, _, _, _) => {
                    h2877
                }
            }
        };
        let a2882 = 0u64;
        nat_rem(a2880, a2881, a2882)
    })?;
    let v13: u64 = {
        let a2886 = v11;
        let a2887 = {
            let m2885 = match v3.clone() {
                Adt5::C0(_, _, h2884, _, _, _, _, _) => {
                    h2884
                }
            };
            match m2885 {
                Adt6::C0(h2883, _, _, _) => {
                    h2883
                }
            }
        };
        let a2888 = 0u64;
        nat_quot(a2886, a2887, a2888)
    };
    let v17: Adt12 = {
        let m2889 = v8;
        match m2889 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2890 = v10;
                match m2890 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2891 = v12;
                        match m2891 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2900 = {
                                        let a2895 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a2896 = {
                                            let a2893 = v7;
                                            let a2894 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2892, _, _, _) => {
                                                    h2892
                                                }
                                            };
                                            nat_mul(a2893, a2894)?
                                        };
                                        nat_add(a2895, a2896)?
                                    };
                                    let a2901 = {
                                        let a2898 = v13;
                                        let a2899 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2897, _, _, _, _) => {
                                                h2897
                                            }
                                        };
                                        nat_mul(a2898, a2899)?
                                    };
                                    nat_add(a2900, a2901)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2902, _, _) => {
            h2902
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2903) => {
            h2903
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f93(v0, v2)?), {
        let a2905 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2904, _, _) => {
                h2904
            }
        };
        let a2906 = 0u64;
        if nat_eq(a2905, a2906) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2908 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2907) => {
                h2907
            }
        };
        let a2909 = 0u64;
        if nat_eq(a2908, a2909) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f86(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2922 = f195(v0, v1, match v2.clone() {
        Adt3::C0(h2910, _, _, _, _, _, _, _) => {
            h2910
        }
    }, match v2.clone() {
        Adt3::C0(_, h2911, _, _, _, _, _, _) => {
            h2911
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2912, _, _, _, _, _) => {
            h2912
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2913, _, _, _, _) => {
            h2913
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2914, _, _, _) => {
            h2914
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2915, _, _) => {
            h2915
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2916, _) => {
            h2916
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2917) => {
            h2917
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2918, _, _, _) => {
            h2918
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2919, _, _) => {
            h2919
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2920, _) => {
            h2920
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2921) => {
            h2921
        }
    }, v4)?;
    match m2922 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f20(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f87(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2925 = v6;
        let a2926 = {
            let a2923 = 3u64;
            let a2924 = v5;
            nat_add(a2923, a2924)?
        };
        nat_sub(a2925, a2926)
    };
    let a2928 = v0;
    let a2929 = f50(v3, match v1.clone() {
        Adt3::C0(h2927, _, _, _, _, _, _, _) => {
            h2927
        }
    }, v5, v8)?;
    if nat_lt(a2928, a2929) {
        let v9: u64 = f102(match v1.clone() {
            Adt3::C0(h2930, _, _, _, _, _, _, _) => {
                h2930
            }
        }, v5, v8)?;
        let v10: u64 = f103(v3, match v1.clone() {
            Adt3::C0(h2931, _, _, _, _, _, _, _) => {
                h2931
            }
        }, v5)?;
        let a2932 = v0;
        let a2933 = v9;
        if nat_lt(a2932, a2933) {
            f196(v0, v2, f44(v3), {
                let a2934 = 3u64;
                let a2935 = v5;
                nat_add(a2934, a2935)?
            }, v7)
        } else {
            let a2938 = v0;
            let a2939 = {
                let a2936 = v9;
                let a2937 = v10;
                nat_add(a2936, a2937)?
            };
            if nat_lt(a2938, a2939) {
                let v11: u64 = {
                    let a2940 = v0;
                    let a2941 = v9;
                    nat_sub(a2940, a2941)
                };
                let v12: u64 = f197(v3, match v1.clone() {
                    Adt3::C0(h2942, _, _, _, _, _, _, _) => {
                        h2942
                    }
                })?;
                f198({
                    let a2943 = v11;
                    let a2944 = v12;
                    let a2945 = 0u64;
                    nat_rem(a2943, a2944, a2945)
                }, {
                    let a2946 = v11;
                    let a2947 = v12;
                    let a2948 = 0u64;
                    nat_quot(a2946, a2947, a2948)
                }, v1, v3, v4, v7)
            } else {
                f199({
                    let a2951 = v0;
                    let a2952 = {
                        let a2949 = v9;
                        let a2950 = v10;
                        nat_add(a2949, a2950)?
                    };
                    nat_sub(a2951, a2952)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2953 = v0;
    let a2954 = f36(v1)?;
    if nat_lt(a2953, a2954) {
        let a2955 = v0;
        let a2956 = 4u64;
        if nat_lt(a2955, a2956) {
            let a2957 = v0;
            let a2958 = 0u64;
            if nat_eq(a2957, a2958) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2959 = v0;
                let a2960 = 1u64;
                if nat_eq(a2959, a2960) {
                    let a2961 = v5;
                    let a2962 = 0u64;
                    if nat_eq(a2961, a2962) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2963 = v0;
                    let a2964 = 2u64;
                    if nat_eq(a2963, a2964) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2965 = v0;
            let a2966 = 4u64;
            if if nat_eq(a2965, a2966) {
                let a2967 = f200(v5)?;
                let a2968 = v1;
                !nat_eq(a2967, a2968)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2971 = {
                        let a2969 = v0;
                        let a2970 = 4u64;
                        nat_sub(a2969, a2970)
                    };
                    let a2972 = 3u64;
                    let a2973 = 0u64;
                    nat_quot(a2971, a2972, a2973)
                };
                let v7: u64 = {
                    let a2976 = {
                        let a2974 = v0;
                        let a2975 = 4u64;
                        nat_sub(a2974, a2975)
                    };
                    let a2977 = 3u64;
                    let a2978 = 0u64;
                    nat_rem(a2976, a2977, a2978)
                };
                let a2979 = v7;
                let a2980 = 0u64;
                if nat_eq(a2979, a2980) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2981 = v7;
                    let a2982 = 1u64;
                    if nat_eq(a2981, a2982) {
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

pub fn f89(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2983 = v1;
        let a2984 = 5u64;
        nat_eq(a2983, a2984)
    } else {
        false
    } {
        let a2985 = v2;
        let a2986 = 16u64;
        nat_eq(a2985, a2986)
    } else {
        false
    } {
        let a2987 = v3;
        let a2988 = 0u64;
        nat_eq(a2987, a2988)
    } else {
        false
    } {
        let a2989 = v4;
        let a2990 = 0u64;
        nat_eq(a2989, a2990)
    } else {
        false
    }
}

pub fn f90(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2991 = v1;
    let a2992 = 0u64;
    if nat_eq(a2991, a2992) {
        if v0 {
            let a2993 = v3;
            let a2994 = 0u64;
            if if if nat_eq(a2993, a2994) {
                let a2995 = v4;
                let a2996 = 43u64;
                nat_eq(a2995, a2996)
            } else {
                false
            } {
                true
            } else {
                let a2997 = v3;
                let a2998 = 43u64;
                if nat_eq(a2997, a2998) {
                    let a2999 = v4;
                    let a3000 = 0u64;
                    nat_eq(a2999, a3000)
                } else {
                    false
                }
            } {
                f169(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3001 = v1;
        let a3002 = 1u64;
        if nat_eq(a3001, a3002) {
            Adt11::C1(v2)
        } else {
            let a3003 = v1;
            let a3004 = 2u64;
            if nat_eq(a3003, a3004) {
                Adt11::C5(v2)
            } else {
                let a3005 = v1;
                let a3006 = 3u64;
                if nat_eq(a3005, a3006) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3007 = v1;
                    let a3008 = 4u64;
                    if nat_eq(a3007, a3008) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f89(v0, v1, v2, v3, v4) {
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

pub fn f91(v0: u64, v1: u64) -> Adt12 {
    let a3009 = v0;
    let a3010 = 0u64;
    if nat_eq(a3009, a3010) {
        Adt12::C0(0u64, v1)
    } else {
        let a3011 = v0;
        let a3012 = 1u64;
        if nat_eq(a3011, a3012) {
            Adt12::C1(v1)
        } else {
            let a3013 = v0;
            let a3014 = 2u64;
            if nat_eq(a3013, a3014) {
                Adt12::C2(v1)
            } else {
                let a3015 = v0;
                let a3016 = 3u64;
                if nat_eq(a3015, a3016) {
                    Adt12::C3(v1)
                } else {
                    let a3017 = v0;
                    let a3018 = 13u64;
                    if nat_lt(a3017, a3018) {
                        Adt12::C0({
                            let a3019 = v0;
                            let a3020 = 3u64;
                            nat_sub(a3019, a3020)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f92(v0: u64, v1: u64) -> Adt13 {
    let a3021 = v0;
    let a3022 = 1u64;
    if nat_eq(a3021, a3022) {
        Adt13::C0(v1)
    } else {
        let a3023 = v0;
        let a3024 = 3u64;
        if if nat_eq(a3023, a3024) {
            true
        } else {
            let a3025 = v0;
            let a3026 = 6u64;
            nat_eq(a3025, a3026)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f93(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3028 = v0;
        let a3029 = match v1.clone() {
            Adt3::C0(h3027, _, _, _, _, _, _, _) => {
                h3027
            }
        };
        let a3030 = 0u64;
        nat_quot(a3028, a3029, a3030)
    };
    let v3: u64 = {
        let a3032 = v2;
        let a3033 = match v1.clone() {
            Adt3::C0(_, h3031, _, _, _, _, _, _) => {
                h3031
            }
        };
        let a3034 = 0u64;
        nat_quot(a3032, a3033, a3034)
    };
    f144({
        let a3036 = v0;
        let a3037 = match v1.clone() {
            Adt3::C0(h3035, _, _, _, _, _, _, _) => {
                h3035
            }
        };
        let a3038 = 0u64;
        nat_rem(a3036, a3037, a3038)
    }, {
        let a3040 = v2;
        let a3041 = match v1.clone() {
            Adt3::C0(_, h3039, _, _, _, _, _, _) => {
                h3039
            }
        };
        let a3042 = 0u64;
        nat_rem(a3040, a3041, a3042)
    }, {
        let a3044 = v3;
        let a3045 = match v1.clone() {
            Adt3::C0(_, _, h3043, _, _, _, _, _) => {
                h3043
            }
        };
        let a3046 = 0u64;
        nat_rem(a3044, a3045, a3046)
    }, {
        let a3048 = v3;
        let a3049 = match v1.clone() {
            Adt3::C0(_, _, h3047, _, _, _, _, _) => {
                h3047
            }
        };
        let a3050 = 0u64;
        nat_quot(a3048, a3049, a3050)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3051, _, _, _) => {
            h3051
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3052, _, _) => {
            h3052
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3053, _) => {
            h3053
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3054) => {
            h3054
        }
    })
}

pub fn f94(v0: Adt6, v1: u64) -> R<u64> {
    let a3056 = 0u64;
    let a3057 = match v0.clone() {
        Adt6::C0(h3055, _, _, _) => {
            h3055
        }
    };
    if if if if if nat_lt(a3056, a3057) {
        let a3059 = 0u64;
        let a3060 = match v0.clone() {
            Adt6::C0(_, h3058, _, _) => {
                h3058
            }
        };
        nat_lt(a3059, a3060)
    } else {
        false
    } {
        let a3062 = 0u64;
        let a3063 = match v0.clone() {
            Adt6::C0(_, _, _, h3061) => {
                h3061
            }
        };
        nat_lt(a3062, a3063)
    } else {
        false
    } {
        let a3065 = match v0.clone() {
            Adt6::C0(_, h3064, _, _) => {
                h3064
            }
        };
        let a3066 = 2147483648u64;
        nat_lt(a3065, a3066)
    } else {
        false
    } {
        let a3068 = match v0.clone() {
            Adt6::C0(_, _, _, h3067) => {
                h3067
            }
        };
        let a3069 = 2147483648u64;
        nat_lt(a3068, a3069)
    } else {
        false
    } {
        let v2: u64 = {
            let a3073 = v1;
            let a3074 = {
                let a3071 = 2u64;
                let a3072 = f146(match v0.clone() {
                    Adt6::C0(_, _, h3070, _) => {
                        h3070
                    }
                });
                nat_mul(a3071, a3072)?
            };
            nat_add(a3073, a3074)?
        };
        let v3: u64 = {
            let a3086 = {
                let a3081 = 1u64;
                let a3082 = {
                    let a3079 = match v0.clone() {
                        Adt6::C0(_, _, _, h3075) => {
                            h3075
                        }
                    };
                    let a3080 = {
                        let a3077 = match v0.clone() {
                            Adt6::C0(h3076, _, _, _) => {
                                h3076
                            }
                        };
                        let a3078 = 1u64;
                        nat_sub(a3077, a3078)
                    };
                    nat_mul(a3079, a3080)?
                };
                nat_add(a3081, a3082)?
            };
            let a3087 = {
                let a3084 = 2u64;
                let a3085 = f145(match v0.clone() {
                    Adt6::C0(_, _, h3083, _) => {
                        h3083
                    }
                });
                nat_mul(a3084, a3085)?
            };
            nat_add(a3086, a3087)?
        };
        let a3088 = v3;
        let a3089 = v2;
        if nat_le(a3088, a3089) {
            let a3096 = 1u64;
            let a3097 = {
                let a3093 = {
                    let a3090 = v2;
                    let a3091 = v3;
                    nat_sub(a3090, a3091)
                };
                let a3094 = match v0 {
                    Adt6::C0(_, h3092, _, _) => {
                        h3092
                    }
                };
                let a3095 = 0u64;
                nat_quot(a3093, a3094, a3095)
            };
            nat_add(a3096, a3097)
        } else {
            let a3101 = {
                let a3098 = v3;
                let a3099 = v2;
                nat_sub(a3098, a3099)
            };
            let a3102 = match v0 {
                Adt6::C0(_, h3100, _, _) => {
                    h3100
                }
            };
            if nat_lt(a3101, a3102) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f95(v0: u64) -> u64 {
    let v1: u64 = {
        let a3103 = v0;
        let a3104 = 4294967296u64;
        let a3105 = 0u64;
        nat_rem(a3103, a3104, a3105)
    };
    let a3106 = v1;
    let a3107 = 2147483648u64;
    if nat_lt(a3106, a3107) {
        v1
    } else {
        0u64
    }
}

pub fn f96(v0: u64) -> R<u64> {
    let a3110 = 2u64;
    let a3111 = {
        let a3108 = 2u64;
        let a3109 = v0;
        nat_mul(a3108, a3109)?
    };
    nat_add(a3110, a3111)
}

pub fn f97(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3113 = match v1.clone() {
        Adt14::C0(h3112, _, _, _, _, _, _, _, _, _, _) => {
            h3112
        }
    };
    let a3114 = 0u64;
    if if if if nat_eq(a3113, a3114) {
        let a3116 = match v2.clone() {
            Adt14::C0(h3115, _, _, _, _, _, _, _, _, _, _) => {
                h3115
            }
        };
        let a3117 = 0u64;
        nat_eq(a3116, a3117)
    } else {
        false
    } {
        f201(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f202(match v1.clone() {
            Adt14::C0(_, h3118, _, _, _, _, _, _, _, _, _) => {
                h3118
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3119, _, _, _, _, _, _, _, _) => {
                h3119
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3120, _, _, _, _, _, _, _) => {
                h3120
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3121, _, _, _, _, _, _) => {
                h3121
            }
        }) {
            true
        } else {
            if if if if f45(v0) {
                f203(v1.clone(), v2.clone())
            } else {
                false
            } {
                f204(v1.clone())
            } else {
                false
            } {
                f204(v2.clone())
            } else {
                false
            } {
                if f205(v1.clone(), v2.clone())? {
                    true
                } else {
                    f206(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f202(match v1.clone() {
            Adt14::C0(_, h3122, _, _, _, _, _, _, _, _, _) => {
                h3122
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3123, _, _, _, _, _, _, _, _) => {
                h3123
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3124, _, _, _, _, _, _, _) => {
                h3124
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3125, _, _, _, _, _, _) => {
                h3125
            }
        }) {
            0u64
        } else {
            let a3131 = match v1.clone() {
                Adt14::C0(_, _, h3126, _, _, _, _, _, _, _, _) => {
                    h3126
                }
            };
            let a3132 = {
                let a3129 = match v1.clone() {
                    Adt14::C0(_, _, _, h3127, _, _, _, _, _, _, _) => {
                        h3127
                    }
                };
                let a3130 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3128, _, _, _, _, _, _) => {
                        h3128
                    }
                };
                nat_mul(a3129, a3130)?
            };
            nat_mul(a3131, a3132)?
        };
        let v4: u64 = {
            let a3133 = v3;
            let a3134 = 0u64;
            if if nat_eq(a3133, a3134) {
                true
            } else {
                f207(v2.clone())?
            } {
                0u64
            } else {
                let a3136 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3135, _) => {
                        h3135
                    }
                };
                let a3137 = 4u64;
                let a3138 = 0u64;
                nat_quot(a3136, a3137, a3138)
            }
        };
        Ok(Adt15::C1(f208(f209(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3139, _, _, _, _, _) => {
                h3139
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3140, _, _, _, _, _) => {
                h3140
            }
        }), match v1 {
            Adt14::C0(_, h3141, _, _, _, _, _, _, _, _, _) => {
                h3141
            }
        }, v3, v4)?, 4u64, 64u64, v3, f210(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f211(v3.clone());
    let a3143 = match v2.clone() {
        Adt14::C0(h3142, _, _, _, _, _, _, _, _, _, _) => {
            h3142
        }
    };
    let a3144 = 0u64;
    if if if if nat_eq(a3143, a3144) {
        let a3146 = match v3.clone() {
            Adt14::C0(h3145, _, _, _, _, _, _, _, _, _, _) => {
                h3145
            }
        };
        let a3147 = 0u64;
        nat_eq(a3146, a3147)
    } else {
        false
    } {
        f201(v2.clone(), v3)
    } else {
        false
    } {
        if f202(match v2.clone() {
            Adt14::C0(_, h3148, _, _, _, _, _, _, _, _, _) => {
                h3148
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3149, _, _, _, _, _, _, _, _) => {
                h3149
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3150, _, _, _, _, _, _, _) => {
                h3150
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3151, _, _, _, _, _, _) => {
                h3151
            }
        }) {
            true
        } else {
            let a3153 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3152, _, _, _, _, _) => {
                    h3152
                }
            };
            let a3154 = 4u64;
            if if if if if nat_eq(a3153, a3154) {
                f212(match v2.clone() {
                    Adt14::C0(_, h3155, _, _, _, _, _, _, _, _, _) => {
                        h3155
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3156, _, _, _, _, _, _, _, _) => {
                        h3156
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3157, _, _, _, _, _, _, _) => {
                        h3157
                    }
                }, v0)?
            } else {
                false
            } {
                f204(v2.clone())
            } else {
                false
            } {
                f204(v4.clone())
            } else {
                false
            } {
                let a3159 = f213(v4.clone())?;
                let a3160 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3158, _) => {
                        h3158
                    }
                };
                nat_le(a3159, a3160)
            } else {
                false
            } {
                if f205(v2.clone(), v4.clone())? {
                    true
                } else {
                    f206(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f202(match v2.clone() {
            Adt14::C0(_, h3161, _, _, _, _, _, _, _, _, _) => {
                h3161
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3162, _, _, _, _, _, _, _, _) => {
                h3162
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3163, _, _, _, _, _, _, _) => {
                h3163
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3164, _, _, _, _, _, _) => {
                h3164
            }
        }) {
            true
        } else {
            f207(v4.clone())?
        } {
            0u64
        } else {
            let a3166 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3165, _) => {
                    h3165
                }
            };
            let a3167 = 4u64;
            let a3168 = 0u64;
            nat_quot(a3166, a3167, a3168)
        };
        Ok(Adt15::C1(f214(match v2.clone() {
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
        }, match v2 {
            Adt14::C0(_, _, _, _, h3172, _, _, _, _, _, _) => {
                h3172
            }
        }, v0, v5)?, 4u64, 65u64, v0, f210(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f99(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f211(v4.clone());
    if if if if if if if v1 {
        let a3174 = match v2.clone() {
            Adt14::C0(h3173, _, _, _, _, _, _, _, _, _, _) => {
                h3173
            }
        };
        let a3175 = 0u64;
        nat_eq(a3174, a3175)
    } else {
        false
    } {
        let a3177 = match v3.clone() {
            Adt14::C0(h3176, _, _, _, _, _, _, _, _, _, _) => {
                h3176
            }
        };
        let a3178 = 0u64;
        nat_eq(a3177, a3178)
    } else {
        false
    } {
        let a3180 = match v4.clone() {
            Adt14::C0(h3179, _, _, _, _, _, _, _, _, _, _) => {
                h3179
            }
        };
        let a3181 = 0u64;
        nat_eq(a3180, a3181)
    } else {
        false
    } {
        f201(v2.clone(), v3.clone())
    } else {
        false
    } {
        f201(v2.clone(), v4)
    } else {
        false
    } {
        if f202(match v2.clone() {
            Adt14::C0(_, h3182, _, _, _, _, _, _, _, _, _) => {
                h3182
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3183, _, _, _, _, _, _, _, _) => {
                h3183
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3184, _, _, _, _, _, _, _) => {
                h3184
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3185, _, _, _, _, _, _) => {
                h3185
            }
        }) {
            true
        } else {
            let a3187 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3186, _, _, _, _, _) => {
                    h3186
                }
            };
            let a3188 = 4u64;
            if if if if if if if nat_eq(a3187, a3188) {
                let a3190 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3189, _, _, _, _, _) => {
                        h3189
                    }
                };
                let a3191 = 4u64;
                nat_eq(a3190, a3191)
            } else {
                false
            } {
                f204(v2.clone())
            } else {
                false
            } {
                f204(v3.clone())
            } else {
                false
            } {
                f204(v5.clone())
            } else {
                false
            } {
                let a3193 = f213(v5.clone())?;
                let a3194 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3192, _) => {
                        h3192
                    }
                };
                nat_le(a3193, a3194)
            } else {
                false
            } {
                if f205(v2.clone(), v5.clone())? {
                    true
                } else {
                    f206(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f205(v3.clone(), v5.clone())? {
                    true
                } else {
                    f206(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f202(match v2.clone() {
            Adt14::C0(_, h3195, _, _, _, _, _, _, _, _, _) => {
                h3195
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3196, _, _, _, _, _, _, _, _) => {
                h3196
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3197, _, _, _, _, _, _, _) => {
                h3197
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3198, _, _, _, _, _, _) => {
                h3198
            }
        }) {
            0u64
        } else {
            let a3204 = match v2.clone() {
                Adt14::C0(_, _, h3199, _, _, _, _, _, _, _, _) => {
                    h3199
                }
            };
            let a3205 = {
                let a3202 = match v2.clone() {
                    Adt14::C0(_, _, _, h3200, _, _, _, _, _, _, _) => {
                        h3200
                    }
                };
                let a3203 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3201, _, _, _, _, _, _) => {
                        h3201
                    }
                };
                nat_mul(a3202, a3203)?
            };
            nat_mul(a3204, a3205)?
        };
        let v7: u64 = {
            let a3206 = v6;
            let a3207 = 0u64;
            if if nat_eq(a3206, a3207) {
                true
            } else {
                f207(v5.clone())?
            } {
                0u64
            } else {
                let a3209 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3208, _) => {
                        h3208
                    }
                };
                let a3210 = 4u64;
                let a3211 = 0u64;
                nat_quot(a3209, a3210, a3211)
            }
        };
        Ok(Adt15::C1(f215(match v2 {
            Adt14::C0(_, h3212, _, _, _, _, _, _, _, _, _) => {
                h3212
            }
        }, v6, v7)?, 4u64, 66u64, v6, f210(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f216(v0, v3, v6) {
        f216(v1, v4, v6)
    } else {
        false
    } {
        f216(v2, v5, v6)
    } else {
        false
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f217(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f217(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f217(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f217(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f217(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f217(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3213 = v2;
    let a3214 = 0u64;
    if nat_eq(a3213, a3214) {
        Ok(0u64)
    } else {
        let a3217 = 2u64;
        let a3218 = {
            let a3215 = v0;
            let a3216 = v1;
            nat_mul(a3215, a3216)?
        };
        nat_mul(a3217, a3218)
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3219 = v2;
    let a3220 = f197(v0, v1)?;
    nat_mul(a3219, a3220)
}

pub fn f104(v0: u64) -> u64 {
    let a3221 = v0;
    let a3222 = 30u64;
    if nat_eq(a3221, a3222) {
        3u64
    } else {
        1u64
    }
}

pub fn f105(v0: u64) -> R<u64> {
    let a3225 = 5u64;
    let a3226 = {
        let a3223 = 4u64;
        let a3224 = v0;
        nat_mul(a3223, a3224)?
    };
    nat_add(a3225, a3226)
}

pub fn f106(v0: u64) -> bool {
    let m3227 = f218(v0);
    match m3227 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3229 = 1u64;
            let a3230 = match v1 {
                Adt19::C0(_, h3228, _) => {
                    h3228
                }
            };
            nat_lt(a3229, a3230)
        }
    }
}

pub fn f107(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3231, _, _, _, _, _, _, _, _, _, _, _) => {
            h3231
        }
    };
    let v4: u64 = match v0 {
        Adt1::C0(_, h3232, _, _, _, _, _, _, _, _, _, _) => {
            h3232
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, h3233, _, _, _, _, _, _, _, _, _) => {
            h3233
        }
    };
    let a3235 = match v1.clone() {
        Adt2::C0(h3234, _, _, _, _, _, _, _, _, _, _) => {
            h3234
        }
    };
    let a3236 = 0u64;
    if if if if if if if if if if if if if nat_eq(a3235, a3236) {
        let a3238 = match v2.clone() {
            Adt2::C0(h3237, _, _, _, _, _, _, _, _, _, _) => {
                h3237
            }
        };
        let a3239 = 0u64;
        nat_eq(a3238, a3239)
    } else {
        false
    } {
        let a3241 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h3240, _, _, _, _, _) => {
                h3240
            }
        };
        let a3242 = 4u64;
        nat_eq(a3241, a3242)
    } else {
        false
    } {
        let a3244 = match v2.clone() {
            Adt2::C0(_, h3243, _, _, _, _, _, _, _, _, _) => {
                h3243
            }
        };
        let a3245 = v3;
        nat_eq(a3244, a3245)
    } else {
        false
    } {
        let a3247 = match v2.clone() {
            Adt2::C0(_, _, h3246, _, _, _, _, _, _, _, _) => {
                h3246
            }
        };
        let a3248 = v5;
        nat_eq(a3247, a3248)
    } else {
        false
    } {
        let a3250 = match v2.clone() {
            Adt2::C0(_, _, _, h3249, _, _, _, _, _, _, _) => {
                h3249
            }
        };
        let a3251 = 1u64;
        nat_eq(a3250, a3251)
    } else {
        false
    } {
        let a3253 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3252, _, _, _, _, _, _) => {
                h3252
            }
        };
        let a3254 = 1u64;
        nat_eq(a3253, a3254)
    } else {
        false
    } {
        let a3255 = v3;
        let a3256 = 2147483648u64;
        nat_lt(a3255, a3256)
    } else {
        false
    } {
        let a3257 = v4;
        let a3258 = 4294967296u64;
        nat_lt(a3257, a3258)
    } else {
        false
    } {
        f38(v2.clone())?
    } else {
        false
    } {
        let a3262 = {
            let a3259 = v5;
            let a3260 = 4u64;
            nat_mul(a3259, a3260)?
        };
        let a3263 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3261, _) => {
                h3261
            }
        };
        nat_le(a3262, a3263)
    } else {
        false
    } {
        f26(v2.clone(), f23(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3264 = v5;
            let a3265 = 0u64;
            if nat_eq(a3264, a3265) {
                0u64
            } else {
                f219(v3, v5)?
            }
        }, 4u64, f25(v1, 1u64), f25(v2, 1u64), Adt4::C44(v4)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f21(v2.clone(), v3.clone()) {
        let a3267 = match v3.clone() {
            Adt2::C0(h3266, _, _, _, _, _, _, _, _, _, _) => {
                h3266
            }
        };
        let a3268 = 0u64;
        nat_eq(a3267, a3268)
    } else {
        false
    } {
        f38(v3.clone())?
    } else {
        false
    } {
        f26(v3.clone(), f23(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        f26(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f30(v2.clone(), v3.clone())?
    } else {
        false
    } {
        let a3270 = match v2.clone() {
            Adt2::C0(h3269, _, _, _, _, _, _, _, _, _, _) => {
                h3269
            }
        };
        let a3271 = 1u64;
        nat_le(a3270, a3271)
    } else {
        false
    } {
        let a3273 = match v0.clone() {
            Adt1::C0(h3272, _, _, _, _, _, _, _, _, _, _, _) => {
                h3272
            }
        };
        let a3274 = 0u64;
        if nat_eq(a3273, a3274) {
            let a3276 = match v2.clone() {
                Adt2::C0(h3275, _, _, _, _, _, _, _, _, _, _) => {
                    h3275
                }
            };
            let a3277 = 0u64;
            if if nat_eq(a3276, a3277) {
                f38(v2.clone())?
            } else {
                false
            } {
                f26(v2.clone(), f23(v2, 1u64)?)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        f117(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f109(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3279 = match v1.clone() {
        Adt2::C0(h3278, _, _, _, _, _, _, _, _, _, _) => {
            h3278
        }
    };
    let a3280 = 0u64;
    if if if if if if if nat_eq(a3279, a3280) {
        let a3282 = match v3.clone() {
            Adt2::C0(h3281, _, _, _, _, _, _, _, _, _, _) => {
                h3281
            }
        };
        let a3283 = 0u64;
        nat_eq(a3282, a3283)
    } else {
        false
    } {
        let a3285 = match v2.clone() {
            Adt2::C0(h3284, _, _, _, _, _, _, _, _, _, _) => {
                h3284
            }
        };
        let a3286 = 1u64;
        nat_le(a3285, a3286)
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
        let a3288 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3287, _, _, _, _, _) => {
                h3287
            }
        };
        let a3289 = 0u64;
        if nat_eq(a3288, a3289) {
            true
        } else {
            let a3291 = match v3.clone() {
                Adt2::C0(_, _, h3290, _, _, _, _, _, _, _, _) => {
                    h3290
                }
            };
            let a3292 = 1u64;
            if nat_le(a3291, a3292) {
                true
            } else {
                let a3297 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3293, _, _, _, _) => {
                        h3293
                    }
                };
                let a3298 = {
                    let a3295 = 4u64;
                    let a3296 = match v3.clone() {
                        Adt2::C0(_, h3294, _, _, _, _, _, _, _, _, _) => {
                            h3294
                        }
                    };
                    nat_mul(a3295, a3296)?
                };
                nat_eq(a3297, a3298)
            }
        }
    } else {
        false
    } {
        f118(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3300 = match v0.clone() {
        Adt2::C0(h3299, _, _, _, _, _, _, _, _, _, _) => {
            h3299
        }
    };
    let a3301 = 0u64;
    if if if if if if if if if if if if nat_eq(a3300, a3301) {
        f38(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3303 = match v1.clone() {
            Adt2::C0(h3302, _, _, _, _, _, _, _, _, _, _) => {
                h3302
            }
        };
        let a3304 = 0u64;
        if if nat_eq(a3303, a3304) {
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
        let a3306 = match v2.clone() {
            Adt2::C0(h3305, _, _, _, _, _, _, _, _, _, _) => {
                h3305
            }
        };
        let a3307 = 0u64;
        if if nat_eq(a3306, a3307) {
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
        let a3310 = match v0.clone() {
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
        let a3314 = match v0.clone() {
            Adt2::C0(_, _, h3312, _, _, _, _, _, _, _, _) => {
                h3312
            }
        };
        let a3315 = match v1.clone() {
            Adt2::C0(_, _, h3313, _, _, _, _, _, _, _, _) => {
                h3313
            }
        };
        nat_eq(a3314, a3315)
    } else {
        false
    } {
        let a3318 = match v0.clone() {
            Adt2::C0(_, _, _, h3316, _, _, _, _, _, _, _) => {
                h3316
            }
        };
        let a3319 = match v1.clone() {
            Adt2::C0(_, _, _, h3317, _, _, _, _, _, _, _) => {
                h3317
            }
        };
        nat_eq(a3318, a3319)
    } else {
        false
    } {
        let a3322 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3320, _, _, _, _, _, _) => {
                h3320
            }
        };
        let a3323 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3321, _, _, _, _, _, _) => {
                h3321
            }
        };
        nat_eq(a3322, a3323)
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
            let a3332 = {
                let a3329 = {
                    let a3326 = match v1.clone() {
                        Adt2::C0(_, _, _, h3324, _, _, _, _, _, _, _) => {
                            h3324
                        }
                    };
                    let a3327 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3325, _, _, _, _, _, _) => {
                            h3325
                        }
                    };
                    nat_mul(a3326, a3327)?
                };
                let a3330 = match v1 {
                    Adt2::C0(_, h3328, _, _, _, _, _, _, _, _, _) => {
                        h3328
                    }
                };
                nat_mul(a3329, a3330)?
            };
            let a3333 = f177(match v0.clone() {
                Adt2::C0(_, h3331, _, _, _, _, _, _, _, _, _) => {
                    h3331
                }
            })?;
            nat_mul(a3332, a3333)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f111(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3335 = 0u64;
        let a3336 = match v0.clone() {
            Adt1::C0(_, h3334, _, _, _, _, _, _, _, _, _, _) => {
                h3334
            }
        };
        nat_lt(a3335, a3336)
    };
    let a3338 = match v1.clone() {
        Adt2::C0(h3337, _, _, _, _, _, _, _, _, _, _) => {
            h3337
        }
    };
    let a3339 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3338, a3339) {
        let a3341 = match v2.clone() {
            Adt2::C0(h3340, _, _, _, _, _, _, _, _, _, _) => {
                h3340
            }
        };
        let a3342 = 0u64;
        nat_eq(a3341, a3342)
    } else {
        false
    } {
        let a3344 = match v3.clone() {
            Adt2::C0(h3343, _, _, _, _, _, _, _, _, _, _) => {
                h3343
            }
        };
        let a3345 = 0u64;
        nat_eq(a3344, a3345)
    } else {
        false
    } {
        let a3347 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3346, _, _, _, _, _, _) => {
                h3346
            }
        };
        let a3348 = 1u64;
        nat_eq(a3347, a3348)
    } else {
        false
    } {
        let a3350 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3349, _, _, _, _, _, _) => {
                h3349
            }
        };
        let a3351 = 1u64;
        nat_eq(a3350, a3351)
    } else {
        false
    } {
        let a3353 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3352, _, _, _, _, _, _) => {
                h3352
            }
        };
        let a3354 = 1u64;
        nat_eq(a3353, a3354)
    } else {
        false
    } {
        let a3356 = 0u64;
        let a3357 = match v1.clone() {
            Adt2::C0(_, _, h3355, _, _, _, _, _, _, _, _) => {
                h3355
            }
        };
        nat_lt(a3356, a3357)
    } else {
        false
    } {
        let a3359 = match v3.clone() {
            Adt2::C0(_, _, _, h3358, _, _, _, _, _, _, _) => {
                h3358
            }
        };
        let a3360 = 1u64;
        nat_eq(a3359, a3360)
    } else {
        false
    } {
        let a3363 = match v3.clone() {
            Adt2::C0(_, h3361, _, _, _, _, _, _, _, _, _) => {
                h3361
            }
        };
        let a3364 = match v1.clone() {
            Adt2::C0(_, h3362, _, _, _, _, _, _, _, _, _) => {
                h3362
            }
        };
        nat_eq(a3363, a3364)
    } else {
        false
    } {
        let a3367 = match v3.clone() {
            Adt2::C0(_, _, h3365, _, _, _, _, _, _, _, _) => {
                h3365
            }
        };
        let a3368 = match v1.clone() {
            Adt2::C0(_, _, _, h3366, _, _, _, _, _, _, _) => {
                h3366
            }
        };
        nat_eq(a3367, a3368)
    } else {
        false
    } {
        if v4 {
            f21(v1.clone(), v2.clone())
        } else {
            let a3371 = match v2.clone() {
                Adt2::C0(_, h3369, _, _, _, _, _, _, _, _, _) => {
                    h3369
                }
            };
            let a3372 = match v1.clone() {
                Adt2::C0(_, _, h3370, _, _, _, _, _, _, _, _) => {
                    h3370
                }
            };
            if if nat_eq(a3371, a3372) {
                let a3375 = match v2.clone() {
                    Adt2::C0(_, _, h3373, _, _, _, _, _, _, _, _) => {
                        h3373
                    }
                };
                let a3376 = match v1.clone() {
                    Adt2::C0(_, _, _, h3374, _, _, _, _, _, _, _) => {
                        h3374
                    }
                };
                nat_eq(a3375, a3376)
            } else {
                false
            } {
                let a3378 = match v2.clone() {
                    Adt2::C0(_, _, _, h3377, _, _, _, _, _, _, _) => {
                        h3377
                    }
                };
                let a3379 = 1u64;
                nat_eq(a3378, a3379)
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
            let a3381 = f23(v3.clone(), 1u64)?;
            let a3382 = f171(match v1.clone() {
                Adt2::C0(_, _, h3380, _, _, _, _, _, _, _, _) => {
                    h3380
                }
            }, v4)?;
            nat_mul(a3381, a3382)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C36(f25(v2, 1u64), v4, match v0 {
            Adt1::C0(h3383, _, _, _, _, _, _, _, _, _, _, _) => {
                h3383
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3384, _, _, _, _, _, _, _, _, _) => {
            h3384
        }
    }, match v0.clone() {
        Adt1::C0(h3385, _, _, _, _, _, _, _, _, _, _, _) => {
            h3385
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3386, _, _, _, _, _, _, _, _, _) => {
            h3386
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3387, _, _, _, _, _, _, _) => {
            h3387
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3388, _, _, _, _, _, _, _, _) => {
            h3388
        }
    }, match v0.clone() {
        Adt1::C0(_, h3389, _, _, _, _, _, _, _, _, _, _) => {
            h3389
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3390, _, _, _, _, _, _, _, _) => {
            h3390
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3391, _, _, _, _, _, _) => {
            h3391
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3392, _, _, _, _, _, _, _, _, _, _) => {
            h3392
        }
    };
    let v7: bool = {
        let a3395 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3393, _, _, _) => {
                h3393
            }
        };
        let a3396 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3394, _, _, _, _, _) => {
                h3394
            }
        };
        nat_lt(a3395, a3396)
    };
    let a3397 = v6;
    let a3398 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3397, a3398) {
        true
    } else {
        let a3399 = v6;
        let a3400 = 1u64;
        nat_eq(a3399, a3400)
    } {
        let a3402 = match v2.clone() {
            Adt2::C0(h3401, _, _, _, _, _, _, _, _, _, _) => {
                h3401
            }
        };
        let a3403 = 0u64;
        nat_eq(a3402, a3403)
    } else {
        false
    } {
        let a3405 = match v3.clone() {
            Adt2::C0(h3404, _, _, _, _, _, _, _, _, _, _) => {
                h3404
            }
        };
        let a3406 = 0u64;
        nat_eq(a3405, a3406)
    } else {
        false
    } {
        let a3408 = match v1.clone() {
            Adt2::C0(_, _, _, h3407, _, _, _, _, _, _, _) => {
                h3407
            }
        };
        let a3409 = 1u64;
        nat_eq(a3408, a3409)
    } else {
        false
    } {
        let a3412 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3410, _, _, _, _, _, _) => {
                h3410
            }
        };
        let a3413 = match v2.clone() {
            Adt2::C0(_, _, _, h3411, _, _, _, _, _, _, _) => {
                h3411
            }
        };
        nat_eq(a3412, a3413)
    } else {
        false
    } {
        let a3416 = match v3.clone() {
            Adt2::C0(_, _, _, h3414, _, _, _, _, _, _, _) => {
                h3414
            }
        };
        let a3417 = match v2.clone() {
            Adt2::C0(_, _, _, h3415, _, _, _, _, _, _, _) => {
                h3415
            }
        };
        nat_eq(a3416, a3417)
    } else {
        false
    } {
        let a3420 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3418, _, _, _, _, _, _) => {
                h3418
            }
        };
        let a3421 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3419, _, _, _, _, _, _) => {
                h3419
            }
        };
        nat_eq(a3420, a3421)
    } else {
        false
    } {
        let a3423 = 0u64;
        let a3424 = match v4.clone() {
            Adt6::C0(h3422, _, _, _) => {
                h3422
            }
        };
        if if if if if nat_lt(a3423, a3424) {
            let a3426 = 0u64;
            let a3427 = match v4.clone() {
                Adt6::C0(_, h3425, _, _) => {
                    h3425
                }
            };
            nat_lt(a3426, a3427)
        } else {
            false
        } {
            let a3429 = match v4.clone() {
                Adt6::C0(_, h3428, _, _) => {
                    h3428
                }
            };
            let a3430 = 2147483648u64;
            nat_lt(a3429, a3430)
        } else {
            false
        } {
            let a3432 = 0u64;
            let a3433 = match v4.clone() {
                Adt6::C0(_, _, _, h3431) => {
                    h3431
                }
            };
            nat_lt(a3432, a3433)
        } else {
            false
        } {
            let a3435 = match v4.clone() {
                Adt6::C0(_, _, _, h3434) => {
                    h3434
                }
            };
            let a3436 = 2147483648u64;
            nat_lt(a3435, a3436)
        } else {
            false
        } {
            let a3439 = match v3.clone() {
                Adt2::C0(_, h3437, _, _, _, _, _, _, _, _, _) => {
                    h3437
                }
            };
            let a3440 = f94(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3438, _, _, _, _, _, _, _, _, _) => {
                    h3438
                }
            })?;
            nat_eq(a3439, a3440)
        } else {
            false
        }
    } else {
        false
    } {
        let a3442 = 0u64;
        let a3443 = match v5.clone() {
            Adt6::C0(h3441, _, _, _) => {
                h3441
            }
        };
        if if if if if nat_lt(a3442, a3443) {
            let a3445 = 0u64;
            let a3446 = match v5.clone() {
                Adt6::C0(_, h3444, _, _) => {
                    h3444
                }
            };
            nat_lt(a3445, a3446)
        } else {
            false
        } {
            let a3448 = match v5.clone() {
                Adt6::C0(_, h3447, _, _) => {
                    h3447
                }
            };
            let a3449 = 2147483648u64;
            nat_lt(a3448, a3449)
        } else {
            false
        } {
            let a3451 = 0u64;
            let a3452 = match v5.clone() {
                Adt6::C0(_, _, _, h3450) => {
                    h3450
                }
            };
            nat_lt(a3451, a3452)
        } else {
            false
        } {
            let a3454 = match v5.clone() {
                Adt6::C0(_, _, _, h3453) => {
                    h3453
                }
            };
            let a3455 = 2147483648u64;
            nat_lt(a3454, a3455)
        } else {
            false
        } {
            let a3458 = match v3.clone() {
                Adt2::C0(_, _, h3456, _, _, _, _, _, _, _, _) => {
                    h3456
                }
            };
            let a3459 = f94(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3457, _, _, _, _, _, _, _, _) => {
                    h3457
                }
            })?;
            nat_eq(a3458, a3459)
        } else {
            false
        }
    } else {
        false
    } {
        f220(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f220(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f220(v1.clone(), {
            let a3460 = v6;
            let a3461 = 0u64;
            if nat_eq(a3460, a3461) {
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
            let a3470 = {
                let a3468 = {
                    let a3466 = 3u64;
                    let a3467 = {
                        let a3464 = match v4.clone() {
                            Adt6::C0(h3462, _, _, _) => {
                                h3462
                            }
                        };
                        let a3465 = match v5.clone() {
                            Adt6::C0(h3463, _, _, _) => {
                                h3463
                            }
                        };
                        nat_mul(a3464, a3465)?
                    };
                    nat_mul(a3466, a3467)?
                };
                let a3469 = 2u64;
                nat_add(a3468, a3469)?
            };
            let a3471 = f23(v3.clone(), 1u64)?;
            nat_mul(a3470, a3471)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C34(f25(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f113(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
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
            let a3472 = 6u64;
            let a3473 = f23(v0.clone(), 1u64)?;
            nat_mul(a3472, a3473)?
        }, 2u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C33(f25(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3474 = v2;
    match m3474 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3475, _, _, _, _, _, _, _, _, _) => {
                    h3475
                }
            };
            let v6: u64 = {
                let a3481 = match v1.clone() {
                    Adt2::C0(_, _, h3476, _, _, _, _, _, _, _, _) => {
                        h3476
                    }
                };
                let a3482 = {
                    let a3479 = match v1.clone() {
                        Adt2::C0(_, _, _, h3477, _, _, _, _, _, _, _) => {
                            h3477
                        }
                    };
                    let a3480 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3478, _, _, _, _, _, _) => {
                            h3478
                        }
                    };
                    nat_mul(a3479, a3480)?
                };
                nat_mul(a3481, a3482)?
            };
            if if if if if if if if if if f21(v1.clone(), v4.clone()) {
                f21(v1.clone(), v3.clone())
            } else {
                false
            } {
                f221(v1.clone(), v6)?
            } else {
                false
            } {
                f221(v4.clone(), v6)?
            } else {
                false
            } {
                f221(v3.clone(), v6)?
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
                let a3483 = v5;
                let a3484 = 2147483647u64;
                nat_le(a3483, a3484)
            } else {
                false
            } {
                let a3485 = v6;
                let a3486 = 2147483647u64;
                nat_le(a3485, a3486)
            } else {
                false
            } {
                let a3488 = match v0.clone() {
                    Adt1::C0(h3487, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3487
                    }
                };
                let a3489 = 4294967295u64;
                if nat_le(a3488, a3489) {
                    let a3491 = match v0.clone() {
                        Adt1::C0(_, h3490, _, _, _, _, _, _, _, _, _, _) => {
                            h3490
                        }
                    };
                    let a3492 = 4294967295u64;
                    nat_le(a3491, a3492)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3493 = v5;
                    let a3494 = 0u64;
                    if nat_eq(a3493, a3494) {
                        0u64
                    } else {
                        let a3495 = v6;
                        let a3496 = f170(v5)?;
                        nat_mul(a3495, a3496)?
                    }
                }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3497, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3497
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f115(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3498, _, _, _, _, _, _, _, _, _, _, _) => {
            h3498
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3499, _, _, _, _, _, _, _, _, _, _) => {
            h3499
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3500, _, _, _, _, _, _, _, _, _) => {
            h3500
        }
    };
    let v6: u64 = {
        let a3502 = match v1.clone() {
            Adt2::C0(_, h3501, _, _, _, _, _, _, _, _, _) => {
                h3501
            }
        };
        let a3503 = v4;
        let a3504 = 0u64;
        nat_quot(a3502, a3503, a3504)
    };
    let v7: u64 = {
        let a3506 = match v1.clone() {
            Adt2::C0(h3505, _, _, _, _, _, _, _, _, _, _) => {
                h3505
            }
        };
        let a3507 = 0u64;
        if nat_eq(a3506, a3507) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f181(match v1.clone() {
        Adt2::C0(h3508, _, _, _, _, _, _, _, _, _, _) => {
            h3508
        }
    }) {
        let a3511 = match v1.clone() {
            Adt2::C0(h3509, _, _, _, _, _, _, _, _, _, _) => {
                h3509
            }
        };
        let a3512 = match v2.clone() {
            Adt2::C0(h3510, _, _, _, _, _, _, _, _, _, _) => {
                h3510
            }
        };
        nat_eq(a3511, a3512)
    } else {
        false
    } {
        let a3513 = 0u64;
        let a3514 = v3;
        nat_lt(a3513, a3514)
    } else {
        false
    } {
        let a3515 = v3;
        let a3516 = 2147483648u64;
        nat_lt(a3515, a3516)
    } else {
        false
    } {
        let a3517 = 0u64;
        let a3518 = v4;
        nat_lt(a3517, a3518)
    } else {
        false
    } {
        let a3519 = v4;
        let a3520 = 2147483648u64;
        nat_lt(a3519, a3520)
    } else {
        false
    } {
        let a3521 = v5;
        let a3522 = 2147483648u64;
        nat_lt(a3521, a3522)
    } else {
        false
    } {
        let a3523 = 0u64;
        let a3524 = v6;
        nat_lt(a3523, a3524)
    } else {
        false
    } {
        let a3526 = 0u64;
        let a3527 = match v2.clone() {
            Adt2::C0(_, h3525, _, _, _, _, _, _, _, _, _) => {
                h3525
            }
        };
        nat_lt(a3526, a3527)
    } else {
        false
    } {
        let a3531 = match v1.clone() {
            Adt2::C0(_, h3528, _, _, _, _, _, _, _, _, _) => {
                h3528
            }
        };
        let a3532 = {
            let a3529 = v6;
            let a3530 = v4;
            nat_mul(a3529, a3530)?
        };
        nat_eq(a3531, a3532)
    } else {
        false
    } {
        let a3534 = match v2.clone() {
            Adt2::C0(_, _, h3533, _, _, _, _, _, _, _, _) => {
                h3533
            }
        };
        let a3535 = v4;
        nat_eq(a3534, a3535)
    } else {
        false
    } {
        let a3548 = {
            let a3541 = {
                let a3539 = match v2.clone() {
                    Adt2::C0(_, h3536, _, _, _, _, _, _, _, _, _) => {
                        h3536
                    }
                };
                let a3540 = {
                    let a3537 = v5;
                    let a3538 = 2u64;
                    nat_mul(a3537, a3538)?
                };
                nat_add(a3539, a3540)?
            };
            let a3542 = v3;
            nat_add(a3541, a3542)?
        };
        let a3549 = {
            let a3546 = {
                let a3544 = match v1.clone() {
                    Adt2::C0(_, _, h3543, _, _, _, _, _, _, _, _) => {
                        h3543
                    }
                };
                let a3545 = v3;
                nat_mul(a3544, a3545)?
            };
            let a3547 = v6;
            nat_add(a3546, a3547)?
        };
        nat_eq(a3548, a3549)
    } else {
        false
    } {
        f222(v1.clone(), v7)?
    } else {
        false
    } {
        f222(v2.clone(), v7)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3550 = f23(v2.clone(), 1u64)?;
            let a3551 = f194(v8.clone())?;
            nat_mul(a3550, a3551)?
        }, v7, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3552, _, _, _, _, _, _, _, _, _, _) => {
                h3552
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3553 = v1;
    match m3553 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3555 = match v0.clone() {
                Adt2::C0(h3554, _, _, _, _, _, _, _, _, _, _) => {
                    h3554
                }
            };
            let a3556 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3555, a3556) {
                let a3558 = match v3.clone() {
                    Adt2::C0(h3557, _, _, _, _, _, _, _, _, _, _) => {
                        h3557
                    }
                };
                let a3559 = 0u64;
                nat_eq(a3558, a3559)
            } else {
                false
            } {
                let a3561 = match v2.clone() {
                    Adt2::C0(h3560, _, _, _, _, _, _, _, _, _, _) => {
                        h3560
                    }
                };
                let a3562 = 0u64;
                nat_eq(a3561, a3562)
            } else {
                false
            } {
                let a3564 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3563, _, _, _, _, _, _) => {
                        h3563
                    }
                };
                let a3565 = 1u64;
                nat_eq(a3564, a3565)
            } else {
                false
            } {
                let a3567 = match v3.clone() {
                    Adt2::C0(_, _, _, h3566, _, _, _, _, _, _, _) => {
                        h3566
                    }
                };
                let a3568 = 1u64;
                nat_eq(a3567, a3568)
            } else {
                false
            } {
                let a3570 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3569, _, _, _, _, _, _) => {
                        h3569
                    }
                };
                let a3571 = 1u64;
                nat_eq(a3570, a3571)
            } else {
                false
            } {
                let a3574 = match v0.clone() {
                    Adt2::C0(_, _, h3572, _, _, _, _, _, _, _, _) => {
                        h3572
                    }
                };
                let a3575 = match v3.clone() {
                    Adt2::C0(_, _, h3573, _, _, _, _, _, _, _, _) => {
                        h3573
                    }
                };
                nat_eq(a3574, a3575)
            } else {
                false
            } {
                let a3580 = match v3.clone() {
                    Adt2::C0(_, h3576, _, _, _, _, _, _, _, _, _) => {
                        h3576
                    }
                };
                let a3581 = {
                    let a3578 = match v0.clone() {
                        Adt2::C0(_, h3577, _, _, _, _, _, _, _, _, _) => {
                            h3577
                        }
                    };
                    let a3579 = 1u64;
                    nat_add(a3578, a3579)?
                };
                nat_le(a3580, a3581)
            } else {
                false
            } {
                let a3584 = match v2.clone() {
                    Adt2::C0(_, h3582, _, _, _, _, _, _, _, _, _) => {
                        h3582
                    }
                };
                let a3585 = match v0.clone() {
                    Adt2::C0(_, _, h3583, _, _, _, _, _, _, _, _) => {
                        h3583
                    }
                };
                nat_eq(a3584, a3585)
            } else {
                false
            } {
                let a3593 = match v2.clone() {
                    Adt2::C0(_, _, h3586, _, _, _, _, _, _, _, _) => {
                        h3586
                    }
                };
                let a3594 = {
                    let a3591 = {
                        let a3588 = match v0.clone() {
                            Adt2::C0(_, h3587, _, _, _, _, _, _, _, _, _) => {
                                h3587
                            }
                        };
                        let a3589 = 1u64;
                        nat_add(a3588, a3589)?
                    };
                    let a3592 = match v3.clone() {
                        Adt2::C0(_, h3590, _, _, _, _, _, _, _, _, _) => {
                            h3590
                        }
                    };
                    nat_sub(a3591, a3592)
                };
                nat_eq(a3593, a3594)
            } else {
                false
            } {
                let a3597 = match v2.clone() {
                    Adt2::C0(_, _, _, h3595, _, _, _, _, _, _, _) => {
                        h3595
                    }
                };
                let a3598 = match v0.clone() {
                    Adt2::C0(_, _, _, h3596, _, _, _, _, _, _, _) => {
                        h3596
                    }
                };
                nat_eq(a3597, a3598)
            } else {
                false
            } {
                let a3600 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3599, _, _, _, _, _, _) => {
                        h3599
                    }
                };
                let a3601 = 1u64;
                nat_eq(a3600, a3601)
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                let a3603 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3602, _, _, _, _, _) => {
                        h3602
                    }
                };
                let a3604 = 4u64;
                nat_eq(a3603, a3604)
            } else {
                false
            } {
                let a3606 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3605, _, _, _, _, _) => {
                        h3605
                    }
                };
                let a3607 = 4u64;
                nat_eq(a3606, a3607)
            } else {
                false
            } {
                let a3612 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3608, _, _, _, _) => {
                        h3608
                    }
                };
                let a3613 = {
                    let a3610 = match v0.clone() {
                        Adt2::C0(_, h3609, _, _, _, _, _, _, _, _, _) => {
                            h3609
                        }
                    };
                    let a3611 = 4u64;
                    nat_mul(a3610, a3611)?
                };
                nat_eq(a3612, a3613)
            } else {
                false
            } {
                let a3618 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3614, _, _, _, _) => {
                        h3614
                    }
                };
                let a3619 = {
                    let a3616 = match v3.clone() {
                        Adt2::C0(_, h3615, _, _, _, _, _, _, _, _, _) => {
                            h3615
                        }
                    };
                    let a3617 = 4u64;
                    nat_mul(a3616, a3617)?
                };
                nat_eq(a3618, a3619)
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
                    let a3621 = f23(v2.clone(), 1u64)?;
                    let a3622 = f193(match v3.clone() {
                        Adt2::C0(_, h3620, _, _, _, _, _, _, _, _, _) => {
                            h3620
                        }
                    })?;
                    nat_mul(a3621, a3622)?
                }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C29(f25(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f117(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h3623, _, _, _, _, _, _, _, _, _, _) => {
            h3623
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h3624, _, _, _, _, _, _, _, _) => {
                h3624
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h3625, _, _, _, _, _, _, _, _, _) => {
                h3625
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h3626, _, _, _, _, _, _) => {
                h3626
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h3627, _, _, _, _, _, _, _, _) => {
                h3627
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h3628, _, _, _, _, _, _, _, _, _) => {
                h3628
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h3629, _, _, _, _, _, _, _) => {
                h3629
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h3630, _, _, _, _, _) => {
                h3630
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3632 = match v2.clone() {
        Adt1::C0(h3631, _, _, _, _, _, _, _, _, _, _, _) => {
            h3631
        }
    };
    let a3633 = 1u64;
    if if if if if if if if if if if nat_le(a3632, a3633) {
        let a3635 = match v3.clone() {
            Adt2::C0(h3634, _, _, _, _, _, _, _, _, _, _) => {
                h3634
            }
        };
        let a3636 = 1u64;
        nat_le(a3635, a3636)
    } else {
        false
    } {
        let a3638 = match v4.clone() {
            Adt2::C0(h3637, _, _, _, _, _, _, _, _, _, _) => {
                h3637
            }
        };
        let a3639 = 0u64;
        nat_eq(a3638, a3639)
    } else {
        false
    } {
        let a3644 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h3640, _, _, _, _, _) => {
                h3640
            }
        };
        let a3645 = {
            let a3642 = match v3.clone() {
                Adt2::C0(h3641, _, _, _, _, _, _, _, _, _, _) => {
                    h3641
                }
            };
            let a3643 = 0u64;
            if nat_eq(a3642, a3643) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3644, a3645)
    } else {
        false
    } {
        f38(v4.clone())?
    } else {
        false
    } {
        f30(v3.clone(), v4.clone())?
    } else {
        false
    } {
        f223(v5.clone(), match v3.clone() {
            Adt2::C0(_, h3646, _, _, _, _, _, _, _, _, _) => {
                h3646
            }
        }, match v4.clone() {
            Adt2::C0(_, h3647, _, _, _, _, _, _, _, _, _) => {
                h3647
            }
        })?
    } else {
        false
    } {
        if v1 {
            f223(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h3648, _, _, _, _, _, _, _, _) => {
                    h3648
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h3649, _, _, _, _, _, _, _, _) => {
                    h3649
                }
            })?
        } else {
            let a3652 = match v3.clone() {
                Adt2::C0(_, _, h3650, _, _, _, _, _, _, _, _) => {
                    h3650
                }
            };
            let a3653 = match v4.clone() {
                Adt2::C0(_, _, h3651, _, _, _, _, _, _, _, _) => {
                    h3651
                }
            };
            nat_eq(a3652, a3653)
        }
    } else {
        false
    } {
        let a3656 = match v3.clone() {
            Adt2::C0(_, _, _, h3654, _, _, _, _, _, _, _) => {
                h3654
            }
        };
        let a3657 = match v4.clone() {
            Adt2::C0(_, _, _, h3655, _, _, _, _, _, _, _) => {
                h3655
            }
        };
        nat_eq(a3656, a3657)
    } else {
        false
    } {
        let a3660 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3658, _, _, _, _, _, _) => {
                h3658
            }
        };
        let a3661 = match v4.clone() {
            Adt2::C0(_, _, _, _, h3659, _, _, _, _, _, _) => {
                h3659
            }
        };
        nat_eq(a3660, a3661)
    } else {
        false
    } {
        let a3666 = {
            let a3664 = match v5.clone() {
                Adt6::C0(h3662, _, _, _) => {
                    h3662
                }
            };
            let a3665 = match v6.clone() {
                Adt6::C0(h3663, _, _, _) => {
                    h3663
                }
            };
            nat_mul(a3664, a3665)?
        };
        let a3667 = 2147483648u64;
        nat_lt(a3666, a3667)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a3669 = match v2 {
                Adt1::C0(h3668, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3668
                }
            };
            let a3670 = 0u64;
            nat_eq(a3669, a3670)
        }, match v3.clone() {
            Adt2::C0(h3671, _, _, _, _, _, _, _, _, _, _) => {
                h3671
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a3683 = f23(v3.clone(), 1u64)?;
                let a3684 = {
                    let a3681 = f23(v4.clone(), 1u64)?;
                    let a3682 = f174(match v7.clone() {
                        Adt7::C0(_, _, _, h3672, _) => {
                            h3672
                        }
                    }, {
                        let a3679 = {
                            let m3675 = match v7.clone() {
                                Adt7::C0(h3674, _, _, _, _) => {
                                    h3674
                                }
                            };
                            match m3675 {
                                Adt6::C0(h3673, _, _, _) => {
                                    h3673
                                }
                            }
                        };
                        let a3680 = {
                            let m3678 = match v7.clone() {
                                Adt7::C0(_, h3677, _, _, _) => {
                                    h3677
                                }
                            };
                            match m3678 {
                                Adt6::C0(h3676, _, _, _) => {
                                    h3676
                                }
                            }
                        };
                        nat_mul(a3679, a3680)?
                    })?;
                    nat_mul(a3681, a3682)?
                };
                nat_add(a3683, a3684)?
            }, 4u64, f25(v4, 1u64), f25(v3.clone(), 1u64), Adt4::C43(v7, f23(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a3685 = f23(v4.clone(), 1u64)?;
                let a3686 = f190(v7.clone())?;
                nat_mul(a3685, a3686)?
            }, 4u64, f25(v3, 1u64), f25(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f118(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3687 = v3;
    match m3687 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3689 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3688, _, _, _, _, _) => {
                        h3688
                    }
                };
                let a3690 = 1u64;
                nat_eq(a3689, a3690)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h3691, _, _, _, _, _, _, _, _, _) => {
                    h3691
                }
            }, match v1.clone() {
                Adt1::C0(h3692, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3692
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3693, _, _, _, _, _, _, _, _, _) => {
                    h3693
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h3694, _, _, _, _, _, _, _) => {
                    h3694
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h3695, _, _, _, _, _, _, _, _) => {
                        h3695
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h3696, _, _, _, _, _, _, _, _, _, _) => {
                        h3696
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3697, _, _, _, _, _, _, _, _) => {
                        h3697
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3698, _, _, _, _, _, _) => {
                        h3698
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3699, _, _, _, _, _, _, _) => {
                        h3699
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h3700, _, _, _, _, _, _, _, _) => {
                        h3700
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h3701, _, _, _, _, _, _) => {
                        h3701
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3702, _, _, _, _, _, _, _) => {
                        h3702
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h3703, _, _, _, _, _, _, _, _) => {
                        h3703
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h3704, _, _, _, _, _, _, _, _, _, _) => {
                    h3704
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h3705, _, _, _, _, _, _, _, _, _, _) => {
                    h3705
                }
            };
            let v15: u64 = {
                let a3706 = v13;
                let a3707 = 0u64;
                if nat_eq(a3706, a3707) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a3709 = match v5.clone() {
                    Adt2::C0(_, h3708, _, _, _, _, _, _, _, _, _) => {
                        h3708
                    }
                };
                let a3710 = v15;
                nat_mul(a3709, a3710)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3711, _, _, _) => {
                        h3711
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3712, _, _, _, _) => {
                        h3712
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3713, _, _) => {
                        h3713
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3714, _, _, _) => {
                        h3714
                    }
                }
            };
            let a3716 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h3715, _, _, _, _, _) => {
                    h3715
                }
            };
            let a3717 = 1u64;
            if if if if if if if if if if nat_le(a3716, a3717) {
                let a3718 = v13;
                let a3719 = 0u64;
                if nat_eq(a3718, a3719) {
                    true
                } else {
                    let a3720 = v13;
                    let a3721 = 1u64;
                    if nat_eq(a3720, a3721) {
                        let a3722 = v14;
                        let a3723 = 1u64;
                        nat_eq(a3722, a3723)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3724 = v14;
                let a3725 = 0u64;
                if nat_eq(a3724, a3725) {
                    true
                } else {
                    let a3726 = v14;
                    let a3727 = 1u64;
                    nat_eq(a3726, a3727)
                }
            } else {
                false
            } {
                let a3729 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3728, _, _, _, _, _) => {
                        h3728
                    }
                };
                let a3730 = v15;
                nat_eq(a3729, a3730)
            } else {
                false
            } {
                let a3738 = match v4.clone() {
                    Adt2::C0(_, h3731, _, _, _, _, _, _, _, _, _) => {
                        h3731
                    }
                };
                let a3739 = {
                    let a3736 = v10;
                    let a3737 = {
                        let a3734 = match v7.clone() {
                            Adt6::C0(h3732, _, _, _) => {
                                h3732
                            }
                        };
                        let a3735 = match v8.clone() {
                            Adt6::C0(h3733, _, _, _) => {
                                h3733
                            }
                        };
                        nat_mul(a3734, a3735)?
                    };
                    nat_mul(a3736, a3737)?
                };
                nat_eq(a3738, a3739)
            } else {
                false
            } {
                let a3742 = match v4.clone() {
                    Adt2::C0(_, _, h3740, _, _, _, _, _, _, _, _) => {
                        h3740
                    }
                };
                let a3743 = f94(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h3741, _, _, _, _, _, _, _, _, _) => {
                        h3741
                    }
                })?;
                nat_eq(a3742, a3743)
            } else {
                false
            } {
                let a3745 = 0u64;
                let a3746 = match v4.clone() {
                    Adt2::C0(_, _, h3744, _, _, _, _, _, _, _, _) => {
                        h3744
                    }
                };
                nat_lt(a3745, a3746)
            } else {
                false
            } {
                if v6 {
                    let a3748 = match v2 {
                        Adt2::C0(_, _, _, h3747, _, _, _, _, _, _, _) => {
                            h3747
                        }
                    };
                    let a3749 = v10;
                    if if if nat_eq(a3748, a3749) {
                        let a3751 = match v4.clone() {
                            Adt2::C0(_, _, _, h3750, _, _, _, _, _, _, _) => {
                                h3750
                            }
                        };
                        let a3752 = f94(v8.clone(), v12)?;
                        nat_eq(a3751, a3752)
                    } else {
                        false
                    } {
                        let a3754 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3753, _, _, _, _, _, _) => {
                                h3753
                            }
                        };
                        let a3755 = v11;
                        nat_eq(a3754, a3755)
                    } else {
                        false
                    } {
                        let a3757 = 0u64;
                        let a3758 = match v4.clone() {
                            Adt2::C0(_, _, _, h3756, _, _, _, _, _, _, _) => {
                                h3756
                            }
                        };
                        nat_lt(a3757, a3758)
                    } else {
                        false
                    }
                } else {
                    let a3760 = match v2 {
                        Adt2::C0(_, _, h3759, _, _, _, _, _, _, _, _) => {
                            h3759
                        }
                    };
                    let a3761 = v10;
                    if if if nat_eq(a3760, a3761) {
                        let a3763 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h3762, _, _, _, _, _, _) => {
                                h3762
                            }
                        };
                        let a3764 = 1u64;
                        nat_eq(a3763, a3764)
                    } else {
                        false
                    } {
                        let a3766 = match v4.clone() {
                            Adt2::C0(_, _, _, h3765, _, _, _, _, _, _, _) => {
                                h3765
                            }
                        };
                        let a3767 = v11;
                        nat_eq(a3766, a3767)
                    } else {
                        false
                    } {
                        let a3769 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3768, _, _, _, _, _, _) => {
                                h3768
                            }
                        };
                        let a3770 = 1u64;
                        nat_eq(a3769, a3770)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f48(match v5.clone() {
                    Adt2::C0(_, h3771, _, _, _, _, _, _, _, _, _) => {
                        h3771
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a3791 = {
                        let a3788 = {
                            let a3786 = {
                                let a3780 = {
                                    let a3774 = {
                                        let a3772 = v11;
                                        let a3773 = 1u64;
                                        nat_sub(a3772, a3773)
                                    };
                                    let a3775 = v18;
                                    nat_mul(a3774, a3775)?
                                };
                                let a3781 = {
                                    let a3778 = {
                                        let a3776 = v10;
                                        let a3777 = 1u64;
                                        nat_sub(a3776, a3777)
                                    };
                                    let a3779 = v17;
                                    nat_mul(a3778, a3779)?
                                };
                                nat_add(a3780, a3781)?
                            };
                            let a3787 = {
                                let a3784 = {
                                    let a3782 = v12;
                                    let a3783 = 1u64;
                                    nat_sub(a3782, a3783)
                                };
                                let a3785 = v16;
                                nat_mul(a3784, a3785)?
                            };
                            nat_add(a3786, a3787)?
                        };
                        let a3789 = v16;
                        nat_add(a3788, a3789)?
                    };
                    let a3792 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3790, _) => {
                            h3790
                        }
                    };
                    nat_le(a3791, a3792)
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
                        Adt2::C0(_, h3793, _, _, _, _, _, _, _, _, _) => {
                            h3793
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3794, _, _, _, _, _, _, _, _) => {
                            h3794
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h3795, _, _, _, _, _, _, _) => {
                            h3795
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h3796, _, _, _, _, _) => {
                            h3796
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3797, _, _, _, _) => {
                            h3797
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3798, _, _, _) => {
                            h3798
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a3805 = f23(v5.clone(), 1u64)?;
                        let a3806 = {
                            let a3803 = {
                                let a3801 = match v7.clone() {
                                    Adt6::C0(h3799, _, _, _) => {
                                        h3799
                                    }
                                };
                                let a3802 = match v8.clone() {
                                    Adt6::C0(h3800, _, _, _) => {
                                        h3800
                                    }
                                };
                                nat_mul(a3801, a3802)?
                            };
                            let a3804 = 2u64;
                            nat_add(a3803, a3804)?
                        };
                        nat_mul(a3805, a3806)?
                    }, 4u64, v19, f37(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f23(v4, 1u64)?, {
                        let a3807 = v14;
                        let a3808 = 0u64;
                        if nat_eq(a3807, a3808) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h3809, _, _, _, _, _, _, _, _, _) => {
                            h3809
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f119(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3810 = v2;
    match m3810 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3811, _, _, _, _, _, _, _, _, _) => {
                    h3811
                }
            }, match v0.clone() {
                Adt1::C0(h3812, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3812
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3813, _, _, _, _, _, _, _, _) => {
                    h3813
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3814, _, _, _, _, _) => {
                    h3814
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3815, _, _, _, _, _, _, _, _) => {
                    h3815
                }
            }, match v0.clone() {
                Adt1::C0(_, h3816, _, _, _, _, _, _, _, _, _, _) => {
                    h3816
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3817, _, _, _, _, _, _, _) => {
                    h3817
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3818, _, _, _, _) => {
                    h3818
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3819, _, _, _, _, _, _, _) => {
                    h3819
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3820, _, _, _, _, _, _, _, _, _) => {
                    h3820
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3821, _, _, _, _, _, _) => {
                    h3821
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3822, _, _, _) => {
                    h3822
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3823, _, _) => {
                    h3823
                }
            };
            let v9: u64 = f94(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3824, _, _, _, _, _, _, _) => {
                    h3824
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3825, _, _, _, _, _, _, _, _, _, _) => {
                    h3825
                }
            };
            let v11: u64 = {
                let a3833 = v8;
                let a3834 = {
                    let a3831 = match v5.clone() {
                        Adt6::C0(h3826, _, _, _) => {
                            h3826
                        }
                    };
                    let a3832 = {
                        let a3829 = match v6.clone() {
                            Adt6::C0(h3827, _, _, _) => {
                                h3827
                            }
                        };
                        let a3830 = match v7.clone() {
                            Adt6::C0(h3828, _, _, _) => {
                                h3828
                            }
                        };
                        nat_mul(a3829, a3830)?
                    };
                    nat_mul(a3831, a3832)?
                };
                nat_mul(a3833, a3834)?
            };
            let a3836 = match v4.clone() {
                Adt2::C0(h3835, _, _, _, _, _, _, _, _, _, _) => {
                    h3835
                }
            };
            let a3837 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3836, a3837) {
                let a3838 = v10;
                let a3839 = 0u64;
                if nat_eq(a3838, a3839) {
                    true
                } else {
                    let a3840 = v10;
                    let a3841 = 1u64;
                    nat_eq(a3840, a3841)
                }
            } else {
                false
            } {
                let a3842 = 0u64;
                let a3843 = v8;
                nat_lt(a3842, a3843)
            } else {
                false
            } {
                let a3844 = v8;
                let a3845 = 2147483648u64;
                nat_lt(a3844, a3845)
            } else {
                false
            } {
                let a3847 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3846, _, _, _, _, _) => {
                        h3846
                    }
                };
                let a3848 = 4u64;
                nat_eq(a3847, a3848)
            } else {
                false
            } {
                let a3849 = 0u64;
                let a3850 = v9;
                nat_lt(a3849, a3850)
            } else {
                false
            } {
                let a3852 = 0u64;
                let a3853 = match v3.clone() {
                    Adt2::C0(_, _, h3851, _, _, _, _, _, _, _, _) => {
                        h3851
                    }
                };
                nat_lt(a3852, a3853)
            } else {
                false
            } {
                let a3855 = 0u64;
                let a3856 = match v3.clone() {
                    Adt2::C0(_, _, _, h3854, _, _, _, _, _, _, _) => {
                        h3854
                    }
                };
                nat_lt(a3855, a3856)
            } else {
                false
            } {
                let a3858 = match v3.clone() {
                    Adt2::C0(_, h3857, _, _, _, _, _, _, _, _, _) => {
                        h3857
                    }
                };
                let a3859 = v11;
                nat_eq(a3858, a3859)
            } else {
                false
            } {
                let a3862 = match v3.clone() {
                    Adt2::C0(_, _, h3860, _, _, _, _, _, _, _, _) => {
                        h3860
                    }
                };
                let a3863 = f94(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3861, _, _, _, _, _, _, _, _, _) => {
                        h3861
                    }
                })?;
                nat_eq(a3862, a3863)
            } else {
                false
            } {
                let a3866 = match v3.clone() {
                    Adt2::C0(_, _, _, h3864, _, _, _, _, _, _, _) => {
                        h3864
                    }
                };
                let a3867 = f94(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3865, _, _, _, _, _, _, _, _) => {
                        h3865
                    }
                })?;
                nat_eq(a3866, a3867)
            } else {
                false
            } {
                let a3875 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3868, _, _, _, _, _, _) => {
                        h3868
                    }
                };
                let a3876 = {
                    let a3873 = v9;
                    let a3874 = {
                        let a3870 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3869, _, _, _, _, _, _) => {
                                h3869
                            }
                        };
                        let a3871 = v8;
                        let a3872 = 0u64;
                        nat_quot(a3870, a3871, a3872)
                    };
                    nat_mul(a3873, a3874)?
                };
                nat_eq(a3875, a3876)
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3877 = v10;
                    let a3878 = 0u64;
                    if nat_eq(a3877, a3878) {
                        4u64
                    } else {
                        2u64
                    }
                }, f25(v4.clone(), 1u64), f25(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3879, _, _) => {
                        h3879
                    }
                }, {
                    let a3881 = v8;
                    let a3882 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3880, _, _) => {
                            h3880
                        }
                    };
                    nat_mul(a3881, a3882)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f120(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3883 = v1;
    match m3883 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f224(v0, v3, v2)
        }
    }
}

pub fn f121(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3885 = match v0.clone() {
        Adt2::C0(h3884, _, _, _, _, _, _, _, _, _, _) => {
            h3884
        }
    };
    let a3886 = 1u64;
    if if nat_eq(a3885, a3886) {
        true
    } else {
        let a3888 = match v0.clone() {
            Adt2::C0(h3887, _, _, _, _, _, _, _, _, _, _) => {
                h3887
            }
        };
        let a3889 = 30u64;
        nat_eq(a3888, a3889)
    } {
        let a3891 = match v1.clone() {
            Adt2::C0(h3890, _, _, _, _, _, _, _, _, _, _) => {
                h3890
            }
        };
        let a3892 = 1u64;
        if if if if if if if if if if if nat_eq(a3891, a3892) {
            let a3895 = match v0.clone() {
                Adt2::C0(_, h3893, _, _, _, _, _, _, _, _, _) => {
                    h3893
                }
            };
            let a3896 = match v1.clone() {
                Adt2::C0(_, h3894, _, _, _, _, _, _, _, _, _) => {
                    h3894
                }
            };
            nat_eq(a3895, a3896)
        } else {
            false
        } {
            let a3898 = 0u64;
            let a3899 = match v1.clone() {
                Adt2::C0(_, _, h3897, _, _, _, _, _, _, _, _) => {
                    h3897
                }
            };
            nat_lt(a3898, a3899)
        } else {
            false
        } {
            let a3902 = match v1.clone() {
                Adt2::C0(_, _, h3900, _, _, _, _, _, _, _, _) => {
                    h3900
                }
            };
            let a3903 = match v1.clone() {
                Adt2::C0(_, _, _, h3901, _, _, _, _, _, _, _) => {
                    h3901
                }
            };
            nat_eq(a3902, a3903)
        } else {
            false
        } {
            let a3905 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3904, _, _, _, _, _, _) => {
                    h3904
                }
            };
            let a3906 = 1u64;
            nat_eq(a3905, a3906)
        } else {
            false
        } {
            let a3914 = match v0.clone() {
                Adt2::C0(_, _, h3907, _, _, _, _, _, _, _, _) => {
                    h3907
                }
            };
            let a3915 = {
                let a3912 = {
                    let a3910 = match v1.clone() {
                        Adt2::C0(_, _, h3908, _, _, _, _, _, _, _, _) => {
                            h3908
                        }
                    };
                    let a3911 = match v1.clone() {
                        Adt2::C0(_, _, h3909, _, _, _, _, _, _, _, _) => {
                            h3909
                        }
                    };
                    nat_add(a3910, a3911)?
                };
                let a3913 = 1u64;
                nat_sub(a3912, a3913)
            };
            nat_eq(a3914, a3915)
        } else {
            false
        } {
            let a3917 = 0u64;
            let a3918 = match v0.clone() {
                Adt2::C0(_, _, _, h3916, _, _, _, _, _, _, _) => {
                    h3916
                }
            };
            nat_lt(a3917, a3918)
        } else {
            false
        } {
            let a3920 = 0u64;
            let a3921 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3919, _, _, _, _, _, _) => {
                    h3919
                }
            };
            nat_lt(a3920, a3921)
        } else {
            false
        } {
            let a3923 = 2u64;
            let a3924 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3922, _, _, _, _, _) => {
                    h3922
                }
            };
            nat_le(a3923, a3924)
        } else {
            false
        } {
            let a3926 = 2u64;
            let a3927 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3925, _, _, _, _, _) => {
                    h3925
                }
            };
            nat_le(a3926, a3927)
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

pub fn f122(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3928 = v1;
    match m3928 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f225(v0, v3, v2)
        }
    }
}

pub fn f123(v0: u64) -> bool {
    let a3929 = v0;
    let a3930 = 1u64;
    if if nat_eq(a3929, a3930) {
        true
    } else {
        let a3931 = v0;
        let a3932 = 33u64;
        nat_eq(a3931, a3932)
    } {
        true
    } else {
        let a3933 = v0;
        let a3934 = 34u64;
        nat_eq(a3933, a3934)
    }
}

pub fn f124(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f47(v1, v2.clone()) {
        let a3935 = v3;
        let a3936 = f23(v2.clone(), 1u64)?;
        nat_eq(a3935, a3936)
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
        let a3937 = v3;
        let a3938 = 0u64;
        if nat_eq(a3937, a3938) {
            true
        } else {
            let a3940 = 4u64;
            let a3941 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3939, _, _, _, _, _) => {
                    h3939
                }
            };
            nat_le(a3940, a3941)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3942 = 2u64;
            let a3943 = v3;
            nat_mul(a3942, a3943)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f125(v0: u64) -> bool {
    if f226(v0) {
        !f227(v0)
    } else {
        false
    }
}

pub fn f126(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3944 = v1;
    match m3944 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3946 = f218(match v0.clone() {
                Adt2::C0(h3945, _, _, _, _, _, _, _, _, _, _) => {
                    h3945
                }
            });
            match m3946 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3947, _) => {
                            h3947
                        }
                    };
                    if if if if if if f47(v3, v2.clone()) {
                        f30(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3948 = f23(v0.clone(), 1u64)?;
                        let a3949 = f23(v2.clone(), 1u64)?;
                        nat_eq(a3948, a3949)
                    } else {
                        false
                    } {
                        let a3954 = {
                            let a3951 = match v0.clone() {
                                Adt2::C0(_, h3950, _, _, _, _, _, _, _, _, _) => {
                                    h3950
                                }
                            };
                            let a3952 = v5;
                            let a3953 = 1u64;
                            nat_rem(a3951, a3952, a3953)
                        };
                        let a3955 = 0u64;
                        if nat_eq(a3954, a3955) {
                            let a3958 = match v4 {
                                Adt19::C0(_, _, h3956) => {
                                    h3956
                                }
                            };
                            let a3959 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3957, _, _, _, _, _) => {
                                    h3957
                                }
                            };
                            nat_le(a3958, a3959)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3960 = v5;
                        let a3961 = 1u64;
                        if nat_eq(a3960, a3961) {
                            let a3963 = 4u64;
                            let a3964 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3962, _, _, _, _, _) => {
                                    h3962
                                }
                            };
                            nat_le(a3963, a3964)
                        } else {
                            let a3966 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3965, _, _, _, _, _) => {
                                    h3965
                                }
                            };
                            let a3967 = 4u64;
                            nat_eq(a3966, a3967)
                        }
                    } else {
                        false
                    } {
                        let a3972 = {
                            let a3969 = match v2.clone() {
                                Adt2::C0(_, h3968, _, _, _, _, _, _, _, _, _) => {
                                    h3968
                                }
                            };
                            let a3970 = v5;
                            let a3971 = 1u64;
                            nat_rem(a3969, a3970, a3971)
                        };
                        let a3973 = 0u64;
                        if nat_eq(a3972, a3973) {
                            true
                        } else {
                            f38(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f25(v0.clone(), v5), f25(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3974, _, _, _, _, _, _, _, _, _, _) => {
                                h3974
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

pub fn f127(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3977 = match v1.clone() {
        Adt2::C0(h3975, _, _, _, _, _, _, _, _, _, _) => {
            h3975
        }
    };
    let a3978 = match v3.clone() {
        Adt2::C0(h3976, _, _, _, _, _, _, _, _, _, _) => {
            h3976
        }
    };
    if nat_eq(a3977, a3978) {
        f228(v0, v1, v2, v3)
    } else {
        if if if if f227(match v1.clone() {
            Adt2::C0(h3979, _, _, _, _, _, _, _, _, _, _) => {
                h3979
            }
        }) {
            f227(match v3.clone() {
                Adt2::C0(h3980, _, _, _, _, _, _, _, _, _, _) => {
                    h3980
                }
            })
        } else {
            false
        } {
            let a3981 = v0;
            let a3982 = 33u64;
            if nat_eq(a3981, a3982) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3983 = f23(v1.clone(), 1u64)?;
            let a3984 = f23(v3.clone(), 1u64)?;
            nat_eq(a3983, a3984)
        } else {
            false
        } {
            let m3986 = f218(match v3.clone() {
                Adt2::C0(h3985, _, _, _, _, _, _, _, _, _, _) => {
                    h3985
                }
            });
            match m3986 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3987, _, _, _, _, _, _, _, _, _, _) => {
                            h3987
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3988, _, _, _, _, _, _, _, _, _, _) => {
                            h3988
                        }
                    };
                    let v7: bool = {
                        let a3989 = v5;
                        let a3990 = 43u64;
                        if nat_eq(a3989, a3990) {
                            true
                        } else {
                            let a3991 = v6;
                            let a3992 = 43u64;
                            nat_eq(a3991, a3992)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3993 = f23(v3.clone(), 1u64)?;
                        let a3994 = if v7 {
                            f168(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3993, a3994)?
                    }, match v4 {
                        Adt19::C0(_, _, h3995) => {
                            h3995
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

pub fn f128(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3997 = match v0.clone() {
        Adt2::C0(h3996, _, _, _, _, _, _, _, _, _, _) => {
            h3996
        }
    };
    let a3998 = 0u64;
    if if if if nat_eq(a3997, a3998) {
        true
    } else {
        let a4000 = match v0.clone() {
            Adt2::C0(h3999, _, _, _, _, _, _, _, _, _, _) => {
                h3999
            }
        };
        let a4001 = 1u64;
        nat_eq(a4000, a4001)
    } {
        true
    } else {
        let a4003 = match v0.clone() {
            Adt2::C0(h4002, _, _, _, _, _, _, _, _, _, _) => {
                h4002
            }
        };
        let a4004 = 25u64;
        if nat_eq(a4003, a4004) {
            true
        } else {
            let a4006 = match v0.clone() {
                Adt2::C0(h4005, _, _, _, _, _, _, _, _, _, _) => {
                    h4005
                }
            };
            let a4007 = 26u64;
            nat_eq(a4006, a4007)
        }
    } {
        true
    } else {
        let a4009 = match v0.clone() {
            Adt2::C0(h4008, _, _, _, _, _, _, _, _, _, _) => {
                h4008
            }
        };
        let a4010 = 30u64;
        nat_eq(a4009, a4010)
    } {
        let a4013 = match v0.clone() {
            Adt2::C0(h4011, _, _, _, _, _, _, _, _, _, _) => {
                h4011
            }
        };
        let a4014 = match v1.clone() {
            Adt2::C0(h4012, _, _, _, _, _, _, _, _, _, _) => {
                h4012
            }
        };
        if if nat_eq(a4013, a4014) {
            f229(match v1.clone() {
                Adt2::C0(_, h4015, _, _, _, _, _, _, _, _, _) => {
                    h4015
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h4016, _, _, _, _, _, _, _, _) => {
                    h4016
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h4017, _, _, _, _, _, _, _) => {
                    h4017
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h4018, _, _, _, _, _, _) => {
                    h4018
                }
            }, match v0.clone() {
                Adt2::C0(_, h4019, _, _, _, _, _, _, _, _, _) => {
                    h4019
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h4020, _, _, _, _, _, _, _, _) => {
                    h4020
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h4021, _, _, _, _, _, _, _) => {
                    h4021
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h4022, _, _, _, _, _, _) => {
                    h4022
                }
            })
        } else {
            false
        } {
            let m4024 = f218(match v0.clone() {
                Adt2::C0(h4023, _, _, _, _, _, _, _, _, _, _) => {
                    h4023
                }
            });
            match m4024 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h4025) => {
                            h4025
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

pub fn f129(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4026 = v2;
    match m4026 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a4027 = v0;
            let a4028 = 4u64;
            if if if nat_lt(a4027, a4028) {
                let a4031 = match v1.clone() {
                    Adt2::C0(h4029, _, _, _, _, _, _, _, _, _, _) => {
                        h4029
                    }
                };
                let a4032 = match v4.clone() {
                    Adt2::C0(h4030, _, _, _, _, _, _, _, _, _, _) => {
                        h4030
                    }
                };
                if nat_eq(a4031, a4032) {
                    let a4035 = match v1.clone() {
                        Adt2::C0(h4033, _, _, _, _, _, _, _, _, _, _) => {
                            h4033
                        }
                    };
                    let a4036 = match v3.clone() {
                        Adt2::C0(h4034, _, _, _, _, _, _, _, _, _, _) => {
                            h4034
                        }
                    };
                    nat_eq(a4035, a4036)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f230(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h4037, _, _, _, _, _, _, _, _, _) => {
                        h4037
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4038, _, _, _, _, _, _, _, _, _) => {
                        h4038
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h4039, _, _, _, _, _, _, _, _, _) => {
                        h4039
                    }
                })? {
                    f230(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h4040, _, _, _, _, _, _, _, _) => {
                            h4040
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4041, _, _, _, _, _, _, _, _) => {
                            h4041
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h4042, _, _, _, _, _, _, _, _) => {
                            h4042
                        }
                    })?
                } else {
                    false
                } {
                    f230(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h4043, _, _, _, _, _, _, _) => {
                            h4043
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h4044, _, _, _, _, _, _, _) => {
                            h4044
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h4045, _, _, _, _, _, _, _) => {
                            h4045
                        }
                    })?
                } else {
                    false
                } {
                    f230(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h4046, _, _, _, _, _, _) => {
                            h4046
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h4047, _, _, _, _, _, _) => {
                            h4047
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h4048, _, _, _, _, _, _) => {
                            h4048
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m4050 = f218(match v1.clone() {
                    Adt2::C0(h4049, _, _, _, _, _, _, _, _, _, _) => {
                        h4049
                    }
                });
                match m4050 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h4051, _) => {
                                h4051
                            }
                        };
                        let a4056 = {
                            let a4053 = match v1.clone() {
                                Adt2::C0(_, h4052, _, _, _, _, _, _, _, _, _) => {
                                    h4052
                                }
                            };
                            let a4054 = v6;
                            let a4055 = 1u64;
                            nat_rem(a4053, a4054, a4055)
                        };
                        let a4057 = 0u64;
                        if if if nat_eq(a4056, a4057) {
                            let a4062 = {
                                let a4059 = match v4.clone() {
                                    Adt2::C0(_, h4058, _, _, _, _, _, _, _, _, _) => {
                                        h4058
                                    }
                                };
                                let a4060 = v6;
                                let a4061 = 1u64;
                                nat_rem(a4059, a4060, a4061)
                            };
                            let a4063 = 0u64;
                            nat_eq(a4062, a4063)
                        } else {
                            false
                        } {
                            let a4068 = {
                                let a4065 = match v3.clone() {
                                    Adt2::C0(_, h4064, _, _, _, _, _, _, _, _, _) => {
                                        h4064
                                    }
                                };
                                let a4066 = v6;
                                let a4067 = 1u64;
                                nat_rem(a4065, a4066, a4067)
                            };
                            let a4069 = 0u64;
                            nat_eq(a4068, a4069)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h4070) => {
                                    h4070
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

pub fn f130(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4072 = match v2.clone() {
        Adt2::C0(h4071, _, _, _, _, _, _, _, _, _, _) => {
            h4071
        }
    };
    let a4073 = 0u64;
    if if nat_eq(a4072, a4073) {
        let a4075 = match v3.clone() {
            Adt2::C0(h4074, _, _, _, _, _, _, _, _, _, _) => {
                h4074
            }
        };
        let a4076 = 0u64;
        nat_eq(a4075, a4076)
    } else {
        false
    } {
        let a4077 = v0;
        let a4078 = 61u64;
        if nat_eq(a4077, a4078) {
            let v4: bool = {
                let a4080 = 0u64;
                let a4081 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4079, _, _, _) => {
                        h4079
                    }
                };
                nat_lt(a4080, a4081)
            };
            if if if if if if f231(match v2.clone() {
                Adt2::C0(_, h4082, _, _, _, _, _, _, _, _, _) => {
                    h4082
                }
            }, match v1.clone() {
                Adt1::C0(h4083, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4083
                }
            }, match v1.clone() {
                Adt1::C0(_, h4084, _, _, _, _, _, _, _, _, _, _) => {
                    h4084
                }
            }, match v3.clone() {
                Adt2::C0(_, h4085, _, _, _, _, _, _, _, _, _) => {
                    h4085
                }
            })? {
                f231(match v2.clone() {
                    Adt2::C0(_, _, h4086, _, _, _, _, _, _, _, _) => {
                        h4086
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4087, _, _, _, _, _, _, _, _, _) => {
                        h4087
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4088, _, _, _, _, _, _, _, _) => {
                        h4088
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h4089, _, _, _, _, _, _, _, _) => {
                        h4089
                    }
                })?
            } else {
                false
            } {
                f231(match v2.clone() {
                    Adt2::C0(_, _, _, h4090, _, _, _, _, _, _, _) => {
                        h4090
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4091, _, _, _, _, _, _, _) => {
                        h4091
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4092, _, _, _, _, _, _) => {
                        h4092
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h4093, _, _, _, _, _, _, _) => {
                        h4093
                    }
                })?
            } else {
                false
            } {
                f231(match v2.clone() {
                    Adt2::C0(_, _, _, _, h4094, _, _, _, _, _, _) => {
                        h4094
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4095, _, _, _, _, _) => {
                        h4095
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h4096, _, _, _, _) => {
                        h4096
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h4097, _, _, _, _, _, _) => {
                        h4097
                    }
                })?
            } else {
                false
            } {
                let a4099 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4098, _, _, _) => {
                        h4098
                    }
                };
                let a4100 = 4294967295u64;
                nat_le(a4099, a4100)
            } else {
                false
            } {
                if v4 {
                    !f48(match v2.clone() {
                        Adt2::C0(_, h4101, _, _, _, _, _, _, _, _, _) => {
                            h4101
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h4102, _, _, _, _, _, _, _, _) => {
                            h4102
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h4103, _, _, _, _, _, _, _) => {
                            h4103
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h4104, _, _, _, _, _, _) => {
                            h4104
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h4105, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4105
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4106, _, _, _, _, _, _, _, _, _) => {
                        h4106
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4107, _, _, _, _, _, _, _) => {
                        h4107
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h4108, _, _, _, _, _) => {
                        h4108
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a4109 = v0;
            let a4110 = 62u64;
            if nat_eq(a4109, a4110) {
                let a4112 = match v1.clone() {
                    Adt1::C0(h4111, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4111
                    }
                };
                let a4113 = 2147483648u64;
                if if if if if nat_lt(a4112, a4113) {
                    let a4115 = match v1.clone() {
                        Adt1::C0(_, h4114, _, _, _, _, _, _, _, _, _, _) => {
                            h4114
                        }
                    };
                    let a4116 = 2147483648u64;
                    nat_lt(a4115, a4116)
                } else {
                    false
                } {
                    let a4119 = match v1.clone() {
                        Adt1::C0(h4117, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4117
                        }
                    };
                    let a4120 = match v2.clone() {
                        Adt2::C0(_, h4118, _, _, _, _, _, _, _, _, _) => {
                            h4118
                        }
                    };
                    nat_lt(a4119, a4120)
                } else {
                    false
                } {
                    let a4123 = match v1.clone() {
                        Adt1::C0(_, h4121, _, _, _, _, _, _, _, _, _, _) => {
                            h4121
                        }
                    };
                    let a4124 = match v2.clone() {
                        Adt2::C0(_, h4122, _, _, _, _, _, _, _, _, _) => {
                            h4122
                        }
                    };
                    nat_lt(a4123, a4124)
                } else {
                    false
                } {
                    let a4127 = match v2.clone() {
                        Adt2::C0(_, _, h4125, _, _, _, _, _, _, _, _) => {
                            h4125
                        }
                    };
                    let a4128 = match v3.clone() {
                        Adt2::C0(_, _, h4126, _, _, _, _, _, _, _, _) => {
                            h4126
                        }
                    };
                    if if if nat_eq(a4127, a4128) {
                        let a4131 = match v2.clone() {
                            Adt2::C0(_, _, _, h4129, _, _, _, _, _, _, _) => {
                                h4129
                            }
                        };
                        let a4132 = match v3.clone() {
                            Adt2::C0(_, _, _, h4130, _, _, _, _, _, _, _) => {
                                h4130
                            }
                        };
                        nat_eq(a4131, a4132)
                    } else {
                        false
                    } {
                        let a4135 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4133, _, _, _, _, _, _) => {
                                h4133
                            }
                        };
                        let a4136 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4134, _, _, _, _, _, _) => {
                                h4134
                            }
                        };
                        nat_eq(a4135, a4136)
                    } else {
                        false
                    } {
                        let a4145 = match v3.clone() {
                            Adt2::C0(_, h4137, _, _, _, _, _, _, _, _, _) => {
                                h4137
                            }
                        };
                        let a4146 = {
                            let a4143 = {
                                let a4140 = match v2.clone() {
                                    Adt2::C0(_, h4138, _, _, _, _, _, _, _, _, _) => {
                                        h4138
                                    }
                                };
                                let a4141 = match v1.clone() {
                                    Adt1::C0(h4139, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4139
                                    }
                                };
                                nat_add(a4140, a4141)?
                            };
                            let a4144 = match v1.clone() {
                                Adt1::C0(_, h4142, _, _, _, _, _, _, _, _, _, _) => {
                                    h4142
                                }
                            };
                            nat_add(a4143, a4144)?
                        };
                        nat_eq(a4145, a4146)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4147, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4147
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4148 = v0;
                let a4149 = 63u64;
                if nat_eq(a4148, a4149) {
                    if if if if if f232(match v2.clone() {
                        Adt2::C0(_, h4150, _, _, _, _, _, _, _, _, _) => {
                            h4150
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4151, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4151
                        }
                    })? {
                        f232(match v2.clone() {
                            Adt2::C0(_, _, h4152, _, _, _, _, _, _, _, _) => {
                                h4152
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4153, _, _, _, _, _, _, _, _, _, _) => {
                                h4153
                            }
                        })?
                    } else {
                        false
                    } {
                        f232(match v2.clone() {
                            Adt2::C0(_, _, _, h4154, _, _, _, _, _, _, _) => {
                                h4154
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4155, _, _, _, _, _, _, _, _, _) => {
                                h4155
                            }
                        })?
                    } else {
                        false
                    } {
                        f232(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4156, _, _, _, _, _, _) => {
                                h4156
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4157, _, _, _, _, _, _, _, _) => {
                                h4157
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4160 = match v2.clone() {
                            Adt2::C0(_, h4158, _, _, _, _, _, _, _, _, _) => {
                                h4158
                            }
                        };
                        let a4161 = match v3.clone() {
                            Adt2::C0(_, h4159, _, _, _, _, _, _, _, _, _) => {
                                h4159
                            }
                        };
                        if if if nat_eq(a4160, a4161) {
                            let a4164 = match v2.clone() {
                                Adt2::C0(_, _, h4162, _, _, _, _, _, _, _, _) => {
                                    h4162
                                }
                            };
                            let a4165 = match v3.clone() {
                                Adt2::C0(_, _, h4163, _, _, _, _, _, _, _, _) => {
                                    h4163
                                }
                            };
                            nat_eq(a4164, a4165)
                        } else {
                            false
                        } {
                            let a4168 = match v2.clone() {
                                Adt2::C0(_, _, _, h4166, _, _, _, _, _, _, _) => {
                                    h4166
                                }
                            };
                            let a4169 = match v3.clone() {
                                Adt2::C0(_, _, _, h4167, _, _, _, _, _, _, _) => {
                                    h4167
                                }
                            };
                            nat_eq(a4168, a4169)
                        } else {
                            false
                        } {
                            let a4172 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4170, _, _, _, _, _, _) => {
                                    h4170
                                }
                            };
                            let a4173 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4171, _, _, _, _, _, _) => {
                                    h4171
                                }
                            };
                            nat_eq(a4172, a4173)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4174, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4174
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4175, _, _, _, _, _, _, _, _, _, _) => {
                                h4175
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4176, _, _, _, _, _, _, _, _, _) => {
                                h4176
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4177, _, _, _, _, _, _, _, _) => {
                                h4177
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

pub fn f131(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4178 = v3;
    match m4178 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4179 = v0;
                let a4180 = 4u64;
                nat_eq(a4179, a4180)
            };
            let v7: bool = {
                let a4182 = 0u64;
                let a4183 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4181, _, _, _, _, _, _, _) => {
                        h4181
                    }
                };
                nat_lt(a4182, a4183)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4184, _, _, _, _, _, _, _, _, _) => {
                    h4184
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4185, _, _, _, _, _, _, _, _) => {
                    h4185
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4186, _, _, _, _, _, _, _) => {
                    h4186
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4187, _, _, _, _, _, _) => {
                    h4187
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4188, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4188
                }
            }, match v1.clone() {
                Adt1::C0(_, h4189, _, _, _, _, _, _, _, _, _, _) => {
                    h4189
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4190, _, _, _, _, _, _, _, _, _) => {
                    h4190
                }
            });
            if if if if if if if if if if if if v6 {
                let a4192 = match v2.clone() {
                    Adt2::C0(h4191, _, _, _, _, _, _, _, _, _, _) => {
                        h4191
                    }
                };
                let a4193 = 0u64;
                if nat_eq(a4192, a4193) {
                    let a4195 = match v5.clone() {
                        Adt2::C0(h4194, _, _, _, _, _, _, _, _, _, _) => {
                            h4194
                        }
                    };
                    let a4196 = 0u64;
                    nat_eq(a4195, a4196)
                } else {
                    false
                }
            } else {
                let a4198 = match v2.clone() {
                    Adt2::C0(h4197, _, _, _, _, _, _, _, _, _, _) => {
                        h4197
                    }
                };
                let a4199 = 0u64;
                if if nat_eq(a4198, a4199) {
                    true
                } else {
                    let a4201 = match v2.clone() {
                        Adt2::C0(h4200, _, _, _, _, _, _, _, _, _, _) => {
                            h4200
                        }
                    };
                    let a4202 = 26u64;
                    nat_eq(a4201, a4202)
                } {
                    let a4204 = match v5.clone() {
                        Adt2::C0(h4203, _, _, _, _, _, _, _, _, _, _) => {
                            h4203
                        }
                    };
                    let a4205 = 0u64;
                    if nat_eq(a4204, a4205) {
                        true
                    } else {
                        let a4207 = match v5.clone() {
                            Adt2::C0(h4206, _, _, _, _, _, _, _, _, _, _) => {
                                h4206
                            }
                        };
                        let a4208 = 26u64;
                        nat_eq(a4207, a4208)
                    }
                } else {
                    false
                }
            } {
                let a4211 = match v2.clone() {
                    Adt2::C0(h4209, _, _, _, _, _, _, _, _, _, _) => {
                        h4209
                    }
                };
                let a4212 = match v4.clone() {
                    Adt2::C0(h4210, _, _, _, _, _, _, _, _, _, _) => {
                        h4210
                    }
                };
                nat_eq(a4211, a4212)
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
                let a4214 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4213, _, _, _, _, _) => {
                        h4213
                    }
                };
                let a4215 = 4u64;
                nat_eq(a4214, a4215)
            } else {
                false
            } {
                let a4216 = v8;
                let a4217 = f23(v2.clone(), 1u64)?;
                nat_le(a4216, a4217)
            } else {
                false
            } {
                let a4219 = match v1.clone() {
                    Adt1::C0(_, _, _, h4218, _, _, _, _, _, _, _, _) => {
                        h4218
                    }
                };
                let a4220 = 1073741824u64;
                nat_lt(a4219, a4220)
            } else {
                false
            } {
                let a4222 = match v1.clone() {
                    Adt1::C0(h4221, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4221
                    }
                };
                let a4223 = 4294967295u64;
                if if if if nat_le(a4222, a4223) {
                    let a4225 = match v1.clone() {
                        Adt1::C0(_, h4224, _, _, _, _, _, _, _, _, _, _) => {
                            h4224
                        }
                    };
                    let a4226 = 4294967295u64;
                    nat_le(a4225, a4226)
                } else {
                    false
                } {
                    let a4228 = match v1.clone() {
                        Adt1::C0(_, _, h4227, _, _, _, _, _, _, _, _, _) => {
                            h4227
                        }
                    };
                    let a4229 = 4294967295u64;
                    nat_le(a4228, a4229)
                } else {
                    false
                } {
                    let a4231 = match v1.clone() {
                        Adt1::C0(_, _, _, h4230, _, _, _, _, _, _, _, _) => {
                            h4230
                        }
                    };
                    let a4232 = 4294967295u64;
                    nat_le(a4231, a4232)
                } else {
                    false
                } {
                    let a4234 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4233, _, _, _, _, _, _, _) => {
                            h4233
                        }
                    };
                    let a4235 = 4294967295u64;
                    nat_le(a4234, a4235)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4238 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4236) => {
                            h4236
                        }
                    };
                    let a4239 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4237) => {
                            h4237
                        }
                    };
                    nat_eq(a4238, a4239)
                } else {
                    true
                }
            } else {
                false
            } {
                f233(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4240, _, _, _, _, _, _, _, _) => {
                        h4240
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4241, _) => {
                        h4241
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
                    let a4242 = v10;
                    let a4243 = v8;
                    nat_add(a4242, a4243)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4244, _, _, _, _, _, _, _, _) => {
                        h4244
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f132(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4246 = match v0.clone() {
        Adt2::C0(h4245, _, _, _, _, _, _, _, _, _, _) => {
            h4245
        }
    };
    let a4247 = 0u64;
    if if if nat_eq(a4246, a4247) {
        let a4249 = match v1.clone() {
            Adt2::C0(h4248, _, _, _, _, _, _, _, _, _, _) => {
                h4248
            }
        };
        let a4250 = 0u64;
        nat_eq(a4249, a4250)
    } else {
        false
    } {
        f229(match v0.clone() {
            Adt2::C0(_, h4251, _, _, _, _, _, _, _, _, _) => {
                h4251
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4252, _, _, _, _, _, _, _, _) => {
                h4252
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4253, _, _, _, _, _, _, _) => {
                h4253
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4254, _, _, _, _, _, _) => {
                h4254
            }
        }, match v1.clone() {
            Adt2::C0(_, h4255, _, _, _, _, _, _, _, _, _) => {
                h4255
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4256, _, _, _, _, _, _, _, _) => {
                h4256
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4257, _, _, _, _, _, _, _) => {
                h4257
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4258, _, _, _, _, _, _) => {
                h4258
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4259 = v2;
            let a4260 = f23(v0.clone(), 1u64)?;
            nat_add(a4259, a4260)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f133(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4262 = match v1.clone() {
        Adt2::C0(h4261, _, _, _, _, _, _, _, _, _, _) => {
            h4261
        }
    };
    let a4263 = 0u64;
    if if if nat_eq(a4262, a4263) {
        true
    } else {
        let a4265 = match v1.clone() {
            Adt2::C0(h4264, _, _, _, _, _, _, _, _, _, _) => {
                h4264
            }
        };
        let a4266 = 1u64;
        nat_eq(a4265, a4266)
    } {
        true
    } else {
        let a4268 = match v1.clone() {
            Adt2::C0(h4267, _, _, _, _, _, _, _, _, _, _) => {
                h4267
            }
        };
        let a4269 = 30u64;
        nat_eq(a4268, a4269)
    } {
        let a4272 = match v1.clone() {
            Adt2::C0(h4270, _, _, _, _, _, _, _, _, _, _) => {
                h4270
            }
        };
        let a4273 = match v2.clone() {
            Adt2::C0(h4271, _, _, _, _, _, _, _, _, _, _) => {
                h4271
            }
        };
        if if if if if nat_eq(a4272, a4273) {
            let a4276 = match v1.clone() {
                Adt2::C0(_, h4274, _, _, _, _, _, _, _, _, _) => {
                    h4274
                }
            };
            let a4277 = match v2.clone() {
                Adt2::C0(_, h4275, _, _, _, _, _, _, _, _, _) => {
                    h4275
                }
            };
            nat_eq(a4276, a4277)
        } else {
            false
        } {
            let a4280 = match v1.clone() {
                Adt2::C0(_, _, _, h4278, _, _, _, _, _, _, _) => {
                    h4278
                }
            };
            let a4281 = match v2.clone() {
                Adt2::C0(_, _, _, h4279, _, _, _, _, _, _, _) => {
                    h4279
                }
            };
            nat_eq(a4280, a4281)
        } else {
            false
        } {
            let a4284 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4282, _, _, _, _, _, _) => {
                    h4282
                }
            };
            let a4285 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4283, _, _, _, _, _, _) => {
                    h4283
                }
            };
            nat_eq(a4284, a4285)
        } else {
            false
        } {
            let a4286 = v0;
            let a4287 = 4u64;
            if nat_eq(a4286, a4287) {
                let a4289 = match v1.clone() {
                    Adt2::C0(_, _, h4288, _, _, _, _, _, _, _, _) => {
                        h4288
                    }
                };
                let a4290 = 1u64;
                if nat_eq(a4289, a4290) {
                    let a4293 = match v1.clone() {
                        Adt2::C0(_, h4291, _, _, _, _, _, _, _, _, _) => {
                            h4291
                        }
                    };
                    let a4294 = match v2.clone() {
                        Adt2::C0(_, _, h4292, _, _, _, _, _, _, _, _) => {
                            h4292
                        }
                    };
                    nat_eq(a4293, a4294)
                } else {
                    false
                }
            } else {
                let a4295 = v0;
                let a4296 = 4u64;
                if if nat_lt(a4295, a4296) {
                    let a4299 = match v1.clone() {
                        Adt2::C0(_, h4297, _, _, _, _, _, _, _, _, _) => {
                            h4297
                        }
                    };
                    let a4300 = match v1.clone() {
                        Adt2::C0(_, _, h4298, _, _, _, _, _, _, _, _) => {
                            h4298
                        }
                    };
                    nat_eq(a4299, a4300)
                } else {
                    false
                } {
                    let a4303 = match v1.clone() {
                        Adt2::C0(_, _, h4301, _, _, _, _, _, _, _, _) => {
                            h4301
                        }
                    };
                    let a4304 = match v2.clone() {
                        Adt2::C0(_, _, h4302, _, _, _, _, _, _, _, _) => {
                            h4302
                        }
                    };
                    nat_eq(a4303, a4304)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4306 = f218(match v1.clone() {
                Adt2::C0(h4305, _, _, _, _, _, _, _, _, _, _) => {
                    h4305
                }
            });
            match m4306 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4307) => {
                            h4307
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

pub fn f134(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f234(match v1.clone() {
        Adt2::C0(h4308, _, _, _, _, _, _, _, _, _, _) => {
            h4308
        }
    }, v0) {
        let a4311 = match v1.clone() {
            Adt2::C0(h4309, _, _, _, _, _, _, _, _, _, _) => {
                h4309
            }
        };
        let a4312 = match v2.clone() {
            Adt2::C0(h4310, _, _, _, _, _, _, _, _, _, _) => {
                h4310
            }
        };
        nat_eq(a4311, a4312)
    } else {
        false
    } {
        let a4315 = match v1.clone() {
            Adt2::C0(_, h4313, _, _, _, _, _, _, _, _, _) => {
                h4313
            }
        };
        let a4316 = match v2.clone() {
            Adt2::C0(_, h4314, _, _, _, _, _, _, _, _, _) => {
                h4314
            }
        };
        if if if nat_eq(a4315, a4316) {
            let a4319 = match v1.clone() {
                Adt2::C0(_, _, h4317, _, _, _, _, _, _, _, _) => {
                    h4317
                }
            };
            let a4320 = match v2.clone() {
                Adt2::C0(_, _, h4318, _, _, _, _, _, _, _, _) => {
                    h4318
                }
            };
            nat_eq(a4319, a4320)
        } else {
            false
        } {
            let a4323 = match v1.clone() {
                Adt2::C0(_, _, _, h4321, _, _, _, _, _, _, _) => {
                    h4321
                }
            };
            let a4324 = match v2.clone() {
                Adt2::C0(_, _, _, h4322, _, _, _, _, _, _, _) => {
                    h4322
                }
            };
            nat_eq(a4323, a4324)
        } else {
            false
        } {
            let a4327 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4325, _, _, _, _, _, _) => {
                    h4325
                }
            };
            let a4328 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4326, _, _, _, _, _, _) => {
                    h4326
                }
            };
            nat_eq(a4327, a4328)
        } else {
            false
        }
    } else {
        false
    } {
        let m4330 = f218(match v1.clone() {
            Adt2::C0(h4329, _, _, _, _, _, _, _, _, _, _) => {
                h4329
            }
        });
        match m4330 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4331) => {
                        h4331
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4332, _, _, _, _, _, _, _, _, _, _) => {
                        h4332
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f135(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4333, _, _, _, _, _, _, _, _, _, _, _) => {
                h4333
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4334, _, _, _, _, _, _, _, _, _) => {
                h4334
            }
        }
    };
    let a4335 = 0u64;
    let a4336 = v4;
    if if nat_lt(a4335, a4336) {
        let a4337 = v4;
        let a4338 = 2147483648u64;
        nat_lt(a4337, a4338)
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
        let v7: u64 = f235(match v5.clone() {
            Adt2::C0(_, _, h4339, _, _, _, _, _, _, _, _) => {
                h4339
            }
        }, v4)?;
        let v8: u64 = f235(match v5.clone() {
            Adt2::C0(_, _, _, h4340, _, _, _, _, _, _, _) => {
                h4340
            }
        }, v4)?;
        let a4342 = match v5.clone() {
            Adt2::C0(h4341, _, _, _, _, _, _, _, _, _, _) => {
                h4341
            }
        };
        let a4343 = 0u64;
        if if if if if nat_eq(a4342, a4343) {
            let a4345 = match v6.clone() {
                Adt2::C0(h4344, _, _, _, _, _, _, _, _, _, _) => {
                    h4344
                }
            };
            let a4346 = 0u64;
            nat_eq(a4345, a4346)
        } else {
            false
        } {
            let a4348 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4347, _, _, _, _, _, _) => {
                    h4347
                }
            };
            let a4349 = 1u64;
            nat_eq(a4348, a4349)
        } else {
            false
        } {
            let a4352 = match v5.clone() {
                Adt2::C0(_, h4350, _, _, _, _, _, _, _, _, _) => {
                    h4350
                }
            };
            let a4353 = match v6.clone() {
                Adt2::C0(_, h4351, _, _, _, _, _, _, _, _, _) => {
                    h4351
                }
            };
            nat_eq(a4352, a4353)
        } else {
            false
        } {
            if v0 {
                let a4355 = match v5.clone() {
                    Adt2::C0(_, _, h4354, _, _, _, _, _, _, _, _) => {
                        h4354
                    }
                };
                let a4356 = 2147483648u64;
                if if if nat_lt(a4355, a4356) {
                    let a4358 = match v5.clone() {
                        Adt2::C0(_, _, _, h4357, _, _, _, _, _, _, _) => {
                            h4357
                        }
                    };
                    let a4359 = 2147483648u64;
                    nat_lt(a4358, a4359)
                } else {
                    false
                } {
                    let a4362 = {
                        let a4360 = v7;
                        let a4361 = v8;
                        nat_mul(a4360, a4361)?
                    };
                    let a4363 = 2147483648u64;
                    nat_le(a4362, a4363)
                } else {
                    false
                } {
                    f236(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4365 = match v1.clone() {
                    Adt1::C0(h4364, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4364
                    }
                };
                let a4366 = v7;
                if if if if nat_eq(a4365, a4366) {
                    let a4368 = match v1 {
                        Adt1::C0(_, h4367, _, _, _, _, _, _, _, _, _, _) => {
                            h4367
                        }
                    };
                    let a4369 = v8;
                    nat_eq(a4368, a4369)
                } else {
                    false
                } {
                    f237(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4371 = match v6.clone() {
                        Adt2::C0(_, _, h4370, _, _, _, _, _, _, _, _) => {
                            h4370
                        }
                    };
                    let a4372 = v4;
                    if nat_eq(a4371, a4372) {
                        let a4374 = match v6.clone() {
                            Adt2::C0(_, _, _, h4373, _, _, _, _, _, _, _) => {
                                h4373
                            }
                        };
                        let a4375 = v4;
                        nat_eq(a4374, a4375)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4379 = match v6 {
                        Adt2::C0(_, _, _, _, h4376, _, _, _, _, _, _) => {
                            h4376
                        }
                    };
                    let a4380 = {
                        let a4377 = v7;
                        let a4378 = v8;
                        nat_mul(a4377, a4378)?
                    };
                    nat_eq(a4379, a4380)
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

pub fn f136(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4382 = match v2.clone() {
        Adt2::C0(h4381, _, _, _, _, _, _, _, _, _, _) => {
            h4381
        }
    };
    let a4383 = 0u64;
    if if if if if nat_eq(a4382, a4383) {
        true
    } else {
        let a4385 = match v2.clone() {
            Adt2::C0(h4384, _, _, _, _, _, _, _, _, _, _) => {
                h4384
            }
        };
        let a4386 = 1u64;
        nat_eq(a4385, a4386)
    } {
        true
    } else {
        let a4388 = match v2.clone() {
            Adt2::C0(h4387, _, _, _, _, _, _, _, _, _, _) => {
                h4387
            }
        };
        let a4389 = 30u64;
        nat_eq(a4388, a4389)
    } {
        let a4392 = match v2.clone() {
            Adt2::C0(h4390, _, _, _, _, _, _, _, _, _, _) => {
                h4390
            }
        };
        let a4393 = match v3.clone() {
            Adt2::C0(h4391, _, _, _, _, _, _, _, _, _, _) => {
                h4391
            }
        };
        nat_eq(a4392, a4393)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a4395 = match v1.clone() {
                Adt1::C0(h4394, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4394
                }
            };
            let a4396 = 4294967296u64;
            nat_lt(a4395, a4396)
        } else {
            false
        } {
            let a4398 = match v1.clone() {
                Adt1::C0(_, h4397, _, _, _, _, _, _, _, _, _, _) => {
                    h4397
                }
            };
            let a4399 = 4294967296u64;
            nat_lt(a4398, a4399)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4401 = match v2.clone() {
                Adt2::C0(h4400, _, _, _, _, _, _, _, _, _, _) => {
                    h4400
                }
            };
            let a4402 = 0u64;
            if nat_eq(a4401, a4402) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a4403 = v0;
            let a4404 = 31u64;
            if nat_eq(a4403, a4404) {
                0u64
            } else {
                let a4405 = v0;
                let a4406 = 47u64;
                if nat_eq(a4405, a4406) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4407, _, _, _, _, _, _, _, _, _, _) => {
                h4407
            }
        }, match v1.clone() {
            Adt1::C0(h4408, _, _, _, _, _, _, _, _, _, _, _) => {
                h4408
            }
        }, match v1 {
            Adt1::C0(_, h4409, _, _, _, _, _, _, _, _, _, _) => {
                h4409
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f137(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4411 = match v0.clone() {
        Adt2::C0(h4410, _, _, _, _, _, _, _, _, _, _) => {
            h4410
        }
    };
    let a4412 = 0u64;
    if if if nat_eq(a4411, a4412) {
        let a4414 = match v1.clone() {
            Adt2::C0(h4413, _, _, _, _, _, _, _, _, _, _) => {
                h4413
            }
        };
        let a4415 = 0u64;
        nat_eq(a4414, a4415)
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

pub fn f138(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4417 = match v0.clone() {
        Adt2::C0(h4416, _, _, _, _, _, _, _, _, _, _) => {
            h4416
        }
    };
    let a4418 = 0u64;
    if if nat_eq(a4417, a4418) {
        let a4420 = match v1.clone() {
            Adt2::C0(h4419, _, _, _, _, _, _, _, _, _, _) => {
                h4419
            }
        };
        let a4421 = 26u64;
        nat_eq(a4420, a4421)
    } else {
        false
    } {
        let a4423 = match v0.clone() {
            Adt2::C0(_, _, _, h4422, _, _, _, _, _, _, _) => {
                h4422
            }
        };
        let a4424 = 1u64;
        if if if if if if if nat_eq(a4423, a4424) {
            let a4426 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4425, _, _, _, _, _, _) => {
                    h4425
                }
            };
            let a4427 = 1u64;
            nat_eq(a4426, a4427)
        } else {
            false
        } {
            let a4429 = match v0.clone() {
                Adt2::C0(_, h4428, _, _, _, _, _, _, _, _, _) => {
                    h4428
                }
            };
            let a4430 = 2147483647u64;
            nat_le(a4429, a4430)
        } else {
            false
        } {
            let a4433 = match v1.clone() {
                Adt2::C0(_, h4431, _, _, _, _, _, _, _, _, _) => {
                    h4431
                }
            };
            let a4434 = match v0.clone() {
                Adt2::C0(_, _, h4432, _, _, _, _, _, _, _, _) => {
                    h4432
                }
            };
            nat_eq(a4433, a4434)
        } else {
            false
        } {
            let a4436 = match v1.clone() {
                Adt2::C0(_, _, h4435, _, _, _, _, _, _, _, _) => {
                    h4435
                }
            };
            let a4437 = 1u64;
            nat_eq(a4436, a4437)
        } else {
            false
        } {
            let a4439 = match v1.clone() {
                Adt2::C0(_, _, _, h4438, _, _, _, _, _, _, _) => {
                    h4438
                }
            };
            let a4440 = 1u64;
            nat_eq(a4439, a4440)
        } else {
            false
        } {
            let a4442 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4441, _, _, _, _, _, _) => {
                    h4441
                }
            };
            let a4443 = 1u64;
            nat_eq(a4442, a4443)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4446 = match v0.clone() {
                    Adt2::C0(_, _, h4444, _, _, _, _, _, _, _, _) => {
                        h4444
                    }
                };
                let a4447 = f179(match v0.clone() {
                    Adt2::C0(_, h4445, _, _, _, _, _, _, _, _, _) => {
                        h4445
                    }
                })?;
                nat_mul(a4446, a4447)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f139(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4448 = v1;
    match m4448 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f184(f25(v0.clone(), 1u64))?;
            let a4450 = match v0.clone() {
                Adt2::C0(h4449, _, _, _, _, _, _, _, _, _, _) => {
                    h4449
                }
            };
            let a4451 = 26u64;
            if if if nat_eq(a4450, a4451) {
                let a4453 = match v3.clone() {
                    Adt2::C0(h4452, _, _, _, _, _, _, _, _, _, _) => {
                        h4452
                    }
                };
                let a4454 = 26u64;
                nat_eq(a4453, a4454)
            } else {
                false
            } {
                let a4456 = match v2.clone() {
                    Adt2::C0(h4455, _, _, _, _, _, _, _, _, _, _) => {
                        h4455
                    }
                };
                let a4457 = 27u64;
                nat_eq(a4456, a4457)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a4459 = match v2.clone() {
                        Adt2::C0(_, h4458, _, _, _, _, _, _, _, _, _) => {
                            h4458
                        }
                    };
                    let a4460 = 1u64;
                    nat_eq(a4459, a4460)
                } else {
                    false
                } {
                    let a4462 = match v2.clone() {
                        Adt2::C0(_, _, h4461, _, _, _, _, _, _, _, _) => {
                            h4461
                        }
                    };
                    let a4463 = 1u64;
                    nat_eq(a4462, a4463)
                } else {
                    false
                } {
                    let a4465 = match v2.clone() {
                        Adt2::C0(_, _, _, h4464, _, _, _, _, _, _, _) => {
                            h4464
                        }
                    };
                    let a4466 = 1u64;
                    nat_eq(a4465, a4466)
                } else {
                    false
                } {
                    let a4468 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4467, _, _, _, _, _, _) => {
                            h4467
                        }
                    };
                    let a4469 = 1u64;
                    nat_eq(a4468, a4469)
                } else {
                    false
                } {
                    let a4470 = v4;
                    let a4471 = 9223372036854775808u64;
                    nat_lt(a4470, a4471)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4474 = 2u64;
                        let a4475 = {
                            let a4472 = v4;
                            let a4473 = 2u64;
                            nat_mul(a4472, a4473)?
                        };
                        nat_add(a4474, a4475)?
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

pub fn f140(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4476, _, _, _, _, _, _, _, _, _, _) => {
            h4476
        }
    };
    let a4477 = v3;
    let a4478 = 0u64;
    if if nat_eq(a4477, a4478) {
        true
    } else {
        let a4479 = v0;
        let a4480 = 13u64;
        if nat_eq(a4479, a4480) {
            let a4481 = v3;
            let a4482 = 1u64;
            if nat_eq(a4481, a4482) {
                true
            } else {
                let a4483 = v3;
                let a4484 = 30u64;
                nat_eq(a4483, a4484)
            }
        } else {
            false
        }
    } {
        let a4486 = v3;
        let a4487 = match v2.clone() {
            Adt2::C0(h4485, _, _, _, _, _, _, _, _, _, _) => {
                h4485
            }
        };
        if if nat_eq(a4486, a4487) {
            let a4488 = v0;
            let a4489 = 13u64;
            if nat_eq(a4488, a4489) {
                let a4491 = match v2.clone() {
                    Adt2::C0(_, h4490, _, _, _, _, _, _, _, _, _) => {
                        h4490
                    }
                };
                let a4492 = 1u64;
                if if if nat_eq(a4491, a4492) {
                    let a4494 = match v2.clone() {
                        Adt2::C0(_, _, h4493, _, _, _, _, _, _, _, _) => {
                            h4493
                        }
                    };
                    let a4495 = 1u64;
                    nat_eq(a4494, a4495)
                } else {
                    false
                } {
                    let a4497 = match v2.clone() {
                        Adt2::C0(_, _, _, h4496, _, _, _, _, _, _, _) => {
                            h4496
                        }
                    };
                    let a4498 = 1u64;
                    nat_eq(a4497, a4498)
                } else {
                    false
                } {
                    let a4500 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4499, _, _, _, _, _, _) => {
                            h4499
                        }
                    };
                    let a4501 = 1u64;
                    nat_eq(a4500, a4501)
                } else {
                    false
                }
            } else {
                let a4503 = match v2.clone() {
                    Adt2::C0(_, h4502, _, _, _, _, _, _, _, _, _) => {
                        h4502
                    }
                };
                let a4504 = 1u64;
                if if if nat_eq(a4503, a4504) {
                    let a4507 = match v1.clone() {
                        Adt2::C0(_, _, h4505, _, _, _, _, _, _, _, _) => {
                            h4505
                        }
                    };
                    let a4508 = match v2.clone() {
                        Adt2::C0(_, _, h4506, _, _, _, _, _, _, _, _) => {
                            h4506
                        }
                    };
                    nat_eq(a4507, a4508)
                } else {
                    false
                } {
                    let a4511 = match v1.clone() {
                        Adt2::C0(_, _, _, h4509, _, _, _, _, _, _, _) => {
                            h4509
                        }
                    };
                    let a4512 = match v2.clone() {
                        Adt2::C0(_, _, _, h4510, _, _, _, _, _, _, _) => {
                            h4510
                        }
                    };
                    nat_eq(a4511, a4512)
                } else {
                    false
                } {
                    let a4515 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4513, _, _, _, _, _, _) => {
                            h4513
                        }
                    };
                    let a4516 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4514, _, _, _, _, _, _) => {
                            h4514
                        }
                    };
                    nat_eq(a4515, a4516)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt10::C4(f238(v4.clone(), v0, v3)?, {
                let a4517 = v3;
                let a4518 = 0u64;
                if nat_eq(a4517, a4518) {
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

pub fn f141(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4519 = v0;
    let a4520 = 89u64;
    if if nat_eq(a4519, a4520) {
        let a4521 = v1;
        let a4522 = 16u64;
        nat_eq(a4521, a4522)
    } else {
        false
    } {
        Ok(Some(24u64))
    } else {
        let a4523 = v0;
        let a4524 = 89u64;
        if if nat_eq(a4523, a4524) {
            let a4525 = v1;
            let a4526 = 14u64;
            nat_eq(a4525, a4526)
        } else {
            false
        } {
            Ok(Some(23u64))
        } else {
            let a4527 = v0;
            let a4528 = 89u64;
            if if nat_eq(a4527, a4528) {
                let a4529 = v1;
                let a4530 = 15u64;
                nat_eq(a4529, a4530)
            } else {
                false
            } {
                Ok(Some(22u64))
            } else {
                let a4531 = v0;
                let a4532 = 89u64;
                if if nat_eq(a4531, a4532) {
                    let a4533 = 8u64;
                    let a4534 = v1;
                    if nat_le(a4533, a4534) {
                        let a4535 = v1;
                        let a4536 = 9u64;
                        nat_le(a4535, a4536)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some({
                        let a4537 = v1;
                        let a4538 = 12u64;
                        nat_add(a4537, a4538)?
                    }))
                } else {
                    let a4539 = v0;
                    let a4540 = 89u64;
                    if if nat_eq(a4539, a4540) {
                        let a4541 = v1;
                        let a4542 = 4u64;
                        nat_eq(a4541, a4542)
                    } else {
                        false
                    } {
                        Ok(Some(19u64))
                    } else {
                        let a4543 = 10u64;
                        let a4544 = v0;
                        if if nat_le(a4543, a4544) {
                            let a4545 = v0;
                            let a4546 = 12u64;
                            nat_le(a4545, a4546)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4547 = v0;
                                let a4548 = 3u64;
                                nat_add(a4547, a4548)?
                            }))
                        } else {
                            let a4549 = v0;
                            let a4550 = 89u64;
                            if if nat_eq(a4549, a4550) {
                                let a4551 = v1;
                                let a4552 = 7u64;
                                nat_eq(a4551, a4552)
                            } else {
                                false
                            } {
                                Ok(Some(17u64))
                            } else {
                                let a4553 = v0;
                                let a4554 = 89u64;
                                if if nat_eq(a4553, a4554) {
                                    let a4555 = v1;
                                    let a4556 = 10u64;
                                    nat_eq(a4555, a4556)
                                } else {
                                    false
                                } {
                                    Ok(Some(18u64))
                                } else {
                                    let a4557 = v0;
                                    let a4558 = 89u64;
                                    if if nat_eq(a4557, a4558) {
                                        let a4559 = v1;
                                        let a4560 = 13u64;
                                        nat_eq(a4559, a4560)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(16u64))
                                    } else {
                                        let a4561 = v0;
                                        let a4562 = 8u64;
                                        if nat_eq(a4561, a4562) {
                                            Ok(Some(5u64))
                                        } else {
                                            let a4563 = v0;
                                            let a4564 = 9u64;
                                            if nat_eq(a4563, a4564) {
                                                Ok(Some(6u64))
                                            } else {
                                                let a4565 = v0;
                                                let a4566 = 89u64;
                                                if nat_eq(a4565, a4566) {
                                                    let a4567 = v1;
                                                    let a4568 = 0u64;
                                                    if nat_eq(a4567, a4568) {
                                                        Ok(Some(0u64))
                                                    } else {
                                                        let a4569 = v1;
                                                        let a4570 = 1u64;
                                                        if nat_eq(a4569, a4570) {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            let a4571 = v1;
                                                            let a4572 = 2u64;
                                                            if nat_eq(a4571, a4572) {
                                                                Ok(Some(2u64))
                                                            } else {
                                                                let a4573 = v1;
                                                                let a4574 = 3u64;
                                                                if nat_eq(a4573, a4574) {
                                                                    Ok(Some(3u64))
                                                                } else {
                                                                    let a4575 = v1;
                                                                    let a4576 = 6u64;
                                                                    if nat_eq(a4575, a4576) {
                                                                        Ok(Some(4u64))
                                                                    } else {
                                                                        let a4577 = v1;
                                                                        let a4578 = 12u64;
                                                                        if nat_eq(a4577, a4578) {
                                                                            Ok(Some(7u64))
                                                                        } else {
                                                                            let a4579 = v1;
                                                                            let a4580 = 11u64;
                                                                            if nat_eq(a4579, a4580) {
                                                                                Ok(Some(8u64))
                                                                            } else {
                                                                                let a4581 = v1;
                                                                                let a4582 = 18u64;
                                                                                if nat_eq(a4581, a4582) {
                                                                                    Ok(Some(9u64))
                                                                                } else {
                                                                                    let a4583 = v1;
                                                                                    let a4584 = 19u64;
                                                                                    if nat_eq(a4583, a4584) {
                                                                                        Ok(Some(10u64))
                                                                                    } else {
                                                                                        let a4585 = v1;
                                                                                        let a4586 = 20u64;
                                                                                        if nat_eq(a4585, a4586) {
                                                                                            Ok(Some(11u64))
                                                                                        } else {
                                                                                            let a4587 = v1;
                                                                                            let a4588 = 21u64;
                                                                                            if nat_eq(a4587, a4588) {
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

pub fn f142(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4589 = f239(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4589 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4590 = v0;
            let a4591 = 2u64;
            if if if if nat_eq(a4590, a4591) {
                true
            } else {
                let a4592 = v0;
                let a4593 = 5u64;
                nat_eq(a4592, a4593)
            } {
                true
            } else {
                let a4594 = v0;
                let a4595 = 6u64;
                nat_eq(a4594, a4595)
            } {
                true
            } else {
                let a4596 = v0;
                let a4597 = 7u64;
                nat_eq(a4596, a4597)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4598 = v3;
                    match m4598 {
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

pub fn f143(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4600 = match v1.clone() {
        Adt2::C0(h4599, _, _, _, _, _, _, _, _, _, _) => {
            h4599
        }
    };
    let a4601 = 0u64;
    if if if if if nat_eq(a4600, a4601) {
        true
    } else {
        let a4603 = match v1.clone() {
            Adt2::C0(h4602, _, _, _, _, _, _, _, _, _, _) => {
                h4602
            }
        };
        let a4604 = 1u64;
        nat_eq(a4603, a4604)
    } {
        true
    } else {
        let a4606 = match v1.clone() {
            Adt2::C0(h4605, _, _, _, _, _, _, _, _, _, _) => {
                h4605
            }
        };
        let a4607 = 30u64;
        nat_eq(a4606, a4607)
    } {
        let a4610 = match v1.clone() {
            Adt2::C0(h4608, _, _, _, _, _, _, _, _, _, _) => {
                h4608
            }
        };
        let a4611 = match v2.clone() {
            Adt2::C0(h4609, _, _, _, _, _, _, _, _, _, _) => {
                h4609
            }
        };
        if nat_eq(a4610, a4611) {
            true
        } else {
            let a4613 = match v2.clone() {
                Adt2::C0(h4612, _, _, _, _, _, _, _, _, _, _) => {
                    h4612
                }
            };
            let a4614 = 0u64;
            nat_eq(a4613, a4614)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4616 = f23(v2.clone(), 1u64)?;
            let a4617 = f240(v0, match v2.clone() {
                Adt2::C0(h4615, _, _, _, _, _, _, _, _, _, _) => {
                    h4615
                }
            });
            nat_mul(a4616, a4617)?
        }, {
            let a4619 = match v2.clone() {
                Adt2::C0(h4618, _, _, _, _, _, _, _, _, _, _) => {
                    h4618
                }
            };
            let a4620 = 0u64;
            if nat_eq(a4619, a4620) {
                4u64
            } else {
                2u64
            }
        }, f25(v1.clone(), 1u64), f25(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h4621, _, _, _, _, _, _, _, _, _, _) => {
                h4621
            }
        }, match v2 {
            Adt2::C0(h4622, _, _, _, _, _, _, _, _, _, _) => {
                h4622
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f144(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4635 = {
        let a4631 = {
            let a4627 = {
                let a4623 = v0;
                let a4624 = v4;
                nat_mul(a4623, a4624)?
            };
            let a4628 = {
                let a4625 = v1;
                let a4626 = v5;
                nat_mul(a4625, a4626)?
            };
            nat_add(a4627, a4628)?
        };
        let a4632 = {
            let a4629 = v2;
            let a4630 = v6;
            nat_mul(a4629, a4630)?
        };
        nat_add(a4631, a4632)?
    };
    let a4636 = {
        let a4633 = v3;
        let a4634 = v7;
        nat_mul(a4633, a4634)?
    };
    nat_add(a4635, a4636)
}

pub fn f145(v0: u64) -> u64 {
    let a4637 = v0;
    let a4638 = 2147483648u64;
    if nat_lt(a4637, a4638) {
        0u64
    } else {
        let a4639 = 4294967296u64;
        let a4640 = v0;
        nat_sub(a4639, a4640)
    }
}

pub fn f146(v0: u64) -> u64 {
    let a4641 = v0;
    let a4642 = 2147483648u64;
    if nat_lt(a4641, a4642) {
        v0
    } else {
        0u64
    }
}

pub fn f147(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4643 = v7;
        let a4644 = 6u64;
        if nat_eq(a4643, a4644) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f91(v3, v4), f91(v5, v6), f92(v7, v8), f178(v1), f178(v9))
}

pub fn f148(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4645 = v0;
    match m4645 {
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
        Adt4::C13(v58, v59, v60, v61) => {
            Adt11::C4(v58, v59, v60, v61)
        }
        Adt4::C14(v62, _, _) => {
            let a4646 = 13u64;
            let a4647 = v62;
            if nat_le(a4646, a4647) {
                Adt11::C7(v62, v1, v2)
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
        Adt4::C27(v82) => {
            Adt11::C2(match v82.clone() {
                Adt5::C0(_, _, _, _, _, h4648, _, _) => {
                    h4648
                }
            }, match v82 {
                Adt5::C0(_, _, _, _, _, _, _, h4649) => {
                    h4649
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
        Adt4::C43(_, _) => {
            Adt11::C6
        }
        Adt4::C44(_) => {
            Adt11::C6
        }
    }
}

pub fn f149(v0: Adt4) -> u64 {
    let m4650 = v0;
    match m4650 {
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
        Adt4::C14(v60, _, _) => {
            v60
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
        Adt4::C43(_, _) => {
            0u64
        }
        Adt4::C44(_) => {
            0u64
        }
    }
}

pub fn f150(v0: Adt4, v1: u64) -> u64 {
    let m4651 = v0;
    match m4651 {
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
        Adt4::C16(_, v65) => {
            v65
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v68, _) => {
            let a4652 = v1;
            let a4653 = 6u64;
            if nat_eq(a4652, a4653) {
                v68
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
        Adt4::C26(v82, _, _, _) => {
            let a4662 = v1;
            let a4663 = 6u64;
            if if if nat_eq(a4662, a4663) {
                true
            } else {
                let a4664 = v1;
                let a4665 = 7u64;
                nat_eq(a4664, a4665)
            } {
                let a4666 = v82;
                let a4667 = 30u64;
                nat_eq(a4666, a4667)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v81) => {
            match v81 {
                Adt5::C0(_, _, _, _, _, _, _, h4661) => {
                    h4661
                }
            }
        }
        Adt4::C28(v77) => {
            let a4654 = v1;
            let a4655 = 6u64;
            if nat_eq(a4654, a4655) {
                match v77 {
                    Adt7::C0(_, _, _, _, h4656) => {
                        h4656
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v80) => {
            let a4657 = v1;
            let a4658 = 6u64;
            if if nat_eq(a4657, a4658) {
                true
            } else {
                let a4659 = v1;
                let a4660 = 7u64;
                nat_eq(a4659, a4660)
            } {
                v80
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
        Adt4::C43(_, _) => {
            0u64
        }
        Adt4::C44(_) => {
            0u64
        }
    }
}

pub fn f151(v0: Adt4, v1: u64) -> Adt11 {
    let a4668 = v1;
    let a4669 = 4u64;
    if nat_lt(a4668, a4669) {
        Adt11::C0(v1)
    } else {
        let a4670 = v1;
        let a4671 = 4u64;
        if nat_eq(a4670, a4671) {
            Adt11::C1(0u64)
        } else {
            let a4672 = v1;
            let a4673 = 5u64;
            if nat_eq(a4672, a4673) {
                Adt11::C1(3u64)
            } else {
                let a4674 = v1;
                let a4675 = 6u64;
                if nat_eq(a4674, a4675) {
                    Adt11::C2(f150(v0, v1), 0u64, false)
                } else {
                    let a4676 = v1;
                    let a4677 = 7u64;
                    if nat_eq(a4676, a4677) {
                        Adt11::C2(0u64, f150(v0, v1), false)
                    } else {
                        let a4678 = v1;
                        let a4679 = 16u64;
                        if nat_lt(a4678, a4679) {
                            Adt11::C1({
                                let a4680 = v1;
                                let a4681 = 8u64;
                                nat_sub(a4680, a4681)
                            })
                        } else {
                            let a4682 = v1;
                            let a4683 = 20u64;
                            if nat_lt(a4682, a4683) {
                                Adt11::C5({
                                    let a4684 = v1;
                                    let a4685 = 16u64;
                                    nat_sub(a4684, a4685)
                                })
                            } else {
                                let a4686 = v1;
                                let a4687 = 20u64;
                                if nat_eq(a4686, a4687) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4688 = v1;
                                    let a4689 = 21u64;
                                    if nat_eq(a4688, a4689) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4690 = v1;
                                        let a4691 = 22u64;
                                        if nat_eq(a4690, a4691) {
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

pub fn f152(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4709 = {
        let a4704 = f241(v2, {
            let a4695 = {
                let a4692 = v0;
                let a4693 = v1;
                let a4694 = 0u64;
                nat_quot(a4692, a4693, a4694)
            };
            let a4696 = v2;
            let a4697 = 0u64;
            nat_rem(a4695, a4696, a4697)
        }, {
            let a4701 = {
                let a4698 = v0;
                let a4699 = v1;
                let a4700 = 0u64;
                nat_quot(a4698, a4699, a4700)
            };
            let a4702 = v2;
            let a4703 = 0u64;
            nat_quot(a4701, a4702, a4703)
        })?;
        let a4705 = v1;
        nat_mul(a4704, a4705)?
    };
    let a4710 = {
        let a4706 = v0;
        let a4707 = v1;
        let a4708 = 0u64;
        nat_rem(a4706, a4707, a4708)
    };
    nat_add(a4709, a4710)
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4712 = v0;
        let a4713 = match v2.clone() {
            Adt3::C0(h4711, _, _, _, _, _, _, _) => {
                h4711
            }
        };
        let a4714 = 0u64;
        nat_quot(a4712, a4713, a4714)
    };
    let v4: u64 = {
        let a4716 = v3;
        let a4717 = match v2.clone() {
            Adt3::C0(_, h4715, _, _, _, _, _, _) => {
                h4715
            }
        };
        let a4718 = 0u64;
        nat_quot(a4716, a4717, a4718)
    };
    f242({
        let a4720 = v0;
        let a4721 = match v2.clone() {
            Adt3::C0(h4719, _, _, _, _, _, _, _) => {
                h4719
            }
        };
        let a4722 = 0u64;
        nat_rem(a4720, a4721, a4722)
    }, {
        let a4724 = v3;
        let a4725 = match v2.clone() {
            Adt3::C0(_, h4723, _, _, _, _, _, _) => {
                h4723
            }
        };
        let a4726 = 0u64;
        nat_rem(a4724, a4725, a4726)
    }, {
        let a4728 = v4;
        let a4729 = match v2.clone() {
            Adt3::C0(_, _, h4727, _, _, _, _, _) => {
                h4727
            }
        };
        let a4730 = 0u64;
        nat_rem(a4728, a4729, a4730)
    }, {
        let a4732 = v4;
        let a4733 = match v2 {
            Adt3::C0(_, _, h4731, _, _, _, _, _) => {
                h4731
            }
        };
        let a4734 = 0u64;
        nat_quot(a4732, a4733, a4734)
    }, match v1.clone() {
        Adt3::C0(h4735, _, _, _, _, _, _, _) => {
            h4735
        }
    }, match v1.clone() {
        Adt3::C0(_, h4736, _, _, _, _, _, _) => {
            h4736
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4737, _, _, _, _, _) => {
            h4737
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4738, _, _, _, _) => {
            h4738
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4739, _, _, _) => {
            h4739
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4740, _, _) => {
            h4740
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4741, _) => {
            h4741
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4742) => {
            h4742
        }
    })
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4744 = v0;
        let a4745 = match v3.clone() {
            Adt3::C0(h4743, _, _, _, _, _, _, _) => {
                h4743
            }
        };
        let a4746 = 0u64;
        nat_quot(a4744, a4745, a4746)
    };
    let v6: u64 = {
        let a4748 = v5;
        let a4749 = match v3.clone() {
            Adt3::C0(_, h4747, _, _, _, _, _, _) => {
                h4747
            }
        };
        let a4750 = 0u64;
        nat_quot(a4748, a4749, a4750)
    };
    let v7: u64 = {
        let a4752 = v0;
        let a4753 = match v3.clone() {
            Adt3::C0(h4751, _, _, _, _, _, _, _) => {
                h4751
            }
        };
        let a4754 = 0u64;
        nat_rem(a4752, a4753, a4754)
    };
    let v8: u64 = {
        let a4756 = v5;
        let a4757 = match v3.clone() {
            Adt3::C0(_, h4755, _, _, _, _, _, _) => {
                h4755
            }
        };
        let a4758 = 0u64;
        nat_rem(a4756, a4757, a4758)
    };
    let v9: u64 = {
        let a4760 = v6;
        let a4761 = match v3.clone() {
            Adt3::C0(_, _, h4759, _, _, _, _, _) => {
                h4759
            }
        };
        let a4762 = 0u64;
        nat_rem(a4760, a4761, a4762)
    };
    let v10: u64 = {
        let a4764 = v6;
        let a4765 = match v3.clone() {
            Adt3::C0(_, _, h4763, _, _, _, _, _) => {
                h4763
            }
        };
        let a4766 = 0u64;
        nat_quot(a4764, a4765, a4766)
    };
    let v11: u64 = {
        let a4767 = v4;
        let a4768 = 0u64;
        if nat_eq(a4767, a4768) {
            match v1.clone() {
                Adt3::C0(h4769, _, _, _, _, _, _, _) => {
                    h4769
                }
            }
        } else {
            let a4770 = v4;
            let a4771 = 1u64;
            if nat_eq(a4770, a4771) {
                match v1.clone() {
                    Adt3::C0(_, h4772, _, _, _, _, _, _) => {
                        h4772
                    }
                }
            } else {
                let a4773 = v4;
                let a4774 = 2u64;
                if nat_eq(a4773, a4774) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4775, _, _, _, _, _) => {
                            h4775
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4776, _, _, _, _) => {
                            h4776
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4783 = v11;
        let a4784 = {
            let a4777 = v4;
            let a4778 = 0u64;
            if nat_eq(a4777, a4778) {
                v7
            } else {
                let a4779 = v4;
                let a4780 = 1u64;
                if nat_eq(a4779, a4780) {
                    v8
                } else {
                    let a4781 = v4;
                    let a4782 = 2u64;
                    if nat_eq(a4781, a4782) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4783, a4784)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f144(if if v12 {
        let a4785 = v4;
        let a4786 = 0u64;
        nat_eq(a4785, a4786)
    } else {
        false
    } {
        let a4787 = v7;
        let a4788 = v11;
        nat_sub(a4787, a4788)
    } else {
        v7
    }, if if v12 {
        let a4789 = v4;
        let a4790 = 1u64;
        nat_eq(a4789, a4790)
    } else {
        false
    } {
        let a4791 = v8;
        let a4792 = v11;
        nat_sub(a4791, a4792)
    } else {
        v8
    }, if if v12 {
        let a4793 = v4;
        let a4794 = 2u64;
        nat_eq(a4793, a4794)
    } else {
        false
    } {
        let a4795 = v9;
        let a4796 = v11;
        nat_sub(a4795, a4796)
    } else {
        v9
    }, if if v12 {
        let a4797 = v4;
        let a4798 = 3u64;
        nat_eq(a4797, a4798)
    } else {
        false
    } {
        let a4799 = v10;
        let a4800 = v11;
        nat_sub(a4799, a4800)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4801, _, _, _) => {
            h4801
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4802, _, _) => {
            h4802
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4803, _) => {
            h4803
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4804) => {
            h4804
        }
    })?, f93(v0, v3)?))
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f57(v1.clone(), f243(v8, v3, match v1.clone() {
            Adt3::C0(h4805, _, _, _, _, _, _, _) => {
                h4805
            }
        })?, f243(v9, v4, match v1.clone() {
            Adt3::C0(_, h4806, _, _, _, _, _, _) => {
                h4806
            }
        })?, f243(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4807, _, _, _, _, _) => {
                h4807
            }
        })?, f243(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4808, _, _, _, _) => {
                h4808
            }
        })?)?, f93(v0, v2)?))
    } else {
        if if if if f244(v8, v3, match v1.clone() {
            Adt3::C0(h4809, _, _, _, _, _, _, _) => {
                h4809
            }
        })? {
            f244(v9, v4, match v1.clone() {
                Adt3::C0(_, h4810, _, _, _, _, _, _) => {
                    h4810
                }
            })?
        } else {
            false
        } {
            f244(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4811, _, _, _, _, _) => {
                    h4811
                }
            })?
        } else {
            false
        } {
            f244(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4812, _, _, _, _) => {
                    h4812
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f57(v1, f245(v8, v3)?, f245(v9, v4)?, f245(v10, v5)?, f245(v11, v6)?)?, f93(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f93(v0, v2)?))
        }
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f57(v1.clone(), f246(v4, v3, match v1 {
        Adt3::C0(h4813, _, _, _, _, _, _, _) => {
            h4813
        }
    })?, v5, v6, v7)?, f93(v0, v2)?))
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4814 = v0;
    let a4815 = v3;
    if nat_lt(a4814, a4815) {
        Ok(Adt9::C1(false, {
            let a4816 = v0;
            let a4817 = 4u64;
            nat_mul(a4816, a4817)?
        }, {
            let a4818 = v0;
            let a4819 = 4u64;
            nat_mul(a4818, a4819)?
        }))
    } else {
        let v6: u64 = {
            let a4820 = v0;
            let a4821 = v3;
            nat_sub(a4820, a4821)
        };
        let v7: u64 = f93(v6, v1)?;
        let v8: u64 = f247(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4822 = v0;
    let a4823 = v3;
    if nat_lt(a4822, a4823) {
        Ok(Adt9::C1(true, 0u64, f93(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4824 = v0;
            let a4825 = v3;
            nat_sub(a4824, a4825)
        };
        let v5: u64 = f153(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f93(v4, v1)?, 3u64, v5))
    }
}

pub fn f159(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4827 = v1;
        let a4828 = match v3.clone() {
            Adt3::C0(h4826, _, _, _, _, _, _, _) => {
                h4826
            }
        };
        let a4829 = 0u64;
        nat_rem(a4827, a4828, a4829)
    };
    let v5: u64 = {
        let a4835 = {
            let a4831 = v1;
            let a4832 = match v3.clone() {
                Adt3::C0(h4830, _, _, _, _, _, _, _) => {
                    h4830
                }
            };
            let a4833 = 0u64;
            nat_quot(a4831, a4832, a4833)
        };
        let a4836 = match v3.clone() {
            Adt3::C0(_, h4834, _, _, _, _, _, _) => {
                h4834
            }
        };
        let a4837 = 0u64;
        nat_rem(a4835, a4836, a4837)
    };
    let v6: bool = f248(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f153(v1, v2, v3.clone())?
    } else {
        0u64
    }, f93(v1, v3)?))
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4838 = v3;
        let a4839 = 8u64;
        nat_add(a4838, a4839)?
    }, 0u64, f93(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f153(v0, v5, v2.clone())?
    }, 3u64, f93(v0, v2)?))
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4840 = v4;
    let a4841 = 0u64;
    if if nat_eq(a4840, a4841) {
        f125(v3)
    } else {
        false
    } {
        let m4842 = f218(v3);
        match m4842 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4843, _) => {
                        h4843
                    }
                };
                Ok(Adt9::C5(v3, f93(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4844) => {
                        h4844
                    }
                }, f93({
                    let a4845 = v0;
                    let a4846 = v6;
                    nat_mul(a4845, a4846)?
                }, v2.clone())?, {
                    let a4852 = {
                        let a4850 = {
                            let a4847 = v6;
                            let a4848 = 1u64;
                            nat_sub(a4847, a4848)
                        };
                        let a4851 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4849, _, _, _) => {
                                h4849
                            }
                        };
                        nat_mul(a4850, a4851)?
                    };
                    let a4853 = 4u64;
                    nat_add(a4852, a4853)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4854, _, _, _) => {
                        h4854
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f93(v0, v1)?, f93(v0, v2)?))
    }
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4856 = v0;
        let a4857 = match v2.clone() {
            Adt3::C0(h4855, _, _, _, _, _, _, _) => {
                h4855
            }
        };
        let a4858 = 0u64;
        nat_rem(a4856, a4857, a4858)
    };
    let v7: u64 = {
        let a4860 = v0;
        let a4861 = match v2.clone() {
            Adt3::C0(h4859, _, _, _, _, _, _, _) => {
                h4859
            }
        };
        let a4862 = 0u64;
        nat_quot(a4860, a4861, a4862)
    };
    let v8: u64 = {
        let a4864 = v7;
        let a4865 = match v2.clone() {
            Adt3::C0(_, h4863, _, _, _, _, _, _) => {
                h4863
            }
        };
        let a4866 = 0u64;
        nat_rem(a4864, a4865, a4866)
    };
    let v9: u64 = {
        let a4868 = v7;
        let a4869 = match v2.clone() {
            Adt3::C0(_, h4867, _, _, _, _, _, _) => {
                h4867
            }
        };
        let a4870 = 0u64;
        nat_quot(a4868, a4869, a4870)
    };
    let v10: u64 = {
        let a4872 = v9;
        let a4873 = match v2.clone() {
            Adt3::C0(_, _, h4871, _, _, _, _, _) => {
                h4871
            }
        };
        let a4874 = 0u64;
        nat_rem(a4872, a4873, a4874)
    };
    let v11: u64 = {
        let a4876 = v9;
        let a4877 = match v2.clone() {
            Adt3::C0(_, _, h4875, _, _, _, _, _) => {
                h4875
            }
        };
        let a4878 = 0u64;
        nat_quot(a4876, a4877, a4878)
    };
    let v12: u64 = f93(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4887 = {
                let a4882 = {
                    let a4879 = v10;
                    let a4880 = v4;
                    let a4881 = 0u64;
                    nat_quot(a4879, a4880, a4881)
                };
                let a4883 = v5;
                nat_mul(a4882, a4883)?
            };
            let a4888 = {
                let a4884 = v8;
                let a4885 = v4;
                let a4886 = 0u64;
                nat_quot(a4884, a4885, a4886)
            };
            nat_add(a4887, a4888)?
        };
        let v14: u64 = {
            let a4908 = v6;
            let a4909 = {
                let a4906 = match v1.clone() {
                    Adt3::C0(h4889, _, _, _, _, _, _, _) => {
                        h4889
                    }
                };
                let a4907 = {
                    let a4904 = {
                        let a4890 = v8;
                        let a4891 = v4;
                        let a4892 = 0u64;
                        nat_rem(a4890, a4891, a4892)
                    };
                    let a4905 = {
                        let a4902 = match v1.clone() {
                            Adt3::C0(_, h4893, _, _, _, _, _, _) => {
                                h4893
                            }
                        };
                        let a4903 = {
                            let a4900 = {
                                let a4894 = v10;
                                let a4895 = v4;
                                let a4896 = 0u64;
                                nat_rem(a4894, a4895, a4896)
                            };
                            let a4901 = {
                                let a4898 = match v1.clone() {
                                    Adt3::C0(_, _, h4897, _, _, _, _, _) => {
                                        h4897
                                    }
                                };
                                let a4899 = v13;
                                nat_mul(a4898, a4899)?
                            };
                            nat_add(a4900, a4901)?
                        };
                        nat_mul(a4902, a4903)?
                    };
                    nat_add(a4904, a4905)?
                };
                nat_mul(a4906, a4907)?
            };
            nat_add(a4908, a4909)?
        };
        Ok(Adt9::C1(false, f93(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4915 = {
                let a4913 = {
                    let a4910 = v11;
                    let a4911 = v5;
                    let a4912 = 0u64;
                    nat_rem(a4910, a4911, a4912)
                };
                let a4914 = v4;
                nat_mul(a4913, a4914)?
            };
            let a4916 = v8;
            nat_add(a4915, a4916)?
        };
        let v16: u64 = {
            let a4922 = {
                let a4920 = {
                    let a4917 = v11;
                    let a4918 = v5;
                    let a4919 = 0u64;
                    nat_quot(a4917, a4918, a4919)
                };
                let a4921 = v4;
                nat_mul(a4920, a4921)?
            };
            let a4923 = v10;
            nat_add(a4922, a4923)?
        };
        let a4925 = v15;
        let a4926 = match v1.clone() {
            Adt3::C0(_, h4924, _, _, _, _, _, _) => {
                h4924
            }
        };
        if if nat_lt(a4925, a4926) {
            let a4928 = v16;
            let a4929 = match v1.clone() {
                Adt3::C0(_, _, h4927, _, _, _, _, _) => {
                    h4927
                }
            };
            nat_lt(a4928, a4929)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f93({
                let a4938 = v6;
                let a4939 = {
                    let a4936 = match v1.clone() {
                        Adt3::C0(h4930, _, _, _, _, _, _, _) => {
                            h4930
                        }
                    };
                    let a4937 = {
                        let a4934 = v15;
                        let a4935 = {
                            let a4932 = match v1.clone() {
                                Adt3::C0(_, h4931, _, _, _, _, _, _) => {
                                    h4931
                                }
                            };
                            let a4933 = v16;
                            nat_mul(a4932, a4933)?
                        };
                        nat_add(a4934, a4935)?
                    };
                    nat_mul(a4936, a4937)?
                };
                nat_add(a4938, a4939)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f240(23u64, v4);
    let v6: u64 = {
        let a4940 = v0;
        let a4941 = v5;
        let a4942 = 0u64;
        nat_rem(a4940, a4941, a4942)
    };
    let v7: u64 = {
        let a4943 = v0;
        let a4944 = v5;
        let a4945 = 0u64;
        nat_quot(a4943, a4944, a4945)
    };
    let v8: Adt13 = Adt13::C1(f93(v7, v2)?);
    let a4946 = v6;
    let a4947 = 0u64;
    if nat_eq(a4946, a4947) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f93(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f178(v3), 4u64))
    } else {
        let a4948 = v6;
        let a4949 = 1u64;
        if nat_eq(a4948, a4949) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4950 = v4;
                let a4951 = 0u64;
                if nat_eq(a4950, a4951) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f178(v4)))
        }
    }
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f93(v0, v2.clone())?;
    let v4: u64 = f93(v0, v1.clone())?;
    let a4956 = {
        let a4953 = v0;
        let a4954 = match v1 {
            Adt3::C0(h4952, _, _, _, _, _, _, _) => {
                h4952
            }
        };
        let a4955 = 0u64;
        nat_rem(a4953, a4954, a4955)
    };
    let a4957 = 0u64;
    if nat_eq(a4956, a4957) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f93({
            let a4958 = v0;
            let a4959 = 1u64;
            nat_sub(a4958, a4959)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4960 = v3;
    let a4961 = 13u64;
    if nat_eq(a4960, a4961) {
        f249(v0, v1, v4)
    } else {
        let v5: u64 = f250(match v1.clone() {
            Adt3::C0(h4962, _, _, _, _, _, _, _) => {
                h4962
            }
        }, {
            let a4963 = v3;
            let a4964 = 16u64;
            nat_eq(a4963, a4964)
        })?;
        let v6: u64 = {
            let a4965 = v0;
            let a4966 = v5;
            let a4967 = 0u64;
            nat_quot(a4965, a4966, a4967)
        };
        let v7: u64 = {
            let a4968 = v0;
            let a4969 = v5;
            let a4970 = 0u64;
            nat_rem(a4968, a4969, a4970)
        };
        let v8: u64 = {
            let a4974 = 1u64;
            let a4975 = {
                let a4972 = match v1.clone() {
                    Adt3::C0(h4971, _, _, _, _, _, _, _) => {
                        h4971
                    }
                };
                let a4973 = 2u64;
                nat_mul(a4972, a4973)?
            };
            nat_add(a4974, a4975)?
        };
        let a4976 = v7;
        let a4977 = v8;
        if nat_lt(a4976, a4977) {
            f251(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f93(v6, v2)?;
            let a4978 = v3;
            let a4979 = 14u64;
            if nat_eq(a4978, a4979) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4980 = v7;
                let a4981 = v8;
                if nat_eq(a4980, a4981) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4984 = v7;
                    let a4985 = {
                        let a4982 = v8;
                        let a4983 = 1u64;
                        nat_add(a4982, a4983)?
                    };
                    if nat_eq(a4984, a4985) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4986, _, _, _, _, _, _, _) => {
                                h4986
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

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4987 = v0;
    let a4988 = 0u64;
    if nat_eq(a4987, a4988) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4995 = {
            let a4989 = v0;
            let a4990 = 1u64;
            nat_add(a4989, a4990)?
        };
        let a4996 = {
            let a4993 = 2u64;
            let a4994 = {
                let a4991 = v3;
                let a4992 = 2u64;
                nat_mul(a4991, a4992)?
            };
            nat_add(a4993, a4994)?
        };
        if nat_eq(a4995, a4996) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4997 = v0;
                let a4998 = 1u64;
                nat_sub(a4997, a4998)
            };
            let v5: u64 = {
                let a4999 = v4;
                let a5000 = 2u64;
                let a5001 = 0u64;
                nat_quot(a4999, a5000, a5001)
            };
            let a5005 = {
                let a5002 = v4;
                let a5003 = 2u64;
                let a5004 = 0u64;
                nat_rem(a5002, a5003, a5004)
            };
            let a5006 = 0u64;
            if nat_eq(a5005, a5006) {
                Ok(Adt9::C4(17u64, 0u64, f93(v5, v1)?, 4u64, f93(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f167(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a5007 = v0;
    let a5008 = f41(v1, v2, v3)?;
    if nat_lt(a5007, a5008) {
        let a5009 = v0;
        let a5010 = v1;
        if nat_lt(a5009, a5010) {
            Ok(Adt9::C1(false, {
                let a5011 = 4u64;
                let a5012 = v0;
                nat_mul(a5011, a5012)?
            }, {
                let a5013 = 4u64;
                let a5014 = v0;
                nat_mul(a5013, a5014)?
            }))
        } else {
            let v4: u64 = {
                let a5015 = v0;
                let a5016 = v1;
                nat_sub(a5015, a5016)
            };
            let v5: u64 = f252(v4, v2)?;
            let a5017 = v5;
            let a5018 = 2u64;
            if nat_lt(a5017, a5018) {
                Ok(Adt9::C4(6u64, {
                    let a5019 = v5;
                    let a5020 = 0u64;
                    if nat_eq(a5019, a5020) {
                        4u64
                    } else {
                        5u64
                    }
                }, f253(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f254(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f255(v4, v2)? {
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

pub fn f168(v0: u64, v1: u64) -> u64 {
    let a5021 = v0;
    let a5022 = 0u64;
    if if nat_eq(a5021, a5022) {
        true
    } else {
        let a5023 = v1;
        let a5024 = 0u64;
        nat_eq(a5023, a5024)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f169(v0: u64, v1: u64) -> Adt11 {
    let a5025 = v0;
    let a5026 = 43u64;
    if nat_eq(a5025, a5026) {
        Adt11::C8(3u64)
    } else {
        let a5027 = v1;
        let a5028 = 43u64;
        if nat_eq(a5027, a5028) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f170(v0: u64) -> R<u64> {
    let a5031 = 2u64;
    let a5032 = {
        let a5029 = 6u64;
        let a5030 = v0;
        nat_mul(a5029, a5030)?
    };
    nat_add(a5031, a5032)
}

pub fn f171(v0: u64, v1: bool) -> R<u64> {
    let a5035 = {
        let a5033 = v0;
        let a5034 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a5033, a5034)?
    };
    let a5036 = 1u64;
    nat_add(a5035, a5036)
}

pub fn f172(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a5037 = v0;
    let a5038 = 16u64;
    if nat_lt(a5037, a5038) {
        let v9: u64 = {
            let a5039 = v0;
            let a5040 = 4u64;
            let a5041 = 0u64;
            nat_quot(a5039, a5040, a5041)
        };
        let v10: u64 = {
            let a5046 = {
                let a5044 = 8u64;
                let a5045 = {
                    let a5042 = 4u64;
                    let a5043 = v1;
                    nat_mul(a5042, a5043)?
                };
                nat_add(a5044, a5045)?
            };
            let a5047 = v9;
            nat_add(a5046, a5047)?
        };
        let v11: u64 = f57(v6, v9, v1, v2, 0u64)?;
        let a5051 = {
            let a5048 = v0;
            let a5049 = 4u64;
            let a5050 = 0u64;
            nat_rem(a5048, a5049, a5050)
        };
        let a5052 = 0u64;
        if nat_eq(a5051, a5052) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f57(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f57(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a5056 = {
                let a5053 = v0;
                let a5054 = 4u64;
                let a5055 = 0u64;
                nat_rem(a5053, a5054, a5055)
            };
            let a5057 = 1u64;
            if nat_eq(a5056, a5057) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f57(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a5061 = {
                    let a5058 = v0;
                    let a5059 = 4u64;
                    let a5060 = 0u64;
                    nat_rem(a5058, a5059, a5060)
                };
                let a5062 = 2u64;
                if nat_eq(a5061, a5062) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a5063 = v9;
                        let a5064 = 0u64;
                        if nat_eq(a5063, a5064) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a5065 = v8;
                        let a5066 = 2u64;
                        if nat_eq(a5065, a5066) {
                            let a5067 = v9;
                            let a5068 = 0u64;
                            if nat_eq(a5067, a5068) {
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
        let a5069 = v0;
        let a5070 = 28u64;
        if nat_lt(a5069, a5070) {
            let v12: u64 = {
                let a5071 = v0;
                let a5072 = 16u64;
                nat_sub(a5071, a5072)
            };
            let v13: u64 = {
                let a5073 = v12;
                let a5074 = 3u64;
                let a5075 = 0u64;
                nat_quot(a5073, a5074, a5075)
            };
            let v14: u64 = f57(v6, v13, v1, v2, 0u64)?;
            let a5079 = {
                let a5076 = v12;
                let a5077 = 3u64;
                let a5078 = 0u64;
                nat_rem(a5076, a5077, a5078)
            };
            let a5080 = 0u64;
            if nat_eq(a5079, a5080) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a5084 = {
                    let a5081 = v12;
                    let a5082 = 3u64;
                    let a5083 = 0u64;
                    nat_rem(a5081, a5082, a5083)
                };
                let a5085 = 1u64;
                if nat_eq(a5084, a5085) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a5086 = v13;
                        let a5087 = 0u64;
                        if nat_eq(a5086, a5087) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a5088 = v0;
            let a5089 = 28u64;
            if nat_eq(a5088, a5089) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a5090 = v0;
                    let a5091 = 29u64;
                    nat_sub(a5090, a5091)
                };
                let v16: u64 = {
                    let a5092 = v15;
                    let a5093 = 2u64;
                    let a5094 = 0u64;
                    nat_quot(a5092, a5093, a5094)
                };
                let v17: u64 = f57(v6, v16, v1, v2, 0u64)?;
                let a5098 = {
                    let a5095 = v15;
                    let a5096 = 2u64;
                    let a5097 = 0u64;
                    nat_rem(a5095, a5096, a5097)
                };
                let a5099 = 0u64;
                if nat_eq(a5098, a5099) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f173(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5100 = v0;
        let a5101 = 9u64;
        let a5102 = 0u64;
        nat_rem(a5100, a5101, a5102)
    };
    let v5: u64 = {
        let a5106 = {
            let a5103 = v0;
            let a5104 = 9u64;
            let a5105 = 0u64;
            nat_quot(a5103, a5104, a5105)
        };
        let a5107 = 4u64;
        let a5108 = 0u64;
        nat_rem(a5106, a5107, a5108)
    };
    let v6: bool = {
        let a5115 = {
            let a5112 = {
                let a5109 = v0;
                let a5110 = 36u64;
                let a5111 = 0u64;
                nat_quot(a5109, a5110, a5111)
            };
            let a5113 = 2u64;
            let a5114 = 0u64;
            nat_rem(a5112, a5113, a5114)
        };
        let a5116 = 0u64;
        nat_eq(a5115, a5116)
    };
    let v7: u64 = {
        let a5117 = v4;
        let a5118 = 4u64;
        if nat_lt(a5117, a5118) {
            v4
        } else {
            let a5119 = v4;
            let a5120 = 5u64;
            nat_sub(a5119, a5120)
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
    let a5121 = v4;
    let a5122 = 4u64;
    if nat_lt(a5121, a5122) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a5123 = v4;
            let a5124 = 0u64;
            if nat_eq(a5123, a5124) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a5125 = v4;
        let a5126 = 4u64;
        if nat_eq(a5125, a5126) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f174(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5129 = {
            let a5127 = 3u64;
            let a5128 = v1;
            nat_mul(a5127, a5128)?
        };
        let a5130 = 4u64;
        nat_add(a5129, a5130)
    } else {
        let a5131 = v1;
        let a5132 = 3u64;
        nat_add(a5131, a5132)
    }
}

pub fn f175(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5141 = f60(match v0.clone() {
        Adt7::C0(h5133, _, _, _, _) => {
            h5133
        }
    }, match v1.clone() {
        Adt3::C0(h5134, _, _, _, _, _, _, _) => {
            h5134
        }
    }, f58(v3, v2.clone(), 0u64), {
        let a5138 = v4;
        let a5139 = {
            let m5137 = match v0.clone() {
                Adt7::C0(h5136, _, _, _, _) => {
                    h5136
                }
            };
            match m5137 {
                Adt6::C0(h5135, _, _, _) => {
                    h5135
                }
            }
        };
        let a5140 = 0u64;
        nat_rem(a5138, a5139, a5140)
    })?;
    match m5141 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5150 = f60(match v0.clone() {
                Adt7::C0(_, h5142, _, _, _) => {
                    h5142
                }
            }, match v1.clone() {
                Adt3::C0(_, h5143, _, _, _, _, _, _) => {
                    h5143
                }
            }, f58(v3, v2.clone(), 1u64), {
                let a5147 = v4;
                let a5148 = {
                    let m5146 = match v0 {
                        Adt7::C0(h5145, _, _, _, _) => {
                            h5145
                        }
                    };
                    match m5146 {
                        Adt6::C0(h5144, _, _, _) => {
                            h5144
                        }
                    }
                };
                let a5149 = 0u64;
                nat_quot(a5147, a5148, a5149)
            })?;
            match m5150 {
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

pub fn f176(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5152 = v2;
        let a5153 = f146(match v0.clone() {
            Adt6::C0(_, _, h5151, _) => {
                h5151
            }
        });
        nat_add(a5152, a5153)?
    };
    let v5: u64 = {
        let a5158 = {
            let a5155 = v3;
            let a5156 = match v0.clone() {
                Adt6::C0(_, _, _, h5154) => {
                    h5154
                }
            };
            nat_mul(a5155, a5156)?
        };
        let a5159 = f145(match v0.clone() {
            Adt6::C0(_, _, h5157, _) => {
                h5157
            }
        });
        nat_add(a5158, a5159)?
    };
    let v6: u64 = {
        let a5160 = v4;
        let a5161 = v5;
        nat_sub(a5160, a5161)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5162, _, _) => {
            h5162
        }
    };
    let v8: u64 = {
        let a5163 = v6;
        let a5164 = v7;
        let a5165 = 0u64;
        nat_quot(a5163, a5164, a5165)
    };
    let a5166 = v5;
    let a5167 = v4;
    if if if nat_le(a5166, a5167) {
        let a5171 = {
            let a5168 = v6;
            let a5169 = v7;
            let a5170 = 0u64;
            nat_rem(a5168, a5169, a5170)
        };
        let a5172 = 0u64;
        nat_eq(a5171, a5172)
    } else {
        false
    } {
        let a5173 = v8;
        let a5174 = v1;
        nat_lt(a5173, a5174)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f177(v0: u64) -> R<u64> {
    let a5177 = v0;
    let a5178 = {
        let a5175 = v0;
        let a5176 = 2u64;
        nat_add(a5175, a5176)?
    };
    nat_mul(a5177, a5178)
}

pub fn f178(v0: u64) -> u64 {
    let m5179 = f218(v0);
    match m5179 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5181 = match v1.clone() {
                Adt19::C0(_, h5180, _) => {
                    h5180
                }
            };
            let a5182 = 1u64;
            if if nat_eq(a5181, a5182) {
                let a5184 = match v1.clone() {
                    Adt19::C0(_, _, h5183) => {
                        h5183
                    }
                };
                let a5185 = 1u64;
                if if if nat_eq(a5184, a5185) {
                    true
                } else {
                    let a5187 = match v1.clone() {
                        Adt19::C0(_, _, h5186) => {
                            h5186
                        }
                    };
                    let a5188 = 2u64;
                    nat_eq(a5187, a5188)
                } {
                    true
                } else {
                    let a5190 = match v1.clone() {
                        Adt19::C0(_, _, h5189) => {
                            h5189
                        }
                    };
                    let a5191 = 4u64;
                    nat_eq(a5190, a5191)
                } {
                    true
                } else {
                    let a5193 = match v1.clone() {
                        Adt19::C0(_, _, h5192) => {
                            h5192
                        }
                    };
                    let a5194 = 8u64;
                    nat_eq(a5193, a5194)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h5195) => {
                        h5195
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f179(v0: u64) -> R<u64> {
    let a5198 = 3u64;
    let a5199 = {
        let a5196 = v0;
        let a5197 = 3u64;
        nat_mul(a5196, a5197)?
    };
    nat_add(a5198, a5199)
}

pub fn f180(v0: u64, v1: u64) -> R<u64> {
    if f181(v0) {
        let a5200 = 2u64;
        let a5201 = v1;
        nat_mul(a5200, a5201)
    } else {
        let a5202 = v1;
        let a5203 = f256(v0);
        let a5204 = 0u64;
        Ok(nat_quot(a5202, a5203, a5204))
    }
}

pub fn f181(v0: u64) -> bool {
    let a5205 = v0;
    let a5206 = 0u64;
    if if nat_eq(a5205, a5206) {
        true
    } else {
        let a5207 = v0;
        let a5208 = 1u64;
        nat_eq(a5207, a5208)
    } {
        true
    } else {
        let a5209 = v0;
        let a5210 = 30u64;
        nat_eq(a5209, a5210)
    }
}

pub fn f182(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m5211 = f218(v0);
    match m5211 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f144(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h5212, _, _, _) => {
                    h5212
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h5213, _, _) => {
                    h5213
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h5214, _) => {
                    h5214
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h5215) => {
                    h5215
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h5216) => {
                    h5216
                }
            }, f144({
                let a5218 = v2;
                let a5219 = match v6.clone() {
                    Adt19::C0(_, h5217, _) => {
                        h5217
                    }
                };
                nat_mul(a5218, a5219)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h5220, _, _, _) => {
                    h5220
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h5221, _, _) => {
                    h5221
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h5222, _) => {
                    h5222
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h5223) => {
                    h5223
                }
            })?, match v6 {
                Adt19::C0(_, h5224, _) => {
                    h5224
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h5225, _, _, _) => {
                    h5225
                }
            }))
        }
    }
}

pub fn f183(v0: Adt3, v1: Adt3) -> R<u64> {
    let a5233 = {
        let a5226 = 2u64;
        let a5227 = f184(v1.clone())?;
        nat_mul(a5226, a5227)?
    };
    let a5234 = {
        let a5231 = f184(v1)?;
        let a5232 = {
            let a5229 = 1u64;
            let a5230 = match v0 {
                Adt3::C0(h5228, _, _, _, _, _, _, _) => {
                    h5228
                }
            };
            nat_add(a5229, a5230)?
        };
        nat_mul(a5231, a5232)?
    };
    nat_add(a5233, a5234)
}

pub fn f184(v0: Adt3) -> R<u64> {
    if f48(match v0.clone() {
        Adt3::C0(h5235, _, _, _, _, _, _, _) => {
            h5235
        }
    }, match v0.clone() {
        Adt3::C0(_, h5236, _, _, _, _, _, _) => {
            h5236
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h5237, _, _, _, _, _) => {
            h5237
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h5238, _, _, _, _) => {
            h5238
        }
    }) {
        Ok(0u64)
    } else {
        let a5247 = match v0.clone() {
            Adt3::C0(h5239, _, _, _, _, _, _, _) => {
                h5239
            }
        };
        let a5248 = {
            let a5245 = match v0.clone() {
                Adt3::C0(_, h5240, _, _, _, _, _, _) => {
                    h5240
                }
            };
            let a5246 = {
                let a5243 = match v0.clone() {
                    Adt3::C0(_, _, h5241, _, _, _, _, _) => {
                        h5241
                    }
                };
                let a5244 = match v0 {
                    Adt3::C0(_, _, _, h5242, _, _, _, _) => {
                        h5242
                    }
                };
                nat_mul(a5243, a5244)?
            };
            nat_mul(a5245, a5246)?
        };
        nat_mul(a5247, a5248)
    }
}

pub fn f185(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f93(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f186(v0: u64, v1: u64) -> bool {
    let a5249 = v1;
    let a5250 = 2147483647u64;
    if nat_le(a5249, a5250) {
        let a5251 = v1;
        let a5252 = v0;
        nat_lt(a5251, a5252)
    } else {
        false
    }
}

pub fn f187(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5253 = f218(v0);
    match m5253 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5255 = v2;
                let a5256 = match v7.clone() {
                    Adt19::C0(_, h5254, _) => {
                        h5254
                    }
                };
                nat_mul(a5255, a5256)?
            };
            let m5269 = f257(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5257, _, _, _, _, _, _, _) => {
                    h5257
                }
            }, match v5.clone() {
                Adt3::C0(_, h5258, _, _, _, _, _, _) => {
                    h5258
                }
            }, match v5 {
                Adt3::C0(_, _, h5259, _, _, _, _, _) => {
                    h5259
                }
            }, {
                let a5262 = match v3.clone() {
                    Adt3::C0(h5260, _, _, _, _, _, _, _) => {
                        h5260
                    }
                };
                let a5263 = match v7.clone() {
                    Adt19::C0(_, h5261, _) => {
                        h5261
                    }
                };
                nat_mul(a5262, a5263)?
            }, match v3.clone() {
                Adt3::C0(_, h5264, _, _, _, _, _, _) => {
                    h5264
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5265, _, _, _) => {
                    h5265
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5266, _, _) => {
                    h5266
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5267, _) => {
                    h5267
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5268) => {
                    h5268
                }
            })?;
            match m5269 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5270 = v0;
                    let a5271 = 0u64;
                    if if nat_eq(a5270, a5271) {
                        true
                    } else {
                        let a5272 = v0;
                        let a5273 = 26u64;
                        nat_eq(a5272, a5273)
                    } {
                        Ok(Adt9::C1(false, v9, f93({
                            let a5277 = {
                                let a5275 = v1;
                                let a5276 = match v4.clone() {
                                    Adt3::C0(h5274, _, _, _, _, _, _, _) => {
                                        h5274
                                    }
                                };
                                nat_mul(a5275, a5276)?
                            };
                            let a5278 = v8;
                            nat_add(a5277, a5278)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5279) => {
                                h5279
                            }
                        }, f93({
                            let a5283 = {
                                let a5281 = v1;
                                let a5282 = match v4.clone() {
                                    Adt3::C0(h5280, _, _, _, _, _, _, _) => {
                                        h5280
                                    }
                                };
                                nat_mul(a5281, a5282)?
                            };
                            let a5284 = v8;
                            nat_add(a5283, a5284)?
                        }, v4.clone())?, {
                            let a5291 = {
                                let a5289 = {
                                    let a5286 = match v7 {
                                        Adt19::C0(_, h5285, _) => {
                                            h5285
                                        }
                                    };
                                    let a5287 = 1u64;
                                    nat_sub(a5286, a5287)
                                };
                                let a5290 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5288, _, _, _) => {
                                        h5288
                                    }
                                };
                                nat_mul(a5289, a5290)?
                            };
                            let a5292 = 4u64;
                            nat_add(a5291, a5292)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5293, _, _, _) => {
                                h5293
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f188(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5294 = v1;
    let a5295 = 0u64;
    if if nat_eq(a5294, a5295) {
        true
    } else {
        let a5296 = v2;
        let a5297 = 0u64;
        nat_eq(a5296, a5297)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f189(v0: u64, v1: u64) -> bool {
    let a5298 = v0;
    let a5299 = 4294967295u64;
    if nat_eq(a5298, a5299) {
        true
    } else {
        let a5300 = v0;
        let a5301 = 2147483648u64;
        if nat_lt(a5300, a5301) {
            let a5302 = v0;
            let a5303 = v1;
            nat_lt(a5302, a5303)
        } else {
            false
        }
    }
}

pub fn f190(v0: Adt7) -> R<u64> {
    let a5315 = if match v0.clone() {
        Adt7::C0(_, _, _, h5304, _) => {
            h5304
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5316 = {
        let a5313 = {
            let a5311 = {
                let m5307 = match v0.clone() {
                    Adt7::C0(h5306, _, _, _, _) => {
                        h5306
                    }
                };
                match m5307 {
                    Adt6::C0(h5305, _, _, _) => {
                        h5305
                    }
                }
            };
            let a5312 = {
                let m5310 = match v0.clone() {
                    Adt7::C0(_, h5309, _, _, _) => {
                        h5309
                    }
                };
                match m5310 {
                    Adt6::C0(h5308, _, _, _) => {
                        h5308
                    }
                }
            };
            nat_mul(a5311, a5312)?
        };
        let a5314 = f191(v0)?;
        nat_mul(a5313, a5314)?
    };
    nat_add(a5315, a5316)
}

pub fn f191(v0: Adt7) -> R<u64> {
    let a5321 = if match v0.clone() {
        Adt7::C0(_, _, _, h5317, _) => {
            h5317
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5322 = {
        let a5319 = match v0 {
            Adt7::C0(_, _, _, _, h5318) => {
                h5318
            }
        };
        let a5320 = 1u64;
        if nat_eq(a5319, a5320) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5321, a5322)
}

pub fn f192(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5327 = {
            let a5324 = v2;
            let a5325 = match v0.clone() {
                Adt6::C0(_, h5323, _, _) => {
                    h5323
                }
            };
            nat_mul(a5324, a5325)?
        };
        let a5328 = f145(match v0.clone() {
            Adt6::C0(_, _, h5326, _) => {
                h5326
            }
        });
        nat_add(a5327, a5328)?
    };
    let v4: u64 = f146(match v0.clone() {
        Adt6::C0(_, _, h5329, _) => {
            h5329
        }
    });
    let v5: u64 = {
        let a5333 = {
            let a5331 = v3;
            let a5332 = match v0 {
                Adt6::C0(h5330, _, _, _) => {
                    h5330
                }
            };
            nat_add(a5331, a5332)?
        };
        let a5334 = v4;
        nat_sub(a5333, a5334)
    };
    let a5339 = {
        let a5335 = v5;
        let a5336 = v1;
        if nat_lt(a5335, a5336) {
            v5
        } else {
            v1
        }
    };
    let a5340 = {
        let a5337 = v3;
        let a5338 = v4;
        nat_sub(a5337, a5338)
    };
    Ok(nat_sub(a5339, a5340))
}

pub fn f193(v0: u64) -> R<u64> {
    let a5343 = 2u64;
    let a5344 = {
        let a5341 = v0;
        let a5342 = 2u64;
        nat_mul(a5341, a5342)?
    };
    nat_add(a5343, a5344)
}

pub fn f194(v0: Adt6) -> R<u64> {
    let a5357 = 2u64;
    let a5358 = {
        let a5355 = 2u64;
        let a5356 = {
            let a5352 = {
                let a5349 = match v0.clone() {
                    Adt6::C0(h5345, _, _, _) => {
                        h5345
                    }
                };
                let a5350 = {
                    let a5347 = match v0.clone() {
                        Adt6::C0(_, h5346, _, _) => {
                            h5346
                        }
                    };
                    let a5348 = 1u64;
                    nat_sub(a5347, a5348)
                };
                nat_add(a5349, a5350)?
            };
            let a5353 = match v0 {
                Adt6::C0(_, h5351, _, _) => {
                    h5351
                }
            };
            let a5354 = 0u64;
            nat_quot(a5352, a5353, a5354)
        };
        nat_mul(a5355, a5356)?
    };
    nat_add(a5357, a5358)
}

pub fn f195(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5359 = v0;
    match m5359 {
        Adt16::C0(v19) => {
            f258(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5360 = v20;
            let a5361 = 66u64;
            if nat_eq(a5360, a5361) {
                f259(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f260(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f196(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5368 = v3;
        let a5369 = {
            let a5365 = f93({
                let a5362 = v0;
                let a5363 = 2u64;
                let a5364 = 0u64;
                nat_quot(a5362, a5363, a5364)
            }, v1)?;
            let a5366 = v2;
            let a5367 = 0u64;
            nat_quot(a5365, a5366, a5367)
        };
        nat_add(a5368, a5369)?
    };
    let a5373 = {
        let a5370 = v0;
        let a5371 = 2u64;
        let a5372 = 0u64;
        nat_rem(a5370, a5371, a5372)
    };
    let a5374 = 0u64;
    if nat_eq(a5373, a5374) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5375 = v4;
        let a5376 = 0u64;
        if nat_eq(a5375, a5376) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f197(v0: u64, v1: u64) -> R<u64> {
    let a5379 = 8u64;
    let a5380 = {
        let a5377 = f261(v0);
        let a5378 = v1;
        nat_mul(a5377, a5378)?
    };
    nat_add(a5379, a5380)
}

pub fn f198(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5381, _, _, _, _, _, _, _) => {
            h5381
        }
    };
    let v7: u64 = f261(v3);
    let v8: u64 = {
        let a5384 = 1u64;
        let a5385 = {
            let a5382 = v6;
            let a5383 = v7;
            nat_mul(a5382, a5383)?
        };
        nat_add(a5384, a5385)?
    };
    let a5386 = v0;
    let a5387 = 0u64;
    if nat_eq(a5386, a5387) {
        Ok(f262(0u64))
    } else {
        let a5388 = v0;
        let a5389 = v8;
        if nat_lt(a5388, a5389) {
            let v9: u64 = {
                let a5390 = v0;
                let a5391 = 1u64;
                nat_sub(a5390, a5391)
            };
            let v10: u64 = {
                let a5392 = v9;
                let a5393 = v7;
                let a5394 = 0u64;
                nat_rem(a5392, a5393, a5394)
            };
            let v11: u64 = f93({
                let a5400 = {
                    let a5395 = v1;
                    let a5396 = v6;
                    nat_mul(a5395, a5396)?
                };
                let a5401 = {
                    let a5397 = v9;
                    let a5398 = v7;
                    let a5399 = 0u64;
                    nat_quot(a5397, a5398, a5399)
                };
                nat_add(a5400, a5401)?
            }, v2)?;
            let a5402 = v3;
            let a5403 = 30u64;
            if nat_eq(a5402, a5403) {
                let a5404 = v10;
                let a5405 = 0u64;
                if nat_eq(a5404, a5405) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5406 = v10;
                    let a5407 = 1u64;
                    if nat_eq(a5406, a5407) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5408 = v10;
                        let a5409 = 2u64;
                        if nat_eq(a5408, a5409) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5410 = v10;
                let a5411 = 0u64;
                if nat_eq(a5410, a5411) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5412 = v10;
                    let a5413 = 1u64;
                    if nat_eq(a5412, a5413) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5414 = v0;
                let a5415 = v8;
                nat_sub(a5414, a5415)
            };
            let a5416 = v12;
            let a5417 = 0u64;
            if nat_eq(a5416, a5417) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5418 = v12;
                let a5419 = 1u64;
                if nat_eq(a5418, a5419) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5420 = v12;
                    let a5421 = 2u64;
                    if nat_eq(a5420, a5421) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5422 = v12;
                        let a5423 = 3u64;
                        if nat_eq(a5422, a5423) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5424 = v12;
                            let a5425 = 4u64;
                            if nat_eq(a5424, a5425) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5426 = v12;
                                let a5427 = 5u64;
                                if nat_eq(a5426, a5427) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5428 = v12;
                                    let a5429 = 6u64;
                                    if nat_eq(a5428, a5429) {
                                        if f263(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5430 = 3u64;
                                                let a5431 = v1;
                                                nat_add(a5430, a5431)?
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

pub fn f199(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5432 = v0;
        let a5433 = f104(v3);
        let a5434 = 0u64;
        nat_quot(a5432, a5433, a5434)
    };
    let v5: u64 = {
        let a5439 = 3u64;
        let a5440 = {
            let a5436 = v4;
            let a5437 = match v1.clone() {
                Adt3::C0(h5435, _, _, _, _, _, _, _) => {
                    h5435
                }
            };
            let a5438 = 0u64;
            nat_quot(a5436, a5437, a5438)
        };
        nat_add(a5439, a5440)?
    };
    let v6: u64 = f93(v4, v1)?;
    let v7: u64 = f93(v4, v2)?;
    let a5441 = v3;
    let a5442 = 30u64;
    if nat_eq(a5441, a5442) {
        let a5446 = {
            let a5443 = v0;
            let a5444 = 3u64;
            let a5445 = 0u64;
            nat_rem(a5443, a5444, a5445)
        };
        let a5447 = 0u64;
        if nat_eq(a5446, a5447) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5451 = {
                let a5448 = v0;
                let a5449 = 3u64;
                let a5450 = 0u64;
                nat_rem(a5448, a5449, a5450)
            };
            let a5452 = 1u64;
            if nat_eq(a5451, a5452) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f200(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5453 = v0;
        let a5454 = 8388608u64;
        let a5455 = 0u64;
        nat_quot(a5453, a5454, a5455)
    };
    let a5456 = v0;
    let a5457 = 2147483648u64;
    if if if nat_lt(a5456, a5457) {
        let a5458 = 127u64;
        let a5459 = v1;
        nat_le(a5458, a5459)
    } else {
        false
    } {
        let a5460 = v1;
        let a5461 = 190u64;
        nat_lt(a5460, a5461)
    } else {
        false
    } {
        let v2: u64 = {
            let a5465 = 8388608u64;
            let a5466 = {
                let a5462 = v0;
                let a5463 = 8388608u64;
                let a5464 = 0u64;
                nat_rem(a5462, a5463, a5464)
            };
            nat_add(a5465, a5466)?
        };
        let a5467 = v1;
        let a5468 = 150u64;
        if nat_lt(a5467, a5468) {
            let v3: u64 = f264({
                let a5469 = 150u64;
                let a5470 = v1;
                nat_sub(a5469, a5470)
            });
            let a5474 = {
                let a5471 = v2;
                let a5472 = v3;
                let a5473 = 0u64;
                nat_rem(a5471, a5472, a5473)
            };
            let a5475 = 0u64;
            if nat_eq(a5474, a5475) {
                let a5476 = v2;
                let a5477 = v3;
                let a5478 = 0u64;
                Ok(nat_quot(a5476, a5477, a5478))
            } else {
                Ok(0u64)
            }
        } else {
            let a5481 = f264({
                let a5479 = v1;
                let a5480 = 150u64;
                nat_sub(a5479, a5480)
            });
            let a5482 = v2;
            nat_mul(a5481, a5482)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f201(v0: Adt14, v1: Adt14) -> bool {
    let a5485 = match v0.clone() {
        Adt14::C0(_, h5483, _, _, _, _, _, _, _, _, _) => {
            h5483
        }
    };
    let a5486 = match v1.clone() {
        Adt14::C0(_, h5484, _, _, _, _, _, _, _, _, _) => {
            h5484
        }
    };
    if if if nat_eq(a5485, a5486) {
        let a5489 = match v0.clone() {
            Adt14::C0(_, _, h5487, _, _, _, _, _, _, _, _) => {
                h5487
            }
        };
        let a5490 = match v1.clone() {
            Adt14::C0(_, _, h5488, _, _, _, _, _, _, _, _) => {
                h5488
            }
        };
        nat_eq(a5489, a5490)
    } else {
        false
    } {
        let a5493 = match v0.clone() {
            Adt14::C0(_, _, _, h5491, _, _, _, _, _, _, _) => {
                h5491
            }
        };
        let a5494 = match v1.clone() {
            Adt14::C0(_, _, _, h5492, _, _, _, _, _, _, _) => {
                h5492
            }
        };
        nat_eq(a5493, a5494)
    } else {
        false
    } {
        let a5497 = match v0 {
            Adt14::C0(_, _, _, _, h5495, _, _, _, _, _, _) => {
                h5495
            }
        };
        let a5498 = match v1 {
            Adt14::C0(_, _, _, _, h5496, _, _, _, _, _, _) => {
                h5496
            }
        };
        nat_eq(a5497, a5498)
    } else {
        false
    }
}

pub fn f202(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5499 = v0;
    let a5500 = 0u64;
    if if if nat_eq(a5499, a5500) {
        true
    } else {
        let a5501 = v1;
        let a5502 = 0u64;
        nat_eq(a5501, a5502)
    } {
        true
    } else {
        let a5503 = v2;
        let a5504 = 0u64;
        nat_eq(a5503, a5504)
    } {
        true
    } else {
        let a5505 = v3;
        let a5506 = 0u64;
        nat_eq(a5505, a5506)
    }
}

pub fn f203(v0: Adt14, v1: Adt14) -> bool {
    if f209(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5507, _, _, _, _, _) => {
            h5507
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5508, _, _, _, _, _) => {
            h5508
        }
    }) {
        let a5510 = match v0 {
            Adt14::C0(_, h5509, _, _, _, _, _, _, _, _, _) => {
                h5509
            }
        };
        let a5511 = 2147483647u64;
        nat_le(a5510, a5511)
    } else {
        true
    }
}

pub fn f204(v0: Adt14) -> bool {
    let a5516 = {
        let a5513 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5512) => {
                h5512
            }
        };
        let a5514 = 4u64;
        let a5515 = 0u64;
        nat_rem(a5513, a5514, a5515)
    };
    let a5517 = 0u64;
    if if nat_eq(a5516, a5517) {
        f265(match v0.clone() {
            Adt14::C0(_, h5518, _, _, _, _, _, _, _, _, _) => {
                h5518
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5519, _, _, _, _, _) => {
                h5519
            }
        })
    } else {
        false
    } {
        f100(match v0.clone() {
            Adt14::C0(_, _, h5520, _, _, _, _, _, _, _, _) => {
                h5520
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5521, _, _, _, _, _, _, _) => {
                h5521
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5522, _, _, _, _, _, _) => {
                h5522
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5523, _, _, _, _) => {
                h5523
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5524, _, _, _) => {
                h5524
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5525, _, _) => {
                h5525
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f205(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5527 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5526, _) => {
            h5526
        }
    };
    let a5528 = 0u64;
    if nat_eq(a5527, a5528) {
        Ok(true)
    } else {
        let a5530 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5529, _) => {
                h5529
            }
        };
        let a5531 = 0u64;
        if nat_eq(a5530, a5531) {
            Ok(true)
        } else {
            let a5533 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5532) => {
                    h5532
                }
            };
            let a5534 = 0u64;
            if nat_eq(a5533, a5534) {
                Ok(true)
            } else {
                let a5536 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5535) => {
                        h5535
                    }
                };
                let a5537 = 0u64;
                if nat_eq(a5536, a5537) {
                    Ok(true)
                } else {
                    let a5543 = {
                        let a5540 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5538) => {
                                h5538
                            }
                        };
                        let a5541 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5539, _) => {
                                h5539
                            }
                        };
                        nat_add(a5540, a5541)?
                    };
                    let a5544 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5542) => {
                            h5542
                        }
                    };
                    if nat_le(a5543, a5544) {
                        Ok(true)
                    } else {
                        let a5550 = {
                            let a5547 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5545) => {
                                    h5545
                                }
                            };
                            let a5548 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5546, _) => {
                                    h5546
                                }
                            };
                            nat_add(a5547, a5548)?
                        };
                        let a5551 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5549) => {
                                h5549
                            }
                        };
                        Ok(nat_le(a5550, a5551))
                    }
                }
            }
        }
    }
}

pub fn f206(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f201(v0.clone(), v1.clone()) {
        let a5554 = match v0.clone() {
            Adt14::C0(h5552, _, _, _, _, _, _, _, _, _, _) => {
                h5552
            }
        };
        let a5555 = match v1.clone() {
            Adt14::C0(h5553, _, _, _, _, _, _, _, _, _, _) => {
                h5553
            }
        };
        nat_eq(a5554, a5555)
    } else {
        false
    } {
        let a5558 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5556) => {
                h5556
            }
        };
        let a5559 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5557) => {
                h5557
            }
        };
        nat_eq(a5558, a5559)
    } else {
        false
    } {
        let a5562 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5560, _, _, _, _, _) => {
                h5560
            }
        };
        let a5563 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5561, _, _, _, _, _) => {
                h5561
            }
        };
        nat_eq(a5562, a5563)
    } else {
        false
    } {
        let a5566 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5564, _, _, _, _) => {
                h5564
            }
        };
        let a5567 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5565, _, _, _, _) => {
                h5565
            }
        };
        nat_eq(a5566, a5567)
    } else {
        false
    } {
        let a5570 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5568, _, _, _) => {
                h5568
            }
        };
        let a5571 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5569, _, _, _) => {
                h5569
            }
        };
        nat_eq(a5570, a5571)
    } else {
        false
    } {
        let a5574 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5572, _, _) => {
                h5572
            }
        };
        let a5575 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5573, _, _) => {
                h5573
            }
        };
        nat_eq(a5574, a5575)
    } else {
        false
    }
}

pub fn f207(v0: Adt14) -> R<bool> {
    if f266(match v0.clone() {
        Adt14::C0(_, h5576, _, _, _, _, _, _, _, _, _) => {
            h5576
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5577, _, _, _, _, _) => {
            h5577
        }
    }) {
        f101(f267(match v0.clone() {
            Adt14::C0(_, h5578, _, _, _, _, _, _, _, _, _) => {
                h5578
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5579, _, _, _, _, _) => {
                h5579
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5580, _, _, _, _, _, _, _, _) => {
                h5580
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5581, _, _, _, _, _, _, _) => {
                h5581
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5582, _, _, _, _, _, _) => {
                h5582
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5583, _, _, _, _) => {
                h5583
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5584, _, _, _) => {
                h5584
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5585, _, _) => {
                h5585
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f208(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5586 = v1;
    let a5587 = 0u64;
    if if nat_eq(a5586, a5587) {
        true
    } else {
        let a5588 = v2;
        let a5589 = 0u64;
        nat_eq(a5588, a5589)
    } {
        Ok(0u64)
    } else {
        let a5592 = f102(v1, v2, v3)?;
        let a5593 = {
            let a5590 = v2;
            let a5591 = f268(v0, v1)?;
            nat_mul(a5590, a5591)?
        };
        nat_add(a5592, a5593)
    }
}

pub fn f209(v0: u64, v1: u64) -> bool {
    let a5594 = v0;
    let a5595 = 4u64;
    if nat_eq(a5594, a5595) {
        let a5596 = v1;
        let a5597 = 4u64;
        nat_eq(a5596, a5597)
    } else {
        false
    }
}

pub fn f210(v0: u64) -> R<u64> {
    let a5598 = 6u64;
    let a5599 = v0;
    nat_add(a5598, a5599)
}

pub fn f211(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5600, _, _, _, _, _, _, _, _, _, _) => {
            h5600
        }
    }, match v0.clone() {
        Adt14::C0(_, h5601, _, _, _, _, _, _, _, _, _) => {
            h5601
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5602, _, _, _, _, _, _, _, _) => {
            h5602
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5603, _, _, _, _, _, _, _) => {
            h5603
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5604, _, _, _, _, _, _) => {
            h5604
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5605, _, _, _, _) => {
            h5605
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5606, _, _, _) => {
            h5606
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5607, _, _) => {
            h5607
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5608, _) => {
            h5608
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5609) => {
            h5609
        }
    })
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5610 = v0;
    let a5611 = 2147483647u64;
    if if if nat_le(a5610, a5611) {
        let a5612 = 0u64;
        let a5613 = v3;
        if nat_lt(a5612, a5613) {
            let a5614 = v3;
            let a5615 = 2147483647u64;
            nat_le(a5614, a5615)
        } else {
            false
        }
    } else {
        false
    } {
        let a5620 = {
            let a5618 = {
                let a5616 = v2;
                let a5617 = v3;
                nat_add(a5616, a5617)?
            };
            let a5619 = 1u64;
            nat_sub(a5618, a5619)
        };
        let a5621 = 2147483647u64;
        nat_le(a5620, a5621)
    } else {
        false
    } {
        let a5626 = {
            let a5624 = {
                let a5622 = v0;
                let a5623 = v1;
                nat_mul(a5622, a5623)?
            };
            let a5625 = f269(v2, v3)?;
            nat_mul(a5624, a5625)?
        };
        let a5627 = 9223372036854775807u64;
        Ok(nat_le(a5626, a5627))
    } else {
        Ok(false)
    }
}

pub fn f213(v0: Adt14) -> R<u64> {
    let a5658 = {
        let a5650 = {
            let a5642 = {
                let a5634 = 4u64;
                let a5635 = {
                    let a5632 = {
                        let a5629 = match v0.clone() {
                            Adt14::C0(_, h5628, _, _, _, _, _, _, _, _, _) => {
                                h5628
                            }
                        };
                        let a5630 = 1u64;
                        nat_sub(a5629, a5630)
                    };
                    let a5633 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5631, _, _, _, _, _) => {
                            h5631
                        }
                    };
                    nat_mul(a5632, a5633)?
                };
                nat_add(a5634, a5635)?
            };
            let a5643 = {
                let a5640 = {
                    let a5637 = match v0.clone() {
                        Adt14::C0(_, _, h5636, _, _, _, _, _, _, _, _) => {
                            h5636
                        }
                    };
                    let a5638 = 1u64;
                    nat_sub(a5637, a5638)
                };
                let a5641 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5639, _, _, _, _) => {
                        h5639
                    }
                };
                nat_mul(a5640, a5641)?
            };
            nat_add(a5642, a5643)?
        };
        let a5651 = {
            let a5648 = {
                let a5645 = match v0.clone() {
                    Adt14::C0(_, _, _, h5644, _, _, _, _, _, _, _) => {
                        h5644
                    }
                };
                let a5646 = 1u64;
                nat_sub(a5645, a5646)
            };
            let a5649 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5647, _, _, _) => {
                    h5647
                }
            };
            nat_mul(a5648, a5649)?
        };
        nat_add(a5650, a5651)?
    };
    let a5659 = {
        let a5656 = {
            let a5653 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5652, _, _, _, _, _, _) => {
                    h5652
                }
            };
            let a5654 = 1u64;
            nat_sub(a5653, a5654)
        };
        let a5657 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5655, _, _) => {
                h5655
            }
        };
        nat_mul(a5656, a5657)?
    };
    nat_add(a5658, a5659)
}

pub fn f214(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5660 = v0;
    let a5661 = 0u64;
    if if if if nat_eq(a5660, a5661) {
        true
    } else {
        let a5662 = v1;
        let a5663 = 0u64;
        nat_eq(a5662, a5663)
    } {
        true
    } else {
        let a5664 = v2;
        let a5665 = 0u64;
        nat_eq(a5664, a5665)
    } {
        true
    } else {
        let a5666 = v3;
        let a5667 = 0u64;
        nat_eq(a5666, a5667)
    } {
        Ok(0u64)
    } else {
        let a5668 = f270(v0, v1, v2, v3, v5)?;
        let a5669 = f271(v0, v1, v2, v3, v4)?;
        nat_add(a5668, a5669)
    }
}

pub fn f215(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5670 = v0;
    let a5671 = 0u64;
    if if nat_eq(a5670, a5671) {
        true
    } else {
        let a5672 = v1;
        let a5673 = 0u64;
        nat_eq(a5672, a5673)
    } {
        Ok(0u64)
    } else {
        let a5676 = f102(v0, v1, v2)?;
        let a5677 = {
            let a5674 = v1;
            let a5675 = f272(v0)?;
            nat_mul(a5674, a5675)?
        };
        nat_add(a5676, a5677)
    }
}

pub fn f216(v0: u64, v1: u64, v2: u64) -> bool {
    let a5678 = v0;
    let a5679 = 1u64;
    if nat_le(a5678, a5679) {
        true
    } else {
        let a5683 = {
            let a5680 = v1;
            let a5681 = v2;
            let a5682 = 1u64;
            nat_rem(a5680, a5681, a5682)
        };
        let a5684 = 0u64;
        nat_eq(a5683, a5684)
    }
}

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5685 = v1;
    let a5686 = 1u64;
    if if if nat_le(a5685, a5686) {
        true
    } else {
        let a5687 = v0;
        let a5688 = v2;
        nat_le(a5687, a5688)
    } {
        let a5689 = v3;
        let a5690 = 1u64;
        if nat_le(a5689, a5690) {
            true
        } else {
            let a5697 = {
                let a5695 = v0;
                let a5696 = {
                    let a5693 = {
                        let a5691 = v1;
                        let a5692 = 1u64;
                        nat_sub(a5691, a5692)
                    };
                    let a5694 = v2;
                    nat_mul(a5693, a5694)?
                };
                nat_add(a5695, a5696)?
            };
            let a5698 = v4;
            nat_le(a5697, a5698)
        }
    } else {
        false
    } {
        let a5699 = v5;
        let a5700 = 1u64;
        if nat_le(a5699, a5700) {
            Ok(true)
        } else {
            let a5713 = {
                let a5711 = {
                    let a5705 = v0;
                    let a5706 = {
                        let a5703 = {
                            let a5701 = v1;
                            let a5702 = 1u64;
                            nat_sub(a5701, a5702)
                        };
                        let a5704 = v2;
                        nat_mul(a5703, a5704)?
                    };
                    nat_add(a5705, a5706)?
                };
                let a5712 = {
                    let a5709 = {
                        let a5707 = v3;
                        let a5708 = 1u64;
                        nat_sub(a5707, a5708)
                    };
                    let a5710 = v4;
                    nat_mul(a5709, a5710)?
                };
                nat_add(a5711, a5712)?
            };
            let a5714 = v6;
            Ok(nat_le(a5713, a5714))
        }
    } else {
        Ok(false)
    }
}

pub fn f218(v0: u64) -> Adt18 {
    let a5715 = v0;
    let a5716 = 0u64;
    if nat_eq(a5715, a5716) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5717 = v0;
        let a5718 = 1u64;
        if nat_eq(a5717, a5718) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5719 = v0;
            let a5720 = 2u64;
            if nat_eq(a5719, a5720) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5721 = v0;
                let a5722 = 3u64;
                if nat_eq(a5721, a5722) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5723 = v0;
                    let a5724 = 6u64;
                    if nat_eq(a5723, a5724) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5725 = v0;
                        let a5726 = 7u64;
                        if nat_eq(a5725, a5726) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5727 = v0;
                            let a5728 = 8u64;
                            if nat_eq(a5727, a5728) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5729 = v0;
                                let a5730 = 9u64;
                                if nat_eq(a5729, a5730) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5731 = v0;
                                    let a5732 = 10u64;
                                    if nat_eq(a5731, a5732) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5733 = v0;
                                        let a5734 = 11u64;
                                        if nat_eq(a5733, a5734) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5735 = v0;
                                            let a5736 = 12u64;
                                            if nat_eq(a5735, a5736) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5737 = v0;
                                                let a5738 = 13u64;
                                                if nat_eq(a5737, a5738) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5739 = v0;
                                                    let a5740 = 14u64;
                                                    if nat_eq(a5739, a5740) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5741 = v0;
                                                        let a5742 = 15u64;
                                                        if nat_eq(a5741, a5742) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5743 = v0;
                                                            let a5744 = 16u64;
                                                            if nat_eq(a5743, a5744) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5745 = v0;
                                                                let a5746 = 17u64;
                                                                if nat_eq(a5745, a5746) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5747 = v0;
                                                                    let a5748 = 18u64;
                                                                    if nat_eq(a5747, a5748) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5749 = v0;
                                                                        let a5750 = 19u64;
                                                                        if nat_eq(a5749, a5750) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5751 = v0;
                                                                            let a5752 = 20u64;
                                                                            if nat_eq(a5751, a5752) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5753 = v0;
                                                                                let a5754 = 21u64;
                                                                                if nat_eq(a5753, a5754) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5755 = v0;
                                                                                    let a5756 = 22u64;
                                                                                    if nat_eq(a5755, a5756) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5757 = v0;
                                                                                        let a5758 = 23u64;
                                                                                        if nat_eq(a5757, a5758) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5759 = v0;
                                                                                            let a5760 = 24u64;
                                                                                            if nat_eq(a5759, a5760) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5761 = v0;
                                                                                                let a5762 = 25u64;
                                                                                                if nat_eq(a5761, a5762) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5763 = v0;
                                                                                                    let a5764 = 26u64;
                                                                                                    if nat_eq(a5763, a5764) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5765 = v0;
                                                                                                        let a5766 = 27u64;
                                                                                                        if nat_eq(a5765, a5766) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5767 = v0;
                                                                                                            let a5768 = 28u64;
                                                                                                            if nat_eq(a5767, a5768) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5769 = v0;
                                                                                                                let a5770 = 29u64;
                                                                                                                if nat_eq(a5769, a5770) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5771 = v0;
                                                                                                                    let a5772 = 30u64;
                                                                                                                    if nat_eq(a5771, a5772) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5773 = v0;
                                                                                                                        let a5774 = 34u64;
                                                                                                                        if nat_eq(a5773, a5774) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5775 = v0;
                                                                                                                            let a5776 = 35u64;
                                                                                                                            if nat_eq(a5775, a5776) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5777 = v0;
                                                                                                                                let a5778 = 39u64;
                                                                                                                                if nat_eq(a5777, a5778) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5779 = v0;
                                                                                                                                    let a5780 = 40u64;
                                                                                                                                    if nat_eq(a5779, a5780) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5781 = v0;
                                                                                                                                        let a5782 = 41u64;
                                                                                                                                        if nat_eq(a5781, a5782) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5783 = v0;
                                                                                                                                            let a5784 = 42u64;
                                                                                                                                            if nat_eq(a5783, a5784) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5785 = v0;
                                                                                                                                                let a5786 = 43u64;
                                                                                                                                                if nat_eq(a5785, a5786) {
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

pub fn f219(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a5787 = v0;
        let a5788 = 2u64;
        let a5789 = 0u64;
        nat_quot(a5787, a5788, a5789)
    };
    let a5805 = {
        let a5790 = v2;
        let a5791 = 0u64;
        if nat_eq(a5790, a5791) {
            0u64
        } else {
            let a5798 = 7u64;
            let a5799 = {
                let a5796 = v2;
                let a5797 = {
                    let a5794 = 5u64;
                    let a5795 = {
                        let a5792 = 3u64;
                        let a5793 = v1;
                        nat_mul(a5792, a5793)?
                    };
                    nat_add(a5794, a5795)?
                };
                nat_mul(a5796, a5797)?
            };
            nat_add(a5798, a5799)?
        }
    };
    let a5806 = {
        let a5803 = {
            let a5800 = v0;
            let a5801 = 2u64;
            let a5802 = 0u64;
            nat_rem(a5800, a5801, a5802)
        };
        let a5804 = 1u64;
        if nat_eq(a5803, a5804) {
            v1
        } else {
            0u64
        }
    };
    nat_add(a5805, a5806)
}

pub fn f220(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5808 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5807, _, _, _, _, _, _) => {
                    h5807
                }
            };
            let a5809 = 1u64;
            if if if if if nat_le(a5808, a5809) {
                true
            } else {
                let a5811 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5810, _, _) => {
                        h5810
                    }
                };
                let a5812 = v1;
                nat_eq(a5811, a5812)
            } {
                let a5814 = match v0.clone() {
                    Adt2::C0(_, h5813, _, _, _, _, _, _, _, _, _) => {
                        h5813
                    }
                };
                let a5815 = 1u64;
                if nat_le(a5814, a5815) {
                    true
                } else {
                    let a5820 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5816, _, _, _, _, _) => {
                            h5816
                        }
                    };
                    let a5821 = {
                        let a5818 = v1;
                        let a5819 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5817, _, _, _, _, _, _) => {
                                h5817
                            }
                        };
                        nat_mul(a5818, a5819)?
                    };
                    nat_eq(a5820, a5821)
                }
            } else {
                false
            } {
                let a5823 = match v0.clone() {
                    Adt2::C0(_, _, h5822, _, _, _, _, _, _, _, _) => {
                        h5822
                    }
                };
                let a5824 = 1u64;
                if nat_le(a5823, a5824) {
                    true
                } else {
                    let a5832 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5825, _, _, _, _) => {
                            h5825
                        }
                    };
                    let a5833 = {
                        let a5830 = {
                            let a5827 = v1;
                            let a5828 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5826, _, _, _, _, _, _) => {
                                    h5826
                                }
                            };
                            nat_mul(a5827, a5828)?
                        };
                        let a5831 = match v0.clone() {
                            Adt2::C0(_, h5829, _, _, _, _, _, _, _, _, _) => {
                                h5829
                            }
                        };
                        nat_mul(a5830, a5831)?
                    };
                    nat_eq(a5832, a5833)
                }
            } else {
                false
            } {
                let a5835 = match v0.clone() {
                    Adt2::C0(_, _, _, h5834, _, _, _, _, _, _, _) => {
                        h5834
                    }
                };
                let a5836 = 1u64;
                if nat_le(a5835, a5836) {
                    true
                } else {
                    let a5847 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5837, _, _, _) => {
                            h5837
                        }
                    };
                    let a5848 = {
                        let a5845 = {
                            let a5842 = {
                                let a5839 = v1;
                                let a5840 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5838, _, _, _, _, _, _) => {
                                        h5838
                                    }
                                };
                                nat_mul(a5839, a5840)?
                            };
                            let a5843 = match v0.clone() {
                                Adt2::C0(_, h5841, _, _, _, _, _, _, _, _, _) => {
                                    h5841
                                }
                            };
                            nat_mul(a5842, a5843)?
                        };
                        let a5846 = match v0.clone() {
                            Adt2::C0(_, _, h5844, _, _, _, _, _, _, _, _) => {
                                h5844
                            }
                        };
                        nat_mul(a5845, a5846)?
                    };
                    nat_eq(a5847, a5848)
                }
            } else {
                false
            } {
                let a5862 = {
                    let a5859 = {
                        let a5856 = {
                            let a5853 = {
                                let a5850 = v1;
                                let a5851 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5849, _, _, _, _, _, _) => {
                                        h5849
                                    }
                                };
                                nat_mul(a5850, a5851)?
                            };
                            let a5854 = match v0.clone() {
                                Adt2::C0(_, h5852, _, _, _, _, _, _, _, _, _) => {
                                    h5852
                                }
                            };
                            nat_mul(a5853, a5854)?
                        };
                        let a5857 = match v0.clone() {
                            Adt2::C0(_, _, h5855, _, _, _, _, _, _, _, _) => {
                                h5855
                            }
                        };
                        nat_mul(a5856, a5857)?
                    };
                    let a5860 = match v0.clone() {
                        Adt2::C0(_, _, _, h5858, _, _, _, _, _, _, _) => {
                            h5858
                        }
                    };
                    nat_mul(a5859, a5860)?
                };
                let a5863 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5861, _) => {
                        h5861
                    }
                };
                Ok(nat_le(a5862, a5863))
            } else {
                Ok(false)
            }
        } else {
            let a5865 = match v0.clone() {
                Adt2::C0(_, _, _, h5864, _, _, _, _, _, _, _) => {
                    h5864
                }
            };
            let a5866 = 1u64;
            if if if if if nat_le(a5865, a5866) {
                true
            } else {
                let a5868 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5867, _, _, _) => {
                        h5867
                    }
                };
                let a5869 = v1;
                nat_eq(a5868, a5869)
            } {
                let a5871 = match v0.clone() {
                    Adt2::C0(_, h5870, _, _, _, _, _, _, _, _, _) => {
                        h5870
                    }
                };
                let a5872 = 1u64;
                if nat_le(a5871, a5872) {
                    true
                } else {
                    let a5877 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5873, _, _, _, _, _) => {
                            h5873
                        }
                    };
                    let a5878 = {
                        let a5875 = v1;
                        let a5876 = match v0.clone() {
                            Adt2::C0(_, _, _, h5874, _, _, _, _, _, _, _) => {
                                h5874
                            }
                        };
                        nat_mul(a5875, a5876)?
                    };
                    nat_eq(a5877, a5878)
                }
            } else {
                false
            } {
                let a5880 = match v0.clone() {
                    Adt2::C0(_, _, h5879, _, _, _, _, _, _, _, _) => {
                        h5879
                    }
                };
                let a5881 = 1u64;
                if nat_le(a5880, a5881) {
                    true
                } else {
                    let a5889 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5882, _, _, _, _) => {
                            h5882
                        }
                    };
                    let a5890 = {
                        let a5887 = {
                            let a5884 = v1;
                            let a5885 = match v0.clone() {
                                Adt2::C0(_, _, _, h5883, _, _, _, _, _, _, _) => {
                                    h5883
                                }
                            };
                            nat_mul(a5884, a5885)?
                        };
                        let a5888 = match v0.clone() {
                            Adt2::C0(_, h5886, _, _, _, _, _, _, _, _, _) => {
                                h5886
                            }
                        };
                        nat_mul(a5887, a5888)?
                    };
                    nat_eq(a5889, a5890)
                }
            } else {
                false
            } {
                let a5892 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5891, _, _, _, _, _, _) => {
                        h5891
                    }
                };
                let a5893 = 1u64;
                if nat_le(a5892, a5893) {
                    true
                } else {
                    let a5904 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5894, _, _) => {
                            h5894
                        }
                    };
                    let a5905 = {
                        let a5902 = {
                            let a5899 = {
                                let a5896 = v1;
                                let a5897 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5895, _, _, _, _, _, _, _) => {
                                        h5895
                                    }
                                };
                                nat_mul(a5896, a5897)?
                            };
                            let a5900 = match v0.clone() {
                                Adt2::C0(_, h5898, _, _, _, _, _, _, _, _, _) => {
                                    h5898
                                }
                            };
                            nat_mul(a5899, a5900)?
                        };
                        let a5903 = match v0.clone() {
                            Adt2::C0(_, _, h5901, _, _, _, _, _, _, _, _) => {
                                h5901
                            }
                        };
                        nat_mul(a5902, a5903)?
                    };
                    nat_eq(a5904, a5905)
                }
            } else {
                false
            } {
                let a5919 = {
                    let a5916 = {
                        let a5913 = {
                            let a5910 = {
                                let a5907 = v1;
                                let a5908 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5906, _, _, _, _, _, _, _) => {
                                        h5906
                                    }
                                };
                                nat_mul(a5907, a5908)?
                            };
                            let a5911 = match v0.clone() {
                                Adt2::C0(_, h5909, _, _, _, _, _, _, _, _, _) => {
                                    h5909
                                }
                            };
                            nat_mul(a5910, a5911)?
                        };
                        let a5914 = match v0.clone() {
                            Adt2::C0(_, _, h5912, _, _, _, _, _, _, _, _) => {
                                h5912
                            }
                        };
                        nat_mul(a5913, a5914)?
                    };
                    let a5917 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5915, _, _, _, _, _, _) => {
                            h5915
                        }
                    };
                    nat_mul(a5916, a5917)?
                };
                let a5920 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5918, _) => {
                        h5918
                    }
                };
                Ok(nat_le(a5919, a5920))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5922 = match v0.clone() {
            Adt2::C0(_, h5921, _, _, _, _, _, _, _, _, _) => {
                h5921
            }
        };
        let a5923 = 1u64;
        if if if if if nat_le(a5922, a5923) {
            true
        } else {
            let a5925 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5924, _, _, _, _, _) => {
                    h5924
                }
            };
            let a5926 = v1;
            nat_eq(a5925, a5926)
        } {
            let a5928 = match v0.clone() {
                Adt2::C0(_, _, h5927, _, _, _, _, _, _, _, _) => {
                    h5927
                }
            };
            let a5929 = 1u64;
            if nat_le(a5928, a5929) {
                true
            } else {
                let a5934 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5930, _, _, _, _) => {
                        h5930
                    }
                };
                let a5935 = {
                    let a5932 = v1;
                    let a5933 = match v0.clone() {
                        Adt2::C0(_, h5931, _, _, _, _, _, _, _, _, _) => {
                            h5931
                        }
                    };
                    nat_mul(a5932, a5933)?
                };
                nat_eq(a5934, a5935)
            }
        } else {
            false
        } {
            let a5937 = match v0.clone() {
                Adt2::C0(_, _, _, h5936, _, _, _, _, _, _, _) => {
                    h5936
                }
            };
            let a5938 = 1u64;
            if nat_le(a5937, a5938) {
                true
            } else {
                let a5946 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5939, _, _, _) => {
                        h5939
                    }
                };
                let a5947 = {
                    let a5944 = {
                        let a5941 = v1;
                        let a5942 = match v0.clone() {
                            Adt2::C0(_, h5940, _, _, _, _, _, _, _, _, _) => {
                                h5940
                            }
                        };
                        nat_mul(a5941, a5942)?
                    };
                    let a5945 = match v0.clone() {
                        Adt2::C0(_, _, h5943, _, _, _, _, _, _, _, _) => {
                            h5943
                        }
                    };
                    nat_mul(a5944, a5945)?
                };
                nat_eq(a5946, a5947)
            }
        } else {
            false
        } {
            let a5949 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5948, _, _, _, _, _, _) => {
                    h5948
                }
            };
            let a5950 = 1u64;
            if nat_le(a5949, a5950) {
                true
            } else {
                let a5961 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5951, _, _) => {
                        h5951
                    }
                };
                let a5962 = {
                    let a5959 = {
                        let a5956 = {
                            let a5953 = v1;
                            let a5954 = match v0.clone() {
                                Adt2::C0(_, h5952, _, _, _, _, _, _, _, _, _) => {
                                    h5952
                                }
                            };
                            nat_mul(a5953, a5954)?
                        };
                        let a5957 = match v0.clone() {
                            Adt2::C0(_, _, h5955, _, _, _, _, _, _, _, _) => {
                                h5955
                            }
                        };
                        nat_mul(a5956, a5957)?
                    };
                    let a5960 = match v0.clone() {
                        Adt2::C0(_, _, _, h5958, _, _, _, _, _, _, _) => {
                            h5958
                        }
                    };
                    nat_mul(a5959, a5960)?
                };
                nat_eq(a5961, a5962)
            }
        } else {
            false
        } {
            let a5976 = {
                let a5973 = {
                    let a5970 = {
                        let a5967 = {
                            let a5964 = v1;
                            let a5965 = match v0.clone() {
                                Adt2::C0(_, h5963, _, _, _, _, _, _, _, _, _) => {
                                    h5963
                                }
                            };
                            nat_mul(a5964, a5965)?
                        };
                        let a5968 = match v0.clone() {
                            Adt2::C0(_, _, h5966, _, _, _, _, _, _, _, _) => {
                                h5966
                            }
                        };
                        nat_mul(a5967, a5968)?
                    };
                    let a5971 = match v0.clone() {
                        Adt2::C0(_, _, _, h5969, _, _, _, _, _, _, _) => {
                            h5969
                        }
                    };
                    nat_mul(a5970, a5971)?
                };
                let a5974 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5972, _, _, _, _, _, _) => {
                        h5972
                    }
                };
                nat_mul(a5973, a5974)?
            };
            let a5977 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5975, _) => {
                    h5975
                }
            };
            Ok(nat_le(a5976, a5977))
        } else {
            Ok(false)
        }
    }
}

pub fn f221(v0: Adt2, v1: u64) -> R<bool> {
    let a5979 = match v0.clone() {
        Adt2::C0(h5978, _, _, _, _, _, _, _, _, _, _) => {
            h5978
        }
    };
    let a5980 = 0u64;
    if if nat_eq(a5979, a5980) {
        f38(v0.clone())?
    } else {
        false
    } {
        let a5981 = v1;
        let a5982 = 1u64;
        if nat_le(a5981, a5982) {
            Ok(true)
        } else {
            let a5987 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5983, _, _, _, _) => {
                    h5983
                }
            };
            let a5988 = {
                let a5985 = 4u64;
                let a5986 = match v0 {
                    Adt2::C0(_, h5984, _, _, _, _, _, _, _, _, _) => {
                        h5984
                    }
                };
                nat_mul(a5985, a5986)?
            };
            Ok(nat_eq(a5987, a5988))
        }
    } else {
        Ok(false)
    }
}

pub fn f222(v0: Adt2, v1: u64) -> R<bool> {
    let a5990 = match v0.clone() {
        Adt2::C0(_, _, _, h5989, _, _, _, _, _, _, _) => {
            h5989
        }
    };
    let a5991 = 1u64;
    if if if nat_eq(a5990, a5991) {
        let a5993 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5992, _, _, _, _, _, _) => {
                h5992
            }
        };
        let a5994 = 1u64;
        nat_eq(a5993, a5994)
    } else {
        false
    } {
        let a5996 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5995, _, _, _, _, _) => {
                h5995
            }
        };
        let a5997 = v1;
        nat_eq(a5996, a5997)
    } else {
        false
    } {
        let a5999 = match v0.clone() {
            Adt2::C0(_, _, h5998, _, _, _, _, _, _, _, _) => {
                h5998
            }
        };
        let a6000 = 1u64;
        if nat_eq(a5999, a6000) {
            Ok(true)
        } else {
            let a6005 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6001, _, _, _, _) => {
                    h6001
                }
            };
            let a6006 = {
                let a6003 = match v0 {
                    Adt2::C0(_, h6002, _, _, _, _, _, _, _, _, _) => {
                        h6002
                    }
                };
                let a6004 = v1;
                nat_mul(a6003, a6004)?
            };
            Ok(nat_eq(a6005, a6006))
        }
    } else {
        Ok(false)
    }
}

pub fn f223(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a6010 = {
            let a6007 = v2;
            let a6008 = 1u64;
            nat_sub(a6007, a6008)
        };
        let a6011 = match v0.clone() {
            Adt6::C0(_, h6009, _, _) => {
                h6009
            }
        };
        nat_mul(a6010, a6011)?
    };
    let a6013 = 0u64;
    let a6014 = match v0.clone() {
        Adt6::C0(h6012, _, _, _) => {
            h6012
        }
    };
    if if if if if if if if nat_lt(a6013, a6014) {
        let a6016 = match v0.clone() {
            Adt6::C0(h6015, _, _, _) => {
                h6015
            }
        };
        let a6017 = 2147483648u64;
        nat_lt(a6016, a6017)
    } else {
        false
    } {
        let a6019 = 0u64;
        let a6020 = match v0.clone() {
            Adt6::C0(_, h6018, _, _) => {
                h6018
            }
        };
        nat_lt(a6019, a6020)
    } else {
        false
    } {
        let a6022 = match v0.clone() {
            Adt6::C0(_, h6021, _, _) => {
                h6021
            }
        };
        let a6023 = 2147483648u64;
        nat_lt(a6022, a6023)
    } else {
        false
    } {
        let a6024 = v1;
        let a6025 = 2147483648u64;
        nat_lt(a6024, a6025)
    } else {
        false
    } {
        let a6026 = 0u64;
        let a6027 = v2;
        nat_lt(a6026, a6027)
    } else {
        false
    } {
        let a6028 = v3;
        let a6029 = 2147483648u64;
        nat_lt(a6028, a6029)
    } else {
        false
    } {
        let a6041 = {
            let a6036 = {
                let a6033 = v3;
                let a6034 = {
                    let a6031 = match v0.clone() {
                        Adt6::C0(h6030, _, _, _) => {
                            h6030
                        }
                    };
                    let a6032 = 1u64;
                    nat_sub(a6031, a6032)
                };
                nat_add(a6033, a6034)?
            };
            let a6037 = f145(match v0.clone() {
                Adt6::C0(_, _, h6035, _) => {
                    h6035
                }
            });
            nat_add(a6036, a6037)?
        };
        let a6042 = {
            let a6039 = 2147483647u64;
            let a6040 = f146(match v0.clone() {
                Adt6::C0(_, _, h6038, _) => {
                    h6038
                }
            });
            nat_add(a6039, a6040)?
        };
        nat_le(a6041, a6042)
    } else {
        false
    } {
        let a6044 = match v0 {
            Adt6::C0(_, _, h6043, _) => {
                h6043
            }
        };
        let a6045 = 2147483648u64;
        Ok(!nat_eq(a6044, a6045))
    } else {
        Ok(false)
    }
}

pub fn f224(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f273(match v0.clone() {
        Adt2::C0(h6046, _, _, _, _, _, _, _, _, _, _) => {
            h6046
        }
    }, match v0.clone() {
        Adt2::C0(_, h6047, _, _, _, _, _, _, _, _, _) => {
            h6047
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6048, _, _, _, _, _, _, _, _) => {
            h6048
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6049, _, _, _, _, _, _, _) => {
            h6049
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6050, _, _, _, _, _, _) => {
            h6050
        }
    }, match v1.clone() {
        Adt2::C0(h6051, _, _, _, _, _, _, _, _, _, _) => {
            h6051
        }
    }, match v1.clone() {
        Adt2::C0(_, h6052, _, _, _, _, _, _, _, _, _) => {
            h6052
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h6053, _, _, _, _, _, _, _, _) => {
            h6053
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h6054, _, _, _, _, _, _, _) => {
            h6054
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h6055, _, _, _, _, _, _) => {
            h6055
        }
    }, match v2.clone() {
        Adt2::C0(h6056, _, _, _, _, _, _, _, _, _, _) => {
            h6056
        }
    }, match v2.clone() {
        Adt2::C0(_, h6057, _, _, _, _, _, _, _, _, _) => {
            h6057
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h6058, _, _, _, _, _, _, _, _) => {
            h6058
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h6059, _, _, _, _, _, _, _) => {
            h6059
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h6060, _, _, _, _, _, _) => {
            h6060
        }
    }) {
        let m6062 = f218(match v0.clone() {
            Adt2::C0(h6061, _, _, _, _, _, _, _, _, _, _) => {
                h6061
            }
        });
        match m6062 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a6068 = {
                    let a6065 = match v0.clone() {
                        Adt2::C0(_, h6063, _, _, _, _, _, _, _, _, _) => {
                            h6063
                        }
                    };
                    let a6066 = match v3.clone() {
                        Adt19::C0(_, h6064, _) => {
                            h6064
                        }
                    };
                    let a6067 = 0u64;
                    nat_rem(a6065, a6066, a6067)
                };
                let a6069 = 0u64;
                if if nat_eq(a6068, a6069) {
                    if f274(match v0.clone() {
                        Adt2::C0(_, h6070, _, _, _, _, _, _, _, _, _) => {
                            h6070
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6071, _, _, _, _, _, _, _, _, _) => {
                            h6071
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6072, _, _, _, _, _, _, _, _) => {
                            h6072
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6073, _, _, _, _, _, _, _) => {
                            h6073
                        }
                    }) {
                        true
                    } else {
                        let a6075 = 4u64;
                        let a6076 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6074, _, _, _, _, _) => {
                                h6074
                            }
                        };
                        if if nat_le(a6075, a6076) {
                            let a6078 = match v3.clone() {
                                Adt19::C0(_, h6077, _) => {
                                    h6077
                                }
                            };
                            let a6079 = 1u64;
                            if nat_eq(a6078, a6079) {
                                true
                            } else {
                                let a6082 = match v3.clone() {
                                    Adt19::C0(_, _, h6080) => {
                                        h6080
                                    }
                                };
                                let a6083 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h6081, _, _, _, _, _) => {
                                        h6081
                                    }
                                };
                                nat_le(a6082, a6083)
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
                        Adt19::C0(_, h6084, _) => {
                            h6084
                        }
                    });
                    Ok(Adt10::C4(if f274(match v0.clone() {
                        Adt2::C0(_, h6085, _, _, _, _, _, _, _, _, _) => {
                            h6085
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6086, _, _, _, _, _, _, _, _, _) => {
                            h6086
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6087, _, _, _, _, _, _, _, _) => {
                            h6087
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6088, _, _, _, _, _, _, _) => {
                            h6088
                        }
                    }) {
                        0u64
                    } else {
                        f183(v4.clone(), f25(v1.clone(), 1u64))?
                    }, 4u64, v4, f25(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h6089, _, _, _, _, _, _, _, _, _, _) => {
                            h6089
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

pub fn f225(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f226(match v0.clone() {
        Adt2::C0(h6090, _, _, _, _, _, _, _, _, _, _) => {
            h6090
        }
    }) {
        let a6096 = {
            let a6093 = match v0.clone() {
                Adt2::C0(_, h6091, _, _, _, _, _, _, _, _, _) => {
                    h6091
                }
            };
            let a6094 = f256(match v0.clone() {
                Adt2::C0(h6092, _, _, _, _, _, _, _, _, _, _) => {
                    h6092
                }
            });
            let a6095 = 1u64;
            nat_rem(a6093, a6094, a6095)
        };
        let a6097 = 0u64;
        nat_eq(a6096, a6097)
    } else {
        false
    } {
        let a6099 = match v1.clone() {
            Adt2::C0(h6098, _, _, _, _, _, _, _, _, _, _) => {
                h6098
            }
        };
        let a6100 = 26u64;
        nat_eq(a6099, a6100)
    } else {
        false
    } {
        let a6102 = match v2.clone() {
            Adt2::C0(h6101, _, _, _, _, _, _, _, _, _, _) => {
                h6101
            }
        };
        let a6103 = 0u64;
        nat_eq(a6102, a6103)
    } else {
        false
    } {
        let a6106 = match v0.clone() {
            Adt2::C0(_, h6104, _, _, _, _, _, _, _, _, _) => {
                h6104
            }
        };
        let a6107 = match v2.clone() {
            Adt2::C0(_, h6105, _, _, _, _, _, _, _, _, _) => {
                h6105
            }
        };
        nat_eq(a6106, a6107)
    } else {
        false
    } {
        let a6110 = match v1.clone() {
            Adt2::C0(_, h6108, _, _, _, _, _, _, _, _, _) => {
                h6108
            }
        };
        let a6111 = match v0.clone() {
            Adt2::C0(_, _, h6109, _, _, _, _, _, _, _, _) => {
                h6109
            }
        };
        nat_le(a6110, a6111)
    } else {
        false
    } {
        let a6113 = match v0.clone() {
            Adt2::C0(_, _, _, h6112, _, _, _, _, _, _, _) => {
                h6112
            }
        };
        let a6114 = 1u64;
        nat_eq(a6113, a6114)
    } else {
        false
    } {
        let a6116 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6115, _, _, _, _, _, _) => {
                h6115
            }
        };
        let a6117 = 1u64;
        nat_eq(a6116, a6117)
    } else {
        false
    } {
        let a6119 = match v2.clone() {
            Adt2::C0(_, _, _, h6118, _, _, _, _, _, _, _) => {
                h6118
            }
        };
        let a6120 = 1u64;
        nat_eq(a6119, a6120)
    } else {
        false
    } {
        let a6122 = match v2.clone() {
            Adt2::C0(_, _, _, _, h6121, _, _, _, _, _, _) => {
                h6121
            }
        };
        let a6123 = 1u64;
        nat_eq(a6122, a6123)
    } else {
        false
    } {
        let a6125 = match v1.clone() {
            Adt2::C0(_, _, h6124, _, _, _, _, _, _, _, _) => {
                h6124
            }
        };
        let a6126 = 1u64;
        nat_eq(a6125, a6126)
    } else {
        false
    } {
        let a6128 = match v1.clone() {
            Adt2::C0(_, _, _, h6127, _, _, _, _, _, _, _) => {
                h6127
            }
        };
        let a6129 = 1u64;
        nat_eq(a6128, a6129)
    } else {
        false
    } {
        let a6131 = match v1.clone() {
            Adt2::C0(_, _, _, _, h6130, _, _, _, _, _, _) => {
                h6130
            }
        };
        let a6132 = 1u64;
        nat_eq(a6131, a6132)
    } else {
        false
    } {
        if f48(match v2.clone() {
            Adt2::C0(_, h6133, _, _, _, _, _, _, _, _, _) => {
                h6133
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h6134, _, _, _, _, _, _, _, _) => {
                h6134
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h6135, _, _, _, _, _, _, _) => {
                h6135
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h6136, _, _, _, _, _, _) => {
                h6136
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a6138 = match v1.clone() {
                Adt2::C0(_, h6137, _, _, _, _, _, _, _, _, _) => {
                    h6137
                }
            };
            let a6139 = 0u64;
            if nat_eq(a6138, a6139) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a6141 = match v0.clone() {
                    Adt2::C0(_, h6140, _, _, _, _, _, _, _, _, _) => {
                        h6140
                    }
                };
                let a6142 = 2147483647u64;
                if if if if if if nat_le(a6141, a6142) {
                    let a6144 = match v1.clone() {
                        Adt2::C0(_, h6143, _, _, _, _, _, _, _, _, _) => {
                            h6143
                        }
                    };
                    let a6145 = 2147483647u64;
                    nat_le(a6144, a6145)
                } else {
                    false
                } {
                    let a6147 = 4u64;
                    let a6148 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h6146, _, _, _, _, _) => {
                            h6146
                        }
                    };
                    nat_le(a6147, a6148)
                } else {
                    false
                } {
                    let a6154 = {
                        let a6151 = match v2.clone() {
                            Adt2::C0(_, h6149, _, _, _, _, _, _, _, _, _) => {
                                h6149
                            }
                        };
                        let a6152 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6150, _, _, _, _, _) => {
                                h6150
                            }
                        };
                        nat_mul(a6151, a6152)?
                    };
                    let a6155 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6153, _, _, _, _) => {
                            h6153
                        }
                    };
                    nat_le(a6154, a6155)
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
                        let a6168 = {
                            let a6159 = {
                                let a6157 = 2u64;
                                let a6158 = match v1.clone() {
                                    Adt2::C0(_, h6156, _, _, _, _, _, _, _, _, _) => {
                                        h6156
                                    }
                                };
                                nat_mul(a6157, a6158)?
                            };
                            let a6160 = f23(v2.clone(), 1u64)?;
                            nat_add(a6159, a6160)?
                        };
                        let a6169 = {
                            let a6166 = match v1.clone() {
                                Adt2::C0(_, h6161, _, _, _, _, _, _, _, _, _) => {
                                    h6161
                                }
                            };
                            let a6167 = {
                                let a6164 = 1u64;
                                let a6165 = f180(match v0.clone() {
                                    Adt2::C0(h6162, _, _, _, _, _, _, _, _, _, _) => {
                                        h6162
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h6163, _, _, _, _, _, _, _, _, _) => {
                                        h6163
                                    }
                                })?;
                                nat_add(a6164, a6165)?
                            };
                            nat_mul(a6166, a6167)?
                        };
                        nat_add(a6168, a6169)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h6170, _, _, _, _, _, _, _, _, _) => {
                            h6170
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6171, _, _, _, _, _, _, _, _, _) => {
                            h6171
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6172, _, _, _, _, _) => {
                            h6172
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6173, _, _, _, _) => {
                            h6173
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6174, _, _, _) => {
                            h6174
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6175, _, _) => {
                            h6175
                        }
                    }), f25(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h6176, _, _, _, _, _, _, _, _, _, _) => {
                            h6176
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

pub fn f226(v0: u64) -> bool {
    let a6177 = v0;
    let a6178 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a6177, a6178) {
        true
    } else {
        let a6179 = v0;
        let a6180 = 1u64;
        nat_eq(a6179, a6180)
    } {
        true
    } else {
        let a6181 = v0;
        let a6182 = 2u64;
        nat_eq(a6181, a6182)
    } {
        true
    } else {
        let a6183 = v0;
        let a6184 = 3u64;
        nat_eq(a6183, a6184)
    } {
        true
    } else {
        let a6185 = v0;
        let a6186 = 6u64;
        nat_eq(a6185, a6186)
    } {
        true
    } else {
        let a6187 = v0;
        let a6188 = 7u64;
        nat_eq(a6187, a6188)
    } {
        true
    } else {
        let a6189 = v0;
        let a6190 = 8u64;
        nat_eq(a6189, a6190)
    } {
        true
    } else {
        let a6191 = v0;
        let a6192 = 10u64;
        nat_eq(a6191, a6192)
    } {
        true
    } else {
        let a6193 = v0;
        let a6194 = 11u64;
        nat_eq(a6193, a6194)
    } {
        true
    } else {
        let a6195 = v0;
        let a6196 = 12u64;
        nat_eq(a6195, a6196)
    } {
        true
    } else {
        let a6197 = v0;
        let a6198 = 13u64;
        nat_eq(a6197, a6198)
    } {
        true
    } else {
        let a6199 = v0;
        let a6200 = 14u64;
        nat_eq(a6199, a6200)
    } {
        true
    } else {
        let a6201 = v0;
        let a6202 = 16u64;
        nat_eq(a6201, a6202)
    } {
        true
    } else {
        let a6203 = v0;
        let a6204 = 17u64;
        nat_eq(a6203, a6204)
    } {
        true
    } else {
        let a6205 = v0;
        let a6206 = 18u64;
        nat_eq(a6205, a6206)
    } {
        true
    } else {
        let a6207 = v0;
        let a6208 = 19u64;
        nat_eq(a6207, a6208)
    } {
        true
    } else {
        let a6209 = v0;
        let a6210 = 20u64;
        nat_eq(a6209, a6210)
    } {
        true
    } else {
        let a6211 = v0;
        let a6212 = 21u64;
        nat_eq(a6211, a6212)
    } {
        true
    } else {
        let a6213 = v0;
        let a6214 = 22u64;
        nat_eq(a6213, a6214)
    } {
        true
    } else {
        let a6215 = v0;
        let a6216 = 23u64;
        nat_eq(a6215, a6216)
    } {
        true
    } else {
        let a6217 = v0;
        let a6218 = 26u64;
        nat_eq(a6217, a6218)
    } {
        true
    } else {
        let a6219 = v0;
        let a6220 = 29u64;
        nat_eq(a6219, a6220)
    } {
        true
    } else {
        let a6221 = v0;
        let a6222 = 30u64;
        nat_eq(a6221, a6222)
    } {
        true
    } else {
        let a6223 = v0;
        let a6224 = 34u64;
        nat_eq(a6223, a6224)
    } {
        true
    } else {
        let a6225 = v0;
        let a6226 = 35u64;
        nat_eq(a6225, a6226)
    } {
        true
    } else {
        let a6227 = v0;
        let a6228 = 39u64;
        nat_eq(a6227, a6228)
    } {
        true
    } else {
        let a6229 = v0;
        let a6230 = 40u64;
        nat_eq(a6229, a6230)
    } {
        true
    } else {
        let a6231 = v0;
        let a6232 = 41u64;
        nat_eq(a6231, a6232)
    } {
        true
    } else {
        let a6233 = v0;
        let a6234 = 42u64;
        nat_eq(a6233, a6234)
    } {
        true
    } else {
        let a6235 = v0;
        let a6236 = 43u64;
        nat_eq(a6235, a6236)
    }
}

pub fn f227(v0: u64) -> bool {
    if f181(v0) {
        true
    } else {
        let a6237 = v0;
        let a6238 = 43u64;
        nat_eq(a6237, a6238)
    }
}

pub fn f228(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a6241 = match v1.clone() {
        Adt2::C0(h6239, _, _, _, _, _, _, _, _, _, _) => {
            h6239
        }
    };
    let a6242 = match v3.clone() {
        Adt2::C0(h6240, _, _, _, _, _, _, _, _, _, _) => {
            h6240
        }
    };
    if if nat_eq(a6241, a6242) {
        let a6243 = v0;
        let a6244 = 33u64;
        if nat_eq(a6243, a6244) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m6246 = f218(match v1.clone() {
            Adt2::C0(h6245, _, _, _, _, _, _, _, _, _, _) => {
                h6245
            }
        });
        match m6246 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h6247, _) => {
                        h6247
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a6252 = {
                    let a6249 = match v1.clone() {
                        Adt2::C0(_, h6248, _, _, _, _, _, _, _, _, _) => {
                            h6248
                        }
                    };
                    let a6250 = v5;
                    let a6251 = 1u64;
                    nat_rem(a6249, a6250, a6251)
                };
                let a6253 = 0u64;
                if if if nat_eq(a6252, a6253) {
                    let a6258 = {
                        let a6255 = match v3.clone() {
                            Adt2::C0(_, h6254, _, _, _, _, _, _, _, _, _) => {
                                h6254
                            }
                        };
                        let a6256 = v5;
                        let a6257 = 1u64;
                        nat_rem(a6255, a6256, a6257)
                    };
                    let a6259 = 0u64;
                    nat_eq(a6258, a6259)
                } else {
                    false
                } {
                    let a6260 = v6;
                    let a6261 = f23(v3.clone(), v5)?;
                    nat_eq(a6260, a6261)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6262) => {
                            h6262
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

pub fn f229(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f48(v4, v5, v6, v7) {
        f48(v0, v1, v2, v3)
    } else {
        let a6266 = {
            let a6263 = v0;
            let a6264 = v4;
            let a6265 = 1u64;
            nat_rem(a6263, a6264, a6265)
        };
        let a6267 = 0u64;
        if if if nat_eq(a6266, a6267) {
            let a6271 = {
                let a6268 = v1;
                let a6269 = v5;
                let a6270 = 1u64;
                nat_rem(a6268, a6269, a6270)
            };
            let a6272 = 0u64;
            nat_eq(a6271, a6272)
        } else {
            false
        } {
            let a6276 = {
                let a6273 = v2;
                let a6274 = v6;
                let a6275 = 1u64;
                nat_rem(a6273, a6274, a6275)
            };
            let a6277 = 0u64;
            nat_eq(a6276, a6277)
        } else {
            false
        } {
            let a6281 = {
                let a6278 = v3;
                let a6279 = v7;
                let a6280 = 1u64;
                nat_rem(a6278, a6279, a6280)
            };
            let a6282 = 0u64;
            nat_eq(a6281, a6282)
        } else {
            false
        }
    }
}

pub fn f230(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6283 = v0;
    let a6284 = v1;
    if nat_eq(a6283, a6284) {
        let a6287 = v4;
        let a6288 = {
            let a6285 = v2;
            let a6286 = v3;
            nat_add(a6285, a6286)?
        };
        Ok(nat_eq(a6287, a6288))
    } else {
        let a6289 = v2;
        let a6290 = v3;
        if nat_eq(a6289, a6290) {
            let a6291 = v4;
            let a6292 = v2;
            Ok(nat_eq(a6291, a6292))
        } else {
            Ok(false)
        }
    }
}

pub fn f231(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6295 = {
            let a6293 = v0;
            let a6294 = f146(v1);
            nat_add(a6293, a6294)?
        };
        let a6296 = f146(v2);
        nat_add(a6295, a6296)?
    };
    let v5: u64 = {
        let a6297 = f145(v1);
        let a6298 = f145(v2);
        nat_add(a6297, a6298)?
    };
    let a6299 = v1;
    let a6300 = 4294967295u64;
    if if nat_le(a6299, a6300) {
        let a6301 = v2;
        let a6302 = 4294967295u64;
        nat_le(a6301, a6302)
    } else {
        false
    } {
        let a6303 = v5;
        let a6304 = v4;
        if nat_le(a6303, a6304) {
            let a6307 = v3;
            let a6308 = {
                let a6305 = v4;
                let a6306 = v5;
                nat_sub(a6305, a6306)
            };
            Ok(nat_eq(a6307, a6308))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f232(v0: u64, v1: u64) -> R<bool> {
    let a6309 = v1;
    let a6310 = 4294967295u64;
    if nat_le(a6309, a6310) {
        let a6313 = {
            let a6311 = f146(v1);
            let a6312 = f145(v1);
            nat_add(a6311, a6312)?
        };
        let a6314 = v0;
        Ok(nat_lt(a6313, a6314))
    } else {
        Ok(false)
    }
}

pub fn f233(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f275(v0.clone(), false)?;
    let v6: u64 = f275(v0, true)?;
    let a6315 = v6;
    let a6316 = v1;
    if if nat_le(a6315, a6316) {
        let a6323 = {
            let a6321 = {
                let a6317 = v1;
                let a6318 = v5;
                nat_add(a6317, a6318)?
            };
            let a6322 = {
                let a6319 = v3;
                let a6320 = 0u64;
                if nat_eq(a6319, a6320) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6321, a6322)?
        };
        let a6324 = v2;
        nat_le(a6323, a6324)
    } else {
        false
    } {
        if v4 {
            let a6329 = {
                let a6327 = {
                    let a6325 = v1;
                    let a6326 = v5;
                    nat_add(a6325, a6326)?
                };
                let a6328 = v6;
                nat_sub(a6327, a6328)
            };
            let a6330 = v2;
            Ok(nat_lt(a6329, a6330))
        } else {
            let a6335 = {
                let a6333 = {
                    let a6331 = v1;
                    let a6332 = v5;
                    nat_add(a6331, a6332)?
                };
                let a6334 = v6;
                nat_sub(a6333, a6334)
            };
            let a6336 = v2;
            Ok(nat_le(a6335, a6336))
        }
    } else {
        Ok(false)
    }
}

pub fn f234(v0: u64, v1: u64) -> bool {
    let a6337 = v0;
    let a6338 = 0u64;
    if if nat_eq(a6337, a6338) {
        true
    } else {
        let a6339 = v0;
        let a6340 = 1u64;
        nat_eq(a6339, a6340)
    } {
        let a6341 = v1;
        let a6342 = 4294967296u64;
        nat_lt(a6341, a6342)
    } else {
        false
    }
}

pub fn f235(v0: u64, v1: u64) -> R<u64> {
    let a6347 = {
        let a6345 = v0;
        let a6346 = {
            let a6343 = v1;
            let a6344 = 1u64;
            nat_sub(a6343, a6344)
        };
        nat_add(a6345, a6346)?
    };
    let a6348 = v1;
    let a6349 = 0u64;
    Ok(nat_quot(a6347, a6348, a6349))
}

pub fn f236(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f48(match v0.clone() {
        Adt2::C0(_, h6350, _, _, _, _, _, _, _, _, _) => {
            h6350
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6351, _, _, _, _, _, _, _, _) => {
            h6351
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6352, _, _, _, _, _, _, _) => {
            h6352
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6353, _, _, _, _, _, _) => {
            h6353
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6356 = match v1.clone() {
                Adt2::C0(_, h6354, _, _, _, _, _, _, _, _, _) => {
                    h6354
                }
            };
            let a6357 = match v1.clone() {
                Adt2::C0(_, _, h6355, _, _, _, _, _, _, _, _) => {
                    h6355
                }
            };
            nat_mul(a6356, a6357)?
        };
        let v5: u64 = {
            let a6359 = v4;
            let a6360 = match v1.clone() {
                Adt2::C0(_, _, _, h6358, _, _, _, _, _, _, _) => {
                    h6358
                }
            };
            nat_mul(a6359, a6360)?
        };
        let a6373 = {
            let a6371 = {
                let a6362 = match v0.clone() {
                    Adt2::C0(_, h6361, _, _, _, _, _, _, _, _, _) => {
                        h6361
                    }
                };
                let a6363 = 1u64;
                nat_sub(a6362, a6363)
            };
            let a6372 = {
                let a6369 = f276(match v0.clone() {
                    Adt2::C0(_, _, h6364, _, _, _, _, _, _, _, _) => {
                        h6364
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6365, _, _, _, _, _, _, _, _, _) => {
                        h6365
                    }
                })?;
                let a6370 = f276(match v0 {
                    Adt2::C0(_, _, _, h6366, _, _, _, _, _, _, _) => {
                        h6366
                    }
                }, v2, {
                    let a6367 = v3;
                    let a6368 = v5;
                    nat_mul(a6367, a6368)?
                }, v4)?;
                nat_add(a6369, a6370)?
            };
            nat_add(a6371, a6372)?
        };
        let a6374 = f23(v1, 1u64)?;
        Ok(nat_lt(a6373, a6374))
    }
}

pub fn f237(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6375 = v0;
    let a6376 = 2147483648u64;
    if if if if nat_lt(a6375, a6376) {
        let a6377 = v1;
        let a6378 = 2147483648u64;
        nat_lt(a6377, a6378)
    } else {
        false
    } {
        let a6381 = {
            let a6379 = v0;
            let a6380 = v1;
            nat_mul(a6379, a6380)?
        };
        let a6382 = 2147483648u64;
        nat_lt(a6381, a6382)
    } else {
        false
    } {
        let a6387 = {
            let a6385 = {
                let a6383 = v0;
                let a6384 = 1u64;
                nat_sub(a6383, a6384)
            };
            let a6386 = v2;
            nat_mul(a6385, a6386)?
        };
        let a6388 = 2147483648u64;
        nat_lt(a6387, a6388)
    } else {
        false
    } {
        let a6393 = {
            let a6391 = {
                let a6389 = v1;
                let a6390 = 1u64;
                nat_sub(a6389, a6390)
            };
            let a6392 = v2;
            nat_mul(a6391, a6392)?
        };
        let a6394 = 2147483648u64;
        Ok(nat_lt(a6393, a6394))
    } else {
        Ok(false)
    }
}

pub fn f238(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6395 = v1;
    let a6396 = 13u64;
    if if nat_eq(a6395, a6396) {
        f48(match v0.clone() {
            Adt3::C0(h6397, _, _, _, _, _, _, _) => {
                h6397
            }
        }, match v0.clone() {
            Adt3::C0(_, h6398, _, _, _, _, _, _) => {
                h6398
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6399, _, _, _, _, _) => {
                h6399
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6400, _, _, _, _) => {
                h6400
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6406 = match v0.clone() {
                Adt3::C0(_, h6401, _, _, _, _, _, _) => {
                    h6401
                }
            };
            let a6407 = {
                let a6404 = match v0.clone() {
                    Adt3::C0(_, _, h6402, _, _, _, _, _) => {
                        h6402
                    }
                };
                let a6405 = match v0.clone() {
                    Adt3::C0(_, _, _, h6403, _, _, _, _) => {
                        h6403
                    }
                };
                nat_mul(a6404, a6405)?
            };
            nat_mul(a6406, a6407)?
        };
        let a6408 = v1;
        let a6409 = 13u64;
        if nat_eq(a6408, a6409) {
            let a6418 = 2u64;
            let a6419 = if f277(v0.clone(), v2) {
                let a6411 = v3;
                let a6412 = f250(match v0 {
                    Adt3::C0(h6410, _, _, _, _, _, _, _) => {
                        h6410
                    }
                }, false)?;
                nat_mul(a6411, a6412)?
            } else {
                let a6416 = v3;
                let a6417 = {
                    let a6414 = match v0 {
                        Adt3::C0(h6413, _, _, _, _, _, _, _) => {
                            h6413
                        }
                    };
                    let a6415 = 2u64;
                    nat_mul(a6414, a6415)?
                };
                nat_mul(a6416, a6417)?
            };
            nat_add(a6418, a6419)
        } else {
            let a6423 = v3;
            let a6424 = f250(match v0 {
                Adt3::C0(h6420, _, _, _, _, _, _, _) => {
                    h6420
                }
            }, {
                let a6421 = v1;
                let a6422 = 16u64;
                nat_eq(a6421, a6422)
            })?;
            nat_mul(a6423, a6424)
        }
    }
}

pub fn f239(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6425 = v0;
    let a6426 = 30u64;
    if if nat_eq(a6425, a6426) {
        let a6429 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6427, _, _, _, _) => {
                h6427
            }
        };
        let a6430 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6428, _, _, _, _, _) => {
                h6428
            }
        };
        nat_lt(a6429, a6430)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6431 = v3;
        match m6431 {
            None => {
                f278(v0, v1, match v2.clone() {
                    Adt2::C0(h6432, _, _, _, _, _, _, _, _, _, _) => {
                        h6432
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6433, _, _, _, _, _, _, _, _, _, _) => {
                        h6433
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6434, _, _, _, _, _, _, _, _, _) => {
                        h6434
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6435, _, _, _, _, _, _, _, _) => {
                        h6435
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6436, _, _, _, _, _, _, _) => {
                        h6436
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6437, _, _, _, _, _, _) => {
                        h6437
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6438, _, _, _, _, _, _, _, _, _) => {
                        h6438
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6439, _, _, _, _, _, _, _, _) => {
                        h6439
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6440, _, _, _, _, _, _, _) => {
                        h6440
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6441, _, _, _, _, _, _) => {
                        h6441
                    }
                })
            }
            Some(v5) => {
                f278(v0, v1, match v2.clone() {
                    Adt2::C0(h6442, _, _, _, _, _, _, _, _, _, _) => {
                        h6442
                    }
                }, match v5.clone() {
                    Adt2::C0(h6443, _, _, _, _, _, _, _, _, _, _) => {
                        h6443
                    }
                }, match v4.clone() {
                    Adt2::C0(h6444, _, _, _, _, _, _, _, _, _, _) => {
                        h6444
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6445, _, _, _, _, _, _, _, _, _) => {
                        h6445
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6446, _, _, _, _, _, _, _, _) => {
                        h6446
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6447, _, _, _, _, _, _, _) => {
                        h6447
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6448, _, _, _, _, _, _) => {
                        h6448
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6449, _, _, _, _, _, _, _, _, _) => {
                        h6449
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6450, _, _, _, _, _, _, _, _) => {
                        h6450
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6451, _, _, _, _, _, _, _) => {
                        h6451
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6452, _, _, _, _, _, _) => {
                        h6452
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6453, _, _, _, _, _, _, _, _, _) => {
                        h6453
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6454, _, _, _, _, _, _, _, _) => {
                        h6454
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6455, _, _, _, _, _, _, _) => {
                        h6455
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6456, _, _, _, _, _, _) => {
                        h6456
                    }
                })
            }
        }
    }
}

pub fn f240(v0: u64, v1: u64) -> u64 {
    let a6457 = v0;
    let a6458 = 23u64;
    if nat_eq(a6457, a6458) {
        let a6459 = v1;
        let a6460 = 0u64;
        if nat_eq(a6459, a6460) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6465 = {
        let a6463 = {
            let a6461 = v0;
            let a6462 = v1;
            nat_sub(a6461, a6462)
        };
        let a6464 = 1u64;
        nat_sub(a6463, a6464)
    };
    let a6466 = v2;
    nat_add(a6465, a6466)
}

pub fn f242(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f144({
        let a6467 = v0;
        let a6468 = v4;
        let a6469 = 0u64;
        nat_rem(a6467, a6468, a6469)
    }, {
        let a6470 = v1;
        let a6471 = v5;
        let a6472 = 0u64;
        nat_rem(a6470, a6471, a6472)
    }, {
        let a6473 = v2;
        let a6474 = v6;
        let a6475 = 0u64;
        nat_rem(a6473, a6474, a6475)
    }, {
        let a6476 = v3;
        let a6477 = v7;
        let a6478 = 0u64;
        nat_rem(a6476, a6477, a6478)
    }, v8, v9, v10, v11)
}

pub fn f243(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6479 = v1;
    let a6480 = 2147483648u64;
    if nat_lt(a6479, a6480) {
        let a6491 = {
            let a6489 = {
                let a6484 = {
                    let a6481 = v0;
                    let a6482 = v2;
                    let a6483 = 0u64;
                    nat_rem(a6481, a6482, a6483)
                };
                let a6485 = v2;
                nat_add(a6484, a6485)?
            };
            let a6490 = {
                let a6486 = v1;
                let a6487 = v2;
                let a6488 = 0u64;
                nat_rem(a6486, a6487, a6488)
            };
            nat_sub(a6489, a6490)
        };
        let a6492 = v2;
        let a6493 = 0u64;
        Ok(nat_rem(a6491, a6492, a6493))
    } else {
        let a6496 = {
            let a6494 = v0;
            let a6495 = f145(v1);
            nat_add(a6494, a6495)?
        };
        let a6497 = v2;
        let a6498 = 0u64;
        Ok(nat_rem(a6496, a6497, a6498))
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6501 = f146(v1);
    let a6502 = {
        let a6499 = v0;
        let a6500 = f145(v1);
        nat_add(a6499, a6500)?
    };
    if nat_le(a6501, a6502) {
        let a6503 = f245(v0, v1)?;
        let a6504 = v2;
        Ok(nat_lt(a6503, a6504))
    } else {
        Ok(false)
    }
}

pub fn f245(v0: u64, v1: u64) -> R<u64> {
    let a6507 = {
        let a6505 = v0;
        let a6506 = f145(v1);
        nat_add(a6505, a6506)?
    };
    let a6508 = f146(v1);
    Ok(nat_sub(a6507, a6508))
}

pub fn f246(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6509 = v0;
    let a6510 = v1;
    if nat_lt(a6509, a6510) {
        let a6511 = v1;
        let a6512 = v0;
        Ok(nat_sub(a6511, a6512))
    } else {
        let a6515 = v0;
        let a6516 = {
            let a6513 = v1;
            let a6514 = v2;
            nat_add(a6513, a6514)?
        };
        if nat_lt(a6515, a6516) {
            let a6517 = v0;
            let a6518 = v1;
            Ok(nat_sub(a6517, a6518))
        } else {
            let a6525 = {
                let a6521 = 2u64;
                let a6522 = {
                    let a6519 = v2;
                    let a6520 = 1u64;
                    nat_sub(a6519, a6520)
                };
                nat_mul(a6521, a6522)?
            };
            let a6526 = {
                let a6523 = v0;
                let a6524 = v1;
                nat_sub(a6523, a6524)
            };
            Ok(nat_sub(a6525, a6526))
        }
    }
}

pub fn f247(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6529 = {
        let a6527 = v2;
        let a6528 = f279(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a6527, a6528)?
    };
    let a6530 = f279(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6529, a6530))
}

pub fn f248(v0: u64, v1: u64, v2: u64) -> bool {
    let a6531 = v0;
    let a6532 = 0u64;
    if nat_eq(a6531, a6532) {
        let a6533 = v2;
        let a6534 = v1;
        nat_le(a6533, a6534)
    } else {
        let a6535 = v0;
        let a6536 = 1u64;
        if nat_eq(a6535, a6536) {
            let a6537 = v2;
            let a6538 = v1;
            nat_lt(a6537, a6538)
        } else {
            let a6539 = v0;
            let a6540 = 2u64;
            if nat_eq(a6539, a6540) {
                let a6541 = v1;
                let a6542 = v2;
                nat_le(a6541, a6542)
            } else {
                let a6543 = v0;
                let a6544 = 3u64;
                if nat_eq(a6543, a6544) {
                    let a6545 = v1;
                    let a6546 = v2;
                    nat_lt(a6545, a6546)
                } else {
                    let a6547 = v1;
                    let a6548 = v2;
                    nat_eq(a6547, a6548)
                }
            }
        }
    }
}

pub fn f249(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6549 = v0;
    let a6550 = 0u64;
    if nat_eq(a6549, a6550) {
        Ok(f262(0u64))
    } else {
        let a6553 = {
            let a6551 = v0;
            let a6552 = 1u64;
            nat_add(a6551, a6552)?
        };
        let a6554 = f238(v1.clone(), 13u64, v2)?;
        if nat_eq(a6553, a6554) {
            Ok(Adt9::C4({
                let a6555 = v2;
                let a6556 = 0u64;
                if nat_eq(a6555, a6556) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6557 = v0;
                let a6558 = 1u64;
                nat_sub(a6557, a6558)
            };
            if f277(v1.clone(), v2) {
                let v4: u64 = f250(match v1.clone() {
                    Adt3::C0(h6559, _, _, _, _, _, _, _) => {
                        h6559
                    }
                }, false)?;
                let v5: u64 = {
                    let a6560 = v3;
                    let a6561 = v4;
                    let a6562 = 0u64;
                    nat_quot(a6560, a6561, a6562)
                };
                let v6: u64 = {
                    let a6563 = v3;
                    let a6564 = v4;
                    let a6565 = 0u64;
                    nat_rem(a6563, a6564, a6565)
                };
                let a6568 = {
                    let a6566 = v6;
                    let a6567 = 1u64;
                    nat_add(a6566, a6567)?
                };
                let a6569 = v4;
                if nat_eq(a6568, a6569) {
                    Ok(Adt9::C4({
                        let a6570 = v2;
                        let a6571 = 0u64;
                        if nat_eq(a6570, a6571) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f251(v6, v5, v1, v2)
                }
            } else {
                f280({
                    let a6572 = v3;
                    let a6573 = 2u64;
                    let a6574 = 0u64;
                    nat_quot(a6572, a6573, a6574)
                }, {
                    let a6575 = v3;
                    let a6576 = 2u64;
                    let a6577 = 0u64;
                    nat_rem(a6575, a6576, a6577)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f250(v0: u64, v1: bool) -> R<u64> {
    let a6580 = {
        let a6578 = v0;
        let a6579 = 2u64;
        nat_mul(a6578, a6579)?
    };
    let a6581 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6580, a6581)
}

pub fn f251(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6582 = v0;
    let a6583 = 0u64;
    if nat_eq(a6582, a6583) {
        Ok(f262(1u64))
    } else {
        f280({
            let a6592 = {
                let a6585 = v1;
                let a6586 = match v2.clone() {
                    Adt3::C0(h6584, _, _, _, _, _, _, _) => {
                        h6584
                    }
                };
                nat_mul(a6585, a6586)?
            };
            let a6593 = {
                let a6589 = {
                    let a6587 = v0;
                    let a6588 = 1u64;
                    nat_sub(a6587, a6588)
                };
                let a6590 = 2u64;
                let a6591 = 0u64;
                nat_quot(a6589, a6590, a6591)
            };
            nat_add(a6592, a6593)?
        }, {
            let a6596 = {
                let a6594 = v0;
                let a6595 = 1u64;
                nat_sub(a6594, a6595)
            };
            let a6597 = 2u64;
            let a6598 = 0u64;
            nat_rem(a6596, a6597, a6598)
        }, v2, v3, 1u64)
    }
}

pub fn f252(v0: u64, v1: u64) -> R<u64> {
    let a6599 = v0;
    let a6600 = f96(v1)?;
    let a6601 = 0u64;
    Ok(nat_rem(a6599, a6600, a6601))
}

pub fn f253(v0: u64, v1: u64) -> R<u64> {
    let a6602 = 4u64;
    let a6603 = f281(v0, v1)?;
    nat_mul(a6602, a6603)
}

pub fn f254(v0: u64, v1: u64) -> R<u64> {
    let a6604 = 4u64;
    let a6605 = f282(v0, v1)?;
    nat_mul(a6604, a6605)
}

pub fn f255(v0: u64, v1: u64) -> R<bool> {
    let a6611 = {
        let a6608 = {
            let a6606 = f252(v0, v1)?;
            let a6607 = 2u64;
            nat_sub(a6606, a6607)
        };
        let a6609 = 2u64;
        let a6610 = 0u64;
        nat_rem(a6608, a6609, a6610)
    };
    let a6612 = 0u64;
    Ok(nat_eq(a6611, a6612))
}

pub fn f256(v0: u64) -> u64 {
    let m6613 = f218(v0);
    match m6613 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6614, _) => {
                    h6614
                }
            }
        }
    }
}

pub fn f257(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6615 = f218(v0);
    match m6615 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f226(v0) {
                let a6616 = v1;
                let a6617 = v7;
                nat_lt(a6616, a6617)
            } else {
                false
            } {
                f186(v8, v2)
            } else {
                false
            } {
                let a6618 = v3;
                let a6619 = f283(v4, v5, v6)?;
                nat_lt(a6618, a6619)
            } else {
                false
            } {
                let a6624 = {
                    let a6621 = v7;
                    let a6622 = match v13.clone() {
                        Adt19::C0(_, h6620, _) => {
                            h6620
                        }
                    };
                    let a6623 = 0u64;
                    nat_rem(a6621, a6622, a6623)
                };
                let a6625 = 0u64;
                nat_eq(a6624, a6625)
            } else {
                false
            } {
                Ok(Some({
                    let a6654 = {
                        let a6644 = {
                            let a6634 = {
                                let a6630 = {
                                    let a6627 = v1;
                                    let a6628 = match v13 {
                                        Adt19::C0(_, h6626, _) => {
                                            h6626
                                        }
                                    };
                                    let a6629 = 0u64;
                                    nat_quot(a6627, a6628, a6629)
                                };
                                let a6631 = v9;
                                nat_mul(a6630, a6631)?
                            };
                            let a6635 = {
                                let a6632 = v2;
                                let a6633 = v10;
                                nat_mul(a6632, a6633)?
                            };
                            nat_add(a6634, a6635)?
                        };
                        let a6645 = {
                            let a6642 = {
                                let a6639 = {
                                    let a6636 = v3;
                                    let a6637 = v4;
                                    let a6638 = 0u64;
                                    nat_quot(a6636, a6637, a6638)
                                };
                                let a6640 = v5;
                                let a6641 = 0u64;
                                nat_rem(a6639, a6640, a6641)
                            };
                            let a6643 = v11;
                            nat_mul(a6642, a6643)?
                        };
                        nat_add(a6644, a6645)?
                    };
                    let a6655 = {
                        let a6652 = {
                            let a6649 = {
                                let a6646 = v3;
                                let a6647 = v4;
                                let a6648 = 0u64;
                                nat_quot(a6646, a6647, a6648)
                            };
                            let a6650 = v5;
                            let a6651 = 0u64;
                            nat_quot(a6649, a6650, a6651)
                        };
                        let a6653 = v12;
                        nat_mul(a6652, a6653)?
                    };
                    nat_add(a6654, a6655)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f258(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f284(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f259(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f285(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f260(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6656 = v0;
    let a6657 = 64u64;
    if nat_eq(a6656, a6657) {
        f286(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6658 = v0;
        let a6659 = 65u64;
        if nat_eq(a6658, a6659) {
            f287(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f261(v0: u64) -> u64 {
    let a6660 = v0;
    let a6661 = 30u64;
    if nat_eq(a6660, a6661) {
        4u64
    } else {
        3u64
    }
}

pub fn f262(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f263(v0: u64) -> bool {
    let a6662 = 0u64;
    let a6663 = v0;
    if nat_lt(a6662, a6663) {
        let a6664 = v0;
        let a6665 = 2139095040u64;
        nat_le(a6664, a6665)
    } else {
        false
    }
}

pub fn f264(v0: u64) -> u64 {
    let a6666 = v0;
    let a6667 = 40u64;
    if nat_lt(a6666, a6667) {
        let a6668 = v0;
        let a6669 = 20u64;
        if nat_lt(a6668, a6669) {
            let a6670 = v0;
            let a6671 = 10u64;
            if nat_lt(a6670, a6671) {
                let a6672 = v0;
                let a6673 = 5u64;
                if nat_lt(a6672, a6673) {
                    let a6674 = v0;
                    let a6675 = 2u64;
                    if nat_lt(a6674, a6675) {
                        let a6676 = v0;
                        let a6677 = 1u64;
                        if nat_lt(a6676, a6677) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6678 = v0;
                        let a6679 = 3u64;
                        if nat_lt(a6678, a6679) {
                            4u64
                        } else {
                            let a6680 = v0;
                            let a6681 = 4u64;
                            if nat_lt(a6680, a6681) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6682 = v0;
                    let a6683 = 7u64;
                    if nat_lt(a6682, a6683) {
                        let a6684 = v0;
                        let a6685 = 6u64;
                        if nat_lt(a6684, a6685) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6686 = v0;
                        let a6687 = 8u64;
                        if nat_lt(a6686, a6687) {
                            128u64
                        } else {
                            let a6688 = v0;
                            let a6689 = 9u64;
                            if nat_lt(a6688, a6689) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6690 = v0;
                let a6691 = 15u64;
                if nat_lt(a6690, a6691) {
                    let a6692 = v0;
                    let a6693 = 12u64;
                    if nat_lt(a6692, a6693) {
                        let a6694 = v0;
                        let a6695 = 11u64;
                        if nat_lt(a6694, a6695) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6696 = v0;
                        let a6697 = 13u64;
                        if nat_lt(a6696, a6697) {
                            4096u64
                        } else {
                            let a6698 = v0;
                            let a6699 = 14u64;
                            if nat_lt(a6698, a6699) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6700 = v0;
                    let a6701 = 17u64;
                    if nat_lt(a6700, a6701) {
                        let a6702 = v0;
                        let a6703 = 16u64;
                        if nat_lt(a6702, a6703) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6704 = v0;
                        let a6705 = 18u64;
                        if nat_lt(a6704, a6705) {
                            131072u64
                        } else {
                            let a6706 = v0;
                            let a6707 = 19u64;
                            if nat_lt(a6706, a6707) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6708 = v0;
            let a6709 = 30u64;
            if nat_lt(a6708, a6709) {
                let a6710 = v0;
                let a6711 = 25u64;
                if nat_lt(a6710, a6711) {
                    let a6712 = v0;
                    let a6713 = 22u64;
                    if nat_lt(a6712, a6713) {
                        let a6714 = v0;
                        let a6715 = 21u64;
                        if nat_lt(a6714, a6715) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6716 = v0;
                        let a6717 = 23u64;
                        if nat_lt(a6716, a6717) {
                            4194304u64
                        } else {
                            let a6718 = v0;
                            let a6719 = 24u64;
                            if nat_lt(a6718, a6719) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6720 = v0;
                    let a6721 = 27u64;
                    if nat_lt(a6720, a6721) {
                        let a6722 = v0;
                        let a6723 = 26u64;
                        if nat_lt(a6722, a6723) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6724 = v0;
                        let a6725 = 28u64;
                        if nat_lt(a6724, a6725) {
                            134217728u64
                        } else {
                            let a6726 = v0;
                            let a6727 = 29u64;
                            if nat_lt(a6726, a6727) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6728 = v0;
                let a6729 = 35u64;
                if nat_lt(a6728, a6729) {
                    let a6730 = v0;
                    let a6731 = 32u64;
                    if nat_lt(a6730, a6731) {
                        let a6732 = v0;
                        let a6733 = 31u64;
                        if nat_lt(a6732, a6733) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6734 = v0;
                        let a6735 = 33u64;
                        if nat_lt(a6734, a6735) {
                            4294967296u64
                        } else {
                            let a6736 = v0;
                            let a6737 = 34u64;
                            if nat_lt(a6736, a6737) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6738 = v0;
                    let a6739 = 37u64;
                    if nat_lt(a6738, a6739) {
                        let a6740 = v0;
                        let a6741 = 36u64;
                        if nat_lt(a6740, a6741) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6742 = v0;
                        let a6743 = 38u64;
                        if nat_lt(a6742, a6743) {
                            137438953472u64
                        } else {
                            let a6744 = v0;
                            let a6745 = 39u64;
                            if nat_lt(a6744, a6745) {
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

pub fn f265(v0: u64, v1: u64) -> bool {
    let a6746 = v0;
    let a6747 = 1u64;
    if nat_le(a6746, a6747) {
        true
    } else {
        let a6751 = {
            let a6748 = v1;
            let a6749 = 4u64;
            let a6750 = 0u64;
            nat_rem(a6748, a6749, a6750)
        };
        let a6752 = 0u64;
        nat_eq(a6751, a6752)
    }
}

pub fn f266(v0: u64, v1: u64) -> bool {
    let a6753 = v0;
    let a6754 = 1u64;
    if nat_le(a6753, a6754) {
        true
    } else {
        let a6755 = 4u64;
        let a6756 = v1;
        nat_le(a6755, a6756)
    }
}

pub fn f267(v0: u64, v1: u64) -> R<u64> {
    let a6757 = v0;
    let a6758 = 0u64;
    if nat_eq(a6757, a6758) {
        Ok(0u64)
    } else {
        let a6763 = 4u64;
        let a6764 = {
            let a6761 = {
                let a6759 = v0;
                let a6760 = 1u64;
                nat_sub(a6759, a6760)
            };
            let a6762 = v1;
            nat_mul(a6761, a6762)?
        };
        nat_add(a6763, a6764)
    }
}

pub fn f268(v0: bool, v1: u64) -> R<u64> {
    let a6769 = {
        let a6767 = {
            let a6765 = f288(v0, v1)?;
            let a6766 = f289(v0, v1)?;
            nat_add(a6765, a6766)?
        };
        let a6768 = f290(v0);
        nat_add(a6767, a6768)?
    };
    let a6770 = v1;
    nat_add(a6769, a6770)
}

pub fn f269(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f291(v0, v1)?;
    let v3: u64 = {
        let a6775 = {
            let a6773 = {
                let a6771 = v1;
                let a6772 = 1u64;
                nat_sub(a6771, a6772)
            };
            let a6774 = v2;
            nat_mul(a6773, a6774)?
        };
        let a6776 = v0;
        nat_sub(a6775, a6776)
    };
    let a6777 = v2;
    let a6778 = v3;
    if nat_lt(a6777, a6778) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f270(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6779 = v4;
    let a6780 = 0u64;
    if nat_eq(a6779, a6780) {
        Ok(0u64)
    } else {
        let a6787 = 2u64;
        let a6788 = {
            let a6785 = {
                let a6781 = v0;
                let a6782 = v1;
                nat_mul(a6781, a6782)?
            };
            let a6786 = {
                let a6783 = v2;
                let a6784 = v3;
                nat_mul(a6783, a6784)?
            };
            nat_mul(a6785, a6786)?
        };
        nat_mul(a6787, a6788)
    }
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6793 = v3;
    let a6794 = {
        let a6791 = {
            let a6789 = f292(v2, v4)?;
            let a6790 = f293(v0, v1, f291(v2, v4)?)?;
            nat_mul(a6789, a6790)?
        };
        let a6792 = f293(v0, v1, f294(v2, v4)?)?;
        nat_add(a6791, a6792)?
    };
    nat_mul(a6793, a6794)
}

pub fn f272(v0: u64) -> R<u64> {
    let a6797 = 13u64;
    let a6798 = {
        let a6795 = 9u64;
        let a6796 = v0;
        nat_mul(a6795, a6796)?
    };
    nat_add(a6797, a6798)
}

pub fn f273(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f295(v0, v1, v6, v7, v8) {
        let a6799 = v5;
        let a6800 = 26u64;
        nat_eq(a6799, a6800)
    } else {
        false
    } {
        let a6801 = v3;
        let a6802 = v7;
        nat_eq(a6801, a6802)
    } else {
        false
    } {
        let a6803 = v4;
        let a6804 = v8;
        nat_eq(a6803, a6804)
    } else {
        false
    } {
        let a6805 = v9;
        let a6806 = 1u64;
        nat_eq(a6805, a6806)
    } else {
        false
    } {
        let a6807 = v10;
        let a6808 = f296(v0);
        nat_eq(a6807, a6808)
    } else {
        false
    } {
        let a6809 = v11;
        let a6810 = v1;
        nat_eq(a6809, a6810)
    } else {
        false
    } {
        let a6811 = v12;
        let a6812 = v6;
        nat_eq(a6811, a6812)
    } else {
        false
    } {
        let a6813 = v13;
        let a6814 = v7;
        nat_eq(a6813, a6814)
    } else {
        false
    } {
        let a6815 = v14;
        let a6816 = v8;
        nat_eq(a6815, a6816)
    } else {
        false
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6817 = v0;
    let a6818 = 0u64;
    if if if nat_eq(a6817, a6818) {
        true
    } else {
        let a6819 = v1;
        let a6820 = 0u64;
        nat_eq(a6819, a6820)
    } {
        true
    } else {
        let a6821 = v2;
        let a6822 = 0u64;
        nat_eq(a6821, a6822)
    } {
        true
    } else {
        let a6823 = v3;
        let a6824 = 0u64;
        nat_eq(a6823, a6824)
    }
}

pub fn f275(v0: Adt3, v1: bool) -> R<u64> {
    f279(v0.clone(), {
        let a6826 = match v0.clone() {
            Adt3::C0(h6825, _, _, _, _, _, _, _) => {
                h6825
            }
        };
        let a6827 = 1u64;
        nat_sub(a6826, a6827)
    }, {
        let a6829 = match v0.clone() {
            Adt3::C0(_, h6828, _, _, _, _, _, _) => {
                h6828
            }
        };
        let a6830 = 1u64;
        nat_sub(a6829, a6830)
    }, {
        let a6832 = match v0.clone() {
            Adt3::C0(_, _, h6831, _, _, _, _, _) => {
                h6831
            }
        };
        let a6833 = 1u64;
        nat_sub(a6832, a6833)
    }, {
        let a6835 = match v0 {
            Adt3::C0(_, _, _, h6834, _, _, _, _) => {
                h6834
            }
        };
        let a6836 = 1u64;
        nat_sub(a6835, a6836)
    }, v1)
}

pub fn f276(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6837 = v0;
    let a6838 = 0u64;
    if nat_eq(a6837, a6838) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6839 = v0;
            let a6840 = 1u64;
            nat_sub(a6839, a6840)
        };
        let v5: u64 = {
            let a6841 = v4;
            let a6842 = v1;
            let a6843 = 0u64;
            nat_quot(a6841, a6842, a6843)
        };
        let v6: u64 = {
            let a6851 = {
                let a6844 = v5;
                let a6845 = v2;
                nat_mul(a6844, a6845)?
            };
            let a6852 = {
                let a6849 = {
                    let a6846 = v4;
                    let a6847 = v1;
                    let a6848 = 0u64;
                    nat_rem(a6846, a6847, a6848)
                };
                let a6850 = v3;
                nat_mul(a6849, a6850)?
            };
            nat_add(a6851, a6852)?
        };
        let a6853 = v5;
        let a6854 = 0u64;
        if nat_eq(a6853, a6854) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6863 = {
                    let a6857 = {
                        let a6855 = v5;
                        let a6856 = 1u64;
                        nat_sub(a6855, a6856)
                    };
                    let a6858 = v2;
                    nat_mul(a6857, a6858)?
                };
                let a6864 = {
                    let a6861 = {
                        let a6859 = v1;
                        let a6860 = 1u64;
                        nat_sub(a6859, a6860)
                    };
                    let a6862 = v3;
                    nat_mul(a6861, a6862)?
                };
                nat_add(a6863, a6864)?
            };
            let a6865 = v6;
            let a6866 = v7;
            if nat_lt(a6865, a6866) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f277(v0: Adt3, v1: u64) -> bool {
    let a6867 = v1;
    let a6868 = 0u64;
    if !nat_eq(a6867, a6868) {
        true
    } else {
        let a6870 = match v0 {
            Adt3::C0(_, _, _, _, h6869, _, _, _) => {
                h6869
            }
        };
        let a6871 = 4u64;
        nat_eq(a6870, a6871)
    }
}

pub fn f278(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6872 = f297(v0, v1);
    match m6872 {
        None => {
            if v5 {
                let m6873 = f298(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6873 {
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
                let a6874 = 4u64;
                let a6875 = v28;
                nat_le(a6874, a6875)
            };
            let a6876 = v2;
            let a6877 = 0u64;
            if if if if if nat_eq(a6876, a6877) {
                let a6878 = v4;
                let a6879 = 0u64;
                nat_eq(a6878, a6879)
            } else {
                false
            } {
                let a6880 = v6;
                let a6881 = v14;
                if nat_eq(a6880, a6881) {
                    let a6882 = v7;
                    let a6883 = v15;
                    nat_eq(a6882, a6883)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6884 = v8;
                let a6885 = v16;
                if nat_eq(a6884, a6885) {
                    let a6886 = v9;
                    let a6887 = v17;
                    nat_eq(a6886, a6887)
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
                        let a6888 = v3;
                        let a6889 = 0u64;
                        nat_eq(a6888, a6889)
                    } else {
                        false
                    } {
                        f229(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6894 = {
                        let a6890 = v6;
                        let a6891 = v7;
                        nat_mul(a6890, a6891)?
                    };
                    let a6895 = {
                        let a6892 = v8;
                        let a6893 = v9;
                        nat_mul(a6892, a6893)?
                    };
                    nat_mul(a6894, a6895)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f279(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6916 = {
        let a6910 = {
            let a6904 = {
                let a6898 = v1;
                let a6899 = if v5 {
                    f145(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6896, _, _, _) => {
                            h6896
                        }
                    })
                } else {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6897, _, _, _) => {
                            h6897
                        }
                    })
                };
                nat_mul(a6898, a6899)?
            };
            let a6905 = {
                let a6902 = v2;
                let a6903 = if v5 {
                    f145(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6900, _, _) => {
                            h6900
                        }
                    })
                } else {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6901, _, _) => {
                            h6901
                        }
                    })
                };
                nat_mul(a6902, a6903)?
            };
            nat_add(a6904, a6905)?
        };
        let a6911 = {
            let a6908 = v3;
            let a6909 = if v5 {
                f145(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6906, _) => {
                        h6906
                    }
                })
            } else {
                f146(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6907, _) => {
                        h6907
                    }
                })
            };
            nat_mul(a6908, a6909)?
        };
        nat_add(a6910, a6911)?
    };
    let a6917 = {
        let a6914 = v4;
        let a6915 = if v5 {
            f145(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6912) => {
                    h6912
                }
            })
        } else {
            f146(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6913) => {
                    h6913
                }
            })
        };
        nat_mul(a6914, a6915)?
    };
    nat_add(a6916, a6917)
}

pub fn f280(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6918 = v1;
    let a6919 = 0u64;
    if nat_eq(a6918, a6919) {
        Ok(Adt9::C4({
            let a6920 = v3;
            let a6921 = 0u64;
            if nat_eq(a6920, a6921) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f93(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6922 = v3;
            let a6923 = 0u64;
            if nat_eq(a6922, a6923) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f281(v0: u64, v1: u64) -> R<u64> {
    let a6924 = v0;
    let a6925 = f96(v1)?;
    let a6926 = 0u64;
    Ok(nat_quot(a6924, a6925, a6926))
}

pub fn f282(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f281(v0, v1)?;
    let v3: u64 = {
        let a6929 = {
            let a6927 = f252(v0, v1)?;
            let a6928 = 2u64;
            nat_sub(a6927, a6928)
        };
        let a6930 = 2u64;
        let a6931 = 0u64;
        nat_quot(a6929, a6930, a6931)
    };
    let v4: u64 = {
        let a6932 = v2;
        let a6933 = v1;
        nat_mul(a6932, a6933)?
    };
    if f255(v0, v1)? {
        let a6934 = v4;
        let a6935 = v3;
        nat_add(a6934, a6935)
    } else {
        let a6947 = {
            let a6943 = v4;
            let a6944 = {
                let a6941 = {
                    let a6936 = v1;
                    let a6937 = 1u64;
                    nat_sub(a6936, a6937)
                };
                let a6942 = {
                    let a6938 = v2;
                    let a6939 = v1;
                    let a6940 = 0u64;
                    nat_rem(a6938, a6939, a6940)
                };
                nat_mul(a6941, a6942)?
            };
            nat_sub(a6943, a6944)
        };
        let a6948 = {
            let a6945 = v3;
            let a6946 = v1;
            nat_mul(a6945, a6946)?
        };
        nat_add(a6947, a6948)
    }
}

pub fn f283(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6949 = v0;
    let a6950 = 0u64;
    if if if nat_eq(a6949, a6950) {
        true
    } else {
        let a6951 = v1;
        let a6952 = 0u64;
        nat_eq(a6951, a6952)
    } {
        true
    } else {
        let a6953 = v2;
        let a6954 = 0u64;
        nat_eq(a6953, a6954)
    } {
        Ok(0u64)
    } else {
        let a6957 = {
            let a6955 = v0;
            let a6956 = v1;
            nat_mul(a6955, a6956)?
        };
        let a6958 = v2;
        nat_mul(a6957, a6958)
    }
}

pub fn f284(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f105(match v1.clone() {
        Adt20::C0(h6959, _, _, _, _, _, _, _) => {
            h6959
        }
    })?;
    let v6: u64 = {
        let a6965 = match v1.clone() {
            Adt20::C0(_, h6960, _, _, _, _, _, _) => {
                h6960
            }
        };
        let a6966 = {
            let a6963 = match v1.clone() {
                Adt20::C0(_, _, h6961, _, _, _, _, _) => {
                    h6961
                }
            };
            let a6964 = match v1.clone() {
                Adt20::C0(_, _, _, h6962, _, _, _, _) => {
                    h6962
                }
            };
            nat_mul(a6963, a6964)?
        };
        nat_mul(a6965, a6966)?
    };
    let a6968 = v0;
    let a6969 = f53(match v1.clone() {
        Adt20::C0(h6967, _, _, _, _, _, _, _) => {
            h6967
        }
    }, v6)?;
    if nat_lt(a6968, a6969) {
        f299({
            let a6970 = v0;
            let a6971 = v5;
            let a6972 = 0u64;
            nat_rem(a6970, a6971, a6972)
        }, {
            let a6973 = v0;
            let a6974 = v5;
            let a6975 = 0u64;
            nat_quot(a6973, a6974, a6975)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f285(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6976 = v6;
        let a6977 = 6u64;
        nat_sub(a6976, a6977)
    };
    let v9: Adt20 = f300(v3);
    let a6979 = v0;
    let a6980 = f215(match v1.clone() {
        Adt20::C0(h6978, _, _, _, _, _, _, _) => {
            h6978
        }
    }, v5, v8)?;
    if nat_lt(a6979, a6980) {
        let v10: u64 = f102(match v1.clone() {
            Adt20::C0(h6981, _, _, _, _, _, _, _) => {
                h6981
            }
        }, v5, v8)?;
        let a6982 = v0;
        let a6983 = v10;
        if nat_lt(a6982, a6983) {
            f301(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6984 = v0;
                let a6985 = v10;
                nat_sub(a6984, a6985)
            };
            let v12: u64 = f272(match v1.clone() {
                Adt20::C0(h6986, _, _, _, _, _, _, _) => {
                    h6986
                }
            })?;
            f302({
                let a6987 = v11;
                let a6988 = v12;
                let a6989 = 0u64;
                nat_rem(a6987, a6988, a6989)
            }, {
                let a6990 = v11;
                let a6991 = v12;
                let a6992 = 0u64;
                nat_quot(a6990, a6991, a6992)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f286(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6993 = v5;
        let a6994 = 6u64;
        nat_sub(a6993, a6994)
    };
    let v8: bool = f209(match v1.clone() {
        Adt20::C0(_, _, _, _, h6995, _, _, _) => {
            h6995
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6996, _, _, _) => {
            h6996
        }
    });
    let a6998 = v0;
    let a6999 = f208(v8, match v1.clone() {
        Adt20::C0(h6997, _, _, _, _, _, _, _) => {
            h6997
        }
    }, v4, v7)?;
    if nat_lt(a6998, a6999) {
        let v9: u64 = f102(match v1.clone() {
            Adt20::C0(h7000, _, _, _, _, _, _, _) => {
                h7000
            }
        }, v4, v7)?;
        let a7001 = v0;
        let a7002 = v9;
        if nat_lt(a7001, a7002) {
            f301(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a7003 = v0;
                let a7004 = v9;
                nat_sub(a7003, a7004)
            };
            let v11: u64 = f268(v8, match v1.clone() {
                Adt20::C0(h7005, _, _, _, _, _, _, _) => {
                    h7005
                }
            })?;
            f303({
                let a7006 = v10;
                let a7007 = v11;
                let a7008 = 0u64;
                nat_rem(a7006, a7007, a7008)
            }, {
                let a7009 = v10;
                let a7010 = v11;
                let a7011 = 0u64;
                nat_quot(a7009, a7010, a7011)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f287(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7012 = v5;
        let a7013 = 6u64;
        nat_sub(a7012, a7013)
    };
    let v8: Adt20 = f300(v2);
    let v9: u64 = f270(match v1.clone() {
        Adt20::C0(h7014, _, _, _, _, _, _, _) => {
            h7014
        }
    }, match v1.clone() {
        Adt20::C0(_, h7015, _, _, _, _, _, _) => {
            h7015
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7016, _, _, _, _, _) => {
            h7016
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7017, _, _, _, _) => {
            h7017
        }
    }, v7)?;
    let a7022 = v0;
    let a7023 = f214(match v1.clone() {
        Adt20::C0(h7018, _, _, _, _, _, _, _) => {
            h7018
        }
    }, match v1.clone() {
        Adt20::C0(_, h7019, _, _, _, _, _, _) => {
            h7019
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7020, _, _, _, _, _) => {
            h7020
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7021, _, _, _, _) => {
            h7021
        }
    }, v4, v7)?;
    if nat_lt(a7022, a7023) {
        let a7024 = v0;
        let a7025 = v9;
        if nat_lt(a7024, a7025) {
            f301(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f304({
                let a7026 = v0;
                let a7027 = v9;
                nat_sub(a7026, a7027)
            }, match v1.clone() {
                Adt20::C0(h7028, _, _, _, _, _, _, _) => {
                    h7028
                }
            }, match v1.clone() {
                Adt20::C0(_, h7029, _, _, _, _, _, _) => {
                    h7029
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h7030, _, _, _, _, _) => {
                    h7030
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h7031, _, _, _, _) => {
                    h7031
                }
            }, v4)?;
            f305(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f288(v0: bool, v1: u64) -> R<u64> {
    let a7034 = if v0 {
        5u64
    } else {
        4u64
    };
    let a7035 = {
        let a7032 = f306(v0);
        let a7033 = v1;
        nat_mul(a7032, a7033)?
    };
    nat_add(a7034, a7035)
}

pub fn f289(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a7043 = {
            let a7036 = 13u64;
            let a7037 = f307(v1);
            nat_mul(a7036, a7037)?
        };
        let a7044 = {
            let a7041 = 4u64;
            let a7042 = {
                let a7038 = v1;
                let a7039 = 4u64;
                let a7040 = 0u64;
                nat_rem(a7038, a7039, a7040)
            };
            nat_mul(a7041, a7042)?
        };
        nat_add(a7043, a7044)
    } else {
        let a7045 = 3u64;
        let a7046 = v1;
        nat_mul(a7045, a7046)
    }
}

pub fn f290(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f291(v0: u64, v1: u64) -> R<u64> {
    let a7051 = {
        let a7049 = {
            let a7047 = v0;
            let a7048 = v1;
            nat_add(a7047, a7048)?
        };
        let a7050 = 1u64;
        nat_sub(a7049, a7050)
    };
    let a7052 = v1;
    let a7053 = 0u64;
    Ok(nat_quot(a7051, a7052, a7053))
}

pub fn f292(v0: u64, v1: u64) -> R<u64> {
    let a7054 = v0;
    let a7055 = f291(v0, v1)?;
    let a7056 = 0u64;
    Ok(nat_quot(a7054, a7055, a7056))
}

pub fn f293(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7057 = v2;
    let a7058 = 0u64;
    if nat_eq(a7057, a7058) {
        Ok(0u64)
    } else {
        let a7063 = 11u64;
        let a7064 = {
            let a7061 = {
                let a7059 = v1;
                let a7060 = v2;
                nat_mul(a7059, a7060)?
            };
            let a7062 = f308(v0)?;
            nat_mul(a7061, a7062)?
        };
        nat_add(a7063, a7064)
    }
}

pub fn f294(v0: u64, v1: u64) -> R<u64> {
    let a7065 = v0;
    let a7066 = f291(v0, v1)?;
    let a7067 = 0u64;
    Ok(nat_rem(a7065, a7066, a7067))
}

pub fn f295(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f274(v1, v2, v3, v4) {
        f309(v0)
    } else {
        f226(v0)
    }
}

pub fn f296(v0: u64) -> u64 {
    let a7068 = v0;
    let a7069 = 26u64;
    if nat_eq(a7068, a7069) {
        26u64
    } else {
        0u64
    }
}

pub fn f297(v0: u64, v1: u64) -> Option<u64> {
    let a7070 = v0;
    let a7071 = 2u64;
    if nat_eq(a7070, a7071) {
        Some(0u64)
    } else {
        let a7072 = v0;
        let a7073 = 5u64;
        if nat_eq(a7072, a7073) {
            Some(1u64)
        } else {
            let a7074 = v0;
            let a7075 = 6u64;
            if nat_eq(a7074, a7075) {
                Some(2u64)
            } else {
                let a7076 = v0;
                let a7077 = 7u64;
                if nat_eq(a7076, a7077) {
                    Some(3u64)
                } else {
                    let a7078 = v0;
                    let a7079 = 9u64;
                    if nat_eq(a7078, a7079) {
                        Some(4u64)
                    } else {
                        let a7080 = v0;
                        let a7081 = 8u64;
                        if nat_eq(a7080, a7081) {
                            Some(5u64)
                        } else {
                            let a7082 = v0;
                            let a7083 = 89u64;
                            if nat_eq(a7082, a7083) {
                                let a7084 = v1;
                                let a7085 = 2u64;
                                if nat_eq(a7084, a7085) {
                                    Some(6u64)
                                } else {
                                    let a7086 = v1;
                                    let a7087 = 0u64;
                                    if nat_eq(a7086, a7087) {
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

pub fn f298(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7088 = v0;
    let a7089 = 30u64;
    if nat_eq(a7088, a7089) {
        let a7090 = v2;
        let a7091 = 0u64;
        if if if nat_eq(a7090, a7091) {
            true
        } else {
            let a7092 = v2;
            let a7093 = 1u64;
            nat_eq(a7092, a7093)
        } {
            let a7094 = v1;
            let a7095 = 0u64;
            if nat_eq(a7094, a7095) {
                true
            } else {
                f310(v1, v4)
            }
        } else {
            false
        } {
            f311(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f312(v1) {
            f312(v2)
        } else {
            false
        } {
            let a7096 = v1;
            let a7097 = 0u64;
            if if nat_eq(a7096, a7097) {
                true
            } else {
                let a7098 = v2;
                let a7099 = 0u64;
                nat_eq(a7098, a7099)
            } {
                true
            } else {
                let a7100 = v1;
                let a7101 = v2;
                nat_eq(a7100, a7101)
            }
        } else {
            false
        } {
            true
        } else {
            let a7102 = v1;
            let a7103 = 43u64;
            if nat_eq(a7102, a7103) {
                f312(v2)
            } else {
                false
            }
        } {
            f311(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f299(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h7104, _, _, _, _, _, _, _) => {
            h7104
        }
    };
    let v7: u64 = {
        let a7107 = 1u64;
        let a7108 = {
            let a7105 = 3u64;
            let a7106 = v6;
            nat_mul(a7105, a7106)?
        };
        nat_add(a7107, a7108)?
    };
    let a7109 = v0;
    let a7110 = 0u64;
    if nat_eq(a7109, a7110) {
        Ok(f313(0u64))
    } else {
        let a7111 = v0;
        let a7112 = v7;
        if nat_lt(a7111, a7112) {
            let v8: u64 = {
                let a7113 = v0;
                let a7114 = 1u64;
                nat_sub(a7113, a7114)
            };
            let v9: u64 = {
                let a7120 = {
                    let a7115 = v1;
                    let a7116 = v6;
                    nat_mul(a7115, a7116)?
                };
                let a7121 = {
                    let a7117 = v8;
                    let a7118 = 3u64;
                    let a7119 = 0u64;
                    nat_quot(a7117, a7118, a7119)
                };
                nat_add(a7120, a7121)?
            };
            let v10: u64 = f314(v9, v2)?;
            let v11: u64 = {
                let a7122 = v8;
                let a7123 = 3u64;
                let a7124 = 0u64;
                nat_rem(a7122, a7123, a7124)
            };
            let a7125 = v11;
            let a7126 = 0u64;
            if nat_eq(a7125, a7126) {
                Ok(f315(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a7127 = v11;
                let a7128 = 1u64;
                if nat_eq(a7127, a7128) {
                    Ok(f315(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f315(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a7129 = v0;
            let a7130 = v7;
            if nat_eq(a7129, a7130) {
                Ok(f315(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a7133 = v0;
                let a7134 = {
                    let a7131 = v7;
                    let a7132 = 1u64;
                    nat_add(a7131, a7132)?
                };
                if nat_eq(a7133, a7134) {
                    Ok(f315(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7137 = v0;
                    let a7138 = {
                        let a7135 = v7;
                        let a7136 = 2u64;
                        nat_add(a7135, a7136)?
                    };
                    if nat_eq(a7137, a7138) {
                        Ok(f315(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a7141 = v0;
                        let a7142 = {
                            let a7139 = v7;
                            let a7140 = 3u64;
                            nat_add(a7139, a7140)?
                        };
                        if nat_eq(a7141, a7142) {
                            let a7143 = v5;
                            let a7144 = 1u64;
                            if if nat_eq(a7143, a7144) {
                                true
                            } else {
                                let a7145 = v5;
                                let a7146 = 2u64;
                                nat_eq(a7145, a7146)
                            } {
                                Ok(f315(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f315(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7153 = {
                                    let a7147 = v1;
                                    let a7148 = v6;
                                    nat_mul(a7147, a7148)?
                                };
                                let a7154 = {
                                    let a7151 = v0;
                                    let a7152 = {
                                        let a7149 = v7;
                                        let a7150 = 4u64;
                                        nat_add(a7149, a7150)?
                                    };
                                    nat_sub(a7151, a7152)
                                };
                                nat_add(a7153, a7154)?
                            };
                            Ok(f315(10u64, 0u64, f314(v12, v2)?, 1u64, 2u64, 3u64, f314(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f300(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h7155, _, _, _, _, _, _, _) => {
            h7155
        }
    }, match v0.clone() {
        Adt20::C0(_, h7156, _, _, _, _, _, _) => {
            h7156
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h7157, _, _, _, _, _) => {
            h7157
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h7158, _, _, _, _) => {
            h7158
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h7159, _, _) => {
            h7159
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h7160, _) => {
            h7160
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h7161) => {
            h7161
        }
    })
}

pub fn f301(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a7168 = v3;
        let a7169 = {
            let a7165 = f314({
                let a7162 = v0;
                let a7163 = 2u64;
                let a7164 = 0u64;
                nat_quot(a7162, a7163, a7164)
            }, v1)?;
            let a7166 = v2;
            let a7167 = 0u64;
            nat_quot(a7165, a7166, a7167)
        };
        nat_add(a7168, a7169)?
    };
    let a7173 = {
        let a7170 = v0;
        let a7171 = 2u64;
        let a7172 = 0u64;
        nat_rem(a7170, a7171, a7172)
    };
    let a7174 = 0u64;
    if nat_eq(a7173, a7174) {
        Ok(f315(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a7175 = v4;
        let a7176 = 0u64;
        if nat_eq(a7175, a7176) {
            Ok(f315(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f302(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f316(match v2.clone() {
        Adt20::C0(h7177, _, _, _, _, _, _, _) => {
            h7177
        }
    })?;
    let v7: u64 = f317(match v2.clone() {
        Adt20::C0(h7178, _, _, _, _, _, _, _) => {
            h7178
        }
    })?;
    let a7179 = v0;
    let a7180 = 2u64;
    if nat_lt(a7179, a7180) {
        Ok(f313(v0))
    } else {
        let a7181 = v0;
        let a7182 = v6;
        if nat_lt(a7181, a7182) {
            f318({
                let a7183 = v0;
                let a7184 = 2u64;
                nat_sub(a7183, a7184)
            }, v1, v2, v3)
        } else {
            let a7185 = v0;
            let a7186 = v7;
            if nat_lt(a7185, a7186) {
                Ok(f319({
                    let a7187 = v0;
                    let a7188 = v6;
                    nat_sub(a7187, a7188)
                }, match v2 {
                    Adt20::C0(h7189, _, _, _, _, _, _, _) => {
                        h7189
                    }
                }, v5))
            } else {
                let a7191 = v0;
                let a7192 = f272(match v2.clone() {
                    Adt20::C0(h7190, _, _, _, _, _, _, _) => {
                        h7190
                    }
                })?;
                if nat_lt(a7191, a7192) {
                    f320({
                        let a7193 = v0;
                        let a7194 = v7;
                        nat_sub(a7193, a7194)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f303(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f288(v5, match v2.clone() {
        Adt20::C0(h7195, _, _, _, _, _, _, _) => {
            h7195
        }
    })?;
    let v7: u64 = {
        let a7197 = v6;
        let a7198 = f289(v5, match v2.clone() {
            Adt20::C0(h7196, _, _, _, _, _, _, _) => {
                h7196
            }
        })?;
        nat_add(a7197, a7198)?
    };
    let v8: u64 = {
        let a7199 = v7;
        let a7200 = f290(v5);
        nat_add(a7199, a7200)?
    };
    let a7201 = v0;
    let a7202 = v6;
    if nat_lt(a7201, a7202) {
        f321(v0, v1, v2, v5)
    } else {
        let a7203 = v0;
        let a7204 = v7;
        if nat_lt(a7203, a7204) {
            f322({
                let a7205 = v0;
                let a7206 = v6;
                nat_sub(a7205, a7206)
            }, v1, v2, v3, v5)
        } else {
            let a7207 = v0;
            let a7208 = v8;
            if nat_lt(a7207, a7208) {
                Ok(f323({
                    let a7209 = v0;
                    let a7210 = v7;
                    nat_sub(a7209, a7210)
                }, match v2 {
                    Adt20::C0(h7211, _, _, _, _, _, _, _) => {
                        h7211
                    }
                }, v4, v5))
            } else {
                let a7215 = v0;
                let a7216 = {
                    let a7213 = v8;
                    let a7214 = match v2.clone() {
                        Adt20::C0(h7212, _, _, _, _, _, _, _) => {
                            h7212
                        }
                    };
                    nat_add(a7213, a7214)?
                };
                if nat_lt(a7215, a7216) {
                    let v9: u64 = f314({
                        let a7222 = {
                            let a7218 = v1;
                            let a7219 = match v2 {
                                Adt20::C0(h7217, _, _, _, _, _, _, _) => {
                                    h7217
                                }
                            };
                            nat_mul(a7218, a7219)?
                        };
                        let a7223 = {
                            let a7220 = v0;
                            let a7221 = v8;
                            nat_sub(a7220, a7221)
                        };
                        nat_add(a7222, a7223)?
                    }, v3)?;
                    Ok(f315(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f304(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f291(v3, v5)?;
    let v7: u64 = f293(v1, v2, v6)?;
    let v8: u64 = {
        let a7226 = {
            let a7224 = f292(v3, v5)?;
            let a7225 = v4;
            nat_mul(a7224, a7225)?
        };
        let a7227 = v7;
        nat_mul(a7226, a7227)?
    };
    let a7228 = v0;
    let a7229 = v8;
    if nat_lt(a7228, a7229) {
        let v9: u64 = {
            let a7230 = v0;
            let a7231 = v7;
            let a7232 = 0u64;
            nat_quot(a7230, a7231, a7232)
        };
        Ok(Adt21::C0({
            let a7236 = {
                let a7233 = v9;
                let a7234 = v4;
                let a7235 = 0u64;
                nat_quot(a7233, a7234, a7235)
            };
            let a7237 = v6;
            nat_mul(a7236, a7237)?
        }, v6, {
            let a7238 = v9;
            let a7239 = v4;
            let a7240 = 0u64;
            nat_rem(a7238, a7239, a7240)
        }, {
            let a7241 = v0;
            let a7242 = v7;
            let a7243 = 0u64;
            nat_rem(a7241, a7242, a7243)
        }))
    } else {
        let v10: u64 = {
            let a7244 = v0;
            let a7245 = v8;
            nat_sub(a7244, a7245)
        };
        let v11: u64 = f293(v1, v2, f294(v3, v5)?)?;
        Ok(Adt21::C0({
            let a7246 = f292(v3, v5)?;
            let a7247 = v6;
            nat_mul(a7246, a7247)?
        }, f294(v3, v5)?, {
            let a7248 = v10;
            let a7249 = v11;
            let a7250 = 0u64;
            nat_quot(a7248, a7249, a7250)
        }, {
            let a7251 = v10;
            let a7252 = v11;
            let a7253 = 0u64;
            nat_rem(a7251, a7252, a7253)
        }))
    }
}

pub fn f305(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7254) => {
            h7254
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7255, _, _, _, _, _, _, _) => {
            h7255
        }
    };
    let v6: u64 = {
        let a7258 = match v0.clone() {
            Adt21::C0(_, h7256, _, _) => {
                h7256
            }
        };
        let a7259 = match v1.clone() {
            Adt20::C0(_, h7257, _, _, _, _, _, _) => {
                h7257
            }
        };
        nat_mul(a7258, a7259)?
    };
    let v7: u64 = {
        let a7260 = v6;
        let a7261 = v5;
        nat_mul(a7260, a7261)?
    };
    let v8: u64 = {
        let a7264 = {
            let a7262 = 2u64;
            let a7263 = v5;
            nat_mul(a7262, a7263)?
        };
        let a7265 = 2u64;
        nat_add(a7264, a7265)?
    };
    let v9: u64 = {
        let a7268 = {
            let a7266 = 4u64;
            let a7267 = v5;
            nat_mul(a7266, a7267)?
        };
        let a7269 = 2u64;
        nat_add(a7268, a7269)?
    };
    let v10: u64 = {
        let a7272 = 1u64;
        let a7273 = {
            let a7270 = v6;
            let a7271 = v8;
            nat_mul(a7270, a7271)?
        };
        nat_add(a7272, a7273)?
    };
    let v11: u64 = {
        let a7274 = v10;
        let a7275 = 4u64;
        nat_add(a7274, a7275)?
    };
    let v12: u64 = {
        let a7278 = v11;
        let a7279 = {
            let a7276 = v6;
            let a7277 = v9;
            nat_mul(a7276, a7277)?
        };
        nat_add(a7278, a7279)?
    };
    let v13: u64 = {
        let a7280 = v12;
        let a7281 = 6u64;
        nat_add(a7280, a7281)?
    };
    let a7282 = v4;
    let a7283 = 0u64;
    if nat_eq(a7282, a7283) {
        Ok(f313(0u64))
    } else {
        let a7284 = v4;
        let a7285 = v10;
        if nat_lt(a7284, a7285) {
            let v14: u64 = {
                let a7286 = v4;
                let a7287 = 1u64;
                nat_sub(a7286, a7287)
            };
            f324(false, {
                let a7288 = v14;
                let a7289 = v8;
                let a7290 = 0u64;
                nat_rem(a7288, a7289, a7290)
            }, {
                let a7291 = v14;
                let a7292 = v8;
                let a7293 = 0u64;
                nat_quot(a7291, a7292, a7293)
            }, v0, v1, v2)
        } else {
            let a7294 = v4;
            let a7295 = v11;
            if nat_lt(a7294, a7295) {
                Ok(f325({
                    let a7296 = v4;
                    let a7297 = v10;
                    nat_sub(a7296, a7297)
                }, v7))
            } else {
                let a7298 = v4;
                let a7299 = v12;
                if nat_lt(a7298, a7299) {
                    let v15: u64 = {
                        let a7300 = v4;
                        let a7301 = v11;
                        nat_sub(a7300, a7301)
                    };
                    f324(true, {
                        let a7302 = v15;
                        let a7303 = v9;
                        let a7304 = 0u64;
                        nat_rem(a7302, a7303, a7304)
                    }, {
                        let a7305 = v15;
                        let a7306 = v9;
                        let a7307 = 0u64;
                        nat_quot(a7305, a7306, a7307)
                    }, v0, v1, v2)
                } else {
                    let a7308 = v4;
                    let a7309 = v13;
                    if nat_lt(a7308, a7309) {
                        Ok(f326({
                            let a7310 = v4;
                            let a7311 = v12;
                            nat_sub(a7310, a7311)
                        }, v7, v3))
                    } else {
                        let a7314 = v4;
                        let a7315 = {
                            let a7312 = v13;
                            let a7313 = v7;
                            nat_add(a7312, a7313)?
                        };
                        if nat_lt(a7314, a7315) {
                            let v16: u64 = {
                                let a7316 = v4;
                                let a7317 = v13;
                                nat_sub(a7316, a7317)
                            };
                            let v17: u64 = {
                                let a7318 = v16;
                                let a7319 = v5;
                                let a7320 = 0u64;
                                nat_quot(a7318, a7319, a7320)
                            };
                            let v18: u64 = {
                                let a7326 = {
                                    let a7321 = f327(v17, v0, v1)?;
                                    let a7322 = v5;
                                    nat_mul(a7321, a7322)?
                                };
                                let a7327 = {
                                    let a7323 = v16;
                                    let a7324 = v5;
                                    let a7325 = 0u64;
                                    nat_rem(a7323, a7324, a7325)
                                };
                                nat_add(a7326, a7327)?
                            };
                            let v19: u64 = f314(v18, v2)?;
                            Ok(f315(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f306(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f307(v0: u64) -> u64 {
    let a7328 = v0;
    let a7329 = 4u64;
    let a7330 = 0u64;
    nat_quot(a7328, a7329, a7330)
}

pub fn f308(v0: u64) -> R<u64> {
    let a7333 = {
        let a7331 = 7u64;
        let a7332 = v0;
        nat_mul(a7331, a7332)?
    };
    let a7334 = 4u64;
    nat_add(a7333, a7334)
}

pub fn f309(v0: u64) -> bool {
    let m7335 = f218(v0);
    match m7335 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f310(v0: u64, v1: u64) -> bool {
    let a7336 = v0;
    let a7337 = 1u64;
    if nat_eq(a7336, a7337) {
        true
    } else {
        let v2: u64 = f328(v0);
        let a7338 = 0u64;
        let a7339 = v2;
        if nat_lt(a7338, a7339) {
            let a7343 = {
                let a7340 = v1;
                let a7341 = v2;
                let a7342 = 1u64;
                nat_rem(a7340, a7341, a7342)
            };
            let a7344 = 0u64;
            nat_eq(a7343, a7344)
        } else {
            false
        }
    }
}

pub fn f311(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7345 = v0;
    let a7346 = 28u64;
    if if if if nat_eq(a7345, a7346) {
        let a7347 = v1;
        let a7348 = 0u64;
        nat_eq(a7347, a7348)
    } else {
        false
    } {
        let a7349 = v2;
        let a7350 = 0u64;
        nat_eq(a7349, a7350)
    } else {
        false
    } {
        let a7351 = v3;
        let a7352 = 0u64;
        nat_eq(a7351, a7352)
    } else {
        false
    } {
        let a7353 = v4;
        let a7354 = v8;
        if if if if if if if if nat_eq(a7353, a7354) {
            let a7355 = v5;
            let a7356 = v12;
            nat_eq(a7355, a7356)
        } else {
            false
        } {
            let a7357 = v9;
            let a7358 = v13;
            nat_eq(a7357, a7358)
        } else {
            false
        } {
            let a7359 = v10;
            let a7360 = v14;
            nat_eq(a7359, a7360)
        } else {
            false
        } {
            let a7361 = v11;
            let a7362 = v15;
            nat_eq(a7361, a7362)
        } else {
            false
        } {
            let a7363 = 0u64;
            let a7364 = v6;
            nat_lt(a7363, a7364)
        } else {
            false
        } {
            let a7365 = 0u64;
            let a7366 = v7;
            nat_lt(a7365, a7366)
        } else {
            false
        } {
            let a7370 = {
                let a7367 = v10;
                let a7368 = v6;
                let a7369 = 1u64;
                nat_rem(a7367, a7368, a7369)
            };
            let a7371 = 0u64;
            if nat_eq(a7370, a7371) {
                let a7375 = {
                    let a7372 = v11;
                    let a7373 = v7;
                    let a7374 = 1u64;
                    nat_rem(a7372, a7373, a7374)
                };
                let a7376 = 0u64;
                nat_eq(a7375, a7376)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7377 = v10;
                let a7378 = v6;
                let a7379 = 0u64;
                nat_quot(a7377, a7378, a7379)
            }, {
                let a7380 = v11;
                let a7381 = v7;
                let a7382 = 0u64;
                nat_quot(a7380, a7381, a7382)
            }, {
                let a7383 = v5;
                let a7384 = v4;
                nat_mul(a7383, a7384)?
            }, {
                let a7385 = v4;
                let a7386 = v9;
                nat_mul(a7385, a7386)?
            }, {
                let a7387 = v5;
                let a7388 = v9;
                nat_mul(a7387, a7388)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f312(v0: u64) -> bool {
    let a7389 = v0;
    let a7390 = 0u64;
    if if nat_eq(a7389, a7390) {
        true
    } else {
        let a7391 = v0;
        let a7392 = 1u64;
        nat_eq(a7391, a7392)
    } {
        true
    } else {
        let a7393 = v0;
        let a7394 = 30u64;
        nat_eq(a7393, a7394)
    }
}

pub fn f313(v0: u64) -> Adt17 {
    f315(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f314(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7396 = v0;
        let a7397 = match v1.clone() {
            Adt20::C0(h7395, _, _, _, _, _, _, _) => {
                h7395
            }
        };
        let a7398 = 0u64;
        nat_quot(a7396, a7397, a7398)
    };
    let v3: u64 = {
        let a7400 = v2;
        let a7401 = match v1.clone() {
            Adt20::C0(_, h7399, _, _, _, _, _, _) => {
                h7399
            }
        };
        let a7402 = 0u64;
        nat_quot(a7400, a7401, a7402)
    };
    f144({
        let a7404 = v0;
        let a7405 = match v1.clone() {
            Adt20::C0(h7403, _, _, _, _, _, _, _) => {
                h7403
            }
        };
        let a7406 = 0u64;
        nat_rem(a7404, a7405, a7406)
    }, {
        let a7408 = v2;
        let a7409 = match v1.clone() {
            Adt20::C0(_, h7407, _, _, _, _, _, _) => {
                h7407
            }
        };
        let a7410 = 0u64;
        nat_rem(a7408, a7409, a7410)
    }, {
        let a7412 = v3;
        let a7413 = match v1.clone() {
            Adt20::C0(_, _, h7411, _, _, _, _, _) => {
                h7411
            }
        };
        let a7414 = 0u64;
        nat_rem(a7412, a7413, a7414)
    }, {
        let a7416 = v3;
        let a7417 = match v1.clone() {
            Adt20::C0(_, _, h7415, _, _, _, _, _) => {
                h7415
            }
        };
        let a7418 = 0u64;
        nat_quot(a7416, a7417, a7418)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7419, _, _, _) => {
            h7419
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7420, _, _) => {
            h7420
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7421, _) => {
            h7421
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7422) => {
            h7422
        }
    })
}

pub fn f315(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7423 = v0;
    let a7424 = 4u64;
    if if if nat_le(a7423, a7424) {
        true
    } else {
        let a7425 = 9u64;
        let a7426 = v0;
        if nat_le(a7425, a7426) {
            let a7427 = v0;
            let a7428 = 12u64;
            nat_le(a7427, a7428)
        } else {
            false
        }
    } {
        true
    } else {
        let a7429 = v0;
        let a7430 = 16u64;
        if if nat_eq(a7429, a7430) {
            true
        } else {
            let a7431 = v0;
            let a7432 = 19u64;
            nat_eq(a7431, a7432)
        } {
            true
        } else {
            let a7433 = v0;
            let a7434 = 21u64;
            if nat_eq(a7433, a7434) {
                true
            } else {
                let a7435 = v0;
                let a7436 = 22u64;
                nat_eq(a7435, a7436)
            }
        }
    } {
        Adt17::C1({
            let a7437 = v0;
            let a7438 = 4u64;
            if if nat_lt(a7437, a7438) {
                true
            } else {
                let a7439 = 21u64;
                let a7440 = v0;
                nat_le(a7439, a7440)
            } {
                0u64
            } else {
                let a7441 = v0;
                let a7442 = 16u64;
                if if nat_eq(a7441, a7442) {
                    true
                } else {
                    let a7443 = v0;
                    let a7444 = 19u64;
                    nat_eq(a7443, a7444)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7445 = v0;
            let a7446 = 4u64;
            if nat_lt(a7445, a7446) {
                v0
            } else {
                let a7447 = v0;
                let a7448 = 4u64;
                if if nat_eq(a7447, a7448) {
                    true
                } else {
                    let a7449 = v0;
                    let a7450 = 16u64;
                    nat_eq(a7449, a7450)
                } {
                    0u64
                } else {
                    let a7451 = v0;
                    let a7452 = 19u64;
                    if nat_eq(a7451, a7452) {
                        3u64
                    } else {
                        let a7453 = 21u64;
                        let a7454 = v0;
                        if nat_le(a7453, a7454) {
                            let a7455 = v0;
                            let a7456 = 17u64;
                            nat_sub(a7455, a7456)
                        } else {
                            let a7457 = v0;
                            let a7458 = 8u64;
                            nat_sub(a7457, a7458)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f316(v0: u64) -> R<u64> {
    let a7461 = 2u64;
    let a7462 = {
        let a7459 = 6u64;
        let a7460 = v0;
        nat_mul(a7459, a7460)?
    };
    nat_add(a7461, a7462)
}

pub fn f317(v0: u64) -> R<u64> {
    let a7463 = f316(v0)?;
    let a7464 = 11u64;
    nat_add(a7463, a7464)
}

pub fn f318(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7465 = v0;
        let a7466 = 6u64;
        let a7467 = 0u64;
        nat_rem(a7465, a7466, a7467)
    };
    let v5: u64 = {
        let a7474 = {
            let a7469 = v1;
            let a7470 = match v2.clone() {
                Adt20::C0(h7468, _, _, _, _, _, _, _) => {
                    h7468
                }
            };
            nat_mul(a7469, a7470)?
        };
        let a7475 = {
            let a7471 = v0;
            let a7472 = 6u64;
            let a7473 = 0u64;
            nat_quot(a7471, a7472, a7473)
        };
        nat_add(a7474, a7475)?
    };
    let v6: u64 = f314(v5, v3)?;
    let v7: u64 = f314(v5, v2)?;
    let a7476 = v4;
    let a7477 = 0u64;
    if nat_eq(a7476, a7477) {
        Ok(f315(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7478 = v4;
        let a7479 = 1u64;
        if nat_eq(a7478, a7479) {
            Ok(f315(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7480 = v4;
            let a7481 = 2u64;
            if nat_eq(a7480, a7481) {
                Ok(f315(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7482 = v4;
                let a7483 = 3u64;
                if nat_eq(a7482, a7483) {
                    Ok(f315(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7484 = v4;
                    let a7485 = 4u64;
                    if nat_eq(a7484, a7485) {
                        Ok(f315(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f315(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f319(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7486 = v0;
    let a7487 = 0u64;
    if nat_eq(a7486, a7487) {
        f315(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7488 = v0;
        let a7489 = 1u64;
        if nat_eq(a7488, a7489) {
            f315(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7490 = v0;
            let a7491 = 2u64;
            if nat_eq(a7490, a7491) {
                f315(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7492 = v0;
                let a7493 = 3u64;
                if nat_eq(a7492, a7493) {
                    f315(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7494 = v0;
                    let a7495 = 4u64;
                    if nat_eq(a7494, a7495) {
                        f315(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7496 = v0;
                        let a7497 = 5u64;
                        if nat_eq(a7496, a7497) {
                            f315(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7498 = v0;
                            let a7499 = 6u64;
                            if nat_eq(a7498, a7499) {
                                f315(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7500 = v0;
                                let a7501 = 7u64;
                                if nat_eq(a7500, a7501) {
                                    f315(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7502 = v0;
                                    let a7503 = 8u64;
                                    if nat_eq(a7502, a7503) {
                                        f315(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7504 = v0;
                                        let a7505 = 9u64;
                                        if nat_eq(a7504, a7505) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7506 = v0;
                                            let a7507 = 10u64;
                                            if nat_eq(a7506, a7507) {
                                                f315(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f320(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7508 = v0;
        let a7509 = 3u64;
        let a7510 = 0u64;
        nat_rem(a7508, a7509, a7510)
    };
    let v6: u64 = {
        let a7517 = {
            let a7512 = v1;
            let a7513 = match v2.clone() {
                Adt20::C0(h7511, _, _, _, _, _, _, _) => {
                    h7511
                }
            };
            nat_mul(a7512, a7513)?
        };
        let a7518 = {
            let a7514 = v0;
            let a7515 = 3u64;
            let a7516 = 0u64;
            nat_quot(a7514, a7515, a7516)
        };
        nat_add(a7517, a7518)?
    };
    let v7: u64 = f314(v6, v3)?;
    let v8: u64 = f314(v6, v2)?;
    let v9: u64 = f314(v6, v4)?;
    let a7519 = v5;
    let a7520 = 0u64;
    if nat_eq(a7519, a7520) {
        Ok(f315(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7521 = v5;
        let a7522 = 1u64;
        if nat_eq(a7521, a7522) {
            Ok(f315(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f315(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f321(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7523, _, _, _, _, _, _, _) => {
            h7523
        }
    };
    let v5: u64 = {
        let a7526 = 1u64;
        let a7527 = {
            let a7524 = v4;
            let a7525 = f306(v3);
            nat_mul(a7524, a7525)?
        };
        nat_add(a7526, a7527)?
    };
    let a7528 = v0;
    let a7529 = 0u64;
    if nat_eq(a7528, a7529) {
        Ok(f313(0u64))
    } else {
        let a7530 = v0;
        let a7531 = v5;
        if nat_lt(a7530, a7531) {
            let v6: u64 = {
                let a7532 = v0;
                let a7533 = 1u64;
                nat_sub(a7532, a7533)
            };
            let v7: u64 = f314({
                let a7539 = {
                    let a7534 = v1;
                    let a7535 = v4;
                    nat_mul(a7534, a7535)?
                };
                let a7540 = {
                    let a7536 = v6;
                    let a7537 = f306(v3);
                    let a7538 = 0u64;
                    nat_quot(a7536, a7537, a7538)
                };
                nat_add(a7539, a7540)?
            }, v2)?;
            if v3 {
                let a7544 = {
                    let a7541 = v6;
                    let a7542 = 2u64;
                    let a7543 = 0u64;
                    nat_rem(a7541, a7542, a7543)
                };
                let a7545 = 0u64;
                if nat_eq(a7544, a7545) {
                    Ok(f315(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f315(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f315(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7546 = v0;
                let a7547 = v5;
                nat_sub(a7546, a7547)
            };
            let a7548 = v8;
            let a7549 = 0u64;
            if nat_eq(a7548, a7549) {
                Ok(f315(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7550 = v8;
                    let a7551 = 1u64;
                    nat_eq(a7550, a7551)
                } else {
                    false
                } {
                    Ok(f315(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7552 = v8;
                    let a7553 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7552, a7553) {
                        Ok(f315(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7554 = v8;
                        let a7555 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7554, a7555) {
                            Ok(f313(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f322(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7557 = 13u64;
        let a7558 = f307(match v2.clone() {
            Adt20::C0(h7556, _, _, _, _, _, _, _) => {
                h7556
            }
        });
        nat_mul(a7557, a7558)?
    } else {
        0u64
    };
    let a7559 = v0;
    let a7560 = v5;
    if nat_lt(a7559, a7560) {
        f329({
            let a7561 = v0;
            let a7562 = 13u64;
            let a7563 = 0u64;
            nat_rem(a7561, a7562, a7563)
        }, v1, {
            let a7564 = v0;
            let a7565 = 13u64;
            let a7566 = 0u64;
            nat_quot(a7564, a7565, a7566)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7567 = v0;
            let a7568 = v5;
            nat_sub(a7567, a7568)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f330({
            let a7569 = v6;
            let a7570 = v7;
            let a7571 = 0u64;
            nat_rem(a7569, a7570, a7571)
        }, {
            let a7581 = {
                let a7576 = {
                    let a7573 = v1;
                    let a7574 = match v2.clone() {
                        Adt20::C0(h7572, _, _, _, _, _, _, _) => {
                            h7572
                        }
                    };
                    nat_mul(a7573, a7574)?
                };
                let a7577 = if v4 {
                    f331(match v2.clone() {
                        Adt20::C0(h7575, _, _, _, _, _, _, _) => {
                            h7575
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7576, a7577)?
            };
            let a7582 = {
                let a7578 = v6;
                let a7579 = v7;
                let a7580 = 0u64;
                nat_quot(a7578, a7579, a7580)
            };
            nat_add(a7581, a7582)?
        }, v2, v3, v4)
    }
}

pub fn f323(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7583 = v0;
    let a7584 = 0u64;
    if nat_eq(a7583, a7584) {
        f315(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7585 = v0;
        let a7586 = 1u64;
        if nat_eq(a7585, a7586) {
            f315(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7587 = v0;
                let a7588 = 2u64;
                nat_eq(a7587, a7588)
            } else {
                false
            } {
                f315(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7589 = v0;
                let a7590 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7589, a7590) {
                    f315(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7591 = v0;
                    let a7592 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7591, a7592) {
                        f315(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7593 = v0;
                        let a7594 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7593, a7594) {
                            f315(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f324(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7598 = 1u64;
        let a7599 = {
            let a7596 = match v4.clone() {
                Adt20::C0(h7595, _, _, _, _, _, _, _) => {
                    h7595
                }
            };
            let a7597 = v6;
            nat_mul(a7596, a7597)?
        };
        nat_add(a7598, a7599)?
    };
    let a7600 = v1;
    let a7601 = 0u64;
    if nat_eq(a7600, a7601) {
        Ok(f313(1u64))
    } else {
        let a7602 = v1;
        let a7603 = v7;
        if nat_lt(a7602, a7603) {
            let v8: u64 = {
                let a7604 = v1;
                let a7605 = 1u64;
                nat_sub(a7604, a7605)
            };
            let v9: u64 = {
                let a7612 = {
                    let a7607 = f327(v2, v3, v4.clone())?;
                    let a7608 = match v4.clone() {
                        Adt20::C0(h7606, _, _, _, _, _, _, _) => {
                            h7606
                        }
                    };
                    nat_mul(a7607, a7608)?
                };
                let a7613 = {
                    let a7609 = v8;
                    let a7610 = v6;
                    let a7611 = 0u64;
                    nat_quot(a7609, a7610, a7611)
                };
                nat_add(a7612, a7613)?
            };
            let v10: u64 = f314(v9, v4)?;
            let v11: u64 = f314(v9, v5)?;
            let v12: u64 = {
                let a7614 = v8;
                let a7615 = v6;
                let a7616 = 0u64;
                nat_rem(a7614, a7615, a7616)
            };
            if v0 {
                let a7617 = v12;
                let a7618 = 0u64;
                if nat_eq(a7617, a7618) {
                    Ok(f315(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7619 = v12;
                    let a7620 = 1u64;
                    if nat_eq(a7619, a7620) {
                        Ok(f315(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7621 = v12;
                        let a7622 = 2u64;
                        if nat_eq(a7621, a7622) {
                            Ok(f315(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f315(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7623 = v12;
                let a7624 = 0u64;
                if nat_eq(a7623, a7624) {
                    Ok(f315(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f315(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7625 = v1;
            let a7626 = v7;
            if nat_eq(a7625, a7626) {
                Ok(f315(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f325(v0: u64, v1: u64) -> Adt17 {
    let a7627 = v0;
    let a7628 = 0u64;
    if nat_eq(a7627, a7628) {
        f315(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7629 = v0;
        let a7630 = 1u64;
        if nat_eq(a7629, a7630) {
            f315(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7631 = v0;
            let a7632 = 2u64;
            if nat_eq(a7631, a7632) {
                f315(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7633 = v0;
                let a7634 = 3u64;
                if nat_eq(a7633, a7634) {
                    f313(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f326(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7635 = v0;
    let a7636 = 0u64;
    if nat_eq(a7635, a7636) {
        f315(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7637 = v0;
        let a7638 = 1u64;
        if nat_eq(a7637, a7638) {
            f315(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7639 = v0;
            let a7640 = 2u64;
            if nat_eq(a7639, a7640) {
                f315(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7641 = v0;
                let a7642 = 3u64;
                if nat_eq(a7641, a7642) {
                    f315(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7643 = v0;
                    let a7644 = 4u64;
                    if nat_eq(a7643, a7644) {
                        f315(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7645 = v0;
                        let a7646 = 5u64;
                        if nat_eq(a7645, a7646) {
                            f315(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f327(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7657 = {
        let a7655 = {
            let a7652 = {
                let a7649 = match v1.clone() {
                    Adt21::C0(_, _, h7647, _) => {
                        h7647
                    }
                };
                let a7650 = match v2.clone() {
                    Adt20::C0(_, _, h7648, _, _, _, _, _) => {
                        h7648
                    }
                };
                nat_mul(a7649, a7650)?
            };
            let a7653 = match v1 {
                Adt21::C0(h7651, _, _, _) => {
                    h7651
                }
            };
            nat_add(a7652, a7653)?
        };
        let a7656 = match v2 {
            Adt20::C0(_, h7654, _, _, _, _, _, _) => {
                h7654
            }
        };
        nat_mul(a7655, a7656)?
    };
    let a7658 = v0;
    nat_add(a7657, a7658)
}

pub fn f328(v0: u64) -> u64 {
    if f226(v0) {
        let m7659 = f218(v0);
        match m7659 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7661 = 1u64;
                let a7662 = match v1.clone() {
                    Adt19::C0(_, h7660, _) => {
                        h7660
                    }
                };
                if if nat_lt(a7661, a7662) {
                    true
                } else {
                    let a7663 = v0;
                    let a7664 = 43u64;
                    nat_eq(a7663, a7664)
                } {
                    match v1 {
                        Adt19::C0(_, h7665, _) => {
                            h7665
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

pub fn f329(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7673 = {
            let a7671 = {
                let a7667 = v1;
                let a7668 = match v3.clone() {
                    Adt20::C0(h7666, _, _, _, _, _, _, _) => {
                        h7666
                    }
                };
                nat_mul(a7667, a7668)?
            };
            let a7672 = {
                let a7669 = v2;
                let a7670 = 4u64;
                nat_mul(a7669, a7670)?
            };
            nat_add(a7671, a7672)?
        };
        let a7674 = f332(v0);
        nat_add(a7673, a7674)?
    };
    let v6: u64 = f314(v5, v3)?;
    let v7: u64 = f314(v5, v4)?;
    let a7675 = v0;
    let a7676 = 0u64;
    if if if if nat_eq(a7675, a7676) {
        true
    } else {
        let a7677 = v0;
        let a7678 = 2u64;
        nat_eq(a7677, a7678)
    } {
        true
    } else {
        let a7679 = v0;
        let a7680 = 5u64;
        nat_eq(a7679, a7680)
    } {
        true
    } else {
        let a7681 = v0;
        let a7682 = 7u64;
        nat_eq(a7681, a7682)
    } {
        Ok(f315(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7683 = v0;
        let a7684 = 1u64;
        if if if if nat_eq(a7683, a7684) {
            true
        } else {
            let a7685 = v0;
            let a7686 = 3u64;
            nat_eq(a7685, a7686)
        } {
            true
        } else {
            let a7687 = v0;
            let a7688 = 6u64;
            nat_eq(a7687, a7688)
        } {
            true
        } else {
            let a7689 = v0;
            let a7690 = 8u64;
            nat_eq(a7689, a7690)
        } {
            Ok(f315(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7691 = v0;
                let a7692 = 1u64;
                if nat_eq(a7691, a7692) {
                    4u64
                } else {
                    let a7693 = v0;
                    let a7694 = 6u64;
                    if nat_eq(a7693, a7694) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7695 = v0;
            let a7696 = 4u64;
            if nat_eq(a7695, a7696) {
                Ok(f315(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7697 = v0;
                let a7698 = 9u64;
                if nat_eq(a7697, a7698) {
                    Ok(f315(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7699 = v0;
                    let a7700 = 10u64;
                    if nat_eq(a7699, a7700) {
                        Ok(f315(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7701 = v0;
                        let a7702 = 11u64;
                        if nat_eq(a7701, a7702) {
                            Ok(f315(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7703 = v0;
                            let a7704 = 12u64;
                            if nat_eq(a7703, a7704) {
                                Ok(f315(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f330(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f314(v1, v2)?;
    let v6: u64 = f314(v1, v3)?;
    let a7705 = v0;
    let a7706 = 0u64;
    if nat_eq(a7705, a7706) {
        Ok(f315(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7707 = v0;
        let a7708 = 1u64;
        if nat_eq(a7707, a7708) {
            Ok(f315(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7709 = v0;
                let a7710 = 2u64;
                if nat_eq(a7709, a7710) {
                    Ok(f315(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7711 = v0;
                    let a7712 = 3u64;
                    if nat_eq(a7711, a7712) {
                        Ok(f315(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7713 = v0;
                let a7714 = 2u64;
                if nat_eq(a7713, a7714) {
                    Ok(f315(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f331(v0: u64) -> R<u64> {
    let a7715 = 4u64;
    let a7716 = f307(v0);
    nat_mul(a7715, a7716)
}

pub fn f332(v0: u64) -> u64 {
    let a7717 = v0;
    let a7718 = 5u64;
    if nat_lt(a7717, a7718) {
        let a7719 = v0;
        let a7720 = 2u64;
        if nat_lt(a7719, a7720) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7721 = v0;
        let a7722 = 7u64;
        if nat_lt(a7721, a7722) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
