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
            if if if if nat_eq(a2, a3) {
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
                let a8 = v1;
                let a9 = 94u64;
                if if nat_eq(a8, a9) {
                    true
                } else {
                    let a10 = v1;
                    let a11 = 95u64;
                    nat_eq(a10, a11)
                } {
                    Ok(Adt9::C0(f2({
                        let a12 = v1;
                        let a13 = 95u64;
                        nat_eq(a12, a13)
                    }, v3, v4, v5, {
                        let m14 = v6;
                        match m14 {
                            None => {
                                let m15 = v7;
                                match m15 {
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
                    let a16 = v1;
                    let a17 = 96u64;
                    if if nat_eq(a16, a17) {
                        true
                    } else {
                        let a18 = v1;
                        let a19 = 97u64;
                        nat_eq(a18, a19)
                    } {
                        Ok(Adt9::C0(f3({
                            let a20 = v1;
                            let a21 = 96u64;
                            nat_eq(a20, a21)
                        }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                    } else {
                        let v15: bool = {
                            let m22 = v7;
                            match m22 {
                                None => {
                                    v8
                                }
                                Some(_) => {
                                    true
                                }
                            }
                        };
                        let a23 = v1;
                        let a24 = 85u64;
                        if nat_eq(a23, a24) {
                            Ok(Adt9::C0(f4(v2, v3, v4, v5, {
                                let m25 = v6;
                                match m25 {
                                    None => {
                                        v15
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            }, v11)?))
                        } else {
                            let a26 = v1;
                            let a27 = 87u64;
                            if nat_eq(a26, a27) {
                                Ok(Adt9::C0(f5(v3, v4, v5, v6, v15, v11)?))
                            } else {
                                let a28 = v1;
                                let a29 = 64u64;
                                if nat_eq(a28, a29) {
                                    Ok(Adt9::C0(f6(v2, v11)?))
                                } else {
                                    let m30 = v3;
                                    match m30 {
                                        None => {
                                            Ok(Adt9::C0(Adt10::C1))
                                        }
                                        Some(v17) => {
                                            let a31 = v1;
                                            let a32 = 48u64;
                                            if if nat_eq(a31, a32) {
                                                true
                                            } else {
                                                let a33 = v1;
                                                let a34 = 56u64;
                                                nat_eq(a33, a34)
                                            } {
                                                Ok(Adt9::C0(f7({
                                                    let a35 = v1;
                                                    let a36 = 56u64;
                                                    nat_eq(a35, a36)
                                                }, v2, v17, v4, v11)?))
                                            } else {
                                                let a37 = v1;
                                                let a38 = 53u64;
                                                if if nat_eq(a37, a38) {
                                                    true
                                                } else {
                                                    let a39 = v1;
                                                    let a40 = 54u64;
                                                    nat_eq(a39, a40)
                                                } {
                                                    Ok(Adt9::C0(f8({
                                                        let a41 = v1;
                                                        let a42 = 54u64;
                                                        nat_eq(a41, a42)
                                                    }, v2, v17, v4, v11)?))
                                                } else {
                                                    let a43 = v1;
                                                    let a44 = 78u64;
                                                    if nat_eq(a43, a44) {
                                                        Ok(Adt9::C0(f9(v2, v17, v4, v5, v11)?))
                                                    } else {
                                                        let a45 = v1;
                                                        let a46 = 23u64;
                                                        if if if nat_eq(a45, a46) {
                                                            true
                                                        } else {
                                                            let a47 = v1;
                                                            let a48 = 26u64;
                                                            nat_eq(a47, a48)
                                                        } {
                                                            true
                                                        } else {
                                                            let a49 = v1;
                                                            let a50 = 25u64;
                                                            nat_eq(a49, a50)
                                                        } {
                                                            Ok(Adt9::C0(f10(v1, v2, v17, v4, v11)?))
                                                        } else {
                                                            let a51 = v1;
                                                            let a52 = 24u64;
                                                            if nat_eq(a51, a52) {
                                                                Ok(Adt9::C0(f11(v2, v17, v11)?))
                                                            } else {
                                                                let a53 = v1;
                                                                let a54 = 27u64;
                                                                if nat_eq(a53, a54) {
                                                                    Ok(Adt9::C0(f12(v2, v17, v11)?))
                                                                } else {
                                                                    let a55 = v1;
                                                                    let a56 = 3u64;
                                                                    if nat_eq(a55, a56) {
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
            let a58 = 0u64;
            let a59 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h57, _, _, _, _, _, _, _) => {
                    h57
                }
            };
            if nat_lt(a58, a59) {
                f15(v18, v19, v20, v21)
            } else {
                f16(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a60 = 3240099840u64;
                let a61 = v22;
                if if nat_le(a60, a61) {
                    let a62 = v22;
                    let a63 = 4286578688u64;
                    nat_le(a62, a63)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a64 = 1092616192u64;
                    let a65 = v22;
                    if if nat_le(a64, a65) {
                        let a66 = v22;
                        let a67 = 2139095040u64;
                        nat_le(a66, a67)
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
    let m68 = v0;
    match m68 {
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
        let m69 = v1.clone();
        match m69 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m70 = v2.clone();
        match m70 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m71 = v1.clone();
        match m71 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m72 = v3.clone();
        match m72 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m73 = v2.clone();
        match m73 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a79 = {
            let a76 = match v10.clone() {
                Adt2::C0(_, _, h74, _, _, _, _, _, _, _, _) => {
                    h74
                }
            };
            let a77 = match v10.clone() {
                Adt2::C0(_, _, _, h75, _, _, _, _, _, _, _) => {
                    h75
                }
            };
            nat_mul(a76, a77)?
        };
        let a80 = match v10.clone() {
            Adt2::C0(_, _, _, _, h78, _, _, _, _, _, _) => {
                h78
            }
        };
        nat_mul(a79, a80)?
    };
    let m81 = v1;
    if if if if if if if if if if match m81 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m82 = v2;
        match m82 {
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
            let m83 = v3;
            match m83 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m84 = v3;
            !match m84 {
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
        let a85 = 0u64;
        let a86 = v14;
        nat_lt(a85, a86)
    } else {
        false
    } {
        let a88 = 0u64;
        let a89 = match v10.clone() {
            Adt2::C0(_, h87, _, _, _, _, _, _, _, _, _) => {
                h87
            }
        };
        nat_lt(a88, a89)
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
            let a91 = match v7.clone() {
                Adt2::C0(h90, _, _, _, _, _, _, _, _, _, _) => {
                    h90
                }
            };
            let a92 = 0u64;
            if if if if nat_eq(a91, a92) {
                let a93 = f23(v7.clone(), 1u64)?;
                let a94 = 1u64;
                nat_eq(a93, a94)
            } else {
                false
            } {
                let a96 = 4u64;
                let a97 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h95, _) => {
                        h95
                    }
                };
                nat_le(a96, a97)
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
            let a99 = match v5.clone() {
                Adt2::C0(h98, _, _, _, _, _, _, _, _, _, _) => {
                    h98
                }
            };
            let a100 = 0u64;
            if if nat_eq(a99, a100) {
                let a101 = f23(v5.clone(), 1u64)?;
                let a102 = 1u64;
                nat_eq(a101, a102)
            } else {
                false
            } {
                let a104 = 4u64;
                let a105 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h103, _) => {
                        h103
                    }
                };
                nat_le(a104, a105)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a111 = {
                let a109 = 2u64;
                let a110 = {
                    let a107 = v14;
                    let a108 = f24(v0, match v10.clone() {
                        Adt2::C0(_, h106, _, _, _, _, _, _, _, _, _) => {
                            h106
                        }
                    })?;
                    nat_mul(a107, a108)?
                };
                nat_add(a109, a110)?
            };
            let a112 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a111, a112)?
        }, 4u64, f25(v10, 1u64), f25(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h113, _, _, _, _) => {
                h113
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m114 = v3.clone();
        match m114 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m115 = v4.clone();
        match m115 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m116 = v5.clone();
        match m116 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m117 = v6.clone();
        match m117 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m118 = v7.clone();
        match m118 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m119 = v5.clone();
        match m119 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m120 = v3;
    if if if if if if if if if if if match m120 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m121 = v4;
        match m121 {
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
        let m122 = v5;
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
    } {
        if v0 {
            let m123 = v6;
            if match m123 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m124 = v7;
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
            }
        } else {
            let m125 = v6;
            if !match m125 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m126 = v7;
                !match m126 {
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
            let a133 = {
                let a130 = {
                    let a127 = v1;
                    let a128 = 4u64;
                    let a129 = 0u64;
                    nat_quot(a127, a128, a129)
                };
                let a131 = 2u64;
                let a132 = 0u64;
                nat_rem(a130, a131, a132)
            };
            let a134 = 1u64;
            nat_eq(a133, a134)
        } else {
            false
        } {
            let a136 = match v11.clone() {
                Adt2::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            };
            let a137 = 0u64;
            nat_eq(a136, a137)
        } else {
            false
        } {
            let a139 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h138, _, _, _, _, _) => {
                    h138
                }
            };
            let a140 = 4u64;
            nat_eq(a139, a140)
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
            let a142 = match v11.clone() {
                Adt2::C0(_, h141, _, _, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            if nat_le(a142, a143) {
                true
            } else {
                let a146 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h144, _, _, _, _, _) => {
                        h144
                    }
                };
                let a147 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h145, _, _, _, _, _) => {
                        h145
                    }
                };
                nat_eq(a146, a147)
            }
        } else {
            false
        } {
            let a149 = match v11.clone() {
                Adt2::C0(_, _, h148, _, _, _, _, _, _, _, _) => {
                    h148
                }
            };
            let a150 = 1u64;
            if nat_le(a149, a150) {
                true
            } else {
                let a153 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h151, _, _, _, _) => {
                        h151
                    }
                };
                let a154 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h152, _, _, _, _) => {
                        h152
                    }
                };
                nat_eq(a153, a154)
            }
        } else {
            false
        } {
            let a156 = match v11.clone() {
                Adt2::C0(_, _, _, h155, _, _, _, _, _, _, _) => {
                    h155
                }
            };
            let a157 = 1u64;
            if nat_le(a156, a157) {
                true
            } else {
                let a160 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h158, _, _, _) => {
                        h158
                    }
                };
                let a161 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h159, _, _, _) => {
                        h159
                    }
                };
                nat_eq(a160, a161)
            }
        } else {
            false
        } {
            let a163 = match v11.clone() {
                Adt2::C0(_, _, _, _, h162, _, _, _, _, _, _) => {
                    h162
                }
            };
            let a164 = 1u64;
            if nat_le(a163, a164) {
                true
            } else {
                let a167 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h165, _, _) => {
                        h165
                    }
                };
                let a168 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h166, _, _) => {
                        h166
                    }
                };
                nat_eq(a167, a168)
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
        let a170 = match v20.clone() {
            Adt2::C0(h169, _, _, _, _, _, _, _, _, _, _) => {
                h169
            }
        };
        let a171 = 0u64;
        nat_eq(a170, a171)
    } else {
        false
    } {
        let a172 = f23(v20.clone(), 1u64)?;
        let a173 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a172, a173)
    } else {
        false
    } {
        let a175 = if v0 {
            28u64
        } else {
            8u64
        };
        let a176 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h174, _) => {
                h174
            }
        };
        nat_le(a175, a176)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a179 = f28(v0);
            let a180 = {
                let a177 = f23(v11.clone(), 1u64)?;
                let a178 = f29(v0);
                nat_mul(a177, a178)?
            };
            nat_add(a179, a180)?
        }, 4u64, f25(v11, 1u64), f25(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m181 = v1.clone();
        match m181 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m182 = v2.clone();
        match m182 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m183 = v3.clone();
        match m183 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h184, _, _, _, _, _, _, _, _, _, _) => {
            h184
        }
    };
    let m185 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m185 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m186 = v2;
        match m186 {
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
        let m187 = v3;
        match m187 {
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
        let a189 = match v7.clone() {
            Adt2::C0(h188, _, _, _, _, _, _, _, _, _, _) => {
                h188
            }
        };
        let a190 = 0u64;
        if if nat_eq(a189, a190) {
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
        let a192 = match v9.clone() {
            Adt2::C0(h191, _, _, _, _, _, _, _, _, _, _) => {
                h191
            }
        };
        let a193 = 0u64;
        if if nat_eq(a192, a193) {
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
        let a195 = match v11.clone() {
            Adt2::C0(h194, _, _, _, _, _, _, _, _, _, _) => {
                h194
            }
        };
        let a196 = 0u64;
        if if nat_eq(a195, a196) {
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
        let a198 = match v7.clone() {
            Adt2::C0(_, h197, _, _, _, _, _, _, _, _, _) => {
                h197
            }
        };
        let a199 = 24u64;
        nat_eq(a198, a199)
    } else {
        false
    } {
        let a201 = match v7.clone() {
            Adt2::C0(_, _, _, h200, _, _, _, _, _, _, _) => {
                h200
            }
        };
        let a202 = 1u64;
        nat_eq(a201, a202)
    } else {
        false
    } {
        let a204 = match v7.clone() {
            Adt2::C0(_, _, _, _, h203, _, _, _, _, _, _) => {
                h203
            }
        };
        let a205 = 1u64;
        nat_eq(a204, a205)
    } else {
        false
    } {
        let a207 = match v11.clone() {
            Adt2::C0(_, h206, _, _, _, _, _, _, _, _, _) => {
                h206
            }
        };
        let a208 = 24u64;
        nat_eq(a207, a208)
    } else {
        false
    } {
        let a210 = match v11.clone() {
            Adt2::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                h209
            }
        };
        let a211 = 1u64;
        nat_eq(a210, a211)
    } else {
        false
    } {
        let a213 = match v11.clone() {
            Adt2::C0(_, _, _, h212, _, _, _, _, _, _, _) => {
                h212
            }
        };
        let a214 = 1u64;
        nat_eq(a213, a214)
    } else {
        false
    } {
        let a216 = match v11.clone() {
            Adt2::C0(_, _, _, _, h215, _, _, _, _, _, _) => {
                h215
            }
        };
        let a217 = 1u64;
        nat_eq(a216, a217)
    } else {
        false
    } {
        let a219 = match v9.clone() {
            Adt2::C0(_, _, h218, _, _, _, _, _, _, _, _) => {
                h218
            }
        };
        let a220 = 1u64;
        nat_eq(a219, a220)
    } else {
        false
    } {
        let a222 = match v9.clone() {
            Adt2::C0(_, _, _, h221, _, _, _, _, _, _, _) => {
                h221
            }
        };
        let a223 = 1u64;
        nat_eq(a222, a223)
    } else {
        false
    } {
        let a225 = match v9.clone() {
            Adt2::C0(_, _, _, _, h224, _, _, _, _, _, _) => {
                h224
            }
        };
        let a226 = 1u64;
        nat_eq(a225, a226)
    } else {
        false
    } {
        let a228 = 3u64;
        let a229 = match v9.clone() {
            Adt2::C0(_, h227, _, _, _, _, _, _, _, _, _) => {
                h227
            }
        };
        nat_le(a228, a229)
    } else {
        false
    } {
        let a231 = match v5.clone() {
            Adt2::C0(_, h230, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a232 = 4u64;
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a234 = match v5.clone() {
            Adt2::C0(_, _, h233, _, _, _, _, _, _, _, _) => {
                h233
            }
        };
        let a235 = 4u64;
        nat_eq(a234, a235)
    } else {
        false
    } {
        let a238 = match v5.clone() {
            Adt2::C0(_, _, _, h236, _, _, _, _, _, _, _) => {
                h236
            }
        };
        let a239 = match v7.clone() {
            Adt2::C0(_, _, h237, _, _, _, _, _, _, _, _) => {
                h237
            }
        };
        nat_eq(a238, a239)
    } else {
        false
    } {
        let a241 = match v5.clone() {
            Adt2::C0(_, _, _, _, h240, _, _, _, _, _, _) => {
                h240
            }
        };
        let a242 = 1u64;
        nat_eq(a241, a242)
    } else {
        false
    } {
        f31(v5.clone())?
    } else {
        false
    } {
        let a243 = 0u64;
        let a244 = v12;
        nat_lt(a243, a244)
    } else {
        false
    } {
        let a245 = v12;
        let a246 = 2147483648u64;
        nat_lt(a245, a246)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a248 = match v7.clone() {
                Adt2::C0(_, _, h247, _, _, _, _, _, _, _, _) => {
                    h247
                }
            };
            let a249 = f32(v12)?;
            nat_mul(a248, a249)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C38(f25(v9, 1u64), f25(v11, 1u64), match v0 {
            Adt1::C0(h250, _, _, _, _, _, _, _, _, _, _, _) => {
                h250
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m251 = v0.clone();
        match m251 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m252 = v1.clone();
        match m252 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m253 = v2.clone();
        match m253 {
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
        let m254 = v0;
        match m254 {
            None => {
                false
            }
            Some(_) => {
                let m255 = v1;
                match m255 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m256 = v2;
                        match m256 {
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
        let a258 = match v7.clone() {
            Adt2::C0(h257, _, _, _, _, _, _, _, _, _, _) => {
                h257
            }
        };
        let a259 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a258, a259) {
            let a261 = match v7.clone() {
                Adt2::C0(_, _, _, _, h260, _, _, _, _, _, _) => {
                    h260
                }
            };
            let a262 = 1u64;
            nat_eq(a261, a262)
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
            let a264 = match v9.clone() {
                Adt2::C0(h263, _, _, _, _, _, _, _, _, _, _) => {
                    h263
                }
            };
            let a265 = 0u64;
            if if if nat_eq(a264, a265) {
                let a267 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h266, _, _, _, _, _, _) => {
                        h266
                    }
                };
                let a268 = 1u64;
                nat_eq(a267, a268)
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
            let a270 = match v11.clone() {
                Adt2::C0(h269, _, _, _, _, _, _, _, _, _, _) => {
                    h269
                }
            };
            let a271 = 0u64;
            if if if nat_eq(a270, a271) {
                let a273 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h272, _, _, _, _, _, _) => {
                        h272
                    }
                };
                let a274 = 1u64;
                nat_eq(a273, a274)
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
            let a276 = match v7.clone() {
                Adt2::C0(_, _, _, h275, _, _, _, _, _, _, _) => {
                    h275
                }
            };
            let a277 = 1u64;
            nat_eq(a276, a277)
        } else {
            false
        } {
            let a279 = match v11.clone() {
                Adt2::C0(_, _, _, h278, _, _, _, _, _, _, _) => {
                    h278
                }
            };
            let a280 = 1u64;
            nat_eq(a279, a280)
        } else {
            false
        } {
            let a283 = match v9.clone() {
                Adt2::C0(_, h281, _, _, _, _, _, _, _, _, _) => {
                    h281
                }
            };
            let a284 = match v7.clone() {
                Adt2::C0(_, h282, _, _, _, _, _, _, _, _, _) => {
                    h282
                }
            };
            nat_eq(a283, a284)
        } else {
            false
        } {
            let a287 = match v9.clone() {
                Adt2::C0(_, _, _, h285, _, _, _, _, _, _, _) => {
                    h285
                }
            };
            let a288 = match v7.clone() {
                Adt2::C0(_, _, h286, _, _, _, _, _, _, _, _) => {
                    h286
                }
            };
            nat_eq(a287, a288)
        } else {
            false
        } {
            let a291 = match v11.clone() {
                Adt2::C0(_, h289, _, _, _, _, _, _, _, _, _) => {
                    h289
                }
            };
            let a292 = match v9.clone() {
                Adt2::C0(_, _, h290, _, _, _, _, _, _, _, _) => {
                    h290
                }
            };
            nat_eq(a291, a292)
        } else {
            false
        } {
            let a295 = match v11.clone() {
                Adt2::C0(_, _, h293, _, _, _, _, _, _, _, _) => {
                    h293
                }
            };
            let a296 = match v7.clone() {
                Adt2::C0(_, _, h294, _, _, _, _, _, _, _, _) => {
                    h294
                }
            };
            nat_eq(a295, a296)
        } else {
            false
        } {
            let a298 = 0u64;
            let a299 = match v9.clone() {
                Adt2::C0(_, _, h297, _, _, _, _, _, _, _, _) => {
                    h297
                }
            };
            if nat_lt(a298, a299) {
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
            let m300 = v3.clone();
            match m300 {
                None => {
                    true
                }
                Some(v15) => {
                    let a302 = match v15.clone() {
                        Adt2::C0(h301, _, _, _, _, _, _, _, _, _, _) => {
                            h301
                        }
                    };
                    let a303 = 0u64;
                    if if if if if if nat_eq(a302, a303) {
                        let a305 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h304, _, _, _, _, _, _) => {
                                h304
                            }
                        };
                        let a306 = 1u64;
                        nat_eq(a305, a306)
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
                        let a309 = match v15.clone() {
                            Adt2::C0(_, h307, _, _, _, _, _, _, _, _, _) => {
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
                        let a313 = match v15.clone() {
                            Adt2::C0(_, _, h311, _, _, _, _, _, _, _, _) => {
                                h311
                            }
                        };
                        let a314 = match v9.clone() {
                            Adt2::C0(_, _, h312, _, _, _, _, _, _, _, _) => {
                                h312
                            }
                        };
                        nat_eq(a313, a314)
                    } else {
                        false
                    } {
                        let a317 = match v15 {
                            Adt2::C0(_, _, _, h315, _, _, _, _, _, _, _) => {
                                h315
                            }
                        };
                        let a318 = match v7.clone() {
                            Adt2::C0(_, _, h316, _, _, _, _, _, _, _, _) => {
                                h316
                            }
                        };
                        nat_eq(a317, a318)
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
            let m319 = v3;
            match m319 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f25(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a321 = f23(v5.clone(), 1u64)?;
            let a322 = f33(match v9.clone() {
                Adt2::C0(_, _, h320, _, _, _, _, _, _, _, _) => {
                    h320
                }
            }, v17.clone())?;
            nat_mul(a321, a322)?
        }, 4u64, f25(v7, 1u64), f25(v5, 1u64), Adt4::C37(f25(v9, 1u64), f25(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f23(v1.clone(), 1u64)?;
    let a324 = match v1.clone() {
        Adt2::C0(h323, _, _, _, _, _, _, _, _, _, _) => {
            h323
        }
    };
    let a325 = 0u64;
    if if if if if if if if nat_eq(a324, a325) {
        let a327 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h326, _, _, _, _, _) => {
                h326
            }
        };
        let a328 = 4u64;
        nat_eq(a327, a328)
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(h329, _, _, _, _, _, _, _, _, _, _, _) => {
                h329
            }
        })
    } else {
        false
    } {
        f34(match v0.clone() {
            Adt1::C0(_, h330, _, _, _, _, _, _, _, _, _, _) => {
                h330
            }
        })
    } else {
        false
    } {
        let a332 = 0u64;
        let a333 = match v0.clone() {
            Adt1::C0(_, _, h331, _, _, _, _, _, _, _, _, _) => {
                h331
            }
        };
        nat_lt(a332, a333)
    } else {
        false
    } {
        let a335 = match v0.clone() {
            Adt1::C0(_, _, h334, _, _, _, _, _, _, _, _, _) => {
                h334
            }
        };
        let a336 = 2139095040u64;
        nat_lt(a335, a336)
    } else {
        false
    } {
        let a337 = v2;
        let a338 = 9223372036854775808u64;
        nat_lt(a337, a338)
    } else {
        false
    } {
        let a340 = f35(v2)?;
        let a341 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h339, _) => {
                h339
            }
        };
        nat_le(a340, a341)
    } else {
        false
    } {
        Ok(Adt10::C4(f36(v2)?, 4u64, f25(v1.clone(), 1u64), f25(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h342, _, _, _, _, _, _, _, _, _, _, _) => {
                h342
            }
        }, match v0.clone() {
            Adt1::C0(_, h343, _, _, _, _, _, _, _, _, _, _) => {
                h343
            }
        }, match v0 {
            Adt1::C0(_, _, h344, _, _, _, _, _, _, _, _, _) => {
                h344
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m345 = v3;
    match m345 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f37(v0, v2.clone());
            let v7: Adt3 = f37(v0, v5.clone());
            let v8: Adt3 = f37(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h346, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h347, _, _, _, _, _, _) => {
                    h347
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h348, _, _, _, _, _) => {
                    h348
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h349, _, _, _, _) => {
                    h349
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h350, _, _, _, _, _, _, _) => {
                    h350
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h351, _, _, _, _, _, _) => {
                    h351
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h352, _, _, _, _, _) => {
                    h352
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h353, _, _, _, _, _, _, _, _, _, _, _) => {
                    h353
                }
            };
            let v17: u64 = {
                let a358 = v13;
                let a359 = {
                    let a356 = {
                        let a354 = v9;
                        let a355 = 1u64;
                        nat_sub(a354, a355)
                    };
                    let a357 = v16;
                    nat_mul(a356, a357)?
                };
                nat_add(a358, a359)?
            };
            let v18: u64 = {
                let a364 = v14;
                let a365 = {
                    let a362 = {
                        let a360 = v10;
                        let a361 = 1u64;
                        nat_sub(a360, a361)
                    };
                    let a363 = v16;
                    nat_mul(a362, a363)?
                };
                nat_add(a364, a365)?
            };
            let v19: u64 = {
                let a368 = {
                    let a366 = v9;
                    let a367 = v10;
                    nat_mul(a366, a367)?
                };
                let a369 = v12;
                nat_mul(a368, a369)?
            };
            let v20: u64 = {
                let a372 = {
                    let a370 = v13;
                    let a371 = v14;
                    nat_mul(a370, a371)?
                };
                let a373 = v15;
                nat_mul(a372, a373)?
            };
            let v21: u64 = {
                let a374 = v19;
                let a375 = 32u64;
                if nat_lt(a374, a375) {
                    v19
                } else {
                    32u64
                }
            };
            let a377 = match v2.clone() {
                Adt2::C0(h376, _, _, _, _, _, _, _, _, _, _) => {
                    h376
                }
            };
            let a378 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a377, a378) {
                true
            } else {
                let a380 = match v2.clone() {
                    Adt2::C0(h379, _, _, _, _, _, _, _, _, _, _) => {
                        h379
                    }
                };
                let a381 = 1u64;
                nat_eq(a380, a381)
            } {
                let a383 = match v5.clone() {
                    Adt2::C0(h382, _, _, _, _, _, _, _, _, _, _) => {
                        h382
                    }
                };
                let a384 = 0u64;
                nat_eq(a383, a384)
            } else {
                false
            } {
                let a386 = match v4.clone() {
                    Adt2::C0(h385, _, _, _, _, _, _, _, _, _, _) => {
                        h385
                    }
                };
                let a387 = 0u64;
                nat_eq(a386, a387)
            } else {
                false
            } {
                let a388 = v16;
                let a389 = 2147483648u64;
                nat_lt(a388, a389)
            } else {
                false
            } {
                let a390 = 0u64;
                let a391 = v9;
                nat_lt(a390, a391)
            } else {
                false
            } {
                let a392 = 0u64;
                let a393 = v10;
                nat_lt(a392, a393)
            } else {
                false
            } {
                let a394 = 0u64;
                let a395 = v11;
                nat_lt(a394, a395)
            } else {
                false
            } {
                let a396 = 0u64;
                let a397 = v12;
                nat_lt(a396, a397)
            } else {
                false
            } {
                let a398 = 0u64;
                let a399 = v13;
                nat_lt(a398, a399)
            } else {
                false
            } {
                let a400 = 0u64;
                let a401 = v14;
                nat_lt(a400, a401)
            } else {
                false
            } {
                let a402 = 0u64;
                let a403 = v15;
                nat_lt(a402, a403)
            } else {
                false
            } {
                let a404 = 0u64;
                let a405 = v16;
                nat_lt(a404, a405)
            } else {
                false
            } {
                let a407 = match v6.clone() {
                    Adt3::C0(_, _, _, h406, _, _, _, _) => {
                        h406
                    }
                };
                let a408 = v11;
                nat_eq(a407, a408)
            } else {
                false
            } {
                let a410 = match v8.clone() {
                    Adt3::C0(h409, _, _, _, _, _, _, _) => {
                        h409
                    }
                };
                let a411 = v17;
                nat_eq(a410, a411)
            } else {
                false
            } {
                let a413 = match v8.clone() {
                    Adt3::C0(_, h412, _, _, _, _, _, _) => {
                        h412
                    }
                };
                let a414 = v18;
                nat_eq(a413, a414)
            } else {
                false
            } {
                let a416 = match v8.clone() {
                    Adt3::C0(_, _, h415, _, _, _, _, _) => {
                        h415
                    }
                };
                let a417 = v15;
                nat_eq(a416, a417)
            } else {
                false
            } {
                let a419 = match v8.clone() {
                    Adt3::C0(_, _, _, h418, _, _, _, _) => {
                        h418
                    }
                };
                let a420 = v12;
                nat_eq(a419, a420)
            } else {
                false
            } {
                f38(v4.clone())?
            } else {
                false
            } {
                let a425 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h421, _, _, _, _, _) => {
                        h421
                    }
                };
                let a426 = {
                    let a423 = match v2.clone() {
                        Adt2::C0(h422, _, _, _, _, _, _, _, _, _, _) => {
                            h422
                        }
                    };
                    let a424 = 1u64;
                    if nat_eq(a423, a424) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a425, a426)
            } else {
                false
            } {
                let a428 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h427, _, _, _, _, _) => {
                        h427
                    }
                };
                let a429 = 4u64;
                nat_eq(a428, a429)
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
                    let a434 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h430, _, _, _, _) => {
                            h430
                        }
                    };
                    let a435 = {
                        let a432 = v13;
                        let a433 = match v2 {
                            Adt2::C0(_, _, _, _, _, h431, _, _, _, _, _) => {
                                h431
                            }
                        };
                        nat_mul(a432, a433)?
                    };
                    nat_eq(a434, a435)
                } else {
                    let a437 = match v1.clone() {
                        Adt1::C0(_, h436, _, _, _, _, _, _, _, _, _, _) => {
                            h436
                        }
                    };
                    let a438 = 0u64;
                    if if if if if if nat_eq(a437, a438) {
                        let a440 = match v1 {
                            Adt1::C0(_, _, h439, _, _, _, _, _, _, _, _, _) => {
                                h439
                            }
                        };
                        let a441 = 1u64;
                        nat_eq(a440, a441)
                    } else {
                        false
                    } {
                        let a443 = match v2 {
                            Adt2::C0(_, _, _, _, h442, _, _, _, _, _, _) => {
                                h442
                            }
                        };
                        let a444 = 1u64;
                        nat_eq(a443, a444)
                    } else {
                        false
                    } {
                        let a446 = match v5.clone() {
                            Adt2::C0(_, _, _, h445, _, _, _, _, _, _, _) => {
                                h445
                            }
                        };
                        let a447 = 1u64;
                        nat_eq(a446, a447)
                    } else {
                        false
                    } {
                        let a449 = match v5 {
                            Adt2::C0(_, _, _, _, h448, _, _, _, _, _, _) => {
                                h448
                            }
                        };
                        let a450 = 1u64;
                        nat_eq(a449, a450)
                    } else {
                        false
                    } {
                        let a452 = match v4.clone() {
                            Adt2::C0(_, _, _, h451, _, _, _, _, _, _, _) => {
                                h451
                            }
                        };
                        let a453 = 1u64;
                        nat_eq(a452, a453)
                    } else {
                        false
                    } {
                        let a455 = match v4 {
                            Adt2::C0(_, _, _, _, h454, _, _, _, _, _, _) => {
                                h454
                            }
                        };
                        let a456 = 1u64;
                        nat_eq(a455, a456)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a457 = v11;
                    let a458 = v20;
                    nat_mul(a457, a458)?
                }, v21, {
                    let a459 = v21;
                    let a460 = v11;
                    nat_mul(a459, a460)?
                }, {
                    let a461 = v21;
                    let a462 = v20;
                    nat_mul(a461, a462)?
                }, {
                    let a467 = {
                        let a463 = v17;
                        let a464 = v18;
                        nat_mul(a463, a464)?
                    };
                    let a468 = {
                        let a465 = v15;
                        let a466 = v12;
                        nat_mul(a465, a466)?
                    };
                    nat_mul(a467, a468)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f8(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m469 = v3;
    match m469 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h470, _, _, _, _, _, _, _, _, _) => {
                    h470
                }
            }, match v1.clone() {
                Adt1::C0(h471, _, _, _, _, _, _, _, _, _, _, _) => {
                    h471
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h472, _, _, _, _, _, _, _, _) => {
                        h472
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h473, _, _, _, _, _, _, _, _, _) => {
                        h473
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h474, _, _, _, _, _) => {
                        h474
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h475, _, _, _, _, _, _, _) => {
                        h475
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h476, _, _, _, _, _, _, _, _) => {
                    h476
                }
            }, match v1.clone() {
                Adt1::C0(_, h477, _, _, _, _, _, _, _, _, _, _) => {
                    h477
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h478, _, _, _, _, _, _, _) => {
                        h478
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h479, _, _, _, _, _, _, _, _) => {
                        h479
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h480, _, _, _, _) => {
                        h480
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h481, _, _, _, _, _, _) => {
                        h481
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h482, _, _, _, _, _, _, _) => {
                        h482
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h483, _, _, _, _, _, _, _, _, _) => {
                        h483
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h484, _, _, _, _, _, _) => {
                        h484
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h485, _, _, _) => {
                        h485
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f39(v6.clone(), match v5.clone() {
                Adt2::C0(_, h486, _, _, _, _, _, _, _, _, _) => {
                    h486
                }
            })?;
            let v10: u64 = f39(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h487, _, _, _, _, _, _, _, _) => {
                    h487
                }
            })?;
            let v11: u64 = if v0 {
                f39(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h488, _, _, _, _, _, _, _) => {
                        h488
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h489, _, _) => {
                        h489
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h490, _, _, _, _, _, _, _) => {
                        h490
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h491, _) => {
                        h491
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h492, _, _, _, _, _, _) => {
                        h492
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h493) => {
                        h493
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h494, _, _, _, _, _, _) => {
                        h494
                    }
                }
            };
            let v15: u64 = {
                let a499 = {
                    let a495 = v9;
                    let a496 = v10;
                    nat_mul(a495, a496)?
                };
                let a500 = {
                    let a497 = v11;
                    let a498 = v13;
                    nat_mul(a497, a498)?
                };
                nat_mul(a499, a500)?
            };
            let v16: u64 = {
                let a508 = {
                    let a503 = match v6.clone() {
                        Adt6::C0(h501, _, _, _) => {
                            h501
                        }
                    };
                    let a504 = match v7.clone() {
                        Adt6::C0(h502, _, _, _) => {
                            h502
                        }
                    };
                    nat_mul(a503, a504)?
                };
                let a509 = {
                    let a506 = match v8.clone() {
                        Adt6::C0(h505, _, _, _) => {
                            h505
                        }
                    };
                    let a507 = v12;
                    nat_mul(a506, a507)?
                };
                nat_mul(a508, a509)?
            };
            let v17: u64 = {
                let a510 = v15;
                let a511 = 32u64;
                if nat_lt(a510, a511) {
                    v15
                } else {
                    32u64
                }
            };
            let a513 = match v2.clone() {
                Adt2::C0(h512, _, _, _, _, _, _, _, _, _, _) => {
                    h512
                }
            };
            let a514 = 0u64;
            if if if if if if if if if if if nat_eq(a513, a514) {
                true
            } else {
                let a516 = match v2.clone() {
                    Adt2::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
                        h515
                    }
                };
                let a517 = 1u64;
                nat_eq(a516, a517)
            } {
                let a519 = match v5.clone() {
                    Adt2::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
                        h518
                    }
                };
                let a520 = 0u64;
                nat_eq(a519, a520)
            } else {
                false
            } {
                let a522 = match v4.clone() {
                    Adt2::C0(h521, _, _, _, _, _, _, _, _, _, _) => {
                        h521
                    }
                };
                let a523 = 0u64;
                nat_eq(a522, a523)
            } else {
                false
            } {
                let a525 = match v4.clone() {
                    Adt2::C0(_, h524, _, _, _, _, _, _, _, _, _) => {
                        h524
                    }
                };
                let a526 = v9;
                nat_eq(a525, a526)
            } else {
                false
            } {
                let a528 = match v4.clone() {
                    Adt2::C0(_, _, h527, _, _, _, _, _, _, _, _) => {
                        h527
                    }
                };
                let a529 = v10;
                nat_eq(a528, a529)
            } else {
                false
            } {
                if v0 {
                    let a530 = 0u64;
                    let a531 = v12;
                    if if if if if if if nat_lt(a530, a531) {
                        let a532 = v12;
                        let a533 = 2147483648u64;
                        nat_lt(a532, a533)
                    } else {
                        false
                    } {
                        let a534 = v14;
                        let a535 = 2147483648u64;
                        nat_lt(a534, a535)
                    } else {
                        false
                    } {
                        let a536 = v13;
                        let a537 = 2147483648u64;
                        nat_lt(a536, a537)
                    } else {
                        false
                    } {
                        let a541 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h538, _, _, _, _, _, _) => {
                                h538
                            }
                        };
                        let a542 = {
                            let a539 = v12;
                            let a540 = v14;
                            nat_mul(a539, a540)?
                        };
                        nat_eq(a541, a542)
                    } else {
                        false
                    } {
                        let a546 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h543, _, _, _, _, _, _) => {
                                h543
                            }
                        };
                        let a547 = {
                            let a544 = v12;
                            let a545 = v13;
                            nat_mul(a544, a545)?
                        };
                        nat_eq(a546, a547)
                    } else {
                        false
                    } {
                        let a549 = match v4.clone() {
                            Adt2::C0(_, _, _, h548, _, _, _, _, _, _, _) => {
                                h548
                            }
                        };
                        let a550 = v11;
                        nat_eq(a549, a550)
                    } else {
                        false
                    } {
                        let a554 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h551, _, _, _, _, _, _) => {
                                h551
                            }
                        };
                        let a555 = {
                            let a552 = v14;
                            let a553 = v13;
                            nat_mul(a552, a553)?
                        };
                        nat_eq(a554, a555)
                    } else {
                        false
                    }
                } else {
                    let a557 = match v2.clone() {
                        Adt2::C0(_, _, _, h556, _, _, _, _, _, _, _) => {
                            h556
                        }
                    };
                    let a558 = v12;
                    if if if nat_eq(a557, a558) {
                        let a560 = match v4.clone() {
                            Adt2::C0(_, _, _, h559, _, _, _, _, _, _, _) => {
                                h559
                            }
                        };
                        let a561 = v14;
                        nat_eq(a560, a561)
                    } else {
                        false
                    } {
                        let a563 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h562, _, _, _, _, _, _) => {
                                h562
                            }
                        };
                        let a564 = v13;
                        nat_eq(a563, a564)
                    } else {
                        false
                    } {
                        let a565 = 0u64;
                        let a566 = v16;
                        nat_lt(a565, a566)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a567 = 0u64;
                let a568 = v14;
                nat_lt(a567, a568)
            } else {
                false
            } {
                let a569 = 0u64;
                let a570 = v15;
                nat_lt(a569, a570)
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
                    let a571 = v16;
                    let a572 = v14;
                    nat_mul(a571, a572)?
                }, v17, {
                    let a573 = v17;
                    let a574 = v16;
                    nat_mul(a573, a574)?
                }, {
                    let a575 = v17;
                    let a576 = v14;
                    nat_mul(a575, a576)?
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
    let m577 = v2;
    match m577 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m578 = v3;
            match m578 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a580 = match v1.clone() {
                        Adt2::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                            h579
                        }
                    };
                    let a581 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a580, a581) {
                        let a583 = match v5.clone() {
                            Adt2::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                                h582
                            }
                        };
                        let a584 = 0u64;
                        nat_eq(a583, a584)
                    } else {
                        false
                    } {
                        let a586 = match v6.clone() {
                            Adt2::C0(h585, _, _, _, _, _, _, _, _, _, _) => {
                                h585
                            }
                        };
                        let a587 = 0u64;
                        nat_eq(a586, a587)
                    } else {
                        false
                    } {
                        let a589 = match v4.clone() {
                            Adt2::C0(h588, _, _, _, _, _, _, _, _, _, _) => {
                                h588
                            }
                        };
                        let a590 = 0u64;
                        nat_eq(a589, a590)
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
                        let a596 = match v1.clone() {
                            Adt2::C0(_, h591, _, _, _, _, _, _, _, _, _) => {
                                h591
                            }
                        };
                        let a597 = {
                            let a594 = match v5.clone() {
                                Adt2::C0(_, h592, _, _, _, _, _, _, _, _, _) => {
                                    h592
                                }
                            };
                            let a595 = match v5.clone() {
                                Adt2::C0(_, h593, _, _, _, _, _, _, _, _, _) => {
                                    h593
                                }
                            };
                            nat_mul(a594, a595)?
                        };
                        nat_eq(a596, a597)
                    } else {
                        false
                    } {
                        let a603 = match v1.clone() {
                            Adt2::C0(_, _, h598, _, _, _, _, _, _, _, _) => {
                                h598
                            }
                        };
                        let a604 = {
                            let a601 = match v5.clone() {
                                Adt2::C0(_, _, h599, _, _, _, _, _, _, _, _) => {
                                    h599
                                }
                            };
                            let a602 = match v5.clone() {
                                Adt2::C0(_, _, _, h600, _, _, _, _, _, _, _) => {
                                    h600
                                }
                            };
                            nat_mul(a601, a602)?
                        };
                        nat_eq(a603, a604)
                    } else {
                        false
                    } {
                        let a607 = match v1.clone() {
                            Adt2::C0(_, _, _, h605, _, _, _, _, _, _, _) => {
                                h605
                            }
                        };
                        let a608 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h606, _, _, _, _, _, _) => {
                                h606
                            }
                        };
                        nat_eq(a607, a608)
                    } else {
                        false
                    } {
                        let a610 = match v0.clone() {
                            Adt1::C0(h609, _, _, _, _, _, _, _, _, _, _, _) => {
                                h609
                            }
                        };
                        let a611 = 4294967296u64;
                        nat_lt(a610, a611)
                    } else {
                        false
                    } {
                        let a613 = match v0.clone() {
                            Adt1::C0(h612, _, _, _, _, _, _, _, _, _, _, _) => {
                                h612
                            }
                        };
                        let a614 = 0u64;
                        if nat_eq(a613, a614) {
                            if f30(v1.clone(), v4.clone())? {
                                true
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
                            let a621 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h619) => {
                                    h619
                                }
                            };
                            let a622 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h620) => {
                                    h620
                                }
                            };
                            nat_eq(a621, a622)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a624 = match v4.clone() {
                                Adt2::C0(_, h623, _, _, _, _, _, _, _, _, _) => {
                                    h623
                                }
                            };
                            let a625 = 0u64;
                            if if if nat_eq(a624, a625) {
                                true
                            } else {
                                let a627 = match v4.clone() {
                                    Adt2::C0(_, _, h626, _, _, _, _, _, _, _, _) => {
                                        h626
                                    }
                                };
                                let a628 = 0u64;
                                nat_eq(a627, a628)
                            } {
                                true
                            } else {
                                let a630 = match v4.clone() {
                                    Adt2::C0(_, _, _, h629, _, _, _, _, _, _, _) => {
                                        h629
                                    }
                                };
                                let a631 = 0u64;
                                nat_eq(a630, a631)
                            } {
                                true
                            } else {
                                let a633 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h632, _, _, _, _, _, _) => {
                                        h632
                                    }
                                };
                                let a634 = 0u64;
                                nat_eq(a633, a634)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h635, _, _, _, _, _, _, _, _, _) => {
                                h635
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a637 = match v0 {
                                Adt1::C0(h636, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h636
                                }
                            };
                            let a638 = 0u64;
                            if nat_eq(a637, a638) {
                                f23(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f40(v8, match v5.clone() {
                                Adt2::C0(_, _, h639, _, _, _, _, _, _, _, _) => {
                                    h639
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h640, _, _, _, _, _, _, _) => {
                                    h640
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h641, _, _, _, _, _, _) => {
                                    h641
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
        let m642 = v3;
        match m642 {
            None => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h643, _, _, _, _, _, _, _, _, _, _, _) => {
                        h643
                    }
                }, match v1 {
                    Adt1::C0(_, h644, _, _, _, _, _, _, _, _, _, _) => {
                        h644
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f43(v0, match v1.clone() {
                    Adt1::C0(h645, _, _, _, _, _, _, _, _, _, _, _) => {
                        h645
                    }
                }, match v1 {
                    Adt1::C0(_, h646, _, _, _, _, _, _, _, _, _, _) => {
                        h646
                    }
                }, true, v5, f42(v7), v6)?
            }
        }
    };
    let m647 = v8;
    match m647 {
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
        Adt2::C0(h648, _, _, _, _, _, _, _, _, _, _) => {
            h648
        }
    };
    let v4: u64 = f44(v3);
    let a650 = match v1.clone() {
        Adt2::C0(h649, _, _, _, _, _, _, _, _, _, _) => {
            h649
        }
    };
    let a651 = 0u64;
    if if if if if if if if nat_eq(a650, a651) {
        true
    } else {
        let a653 = match v1.clone() {
            Adt2::C0(h652, _, _, _, _, _, _, _, _, _, _) => {
                h652
            }
        };
        let a654 = 30u64;
        nat_eq(a653, a654)
    } {
        let a657 = match v1.clone() {
            Adt2::C0(h655, _, _, _, _, _, _, _, _, _, _) => {
                h655
            }
        };
        let a658 = match v2.clone() {
            Adt2::C0(h656, _, _, _, _, _, _, _, _, _, _) => {
                h656
            }
        };
        nat_eq(a657, a658)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f45(match v0.clone() {
            Adt1::C0(h659, _, _, _, _, _, _, _, _, _, _, _) => {
                h659
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
            Adt2::C0(_, h660, _, _, _, _, _, _, _, _, _) => {
                h660
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h661, _, _, _, _, _, _, _, _) => {
                h661
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h662, _, _, _, _, _, _, _) => {
                h662
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h663, _, _, _, _, _, _) => {
                h663
            }
        }) {
            0u64
        } else {
            let a669 = match v1.clone() {
                Adt2::C0(_, _, h664, _, _, _, _, _, _, _, _) => {
                    h664
                }
            };
            let a670 = {
                let a667 = match v1.clone() {
                    Adt2::C0(_, _, _, h665, _, _, _, _, _, _, _) => {
                        h665
                    }
                };
                let a668 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h666, _, _, _, _, _, _) => {
                        h666
                    }
                };
                nat_mul(a667, a668)?
            };
            nat_mul(a669, a670)?
        };
        let v6: u64 = {
            let a671 = v5;
            let a672 = 0u64;
            if if nat_eq(a671, a672) {
                true
            } else {
                f49(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a674 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h673, _) => {
                        h673
                    }
                };
                let a675 = v4;
                let a676 = 0u64;
                nat_quot(a674, a675, a676)
            }
        };
        Ok(Adt10::C4(f50(v3, match v1.clone() {
            Adt2::C0(_, h677, _, _, _, _, _, _, _, _, _) => {
                h677
            }
        }, v5, v6)?, v4, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h678, _, _, _, _, _, _, _, _, _, _, _) => {
                h678
            }
        }, v5, f51(match v1 {
            Adt2::C0(_, h679, _, _, _, _, _, _, _, _, _) => {
                h679
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f12(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a681 = match v1.clone() {
        Adt2::C0(h680, _, _, _, _, _, _, _, _, _, _) => {
            h680
        }
    };
    let a682 = 0u64;
    if if if if if nat_eq(a681, a682) {
        let a684 = match v2.clone() {
            Adt2::C0(h683, _, _, _, _, _, _, _, _, _, _) => {
                h683
            }
        };
        let a685 = 0u64;
        nat_eq(a684, a685)
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        f52(match v0.clone() {
            Adt1::C0(h686, _, _, _, _, _, _, _, _, _, _, _) => {
                h686
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
            let a692 = match v3.clone() {
                Adt3::C0(_, h687, _, _, _, _, _, _) => {
                    h687
                }
            };
            let a693 = {
                let a690 = match v3.clone() {
                    Adt3::C0(_, _, h688, _, _, _, _, _) => {
                        h688
                    }
                };
                let a691 = match v3.clone() {
                    Adt3::C0(_, _, _, h689, _, _, _, _) => {
                        h689
                    }
                };
                nat_mul(a690, a691)?
            };
            nat_mul(a692, a693)?
        };
        Ok(Adt10::C4(f53(match v3.clone() {
            Adt3::C0(h694, _, _, _, _, _, _, _) => {
                h694
            }
        }, v4)?, 4u64, v3, f25(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h695, _, _, _, _, _, _, _, _, _, _, _) => {
                h695
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f13(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m696 = v1;
    match m696 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m697 = v2;
            match m697 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a699 = match v0.clone() {
                        Adt2::C0(h698, _, _, _, _, _, _, _, _, _, _) => {
                            h698
                        }
                    };
                    let a700 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a699, a700) {
                        let a702 = match v4.clone() {
                            Adt2::C0(h701, _, _, _, _, _, _, _, _, _, _) => {
                                h701
                            }
                        };
                        let a703 = 0u64;
                        nat_eq(a702, a703)
                    } else {
                        false
                    } {
                        let a705 = match v5.clone() {
                            Adt2::C0(h704, _, _, _, _, _, _, _, _, _, _) => {
                                h704
                            }
                        };
                        let a706 = 26u64;
                        nat_eq(a705, a706)
                    } else {
                        false
                    } {
                        let a708 = match v3.clone() {
                            Adt2::C0(h707, _, _, _, _, _, _, _, _, _, _) => {
                                h707
                            }
                        };
                        let a709 = 0u64;
                        nat_eq(a708, a709)
                    } else {
                        false
                    } {
                        f21(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a712 = match v0.clone() {
                            Adt2::C0(_, h710, _, _, _, _, _, _, _, _, _) => {
                                h710
                            }
                        };
                        let a713 = match v4.clone() {
                            Adt2::C0(_, h711, _, _, _, _, _, _, _, _, _) => {
                                h711
                            }
                        };
                        nat_eq(a712, a713)
                    } else {
                        false
                    } {
                        let a716 = match v0.clone() {
                            Adt2::C0(_, _, h714, _, _, _, _, _, _, _, _) => {
                                h714
                            }
                        };
                        let a717 = match v5.clone() {
                            Adt2::C0(_, h715, _, _, _, _, _, _, _, _, _) => {
                                h715
                            }
                        };
                        nat_eq(a716, a717)
                    } else {
                        false
                    } {
                        let a720 = match v0.clone() {
                            Adt2::C0(_, _, _, h718, _, _, _, _, _, _, _) => {
                                h718
                            }
                        };
                        let a721 = match v5.clone() {
                            Adt2::C0(_, _, h719, _, _, _, _, _, _, _, _) => {
                                h719
                            }
                        };
                        nat_eq(a720, a721)
                    } else {
                        false
                    } {
                        let a723 = 0u64;
                        let a724 = match v5.clone() {
                            Adt2::C0(_, _, _, h722, _, _, _, _, _, _, _) => {
                                h722
                            }
                        };
                        nat_lt(a723, a724)
                    } else {
                        false
                    } {
                        let a726 = 0u64;
                        let a727 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h725, _, _, _, _, _, _) => {
                                h725
                            }
                        };
                        nat_lt(a726, a727)
                    } else {
                        false
                    } {
                        let a729 = 4u64;
                        let a730 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h728, _, _, _, _, _) => {
                                h728
                            }
                        };
                        nat_le(a729, a730)
                    } else {
                        false
                    } {
                        let a732 = 4u64;
                        let a733 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h731, _, _, _, _, _) => {
                                h731
                            }
                        };
                        nat_le(a732, a733)
                    } else {
                        false
                    } {
                        let a735 = 4u64;
                        let a736 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h734, _, _, _, _, _) => {
                                h734
                            }
                        };
                        nat_le(a735, a736)
                    } else {
                        false
                    } {
                        let a738 = 4u64;
                        let a739 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h737, _, _, _, _, _) => {
                                h737
                            }
                        };
                        nat_le(a738, a739)
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
                            Adt2::C0(_, h740, _, _, _, _, _, _, _, _, _) => {
                                h740
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h741, _, _, _, _, _, _, _, _) => {
                                h741
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h742, _, _, _, _, _, _, _) => {
                                h742
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h743, _, _, _, _, _, _) => {
                                h743
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
    let a744 = v0;
    let a745 = 28u64;
    if if if nat_eq(a744, a745) {
        true
    } else {
        let a746 = v0;
        let a747 = 29u64;
        nat_eq(a746, a747)
    } {
        let a749 = 0u64;
        let a750 = match v1.clone() {
            Adt1::C0(h748, _, _, _, _, _, _, _, _, _, _, _) => {
                h748
            }
        };
        if nat_lt(a749, a750) {
            true
        } else {
            let a752 = 0u64;
            let a753 = match v1.clone() {
                Adt1::C0(_, _, _, h751, _, _, _, _, _, _, _, _) => {
                    h751
                }
            };
            nat_lt(a752, a753)
        }
    } else {
        false
    } {
        let m757 = f55(match v2.clone() {
            Adt2::C0(h754, _, _, _, _, _, _, _, _, _, _) => {
                h754
            }
        }, match v1.clone() {
            Adt1::C0(h755, _, _, _, _, _, _, _, _, _, _, _) => {
                h755
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h756, _, _, _, _, _, _, _, _) => {
                h756
            }
        });
        match m757 {
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
        Adt8::C0(_, h758, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h758
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h759, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h759
        }
    };
    let v6: u64 = {
        let a760 = v3;
        let a761 = v5;
        let a762 = 0u64;
        nat_rem(a760, a761, a762)
    };
    let a763 = v1;
    let a764 = 0u64;
    if nat_eq(a763, a764) {
        let a766 = v3;
        let a767 = match v0.clone() {
            Adt8::C0(_, _, _, h765, _, _, _, _, _, _, _, _, _, _, _) => {
                h765
            }
        };
        if nat_lt(a766, a767) {
            Ok(Adt9::C8(Some(f57(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h768, _, _, _, _, _, _) => {
                    h768
                }
            }, {
                let a772 = v6;
                let a773 = {
                    let m771 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h770, _, _, _, _, _, _) => {
                            h770
                        }
                    };
                    match m771 {
                        Adt3::C0(h769, _, _, _, _, _, _, _) => {
                            h769
                        }
                    }
                };
                let a774 = 0u64;
                nat_rem(a772, a773, a774)
            }, {
                let a784 = {
                    let a778 = v6;
                    let a779 = {
                        let m777 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h776, _, _, _, _, _, _) => {
                                h776
                            }
                        };
                        match m777 {
                            Adt3::C0(h775, _, _, _, _, _, _, _) => {
                                h775
                            }
                        }
                    };
                    let a780 = 0u64;
                    nat_quot(a778, a779, a780)
                };
                let a785 = {
                    let m783 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h782, _, _, _, _, _, _) => {
                            h782
                        }
                    };
                    match m783 {
                        Adt3::C0(_, h781, _, _, _, _, _, _) => {
                            h781
                        }
                    }
                };
                let a786 = 0u64;
                nat_rem(a784, a785, a786)
            }, {
                let a796 = {
                    let a790 = v6;
                    let a791 = {
                        let m789 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h788, _, _, _, _, _, _) => {
                                h788
                            }
                        };
                        match m789 {
                            Adt3::C0(h787, _, _, _, _, _, _, _) => {
                                h787
                            }
                        }
                    };
                    let a792 = 0u64;
                    nat_quot(a790, a791, a792)
                };
                let a797 = {
                    let m795 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h794, _, _, _, _, _, _) => {
                            h794
                        }
                    };
                    match m795 {
                        Adt3::C0(_, h793, _, _, _, _, _, _) => {
                            h793
                        }
                    }
                };
                let a798 = 0u64;
                nat_quot(a796, a797, a798)
            }, {
                let a799 = v3;
                let a800 = v5;
                let a801 = 0u64;
                nat_quot(a799, a800, a801)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a807 = v2;
            let a808 = {
                let a804 = v3;
                let a805 = {
                    let a802 = v1;
                    let a803 = 1u64;
                    if nat_eq(a802, a803) {
                        v4
                    } else {
                        v5
                    }
                };
                let a806 = 0u64;
                nat_quot(a804, a805, a806)
            };
            nat_add(a807, a808)?
        };
        let a810 = v7;
        let a811 = match v0.clone() {
            Adt8::C0(h809, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h809
            }
        };
        if nat_lt(a810, a811) {
            let v8: u64 = {
                let a815 = v7;
                let a816 = {
                    let m814 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h813, _, _, _, _, _) => {
                            h813
                        }
                    };
                    match m814 {
                        Adt3::C0(h812, _, _, _, _, _, _, _) => {
                            h812
                        }
                    }
                };
                let a817 = 0u64;
                nat_rem(a815, a816, a817)
            };
            let v9: u64 = {
                let a821 = v7;
                let a822 = {
                    let m820 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h819, _, _, _, _, _) => {
                            h819
                        }
                    };
                    match m820 {
                        Adt3::C0(h818, _, _, _, _, _, _, _) => {
                            h818
                        }
                    }
                };
                let a823 = 0u64;
                nat_quot(a821, a822, a823)
            };
            let v10: u64 = {
                let a827 = v9;
                let a828 = {
                    let m826 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h825, _, _, _, _, _) => {
                            h825
                        }
                    };
                    match m826 {
                        Adt3::C0(_, h824, _, _, _, _, _, _) => {
                            h824
                        }
                    }
                };
                let a829 = 0u64;
                nat_rem(a827, a828, a829)
            };
            let v11: u64 = {
                let a833 = v9;
                let a834 = {
                    let m832 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h831, _, _, _, _, _) => {
                            h831
                        }
                    };
                    match m832 {
                        Adt3::C0(_, h830, _, _, _, _, _, _) => {
                            h830
                        }
                    }
                };
                let a835 = 0u64;
                nat_quot(a833, a834, a835)
            };
            let a836 = v1;
            let a837 = 1u64;
            if nat_eq(a836, a837) {
                Ok(Adt9::C8(Some(f57(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h838, _, _, _, _, _) => {
                        h838
                    }
                }, v8, v10, {
                    let a839 = v3;
                    let a840 = v4;
                    let a841 = 0u64;
                    nat_rem(a839, a840, a841)
                }, v11)?)))
            } else {
                let a842 = v1;
                let a843 = 2u64;
                if nat_eq(a842, a843) {
                    Ok(Adt9::C8(Some({
                        let a889 = f57(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h844, _, _, _, _) => {
                                h844
                            }
                        }, {
                            let a856 = {
                                let a848 = v8;
                                let a849 = {
                                    let m847 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h846, _, _, _) => {
                                            h846
                                        }
                                    };
                                    match m847 {
                                        Adt6::C0(_, h845, _, _) => {
                                            h845
                                        }
                                    }
                                };
                                nat_mul(a848, a849)?
                            };
                            let a857 = {
                                let a853 = v6;
                                let a854 = {
                                    let m852 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h851, _, _, _, _, _, _) => {
                                            h851
                                        }
                                    };
                                    match m852 {
                                        Adt3::C0(h850, _, _, _, _, _, _, _) => {
                                            h850
                                        }
                                    }
                                };
                                let a855 = 0u64;
                                nat_rem(a853, a854, a855)
                            };
                            nat_add(a856, a857)?
                        }, {
                            let a875 = {
                                let a861 = v10;
                                let a862 = {
                                    let m860 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h859, _, _, _) => {
                                            h859
                                        }
                                    };
                                    match m860 {
                                        Adt6::C0(_, h858, _, _) => {
                                            h858
                                        }
                                    }
                                };
                                nat_mul(a861, a862)?
                            };
                            let a876 = {
                                let a872 = {
                                    let a866 = v6;
                                    let a867 = {
                                        let m865 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h864, _, _, _, _, _, _) => {
                                                h864
                                            }
                                        };
                                        match m865 {
                                            Adt3::C0(h863, _, _, _, _, _, _, _) => {
                                                h863
                                            }
                                        }
                                    };
                                    let a868 = 0u64;
                                    nat_quot(a866, a867, a868)
                                };
                                let a873 = {
                                    let m871 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h870, _, _, _, _, _, _) => {
                                            h870
                                        }
                                    };
                                    match m871 {
                                        Adt3::C0(_, h869, _, _, _, _, _, _) => {
                                            h869
                                        }
                                    }
                                };
                                let a874 = 0u64;
                                nat_rem(a872, a873, a874)
                            };
                            nat_add(a875, a876)?
                        }, {
                            let a886 = {
                                let a880 = v6;
                                let a881 = {
                                    let m879 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h878, _, _, _, _, _, _) => {
                                            h878
                                        }
                                    };
                                    match m879 {
                                        Adt3::C0(h877, _, _, _, _, _, _, _) => {
                                            h877
                                        }
                                    }
                                };
                                let a882 = 0u64;
                                nat_quot(a880, a881, a882)
                            };
                            let a887 = {
                                let m885 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h884, _, _, _, _, _, _) => {
                                        h884
                                    }
                                };
                                match m885 {
                                    Adt3::C0(_, h883, _, _, _, _, _, _) => {
                                        h883
                                    }
                                }
                            };
                            let a888 = 0u64;
                            nat_quot(a886, a887, a888)
                        }, v11)?;
                        let a890 = 4u64;
                        let a891 = 0u64;
                        nat_quot(a889, a890, a891)
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
    let a892 = v1;
    let a893 = 0u64;
    if nat_eq(a892, a893) {
        let a895 = v3;
        let a896 = match v0.clone() {
            Adt8::C0(_, _, _, h894, _, _, _, _, _, _, _, _, _, _, _) => {
                h894
            }
        };
        if nat_lt(a895, a896) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h897, _, _, _, _, _, _) => {
                    h897
                }
            };
            let v5: u64 = {
                let a899 = v3;
                let a900 = match v0.clone() {
                    Adt8::C0(_, _, h898, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h898
                    }
                };
                let a901 = 0u64;
                nat_quot(a899, a900, a901)
            };
            Ok(Adt9::C8(Some(f57(v4.clone(), f58(v5, v4.clone(), 0u64), f58(v5, v4.clone(), 1u64), f58(v5, v4, 2u64), f59(v0.clone(), v5, {
                let a903 = v3;
                let a904 = match v0 {
                    Adt8::C0(_, _, h902, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h902
                    }
                };
                let a905 = 0u64;
                nat_rem(a903, a904, a905)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h906, _, _, _, _) => {
                h906
            }
        };
        let v7: u64 = {
            let a907 = v1;
            let a908 = 1u64;
            if nat_eq(a907, a908) {
                match v0.clone() {
                    Adt8::C0(_, h909, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h909
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h910, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h910
                    }
                }
            }
        };
        let v8: u64 = {
            let a914 = v2;
            let a915 = {
                let a911 = v3;
                let a912 = v7;
                let a913 = 0u64;
                nat_quot(a911, a912, a913)
            };
            nat_add(a914, a915)?
        };
        let v9: u64 = {
            let a916 = v3;
            let a917 = v7;
            let a918 = 0u64;
            nat_rem(a916, a917, a918)
        };
        let v10: u64 = {
            let a920 = v8;
            let a921 = match v6.clone() {
                Adt3::C0(h919, _, _, _, _, _, _, _) => {
                    h919
                }
            };
            let a922 = 0u64;
            nat_rem(a920, a921, a922)
        };
        let v11: u64 = {
            let a924 = v8;
            let a925 = match v6.clone() {
                Adt3::C0(h923, _, _, _, _, _, _, _) => {
                    h923
                }
            };
            let a926 = 0u64;
            nat_quot(a924, a925, a926)
        };
        let v12: u64 = {
            let a928 = v11;
            let a929 = match v6.clone() {
                Adt3::C0(_, h927, _, _, _, _, _, _) => {
                    h927
                }
            };
            let a930 = 0u64;
            nat_rem(a928, a929, a930)
        };
        let v13: u64 = {
            let a932 = v11;
            let a933 = match v6.clone() {
                Adt3::C0(_, h931, _, _, _, _, _, _) => {
                    h931
                }
            };
            let a934 = 0u64;
            nat_quot(a932, a933, a934)
        };
        let a936 = v8;
        let a937 = match v0.clone() {
            Adt8::C0(h935, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h935
            }
        };
        if nat_lt(a936, a937) {
            let a938 = v1;
            let a939 = 1u64;
            if nat_eq(a938, a939) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h940, _, _, _, _, _, _) => {
                        h940
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h941, _, _, _, _, _) => {
                        h941
                    }
                };
                let m944 = f60(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h942, _, _, _) => {
                        h942
                    }
                }, match v15.clone() {
                    Adt3::C0(h943, _, _, _, _, _, _, _) => {
                        h943
                    }
                }, v10, f58(v9, v14.clone(), 0u64))?;
                match m944 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m947 = f60(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h945, _, _) => {
                                h945
                            }
                        }, match v15 {
                            Adt3::C0(_, h946, _, _, _, _, _, _) => {
                                h946
                            }
                        }, v12, f58(v9, v14, 1u64))?;
                        match m947 {
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
                let a948 = v1;
                let a949 = 2u64;
                if nat_eq(a948, a949) {
                    Ok(Adt9::C8(Some({
                        let a951 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h950) => {
                                h950
                            }
                        };
                        let a952 = 0u64;
                        if nat_eq(a951, a952) {
                            f57(v6, v10, v12, v9, v13)?
                        } else {
                            f57(v6.clone(), v10, v12, {
                                let a954 = v13;
                                let a955 = match v6.clone() {
                                    Adt3::C0(_, _, h953, _, _, _, _, _) => {
                                        h953
                                    }
                                };
                                let a956 = 0u64;
                                nat_rem(a954, a955, a956)
                            }, {
                                let a964 = {
                                    let a962 = {
                                        let a958 = v13;
                                        let a959 = match v6 {
                                            Adt3::C0(_, _, h957, _, _, _, _, _) => {
                                                h957
                                            }
                                        };
                                        let a960 = 0u64;
                                        nat_quot(a958, a959, a960)
                                    };
                                    let a963 = match v0 {
                                        Adt8::C0(_, _, h961, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h961
                                        }
                                    };
                                    nat_mul(a962, a963)?
                                };
                                let a965 = v9;
                                nat_add(a964, a965)?
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
        let m966 = v3.clone();
        match m966 {
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
                f77(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C19(v76, v77) => {
                f78(v76, v0, v1, v2, v77, v4)?
            }
            Adt4::C20(v78, v79) => {
                f79(v78, v0, v1, v2, v79, v4)?
            }
            Adt4::C21 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C22(v80, v81) => {
                f80(v0, v1, v2, v80, v81, v4)?
            }
            Adt4::C23(v95, v96, v97, v98) => {
                f87(v0, v98, v95, v96, v97, v4)?
            }
            Adt4::C24(v94) => {
                if v5 {
                    f85(Adt16::C0(v94), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C26(v90, v91, v92, v93) => {
                let a968 = v90;
                let a969 = 64u64;
                if if if nat_eq(a968, a969) {
                    true
                } else {
                    let a970 = v90;
                    let a971 = 65u64;
                    nat_eq(a970, a971)
                } {
                    true
                } else {
                    let a972 = v90;
                    let a973 = 66u64;
                    nat_eq(a972, a973)
                } {
                    if v5 {
                        f85(Adt16::C1(v90, v91, v92, v93), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a974 = v90;
                    let a975 = 0u64;
                    if if nat_eq(a974, a975) {
                        true
                    } else {
                        let a976 = v90;
                        let a977 = 30u64;
                        nat_eq(a976, a977)
                    } {
                        f86(v0, v1, v2, v90, v91, v92, v93, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v89) => {
                f84(v0, v1, v2, v89)?
            }
            Adt4::C28(v85) => {
                f81(v0, v1, v2, v85, v4)?
            }
            Adt4::C29(v86) => {
                f82(v0, v1, v2, v86)?
            }
            Adt4::C30(v87, _) => {
                f83(v0, v1, v2, v87)?
            }
            Adt4::C31(v33, v34) => {
                f64(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f65(v0, match v1 {
                    Adt3::C0(h967, _, _, _, _, _, _, _) => {
                        h967
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f66(v0, v1, v36, v2)?
            }
            Adt4::C34(v56, v57, v58, v59) => {
                f76(v0, v1, v56, v2, v57, v58, v59)?
            }
            Adt4::C35(v55) => {
                f75(v0, v55, v1, v2)?
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
                f74(v0, v1, v2, v4)?
            }
            Adt4::C42(v53, v54) => {
                f73(v0, v1, v2, v53, v54)?
            }
            Adt4::C43(v51, v52) => {
                f72(v0, v1, v2, v51, v52, v4)?
            }
        }
    }))
}

pub fn f19(v0: Adt4) -> R<Adt9> {
    let m978 = v0;
    match m978 {
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
        Adt4::C25(v73, _, v75) => {
            Ok(Adt9::C6({
                let a984 = {
                    let a980 = 0u64;
                    let a981 = v73;
                    if nat_lt(a980, a981) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a985 = {
                    let a982 = 0u64;
                    let a983 = v75;
                    if nat_lt(a982, a983) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a984, a985)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v81, _, _, v84) => {
            Ok(Adt9::C6({
                let a986 = v81;
                let a987 = 66u64;
                if nat_eq(a986, a987) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v84))
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
        Adt4::C35(v46) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v46))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m979 = v32;
                match m979 {
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
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a988 = v1;
    let a989 = 5u64;
    if if if if if nat_lt(a988, a989) {
        true
    } else {
        f88(v0, v1, v2, v3, v4)
    } {
        let a990 = v5;
        let a991 = 5u64;
        nat_lt(a990, a991)
    } else {
        false
    } {
        let a992 = v7;
        let a993 = 5u64;
        nat_lt(a992, a993)
    } else {
        false
    } {
        let a994 = v9;
        let a995 = 2u64;
        if nat_lt(a994, a995) {
            true
        } else {
            let a996 = v9;
            let a997 = 3u64;
            nat_eq(a996, a997)
        }
    } else {
        false
    } {
        Adt9::C7(f89(v0, v1, v2, v3, v4), f90(v5, v6), f90(v7, v8), f91({
            let a998 = v9;
            let a999 = 0u64;
            if nat_eq(a998, a999) {
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
    let a1002 = match v0.clone() {
        Adt2::C0(_, h1000, _, _, _, _, _, _, _, _, _) => {
            h1000
        }
    };
    let a1003 = match v1.clone() {
        Adt2::C0(_, h1001, _, _, _, _, _, _, _, _, _) => {
            h1001
        }
    };
    if if if nat_eq(a1002, a1003) {
        let a1006 = match v0.clone() {
            Adt2::C0(_, _, h1004, _, _, _, _, _, _, _, _) => {
                h1004
            }
        };
        let a1007 = match v1.clone() {
            Adt2::C0(_, _, h1005, _, _, _, _, _, _, _, _) => {
                h1005
            }
        };
        nat_eq(a1006, a1007)
    } else {
        false
    } {
        let a1010 = match v0.clone() {
            Adt2::C0(_, _, _, h1008, _, _, _, _, _, _, _) => {
                h1008
            }
        };
        let a1011 = match v1.clone() {
            Adt2::C0(_, _, _, h1009, _, _, _, _, _, _, _) => {
                h1009
            }
        };
        nat_eq(a1010, a1011)
    } else {
        false
    } {
        let a1014 = match v0 {
            Adt2::C0(_, _, _, _, h1012, _, _, _, _, _, _) => {
                h1012
            }
        };
        let a1015 = match v1 {
            Adt2::C0(_, _, _, _, h1013, _, _, _, _, _, _) => {
                h1013
            }
        };
        nat_eq(a1014, a1015)
    } else {
        false
    }
}

pub fn f22(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1017 = match v0.clone() {
        Adt2::C0(h1016, _, _, _, _, _, _, _, _, _, _) => {
            h1016
        }
    };
    let a1018 = 0u64;
    if if if nat_eq(a1017, a1018) {
        let a1020 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1019, _, _, _, _, _) => {
                h1019
            }
        };
        let a1021 = 4u64;
        nat_eq(a1020, a1021)
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
        let a1033 = {
            let a1030 = {
                let a1025 = {
                    let a1022 = v1;
                    let a1023 = 1u64;
                    nat_sub(a1022, a1023)
                };
                let a1026 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1024, _, _, _, _) => {
                        h1024
                    }
                };
                nat_mul(a1025, a1026)?
            };
            let a1031 = {
                let a1028 = 4u64;
                let a1029 = match v0.clone() {
                    Adt2::C0(_, h1027, _, _, _, _, _, _, _, _, _) => {
                        h1027
                    }
                };
                nat_mul(a1028, a1029)?
            };
            nat_add(a1030, a1031)?
        };
        let a1034 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1032, _) => {
                h1032
            }
        };
        Ok(nat_le(a1033, a1034))
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a1046 = {
        let a1040 = {
            let a1036 = match v0.clone() {
                Adt2::C0(_, h1035, _, _, _, _, _, _, _, _, _) => {
                    h1035
                }
            };
            let a1037 = v1;
            let a1038 = 0u64;
            nat_quot(a1036, a1037, a1038)
        };
        let a1041 = match v0.clone() {
            Adt2::C0(_, _, h1039, _, _, _, _, _, _, _, _) => {
                h1039
            }
        };
        nat_mul(a1040, a1041)?
    };
    let a1047 = {
        let a1044 = match v0.clone() {
            Adt2::C0(_, _, _, h1042, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1045 = match v0 {
            Adt2::C0(_, _, _, _, h1043, _, _, _, _, _, _) => {
                h1043
            }
        };
        nat_mul(a1044, a1045)?
    };
    nat_mul(a1046, a1047)
}

pub fn f24(v0: bool, v1: u64) -> R<u64> {
    let a1050 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1051 = {
        let a1048 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1049 = v1;
        nat_mul(a1048, a1049)?
    };
    nat_add(a1050, a1051)
}

pub fn f25(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1053 = match v0.clone() {
            Adt2::C0(_, h1052, _, _, _, _, _, _, _, _, _) => {
                h1052
            }
        };
        let a1054 = v1;
        let a1055 = 0u64;
        nat_quot(a1053, a1054, a1055)
    }, match v0.clone() {
        Adt2::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
            h1056
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1057, _, _, _, _, _, _, _) => {
            h1057
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1058, _, _, _, _, _, _) => {
            h1058
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1059, _, _, _, _, _) => {
            h1059
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1060, _, _, _, _) => {
            h1060
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1061, _, _, _) => {
            h1061
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1062, _, _) => {
            h1062
        }
    })
}

pub fn f26(v0: Adt2, v1: u64) -> R<bool> {
    let a1063 = v1;
    let a1064 = 0u64;
    if nat_eq(a1063, a1064) {
        Ok(true)
    } else {
        let a1070 = {
            let a1067 = f92({
                let a1065 = v1;
                let a1066 = 1u64;
                nat_sub(a1065, a1066)
            }, f25(v0.clone(), 1u64))?;
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
}

pub fn f27(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1073 = match v1.clone() {
        Adt2::C0(h1072, _, _, _, _, _, _, _, _, _, _) => {
            h1072
        }
    };
    let a1074 = 0u64;
    if if nat_eq(a1073, a1074) {
        f21(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1075 = f23(v0.clone(), 1u64)?;
        let a1076 = 0u64;
        if nat_eq(a1075, a1076) {
            Ok(true)
        } else {
            let a1082 = {
                let a1079 = f92({
                    let a1077 = f23(v0.clone(), 1u64)?;
                    let a1078 = 1u64;
                    nat_sub(a1077, a1078)
                }, f25(v0, 1u64))?;
                let a1080 = 4u64;
                nat_add(a1079, a1080)?
            };
            let a1083 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1081, _) => {
                    h1081
                }
            };
            Ok(nat_le(a1082, a1083))
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
    let a1085 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1084, _) => {
            h1084
        }
    };
    let a1086 = 0u64;
    if nat_eq(a1085, a1086) {
        Ok(true)
    } else {
        let a1088 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1087, _) => {
                h1087
            }
        };
        let a1089 = 0u64;
        if nat_eq(a1088, a1089) {
            Ok(true)
        } else {
            let a1091 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1090) => {
                    h1090
                }
            };
            let a1092 = 0u64;
            if nat_eq(a1091, a1092) {
                Ok(true)
            } else {
                let a1094 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1093) => {
                        h1093
                    }
                };
                let a1095 = 0u64;
                if nat_eq(a1094, a1095) {
                    Ok(true)
                } else {
                    let a1101 = {
                        let a1098 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1096) => {
                                h1096
                            }
                        };
                        let a1099 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1097, _) => {
                                h1097
                            }
                        };
                        nat_add(a1098, a1099)?
                    };
                    let a1102 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1100) => {
                            h1100
                        }
                    };
                    if nat_le(a1101, a1102) {
                        Ok(true)
                    } else {
                        let a1108 = {
                            let a1105 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1103) => {
                                    h1103
                                }
                            };
                            let a1106 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1104, _) => {
                                    h1104
                                }
                            };
                            nat_add(a1105, a1106)?
                        };
                        let a1109 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1107) => {
                                h1107
                            }
                        };
                        Ok(nat_le(a1108, a1109))
                    }
                }
            }
        }
    }
}

pub fn f31(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1115 = match v0.clone() {
            Adt2::C0(_, _, h1110, _, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1116 = {
            let a1113 = match v0.clone() {
                Adt2::C0(_, _, _, h1111, _, _, _, _, _, _, _) => {
                    h1111
                }
            };
            let a1114 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1112, _, _, _, _, _, _) => {
                    h1112
                }
            };
            nat_mul(a1113, a1114)?
        };
        nat_mul(a1115, a1116)?
    };
    let v2: u64 = f23(v0.clone(), 1u64)?;
    let a1118 = match v0.clone() {
        Adt2::C0(h1117, _, _, _, _, _, _, _, _, _, _) => {
            h1117
        }
    };
    let a1119 = 0u64;
    if if if if if if if nat_eq(a1118, a1119) {
        let a1121 = match v0.clone() {
            Adt2::C0(_, h1120, _, _, _, _, _, _, _, _, _) => {
                h1120
            }
        };
        let a1122 = 2147483647u64;
        nat_le(a1121, a1122)
    } else {
        false
    } {
        let a1123 = v1;
        let a1124 = 2147483647u64;
        nat_le(a1123, a1124)
    } else {
        false
    } {
        let a1126 = match v0.clone() {
            Adt2::C0(_, h1125, _, _, _, _, _, _, _, _, _) => {
                h1125
            }
        };
        let a1127 = 1u64;
        if nat_eq(a1126, a1127) {
            true
        } else {
            let a1129 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1128, _, _, _, _, _) => {
                    h1128
                }
            };
            let a1130 = 4u64;
            nat_eq(a1129, a1130)
        }
    } else {
        false
    } {
        let a1131 = v1;
        let a1132 = 1u64;
        if nat_le(a1131, a1132) {
            true
        } else {
            let a1137 = {
                let a1134 = match v0.clone() {
                    Adt2::C0(_, h1133, _, _, _, _, _, _, _, _, _) => {
                        h1133
                    }
                };
                let a1135 = 4u64;
                nat_mul(a1134, a1135)?
            };
            let a1138 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1136, _, _, _, _) => {
                    h1136
                }
            };
            nat_le(a1137, a1138)
        }
    } else {
        false
    } {
        let a1140 = match v0.clone() {
            Adt2::C0(_, _, _, h1139, _, _, _, _, _, _, _) => {
                h1139
            }
        };
        let a1141 = 1u64;
        if nat_eq(a1140, a1141) {
            true
        } else {
            let a1147 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1142, _, _, _) => {
                    h1142
                }
            };
            let a1148 = {
                let a1145 = match v0.clone() {
                    Adt2::C0(_, _, h1143, _, _, _, _, _, _, _, _) => {
                        h1143
                    }
                };
                let a1146 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1144, _, _, _, _) => {
                        h1144
                    }
                };
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
            true
        } else {
            let a1160 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1152, _, _) => {
                    h1152
                }
            };
            let a1161 = {
                let a1158 = match v0.clone() {
                    Adt2::C0(_, _, _, h1153, _, _, _, _, _, _, _) => {
                        h1153
                    }
                };
                let a1159 = {
                    let a1156 = match v0.clone() {
                        Adt2::C0(_, _, h1154, _, _, _, _, _, _, _, _) => {
                            h1154
                        }
                    };
                    let a1157 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1155, _, _, _, _) => {
                            h1155
                        }
                    };
                    nat_mul(a1156, a1157)?
                };
                nat_mul(a1158, a1159)?
            };
            nat_eq(a1160, a1161)
        }
    } else {
        false
    } {
        let a1162 = v2;
        let a1163 = 0u64;
        if nat_eq(a1162, a1163) {
            Ok(true)
        } else {
            let a1169 = {
                let a1166 = f92({
                    let a1164 = v2;
                    let a1165 = 1u64;
                    nat_sub(a1164, a1165)
                }, f25(v0.clone(), 1u64))?;
                let a1167 = 4u64;
                nat_add(a1166, a1167)?
            };
            let a1170 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1168, _) => {
                    h1168
                }
            };
            Ok(nat_le(a1169, a1170))
        }
    } else {
        Ok(false)
    }
}

pub fn f32(v0: u64) -> R<u64> {
    let a1177 = 148u64;
    let a1178 = {
        let a1175 = 36u64;
        let a1176 = {
            let a1173 = {
                let a1171 = 2u64;
                let a1172 = v0;
                nat_mul(a1171, a1172)?
            };
            let a1174 = 1u64;
            nat_sub(a1173, a1174)
        };
        nat_mul(a1175, a1176)?
    };
    nat_add(a1177, a1178)
}

pub fn f33(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1179 = v1;
    match m1179 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1182 = 1u64;
            let a1183 = {
                let a1180 = 2u64;
                let a1181 = v0;
                nat_mul(a1180, a1181)?
            };
            nat_add(a1182, a1183)
        }
    }
}

pub fn f34(v0: u64) -> bool {
    let a1184 = v0;
    let a1185 = 4294967296u64;
    if nat_lt(a1184, a1185) {
        let a1189 = {
            let a1186 = v0;
            let a1187 = 2147483648u64;
            let a1188 = 0u64;
            nat_rem(a1186, a1187, a1188)
        };
        let a1190 = 2139095040u64;
        nat_lt(a1189, a1190)
    } else {
        false
    }
}

pub fn f35(v0: u64) -> R<u64> {
    let a1191 = 4u64;
    let a1192 = v0;
    nat_mul(a1191, a1192)
}

pub fn f36(v0: u64) -> R<u64> {
    let a1193 = v0;
    let a1194 = 0u64;
    if nat_eq(a1193, a1194) {
        Ok(0u64)
    } else {
        let a1197 = 4u64;
        let a1198 = {
            let a1195 = 3u64;
            let a1196 = v0;
            nat_mul(a1195, a1196)?
        };
        nat_add(a1197, a1198)
    }
}

pub fn f37(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f25(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1199, _, _, _, _, _, _, _, _, _) => {
                h1199
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1200, _, _, _, _, _, _, _, _) => {
                h1200
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1201, _, _, _, _, _, _, _) => {
                h1201
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1202, _, _, _, _, _) => {
                h1202
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1203, _, _, _, _) => {
                h1203
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1204, _, _, _) => {
                h1204
            }
        })
    }
}

pub fn f38(v0: Adt2) -> R<bool> {
    let a1206 = match v0.clone() {
        Adt2::C0(_, h1205, _, _, _, _, _, _, _, _, _) => {
            h1205
        }
    };
    let a1207 = 1u64;
    if if if if nat_eq(a1206, a1207) {
        true
    } else {
        let a1209 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1208, _, _, _, _, _) => {
                h1208
            }
        };
        let a1210 = 4u64;
        nat_eq(a1209, a1210)
    } {
        let a1212 = match v0.clone() {
            Adt2::C0(_, _, h1211, _, _, _, _, _, _, _, _) => {
                h1211
            }
        };
        let a1213 = 1u64;
        if nat_eq(a1212, a1213) {
            true
        } else {
            let a1218 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1214, _, _, _, _) => {
                    h1214
                }
            };
            let a1219 = {
                let a1216 = match v0.clone() {
                    Adt2::C0(_, h1215, _, _, _, _, _, _, _, _, _) => {
                        h1215
                    }
                };
                let a1217 = 4u64;
                nat_mul(a1216, a1217)?
            };
            nat_eq(a1218, a1219)
        }
    } else {
        false
    } {
        let a1221 = match v0.clone() {
            Adt2::C0(_, _, _, h1220, _, _, _, _, _, _, _) => {
                h1220
            }
        };
        let a1222 = 1u64;
        if nat_eq(a1221, a1222) {
            true
        } else {
            let a1230 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1223, _, _, _) => {
                    h1223
                }
            };
            let a1231 = {
                let a1228 = {
                    let a1226 = match v0.clone() {
                        Adt2::C0(_, h1224, _, _, _, _, _, _, _, _, _) => {
                            h1224
                        }
                    };
                    let a1227 = match v0.clone() {
                        Adt2::C0(_, _, h1225, _, _, _, _, _, _, _, _) => {
                            h1225
                        }
                    };
                    nat_mul(a1226, a1227)?
                };
                let a1229 = 4u64;
                nat_mul(a1228, a1229)?
            };
            nat_eq(a1230, a1231)
        }
    } else {
        false
    } {
        let a1233 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1232, _, _, _, _, _, _) => {
                h1232
            }
        };
        let a1234 = 1u64;
        if nat_eq(a1233, a1234) {
            Ok(true)
        } else {
            let a1245 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1235, _, _) => {
                    h1235
                }
            };
            let a1246 = {
                let a1243 = {
                    let a1241 = {
                        let a1238 = match v0.clone() {
                            Adt2::C0(_, h1236, _, _, _, _, _, _, _, _, _) => {
                                h1236
                            }
                        };
                        let a1239 = match v0.clone() {
                            Adt2::C0(_, _, h1237, _, _, _, _, _, _, _, _) => {
                                h1237
                            }
                        };
                        nat_mul(a1238, a1239)?
                    };
                    let a1242 = match v0 {
                        Adt2::C0(_, _, _, h1240, _, _, _, _, _, _, _) => {
                            h1240
                        }
                    };
                    nat_mul(a1241, a1242)?
                };
                let a1244 = 4u64;
                nat_mul(a1243, a1244)?
            };
            Ok(nat_eq(a1245, a1246))
        }
    } else {
        Ok(false)
    }
}

pub fn f39(v0: Adt6, v1: u64) -> R<u64> {
    let a1248 = match v0.clone() {
        Adt6::C0(h1247, _, _, _) => {
            h1247
        }
    };
    let a1249 = 0u64;
    if nat_eq(a1248, a1249) {
        f93(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1250, _, _) => {
                h1250
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1251, _) => {
                h1251
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1252) => {
                h1252
            }
        }), {
            let a1254 = v1;
            let a1255 = match v0 {
                Adt6::C0(_, _, _, h1253) => {
                    h1253
                }
            };
            nat_add(a1254, a1255)?
        })
    } else {
        f93(v0, v1)
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1260 = {
        let a1256 = v0;
        let a1257 = v1;
        nat_mul(a1256, a1257)?
    };
    let a1261 = {
        let a1258 = v2;
        let a1259 = f94(v3);
        nat_mul(a1258, a1259)?
    };
    nat_mul(a1260, a1261)
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1264 = v0;
    let a1265 = {
        let a1262 = f95(v1)?;
        let a1263 = v2;
        nat_mul(a1262, a1263)?
    };
    nat_add(a1264, a1265)
}

pub fn f42(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1266, _, _, _, _, _, _, _, _, _, _) => {
            h1266
        }
    }, match v0.clone() {
        Adt2::C0(_, h1267, _, _, _, _, _, _, _, _, _) => {
            h1267
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1268, _, _, _, _, _, _, _, _) => {
            h1268
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1269, _, _, _, _, _, _, _) => {
            h1269
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1270, _, _, _, _, _, _) => {
            h1270
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1271, _, _, _, _, _) => {
            h1271
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1272, _, _, _, _) => {
            h1272
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1273, _, _, _) => {
            h1273
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1274, _, _) => {
            h1274
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1275, _) => {
            h1275
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1276) => {
            h1276
        }
    })
}

pub fn f43(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1277 = v0;
    let a1278 = 23u64;
    if nat_eq(a1277, a1278) {
        f96(v1, v4, v6)
    } else {
        let a1279 = v0;
        let a1280 = 26u64;
        if nat_eq(a1279, a1280) {
            f97(v1, v2, v4, v6)
        } else {
            let a1281 = v0;
            let a1282 = 25u64;
            if nat_eq(a1281, a1282) {
                f98(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f44(v0: u64) -> u64 {
    let a1283 = v0;
    let a1284 = 30u64;
    if nat_eq(a1283, a1284) {
        2u64
    } else {
        4u64
    }
}

pub fn f45(v0: u64) -> bool {
    let a1285 = v0;
    let a1286 = 2139095040u64;
    if nat_le(a1285, a1286) {
        true
    } else {
        let a1287 = v0;
        let a1288 = 2147483648u64;
        nat_eq(a1287, a1288)
    }
}

pub fn f46(v0: Adt2, v1: u64) -> bool {
    let a1290 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1289, _, _, _, _, _) => {
            h1289
        }
    };
    let a1291 = v1;
    if if nat_eq(a1290, a1291) {
        let a1296 = {
            let a1293 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1292) => {
                    h1292
                }
            };
            let a1294 = v1;
            let a1295 = 0u64;
            nat_rem(a1293, a1294, a1295)
        };
        let a1297 = 0u64;
        nat_eq(a1296, a1297)
    } else {
        false
    } {
        f99(match v0.clone() {
            Adt2::C0(_, _, h1298, _, _, _, _, _, _, _, _) => {
                h1298
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1299, _, _, _, _, _, _, _) => {
                h1299
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1300, _, _, _, _, _, _) => {
                h1300
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1301, _, _, _, _) => {
                h1301
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1302, _, _, _) => {
                h1302
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1303, _, _) => {
                h1303
            }
        }, v1)
    } else {
        false
    }
}

pub fn f47(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f21(v0.clone(), v1.clone()) {
        let a1306 = match v0.clone() {
            Adt2::C0(h1304, _, _, _, _, _, _, _, _, _, _) => {
                h1304
            }
        };
        let a1307 = match v1.clone() {
            Adt2::C0(h1305, _, _, _, _, _, _, _, _, _, _) => {
                h1305
            }
        };
        nat_eq(a1306, a1307)
    } else {
        false
    } {
        let a1310 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1308) => {
                h1308
            }
        };
        let a1311 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1309) => {
                h1309
            }
        };
        nat_eq(a1310, a1311)
    } else {
        false
    } {
        let a1314 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1312, _, _, _, _, _) => {
                h1312
            }
        };
        let a1315 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1313, _, _, _, _, _) => {
                h1313
            }
        };
        nat_eq(a1314, a1315)
    } else {
        false
    } {
        let a1318 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1316, _, _, _, _) => {
                h1316
            }
        };
        let a1319 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1317, _, _, _, _) => {
                h1317
            }
        };
        nat_eq(a1318, a1319)
    } else {
        false
    } {
        let a1322 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1320, _, _, _) => {
                h1320
            }
        };
        let a1323 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1321, _, _, _) => {
                h1321
            }
        };
        nat_eq(a1322, a1323)
    } else {
        false
    } {
        let a1326 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1324, _, _) => {
                h1324
            }
        };
        let a1327 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1325, _, _) => {
                h1325
            }
        };
        nat_eq(a1326, a1327)
    } else {
        false
    }
}

pub fn f48(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1328 = v0;
    let a1329 = 0u64;
    if if if nat_eq(a1328, a1329) {
        true
    } else {
        let a1330 = v1;
        let a1331 = 0u64;
        nat_eq(a1330, a1331)
    } {
        true
    } else {
        let a1332 = v2;
        let a1333 = 0u64;
        nat_eq(a1332, a1333)
    } {
        true
    } else {
        let a1334 = v3;
        let a1335 = 0u64;
        nat_eq(a1334, a1335)
    }
}

pub fn f49(v0: Adt2, v1: u64) -> R<bool> {
    f100({
        let a1337 = match v0.clone() {
            Adt2::C0(_, h1336, _, _, _, _, _, _, _, _, _) => {
                h1336
            }
        };
        let a1338 = v1;
        nat_mul(a1337, a1338)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1339, _, _, _, _, _, _, _, _) => {
            h1339
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1340, _, _, _, _, _, _, _) => {
            h1340
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1341, _, _, _, _, _, _) => {
            h1341
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1342, _, _, _, _) => {
            h1342
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1343, _, _, _) => {
            h1343
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1344, _, _) => {
            h1344
        }
    })
}

pub fn f50(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1345 = v1;
    let a1346 = 0u64;
    if if nat_eq(a1345, a1346) {
        true
    } else {
        let a1347 = v2;
        let a1348 = 0u64;
        nat_eq(a1347, a1348)
    } {
        Ok(0u64)
    } else {
        let a1355 = {
            let a1349 = f101(v1, v2, v3)?;
            let a1350 = f102(v0, v1, v2)?;
            nat_add(a1349, a1350)?
        };
        let a1356 = {
            let a1353 = f103(v0);
            let a1354 = {
                let a1351 = v1;
                let a1352 = v2;
                nat_mul(a1351, a1352)?
            };
            nat_mul(a1353, a1354)?
        };
        nat_add(a1355, a1356)
    }
}

pub fn f51(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1357 = v0;
    let a1358 = 0u64;
    if if nat_eq(a1357, a1358) {
        true
    } else {
        let a1359 = v1;
        let a1360 = 0u64;
        nat_eq(a1359, a1360)
    } {
        Ok(3u64)
    } else {
        let a1363 = {
            let a1361 = 3u64;
            let a1362 = v1;
            nat_add(a1361, a1362)?
        };
        let a1364 = v2;
        nat_add(a1363, a1364)
    }
}

pub fn f52(v0: u64) -> bool {
    let a1365 = v0;
    let a1366 = 2139095040u64;
    if nat_le(a1365, a1366) {
        true
    } else {
        let a1367 = v0;
        let a1368 = 2147483648u64;
        nat_eq(a1367, a1368)
    }
}

pub fn f53(v0: u64, v1: u64) -> R<u64> {
    let a1369 = v0;
    let a1370 = 0u64;
    if nat_eq(a1369, a1370) {
        Ok(0u64)
    } else {
        let a1371 = v1;
        let a1372 = f104(v0)?;
        nat_mul(a1371, a1372)
    }
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1373 = v0;
    let a1374 = 0u64;
    if if if if nat_eq(a1373, a1374) {
        true
    } else {
        let a1375 = v1;
        let a1376 = 0u64;
        nat_eq(a1375, a1376)
    } {
        true
    } else {
        let a1377 = v2;
        let a1378 = 0u64;
        nat_eq(a1377, a1378)
    } {
        true
    } else {
        let a1379 = v3;
        let a1380 = 0u64;
        nat_eq(a1379, a1380)
    } {
        Ok(0u64)
    } else {
        let a1393 = {
            let a1383 = 2u64;
            let a1384 = {
                let a1381 = v1;
                let a1382 = v2;
                nat_mul(a1381, a1382)?
            };
            nat_mul(a1383, a1384)?
        };
        let a1394 = {
            let a1391 = {
                let a1387 = {
                    let a1385 = v1;
                    let a1386 = v2;
                    nat_mul(a1385, a1386)?
                };
                let a1388 = v3;
                nat_mul(a1387, a1388)?
            };
            let a1392 = {
                let a1389 = 1u64;
                let a1390 = v0;
                nat_add(a1389, a1390)?
            };
            nat_mul(a1391, a1392)?
        };
        nat_add(a1393, a1394)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1395 = v1;
        let a1396 = 0u64;
        if if if nat_eq(a1395, a1396) {
            true
        } else {
            let a1397 = v1;
            let a1398 = 10u64;
            nat_eq(a1397, a1398)
        } {
            true
        } else {
            let a1399 = v1;
            let a1400 = 15u64;
            nat_eq(a1399, a1400)
        } {
            true
        } else {
            let a1401 = v1;
            let a1402 = 20u64;
            nat_eq(a1401, a1402)
        }
    };
    let v4: bool = {
        let a1403 = v2;
        let a1404 = 0u64;
        if if if if if nat_eq(a1403, a1404) {
            true
        } else {
            let a1405 = v2;
            let a1406 = 10u64;
            nat_eq(a1405, a1406)
        } {
            true
        } else {
            let a1407 = v2;
            let a1408 = 15u64;
            nat_eq(a1407, a1408)
        } {
            true
        } else {
            let a1409 = v2;
            let a1410 = 20u64;
            nat_eq(a1409, a1410)
        } {
            true
        } else {
            let a1411 = v2;
            let a1412 = 30u64;
            nat_eq(a1411, a1412)
        } {
            true
        } else {
            let a1413 = v2;
            let a1414 = 40u64;
            nat_eq(a1413, a1414)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1415 = v0;
            let a1416 = 30u64;
            if if nat_eq(a1415, a1416) {
                let a1417 = v2;
                let a1418 = 10u64;
                nat_eq(a1417, a1418)
            } else {
                false
            } {
                true
            } else {
                if f105(v0) {
                    let a1419 = v2;
                    let a1420 = 10u64;
                    if if nat_eq(a1419, a1420) {
                        true
                    } else {
                        let a1421 = v2;
                        let a1422 = 15u64;
                        nat_eq(a1421, a1422)
                    } {
                        true
                    } else {
                        let a1423 = v2;
                        let a1424 = 20u64;
                        nat_eq(a1423, a1424)
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
        Adt1::C0(h1425, _, _, _, _, _, _, _, _, _, _, _) => {
            h1425
        }
    };
    Ok(Adt9::C0({
        let a1426 = v0;
        let a1427 = 59u64;
        if nat_eq(a1426, a1427) {
            let m1428 = v3;
            match m1428 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f106(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1429 = v0;
            let a1430 = 50u64;
            if nat_eq(a1429, a1430) {
                let m1431 = v3;
                match m1431 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f107(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1432 = v0;
                let a1433 = 82u64;
                if nat_eq(a1432, a1433) {
                    let m1434 = v3;
                    match m1434 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f108(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1435 = v0;
                    let a1436 = 86u64;
                    if nat_eq(a1435, a1436) {
                        let m1437 = v3;
                        match m1437 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f109(v1, v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1438 = v0;
                        let a1439 = 55u64;
                        if nat_eq(a1438, a1439) {
                            let m1440 = v3;
                            match m1440 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f110(v1, v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1441 = v0;
                            let a1442 = 22u64;
                            if nat_eq(a1441, a1442) {
                                let m1443 = v3;
                                match m1443 {
                                    None => {
                                        Adt10::C1
                                    }
                                    Some(v11) => {
                                        f111(v2, v11, v4)?
                                    }
                                }
                            } else {
                                let a1444 = v0;
                                let a1445 = 44u64;
                                if nat_eq(a1444, a1445) {
                                    f112(v1, v2, v3, v4)?
                                } else {
                                    let a1446 = v0;
                                    let a1447 = 52u64;
                                    if nat_eq(a1446, a1447) {
                                        f113(v1, v2, v4)?
                                    } else {
                                        let a1448 = v0;
                                        let a1449 = 73u64;
                                        if nat_eq(a1448, a1449) {
                                            f114(v2, v3, v4)?
                                        } else {
                                            let a1450 = v0;
                                            let a1451 = 57u64;
                                            if if nat_eq(a1450, a1451) {
                                                true
                                            } else {
                                                let a1452 = v0;
                                                let a1453 = 58u64;
                                                nat_eq(a1452, a1453)
                                            } {
                                                let m1454 = v3;
                                                match m1454 {
                                                    None => {
                                                        f115(false, {
                                                            let a1455 = v0;
                                                            let a1456 = 58u64;
                                                            nat_eq(a1455, a1456)
                                                        }, v1, v2, v4)?
                                                    }
                                                    Some(_) => {
                                                        Adt10::C1
                                                    }
                                                }
                                            } else {
                                                let a1457 = v0;
                                                let a1458 = 49u64;
                                                if nat_eq(a1457, a1458) {
                                                    f116(false, v1, v2, v3, v4)?
                                                } else {
                                                    let a1459 = v0;
                                                    let a1460 = 51u64;
                                                    if nat_eq(a1459, a1460) {
                                                        f117(v1, v2, v3, v4)?
                                                    } else {
                                                        let a1461 = v0;
                                                        let a1462 = 39u64;
                                                        if nat_eq(a1461, a1462) {
                                                            f118(v2, v3, v4)?
                                                        } else {
                                                            let a1463 = v0;
                                                            let a1464 = 77u64;
                                                            if nat_eq(a1463, a1464) {
                                                                f119(v2, v4)?
                                                            } else {
                                                                let a1465 = v0;
                                                                let a1466 = 40u64;
                                                                if nat_eq(a1465, a1466) {
                                                                    f120(v2, v3, v4)?
                                                                } else {
                                                                    if f121(v0) {
                                                                        let a1467 = v0;
                                                                        let a1468 = 33u64;
                                                                        if if if nat_eq(a1467, a1468) {
                                                                            let a1470 = match v2.clone() {
                                                                                Adt2::C0(h1469, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    h1469
                                                                                }
                                                                            };
                                                                            let a1471 = 0u64;
                                                                            nat_eq(a1470, a1471)
                                                                        } else {
                                                                            false
                                                                        } {
                                                                            let a1473 = match v4.clone() {
                                                                                Adt2::C0(h1472, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    h1472
                                                                                }
                                                                            };
                                                                            let a1474 = 26u64;
                                                                            nat_eq(a1473, a1474)
                                                                        } else {
                                                                            false
                                                                        } {
                                                                            let m1475 = v3;
                                                                            match m1475 {
                                                                                None => {
                                                                                    Adt10::C1
                                                                                }
                                                                                Some(v13) => {
                                                                                    f122(v2, v13, v4)?
                                                                                }
                                                                            }
                                                                        } else {
                                                                            let a1476 = v0;
                                                                            let a1477 = 33u64;
                                                                            if if if nat_eq(a1476, a1477) {
                                                                                let a1479 = match v4.clone() {
                                                                                    Adt2::C0(h1478, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1478
                                                                                    }
                                                                                };
                                                                                let a1480 = 0u64;
                                                                                nat_eq(a1479, a1480)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                f123(match v2.clone() {
                                                                                    Adt2::C0(h1481, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1481
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                f124(v2, v3, v4)?
                                                                            } else {
                                                                                f125(v0, v2, {
                                                                                    let m1482 = v3;
                                                                                    match m1482 {
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
                                                                        let a1483 = v0;
                                                                        let a1484 = 19u64;
                                                                        if nat_eq(a1483, a1484) {
                                                                            f126(v2, v4)?
                                                                        } else {
                                                                            let a1485 = v0;
                                                                            let a1486 = 21u64;
                                                                            if nat_eq(a1485, a1486) {
                                                                                f127(v5, v2, v3, v4)?
                                                                            } else {
                                                                                let a1487 = v0;
                                                                                let a1488 = 61u64;
                                                                                if if if nat_eq(a1487, a1488) {
                                                                                    true
                                                                                } else {
                                                                                    let a1489 = v0;
                                                                                    let a1490 = 62u64;
                                                                                    nat_eq(a1489, a1490)
                                                                                } {
                                                                                    true
                                                                                } else {
                                                                                    let a1491 = v0;
                                                                                    let a1492 = 63u64;
                                                                                    nat_eq(a1491, a1492)
                                                                                } {
                                                                                    f128(v0, v1, v2, v4)?
                                                                                } else {
                                                                                    let a1493 = v0;
                                                                                    let a1494 = 4u64;
                                                                                    if if nat_eq(a1493, a1494) {
                                                                                        true
                                                                                    } else {
                                                                                        let a1495 = v0;
                                                                                        let a1496 = 32u64;
                                                                                        nat_eq(a1495, a1496)
                                                                                    } {
                                                                                        f129(v0, v1, v2, v3, v4)?
                                                                                    } else {
                                                                                        let a1497 = v0;
                                                                                        let a1498 = 20u64;
                                                                                        if nat_eq(a1497, a1498) {
                                                                                            f130(v2, v4)?
                                                                                        } else {
                                                                                            let a1499 = v0;
                                                                                            let a1500 = 42u64;
                                                                                            if nat_eq(a1499, a1500) {
                                                                                                f131(4u64, v2, v4)?
                                                                                            } else {
                                                                                                let a1501 = v0;
                                                                                                let a1502 = 69u64;
                                                                                                if nat_eq(a1501, a1502) {
                                                                                                    let a1503 = v5;
                                                                                                    let a1504 = 4u64;
                                                                                                    if nat_lt(a1503, a1504) {
                                                                                                        f131(v5, v2, v4)?
                                                                                                    } else {
                                                                                                        Adt10::C1
                                                                                                    }
                                                                                                } else {
                                                                                                    let a1505 = v0;
                                                                                                    let a1506 = 70u64;
                                                                                                    if nat_eq(a1505, a1506) {
                                                                                                        f132(v5, v2, v4)?
                                                                                                    } else {
                                                                                                        let a1507 = v0;
                                                                                                        let a1508 = 75u64;
                                                                                                        if if nat_eq(a1507, a1508) {
                                                                                                            true
                                                                                                        } else {
                                                                                                            let a1509 = v0;
                                                                                                            let a1510 = 76u64;
                                                                                                            nat_eq(a1509, a1510)
                                                                                                        } {
                                                                                                            f133({
                                                                                                                let a1511 = v0;
                                                                                                                let a1512 = 76u64;
                                                                                                                nat_eq(a1511, a1512)
                                                                                                            }, v1, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1513 = v0;
                                                                                                            let a1514 = 31u64;
                                                                                                            if if if nat_eq(a1513, a1514) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1515 = v0;
                                                                                                                let a1516 = 47u64;
                                                                                                                nat_eq(a1515, a1516)
                                                                                                            } {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1517 = v0;
                                                                                                                let a1518 = 68u64;
                                                                                                                nat_eq(a1517, a1518)
                                                                                                            } {
                                                                                                                f134(v0, v1, v2, v4)?
                                                                                                            } else {
                                                                                                                let a1519 = v0;
                                                                                                                let a1520 = 15u64;
                                                                                                                if nat_eq(a1519, a1520) {
                                                                                                                    f135(v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1521 = v0;
                                                                                                                    let a1522 = 17u64;
                                                                                                                    if nat_eq(a1521, a1522) {
                                                                                                                        f136(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1523 = v0;
                                                                                                                        let a1524 = 18u64;
                                                                                                                        if nat_eq(a1523, a1524) {
                                                                                                                            f137(v2, v3, v4)?
                                                                                                                        } else {
                                                                                                                            let a1525 = v0;
                                                                                                                            let a1526 = 13u64;
                                                                                                                            if if if nat_eq(a1525, a1526) {
                                                                                                                                true
                                                                                                                            } else {
                                                                                                                                let a1527 = v0;
                                                                                                                                let a1528 = 14u64;
                                                                                                                                nat_eq(a1527, a1528)
                                                                                                                            } {
                                                                                                                                true
                                                                                                                            } else {
                                                                                                                                let a1529 = v0;
                                                                                                                                let a1530 = 16u64;
                                                                                                                                nat_eq(a1529, a1530)
                                                                                                                            } {
                                                                                                                                f138(v0, v2, v4)?
                                                                                                                            } else {
                                                                                                                                let m1531 = f139(v0, v5)?;
                                                                                                                                match m1531 {
                                                                                                                                    None => {
                                                                                                                                        f140(v0, v5, v2, v3, v4)?
                                                                                                                                    }
                                                                                                                                    Some(v15) => {
                                                                                                                                        f141(v15, v2, v4)?
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    f142(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1532, _, _, _) => {
            h1532
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1533, _, _) => {
            h1533
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1534, _) => {
            h1534
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1535) => {
            h1535
        }
    })
}

pub fn f58(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1537 = v0;
        let a1538 = match v1.clone() {
            Adt3::C0(h1536, _, _, _, _, _, _, _) => {
                h1536
            }
        };
        let a1539 = 0u64;
        nat_quot(a1537, a1538, a1539)
    };
    let v4: u64 = {
        let a1541 = v3;
        let a1542 = match v1.clone() {
            Adt3::C0(_, h1540, _, _, _, _, _, _) => {
                h1540
            }
        };
        let a1543 = 0u64;
        nat_quot(a1541, a1542, a1543)
    };
    let a1544 = v2;
    let a1545 = 0u64;
    if nat_eq(a1544, a1545) {
        let a1547 = v0;
        let a1548 = match v1 {
            Adt3::C0(h1546, _, _, _, _, _, _, _) => {
                h1546
            }
        };
        let a1549 = 0u64;
        nat_rem(a1547, a1548, a1549)
    } else {
        let a1550 = v2;
        let a1551 = 1u64;
        if nat_eq(a1550, a1551) {
            let a1553 = v3;
            let a1554 = match v1 {
                Adt3::C0(_, h1552, _, _, _, _, _, _) => {
                    h1552
                }
            };
            let a1555 = 0u64;
            nat_rem(a1553, a1554, a1555)
        } else {
            let a1556 = v2;
            let a1557 = 2u64;
            if nat_eq(a1556, a1557) {
                let a1559 = v4;
                let a1560 = match v1 {
                    Adt3::C0(_, _, h1558, _, _, _, _, _) => {
                        h1558
                    }
                };
                let a1561 = 0u64;
                nat_rem(a1559, a1560, a1561)
            } else {
                let a1563 = v4;
                let a1564 = match v1 {
                    Adt3::C0(_, _, h1562, _, _, _, _, _) => {
                        h1562
                    }
                };
                let a1565 = 0u64;
                nat_quot(a1563, a1564, a1565)
            }
        }
    }
}

pub fn f59(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1567 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1566) => {
            h1566
        }
    };
    let a1568 = 0u64;
    if nat_eq(a1567, a1568) {
        Ok(v2)
    } else {
        let a1573 = {
            let a1570 = v2;
            let a1571 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1569) => {
                    h1569
                }
            };
            nat_mul(a1570, a1571)?
        };
        let a1574 = f58(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1572, _, _, _, _, _, _) => {
                h1572
            }
        }, 3u64);
        nat_add(a1573, a1574)
    }
}

pub fn f60(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1584 = {
            let a1581 = {
                let a1576 = v2;
                let a1577 = match v0.clone() {
                    Adt6::C0(_, h1575, _, _) => {
                        h1575
                    }
                };
                nat_mul(a1576, a1577)?
            };
            let a1582 = {
                let a1579 = v3;
                let a1580 = match v0.clone() {
                    Adt6::C0(_, _, _, h1578) => {
                        h1578
                    }
                };
                nat_mul(a1579, a1580)?
            };
            nat_add(a1581, a1582)?
        };
        let a1585 = f143(match v0.clone() {
            Adt6::C0(_, _, h1583, _) => {
                h1583
            }
        });
        nat_add(a1584, a1585)?
    };
    let v5: u64 = f144(match v0 {
        Adt6::C0(_, _, h1586, _) => {
            h1586
        }
    });
    let a1587 = v5;
    let a1588 = v4;
    if if nat_le(a1587, a1588) {
        let a1591 = {
            let a1589 = v4;
            let a1590 = v5;
            nat_sub(a1589, a1590)
        };
        let a1592 = v1;
        nat_lt(a1591, a1592)
    } else {
        false
    } {
        Ok(Some({
            let a1593 = v4;
            let a1594 = v5;
            nat_sub(a1593, a1594)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f61(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1595, _, _, _, _, _) => {
            h1595
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1596, _, _, _, _, _, _) => {
            h1596
        }
    };
    let v7: u64 = f58(v1, v6.clone(), 2u64);
    let a1598 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1597) => {
            h1597
        }
    };
    let a1599 = 0u64;
    if nat_eq(a1598, a1599) {
        Ok(Adt9::C8(Some(f57(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1602 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1601, _, _, _, _) => {
                    h1601
                }
            };
            match m1602 {
                Adt3::C0(_, _, h1600, _, _, _, _, _) => {
                    h1600
                }
            }
        };
        let m1608 = f60(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1603, _) => {
                h1603
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1604, _, _, _, _, _) => {
                h1604
            }
        }, {
            let a1605 = v2;
            let a1606 = v8;
            let a1607 = 0u64;
            nat_rem(a1605, a1606, a1607)
        }, v7)?;
        match m1608 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1615 = {
                        let a1613 = {
                            let a1609 = v2;
                            let a1610 = v8;
                            let a1611 = 0u64;
                            nat_quot(a1609, a1610, a1611)
                        };
                        let a1614 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1612) => {
                                h1612
                            }
                        };
                        nat_mul(a1613, a1614)?
                    };
                    let a1616 = f58(v1, v6, 3u64);
                    nat_add(a1615, a1616)?
                };
                Ok(Adt9::C8(Some(f57(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f62(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1617 = v1;
    match m1617 {
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
            f145(f146(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f147(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f148(v0.clone(), v24);
            f145(f149(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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
    let m1618 = v3;
    match m1618 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f92(v0, v1)?, f92(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f151(v0, v1, v2.clone())?, f92(v0, v2)?))
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
            Ok(Adt9::C1(true, 0u64, f92(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f158(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v55, v56) => {
            f159(v0, v1, v2, v55, v56)
        }
        Adt4::C12(v57, v58, v59) => {
            f160(v0, v1, v2, v57, v58, v59)
        }
        Adt4::C13(_, v61, _, _) => {
            Ok(Adt9::C3(v61, v61, f92(v0, v1)?, f92(v0, v2)?))
        }
        Adt4::C14(v64, v65, v66) => {
            let a1621 = v64;
            let a1622 = 23u64;
            if nat_eq(a1621, a1622) {
                f161(v0, v1, v2, v65, v66)
            } else {
                Ok(Adt9::C3(v65, v66, f92(v0, v1)?, f92(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f162(v0, v1, v2)
        }
        Adt4::C16(v67, v68) => {
            f163(v0, v1, v2, v67, v68)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v69, v70) => {
            f164(v0, v1, v70, v69)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f92(f150(v0, match v2.clone() {
                Adt3::C0(h1619, _, _, _, _, _, _, _) => {
                    h1619
                }
            }, match v2.clone() {
                Adt3::C0(_, h1620, _, _, _, _, _, _) => {
                    h1620
                }
            })?, v1)?, f92(v0, v2)?))
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
        Adt4::C25(v75, v76, v77) => {
            f165(v0, v75, v76, v77)
        }
        Adt4::C26(v83, _, _, _) => {
            let a1623 = v83;
            let a1624 = 64u64;
            if if if nat_eq(a1623, a1624) {
                true
            } else {
                let a1625 = v83;
                let a1626 = 65u64;
                nat_eq(a1625, a1626)
            } {
                true
            } else {
                let a1627 = v83;
                let a1628 = 66u64;
                nat_eq(a1627, a1628)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v82) => {
            f84(v0, v1, v2, v82)
        }
        Adt4::C28(v78) => {
            f81(v0, v1, v2, v78, 0u64)
        }
        Adt4::C29(v79) => {
            f82(v0, v1, v2, v79)
        }
        Adt4::C30(v80, _) => {
            f83(v0, v1, v2, v80)
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
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f166(v3, v4);
    let v6: u64 = {
        let a1629 = v0;
        let a1630 = v5;
        let a1631 = 0u64;
        nat_quot(a1629, a1630, a1631)
    };
    let a1632 = v5;
    let a1633 = 1u64;
    if nat_eq(a1632, a1633) {
        Ok(f145(f167(v3, v4), v3, v4, 0u64, f92(v6, v1)?, 2u64, 0u64, 3u64, f92(v6, v2)?))
    } else {
        let a1637 = {
            let a1634 = v0;
            let a1635 = 2u64;
            let a1636 = 0u64;
            nat_rem(a1634, a1635, a1636)
        };
        let a1638 = 0u64;
        if nat_eq(a1637, a1638) {
            Ok(f145(f167(v3, 0u64), v3, 0u64, 0u64, f92(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f145(f167(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f92(v6, v2)?))
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f168(v1)?;
    let v4: u64 = {
        let a1639 = v0;
        let a1640 = v3;
        let a1641 = 0u64;
        nat_rem(a1639, a1640, a1641)
    };
    let v5: u64 = {
        let a1644 = {
            let a1642 = v1;
            let a1643 = 3u64;
            nat_mul(a1642, a1643)?
        };
        let a1645 = 1u64;
        nat_add(a1644, a1645)?
    };
    let a1646 = v4;
    let a1647 = 0u64;
    if nat_eq(a1646, a1647) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1648 = v4;
        let a1649 = v5;
        if nat_eq(a1648, a1649) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1654 = v4;
                let a1655 = {
                    let a1650 = v4;
                    let a1651 = v5;
                    if nat_lt(a1650, a1651) {
                        1u64
                    } else {
                        let a1652 = v5;
                        let a1653 = 1u64;
                        nat_add(a1652, a1653)?
                    }
                };
                nat_sub(a1654, a1655)
            };
            let v7: u64 = {
                let a1656 = v6;
                let a1657 = 3u64;
                let a1658 = 0u64;
                nat_rem(a1656, a1657, a1658)
            };
            let v8: u64 = {
                let a1669 = 4u64;
                let a1670 = {
                    let a1667 = {
                        let a1662 = {
                            let a1659 = v0;
                            let a1660 = v3;
                            let a1661 = 0u64;
                            nat_quot(a1659, a1660, a1661)
                        };
                        let a1663 = v1;
                        nat_mul(a1662, a1663)?
                    };
                    let a1668 = {
                        let a1664 = v6;
                        let a1665 = 3u64;
                        let a1666 = 0u64;
                        nat_quot(a1664, a1665, a1666)
                    };
                    nat_add(a1667, a1668)?
                };
                nat_mul(a1669, a1670)?
            };
            let a1671 = v4;
            let a1672 = v5;
            if nat_lt(a1671, a1672) {
                let a1673 = v7;
                let a1674 = 0u64;
                if nat_eq(a1673, a1674) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1675 = v7;
                    let a1676 = 1u64;
                    if nat_eq(a1675, a1676) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1677 = v7;
                let a1678 = 0u64;
                if nat_eq(a1677, a1678) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1679 = v7;
                    let a1680 = 1u64;
                    if nat_eq(a1679, a1680) {
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
        let a1681 = v0;
        let a1682 = 6u64;
        let a1683 = 0u64;
        nat_quot(a1681, a1682, a1683)
    };
    let v5: u64 = {
        let a1684 = v0;
        let a1685 = 6u64;
        let a1686 = 0u64;
        nat_rem(a1684, a1685, a1686)
    };
    let a1687 = v5;
    let a1688 = 0u64;
    if nat_eq(a1687, a1688) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f92(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1689 = v5;
        let a1690 = 1u64;
        if nat_eq(a1689, a1690) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1691 = v5;
            let a1692 = 2u64;
            if nat_eq(a1691, a1692) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f92(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1693 = v5;
                let a1694 = 3u64;
                if nat_eq(a1693, a1694) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1695 = v5;
                    let a1696 = 4u64;
                    if nat_eq(a1695, a1696) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f92(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f92(v4, v3)?), 4u64, 4u64))
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
    let v7: u64 = f169(match v1.clone() {
        Adt3::C0(_, h1697, _, _, _, _, _, _) => {
            h1697
        }
    }, v4)?;
    let v8: u64 = {
        let a1698 = v0;
        let a1699 = v7;
        let a1700 = 0u64;
        nat_rem(a1698, a1699, a1700)
    };
    let v9: u64 = {
        let a1701 = v0;
        let a1702 = v7;
        let a1703 = 0u64;
        nat_quot(a1701, a1702, a1703)
    };
    let a1706 = {
        let a1704 = v8;
        let a1705 = 1u64;
        nat_add(a1704, a1705)?
    };
    let a1707 = v7;
    if nat_eq(a1706, a1707) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f92(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1708 = v8;
            let a1709 = v6;
            let a1710 = 0u64;
            nat_quot(a1708, a1709, a1710)
        };
        let v11: u64 = {
            let a1711 = v8;
            let a1712 = v6;
            let a1713 = 0u64;
            nat_rem(a1711, a1712, a1713)
        };
        let v12: u64 = {
            let a1715 = v9;
            let a1716 = match v1.clone() {
                Adt3::C0(h1714, _, _, _, _, _, _, _) => {
                    h1714
                }
            };
            let a1717 = 0u64;
            nat_rem(a1715, a1716, a1717)
        };
        let v13: u64 = {
            let a1719 = v9;
            let a1720 = match v1.clone() {
                Adt3::C0(h1718, _, _, _, _, _, _, _) => {
                    h1718
                }
            };
            let a1721 = 0u64;
            nat_quot(a1719, a1720, a1721)
        };
        let v14: u64 = if v4 {
            let a1733 = {
                let a1728 = {
                    let a1723 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1722, _, _, _) => {
                            h1722
                        }
                    };
                    let a1724 = v12;
                    nat_mul(a1723, a1724)?
                };
                let a1729 = {
                    let a1726 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1725, _, _) => {
                            h1725
                        }
                    };
                    let a1727 = v10;
                    nat_mul(a1726, a1727)?
                };
                nat_add(a1728, a1729)?
            };
            let a1734 = {
                let a1731 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1730, _) => {
                        h1730
                    }
                };
                let a1732 = v13;
                nat_mul(a1731, a1732)?
            };
            nat_add(a1733, a1734)?
        } else {
            let a1741 = {
                let a1736 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1735, _, _, _) => {
                        h1735
                    }
                };
                let a1737 = v10;
                nat_mul(a1736, a1737)?
            };
            let a1742 = {
                let a1739 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1738, _, _) => {
                        h1738
                    }
                };
                let a1740 = v13;
                nat_mul(a1739, a1740)?
            };
            nat_add(a1741, a1742)?
        };
        if if v4 {
            let a1743 = v11;
            let a1744 = 0u64;
            nat_eq(a1743, a1744)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1747 = {
                let a1745 = v11;
                let a1746 = 1u64;
                nat_add(a1745, a1746)?
            };
            let a1748 = v6;
            if nat_eq(a1747, a1748) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1749 = v10;
                    let a1750 = 0u64;
                    if nat_eq(a1749, a1750) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1762 = {
                        let a1757 = {
                            let a1752 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1751, _, _, _) => {
                                    h1751
                                }
                            };
                            let a1753 = v12;
                            nat_mul(a1752, a1753)?
                        };
                        let a1758 = {
                            let a1755 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1754, _, _) => {
                                    h1754
                                }
                            };
                            let a1756 = v10;
                            nat_mul(a1755, a1756)?
                        };
                        nat_add(a1757, a1758)?
                    };
                    let a1763 = {
                        let a1760 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1759, _) => {
                                h1759
                            }
                        };
                        let a1761 = v13;
                        nat_mul(a1760, a1761)?
                    };
                    nat_add(a1762, a1763)?
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
        Adt3::C0(_, h1764, _, _, _, _, _, _) => {
            h1764
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1765 = v0;
        let a1766 = v6;
        let a1767 = 0u64;
        nat_rem(a1765, a1766, a1767)
    };
    let v8: u64 = {
        let a1768 = v0;
        let a1769 = v6;
        let a1770 = 0u64;
        nat_quot(a1768, a1769, a1770)
    };
    let v9: u64 = {
        let a1772 = v8;
        let a1773 = match v5.clone() {
            Adt3::C0(h1771, _, _, _, _, _, _, _) => {
                h1771
            }
        };
        let a1774 = 0u64;
        nat_rem(a1772, a1773, a1774)
    };
    let v10: u64 = {
        let a1776 = v8;
        let a1777 = match v5.clone() {
            Adt3::C0(h1775, _, _, _, _, _, _, _) => {
                h1775
            }
        };
        let a1778 = 0u64;
        nat_quot(a1776, a1777, a1778)
    };
    let v11: u64 = {
        let a1780 = v10;
        let a1781 = match v2.clone() {
            Adt3::C0(_, h1779, _, _, _, _, _, _) => {
                h1779
            }
        };
        let a1782 = 0u64;
        nat_rem(a1780, a1781, a1782)
    };
    let v12: u64 = {
        let a1784 = v10;
        let a1785 = match v2.clone() {
            Adt3::C0(_, h1783, _, _, _, _, _, _) => {
                h1783
            }
        };
        let a1786 = 0u64;
        nat_quot(a1784, a1785, a1786)
    };
    let a1787 = v7;
    let a1788 = 0u64;
    if nat_eq(a1787, a1788) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1795 = {
                let a1790 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1789, _, _, _) => {
                        h1789
                    }
                };
                let a1791 = v9;
                nat_mul(a1790, a1791)?
            };
            let a1796 = {
                let a1793 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1792, _, _) => {
                        h1792
                    }
                };
                let a1794 = v12;
                nat_mul(a1793, a1794)?
            };
            nat_add(a1795, a1796)?
        }), Adt12::C0(2u64, {
            let a1803 = {
                let a1798 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1797, _, _, _) => {
                        h1797
                    }
                };
                let a1799 = v11;
                nat_mul(a1798, a1799)?
            };
            let a1804 = {
                let a1801 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1800, _, _) => {
                        h1800
                    }
                };
                let a1802 = v12;
                nat_mul(a1801, a1802)?
            };
            nat_add(a1803, a1804)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1805 = v4;
        match m1805 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1817 = {
                        let a1812 = {
                            let a1807 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1806, _, _, _) => {
                                    h1806
                                }
                            };
                            let a1808 = v9;
                            nat_mul(a1807, a1808)?
                        };
                        let a1813 = {
                            let a1810 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1809, _, _) => {
                                    h1809
                                }
                            };
                            let a1811 = v11;
                            nat_mul(a1810, a1811)?
                        };
                        nat_add(a1812, a1813)?
                    };
                    let a1818 = {
                        let a1815 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1814, _) => {
                                h1814
                            }
                        };
                        let a1816 = v12;
                        nat_mul(a1815, a1816)?
                    };
                    nat_add(a1817, a1818)?
                }), Adt13::C1(f92(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1819 = v7;
                    let a1820 = 1u64;
                    nat_sub(a1819, a1820)
                };
                let v15: u64 = {
                    let a1821 = v14;
                    let a1822 = 2u64;
                    let a1823 = 0u64;
                    nat_quot(a1821, a1822, a1823)
                };
                let a1827 = {
                    let a1824 = v14;
                    let a1825 = 2u64;
                    let a1826 = 0u64;
                    nat_rem(a1824, a1825, a1826)
                };
                let a1828 = 0u64;
                if nat_eq(a1827, a1828) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1840 = {
                            let a1835 = {
                                let a1830 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1829, _, _, _) => {
                                        h1829
                                    }
                                };
                                let a1831 = v9;
                                nat_mul(a1830, a1831)?
                            };
                            let a1836 = {
                                let a1833 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1832, _, _) => {
                                        h1832
                                    }
                                };
                                let a1834 = v15;
                                nat_mul(a1833, a1834)?
                            };
                            nat_add(a1835, a1836)?
                        };
                        let a1841 = {
                            let a1838 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1837, _) => {
                                    h1837
                                }
                            };
                            let a1839 = v12;
                            nat_mul(a1838, a1839)?
                        };
                        nat_add(a1840, a1841)?
                    }), Adt12::C0(3u64, {
                        let a1853 = {
                            let a1848 = {
                                let a1843 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1842, _, _, _) => {
                                        h1842
                                    }
                                };
                                let a1844 = v11;
                                nat_mul(a1843, a1844)?
                            };
                            let a1849 = {
                                let a1846 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1845, _, _) => {
                                        h1845
                                    }
                                };
                                let a1847 = v15;
                                nat_mul(a1846, a1847)?
                            };
                            nat_add(a1848, a1849)?
                        };
                        let a1854 = {
                            let a1851 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1850, _) => {
                                    h1850
                                }
                            };
                            let a1852 = v12;
                            nat_mul(a1851, a1852)?
                        };
                        nat_add(a1853, a1854)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1857 = {
                            let a1855 = v7;
                            let a1856 = 1u64;
                            nat_add(a1855, a1856)?
                        };
                        let a1858 = v6;
                        if nat_eq(a1857, a1858) {
                            Adt13::C1(f92(v8, v5)?)
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
        let a1859 = v0;
        let a1860 = v8;
        let a1861 = 0u64;
        nat_rem(a1859, a1860, a1861)
    };
    let v10: u64 = {
        let a1862 = v0;
        let a1863 = v8;
        let a1864 = 0u64;
        nat_quot(a1862, a1863, a1864)
    };
    let a1865 = v9;
    let a1866 = 148u64;
    if nat_lt(a1865, a1866) {
        f170({
            let a1867 = v9;
            let a1868 = 37u64;
            let a1869 = 0u64;
            nat_rem(a1867, a1868, a1869)
        }, {
            let a1870 = v9;
            let a1871 = 37u64;
            let a1872 = 0u64;
            nat_quot(a1870, a1871, a1872)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f171({
            let a1873 = v9;
            let a1874 = 148u64;
            nat_sub(a1873, a1874)
        }, v10, v4, v5)
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f28(v2);
    let a1875 = v0;
    let a1876 = v3;
    if nat_lt(a1875, a1876) {
        let a1877 = v0;
        let a1878 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1877, a1878) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1879 = 4u64;
                let a1880 = v0;
                nat_mul(a1879, a1880)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1881 = v0;
            let a1882 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1881, a1882) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1885 = v0;
                let a1886 = {
                    let a1883 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1884 = 1u64;
                    nat_add(a1883, a1884)?
                };
                if nat_eq(a1885, a1886) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1887 = v0;
                    let a1888 = 9u64;
                    if nat_eq(a1887, a1888) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1889 = v0;
            let a1890 = v3;
            nat_sub(a1889, a1890)
        };
        let v5: u64 = f29(v2);
        let v6: u64 = {
            let a1891 = v4;
            let a1892 = v5;
            let a1893 = 0u64;
            nat_rem(a1891, a1892, a1893)
        };
        let v7: u64 = f92({
            let a1894 = v4;
            let a1895 = v5;
            let a1896 = 0u64;
            nat_quot(a1894, a1895, a1896)
        }, v1)?;
        if v2 {
            let a1897 = v6;
            let a1898 = 7u64;
            if nat_lt(a1897, a1898) {
                let a1899 = v6;
                let a1900 = 3u64;
                if nat_lt(a1899, a1900) {
                    let a1901 = v6;
                    let a1902 = 1u64;
                    if nat_lt(a1901, a1902) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1903 = v6;
                        let a1904 = 2u64;
                        if nat_lt(a1903, a1904) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1905 = v6;
                    let a1906 = 5u64;
                    if nat_lt(a1905, a1906) {
                        let a1907 = v6;
                        let a1908 = 4u64;
                        if nat_lt(a1907, a1908) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1909 = v6;
                        let a1910 = 6u64;
                        if nat_lt(a1909, a1910) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a1911 = v6;
                let a1912 = 11u64;
                if nat_lt(a1911, a1912) {
                    let a1913 = v6;
                    let a1914 = 9u64;
                    if nat_lt(a1913, a1914) {
                        let a1915 = v6;
                        let a1916 = 8u64;
                        if nat_lt(a1915, a1916) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a1917 = v6;
                        let a1918 = 10u64;
                        if nat_lt(a1917, a1918) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1919 = v6;
                    let a1920 = 13u64;
                    if nat_lt(a1919, a1920) {
                        let a1921 = v6;
                        let a1922 = 12u64;
                        if nat_lt(a1921, a1922) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a1923 = v6;
                        let a1924 = 14u64;
                        if nat_lt(a1923, a1924) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a1925 = v6;
            let a1926 = 1u64;
            if nat_lt(a1925, a1926) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a1927 = v6;
                let a1928 = 2u64;
                if nat_lt(a1927, a1928) {
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
        Adt3::C0(h1929, _, _, _, _, _, _, _) => {
            h1929
        }
    };
    let v8: u64 = f24(v3, v7)?;
    let v9: u64 = {
        let a1930 = v0;
        let a1931 = 2u64;
        nat_sub(a1930, a1931)
    };
    let v10: u64 = {
        let a1932 = v9;
        let a1933 = v8;
        let a1934 = 0u64;
        nat_quot(a1932, a1933, a1934)
    };
    let v11: u64 = {
        let a1935 = v9;
        let a1936 = v8;
        let a1937 = 0u64;
        nat_rem(a1935, a1936, a1937)
    };
    let a1938 = v0;
    let a1939 = 0u64;
    if nat_eq(a1938, a1939) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a1940 = v0;
        let a1941 = 1u64;
        if nat_eq(a1940, a1941) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a1942 = v10;
            let a1943 = v5;
            if nat_eq(a1942, a1943) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a1946 = v11;
                let a1947 = {
                    let a1944 = 2u64;
                    let a1945 = v7;
                    nat_mul(a1944, a1945)?
                };
                if nat_lt(a1946, a1947) {
                    let v12: u64 = {
                        let a1948 = v11;
                        let a1949 = 2u64;
                        let a1950 = 0u64;
                        nat_quot(a1948, a1949, a1950)
                    };
                    let a1954 = {
                        let a1951 = v11;
                        let a1952 = 2u64;
                        let a1953 = 0u64;
                        nat_rem(a1951, a1952, a1953)
                    };
                    let a1955 = 0u64;
                    if nat_eq(a1954, a1955) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a1956 = v12;
                            let a1957 = 0u64;
                            if nat_eq(a1956, a1957) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a1963 = {
                                let a1959 = v10;
                                let a1960 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h1958, _, _) => {
                                        h1958
                                    }
                                };
                                nat_mul(a1959, a1960)?
                            };
                            let a1964 = {
                                let a1961 = 4u64;
                                let a1962 = v12;
                                nat_mul(a1961, a1962)?
                            };
                            nat_add(a1963, a1964)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a1965 = v6;
                            let a1966 = 2u64;
                            if nat_eq(a1965, a1966) {
                                let a1967 = v12;
                                let a1968 = 0u64;
                                if nat_eq(a1967, a1968) {
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
                                    let a1974 = {
                                        let a1970 = v10;
                                        let a1971 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1969, _, _) => {
                                                h1969
                                            }
                                        };
                                        nat_mul(a1970, a1971)?
                                    };
                                    let a1975 = {
                                        let a1972 = 4u64;
                                        let a1973 = v12;
                                        nat_mul(a1972, a1973)?
                                    };
                                    nat_add(a1974, a1975)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a1978 = v11;
                    let a1979 = {
                        let a1976 = 6u64;
                        let a1977 = v7;
                        nat_mul(a1976, a1977)?
                    };
                    if nat_lt(a1978, a1979) {
                        let v13: u64 = {
                            let a1982 = v11;
                            let a1983 = {
                                let a1980 = 2u64;
                                let a1981 = v7;
                                nat_mul(a1980, a1981)?
                            };
                            nat_sub(a1982, a1983)
                        };
                        let v14: u64 = {
                            let a1984 = v13;
                            let a1985 = 4u64;
                            let a1986 = 0u64;
                            nat_quot(a1984, a1985, a1986)
                        };
                        let a1990 = {
                            let a1987 = v13;
                            let a1988 = 4u64;
                            let a1989 = 0u64;
                            nat_rem(a1987, a1988, a1989)
                        };
                        let a1991 = 2u64;
                        if nat_lt(a1990, a1991) {
                            let a1995 = {
                                let a1992 = v13;
                                let a1993 = 4u64;
                                let a1994 = 0u64;
                                nat_rem(a1992, a1993, a1994)
                            };
                            let a1996 = 1u64;
                            if nat_lt(a1995, a1996) {
                                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                    1u64
                                } else {
                                    0u64
                                }, {
                                    let a2002 = {
                                        let a1998 = v10;
                                        let a1999 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h1997, _, _) => {
                                                h1997
                                            }
                                        };
                                        nat_mul(a1998, a1999)?
                                    };
                                    let a2003 = {
                                        let a2000 = 4u64;
                                        let a2001 = v14;
                                        nat_mul(a2000, a2001)?
                                    };
                                    nat_add(a2002, a2003)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2009 = {
                                            let a2005 = v10;
                                            let a2006 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2004, _, _) => {
                                                    h2004
                                                }
                                            };
                                            nat_mul(a2005, a2006)?
                                        };
                                        let a2010 = {
                                            let a2007 = 4u64;
                                            let a2008 = v14;
                                            nat_mul(a2007, a2008)?
                                        };
                                        nat_add(a2009, a2010)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2014 = {
                                let a2011 = v13;
                                let a2012 = 4u64;
                                let a2013 = 0u64;
                                nat_rem(a2011, a2012, a2013)
                            };
                            let a2015 = 3u64;
                            if nat_lt(a2014, a2015) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2021 = {
                                            let a2017 = v10;
                                            let a2018 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2016, _, _) => {
                                                    h2016
                                                }
                                            };
                                            nat_mul(a2017, a2018)?
                                        };
                                        let a2022 = {
                                            let a2019 = 4u64;
                                            let a2020 = v14;
                                            nat_mul(a2019, a2020)?
                                        };
                                        nat_add(a2021, a2022)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2023 = v14;
                                    let a2024 = 0u64;
                                    if nat_eq(a2023, a2024) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
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
                        if nat_lt(a2029, a2030) {
                            let a2033 = v11;
                            let a2034 = {
                                let a2031 = 6u64;
                                let a2032 = v7;
                                nat_mul(a2031, a2032)?
                            };
                            if nat_eq(a2033, a2034) {
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
                                    let a2039 = v11;
                                    let a2040 = {
                                        let a2037 = {
                                            let a2035 = 6u64;
                                            let a2036 = v7;
                                            nat_mul(a2035, a2036)?
                                        };
                                        let a2038 = 2u64;
                                        nat_add(a2037, a2038)?
                                    };
                                    nat_sub(a2039, a2040)
                                };
                                let v16: u64 = {
                                    let a2041 = v15;
                                    let a2042 = 3u64;
                                    let a2043 = 0u64;
                                    nat_quot(a2041, a2042, a2043)
                                };
                                let a2047 = {
                                    let a2044 = v15;
                                    let a2045 = 3u64;
                                    let a2046 = 0u64;
                                    nat_rem(a2044, a2045, a2046)
                                };
                                let a2048 = 1u64;
                                if nat_lt(a2047, a2048) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2054 = {
                                            let a2050 = v10;
                                            let a2051 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2049, _, _) => {
                                                    h2049
                                                }
                                            };
                                            nat_mul(a2050, a2051)?
                                        };
                                        let a2055 = {
                                            let a2052 = 4u64;
                                            let a2053 = v16;
                                            nat_mul(a2052, a2053)?
                                        };
                                        nat_add(a2054, a2055)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2059 = {
                                        let a2056 = v15;
                                        let a2057 = 3u64;
                                        let a2058 = 0u64;
                                        nat_rem(a2056, a2057, a2058)
                                    };
                                    let a2060 = 2u64;
                                    if nat_lt(a2059, a2060) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2065 = {
                                                let a2061 = v10;
                                                let a2062 = v4;
                                                nat_mul(a2061, a2062)?
                                            };
                                            let a2066 = {
                                                let a2063 = 4u64;
                                                let a2064 = v16;
                                                nat_mul(a2063, a2064)?
                                            };
                                            nat_add(a2065, a2066)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2072 = {
                                                let a2068 = v10;
                                                let a2069 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2067, _, _) => {
                                                        h2067
                                                    }
                                                };
                                                nat_mul(a2068, a2069)?
                                            };
                                            let a2073 = {
                                                let a2070 = 4u64;
                                                let a2071 = v16;
                                                nat_mul(a2070, a2071)?
                                            };
                                            nat_add(a2072, a2073)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2078 = v11;
                                let a2079 = {
                                    let a2076 = {
                                        let a2074 = 11u64;
                                        let a2075 = v7;
                                        nat_mul(a2074, a2075)?
                                    };
                                    let a2077 = 2u64;
                                    nat_add(a2076, a2077)?
                                };
                                if nat_lt(a2078, a2079) {
                                    let v17: u64 = {
                                        let a2084 = v11;
                                        let a2085 = {
                                            let a2082 = {
                                                let a2080 = 6u64;
                                                let a2081 = v7;
                                                nat_mul(a2080, a2081)?
                                            };
                                            let a2083 = 2u64;
                                            nat_add(a2082, a2083)?
                                        };
                                        nat_sub(a2084, a2085)
                                    };
                                    let v18: u64 = {
                                        let a2086 = v17;
                                        let a2087 = 5u64;
                                        let a2088 = 0u64;
                                        nat_quot(a2086, a2087, a2088)
                                    };
                                    let a2092 = {
                                        let a2089 = v17;
                                        let a2090 = 5u64;
                                        let a2091 = 0u64;
                                        nat_rem(a2089, a2090, a2091)
                                    };
                                    let a2093 = 2u64;
                                    if nat_lt(a2092, a2093) {
                                        let a2097 = {
                                            let a2094 = v17;
                                            let a2095 = 5u64;
                                            let a2096 = 0u64;
                                            nat_rem(a2094, a2095, a2096)
                                        };
                                        let a2098 = 1u64;
                                        if nat_lt(a2097, a2098) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2104 = {
                                                    let a2100 = v10;
                                                    let a2101 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2099, _, _) => {
                                                            h2099
                                                        }
                                                    };
                                                    nat_mul(a2100, a2101)?
                                                };
                                                let a2105 = {
                                                    let a2102 = 4u64;
                                                    let a2103 = v18;
                                                    nat_mul(a2102, a2103)?
                                                };
                                                nat_add(a2104, a2105)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2109 = {
                                            let a2106 = v17;
                                            let a2107 = 5u64;
                                            let a2108 = 0u64;
                                            nat_rem(a2106, a2107, a2108)
                                        };
                                        let a2110 = 3u64;
                                        if nat_lt(a2109, a2110) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2115 = {
                                                    let a2111 = v10;
                                                    let a2112 = v4;
                                                    nat_mul(a2111, a2112)?
                                                };
                                                let a2116 = {
                                                    let a2113 = 4u64;
                                                    let a2114 = v18;
                                                    nat_mul(a2113, a2114)?
                                                };
                                                nat_add(a2115, a2116)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2120 = {
                                                let a2117 = v17;
                                                let a2118 = 5u64;
                                                let a2119 = 0u64;
                                                nat_rem(a2117, a2118, a2119)
                                            };
                                            let a2121 = 4u64;
                                            if nat_lt(a2120, a2121) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2122 = v18;
                                                    let a2123 = 0u64;
                                                    if nat_eq(a2122, a2123) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2128 = v11;
                                    let a2129 = {
                                        let a2126 = {
                                            let a2124 = 11u64;
                                            let a2125 = v7;
                                            nat_mul(a2124, a2125)?
                                        };
                                        let a2127 = 2u64;
                                        nat_add(a2126, a2127)?
                                    };
                                    if nat_eq(a2128, a2129) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2130 = v10;
                                            let a2131 = 0u64;
                                            if nat_eq(a2130, a2131) {
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

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64, v5: u64) -> R<Adt9> {
    let a2132 = v0;
    let a2133 = v4;
    if nat_lt(a2132, a2133) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f92(v0, v2)?), 4u64, 4u64))
    } else {
        let v6: u64 = {
            let a2140 = {
                let m2136 = match v3.clone() {
                    Adt7::C0(h2135, _, _, _, _) => {
                        h2135
                    }
                };
                match m2136 {
                    Adt6::C0(h2134, _, _, _) => {
                        h2134
                    }
                }
            };
            let a2141 = {
                let m2139 = match v3.clone() {
                    Adt7::C0(_, h2138, _, _, _) => {
                        h2138
                    }
                };
                match m2139 {
                    Adt6::C0(h2137, _, _, _) => {
                        h2137
                    }
                }
            };
            nat_mul(a2140, a2141)?
        };
        let v7: u64 = f172(match v3.clone() {
            Adt7::C0(_, _, _, h2142, _) => {
                h2142
            }
        }, v6)?;
        let v8: u64 = {
            let a2143 = v0;
            let a2144 = v4;
            nat_sub(a2143, a2144)
        };
        let v9: u64 = {
            let a2145 = v8;
            let a2146 = v7;
            let a2147 = 0u64;
            nat_quot(a2145, a2146, a2147)
        };
        let v10: u64 = {
            let a2148 = v8;
            let a2149 = v7;
            let a2150 = 0u64;
            nat_rem(a2148, a2149, a2150)
        };
        if match v3.clone() {
            Adt7::C0(_, _, _, h2151, _) => {
                h2151
            }
        } {
            let a2152 = v10;
            let a2153 = 0u64;
            if nat_eq(a2152, a2153) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(4286578687u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2154 = v10;
                let a2155 = 1u64;
                if nat_eq(a2154, a2155) {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2158 = {
                        let a2156 = v10;
                        let a2157 = 2u64;
                        nat_add(a2156, a2157)?
                    };
                    let a2159 = v7;
                    if nat_eq(a2158, a2159) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a2162 = {
                            let a2160 = v10;
                            let a2161 = 1u64;
                            nat_add(a2160, a2161)?
                        };
                        let a2163 = v7;
                        if nat_eq(a2162, a2163) {
                            let a2164 = v5;
                            let a2165 = 0u64;
                            if nat_eq(a2164, a2165) {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3({
                                    let a2166 = v5;
                                    let a2167 = 1u64;
                                    nat_sub(a2166, a2167)
                                }), Adt12::C0(0u64, f92(v9, v1)?), Adt13::C1({
                                    let a2168 = v5;
                                    let a2169 = 1u64;
                                    nat_sub(a2168, a2169)
                                }), 4u64, 4u64))
                            }
                        } else {
                            let v11: u64 = {
                                let a2170 = v10;
                                let a2171 = 2u64;
                                nat_sub(a2170, a2171)
                            };
                            let v12: u64 = {
                                let a2172 = v11;
                                let a2173 = 3u64;
                                let a2174 = 0u64;
                                nat_rem(a2172, a2173, a2174)
                            };
                            let m2178 = f173(v3, v2, v1, v9, {
                                let a2175 = v11;
                                let a2176 = 3u64;
                                let a2177 = 0u64;
                                nat_quot(a2175, a2176, a2177)
                            })?;
                            match m2178 {
                                None => {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                                Some(v13) => {
                                    let a2179 = v12;
                                    let a2180 = 0u64;
                                    if nat_eq(a2179, a2180) {
                                        Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C0(1u64, v13), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                                    } else {
                                        let a2181 = 2u64;
                                        let a2182 = v5;
                                        if nat_le(a2181, a2182) {
                                            let a2183 = v12;
                                            let a2184 = 1u64;
                                            if nat_eq(a2183, a2184) {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(1u64, v13), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                                    let a2185 = v13;
                                                    let a2186 = 1u64;
                                                    nat_add(a2185, a2186)?
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
            let a2187 = v10;
            let a2188 = 0u64;
            if nat_eq(a2187, a2188) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v6), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2189 = v10;
                let a2190 = 1u64;
                if nat_eq(a2189, a2190) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2191 = v10;
                    let a2192 = 2u64;
                    if nat_eq(a2191, a2192) {
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f92(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2195 = f173(v3, v2, v1, v9, {
                            let a2193 = v10;
                            let a2194 = 3u64;
                            nat_sub(a2193, a2194)
                        })?;
                        match m2195 {
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

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2198 = match v3.clone() {
            Adt6::C0(h2196, _, _, _) => {
                h2196
            }
        };
        let a2199 = match v4.clone() {
            Adt6::C0(h2197, _, _, _) => {
                h2197
            }
        };
        nat_mul(a2198, a2199)?
    };
    let v6: u64 = {
        let a2200 = v5;
        let a2201 = 2u64;
        nat_add(a2200, a2201)?
    };
    let v7: u64 = {
        let a2202 = v0;
        let a2203 = v6;
        let a2204 = 0u64;
        nat_quot(a2202, a2203, a2204)
    };
    let v8: u64 = {
        let a2205 = v0;
        let a2206 = v6;
        let a2207 = 0u64;
        nat_rem(a2205, a2206, a2207)
    };
    let a2208 = v8;
    let a2209 = 0u64;
    if nat_eq(a2208, a2209) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2212 = {
            let a2210 = v8;
            let a2211 = 1u64;
            nat_add(a2210, a2211)?
        };
        let a2213 = v6;
        if nat_eq(a2212, a2213) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f92(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2214 = v8;
                let a2215 = 1u64;
                nat_sub(a2214, a2215)
            };
            let m2221 = f174(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2216, _, _, _, _, _, _) => {
                    h2216
                }
            }, f58(v7, v2.clone(), 0u64), {
                let a2218 = v9;
                let a2219 = match v3.clone() {
                    Adt6::C0(h2217, _, _, _) => {
                        h2217
                    }
                };
                let a2220 = 0u64;
                nat_rem(a2218, a2219, a2220)
            })?;
            match m2221 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2227 = f174(v4, match v1.clone() {
                        Adt3::C0(_, _, h2222, _, _, _, _, _) => {
                            h2222
                        }
                    }, f58(v7, v2.clone(), 1u64), {
                        let a2224 = v9;
                        let a2225 = match v3 {
                            Adt6::C0(h2223, _, _, _) => {
                                h2223
                            }
                        };
                        let a2226 = 0u64;
                        nat_quot(a2224, a2225, a2226)
                    })?;
                    match m2227 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f57(v1, {
                                let a2230 = {
                                    let a2228 = f58(v7, v2.clone(), 2u64);
                                    let a2229 = v5;
                                    nat_mul(a2228, a2229)?
                                };
                                let a2231 = v9;
                                nat_add(a2230, a2231)?
                            }, v10, v11, f58(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2232, _, _, _, _, _, _, _) => {
            h2232
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2233, _, _, _, _, _, _, _) => {
            h2233
        }
    };
    let v6: u64 = f175(v4)?;
    let v7: u64 = {
        let a2234 = v0;
        let a2235 = v6;
        let a2236 = 0u64;
        nat_quot(a2234, a2235, a2236)
    };
    let v8: u64 = {
        let a2237 = v7;
        let a2238 = v5;
        let a2239 = 0u64;
        nat_quot(a2237, a2238, a2239)
    };
    let v9: u64 = {
        let a2240 = v7;
        let a2241 = v5;
        let a2242 = 0u64;
        nat_rem(a2240, a2241, a2242)
    };
    let v10: u64 = {
        let a2243 = v3;
        let a2244 = v4;
        let a2245 = 0u64;
        nat_rem(a2243, a2244, a2245)
    };
    let v11: u64 = {
        let a2250 = 4u64;
        let a2251 = {
            let a2248 = v8;
            let a2249 = {
                let a2246 = v4;
                let a2247 = v4;
                nat_mul(a2246, a2247)?
            };
            nat_mul(a2248, a2249)?
        };
        nat_mul(a2250, a2251)?
    };
    let v12: u64 = {
        let a2256 = 4u64;
        let a2257 = {
            let a2254 = v8;
            let a2255 = {
                let a2252 = v4;
                let a2253 = v5;
                nat_mul(a2252, a2253)?
            };
            nat_mul(a2254, a2255)?
        };
        nat_mul(a2256, a2257)?
    };
    let v13: u64 = {
        let a2261 = {
            let a2258 = v0;
            let a2259 = v6;
            let a2260 = 0u64;
            nat_rem(a2258, a2259, a2260)
        };
        let a2262 = f175(v10)?;
        nat_sub(a2261, a2262)
    };
    let a2265 = v13;
    let a2266 = {
        let a2263 = v10;
        let a2264 = 2u64;
        nat_mul(a2263, a2264)?
    };
    if nat_lt(a2265, a2266) {
        let v14: u64 = {
            let a2267 = v13;
            let a2268 = 2u64;
            let a2269 = 0u64;
            nat_quot(a2267, a2268, a2269)
        };
        let a2273 = {
            let a2270 = v13;
            let a2271 = 2u64;
            let a2272 = 0u64;
            nat_rem(a2270, a2271, a2272)
        };
        let a2274 = 0u64;
        if nat_eq(a2273, a2274) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2281 = v11;
                let a2282 = {
                    let a2279 = 4u64;
                    let a2280 = {
                        let a2277 = {
                            let a2275 = v10;
                            let a2276 = v4;
                            nat_mul(a2275, a2276)?
                        };
                        let a2278 = v14;
                        nat_add(a2277, a2278)?
                    };
                    nat_mul(a2279, a2280)?
                };
                nat_add(a2281, a2282)?
            }), Adt12::C3({
                let a2289 = v12;
                let a2290 = {
                    let a2287 = 4u64;
                    let a2288 = {
                        let a2285 = {
                            let a2283 = v14;
                            let a2284 = v5;
                            nat_mul(a2283, a2284)?
                        };
                        let a2286 = v9;
                        nat_add(a2285, a2286)?
                    };
                    nat_mul(a2287, a2288)?
                };
                nat_add(a2289, a2290)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2291 = v14;
                let a2292 = 0u64;
                if nat_eq(a2291, a2292) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2295 = v13;
        let a2296 = {
            let a2293 = v10;
            let a2294 = 2u64;
            nat_mul(a2293, a2294)?
        };
        if nat_eq(a2295, a2296) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2303 = v12;
                let a2304 = {
                    let a2301 = 4u64;
                    let a2302 = {
                        let a2299 = {
                            let a2297 = v10;
                            let a2298 = v5;
                            nat_mul(a2297, a2298)?
                        };
                        let a2300 = v9;
                        nat_add(a2299, a2300)?
                    };
                    nat_mul(a2301, a2302)?
                };
                nat_add(a2303, a2304)?
            }), {
                let a2305 = v10;
                let a2306 = 0u64;
                if nat_eq(a2305, a2306) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2311 = v13;
            let a2312 = {
                let a2309 = {
                    let a2307 = v10;
                    let a2308 = 2u64;
                    nat_mul(a2307, a2308)?
                };
                let a2310 = 1u64;
                nat_add(a2309, a2310)?
            };
            if nat_eq(a2311, a2312) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2319 = v11;
                    let a2320 = {
                        let a2317 = 4u64;
                        let a2318 = {
                            let a2315 = {
                                let a2313 = v10;
                                let a2314 = v4;
                                nat_mul(a2313, a2314)?
                            };
                            let a2316 = v10;
                            nat_add(a2315, a2316)?
                        };
                        nat_mul(a2317, a2318)?
                    };
                    nat_add(a2319, a2320)?
                }), Adt13::C1({
                    let a2327 = v12;
                    let a2328 = {
                        let a2325 = 4u64;
                        let a2326 = {
                            let a2323 = {
                                let a2321 = v10;
                                let a2322 = v5;
                                nat_mul(a2321, a2322)?
                            };
                            let a2324 = v9;
                            nat_add(a2323, a2324)?
                        };
                        nat_mul(a2325, a2326)?
                    };
                    nat_add(a2327, a2328)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2331 = {
                        let a2329 = v10;
                        let a2330 = 1u64;
                        nat_add(a2329, a2330)?
                    };
                    let a2332 = v4;
                    if nat_eq(a2331, a2332) {
                        0u64
                    } else {
                        let a2333 = v10;
                        let a2334 = 1u64;
                        nat_add(a2333, a2334)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f75(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2335 = v0;
    let a2336 = v1;
    if nat_lt(a2335, a2336) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f92(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2337 = v0;
            let a2338 = v1;
            nat_sub(a2337, a2338)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f92(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2345 = {
            let a2343 = 3u64;
            let a2344 = {
                let a2341 = match v4.clone() {
                    Adt6::C0(h2339, _, _, _) => {
                        h2339
                    }
                };
                let a2342 = match v5.clone() {
                    Adt6::C0(h2340, _, _, _) => {
                        h2340
                    }
                };
                nat_mul(a2341, a2342)?
            };
            nat_mul(a2343, a2344)?
        };
        let a2346 = 2u64;
        nat_add(a2345, a2346)?
    };
    let v8: u64 = {
        let a2347 = v0;
        let a2348 = v7;
        let a2349 = 0u64;
        nat_quot(a2347, a2348, a2349)
    };
    let v9: u64 = {
        let a2350 = v0;
        let a2351 = v7;
        let a2352 = 0u64;
        nat_rem(a2350, a2351, a2352)
    };
    let a2353 = v9;
    let a2354 = 0u64;
    if nat_eq(a2353, a2354) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2357 = {
            let a2355 = v9;
            let a2356 = 1u64;
            nat_add(a2355, a2356)?
        };
        let a2358 = v7;
        if nat_eq(a2357, a2358) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f92(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2359 = v9;
                let a2360 = 1u64;
                nat_sub(a2359, a2360)
            };
            let v11: u64 = {
                let a2361 = v10;
                let a2362 = 3u64;
                let a2363 = 0u64;
                nat_quot(a2361, a2362, a2363)
            };
            let v12: u64 = {
                let a2365 = v11;
                let a2366 = match v4.clone() {
                    Adt6::C0(h2364, _, _, _) => {
                        h2364
                    }
                };
                let a2367 = 0u64;
                nat_rem(a2365, a2366, a2367)
            };
            let v13: u64 = {
                let a2369 = v11;
                let a2370 = match v4.clone() {
                    Adt6::C0(h2368, _, _, _) => {
                        h2368
                    }
                };
                let a2371 = 0u64;
                nat_quot(a2369, a2370, a2371)
            };
            let v14: u64 = {
                let a2372 = v10;
                let a2373 = 3u64;
                let a2374 = 0u64;
                nat_rem(a2372, a2373, a2374)
            };
            let v15: u64 = f58(v8, v3.clone(), 2u64);
            let m2376 = f60(v4, match v2.clone() {
                Adt3::C0(h2375, _, _, _, _, _, _, _) => {
                    h2375
                }
            }, f58(v8, v3.clone(), 0u64), v12)?;
            match m2376 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2378 = f60(v5, match v2.clone() {
                        Adt3::C0(_, h2377, _, _, _, _, _, _) => {
                            h2377
                        }
                    }, f58(v8, v3.clone(), 1u64), v13)?;
                    match m2378 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2379 = v14;
                            let a2380 = 0u64;
                            if nat_eq(a2379, a2380) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f57(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f176(v6), 4u64))
                            } else {
                                let a2381 = v14;
                                let a2382 = 1u64;
                                if nat_eq(a2381, a2382) {
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

pub fn f77(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f177(match v1.clone() {
        Adt3::C0(h2383, _, _, _, _, _, _, _) => {
            h2383
        }
    })?;
    let v5: u64 = {
        let a2384 = v0;
        let a2385 = v4;
        let a2386 = 0u64;
        nat_quot(a2384, a2385, a2386)
    };
    let v6: u64 = {
        let a2387 = v0;
        let a2388 = v4;
        let a2389 = 0u64;
        nat_rem(a2387, a2388, a2389)
    };
    let a2390 = v6;
    let a2391 = 0u64;
    if nat_eq(a2390, a2391) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2392 = v6;
        let a2393 = 1u64;
        if nat_eq(a2392, a2393) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2396 = {
                let a2394 = v6;
                let a2395 = 1u64;
                nat_add(a2394, a2395)?
            };
            let a2397 = v4;
            if nat_eq(a2396, a2397) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f92(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2398 = v6;
                    let a2399 = 2u64;
                    nat_sub(a2398, a2399)
                };
                let v8: u64 = {
                    let a2400 = v7;
                    let a2401 = 3u64;
                    let a2402 = 0u64;
                    nat_quot(a2400, a2401, a2402)
                };
                let v9: u64 = {
                    let a2403 = v7;
                    let a2404 = 3u64;
                    let a2405 = 0u64;
                    nat_rem(a2403, a2404, a2405)
                };
                let v10: u64 = f92({
                    let a2409 = {
                        let a2407 = v5;
                        let a2408 = match v1.clone() {
                            Adt3::C0(h2406, _, _, _, _, _, _, _) => {
                                h2406
                            }
                        };
                        nat_mul(a2407, a2408)?
                    };
                    let a2410 = v8;
                    nat_add(a2409, a2410)?
                }, v1)?;
                let a2411 = v9;
                let a2412 = 0u64;
                if nat_eq(a2411, a2412) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2413 = v9;
                    let a2414 = 1u64;
                    if nat_eq(a2413, a2414) {
                        let a2415 = v3;
                        let a2416 = 2u64;
                        if nat_eq(a2415, a2416) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2417 = v3;
                        let a2418 = 2u64;
                        if if nat_eq(a2417, a2418) {
                            true
                        } else {
                            let a2419 = v3;
                            let a2420 = 3u64;
                            nat_eq(a2419, a2420)
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

pub fn f78(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2422 = 2u64;
        let a2423 = match v2.clone() {
            Adt3::C0(_, h2421, _, _, _, _, _, _) => {
                h2421
            }
        };
        nat_mul(a2422, a2423)?
    };
    let a2424 = v1;
    let a2425 = v6;
    if nat_lt(a2424, a2425) {
        let a2429 = {
            let a2426 = v1;
            let a2427 = 2u64;
            let a2428 = 0u64;
            nat_rem(a2426, a2427, a2428)
        };
        let a2430 = 0u64;
        if nat_eq(a2429, a2430) {
            Ok(Adt9::C4(19u64, 4u64, f92({
                let a2431 = v1;
                let a2432 = 2u64;
                let a2433 = 0u64;
                nat_quot(a2431, a2432, a2433)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2434 = v5;
            let a2435 = 2147483648u64;
            if if nat_lt(a2434, a2435) {
                let a2437 = v5;
                let a2438 = match v3 {
                    Adt3::C0(_, h2436, _, _, _, _, _, _) => {
                        h2436
                    }
                };
                nat_lt(a2437, a2438)
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
            let a2441 = match v3.clone() {
                Adt3::C0(h2439, _, _, _, _, _, _, _) => {
                    h2439
                }
            };
            let a2442 = match v3.clone() {
                Adt3::C0(_, h2440, _, _, _, _, _, _) => {
                    h2440
                }
            };
            nat_mul(a2441, a2442)?
        };
        let a2445 = {
            let a2443 = v1;
            let a2444 = v6;
            nat_sub(a2443, a2444)
        };
        let a2446 = v7;
        if nat_lt(a2445, a2446) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f92({
                let a2447 = v1;
                let a2448 = v6;
                nat_sub(a2447, a2448)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2451 = {
                    let a2449 = v1;
                    let a2450 = v6;
                    nat_sub(a2449, a2450)
                };
                let a2452 = v7;
                nat_sub(a2451, a2452)
            };
            let v9: u64 = {
                let a2454 = 1u64;
                let a2455 = f178(v0, match v2.clone() {
                    Adt3::C0(h2453, _, _, _, _, _, _, _) => {
                        h2453
                    }
                })?;
                nat_add(a2454, a2455)?
            };
            let v10: u64 = {
                let a2456 = v8;
                let a2457 = v9;
                let a2458 = 0u64;
                nat_quot(a2456, a2457, a2458)
            };
            let v11: u64 = {
                let a2459 = v8;
                let a2460 = v9;
                let a2461 = 0u64;
                nat_rem(a2459, a2460, a2461)
            };
            let a2462 = v11;
            let a2463 = 0u64;
            if nat_eq(a2462, a2463) {
                Ok(Adt9::C4(19u64, 4u64, f92(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f179(v0) {
                    let a2467 = {
                        let a2464 = v11;
                        let a2465 = 2u64;
                        let a2466 = 0u64;
                        nat_rem(a2464, a2465, a2466)
                    };
                    let a2468 = 1u64;
                    if nat_eq(a2467, a2468) {
                        Ok(Adt9::C4(6u64, 0u64, f92({
                            let a2477 = {
                                let a2470 = v10;
                                let a2471 = match v2.clone() {
                                    Adt3::C0(h2469, _, _, _, _, _, _, _) => {
                                        h2469
                                    }
                                };
                                nat_mul(a2470, a2471)?
                            };
                            let a2478 = {
                                let a2474 = {
                                    let a2472 = v11;
                                    let a2473 = 1u64;
                                    nat_sub(a2472, a2473)
                                };
                                let a2475 = 2u64;
                                let a2476 = 0u64;
                                nat_quot(a2474, a2475, a2476)
                            };
                            nat_add(a2477, a2478)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f92({
                            let a2487 = {
                                let a2480 = v5;
                                let a2481 = match v2.clone() {
                                    Adt3::C0(h2479, _, _, _, _, _, _, _) => {
                                        h2479
                                    }
                                };
                                nat_mul(a2480, a2481)?
                            };
                            let a2488 = {
                                let a2484 = {
                                    let a2482 = v11;
                                    let a2483 = 1u64;
                                    nat_sub(a2482, a2483)
                                };
                                let a2485 = 2u64;
                                let a2486 = 0u64;
                                nat_quot(a2484, a2485, a2486)
                            };
                            nat_add(a2487, a2488)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f92({
                            let a2497 = {
                                let a2490 = v5;
                                let a2491 = match v2 {
                                    Adt3::C0(h2489, _, _, _, _, _, _, _) => {
                                        h2489
                                    }
                                };
                                nat_mul(a2490, a2491)?
                            };
                            let a2498 = {
                                let a2494 = {
                                    let a2492 = v11;
                                    let a2493 = 1u64;
                                    nat_sub(a2492, a2493)
                                };
                                let a2495 = 2u64;
                                let a2496 = 0u64;
                                nat_quot(a2494, a2495, a2496)
                            };
                            nat_add(a2497, a2498)?
                        }, v3)?))
                    }
                } else {
                    f180(v0, v10, {
                        let a2499 = v11;
                        let a2500 = 1u64;
                        nat_sub(a2499, a2500)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f79(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2501 = v1;
    let a2502 = f181(v2.clone(), v4.clone())?;
    if nat_lt(a2501, a2502) {
        let a2505 = v1;
        let a2506 = {
            let a2503 = 2u64;
            let a2504 = f182(v4.clone())?;
            nat_mul(a2503, a2504)?
        };
        if nat_lt(a2505, a2506) {
            let a2510 = {
                let a2507 = v1;
                let a2508 = 2u64;
                let a2509 = 0u64;
                nat_rem(a2507, a2508, a2509)
            };
            let a2511 = 0u64;
            if nat_eq(a2510, a2511) {
                f183({
                    let a2512 = v1;
                    let a2513 = 2u64;
                    let a2514 = 0u64;
                    nat_quot(a2512, a2513, a2514)
                }, v4)
            } else {
                if f184(match v2 {
                    Adt3::C0(_, h2515, _, _, _, _, _, _) => {
                        h2515
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2518 = v1;
                let a2519 = {
                    let a2516 = 2u64;
                    let a2517 = f182(v4.clone())?;
                    nat_mul(a2516, a2517)?
                };
                nat_sub(a2518, a2519)
            };
            let v7: u64 = {
                let a2523 = v6;
                let a2524 = {
                    let a2521 = 1u64;
                    let a2522 = match v2.clone() {
                        Adt3::C0(h2520, _, _, _, _, _, _, _) => {
                            h2520
                        }
                    };
                    nat_add(a2521, a2522)?
                };
                let a2525 = 0u64;
                nat_quot(a2523, a2524, a2525)
            };
            let v8: u64 = {
                let a2529 = v6;
                let a2530 = {
                    let a2527 = 1u64;
                    let a2528 = match v2.clone() {
                        Adt3::C0(h2526, _, _, _, _, _, _, _) => {
                            h2526
                        }
                    };
                    nat_add(a2527, a2528)?
                };
                let a2531 = 0u64;
                nat_rem(a2529, a2530, a2531)
            };
            let a2532 = v8;
            let a2533 = 0u64;
            if nat_eq(a2532, a2533) {
                f183(v7, v4)
            } else {
                f185(v0, v7, {
                    let a2534 = v8;
                    let a2535 = 1u64;
                    nat_sub(a2534, a2535)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2538 = match v4.clone() {
            Adt3::C0(h2536, _, _, _, _, _, _, _) => {
                h2536
            }
        };
        let a2539 = match v4.clone() {
            Adt3::C0(_, h2537, _, _, _, _, _, _) => {
                h2537
            }
        };
        nat_mul(a2538, a2539)?
    };
    let v7: u64 = {
        let a2540 = 2u64;
        let a2541 = v6;
        nat_mul(a2540, a2541)?
    };
    let v8: u64 = f186(match v3.clone() {
        Adt3::C0(_, h2542, _, _, _, _, _, _) => {
            h2542
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2543, _, _, _, _, _) => {
            h2543
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2544, _, _, _, _) => {
            h2544
        }
    });
    let a2545 = v0;
    let a2546 = v7;
    if nat_lt(a2545, a2546) {
        let a2550 = {
            let a2547 = v0;
            let a2548 = 2u64;
            let a2549 = 0u64;
            nat_rem(a2547, a2548, a2549)
        };
        let a2551 = 0u64;
        if nat_eq(a2550, a2551) {
            Ok(Adt9::C4(19u64, 5u64, f92({
                let a2552 = v0;
                let a2553 = 2u64;
                let a2554 = 0u64;
                nat_quot(a2552, a2553, a2554)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f187(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2555 = v0;
            let a2556 = v7;
            nat_sub(a2555, a2556)
        };
        let v10: u64 = {
            let a2560 = v9;
            let a2561 = {
                let a2558 = 1u64;
                let a2559 = match v1.clone() {
                    Adt3::C0(h2557, _, _, _, _, _, _, _) => {
                        h2557
                    }
                };
                nat_add(a2558, a2559)?
            };
            let a2562 = 0u64;
            nat_quot(a2560, a2561, a2562)
        };
        let v11: u64 = {
            let a2566 = v9;
            let a2567 = {
                let a2564 = 1u64;
                let a2565 = match v1.clone() {
                    Adt3::C0(h2563, _, _, _, _, _, _, _) => {
                        h2563
                    }
                };
                nat_add(a2564, a2565)?
            };
            let a2568 = 0u64;
            nat_rem(a2566, a2567, a2568)
        };
        let a2569 = v11;
        let a2570 = 0u64;
        if nat_eq(a2569, a2570) {
            Ok(Adt9::C4(19u64, 5u64, f92({
                let a2571 = v10;
                let a2572 = v6;
                let a2573 = 0u64;
                nat_rem(a2571, a2572, a2573)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f187(v5, v8) {
                let v12: u64 = {
                    let a2579 = {
                        let a2575 = v10;
                        let a2576 = match v1.clone() {
                            Adt3::C0(h2574, _, _, _, _, _, _, _) => {
                                h2574
                            }
                        };
                        nat_mul(a2575, a2576)?
                    };
                    let a2580 = {
                        let a2577 = v11;
                        let a2578 = 1u64;
                        nat_sub(a2577, a2578)
                    };
                    nat_add(a2579, a2580)?
                };
                let a2581 = v5;
                let a2582 = 4294967295u64;
                if nat_eq(a2581, a2582) {
                    Ok(Adt9::C1(false, f92(v12, v1)?, f92(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f92(v12, v1)?, 4u64, f92({
                        let a2588 = {
                            let a2584 = v5;
                            let a2585 = match v3.clone() {
                                Adt3::C0(h2583, _, _, _, _, _, _, _) => {
                                    h2583
                                }
                            };
                            nat_mul(a2584, a2585)?
                        };
                        let a2589 = {
                            let a2586 = v11;
                            let a2587 = 1u64;
                            nat_sub(a2586, a2587)
                        };
                        nat_add(a2588, a2589)?
                    }, v3)?, 3u64, f92(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f188(v3.clone())?;
    let v6: u64 = {
        let a2590 = v0;
        let a2591 = v5;
        let a2592 = 0u64;
        nat_quot(a2590, a2591, a2592)
    };
    let v7: u64 = {
        let a2593 = v0;
        let a2594 = v5;
        let a2595 = 0u64;
        nat_rem(a2593, a2594, a2595)
    };
    let v8: u64 = f189(v3.clone())?;
    let a2596 = v7;
    let a2597 = 0u64;
    if nat_eq(a2596, a2597) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2598, _) => {
                h2598
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2602 = v5;
        let a2603 = {
            let a2600 = v7;
            let a2601 = if match v3.clone() {
                Adt7::C0(_, _, _, h2599, _) => {
                    h2599
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2600, a2601)?
        };
        if nat_le(a2602, a2603) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2604, _) => {
                    h2604
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f92(v6, v2)?))
            } else {
                let a2607 = {
                    let a2605 = v7;
                    let a2606 = 2u64;
                    nat_add(a2605, a2606)?
                };
                let a2608 = v5;
                if nat_eq(a2607, a2608) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2609, _, _) => {
                            h2609
                        }
                    } {
                        let a2616 = {
                            let m2612 = match v3.clone() {
                                Adt7::C0(h2611, _, _, _, _) => {
                                    h2611
                                }
                            };
                            match m2612 {
                                Adt6::C0(h2610, _, _, _) => {
                                    h2610
                                }
                            }
                        };
                        let a2617 = {
                            let m2615 = match v3 {
                                Adt7::C0(_, h2614, _, _, _) => {
                                    h2614
                                }
                            };
                            match m2615 {
                                Adt6::C0(h2613, _, _, _) => {
                                    h2613
                                }
                            }
                        };
                        nat_mul(a2616, a2617)?
                    } else {
                        f190(match v3 {
                            Adt7::C0(h2618, _, _, _, _) => {
                                h2618
                            }
                        }, match v1 {
                            Adt3::C0(h2619, _, _, _, _, _, _, _) => {
                                h2619
                            }
                        }, f58(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2620 = v9;
                        let a2621 = 0u64;
                        if nat_eq(a2620, a2621) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f92(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2624 = {
                    let a2622 = v7;
                    let a2623 = 1u64;
                    nat_sub(a2622, a2623)
                };
                let a2625 = v8;
                let a2626 = 0u64;
                nat_rem(a2624, a2625, a2626)
            };
            let m2632 = f173(v3.clone(), v1, v2, v6, {
                let a2629 = {
                    let a2627 = v7;
                    let a2628 = 1u64;
                    nat_sub(a2627, a2628)
                };
                let a2630 = v8;
                let a2631 = 0u64;
                nat_quot(a2629, a2630, a2631)
            })?;
            match m2632 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2634 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2633) => {
                            h2633
                        }
                    };
                    let a2635 = 1u64;
                    if if nat_eq(a2634, a2635) {
                        let a2636 = v10;
                        let a2637 = 0u64;
                        nat_eq(a2636, a2637)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2638, _) => {
                                h2638
                            }
                        } {
                            let a2641 = {
                                let a2639 = v10;
                                let a2640 = 1u64;
                                nat_add(a2639, a2640)?
                            };
                            let a2642 = v8;
                            if nat_eq(a2641, a2642) {
                                let a2643 = v4;
                                let a2644 = 2u64;
                                if nat_eq(a2643, a2644) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2646 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2645) => {
                                                h2645
                                            }
                                        };
                                        let a2647 = 1u64;
                                        if nat_eq(a2646, a2647) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2649 = match v3 {
                                            Adt7::C0(_, _, _, _, h2648) => {
                                                h2648
                                            }
                                        };
                                        let a2650 = 1u64;
                                        if nat_eq(a2649, a2650) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2652 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2651) => {
                                            h2651
                                        }
                                    };
                                    let a2653 = 1u64;
                                    if nat_eq(a2652, a2653) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2655 = match v3 {
                                        Adt7::C0(_, _, _, _, h2654) => {
                                            h2654
                                        }
                                    };
                                    let a2656 = 1u64;
                                    if nat_eq(a2655, a2656) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2658 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2657) => {
                                        h2657
                                    }
                                };
                                let a2659 = 1u64;
                                if nat_eq(a2658, a2659) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2661 = match v3 {
                                    Adt7::C0(_, _, _, _, h2660) => {
                                        h2660
                                    }
                                };
                                let a2662 = 1u64;
                                if nat_eq(a2661, a2662) {
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

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f191(match v3.clone() {
        Adt3::C0(h2663, _, _, _, _, _, _, _) => {
            h2663
        }
    })?;
    let v5: u64 = {
        let a2664 = v0;
        let a2665 = v4;
        let a2666 = 0u64;
        nat_rem(a2664, a2665, a2666)
    };
    let v6: u64 = {
        let a2667 = v0;
        let a2668 = v4;
        let a2669 = 0u64;
        nat_quot(a2667, a2668, a2669)
    };
    let a2670 = v5;
    let a2671 = 0u64;
    if nat_eq(a2670, a2671) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2674 = {
            let a2672 = v5;
            let a2673 = 1u64;
            nat_add(a2672, a2673)?
        };
        let a2675 = v4;
        if nat_eq(a2674, a2675) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f92(v6, v2)?))
        } else {
            let a2679 = {
                let a2676 = v5;
                let a2677 = 2u64;
                let a2678 = 0u64;
                nat_rem(a2676, a2677, a2678)
            };
            let a2680 = 0u64;
            if nat_eq(a2679, a2680) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2683 = {
                        let a2681 = v5;
                        let a2682 = 1u64;
                        nat_sub(a2681, a2682)
                    };
                    let a2684 = 2u64;
                    let a2685 = 0u64;
                    nat_quot(a2683, a2684, a2685)
                };
                let v8: u64 = f58(v6, v2.clone(), 0u64);
                let v9: u64 = f58(v6, v2.clone(), 1u64);
                let v10: u64 = f58(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f57(v1, {
                    let a2686 = v9;
                    let a2687 = v7;
                    nat_add(a2686, a2687)?
                }, v8, v10, 0u64)?, 4u64, f57(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f192(v3.clone())?;
    let v5: u64 = {
        let a2688 = v0;
        let a2689 = v4;
        let a2690 = 0u64;
        nat_rem(a2688, a2689, a2690)
    };
    let v6: u64 = {
        let a2691 = v0;
        let a2692 = v4;
        let a2693 = 0u64;
        nat_quot(a2691, a2692, a2693)
    };
    let a2694 = v5;
    let a2695 = 0u64;
    if nat_eq(a2694, a2695) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2698 = {
            let a2696 = v5;
            let a2697 = 1u64;
            nat_add(a2696, a2697)?
        };
        let a2699 = v4;
        if nat_eq(a2698, a2699) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f92(v6, v2)?))
        } else {
            let a2703 = {
                let a2700 = v5;
                let a2701 = 2u64;
                let a2702 = 0u64;
                nat_rem(a2700, a2701, a2702)
            };
            let a2704 = 0u64;
            if nat_eq(a2703, a2704) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2706 = f58(v6, v2.clone(), 0u64);
                    let a2707 = match v3.clone() {
                        Adt6::C0(_, _, h2705, _) => {
                            h2705
                        }
                    };
                    nat_add(a2706, a2707)?
                };
                let v8: u64 = {
                    let a2715 = {
                        let a2712 = {
                            let a2709 = v7;
                            let a2710 = match v3.clone() {
                                Adt6::C0(_, h2708, _, _) => {
                                    h2708
                                }
                            };
                            nat_add(a2709, a2710)?
                        };
                        let a2713 = match v3.clone() {
                            Adt6::C0(h2711, _, _, _) => {
                                h2711
                            }
                        };
                        nat_sub(a2712, a2713)
                    };
                    let a2716 = match v3.clone() {
                        Adt6::C0(_, h2714, _, _) => {
                            h2714
                        }
                    };
                    let a2717 = 0u64;
                    nat_quot(a2715, a2716, a2717)
                };
                let v9: u64 = {
                    let a2723 = v8;
                    let a2724 = {
                        let a2720 = {
                            let a2718 = v5;
                            let a2719 = 1u64;
                            nat_sub(a2718, a2719)
                        };
                        let a2721 = 2u64;
                        let a2722 = 0u64;
                        nat_quot(a2720, a2721, a2722)
                    };
                    nat_add(a2723, a2724)?
                };
                let v10: u64 = {
                    let a2726 = v9;
                    let a2727 = match v3.clone() {
                        Adt6::C0(_, h2725, _, _) => {
                            h2725
                        }
                    };
                    nat_mul(a2726, a2727)?
                };
                let v11: u64 = {
                    let a2728 = v7;
                    let a2729 = v10;
                    nat_sub(a2728, a2729)
                };
                let a2731 = v9;
                let a2732 = match v1.clone() {
                    Adt3::C0(_, h2730, _, _, _, _, _, _) => {
                        h2730
                    }
                };
                if if if nat_lt(a2731, a2732) {
                    let a2733 = v10;
                    let a2734 = v7;
                    nat_le(a2733, a2734)
                } else {
                    false
                } {
                    let a2736 = v11;
                    let a2737 = match v3.clone() {
                        Adt6::C0(h2735, _, _, _) => {
                            h2735
                        }
                    };
                    nat_lt(a2736, a2737)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f57(v1, {
                        let a2741 = {
                            let a2739 = f58(v6, v2, 1u64);
                            let a2740 = match v3 {
                                Adt6::C0(h2738, _, _, _) => {
                                    h2738
                                }
                            };
                            nat_mul(a2739, a2740)?
                        };
                        let a2742 = v11;
                        nat_add(a2741, a2742)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2744 = v5;
        let a2745 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2743, _) => {
                h2743
            }
        };
        let a2746 = 0u64;
        nat_rem(a2744, a2745, a2746)
    };
    let v7: u64 = {
        let a2748 = v5;
        let a2749 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2747, _) => {
                h2747
            }
        };
        let a2750 = 0u64;
        nat_quot(a2748, a2749, a2750)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2751, _, _, _, _, _, _, _) => {
            h2751
        }
    }, match v1.clone() {
        Adt3::C0(h2752, _, _, _, _, _, _, _) => {
            h2752
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a2756 = v4;
        let a2757 = {
            let m2755 = match v3.clone() {
                Adt5::C0(h2754, _, _, _, _, _, _, _) => {
                    h2754
                }
            };
            match m2755 {
                Adt6::C0(h2753, _, _, _) => {
                    h2753
                }
            }
        };
        let a2758 = 0u64;
        nat_rem(a2756, a2757, a2758)
    })?;
    let v9: u64 = {
        let a2762 = v4;
        let a2763 = {
            let m2761 = match v3.clone() {
                Adt5::C0(h2760, _, _, _, _, _, _, _) => {
                    h2760
                }
            };
            match m2761 {
                Adt6::C0(h2759, _, _, _) => {
                    h2759
                }
            }
        };
        let a2764 = 0u64;
        nat_quot(a2762, a2763, a2764)
    };
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2765, _, _, _, _, _, _) => {
            h2765
        }
    }, match v1.clone() {
        Adt3::C0(_, h2766, _, _, _, _, _, _) => {
            h2766
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a2770 = v9;
        let a2771 = {
            let m2769 = match v3.clone() {
                Adt5::C0(_, h2768, _, _, _, _, _, _) => {
                    h2768
                }
            };
            match m2769 {
                Adt6::C0(h2767, _, _, _) => {
                    h2767
                }
            }
        };
        let a2772 = 0u64;
        nat_rem(a2770, a2771, a2772)
    })?;
    let v11: u64 = {
        let a2776 = v9;
        let a2777 = {
            let m2775 = match v3.clone() {
                Adt5::C0(_, h2774, _, _, _, _, _, _) => {
                    h2774
                }
            };
            match m2775 {
                Adt6::C0(h2773, _, _, _) => {
                    h2773
                }
            }
        };
        let a2778 = 0u64;
        nat_quot(a2776, a2777, a2778)
    };
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2779, _, _, _, _, _) => {
            h2779
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2780, _, _, _, _, _) => {
            h2780
        }
    }, v6, {
        let a2784 = v11;
        let a2785 = {
            let m2783 = match v3.clone() {
                Adt5::C0(_, _, h2782, _, _, _, _, _) => {
                    h2782
                }
            };
            match m2783 {
                Adt6::C0(h2781, _, _, _) => {
                    h2781
                }
            }
        };
        let a2786 = 0u64;
        nat_rem(a2784, a2785, a2786)
    })?;
    let v13: u64 = {
        let a2790 = v11;
        let a2791 = {
            let m2789 = match v3.clone() {
                Adt5::C0(_, _, h2788, _, _, _, _, _) => {
                    h2788
                }
            };
            match m2789 {
                Adt6::C0(h2787, _, _, _) => {
                    h2787
                }
            }
        };
        let a2792 = 0u64;
        nat_quot(a2790, a2791, a2792)
    };
    let v17: Adt12 = {
        let m2793 = v8;
        match m2793 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2794 = v10;
                match m2794 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2795 = v12;
                        match m2795 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2804 = {
                                        let a2799 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a2800 = {
                                            let a2797 = v7;
                                            let a2798 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2796, _, _, _) => {
                                                    h2796
                                                }
                                            };
                                            nat_mul(a2797, a2798)?
                                        };
                                        nat_add(a2799, a2800)?
                                    };
                                    let a2805 = {
                                        let a2802 = v13;
                                        let a2803 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2801, _, _, _, _) => {
                                                h2801
                                            }
                                        };
                                        nat_mul(a2802, a2803)?
                                    };
                                    nat_add(a2804, a2805)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2806, _, _) => {
            h2806
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2807) => {
            h2807
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f92(v0, v2)?), {
        let a2809 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2808, _, _) => {
                h2808
            }
        };
        let a2810 = 0u64;
        if nat_eq(a2809, a2810) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2812 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2811) => {
                h2811
            }
        };
        let a2813 = 0u64;
        if nat_eq(a2812, a2813) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f85(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2826 = f193(v0, v1, match v2.clone() {
        Adt3::C0(h2814, _, _, _, _, _, _, _) => {
            h2814
        }
    }, match v2.clone() {
        Adt3::C0(_, h2815, _, _, _, _, _, _) => {
            h2815
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2816, _, _, _, _, _) => {
            h2816
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2817, _, _, _, _) => {
            h2817
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2818, _, _, _) => {
            h2818
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2819, _, _) => {
            h2819
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2820, _) => {
            h2820
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2821) => {
            h2821
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2822, _, _, _) => {
            h2822
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2823, _, _) => {
            h2823
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2824, _) => {
            h2824
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2825) => {
            h2825
        }
    }, v4)?;
    match m2826 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f20(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2829 = v6;
        let a2830 = {
            let a2827 = 3u64;
            let a2828 = v5;
            nat_add(a2827, a2828)?
        };
        nat_sub(a2829, a2830)
    };
    let a2832 = v0;
    let a2833 = f50(v3, match v1.clone() {
        Adt3::C0(h2831, _, _, _, _, _, _, _) => {
            h2831
        }
    }, v5, v8)?;
    if nat_lt(a2832, a2833) {
        let v9: u64 = f101(match v1.clone() {
            Adt3::C0(h2834, _, _, _, _, _, _, _) => {
                h2834
            }
        }, v5, v8)?;
        let v10: u64 = f102(v3, match v1.clone() {
            Adt3::C0(h2835, _, _, _, _, _, _, _) => {
                h2835
            }
        }, v5)?;
        let a2836 = v0;
        let a2837 = v9;
        if nat_lt(a2836, a2837) {
            f194(v0, v2, f44(v3), {
                let a2838 = 3u64;
                let a2839 = v5;
                nat_add(a2838, a2839)?
            }, v7)
        } else {
            let a2842 = v0;
            let a2843 = {
                let a2840 = v9;
                let a2841 = v10;
                nat_add(a2840, a2841)?
            };
            if nat_lt(a2842, a2843) {
                let v11: u64 = {
                    let a2844 = v0;
                    let a2845 = v9;
                    nat_sub(a2844, a2845)
                };
                let v12: u64 = f195(v3, match v1.clone() {
                    Adt3::C0(h2846, _, _, _, _, _, _, _) => {
                        h2846
                    }
                })?;
                f196({
                    let a2847 = v11;
                    let a2848 = v12;
                    let a2849 = 0u64;
                    nat_rem(a2847, a2848, a2849)
                }, {
                    let a2850 = v11;
                    let a2851 = v12;
                    let a2852 = 0u64;
                    nat_quot(a2850, a2851, a2852)
                }, v1, v3, v4, v7)
            } else {
                f197({
                    let a2855 = v0;
                    let a2856 = {
                        let a2853 = v9;
                        let a2854 = v10;
                        nat_add(a2853, a2854)?
                    };
                    nat_sub(a2855, a2856)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2857 = v0;
    let a2858 = f36(v1)?;
    if nat_lt(a2857, a2858) {
        let a2859 = v0;
        let a2860 = 4u64;
        if nat_lt(a2859, a2860) {
            let a2861 = v0;
            let a2862 = 0u64;
            if nat_eq(a2861, a2862) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2863 = v0;
                let a2864 = 1u64;
                if nat_eq(a2863, a2864) {
                    let a2865 = v5;
                    let a2866 = 0u64;
                    if nat_eq(a2865, a2866) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2867 = v0;
                    let a2868 = 2u64;
                    if nat_eq(a2867, a2868) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2869 = v0;
            let a2870 = 4u64;
            if if nat_eq(a2869, a2870) {
                let a2871 = f198(v5)?;
                let a2872 = v1;
                !nat_eq(a2871, a2872)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2875 = {
                        let a2873 = v0;
                        let a2874 = 4u64;
                        nat_sub(a2873, a2874)
                    };
                    let a2876 = 3u64;
                    let a2877 = 0u64;
                    nat_quot(a2875, a2876, a2877)
                };
                let v7: u64 = {
                    let a2880 = {
                        let a2878 = v0;
                        let a2879 = 4u64;
                        nat_sub(a2878, a2879)
                    };
                    let a2881 = 3u64;
                    let a2882 = 0u64;
                    nat_rem(a2880, a2881, a2882)
                };
                let a2883 = v7;
                let a2884 = 0u64;
                if nat_eq(a2883, a2884) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2885 = v7;
                    let a2886 = 1u64;
                    if nat_eq(a2885, a2886) {
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

pub fn f88(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2887 = v1;
        let a2888 = 5u64;
        nat_eq(a2887, a2888)
    } else {
        false
    } {
        let a2889 = v2;
        let a2890 = 16u64;
        nat_eq(a2889, a2890)
    } else {
        false
    } {
        let a2891 = v3;
        let a2892 = 0u64;
        nat_eq(a2891, a2892)
    } else {
        false
    } {
        let a2893 = v4;
        let a2894 = 0u64;
        nat_eq(a2893, a2894)
    } else {
        false
    }
}

pub fn f89(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2895 = v1;
    let a2896 = 0u64;
    if nat_eq(a2895, a2896) {
        if v0 {
            let a2897 = v3;
            let a2898 = 0u64;
            if if if nat_eq(a2897, a2898) {
                let a2899 = v4;
                let a2900 = 43u64;
                nat_eq(a2899, a2900)
            } else {
                false
            } {
                true
            } else {
                let a2901 = v3;
                let a2902 = 43u64;
                if nat_eq(a2901, a2902) {
                    let a2903 = v4;
                    let a2904 = 0u64;
                    nat_eq(a2903, a2904)
                } else {
                    false
                }
            } {
                f167(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2905 = v1;
        let a2906 = 1u64;
        if nat_eq(a2905, a2906) {
            Adt11::C1(v2)
        } else {
            let a2907 = v1;
            let a2908 = 2u64;
            if nat_eq(a2907, a2908) {
                Adt11::C5(v2)
            } else {
                let a2909 = v1;
                let a2910 = 3u64;
                if nat_eq(a2909, a2910) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2911 = v1;
                    let a2912 = 4u64;
                    if nat_eq(a2911, a2912) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f88(v0, v1, v2, v3, v4) {
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

pub fn f90(v0: u64, v1: u64) -> Adt12 {
    let a2913 = v0;
    let a2914 = 0u64;
    if nat_eq(a2913, a2914) {
        Adt12::C0(0u64, v1)
    } else {
        let a2915 = v0;
        let a2916 = 1u64;
        if nat_eq(a2915, a2916) {
            Adt12::C1(v1)
        } else {
            let a2917 = v0;
            let a2918 = 2u64;
            if nat_eq(a2917, a2918) {
                Adt12::C2(v1)
            } else {
                let a2919 = v0;
                let a2920 = 3u64;
                if nat_eq(a2919, a2920) {
                    Adt12::C3(v1)
                } else {
                    let a2921 = v0;
                    let a2922 = 13u64;
                    if nat_lt(a2921, a2922) {
                        Adt12::C0({
                            let a2923 = v0;
                            let a2924 = 3u64;
                            nat_sub(a2923, a2924)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: u64) -> Adt13 {
    let a2925 = v0;
    let a2926 = 1u64;
    if nat_eq(a2925, a2926) {
        Adt13::C0(v1)
    } else {
        let a2927 = v0;
        let a2928 = 3u64;
        if if nat_eq(a2927, a2928) {
            true
        } else {
            let a2929 = v0;
            let a2930 = 6u64;
            nat_eq(a2929, a2930)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f92(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2932 = v0;
        let a2933 = match v1.clone() {
            Adt3::C0(h2931, _, _, _, _, _, _, _) => {
                h2931
            }
        };
        let a2934 = 0u64;
        nat_quot(a2932, a2933, a2934)
    };
    let v3: u64 = {
        let a2936 = v2;
        let a2937 = match v1.clone() {
            Adt3::C0(_, h2935, _, _, _, _, _, _) => {
                h2935
            }
        };
        let a2938 = 0u64;
        nat_quot(a2936, a2937, a2938)
    };
    f142({
        let a2940 = v0;
        let a2941 = match v1.clone() {
            Adt3::C0(h2939, _, _, _, _, _, _, _) => {
                h2939
            }
        };
        let a2942 = 0u64;
        nat_rem(a2940, a2941, a2942)
    }, {
        let a2944 = v2;
        let a2945 = match v1.clone() {
            Adt3::C0(_, h2943, _, _, _, _, _, _) => {
                h2943
            }
        };
        let a2946 = 0u64;
        nat_rem(a2944, a2945, a2946)
    }, {
        let a2948 = v3;
        let a2949 = match v1.clone() {
            Adt3::C0(_, _, h2947, _, _, _, _, _) => {
                h2947
            }
        };
        let a2950 = 0u64;
        nat_rem(a2948, a2949, a2950)
    }, {
        let a2952 = v3;
        let a2953 = match v1.clone() {
            Adt3::C0(_, _, h2951, _, _, _, _, _) => {
                h2951
            }
        };
        let a2954 = 0u64;
        nat_quot(a2952, a2953, a2954)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2955, _, _, _) => {
            h2955
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2956, _, _) => {
            h2956
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2957, _) => {
            h2957
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2958) => {
            h2958
        }
    })
}

pub fn f93(v0: Adt6, v1: u64) -> R<u64> {
    let a2960 = 0u64;
    let a2961 = match v0.clone() {
        Adt6::C0(h2959, _, _, _) => {
            h2959
        }
    };
    if if if if if nat_lt(a2960, a2961) {
        let a2963 = 0u64;
        let a2964 = match v0.clone() {
            Adt6::C0(_, h2962, _, _) => {
                h2962
            }
        };
        nat_lt(a2963, a2964)
    } else {
        false
    } {
        let a2966 = 0u64;
        let a2967 = match v0.clone() {
            Adt6::C0(_, _, _, h2965) => {
                h2965
            }
        };
        nat_lt(a2966, a2967)
    } else {
        false
    } {
        let a2969 = match v0.clone() {
            Adt6::C0(_, h2968, _, _) => {
                h2968
            }
        };
        let a2970 = 2147483648u64;
        nat_lt(a2969, a2970)
    } else {
        false
    } {
        let a2972 = match v0.clone() {
            Adt6::C0(_, _, _, h2971) => {
                h2971
            }
        };
        let a2973 = 2147483648u64;
        nat_lt(a2972, a2973)
    } else {
        false
    } {
        let v2: u64 = {
            let a2977 = v1;
            let a2978 = {
                let a2975 = 2u64;
                let a2976 = f144(match v0.clone() {
                    Adt6::C0(_, _, h2974, _) => {
                        h2974
                    }
                });
                nat_mul(a2975, a2976)?
            };
            nat_add(a2977, a2978)?
        };
        let v3: u64 = {
            let a2990 = {
                let a2985 = 1u64;
                let a2986 = {
                    let a2983 = match v0.clone() {
                        Adt6::C0(_, _, _, h2979) => {
                            h2979
                        }
                    };
                    let a2984 = {
                        let a2981 = match v0.clone() {
                            Adt6::C0(h2980, _, _, _) => {
                                h2980
                            }
                        };
                        let a2982 = 1u64;
                        nat_sub(a2981, a2982)
                    };
                    nat_mul(a2983, a2984)?
                };
                nat_add(a2985, a2986)?
            };
            let a2991 = {
                let a2988 = 2u64;
                let a2989 = f143(match v0.clone() {
                    Adt6::C0(_, _, h2987, _) => {
                        h2987
                    }
                });
                nat_mul(a2988, a2989)?
            };
            nat_add(a2990, a2991)?
        };
        let a2992 = v3;
        let a2993 = v2;
        if nat_le(a2992, a2993) {
            let a3000 = 1u64;
            let a3001 = {
                let a2997 = {
                    let a2994 = v2;
                    let a2995 = v3;
                    nat_sub(a2994, a2995)
                };
                let a2998 = match v0 {
                    Adt6::C0(_, h2996, _, _) => {
                        h2996
                    }
                };
                let a2999 = 0u64;
                nat_quot(a2997, a2998, a2999)
            };
            nat_add(a3000, a3001)
        } else {
            let a3005 = {
                let a3002 = v3;
                let a3003 = v2;
                nat_sub(a3002, a3003)
            };
            let a3006 = match v0 {
                Adt6::C0(_, h3004, _, _) => {
                    h3004
                }
            };
            if nat_lt(a3005, a3006) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f94(v0: u64) -> u64 {
    let v1: u64 = {
        let a3007 = v0;
        let a3008 = 4294967296u64;
        let a3009 = 0u64;
        nat_rem(a3007, a3008, a3009)
    };
    let a3010 = v1;
    let a3011 = 2147483648u64;
    if nat_lt(a3010, a3011) {
        v1
    } else {
        0u64
    }
}

pub fn f95(v0: u64) -> R<u64> {
    let a3014 = 2u64;
    let a3015 = {
        let a3012 = 2u64;
        let a3013 = v0;
        nat_mul(a3012, a3013)?
    };
    nat_add(a3014, a3015)
}

pub fn f96(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3017 = match v1.clone() {
        Adt14::C0(h3016, _, _, _, _, _, _, _, _, _, _) => {
            h3016
        }
    };
    let a3018 = 0u64;
    if if if if nat_eq(a3017, a3018) {
        let a3020 = match v2.clone() {
            Adt14::C0(h3019, _, _, _, _, _, _, _, _, _, _) => {
                h3019
            }
        };
        let a3021 = 0u64;
        nat_eq(a3020, a3021)
    } else {
        false
    } {
        f199(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f200(match v1.clone() {
            Adt14::C0(_, h3022, _, _, _, _, _, _, _, _, _) => {
                h3022
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3023, _, _, _, _, _, _, _, _) => {
                h3023
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3024, _, _, _, _, _, _, _) => {
                h3024
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3025, _, _, _, _, _, _) => {
                h3025
            }
        }) {
            true
        } else {
            if if if if f45(v0) {
                f201(v1.clone(), v2.clone())
            } else {
                false
            } {
                f202(v1.clone())
            } else {
                false
            } {
                f202(v2.clone())
            } else {
                false
            } {
                if f203(v1.clone(), v2.clone())? {
                    true
                } else {
                    f204(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f200(match v1.clone() {
            Adt14::C0(_, h3026, _, _, _, _, _, _, _, _, _) => {
                h3026
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3027, _, _, _, _, _, _, _, _) => {
                h3027
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3028, _, _, _, _, _, _, _) => {
                h3028
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3029, _, _, _, _, _, _) => {
                h3029
            }
        }) {
            0u64
        } else {
            let a3035 = match v1.clone() {
                Adt14::C0(_, _, h3030, _, _, _, _, _, _, _, _) => {
                    h3030
                }
            };
            let a3036 = {
                let a3033 = match v1.clone() {
                    Adt14::C0(_, _, _, h3031, _, _, _, _, _, _, _) => {
                        h3031
                    }
                };
                let a3034 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3032, _, _, _, _, _, _) => {
                        h3032
                    }
                };
                nat_mul(a3033, a3034)?
            };
            nat_mul(a3035, a3036)?
        };
        let v4: u64 = {
            let a3037 = v3;
            let a3038 = 0u64;
            if if nat_eq(a3037, a3038) {
                true
            } else {
                f205(v2.clone())?
            } {
                0u64
            } else {
                let a3040 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3039, _) => {
                        h3039
                    }
                };
                let a3041 = 4u64;
                let a3042 = 0u64;
                nat_quot(a3040, a3041, a3042)
            }
        };
        Ok(Adt15::C1(f206(f207(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3043, _, _, _, _, _) => {
                h3043
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3044, _, _, _, _, _) => {
                h3044
            }
        }), match v1 {
            Adt14::C0(_, h3045, _, _, _, _, _, _, _, _, _) => {
                h3045
            }
        }, v3, v4)?, 4u64, 64u64, v3, f208(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f97(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f209(v3.clone());
    let a3047 = match v2.clone() {
        Adt14::C0(h3046, _, _, _, _, _, _, _, _, _, _) => {
            h3046
        }
    };
    let a3048 = 0u64;
    if if if if nat_eq(a3047, a3048) {
        let a3050 = match v3.clone() {
            Adt14::C0(h3049, _, _, _, _, _, _, _, _, _, _) => {
                h3049
            }
        };
        let a3051 = 0u64;
        nat_eq(a3050, a3051)
    } else {
        false
    } {
        f199(v2.clone(), v3)
    } else {
        false
    } {
        if f200(match v2.clone() {
            Adt14::C0(_, h3052, _, _, _, _, _, _, _, _, _) => {
                h3052
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3053, _, _, _, _, _, _, _, _) => {
                h3053
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3054, _, _, _, _, _, _, _) => {
                h3054
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3055, _, _, _, _, _, _) => {
                h3055
            }
        }) {
            true
        } else {
            let a3057 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3056, _, _, _, _, _) => {
                    h3056
                }
            };
            let a3058 = 4u64;
            if if if if if nat_eq(a3057, a3058) {
                f210(match v2.clone() {
                    Adt14::C0(_, h3059, _, _, _, _, _, _, _, _, _) => {
                        h3059
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3060, _, _, _, _, _, _, _, _) => {
                        h3060
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3061, _, _, _, _, _, _, _) => {
                        h3061
                    }
                }, v0)?
            } else {
                false
            } {
                f202(v2.clone())
            } else {
                false
            } {
                f202(v4.clone())
            } else {
                false
            } {
                let a3063 = f211(v4.clone())?;
                let a3064 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3062, _) => {
                        h3062
                    }
                };
                nat_le(a3063, a3064)
            } else {
                false
            } {
                if f203(v2.clone(), v4.clone())? {
                    true
                } else {
                    f204(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f200(match v2.clone() {
            Adt14::C0(_, h3065, _, _, _, _, _, _, _, _, _) => {
                h3065
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3066, _, _, _, _, _, _, _, _) => {
                h3066
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3067, _, _, _, _, _, _, _) => {
                h3067
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3068, _, _, _, _, _, _) => {
                h3068
            }
        }) {
            true
        } else {
            f205(v4.clone())?
        } {
            0u64
        } else {
            let a3070 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3069, _) => {
                    h3069
                }
            };
            let a3071 = 4u64;
            let a3072 = 0u64;
            nat_quot(a3070, a3071, a3072)
        };
        Ok(Adt15::C1(f212(match v2.clone() {
            Adt14::C0(_, h3073, _, _, _, _, _, _, _, _, _) => {
                h3073
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3074, _, _, _, _, _, _, _, _) => {
                h3074
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3075, _, _, _, _, _, _, _) => {
                h3075
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3076, _, _, _, _, _, _) => {
                h3076
            }
        }, v0, v5)?, 4u64, 65u64, v0, f208(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f98(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f209(v4.clone());
    if if if if if if if v1 {
        let a3078 = match v2.clone() {
            Adt14::C0(h3077, _, _, _, _, _, _, _, _, _, _) => {
                h3077
            }
        };
        let a3079 = 0u64;
        nat_eq(a3078, a3079)
    } else {
        false
    } {
        let a3081 = match v3.clone() {
            Adt14::C0(h3080, _, _, _, _, _, _, _, _, _, _) => {
                h3080
            }
        };
        let a3082 = 0u64;
        nat_eq(a3081, a3082)
    } else {
        false
    } {
        let a3084 = match v4.clone() {
            Adt14::C0(h3083, _, _, _, _, _, _, _, _, _, _) => {
                h3083
            }
        };
        let a3085 = 0u64;
        nat_eq(a3084, a3085)
    } else {
        false
    } {
        f199(v2.clone(), v3.clone())
    } else {
        false
    } {
        f199(v2.clone(), v4)
    } else {
        false
    } {
        if f200(match v2.clone() {
            Adt14::C0(_, h3086, _, _, _, _, _, _, _, _, _) => {
                h3086
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3087, _, _, _, _, _, _, _, _) => {
                h3087
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3088, _, _, _, _, _, _, _) => {
                h3088
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3089, _, _, _, _, _, _) => {
                h3089
            }
        }) {
            true
        } else {
            let a3091 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3090, _, _, _, _, _) => {
                    h3090
                }
            };
            let a3092 = 4u64;
            if if if if if if if nat_eq(a3091, a3092) {
                let a3094 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3093, _, _, _, _, _) => {
                        h3093
                    }
                };
                let a3095 = 4u64;
                nat_eq(a3094, a3095)
            } else {
                false
            } {
                f202(v2.clone())
            } else {
                false
            } {
                f202(v3.clone())
            } else {
                false
            } {
                f202(v5.clone())
            } else {
                false
            } {
                let a3097 = f211(v5.clone())?;
                let a3098 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3096, _) => {
                        h3096
                    }
                };
                nat_le(a3097, a3098)
            } else {
                false
            } {
                if f203(v2.clone(), v5.clone())? {
                    true
                } else {
                    f204(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f203(v3.clone(), v5.clone())? {
                    true
                } else {
                    f204(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f200(match v2.clone() {
            Adt14::C0(_, h3099, _, _, _, _, _, _, _, _, _) => {
                h3099
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3100, _, _, _, _, _, _, _, _) => {
                h3100
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3101, _, _, _, _, _, _, _) => {
                h3101
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3102, _, _, _, _, _, _) => {
                h3102
            }
        }) {
            0u64
        } else {
            let a3108 = match v2.clone() {
                Adt14::C0(_, _, h3103, _, _, _, _, _, _, _, _) => {
                    h3103
                }
            };
            let a3109 = {
                let a3106 = match v2.clone() {
                    Adt14::C0(_, _, _, h3104, _, _, _, _, _, _, _) => {
                        h3104
                    }
                };
                let a3107 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3105, _, _, _, _, _, _) => {
                        h3105
                    }
                };
                nat_mul(a3106, a3107)?
            };
            nat_mul(a3108, a3109)?
        };
        let v7: u64 = {
            let a3110 = v6;
            let a3111 = 0u64;
            if if nat_eq(a3110, a3111) {
                true
            } else {
                f205(v5.clone())?
            } {
                0u64
            } else {
                let a3113 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3112, _) => {
                        h3112
                    }
                };
                let a3114 = 4u64;
                let a3115 = 0u64;
                nat_quot(a3113, a3114, a3115)
            }
        };
        Ok(Adt15::C1(f213(match v2 {
            Adt14::C0(_, h3116, _, _, _, _, _, _, _, _, _) => {
                h3116
            }
        }, v6, v7)?, 4u64, 66u64, v6, f208(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f99(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f214(v0, v3, v6) {
        f214(v1, v4, v6)
    } else {
        false
    } {
        f214(v2, v5, v6)
    } else {
        false
    }
}

pub fn f100(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f215(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f215(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f215(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f215(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f215(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f215(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3117 = v2;
    let a3118 = 0u64;
    if nat_eq(a3117, a3118) {
        Ok(0u64)
    } else {
        let a3121 = 2u64;
        let a3122 = {
            let a3119 = v0;
            let a3120 = v1;
            nat_mul(a3119, a3120)?
        };
        nat_mul(a3121, a3122)
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3123 = v2;
    let a3124 = f195(v0, v1)?;
    nat_mul(a3123, a3124)
}

pub fn f103(v0: u64) -> u64 {
    let a3125 = v0;
    let a3126 = 30u64;
    if nat_eq(a3125, a3126) {
        3u64
    } else {
        1u64
    }
}

pub fn f104(v0: u64) -> R<u64> {
    let a3129 = 5u64;
    let a3130 = {
        let a3127 = 4u64;
        let a3128 = v0;
        nat_mul(a3127, a3128)?
    };
    nat_add(a3129, a3130)
}

pub fn f105(v0: u64) -> bool {
    let m3131 = f216(v0);
    match m3131 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3133 = 1u64;
            let a3134 = match v1 {
                Adt19::C0(_, h3132, _) => {
                    h3132
                }
            };
            nat_lt(a3133, a3134)
        }
    }
}

pub fn f106(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f21(v2.clone(), v3.clone()) {
        let a3136 = match v3.clone() {
            Adt2::C0(h3135, _, _, _, _, _, _, _, _, _, _) => {
                h3135
            }
        };
        let a3137 = 0u64;
        nat_eq(a3136, a3137)
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
        let a3139 = match v2.clone() {
            Adt2::C0(h3138, _, _, _, _, _, _, _, _, _, _) => {
                h3138
            }
        };
        let a3140 = 1u64;
        nat_le(a3139, a3140)
    } else {
        false
    } {
        let a3142 = match v0.clone() {
            Adt1::C0(h3141, _, _, _, _, _, _, _, _, _, _, _) => {
                h3141
            }
        };
        let a3143 = 0u64;
        if nat_eq(a3142, a3143) {
            let a3145 = match v2.clone() {
                Adt2::C0(h3144, _, _, _, _, _, _, _, _, _, _) => {
                    h3144
                }
            };
            let a3146 = 0u64;
            if if nat_eq(a3145, a3146) {
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
        f115(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3148 = match v1.clone() {
        Adt2::C0(h3147, _, _, _, _, _, _, _, _, _, _) => {
            h3147
        }
    };
    let a3149 = 0u64;
    if if if if if if if nat_eq(a3148, a3149) {
        let a3151 = match v3.clone() {
            Adt2::C0(h3150, _, _, _, _, _, _, _, _, _, _) => {
                h3150
            }
        };
        let a3152 = 0u64;
        nat_eq(a3151, a3152)
    } else {
        false
    } {
        let a3154 = match v2.clone() {
            Adt2::C0(h3153, _, _, _, _, _, _, _, _, _, _) => {
                h3153
            }
        };
        let a3155 = 1u64;
        nat_le(a3154, a3155)
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
        let a3157 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3156, _, _, _, _, _) => {
                h3156
            }
        };
        let a3158 = 0u64;
        if nat_eq(a3157, a3158) {
            true
        } else {
            let a3160 = match v3.clone() {
                Adt2::C0(_, _, h3159, _, _, _, _, _, _, _, _) => {
                    h3159
                }
            };
            let a3161 = 1u64;
            if nat_le(a3160, a3161) {
                true
            } else {
                let a3166 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3162, _, _, _, _) => {
                        h3162
                    }
                };
                let a3167 = {
                    let a3164 = 4u64;
                    let a3165 = match v3.clone() {
                        Adt2::C0(_, h3163, _, _, _, _, _, _, _, _, _) => {
                            h3163
                        }
                    };
                    nat_mul(a3164, a3165)?
                };
                nat_eq(a3166, a3167)
            }
        }
    } else {
        false
    } {
        f116(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f108(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3169 = match v0.clone() {
        Adt2::C0(h3168, _, _, _, _, _, _, _, _, _, _) => {
            h3168
        }
    };
    let a3170 = 0u64;
    if if if if if if if if if if if if nat_eq(a3169, a3170) {
        f38(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3172 = match v1.clone() {
            Adt2::C0(h3171, _, _, _, _, _, _, _, _, _, _) => {
                h3171
            }
        };
        let a3173 = 0u64;
        if if nat_eq(a3172, a3173) {
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
        let a3175 = match v2.clone() {
            Adt2::C0(h3174, _, _, _, _, _, _, _, _, _, _) => {
                h3174
            }
        };
        let a3176 = 0u64;
        if if nat_eq(a3175, a3176) {
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
        let a3179 = match v0.clone() {
            Adt2::C0(_, h3177, _, _, _, _, _, _, _, _, _) => {
                h3177
            }
        };
        let a3180 = match v0.clone() {
            Adt2::C0(_, _, h3178, _, _, _, _, _, _, _, _) => {
                h3178
            }
        };
        nat_eq(a3179, a3180)
    } else {
        false
    } {
        let a3183 = match v0.clone() {
            Adt2::C0(_, _, h3181, _, _, _, _, _, _, _, _) => {
                h3181
            }
        };
        let a3184 = match v1.clone() {
            Adt2::C0(_, _, h3182, _, _, _, _, _, _, _, _) => {
                h3182
            }
        };
        nat_eq(a3183, a3184)
    } else {
        false
    } {
        let a3187 = match v0.clone() {
            Adt2::C0(_, _, _, h3185, _, _, _, _, _, _, _) => {
                h3185
            }
        };
        let a3188 = match v1.clone() {
            Adt2::C0(_, _, _, h3186, _, _, _, _, _, _, _) => {
                h3186
            }
        };
        nat_eq(a3187, a3188)
    } else {
        false
    } {
        let a3191 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3189, _, _, _, _, _, _) => {
                h3189
            }
        };
        let a3192 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3190, _, _, _, _, _, _) => {
                h3190
            }
        };
        nat_eq(a3191, a3192)
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
            let a3201 = {
                let a3198 = {
                    let a3195 = match v1.clone() {
                        Adt2::C0(_, _, _, h3193, _, _, _, _, _, _, _) => {
                            h3193
                        }
                    };
                    let a3196 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3194, _, _, _, _, _, _) => {
                            h3194
                        }
                    };
                    nat_mul(a3195, a3196)?
                };
                let a3199 = match v1 {
                    Adt2::C0(_, h3197, _, _, _, _, _, _, _, _, _) => {
                        h3197
                    }
                };
                nat_mul(a3198, a3199)?
            };
            let a3202 = f175(match v0.clone() {
                Adt2::C0(_, h3200, _, _, _, _, _, _, _, _, _) => {
                    h3200
                }
            })?;
            nat_mul(a3201, a3202)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f109(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3204 = 0u64;
        let a3205 = match v0.clone() {
            Adt1::C0(_, h3203, _, _, _, _, _, _, _, _, _, _) => {
                h3203
            }
        };
        nat_lt(a3204, a3205)
    };
    let a3207 = match v1.clone() {
        Adt2::C0(h3206, _, _, _, _, _, _, _, _, _, _) => {
            h3206
        }
    };
    let a3208 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3207, a3208) {
        let a3210 = match v2.clone() {
            Adt2::C0(h3209, _, _, _, _, _, _, _, _, _, _) => {
                h3209
            }
        };
        let a3211 = 0u64;
        nat_eq(a3210, a3211)
    } else {
        false
    } {
        let a3213 = match v3.clone() {
            Adt2::C0(h3212, _, _, _, _, _, _, _, _, _, _) => {
                h3212
            }
        };
        let a3214 = 0u64;
        nat_eq(a3213, a3214)
    } else {
        false
    } {
        let a3216 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3215, _, _, _, _, _, _) => {
                h3215
            }
        };
        let a3217 = 1u64;
        nat_eq(a3216, a3217)
    } else {
        false
    } {
        let a3219 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3218, _, _, _, _, _, _) => {
                h3218
            }
        };
        let a3220 = 1u64;
        nat_eq(a3219, a3220)
    } else {
        false
    } {
        let a3222 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3221, _, _, _, _, _, _) => {
                h3221
            }
        };
        let a3223 = 1u64;
        nat_eq(a3222, a3223)
    } else {
        false
    } {
        let a3225 = 0u64;
        let a3226 = match v1.clone() {
            Adt2::C0(_, _, h3224, _, _, _, _, _, _, _, _) => {
                h3224
            }
        };
        nat_lt(a3225, a3226)
    } else {
        false
    } {
        let a3228 = match v3.clone() {
            Adt2::C0(_, _, _, h3227, _, _, _, _, _, _, _) => {
                h3227
            }
        };
        let a3229 = 1u64;
        nat_eq(a3228, a3229)
    } else {
        false
    } {
        let a3232 = match v3.clone() {
            Adt2::C0(_, h3230, _, _, _, _, _, _, _, _, _) => {
                h3230
            }
        };
        let a3233 = match v1.clone() {
            Adt2::C0(_, h3231, _, _, _, _, _, _, _, _, _) => {
                h3231
            }
        };
        nat_eq(a3232, a3233)
    } else {
        false
    } {
        let a3236 = match v3.clone() {
            Adt2::C0(_, _, h3234, _, _, _, _, _, _, _, _) => {
                h3234
            }
        };
        let a3237 = match v1.clone() {
            Adt2::C0(_, _, _, h3235, _, _, _, _, _, _, _) => {
                h3235
            }
        };
        nat_eq(a3236, a3237)
    } else {
        false
    } {
        if v4 {
            f21(v1.clone(), v2.clone())
        } else {
            let a3240 = match v2.clone() {
                Adt2::C0(_, h3238, _, _, _, _, _, _, _, _, _) => {
                    h3238
                }
            };
            let a3241 = match v1.clone() {
                Adt2::C0(_, _, h3239, _, _, _, _, _, _, _, _) => {
                    h3239
                }
            };
            if if nat_eq(a3240, a3241) {
                let a3244 = match v2.clone() {
                    Adt2::C0(_, _, h3242, _, _, _, _, _, _, _, _) => {
                        h3242
                    }
                };
                let a3245 = match v1.clone() {
                    Adt2::C0(_, _, _, h3243, _, _, _, _, _, _, _) => {
                        h3243
                    }
                };
                nat_eq(a3244, a3245)
            } else {
                false
            } {
                let a3247 = match v2.clone() {
                    Adt2::C0(_, _, _, h3246, _, _, _, _, _, _, _) => {
                        h3246
                    }
                };
                let a3248 = 1u64;
                nat_eq(a3247, a3248)
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
            let a3250 = f23(v3.clone(), 1u64)?;
            let a3251 = f169(match v1.clone() {
                Adt2::C0(_, _, h3249, _, _, _, _, _, _, _, _) => {
                    h3249
                }
            }, v4)?;
            nat_mul(a3250, a3251)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C36(f25(v2, 1u64), v4, match v0 {
            Adt1::C0(h3252, _, _, _, _, _, _, _, _, _, _, _) => {
                h3252
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3253, _, _, _, _, _, _, _, _, _) => {
            h3253
        }
    }, match v0.clone() {
        Adt1::C0(h3254, _, _, _, _, _, _, _, _, _, _, _) => {
            h3254
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3255, _, _, _, _, _, _, _, _, _) => {
            h3255
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3256, _, _, _, _, _, _, _) => {
            h3256
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3257, _, _, _, _, _, _, _, _) => {
            h3257
        }
    }, match v0.clone() {
        Adt1::C0(_, h3258, _, _, _, _, _, _, _, _, _, _) => {
            h3258
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3259, _, _, _, _, _, _, _, _) => {
            h3259
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3260, _, _, _, _, _, _) => {
            h3260
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3261, _, _, _, _, _, _, _, _, _, _) => {
            h3261
        }
    };
    let v7: bool = {
        let a3264 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3262, _, _, _) => {
                h3262
            }
        };
        let a3265 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3263, _, _, _, _, _) => {
                h3263
            }
        };
        nat_lt(a3264, a3265)
    };
    let a3266 = v6;
    let a3267 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3266, a3267) {
        true
    } else {
        let a3268 = v6;
        let a3269 = 1u64;
        nat_eq(a3268, a3269)
    } {
        let a3271 = match v2.clone() {
            Adt2::C0(h3270, _, _, _, _, _, _, _, _, _, _) => {
                h3270
            }
        };
        let a3272 = 0u64;
        nat_eq(a3271, a3272)
    } else {
        false
    } {
        let a3274 = match v3.clone() {
            Adt2::C0(h3273, _, _, _, _, _, _, _, _, _, _) => {
                h3273
            }
        };
        let a3275 = 0u64;
        nat_eq(a3274, a3275)
    } else {
        false
    } {
        let a3277 = match v1.clone() {
            Adt2::C0(_, _, _, h3276, _, _, _, _, _, _, _) => {
                h3276
            }
        };
        let a3278 = 1u64;
        nat_eq(a3277, a3278)
    } else {
        false
    } {
        let a3281 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3279, _, _, _, _, _, _) => {
                h3279
            }
        };
        let a3282 = match v2.clone() {
            Adt2::C0(_, _, _, h3280, _, _, _, _, _, _, _) => {
                h3280
            }
        };
        nat_eq(a3281, a3282)
    } else {
        false
    } {
        let a3285 = match v3.clone() {
            Adt2::C0(_, _, _, h3283, _, _, _, _, _, _, _) => {
                h3283
            }
        };
        let a3286 = match v2.clone() {
            Adt2::C0(_, _, _, h3284, _, _, _, _, _, _, _) => {
                h3284
            }
        };
        nat_eq(a3285, a3286)
    } else {
        false
    } {
        let a3289 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3287, _, _, _, _, _, _) => {
                h3287
            }
        };
        let a3290 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3288, _, _, _, _, _, _) => {
                h3288
            }
        };
        nat_eq(a3289, a3290)
    } else {
        false
    } {
        let a3292 = 0u64;
        let a3293 = match v4.clone() {
            Adt6::C0(h3291, _, _, _) => {
                h3291
            }
        };
        if if if if if nat_lt(a3292, a3293) {
            let a3295 = 0u64;
            let a3296 = match v4.clone() {
                Adt6::C0(_, h3294, _, _) => {
                    h3294
                }
            };
            nat_lt(a3295, a3296)
        } else {
            false
        } {
            let a3298 = match v4.clone() {
                Adt6::C0(_, h3297, _, _) => {
                    h3297
                }
            };
            let a3299 = 2147483648u64;
            nat_lt(a3298, a3299)
        } else {
            false
        } {
            let a3301 = 0u64;
            let a3302 = match v4.clone() {
                Adt6::C0(_, _, _, h3300) => {
                    h3300
                }
            };
            nat_lt(a3301, a3302)
        } else {
            false
        } {
            let a3304 = match v4.clone() {
                Adt6::C0(_, _, _, h3303) => {
                    h3303
                }
            };
            let a3305 = 2147483648u64;
            nat_lt(a3304, a3305)
        } else {
            false
        } {
            let a3308 = match v3.clone() {
                Adt2::C0(_, h3306, _, _, _, _, _, _, _, _, _) => {
                    h3306
                }
            };
            let a3309 = f93(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3307, _, _, _, _, _, _, _, _, _) => {
                    h3307
                }
            })?;
            nat_eq(a3308, a3309)
        } else {
            false
        }
    } else {
        false
    } {
        let a3311 = 0u64;
        let a3312 = match v5.clone() {
            Adt6::C0(h3310, _, _, _) => {
                h3310
            }
        };
        if if if if if nat_lt(a3311, a3312) {
            let a3314 = 0u64;
            let a3315 = match v5.clone() {
                Adt6::C0(_, h3313, _, _) => {
                    h3313
                }
            };
            nat_lt(a3314, a3315)
        } else {
            false
        } {
            let a3317 = match v5.clone() {
                Adt6::C0(_, h3316, _, _) => {
                    h3316
                }
            };
            let a3318 = 2147483648u64;
            nat_lt(a3317, a3318)
        } else {
            false
        } {
            let a3320 = 0u64;
            let a3321 = match v5.clone() {
                Adt6::C0(_, _, _, h3319) => {
                    h3319
                }
            };
            nat_lt(a3320, a3321)
        } else {
            false
        } {
            let a3323 = match v5.clone() {
                Adt6::C0(_, _, _, h3322) => {
                    h3322
                }
            };
            let a3324 = 2147483648u64;
            nat_lt(a3323, a3324)
        } else {
            false
        } {
            let a3327 = match v3.clone() {
                Adt2::C0(_, _, h3325, _, _, _, _, _, _, _, _) => {
                    h3325
                }
            };
            let a3328 = f93(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3326, _, _, _, _, _, _, _, _) => {
                    h3326
                }
            })?;
            nat_eq(a3327, a3328)
        } else {
            false
        }
    } else {
        false
    } {
        f217(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f217(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f217(v1.clone(), {
            let a3329 = v6;
            let a3330 = 0u64;
            if nat_eq(a3329, a3330) {
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
            let a3339 = {
                let a3337 = {
                    let a3335 = 3u64;
                    let a3336 = {
                        let a3333 = match v4.clone() {
                            Adt6::C0(h3331, _, _, _) => {
                                h3331
                            }
                        };
                        let a3334 = match v5.clone() {
                            Adt6::C0(h3332, _, _, _) => {
                                h3332
                            }
                        };
                        nat_mul(a3333, a3334)?
                    };
                    nat_mul(a3335, a3336)?
                };
                let a3338 = 2u64;
                nat_add(a3337, a3338)?
            };
            let a3340 = f23(v3.clone(), 1u64)?;
            nat_mul(a3339, a3340)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C34(f25(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f111(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
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
            let a3341 = 6u64;
            let a3342 = f23(v0.clone(), 1u64)?;
            nat_mul(a3341, a3342)?
        }, 2u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C33(f25(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3343 = v2;
    match m3343 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3344, _, _, _, _, _, _, _, _, _) => {
                    h3344
                }
            };
            let v6: u64 = {
                let a3350 = match v1.clone() {
                    Adt2::C0(_, _, h3345, _, _, _, _, _, _, _, _) => {
                        h3345
                    }
                };
                let a3351 = {
                    let a3348 = match v1.clone() {
                        Adt2::C0(_, _, _, h3346, _, _, _, _, _, _, _) => {
                            h3346
                        }
                    };
                    let a3349 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3347, _, _, _, _, _, _) => {
                            h3347
                        }
                    };
                    nat_mul(a3348, a3349)?
                };
                nat_mul(a3350, a3351)?
            };
            if if if if if if if if if if f21(v1.clone(), v4.clone()) {
                f21(v1.clone(), v3.clone())
            } else {
                false
            } {
                f218(v1.clone(), v6)?
            } else {
                false
            } {
                f218(v4.clone(), v6)?
            } else {
                false
            } {
                f218(v3.clone(), v6)?
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
                let a3352 = v5;
                let a3353 = 2147483647u64;
                nat_le(a3352, a3353)
            } else {
                false
            } {
                let a3354 = v6;
                let a3355 = 2147483647u64;
                nat_le(a3354, a3355)
            } else {
                false
            } {
                let a3357 = match v0.clone() {
                    Adt1::C0(h3356, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3356
                    }
                };
                let a3358 = 4294967295u64;
                if nat_le(a3357, a3358) {
                    let a3360 = match v0.clone() {
                        Adt1::C0(_, h3359, _, _, _, _, _, _, _, _, _, _) => {
                            h3359
                        }
                    };
                    let a3361 = 4294967295u64;
                    nat_le(a3360, a3361)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3362 = v5;
                    let a3363 = 0u64;
                    if nat_eq(a3362, a3363) {
                        0u64
                    } else {
                        let a3364 = v6;
                        let a3365 = f168(v5)?;
                        nat_mul(a3364, a3365)?
                    }
                }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3366, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3366
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f113(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3367, _, _, _, _, _, _, _, _, _, _, _) => {
            h3367
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3368, _, _, _, _, _, _, _, _, _, _) => {
            h3368
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3369, _, _, _, _, _, _, _, _, _) => {
            h3369
        }
    };
    let v6: u64 = {
        let a3371 = match v1.clone() {
            Adt2::C0(_, h3370, _, _, _, _, _, _, _, _, _) => {
                h3370
            }
        };
        let a3372 = v4;
        let a3373 = 0u64;
        nat_quot(a3371, a3372, a3373)
    };
    let v7: u64 = {
        let a3375 = match v1.clone() {
            Adt2::C0(h3374, _, _, _, _, _, _, _, _, _, _) => {
                h3374
            }
        };
        let a3376 = 0u64;
        if nat_eq(a3375, a3376) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f179(match v1.clone() {
        Adt2::C0(h3377, _, _, _, _, _, _, _, _, _, _) => {
            h3377
        }
    }) {
        let a3380 = match v1.clone() {
            Adt2::C0(h3378, _, _, _, _, _, _, _, _, _, _) => {
                h3378
            }
        };
        let a3381 = match v2.clone() {
            Adt2::C0(h3379, _, _, _, _, _, _, _, _, _, _) => {
                h3379
            }
        };
        nat_eq(a3380, a3381)
    } else {
        false
    } {
        let a3382 = 0u64;
        let a3383 = v3;
        nat_lt(a3382, a3383)
    } else {
        false
    } {
        let a3384 = v3;
        let a3385 = 2147483648u64;
        nat_lt(a3384, a3385)
    } else {
        false
    } {
        let a3386 = 0u64;
        let a3387 = v4;
        nat_lt(a3386, a3387)
    } else {
        false
    } {
        let a3388 = v4;
        let a3389 = 2147483648u64;
        nat_lt(a3388, a3389)
    } else {
        false
    } {
        let a3390 = v5;
        let a3391 = 2147483648u64;
        nat_lt(a3390, a3391)
    } else {
        false
    } {
        let a3392 = 0u64;
        let a3393 = v6;
        nat_lt(a3392, a3393)
    } else {
        false
    } {
        let a3395 = 0u64;
        let a3396 = match v2.clone() {
            Adt2::C0(_, h3394, _, _, _, _, _, _, _, _, _) => {
                h3394
            }
        };
        nat_lt(a3395, a3396)
    } else {
        false
    } {
        let a3400 = match v1.clone() {
            Adt2::C0(_, h3397, _, _, _, _, _, _, _, _, _) => {
                h3397
            }
        };
        let a3401 = {
            let a3398 = v6;
            let a3399 = v4;
            nat_mul(a3398, a3399)?
        };
        nat_eq(a3400, a3401)
    } else {
        false
    } {
        let a3403 = match v2.clone() {
            Adt2::C0(_, _, h3402, _, _, _, _, _, _, _, _) => {
                h3402
            }
        };
        let a3404 = v4;
        nat_eq(a3403, a3404)
    } else {
        false
    } {
        let a3417 = {
            let a3410 = {
                let a3408 = match v2.clone() {
                    Adt2::C0(_, h3405, _, _, _, _, _, _, _, _, _) => {
                        h3405
                    }
                };
                let a3409 = {
                    let a3406 = v5;
                    let a3407 = 2u64;
                    nat_mul(a3406, a3407)?
                };
                nat_add(a3408, a3409)?
            };
            let a3411 = v3;
            nat_add(a3410, a3411)?
        };
        let a3418 = {
            let a3415 = {
                let a3413 = match v1.clone() {
                    Adt2::C0(_, _, h3412, _, _, _, _, _, _, _, _) => {
                        h3412
                    }
                };
                let a3414 = v3;
                nat_mul(a3413, a3414)?
            };
            let a3416 = v6;
            nat_add(a3415, a3416)?
        };
        nat_eq(a3417, a3418)
    } else {
        false
    } {
        f219(v1.clone(), v7)?
    } else {
        false
    } {
        f219(v2.clone(), v7)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3419 = f23(v2.clone(), 1u64)?;
            let a3420 = f192(v8.clone())?;
            nat_mul(a3419, a3420)?
        }, v7, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3421, _, _, _, _, _, _, _, _, _, _) => {
                h3421
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3422 = v1;
    match m3422 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3424 = match v0.clone() {
                Adt2::C0(h3423, _, _, _, _, _, _, _, _, _, _) => {
                    h3423
                }
            };
            let a3425 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3424, a3425) {
                let a3427 = match v3.clone() {
                    Adt2::C0(h3426, _, _, _, _, _, _, _, _, _, _) => {
                        h3426
                    }
                };
                let a3428 = 0u64;
                nat_eq(a3427, a3428)
            } else {
                false
            } {
                let a3430 = match v2.clone() {
                    Adt2::C0(h3429, _, _, _, _, _, _, _, _, _, _) => {
                        h3429
                    }
                };
                let a3431 = 0u64;
                nat_eq(a3430, a3431)
            } else {
                false
            } {
                let a3433 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3432, _, _, _, _, _, _) => {
                        h3432
                    }
                };
                let a3434 = 1u64;
                nat_eq(a3433, a3434)
            } else {
                false
            } {
                let a3436 = match v3.clone() {
                    Adt2::C0(_, _, _, h3435, _, _, _, _, _, _, _) => {
                        h3435
                    }
                };
                let a3437 = 1u64;
                nat_eq(a3436, a3437)
            } else {
                false
            } {
                let a3439 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3438, _, _, _, _, _, _) => {
                        h3438
                    }
                };
                let a3440 = 1u64;
                nat_eq(a3439, a3440)
            } else {
                false
            } {
                let a3443 = match v0.clone() {
                    Adt2::C0(_, _, h3441, _, _, _, _, _, _, _, _) => {
                        h3441
                    }
                };
                let a3444 = match v3.clone() {
                    Adt2::C0(_, _, h3442, _, _, _, _, _, _, _, _) => {
                        h3442
                    }
                };
                nat_eq(a3443, a3444)
            } else {
                false
            } {
                let a3449 = match v3.clone() {
                    Adt2::C0(_, h3445, _, _, _, _, _, _, _, _, _) => {
                        h3445
                    }
                };
                let a3450 = {
                    let a3447 = match v0.clone() {
                        Adt2::C0(_, h3446, _, _, _, _, _, _, _, _, _) => {
                            h3446
                        }
                    };
                    let a3448 = 1u64;
                    nat_add(a3447, a3448)?
                };
                nat_le(a3449, a3450)
            } else {
                false
            } {
                let a3453 = match v2.clone() {
                    Adt2::C0(_, h3451, _, _, _, _, _, _, _, _, _) => {
                        h3451
                    }
                };
                let a3454 = match v0.clone() {
                    Adt2::C0(_, _, h3452, _, _, _, _, _, _, _, _) => {
                        h3452
                    }
                };
                nat_eq(a3453, a3454)
            } else {
                false
            } {
                let a3462 = match v2.clone() {
                    Adt2::C0(_, _, h3455, _, _, _, _, _, _, _, _) => {
                        h3455
                    }
                };
                let a3463 = {
                    let a3460 = {
                        let a3457 = match v0.clone() {
                            Adt2::C0(_, h3456, _, _, _, _, _, _, _, _, _) => {
                                h3456
                            }
                        };
                        let a3458 = 1u64;
                        nat_add(a3457, a3458)?
                    };
                    let a3461 = match v3.clone() {
                        Adt2::C0(_, h3459, _, _, _, _, _, _, _, _, _) => {
                            h3459
                        }
                    };
                    nat_sub(a3460, a3461)
                };
                nat_eq(a3462, a3463)
            } else {
                false
            } {
                let a3466 = match v2.clone() {
                    Adt2::C0(_, _, _, h3464, _, _, _, _, _, _, _) => {
                        h3464
                    }
                };
                let a3467 = match v0.clone() {
                    Adt2::C0(_, _, _, h3465, _, _, _, _, _, _, _) => {
                        h3465
                    }
                };
                nat_eq(a3466, a3467)
            } else {
                false
            } {
                let a3469 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3468, _, _, _, _, _, _) => {
                        h3468
                    }
                };
                let a3470 = 1u64;
                nat_eq(a3469, a3470)
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                let a3472 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3471, _, _, _, _, _) => {
                        h3471
                    }
                };
                let a3473 = 4u64;
                nat_eq(a3472, a3473)
            } else {
                false
            } {
                let a3475 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3474, _, _, _, _, _) => {
                        h3474
                    }
                };
                let a3476 = 4u64;
                nat_eq(a3475, a3476)
            } else {
                false
            } {
                let a3481 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3477, _, _, _, _) => {
                        h3477
                    }
                };
                let a3482 = {
                    let a3479 = match v0.clone() {
                        Adt2::C0(_, h3478, _, _, _, _, _, _, _, _, _) => {
                            h3478
                        }
                    };
                    let a3480 = 4u64;
                    nat_mul(a3479, a3480)?
                };
                nat_eq(a3481, a3482)
            } else {
                false
            } {
                let a3487 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3483, _, _, _, _) => {
                        h3483
                    }
                };
                let a3488 = {
                    let a3485 = match v3.clone() {
                        Adt2::C0(_, h3484, _, _, _, _, _, _, _, _, _) => {
                            h3484
                        }
                    };
                    let a3486 = 4u64;
                    nat_mul(a3485, a3486)?
                };
                nat_eq(a3487, a3488)
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
                    let a3490 = f23(v2.clone(), 1u64)?;
                    let a3491 = f191(match v3.clone() {
                        Adt2::C0(_, h3489, _, _, _, _, _, _, _, _, _) => {
                            h3489
                        }
                    })?;
                    nat_mul(a3490, a3491)?
                }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C29(f25(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f115(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h3492, _, _, _, _, _, _, _, _, _, _) => {
            h3492
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h3493, _, _, _, _, _, _, _, _) => {
                h3493
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h3494, _, _, _, _, _, _, _, _, _) => {
                h3494
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h3495, _, _, _, _, _, _) => {
                h3495
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h3496, _, _, _, _, _, _, _, _) => {
                h3496
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h3497, _, _, _, _, _, _, _, _, _) => {
                h3497
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h3498, _, _, _, _, _, _, _) => {
                h3498
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h3499, _, _, _, _, _) => {
                h3499
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3501 = match v2.clone() {
        Adt1::C0(h3500, _, _, _, _, _, _, _, _, _, _, _) => {
            h3500
        }
    };
    let a3502 = 1u64;
    if if if if if if if if if if if nat_le(a3501, a3502) {
        let a3504 = match v3.clone() {
            Adt2::C0(h3503, _, _, _, _, _, _, _, _, _, _) => {
                h3503
            }
        };
        let a3505 = 1u64;
        nat_le(a3504, a3505)
    } else {
        false
    } {
        let a3507 = match v4.clone() {
            Adt2::C0(h3506, _, _, _, _, _, _, _, _, _, _) => {
                h3506
            }
        };
        let a3508 = 0u64;
        nat_eq(a3507, a3508)
    } else {
        false
    } {
        let a3513 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h3509, _, _, _, _, _) => {
                h3509
            }
        };
        let a3514 = {
            let a3511 = match v3.clone() {
                Adt2::C0(h3510, _, _, _, _, _, _, _, _, _, _) => {
                    h3510
                }
            };
            let a3512 = 0u64;
            if nat_eq(a3511, a3512) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3513, a3514)
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
        f220(v5.clone(), match v3.clone() {
            Adt2::C0(_, h3515, _, _, _, _, _, _, _, _, _) => {
                h3515
            }
        }, match v4.clone() {
            Adt2::C0(_, h3516, _, _, _, _, _, _, _, _, _) => {
                h3516
            }
        })?
    } else {
        false
    } {
        if v1 {
            f220(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h3517, _, _, _, _, _, _, _, _) => {
                    h3517
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h3518, _, _, _, _, _, _, _, _) => {
                    h3518
                }
            })?
        } else {
            let a3521 = match v3.clone() {
                Adt2::C0(_, _, h3519, _, _, _, _, _, _, _, _) => {
                    h3519
                }
            };
            let a3522 = match v4.clone() {
                Adt2::C0(_, _, h3520, _, _, _, _, _, _, _, _) => {
                    h3520
                }
            };
            nat_eq(a3521, a3522)
        }
    } else {
        false
    } {
        let a3525 = match v3.clone() {
            Adt2::C0(_, _, _, h3523, _, _, _, _, _, _, _) => {
                h3523
            }
        };
        let a3526 = match v4.clone() {
            Adt2::C0(_, _, _, h3524, _, _, _, _, _, _, _) => {
                h3524
            }
        };
        nat_eq(a3525, a3526)
    } else {
        false
    } {
        let a3529 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3527, _, _, _, _, _, _) => {
                h3527
            }
        };
        let a3530 = match v4.clone() {
            Adt2::C0(_, _, _, _, h3528, _, _, _, _, _, _) => {
                h3528
            }
        };
        nat_eq(a3529, a3530)
    } else {
        false
    } {
        let a3535 = {
            let a3533 = match v5.clone() {
                Adt6::C0(h3531, _, _, _) => {
                    h3531
                }
            };
            let a3534 = match v6.clone() {
                Adt6::C0(h3532, _, _, _) => {
                    h3532
                }
            };
            nat_mul(a3533, a3534)?
        };
        let a3536 = 2147483648u64;
        nat_lt(a3535, a3536)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a3538 = match v2 {
                Adt1::C0(h3537, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3537
                }
            };
            let a3539 = 0u64;
            nat_eq(a3538, a3539)
        }, match v3.clone() {
            Adt2::C0(h3540, _, _, _, _, _, _, _, _, _, _) => {
                h3540
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a3552 = f23(v3.clone(), 1u64)?;
                let a3553 = {
                    let a3550 = f23(v4.clone(), 1u64)?;
                    let a3551 = f172(match v7.clone() {
                        Adt7::C0(_, _, _, h3541, _) => {
                            h3541
                        }
                    }, {
                        let a3548 = {
                            let m3544 = match v7.clone() {
                                Adt7::C0(h3543, _, _, _, _) => {
                                    h3543
                                }
                            };
                            match m3544 {
                                Adt6::C0(h3542, _, _, _) => {
                                    h3542
                                }
                            }
                        };
                        let a3549 = {
                            let m3547 = match v7.clone() {
                                Adt7::C0(_, h3546, _, _, _) => {
                                    h3546
                                }
                            };
                            match m3547 {
                                Adt6::C0(h3545, _, _, _) => {
                                    h3545
                                }
                            }
                        };
                        nat_mul(a3548, a3549)?
                    })?;
                    nat_mul(a3550, a3551)?
                };
                nat_add(a3552, a3553)?
            }, 4u64, f25(v4, 1u64), f25(v3.clone(), 1u64), Adt4::C43(v7, f23(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a3554 = f23(v4.clone(), 1u64)?;
                let a3555 = f188(v7.clone())?;
                nat_mul(a3554, a3555)?
            }, 4u64, f25(v3, 1u64), f25(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3556 = v3;
    match m3556 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3558 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3557, _, _, _, _, _) => {
                        h3557
                    }
                };
                let a3559 = 1u64;
                nat_eq(a3558, a3559)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h3560, _, _, _, _, _, _, _, _, _) => {
                    h3560
                }
            }, match v1.clone() {
                Adt1::C0(h3561, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3561
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3562, _, _, _, _, _, _, _, _, _) => {
                    h3562
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h3563, _, _, _, _, _, _, _) => {
                    h3563
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h3564, _, _, _, _, _, _, _, _) => {
                        h3564
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h3565, _, _, _, _, _, _, _, _, _, _) => {
                        h3565
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3566, _, _, _, _, _, _, _, _) => {
                        h3566
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3567, _, _, _, _, _, _) => {
                        h3567
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3568, _, _, _, _, _, _, _) => {
                        h3568
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h3569, _, _, _, _, _, _, _, _) => {
                        h3569
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h3570, _, _, _, _, _, _) => {
                        h3570
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3571, _, _, _, _, _, _, _) => {
                        h3571
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h3572, _, _, _, _, _, _, _, _) => {
                        h3572
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h3573, _, _, _, _, _, _, _, _, _, _) => {
                    h3573
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h3574, _, _, _, _, _, _, _, _, _, _) => {
                    h3574
                }
            };
            let v15: u64 = {
                let a3575 = v13;
                let a3576 = 0u64;
                if nat_eq(a3575, a3576) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a3578 = match v5.clone() {
                    Adt2::C0(_, h3577, _, _, _, _, _, _, _, _, _) => {
                        h3577
                    }
                };
                let a3579 = v15;
                nat_mul(a3578, a3579)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3580, _, _, _) => {
                        h3580
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3581, _, _, _, _) => {
                        h3581
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3582, _, _) => {
                        h3582
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3583, _, _, _) => {
                        h3583
                    }
                }
            };
            let a3585 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h3584, _, _, _, _, _) => {
                    h3584
                }
            };
            let a3586 = 1u64;
            if if if if if if if if if if nat_le(a3585, a3586) {
                let a3587 = v13;
                let a3588 = 0u64;
                if nat_eq(a3587, a3588) {
                    true
                } else {
                    let a3589 = v13;
                    let a3590 = 1u64;
                    if nat_eq(a3589, a3590) {
                        let a3591 = v14;
                        let a3592 = 1u64;
                        nat_eq(a3591, a3592)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3593 = v14;
                let a3594 = 0u64;
                if nat_eq(a3593, a3594) {
                    true
                } else {
                    let a3595 = v14;
                    let a3596 = 1u64;
                    nat_eq(a3595, a3596)
                }
            } else {
                false
            } {
                let a3598 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3597, _, _, _, _, _) => {
                        h3597
                    }
                };
                let a3599 = v15;
                nat_eq(a3598, a3599)
            } else {
                false
            } {
                let a3607 = match v4.clone() {
                    Adt2::C0(_, h3600, _, _, _, _, _, _, _, _, _) => {
                        h3600
                    }
                };
                let a3608 = {
                    let a3605 = v10;
                    let a3606 = {
                        let a3603 = match v7.clone() {
                            Adt6::C0(h3601, _, _, _) => {
                                h3601
                            }
                        };
                        let a3604 = match v8.clone() {
                            Adt6::C0(h3602, _, _, _) => {
                                h3602
                            }
                        };
                        nat_mul(a3603, a3604)?
                    };
                    nat_mul(a3605, a3606)?
                };
                nat_eq(a3607, a3608)
            } else {
                false
            } {
                let a3611 = match v4.clone() {
                    Adt2::C0(_, _, h3609, _, _, _, _, _, _, _, _) => {
                        h3609
                    }
                };
                let a3612 = f93(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h3610, _, _, _, _, _, _, _, _, _) => {
                        h3610
                    }
                })?;
                nat_eq(a3611, a3612)
            } else {
                false
            } {
                let a3614 = 0u64;
                let a3615 = match v4.clone() {
                    Adt2::C0(_, _, h3613, _, _, _, _, _, _, _, _) => {
                        h3613
                    }
                };
                nat_lt(a3614, a3615)
            } else {
                false
            } {
                if v6 {
                    let a3617 = match v2 {
                        Adt2::C0(_, _, _, h3616, _, _, _, _, _, _, _) => {
                            h3616
                        }
                    };
                    let a3618 = v10;
                    if if if nat_eq(a3617, a3618) {
                        let a3620 = match v4.clone() {
                            Adt2::C0(_, _, _, h3619, _, _, _, _, _, _, _) => {
                                h3619
                            }
                        };
                        let a3621 = f93(v8.clone(), v12)?;
                        nat_eq(a3620, a3621)
                    } else {
                        false
                    } {
                        let a3623 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3622, _, _, _, _, _, _) => {
                                h3622
                            }
                        };
                        let a3624 = v11;
                        nat_eq(a3623, a3624)
                    } else {
                        false
                    } {
                        let a3626 = 0u64;
                        let a3627 = match v4.clone() {
                            Adt2::C0(_, _, _, h3625, _, _, _, _, _, _, _) => {
                                h3625
                            }
                        };
                        nat_lt(a3626, a3627)
                    } else {
                        false
                    }
                } else {
                    let a3629 = match v2 {
                        Adt2::C0(_, _, h3628, _, _, _, _, _, _, _, _) => {
                            h3628
                        }
                    };
                    let a3630 = v10;
                    if if if nat_eq(a3629, a3630) {
                        let a3632 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h3631, _, _, _, _, _, _) => {
                                h3631
                            }
                        };
                        let a3633 = 1u64;
                        nat_eq(a3632, a3633)
                    } else {
                        false
                    } {
                        let a3635 = match v4.clone() {
                            Adt2::C0(_, _, _, h3634, _, _, _, _, _, _, _) => {
                                h3634
                            }
                        };
                        let a3636 = v11;
                        nat_eq(a3635, a3636)
                    } else {
                        false
                    } {
                        let a3638 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3637, _, _, _, _, _, _) => {
                                h3637
                            }
                        };
                        let a3639 = 1u64;
                        nat_eq(a3638, a3639)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f48(match v5.clone() {
                    Adt2::C0(_, h3640, _, _, _, _, _, _, _, _, _) => {
                        h3640
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a3660 = {
                        let a3657 = {
                            let a3655 = {
                                let a3649 = {
                                    let a3643 = {
                                        let a3641 = v11;
                                        let a3642 = 1u64;
                                        nat_sub(a3641, a3642)
                                    };
                                    let a3644 = v18;
                                    nat_mul(a3643, a3644)?
                                };
                                let a3650 = {
                                    let a3647 = {
                                        let a3645 = v10;
                                        let a3646 = 1u64;
                                        nat_sub(a3645, a3646)
                                    };
                                    let a3648 = v17;
                                    nat_mul(a3647, a3648)?
                                };
                                nat_add(a3649, a3650)?
                            };
                            let a3656 = {
                                let a3653 = {
                                    let a3651 = v12;
                                    let a3652 = 1u64;
                                    nat_sub(a3651, a3652)
                                };
                                let a3654 = v16;
                                nat_mul(a3653, a3654)?
                            };
                            nat_add(a3655, a3656)?
                        };
                        let a3658 = v16;
                        nat_add(a3657, a3658)?
                    };
                    let a3661 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3659, _) => {
                            h3659
                        }
                    };
                    nat_le(a3660, a3661)
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
                        Adt2::C0(_, h3662, _, _, _, _, _, _, _, _, _) => {
                            h3662
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3663, _, _, _, _, _, _, _, _) => {
                            h3663
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h3664, _, _, _, _, _, _, _) => {
                            h3664
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h3665, _, _, _, _, _) => {
                            h3665
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3666, _, _, _, _) => {
                            h3666
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3667, _, _, _) => {
                            h3667
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a3674 = f23(v5.clone(), 1u64)?;
                        let a3675 = {
                            let a3672 = {
                                let a3670 = match v7.clone() {
                                    Adt6::C0(h3668, _, _, _) => {
                                        h3668
                                    }
                                };
                                let a3671 = match v8.clone() {
                                    Adt6::C0(h3669, _, _, _) => {
                                        h3669
                                    }
                                };
                                nat_mul(a3670, a3671)?
                            };
                            let a3673 = 2u64;
                            nat_add(a3672, a3673)?
                        };
                        nat_mul(a3674, a3675)?
                    }, 4u64, v19, f37(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f23(v4, 1u64)?, {
                        let a3676 = v14;
                        let a3677 = 0u64;
                        if nat_eq(a3676, a3677) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h3678, _, _, _, _, _, _, _, _, _) => {
                            h3678
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f117(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3679 = v2;
    match m3679 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3680, _, _, _, _, _, _, _, _, _) => {
                    h3680
                }
            }, match v0.clone() {
                Adt1::C0(h3681, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3681
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3682, _, _, _, _, _, _, _, _) => {
                    h3682
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3683, _, _, _, _, _) => {
                    h3683
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3684, _, _, _, _, _, _, _, _) => {
                    h3684
                }
            }, match v0.clone() {
                Adt1::C0(_, h3685, _, _, _, _, _, _, _, _, _, _) => {
                    h3685
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3686, _, _, _, _, _, _, _) => {
                    h3686
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3687, _, _, _, _) => {
                    h3687
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3688, _, _, _, _, _, _, _) => {
                    h3688
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3689, _, _, _, _, _, _, _, _, _) => {
                    h3689
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3690, _, _, _, _, _, _) => {
                    h3690
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3691, _, _, _) => {
                    h3691
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3692, _, _) => {
                    h3692
                }
            };
            let v9: u64 = f93(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3693, _, _, _, _, _, _, _) => {
                    h3693
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3694, _, _, _, _, _, _, _, _, _, _) => {
                    h3694
                }
            };
            let v11: u64 = {
                let a3702 = v8;
                let a3703 = {
                    let a3700 = match v5.clone() {
                        Adt6::C0(h3695, _, _, _) => {
                            h3695
                        }
                    };
                    let a3701 = {
                        let a3698 = match v6.clone() {
                            Adt6::C0(h3696, _, _, _) => {
                                h3696
                            }
                        };
                        let a3699 = match v7.clone() {
                            Adt6::C0(h3697, _, _, _) => {
                                h3697
                            }
                        };
                        nat_mul(a3698, a3699)?
                    };
                    nat_mul(a3700, a3701)?
                };
                nat_mul(a3702, a3703)?
            };
            let a3705 = match v4.clone() {
                Adt2::C0(h3704, _, _, _, _, _, _, _, _, _, _) => {
                    h3704
                }
            };
            let a3706 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3705, a3706) {
                let a3707 = v10;
                let a3708 = 0u64;
                if nat_eq(a3707, a3708) {
                    true
                } else {
                    let a3709 = v10;
                    let a3710 = 1u64;
                    nat_eq(a3709, a3710)
                }
            } else {
                false
            } {
                let a3711 = 0u64;
                let a3712 = v8;
                nat_lt(a3711, a3712)
            } else {
                false
            } {
                let a3713 = v8;
                let a3714 = 2147483648u64;
                nat_lt(a3713, a3714)
            } else {
                false
            } {
                let a3716 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3715, _, _, _, _, _) => {
                        h3715
                    }
                };
                let a3717 = 4u64;
                nat_eq(a3716, a3717)
            } else {
                false
            } {
                let a3718 = 0u64;
                let a3719 = v9;
                nat_lt(a3718, a3719)
            } else {
                false
            } {
                let a3721 = 0u64;
                let a3722 = match v3.clone() {
                    Adt2::C0(_, _, h3720, _, _, _, _, _, _, _, _) => {
                        h3720
                    }
                };
                nat_lt(a3721, a3722)
            } else {
                false
            } {
                let a3724 = 0u64;
                let a3725 = match v3.clone() {
                    Adt2::C0(_, _, _, h3723, _, _, _, _, _, _, _) => {
                        h3723
                    }
                };
                nat_lt(a3724, a3725)
            } else {
                false
            } {
                let a3727 = match v3.clone() {
                    Adt2::C0(_, h3726, _, _, _, _, _, _, _, _, _) => {
                        h3726
                    }
                };
                let a3728 = v11;
                nat_eq(a3727, a3728)
            } else {
                false
            } {
                let a3731 = match v3.clone() {
                    Adt2::C0(_, _, h3729, _, _, _, _, _, _, _, _) => {
                        h3729
                    }
                };
                let a3732 = f93(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3730, _, _, _, _, _, _, _, _, _) => {
                        h3730
                    }
                })?;
                nat_eq(a3731, a3732)
            } else {
                false
            } {
                let a3735 = match v3.clone() {
                    Adt2::C0(_, _, _, h3733, _, _, _, _, _, _, _) => {
                        h3733
                    }
                };
                let a3736 = f93(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3734, _, _, _, _, _, _, _, _) => {
                        h3734
                    }
                })?;
                nat_eq(a3735, a3736)
            } else {
                false
            } {
                let a3744 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3737, _, _, _, _, _, _) => {
                        h3737
                    }
                };
                let a3745 = {
                    let a3742 = v9;
                    let a3743 = {
                        let a3739 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3738, _, _, _, _, _, _) => {
                                h3738
                            }
                        };
                        let a3740 = v8;
                        let a3741 = 0u64;
                        nat_quot(a3739, a3740, a3741)
                    };
                    nat_mul(a3742, a3743)?
                };
                nat_eq(a3744, a3745)
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3746 = v10;
                    let a3747 = 0u64;
                    if nat_eq(a3746, a3747) {
                        4u64
                    } else {
                        2u64
                    }
                }, f25(v4.clone(), 1u64), f25(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3748, _, _) => {
                        h3748
                    }
                }, {
                    let a3750 = v8;
                    let a3751 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3749, _, _) => {
                            h3749
                        }
                    };
                    nat_mul(a3750, a3751)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f118(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3752 = v1;
    match m3752 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f221(v0, v3, v2)
        }
    }
}

pub fn f119(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3754 = match v0.clone() {
        Adt2::C0(h3753, _, _, _, _, _, _, _, _, _, _) => {
            h3753
        }
    };
    let a3755 = 1u64;
    if if nat_eq(a3754, a3755) {
        true
    } else {
        let a3757 = match v0.clone() {
            Adt2::C0(h3756, _, _, _, _, _, _, _, _, _, _) => {
                h3756
            }
        };
        let a3758 = 30u64;
        nat_eq(a3757, a3758)
    } {
        let a3760 = match v1.clone() {
            Adt2::C0(h3759, _, _, _, _, _, _, _, _, _, _) => {
                h3759
            }
        };
        let a3761 = 1u64;
        if if if if if if if if if if if nat_eq(a3760, a3761) {
            let a3764 = match v0.clone() {
                Adt2::C0(_, h3762, _, _, _, _, _, _, _, _, _) => {
                    h3762
                }
            };
            let a3765 = match v1.clone() {
                Adt2::C0(_, h3763, _, _, _, _, _, _, _, _, _) => {
                    h3763
                }
            };
            nat_eq(a3764, a3765)
        } else {
            false
        } {
            let a3767 = 0u64;
            let a3768 = match v1.clone() {
                Adt2::C0(_, _, h3766, _, _, _, _, _, _, _, _) => {
                    h3766
                }
            };
            nat_lt(a3767, a3768)
        } else {
            false
        } {
            let a3771 = match v1.clone() {
                Adt2::C0(_, _, h3769, _, _, _, _, _, _, _, _) => {
                    h3769
                }
            };
            let a3772 = match v1.clone() {
                Adt2::C0(_, _, _, h3770, _, _, _, _, _, _, _) => {
                    h3770
                }
            };
            nat_eq(a3771, a3772)
        } else {
            false
        } {
            let a3774 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3773, _, _, _, _, _, _) => {
                    h3773
                }
            };
            let a3775 = 1u64;
            nat_eq(a3774, a3775)
        } else {
            false
        } {
            let a3783 = match v0.clone() {
                Adt2::C0(_, _, h3776, _, _, _, _, _, _, _, _) => {
                    h3776
                }
            };
            let a3784 = {
                let a3781 = {
                    let a3779 = match v1.clone() {
                        Adt2::C0(_, _, h3777, _, _, _, _, _, _, _, _) => {
                            h3777
                        }
                    };
                    let a3780 = match v1.clone() {
                        Adt2::C0(_, _, h3778, _, _, _, _, _, _, _, _) => {
                            h3778
                        }
                    };
                    nat_add(a3779, a3780)?
                };
                let a3782 = 1u64;
                nat_sub(a3781, a3782)
            };
            nat_eq(a3783, a3784)
        } else {
            false
        } {
            let a3786 = 0u64;
            let a3787 = match v0.clone() {
                Adt2::C0(_, _, _, h3785, _, _, _, _, _, _, _) => {
                    h3785
                }
            };
            nat_lt(a3786, a3787)
        } else {
            false
        } {
            let a3789 = 0u64;
            let a3790 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3788, _, _, _, _, _, _) => {
                    h3788
                }
            };
            nat_lt(a3789, a3790)
        } else {
            false
        } {
            let a3792 = 2u64;
            let a3793 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3791, _, _, _, _, _) => {
                    h3791
                }
            };
            nat_le(a3792, a3793)
        } else {
            false
        } {
            let a3795 = 2u64;
            let a3796 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3794, _, _, _, _, _) => {
                    h3794
                }
            };
            nat_le(a3795, a3796)
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

pub fn f120(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3797 = v1;
    match m3797 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f222(v0, v3, v2)
        }
    }
}

pub fn f121(v0: u64) -> bool {
    let a3798 = v0;
    let a3799 = 1u64;
    if if nat_eq(a3798, a3799) {
        true
    } else {
        let a3800 = v0;
        let a3801 = 33u64;
        nat_eq(a3800, a3801)
    } {
        true
    } else {
        let a3802 = v0;
        let a3803 = 34u64;
        nat_eq(a3802, a3803)
    }
}

pub fn f122(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f47(v1, v2.clone()) {
        let a3804 = v3;
        let a3805 = f23(v2.clone(), 1u64)?;
        nat_eq(a3804, a3805)
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
        let a3806 = v3;
        let a3807 = 0u64;
        if nat_eq(a3806, a3807) {
            true
        } else {
            let a3809 = 4u64;
            let a3810 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3808, _, _, _, _, _) => {
                    h3808
                }
            };
            nat_le(a3809, a3810)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3811 = 2u64;
            let a3812 = v3;
            nat_mul(a3811, a3812)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f123(v0: u64) -> bool {
    if f223(v0) {
        !f224(v0)
    } else {
        false
    }
}

pub fn f124(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3813 = v1;
    match m3813 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3815 = f216(match v0.clone() {
                Adt2::C0(h3814, _, _, _, _, _, _, _, _, _, _) => {
                    h3814
                }
            });
            match m3815 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3816, _) => {
                            h3816
                        }
                    };
                    if if if if if if f47(v3, v2.clone()) {
                        f30(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3817 = f23(v0.clone(), 1u64)?;
                        let a3818 = f23(v2.clone(), 1u64)?;
                        nat_eq(a3817, a3818)
                    } else {
                        false
                    } {
                        let a3823 = {
                            let a3820 = match v0.clone() {
                                Adt2::C0(_, h3819, _, _, _, _, _, _, _, _, _) => {
                                    h3819
                                }
                            };
                            let a3821 = v5;
                            let a3822 = 1u64;
                            nat_rem(a3820, a3821, a3822)
                        };
                        let a3824 = 0u64;
                        if nat_eq(a3823, a3824) {
                            let a3827 = match v4 {
                                Adt19::C0(_, _, h3825) => {
                                    h3825
                                }
                            };
                            let a3828 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3826, _, _, _, _, _) => {
                                    h3826
                                }
                            };
                            nat_le(a3827, a3828)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3829 = v5;
                        let a3830 = 1u64;
                        if nat_eq(a3829, a3830) {
                            let a3832 = 4u64;
                            let a3833 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3831, _, _, _, _, _) => {
                                    h3831
                                }
                            };
                            nat_le(a3832, a3833)
                        } else {
                            let a3835 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3834, _, _, _, _, _) => {
                                    h3834
                                }
                            };
                            let a3836 = 4u64;
                            nat_eq(a3835, a3836)
                        }
                    } else {
                        false
                    } {
                        let a3841 = {
                            let a3838 = match v2.clone() {
                                Adt2::C0(_, h3837, _, _, _, _, _, _, _, _, _) => {
                                    h3837
                                }
                            };
                            let a3839 = v5;
                            let a3840 = 1u64;
                            nat_rem(a3838, a3839, a3840)
                        };
                        let a3842 = 0u64;
                        if nat_eq(a3841, a3842) {
                            true
                        } else {
                            f38(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f25(v0.clone(), v5), f25(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3843, _, _, _, _, _, _, _, _, _, _) => {
                                h3843
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

pub fn f125(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3846 = match v1.clone() {
        Adt2::C0(h3844, _, _, _, _, _, _, _, _, _, _) => {
            h3844
        }
    };
    let a3847 = match v3.clone() {
        Adt2::C0(h3845, _, _, _, _, _, _, _, _, _, _) => {
            h3845
        }
    };
    if nat_eq(a3846, a3847) {
        f225(v0, v1, v2, v3)
    } else {
        if if if if f224(match v1.clone() {
            Adt2::C0(h3848, _, _, _, _, _, _, _, _, _, _) => {
                h3848
            }
        }) {
            f224(match v3.clone() {
                Adt2::C0(h3849, _, _, _, _, _, _, _, _, _, _) => {
                    h3849
                }
            })
        } else {
            false
        } {
            let a3850 = v0;
            let a3851 = 33u64;
            if nat_eq(a3850, a3851) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3852 = f23(v1.clone(), 1u64)?;
            let a3853 = f23(v3.clone(), 1u64)?;
            nat_eq(a3852, a3853)
        } else {
            false
        } {
            let m3855 = f216(match v3.clone() {
                Adt2::C0(h3854, _, _, _, _, _, _, _, _, _, _) => {
                    h3854
                }
            });
            match m3855 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3856, _, _, _, _, _, _, _, _, _, _) => {
                            h3856
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3857, _, _, _, _, _, _, _, _, _, _) => {
                            h3857
                        }
                    };
                    let v7: bool = {
                        let a3858 = v5;
                        let a3859 = 43u64;
                        if nat_eq(a3858, a3859) {
                            true
                        } else {
                            let a3860 = v6;
                            let a3861 = 43u64;
                            nat_eq(a3860, a3861)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3862 = f23(v3.clone(), 1u64)?;
                        let a3863 = if v7 {
                            f166(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3862, a3863)?
                    }, match v4 {
                        Adt19::C0(_, _, h3864) => {
                            h3864
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

pub fn f126(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3866 = match v0.clone() {
        Adt2::C0(h3865, _, _, _, _, _, _, _, _, _, _) => {
            h3865
        }
    };
    let a3867 = 0u64;
    if if if if nat_eq(a3866, a3867) {
        true
    } else {
        let a3869 = match v0.clone() {
            Adt2::C0(h3868, _, _, _, _, _, _, _, _, _, _) => {
                h3868
            }
        };
        let a3870 = 1u64;
        nat_eq(a3869, a3870)
    } {
        true
    } else {
        let a3872 = match v0.clone() {
            Adt2::C0(h3871, _, _, _, _, _, _, _, _, _, _) => {
                h3871
            }
        };
        let a3873 = 25u64;
        if nat_eq(a3872, a3873) {
            true
        } else {
            let a3875 = match v0.clone() {
                Adt2::C0(h3874, _, _, _, _, _, _, _, _, _, _) => {
                    h3874
                }
            };
            let a3876 = 26u64;
            nat_eq(a3875, a3876)
        }
    } {
        true
    } else {
        let a3878 = match v0.clone() {
            Adt2::C0(h3877, _, _, _, _, _, _, _, _, _, _) => {
                h3877
            }
        };
        let a3879 = 30u64;
        nat_eq(a3878, a3879)
    } {
        let a3882 = match v0.clone() {
            Adt2::C0(h3880, _, _, _, _, _, _, _, _, _, _) => {
                h3880
            }
        };
        let a3883 = match v1.clone() {
            Adt2::C0(h3881, _, _, _, _, _, _, _, _, _, _) => {
                h3881
            }
        };
        if if nat_eq(a3882, a3883) {
            f226(match v1.clone() {
                Adt2::C0(_, h3884, _, _, _, _, _, _, _, _, _) => {
                    h3884
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3885, _, _, _, _, _, _, _, _) => {
                    h3885
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3886, _, _, _, _, _, _, _) => {
                    h3886
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3887, _, _, _, _, _, _) => {
                    h3887
                }
            }, match v0.clone() {
                Adt2::C0(_, h3888, _, _, _, _, _, _, _, _, _) => {
                    h3888
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3889, _, _, _, _, _, _, _, _) => {
                    h3889
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3890, _, _, _, _, _, _, _) => {
                    h3890
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3891, _, _, _, _, _, _) => {
                    h3891
                }
            })
        } else {
            false
        } {
            let m3893 = f216(match v0.clone() {
                Adt2::C0(h3892, _, _, _, _, _, _, _, _, _, _) => {
                    h3892
                }
            });
            match m3893 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3894) => {
                            h3894
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

pub fn f127(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3895 = v2;
    match m3895 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3896 = v0;
            let a3897 = 4u64;
            if if if nat_lt(a3896, a3897) {
                let a3900 = match v1.clone() {
                    Adt2::C0(h3898, _, _, _, _, _, _, _, _, _, _) => {
                        h3898
                    }
                };
                let a3901 = match v4.clone() {
                    Adt2::C0(h3899, _, _, _, _, _, _, _, _, _, _) => {
                        h3899
                    }
                };
                if nat_eq(a3900, a3901) {
                    let a3904 = match v1.clone() {
                        Adt2::C0(h3902, _, _, _, _, _, _, _, _, _, _) => {
                            h3902
                        }
                    };
                    let a3905 = match v3.clone() {
                        Adt2::C0(h3903, _, _, _, _, _, _, _, _, _, _) => {
                            h3903
                        }
                    };
                    nat_eq(a3904, a3905)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f227(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3906, _, _, _, _, _, _, _, _, _) => {
                        h3906
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3907, _, _, _, _, _, _, _, _, _) => {
                        h3907
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3908, _, _, _, _, _, _, _, _, _) => {
                        h3908
                    }
                })? {
                    f227(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3909, _, _, _, _, _, _, _, _) => {
                            h3909
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3910, _, _, _, _, _, _, _, _) => {
                            h3910
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3911, _, _, _, _, _, _, _, _) => {
                            h3911
                        }
                    })?
                } else {
                    false
                } {
                    f227(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3912, _, _, _, _, _, _, _) => {
                            h3912
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3913, _, _, _, _, _, _, _) => {
                            h3913
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3914, _, _, _, _, _, _, _) => {
                            h3914
                        }
                    })?
                } else {
                    false
                } {
                    f227(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3915, _, _, _, _, _, _) => {
                            h3915
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3916, _, _, _, _, _, _) => {
                            h3916
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3917, _, _, _, _, _, _) => {
                            h3917
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3919 = f216(match v1.clone() {
                    Adt2::C0(h3918, _, _, _, _, _, _, _, _, _, _) => {
                        h3918
                    }
                });
                match m3919 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3920, _) => {
                                h3920
                            }
                        };
                        let a3925 = {
                            let a3922 = match v1.clone() {
                                Adt2::C0(_, h3921, _, _, _, _, _, _, _, _, _) => {
                                    h3921
                                }
                            };
                            let a3923 = v6;
                            let a3924 = 1u64;
                            nat_rem(a3922, a3923, a3924)
                        };
                        let a3926 = 0u64;
                        if if if nat_eq(a3925, a3926) {
                            let a3931 = {
                                let a3928 = match v4.clone() {
                                    Adt2::C0(_, h3927, _, _, _, _, _, _, _, _, _) => {
                                        h3927
                                    }
                                };
                                let a3929 = v6;
                                let a3930 = 1u64;
                                nat_rem(a3928, a3929, a3930)
                            };
                            let a3932 = 0u64;
                            nat_eq(a3931, a3932)
                        } else {
                            false
                        } {
                            let a3937 = {
                                let a3934 = match v3.clone() {
                                    Adt2::C0(_, h3933, _, _, _, _, _, _, _, _, _) => {
                                        h3933
                                    }
                                };
                                let a3935 = v6;
                                let a3936 = 1u64;
                                nat_rem(a3934, a3935, a3936)
                            };
                            let a3938 = 0u64;
                            nat_eq(a3937, a3938)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3939) => {
                                    h3939
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

pub fn f128(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3941 = match v2.clone() {
        Adt2::C0(h3940, _, _, _, _, _, _, _, _, _, _) => {
            h3940
        }
    };
    let a3942 = 0u64;
    if if nat_eq(a3941, a3942) {
        let a3944 = match v3.clone() {
            Adt2::C0(h3943, _, _, _, _, _, _, _, _, _, _) => {
                h3943
            }
        };
        let a3945 = 0u64;
        nat_eq(a3944, a3945)
    } else {
        false
    } {
        let a3946 = v0;
        let a3947 = 61u64;
        if nat_eq(a3946, a3947) {
            let v4: bool = {
                let a3949 = 0u64;
                let a3950 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3948, _, _, _) => {
                        h3948
                    }
                };
                nat_lt(a3949, a3950)
            };
            if if if if if if f228(match v2.clone() {
                Adt2::C0(_, h3951, _, _, _, _, _, _, _, _, _) => {
                    h3951
                }
            }, match v1.clone() {
                Adt1::C0(h3952, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3952
                }
            }, match v1.clone() {
                Adt1::C0(_, h3953, _, _, _, _, _, _, _, _, _, _) => {
                    h3953
                }
            }, match v3.clone() {
                Adt2::C0(_, h3954, _, _, _, _, _, _, _, _, _) => {
                    h3954
                }
            })? {
                f228(match v2.clone() {
                    Adt2::C0(_, _, h3955, _, _, _, _, _, _, _, _) => {
                        h3955
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3956, _, _, _, _, _, _, _, _, _) => {
                        h3956
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3957, _, _, _, _, _, _, _, _) => {
                        h3957
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3958, _, _, _, _, _, _, _, _) => {
                        h3958
                    }
                })?
            } else {
                false
            } {
                f228(match v2.clone() {
                    Adt2::C0(_, _, _, h3959, _, _, _, _, _, _, _) => {
                        h3959
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3960, _, _, _, _, _, _, _) => {
                        h3960
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3961, _, _, _, _, _, _) => {
                        h3961
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3962, _, _, _, _, _, _, _) => {
                        h3962
                    }
                })?
            } else {
                false
            } {
                f228(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3963, _, _, _, _, _, _) => {
                        h3963
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3964, _, _, _, _, _) => {
                        h3964
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3965, _, _, _, _) => {
                        h3965
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3966, _, _, _, _, _, _) => {
                        h3966
                    }
                })?
            } else {
                false
            } {
                let a3968 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3967, _, _, _) => {
                        h3967
                    }
                };
                let a3969 = 4294967295u64;
                nat_le(a3968, a3969)
            } else {
                false
            } {
                if v4 {
                    !f48(match v2.clone() {
                        Adt2::C0(_, h3970, _, _, _, _, _, _, _, _, _) => {
                            h3970
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3971, _, _, _, _, _, _, _, _) => {
                            h3971
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3972, _, _, _, _, _, _, _) => {
                            h3972
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3973, _, _, _, _, _, _) => {
                            h3973
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3974, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3974
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3975, _, _, _, _, _, _, _, _, _) => {
                        h3975
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3976, _, _, _, _, _, _, _) => {
                        h3976
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3977, _, _, _, _, _) => {
                        h3977
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3978 = v0;
            let a3979 = 62u64;
            if nat_eq(a3978, a3979) {
                let a3981 = match v1.clone() {
                    Adt1::C0(h3980, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3980
                    }
                };
                let a3982 = 2147483648u64;
                if if if if if nat_lt(a3981, a3982) {
                    let a3984 = match v1.clone() {
                        Adt1::C0(_, h3983, _, _, _, _, _, _, _, _, _, _) => {
                            h3983
                        }
                    };
                    let a3985 = 2147483648u64;
                    nat_lt(a3984, a3985)
                } else {
                    false
                } {
                    let a3988 = match v1.clone() {
                        Adt1::C0(h3986, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3986
                        }
                    };
                    let a3989 = match v2.clone() {
                        Adt2::C0(_, h3987, _, _, _, _, _, _, _, _, _) => {
                            h3987
                        }
                    };
                    nat_lt(a3988, a3989)
                } else {
                    false
                } {
                    let a3992 = match v1.clone() {
                        Adt1::C0(_, h3990, _, _, _, _, _, _, _, _, _, _) => {
                            h3990
                        }
                    };
                    let a3993 = match v2.clone() {
                        Adt2::C0(_, h3991, _, _, _, _, _, _, _, _, _) => {
                            h3991
                        }
                    };
                    nat_lt(a3992, a3993)
                } else {
                    false
                } {
                    let a3996 = match v2.clone() {
                        Adt2::C0(_, _, h3994, _, _, _, _, _, _, _, _) => {
                            h3994
                        }
                    };
                    let a3997 = match v3.clone() {
                        Adt2::C0(_, _, h3995, _, _, _, _, _, _, _, _) => {
                            h3995
                        }
                    };
                    if if if nat_eq(a3996, a3997) {
                        let a4000 = match v2.clone() {
                            Adt2::C0(_, _, _, h3998, _, _, _, _, _, _, _) => {
                                h3998
                            }
                        };
                        let a4001 = match v3.clone() {
                            Adt2::C0(_, _, _, h3999, _, _, _, _, _, _, _) => {
                                h3999
                            }
                        };
                        nat_eq(a4000, a4001)
                    } else {
                        false
                    } {
                        let a4004 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4002, _, _, _, _, _, _) => {
                                h4002
                            }
                        };
                        let a4005 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4003, _, _, _, _, _, _) => {
                                h4003
                            }
                        };
                        nat_eq(a4004, a4005)
                    } else {
                        false
                    } {
                        let a4014 = match v3.clone() {
                            Adt2::C0(_, h4006, _, _, _, _, _, _, _, _, _) => {
                                h4006
                            }
                        };
                        let a4015 = {
                            let a4012 = {
                                let a4009 = match v2.clone() {
                                    Adt2::C0(_, h4007, _, _, _, _, _, _, _, _, _) => {
                                        h4007
                                    }
                                };
                                let a4010 = match v1.clone() {
                                    Adt1::C0(h4008, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4008
                                    }
                                };
                                nat_add(a4009, a4010)?
                            };
                            let a4013 = match v1.clone() {
                                Adt1::C0(_, h4011, _, _, _, _, _, _, _, _, _, _) => {
                                    h4011
                                }
                            };
                            nat_add(a4012, a4013)?
                        };
                        nat_eq(a4014, a4015)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4016, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4016
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4017 = v0;
                let a4018 = 63u64;
                if nat_eq(a4017, a4018) {
                    if if if if if f229(match v2.clone() {
                        Adt2::C0(_, h4019, _, _, _, _, _, _, _, _, _) => {
                            h4019
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4020, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4020
                        }
                    })? {
                        f229(match v2.clone() {
                            Adt2::C0(_, _, h4021, _, _, _, _, _, _, _, _) => {
                                h4021
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4022, _, _, _, _, _, _, _, _, _, _) => {
                                h4022
                            }
                        })?
                    } else {
                        false
                    } {
                        f229(match v2.clone() {
                            Adt2::C0(_, _, _, h4023, _, _, _, _, _, _, _) => {
                                h4023
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4024, _, _, _, _, _, _, _, _, _) => {
                                h4024
                            }
                        })?
                    } else {
                        false
                    } {
                        f229(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4025, _, _, _, _, _, _) => {
                                h4025
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4026, _, _, _, _, _, _, _, _) => {
                                h4026
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4029 = match v2.clone() {
                            Adt2::C0(_, h4027, _, _, _, _, _, _, _, _, _) => {
                                h4027
                            }
                        };
                        let a4030 = match v3.clone() {
                            Adt2::C0(_, h4028, _, _, _, _, _, _, _, _, _) => {
                                h4028
                            }
                        };
                        if if if nat_eq(a4029, a4030) {
                            let a4033 = match v2.clone() {
                                Adt2::C0(_, _, h4031, _, _, _, _, _, _, _, _) => {
                                    h4031
                                }
                            };
                            let a4034 = match v3.clone() {
                                Adt2::C0(_, _, h4032, _, _, _, _, _, _, _, _) => {
                                    h4032
                                }
                            };
                            nat_eq(a4033, a4034)
                        } else {
                            false
                        } {
                            let a4037 = match v2.clone() {
                                Adt2::C0(_, _, _, h4035, _, _, _, _, _, _, _) => {
                                    h4035
                                }
                            };
                            let a4038 = match v3.clone() {
                                Adt2::C0(_, _, _, h4036, _, _, _, _, _, _, _) => {
                                    h4036
                                }
                            };
                            nat_eq(a4037, a4038)
                        } else {
                            false
                        } {
                            let a4041 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4039, _, _, _, _, _, _) => {
                                    h4039
                                }
                            };
                            let a4042 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4040, _, _, _, _, _, _) => {
                                    h4040
                                }
                            };
                            nat_eq(a4041, a4042)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4043, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4043
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4044, _, _, _, _, _, _, _, _, _, _) => {
                                h4044
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4045, _, _, _, _, _, _, _, _, _) => {
                                h4045
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4046, _, _, _, _, _, _, _, _) => {
                                h4046
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

pub fn f129(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4047 = v3;
    match m4047 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4048 = v0;
                let a4049 = 4u64;
                nat_eq(a4048, a4049)
            };
            let v7: bool = {
                let a4051 = 0u64;
                let a4052 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4050, _, _, _, _, _, _, _) => {
                        h4050
                    }
                };
                nat_lt(a4051, a4052)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4053, _, _, _, _, _, _, _, _, _) => {
                    h4053
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4054, _, _, _, _, _, _, _, _) => {
                    h4054
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4055, _, _, _, _, _, _, _) => {
                    h4055
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4056, _, _, _, _, _, _) => {
                    h4056
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4057, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4057
                }
            }, match v1.clone() {
                Adt1::C0(_, h4058, _, _, _, _, _, _, _, _, _, _) => {
                    h4058
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4059, _, _, _, _, _, _, _, _, _) => {
                    h4059
                }
            });
            if if if if if if if if if if if if v6 {
                let a4061 = match v2.clone() {
                    Adt2::C0(h4060, _, _, _, _, _, _, _, _, _, _) => {
                        h4060
                    }
                };
                let a4062 = 0u64;
                if nat_eq(a4061, a4062) {
                    let a4064 = match v5.clone() {
                        Adt2::C0(h4063, _, _, _, _, _, _, _, _, _, _) => {
                            h4063
                        }
                    };
                    let a4065 = 0u64;
                    nat_eq(a4064, a4065)
                } else {
                    false
                }
            } else {
                let a4067 = match v2.clone() {
                    Adt2::C0(h4066, _, _, _, _, _, _, _, _, _, _) => {
                        h4066
                    }
                };
                let a4068 = 0u64;
                if if nat_eq(a4067, a4068) {
                    true
                } else {
                    let a4070 = match v2.clone() {
                        Adt2::C0(h4069, _, _, _, _, _, _, _, _, _, _) => {
                            h4069
                        }
                    };
                    let a4071 = 26u64;
                    nat_eq(a4070, a4071)
                } {
                    let a4073 = match v5.clone() {
                        Adt2::C0(h4072, _, _, _, _, _, _, _, _, _, _) => {
                            h4072
                        }
                    };
                    let a4074 = 0u64;
                    if nat_eq(a4073, a4074) {
                        true
                    } else {
                        let a4076 = match v5.clone() {
                            Adt2::C0(h4075, _, _, _, _, _, _, _, _, _, _) => {
                                h4075
                            }
                        };
                        let a4077 = 26u64;
                        nat_eq(a4076, a4077)
                    }
                } else {
                    false
                }
            } {
                let a4080 = match v2.clone() {
                    Adt2::C0(h4078, _, _, _, _, _, _, _, _, _, _) => {
                        h4078
                    }
                };
                let a4081 = match v4.clone() {
                    Adt2::C0(h4079, _, _, _, _, _, _, _, _, _, _) => {
                        h4079
                    }
                };
                nat_eq(a4080, a4081)
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
                let a4083 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4082, _, _, _, _, _) => {
                        h4082
                    }
                };
                let a4084 = 4u64;
                nat_eq(a4083, a4084)
            } else {
                false
            } {
                let a4085 = v8;
                let a4086 = f23(v2.clone(), 1u64)?;
                nat_le(a4085, a4086)
            } else {
                false
            } {
                let a4088 = match v1.clone() {
                    Adt1::C0(_, _, _, h4087, _, _, _, _, _, _, _, _) => {
                        h4087
                    }
                };
                let a4089 = 1073741824u64;
                nat_lt(a4088, a4089)
            } else {
                false
            } {
                let a4091 = match v1.clone() {
                    Adt1::C0(h4090, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4090
                    }
                };
                let a4092 = 4294967295u64;
                if if if if nat_le(a4091, a4092) {
                    let a4094 = match v1.clone() {
                        Adt1::C0(_, h4093, _, _, _, _, _, _, _, _, _, _) => {
                            h4093
                        }
                    };
                    let a4095 = 4294967295u64;
                    nat_le(a4094, a4095)
                } else {
                    false
                } {
                    let a4097 = match v1.clone() {
                        Adt1::C0(_, _, h4096, _, _, _, _, _, _, _, _, _) => {
                            h4096
                        }
                    };
                    let a4098 = 4294967295u64;
                    nat_le(a4097, a4098)
                } else {
                    false
                } {
                    let a4100 = match v1.clone() {
                        Adt1::C0(_, _, _, h4099, _, _, _, _, _, _, _, _) => {
                            h4099
                        }
                    };
                    let a4101 = 4294967295u64;
                    nat_le(a4100, a4101)
                } else {
                    false
                } {
                    let a4103 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4102, _, _, _, _, _, _, _) => {
                            h4102
                        }
                    };
                    let a4104 = 4294967295u64;
                    nat_le(a4103, a4104)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4107 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4105) => {
                            h4105
                        }
                    };
                    let a4108 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4106) => {
                            h4106
                        }
                    };
                    nat_eq(a4107, a4108)
                } else {
                    true
                }
            } else {
                false
            } {
                f230(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4109, _, _, _, _, _, _, _, _) => {
                        h4109
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4110, _) => {
                        h4110
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
                    let a4111 = v10;
                    let a4112 = v8;
                    nat_add(a4111, a4112)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4113, _, _, _, _, _, _, _, _) => {
                        h4113
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f130(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4115 = match v0.clone() {
        Adt2::C0(h4114, _, _, _, _, _, _, _, _, _, _) => {
            h4114
        }
    };
    let a4116 = 0u64;
    if if if nat_eq(a4115, a4116) {
        let a4118 = match v1.clone() {
            Adt2::C0(h4117, _, _, _, _, _, _, _, _, _, _) => {
                h4117
            }
        };
        let a4119 = 0u64;
        nat_eq(a4118, a4119)
    } else {
        false
    } {
        f226(match v0.clone() {
            Adt2::C0(_, h4120, _, _, _, _, _, _, _, _, _) => {
                h4120
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4121, _, _, _, _, _, _, _, _) => {
                h4121
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4122, _, _, _, _, _, _, _) => {
                h4122
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4123, _, _, _, _, _, _) => {
                h4123
            }
        }, match v1.clone() {
            Adt2::C0(_, h4124, _, _, _, _, _, _, _, _, _) => {
                h4124
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4125, _, _, _, _, _, _, _, _) => {
                h4125
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4126, _, _, _, _, _, _, _) => {
                h4126
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4127, _, _, _, _, _, _) => {
                h4127
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4128 = v2;
            let a4129 = f23(v0.clone(), 1u64)?;
            nat_add(a4128, a4129)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f131(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4131 = match v1.clone() {
        Adt2::C0(h4130, _, _, _, _, _, _, _, _, _, _) => {
            h4130
        }
    };
    let a4132 = 0u64;
    if if if nat_eq(a4131, a4132) {
        true
    } else {
        let a4134 = match v1.clone() {
            Adt2::C0(h4133, _, _, _, _, _, _, _, _, _, _) => {
                h4133
            }
        };
        let a4135 = 1u64;
        nat_eq(a4134, a4135)
    } {
        true
    } else {
        let a4137 = match v1.clone() {
            Adt2::C0(h4136, _, _, _, _, _, _, _, _, _, _) => {
                h4136
            }
        };
        let a4138 = 30u64;
        nat_eq(a4137, a4138)
    } {
        let a4141 = match v1.clone() {
            Adt2::C0(h4139, _, _, _, _, _, _, _, _, _, _) => {
                h4139
            }
        };
        let a4142 = match v2.clone() {
            Adt2::C0(h4140, _, _, _, _, _, _, _, _, _, _) => {
                h4140
            }
        };
        if if if if if nat_eq(a4141, a4142) {
            let a4145 = match v1.clone() {
                Adt2::C0(_, h4143, _, _, _, _, _, _, _, _, _) => {
                    h4143
                }
            };
            let a4146 = match v2.clone() {
                Adt2::C0(_, h4144, _, _, _, _, _, _, _, _, _) => {
                    h4144
                }
            };
            nat_eq(a4145, a4146)
        } else {
            false
        } {
            let a4149 = match v1.clone() {
                Adt2::C0(_, _, _, h4147, _, _, _, _, _, _, _) => {
                    h4147
                }
            };
            let a4150 = match v2.clone() {
                Adt2::C0(_, _, _, h4148, _, _, _, _, _, _, _) => {
                    h4148
                }
            };
            nat_eq(a4149, a4150)
        } else {
            false
        } {
            let a4153 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4151, _, _, _, _, _, _) => {
                    h4151
                }
            };
            let a4154 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4152, _, _, _, _, _, _) => {
                    h4152
                }
            };
            nat_eq(a4153, a4154)
        } else {
            false
        } {
            let a4155 = v0;
            let a4156 = 4u64;
            if nat_eq(a4155, a4156) {
                let a4158 = match v1.clone() {
                    Adt2::C0(_, _, h4157, _, _, _, _, _, _, _, _) => {
                        h4157
                    }
                };
                let a4159 = 1u64;
                if nat_eq(a4158, a4159) {
                    let a4162 = match v1.clone() {
                        Adt2::C0(_, h4160, _, _, _, _, _, _, _, _, _) => {
                            h4160
                        }
                    };
                    let a4163 = match v2.clone() {
                        Adt2::C0(_, _, h4161, _, _, _, _, _, _, _, _) => {
                            h4161
                        }
                    };
                    nat_eq(a4162, a4163)
                } else {
                    false
                }
            } else {
                let a4164 = v0;
                let a4165 = 4u64;
                if if nat_lt(a4164, a4165) {
                    let a4168 = match v1.clone() {
                        Adt2::C0(_, h4166, _, _, _, _, _, _, _, _, _) => {
                            h4166
                        }
                    };
                    let a4169 = match v1.clone() {
                        Adt2::C0(_, _, h4167, _, _, _, _, _, _, _, _) => {
                            h4167
                        }
                    };
                    nat_eq(a4168, a4169)
                } else {
                    false
                } {
                    let a4172 = match v1.clone() {
                        Adt2::C0(_, _, h4170, _, _, _, _, _, _, _, _) => {
                            h4170
                        }
                    };
                    let a4173 = match v2.clone() {
                        Adt2::C0(_, _, h4171, _, _, _, _, _, _, _, _) => {
                            h4171
                        }
                    };
                    nat_eq(a4172, a4173)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4175 = f216(match v1.clone() {
                Adt2::C0(h4174, _, _, _, _, _, _, _, _, _, _) => {
                    h4174
                }
            });
            match m4175 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4176) => {
                            h4176
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

pub fn f132(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f231(match v1.clone() {
        Adt2::C0(h4177, _, _, _, _, _, _, _, _, _, _) => {
            h4177
        }
    }, v0) {
        let a4180 = match v1.clone() {
            Adt2::C0(h4178, _, _, _, _, _, _, _, _, _, _) => {
                h4178
            }
        };
        let a4181 = match v2.clone() {
            Adt2::C0(h4179, _, _, _, _, _, _, _, _, _, _) => {
                h4179
            }
        };
        nat_eq(a4180, a4181)
    } else {
        false
    } {
        let a4184 = match v1.clone() {
            Adt2::C0(_, h4182, _, _, _, _, _, _, _, _, _) => {
                h4182
            }
        };
        let a4185 = match v2.clone() {
            Adt2::C0(_, h4183, _, _, _, _, _, _, _, _, _) => {
                h4183
            }
        };
        if if if nat_eq(a4184, a4185) {
            let a4188 = match v1.clone() {
                Adt2::C0(_, _, h4186, _, _, _, _, _, _, _, _) => {
                    h4186
                }
            };
            let a4189 = match v2.clone() {
                Adt2::C0(_, _, h4187, _, _, _, _, _, _, _, _) => {
                    h4187
                }
            };
            nat_eq(a4188, a4189)
        } else {
            false
        } {
            let a4192 = match v1.clone() {
                Adt2::C0(_, _, _, h4190, _, _, _, _, _, _, _) => {
                    h4190
                }
            };
            let a4193 = match v2.clone() {
                Adt2::C0(_, _, _, h4191, _, _, _, _, _, _, _) => {
                    h4191
                }
            };
            nat_eq(a4192, a4193)
        } else {
            false
        } {
            let a4196 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4194, _, _, _, _, _, _) => {
                    h4194
                }
            };
            let a4197 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4195, _, _, _, _, _, _) => {
                    h4195
                }
            };
            nat_eq(a4196, a4197)
        } else {
            false
        }
    } else {
        false
    } {
        let m4199 = f216(match v1.clone() {
            Adt2::C0(h4198, _, _, _, _, _, _, _, _, _, _) => {
                h4198
            }
        });
        match m4199 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4200) => {
                        h4200
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4201, _, _, _, _, _, _, _, _, _, _) => {
                        h4201
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f133(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4202, _, _, _, _, _, _, _, _, _, _, _) => {
                h4202
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4203, _, _, _, _, _, _, _, _, _) => {
                h4203
            }
        }
    };
    let a4204 = 0u64;
    let a4205 = v4;
    if if nat_lt(a4204, a4205) {
        let a4206 = v4;
        let a4207 = 2147483648u64;
        nat_lt(a4206, a4207)
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
        let v7: u64 = f232(match v5.clone() {
            Adt2::C0(_, _, h4208, _, _, _, _, _, _, _, _) => {
                h4208
            }
        }, v4)?;
        let v8: u64 = f232(match v5.clone() {
            Adt2::C0(_, _, _, h4209, _, _, _, _, _, _, _) => {
                h4209
            }
        }, v4)?;
        let a4211 = match v5.clone() {
            Adt2::C0(h4210, _, _, _, _, _, _, _, _, _, _) => {
                h4210
            }
        };
        let a4212 = 0u64;
        if if if if if nat_eq(a4211, a4212) {
            let a4214 = match v6.clone() {
                Adt2::C0(h4213, _, _, _, _, _, _, _, _, _, _) => {
                    h4213
                }
            };
            let a4215 = 0u64;
            nat_eq(a4214, a4215)
        } else {
            false
        } {
            let a4217 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4216, _, _, _, _, _, _) => {
                    h4216
                }
            };
            let a4218 = 1u64;
            nat_eq(a4217, a4218)
        } else {
            false
        } {
            let a4221 = match v5.clone() {
                Adt2::C0(_, h4219, _, _, _, _, _, _, _, _, _) => {
                    h4219
                }
            };
            let a4222 = match v6.clone() {
                Adt2::C0(_, h4220, _, _, _, _, _, _, _, _, _) => {
                    h4220
                }
            };
            nat_eq(a4221, a4222)
        } else {
            false
        } {
            if v0 {
                let a4224 = match v5.clone() {
                    Adt2::C0(_, _, h4223, _, _, _, _, _, _, _, _) => {
                        h4223
                    }
                };
                let a4225 = 2147483648u64;
                if if if nat_lt(a4224, a4225) {
                    let a4227 = match v5.clone() {
                        Adt2::C0(_, _, _, h4226, _, _, _, _, _, _, _) => {
                            h4226
                        }
                    };
                    let a4228 = 2147483648u64;
                    nat_lt(a4227, a4228)
                } else {
                    false
                } {
                    let a4231 = {
                        let a4229 = v7;
                        let a4230 = v8;
                        nat_mul(a4229, a4230)?
                    };
                    let a4232 = 2147483648u64;
                    nat_le(a4231, a4232)
                } else {
                    false
                } {
                    f233(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4234 = match v1.clone() {
                    Adt1::C0(h4233, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4233
                    }
                };
                let a4235 = v7;
                if if if if nat_eq(a4234, a4235) {
                    let a4237 = match v1 {
                        Adt1::C0(_, h4236, _, _, _, _, _, _, _, _, _, _) => {
                            h4236
                        }
                    };
                    let a4238 = v8;
                    nat_eq(a4237, a4238)
                } else {
                    false
                } {
                    f234(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4240 = match v6.clone() {
                        Adt2::C0(_, _, h4239, _, _, _, _, _, _, _, _) => {
                            h4239
                        }
                    };
                    let a4241 = v4;
                    if nat_eq(a4240, a4241) {
                        let a4243 = match v6.clone() {
                            Adt2::C0(_, _, _, h4242, _, _, _, _, _, _, _) => {
                                h4242
                            }
                        };
                        let a4244 = v4;
                        nat_eq(a4243, a4244)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4248 = match v6 {
                        Adt2::C0(_, _, _, _, h4245, _, _, _, _, _, _) => {
                            h4245
                        }
                    };
                    let a4249 = {
                        let a4246 = v7;
                        let a4247 = v8;
                        nat_mul(a4246, a4247)?
                    };
                    nat_eq(a4248, a4249)
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

pub fn f134(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4251 = match v2.clone() {
        Adt2::C0(h4250, _, _, _, _, _, _, _, _, _, _) => {
            h4250
        }
    };
    let a4252 = 0u64;
    if if if if if nat_eq(a4251, a4252) {
        true
    } else {
        let a4254 = match v2.clone() {
            Adt2::C0(h4253, _, _, _, _, _, _, _, _, _, _) => {
                h4253
            }
        };
        let a4255 = 1u64;
        nat_eq(a4254, a4255)
    } {
        true
    } else {
        let a4257 = match v2.clone() {
            Adt2::C0(h4256, _, _, _, _, _, _, _, _, _, _) => {
                h4256
            }
        };
        let a4258 = 30u64;
        nat_eq(a4257, a4258)
    } {
        let a4261 = match v2.clone() {
            Adt2::C0(h4259, _, _, _, _, _, _, _, _, _, _) => {
                h4259
            }
        };
        let a4262 = match v3.clone() {
            Adt2::C0(h4260, _, _, _, _, _, _, _, _, _, _) => {
                h4260
            }
        };
        nat_eq(a4261, a4262)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a4264 = match v1.clone() {
                Adt1::C0(h4263, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4263
                }
            };
            let a4265 = 4294967296u64;
            nat_lt(a4264, a4265)
        } else {
            false
        } {
            let a4267 = match v1.clone() {
                Adt1::C0(_, h4266, _, _, _, _, _, _, _, _, _, _) => {
                    h4266
                }
            };
            let a4268 = 4294967296u64;
            nat_lt(a4267, a4268)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4270 = match v2.clone() {
                Adt2::C0(h4269, _, _, _, _, _, _, _, _, _, _) => {
                    h4269
                }
            };
            let a4271 = 0u64;
            if nat_eq(a4270, a4271) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a4272 = v0;
            let a4273 = 31u64;
            if nat_eq(a4272, a4273) {
                0u64
            } else {
                let a4274 = v0;
                let a4275 = 47u64;
                if nat_eq(a4274, a4275) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4276, _, _, _, _, _, _, _, _, _, _) => {
                h4276
            }
        }, match v1.clone() {
            Adt1::C0(h4277, _, _, _, _, _, _, _, _, _, _, _) => {
                h4277
            }
        }, match v1 {
            Adt1::C0(_, h4278, _, _, _, _, _, _, _, _, _, _) => {
                h4278
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f135(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4280 = match v0.clone() {
        Adt2::C0(h4279, _, _, _, _, _, _, _, _, _, _) => {
            h4279
        }
    };
    let a4281 = 0u64;
    if if if nat_eq(a4280, a4281) {
        let a4283 = match v1.clone() {
            Adt2::C0(h4282, _, _, _, _, _, _, _, _, _, _) => {
                h4282
            }
        };
        let a4284 = 0u64;
        nat_eq(a4283, a4284)
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

pub fn f136(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4286 = match v0.clone() {
        Adt2::C0(h4285, _, _, _, _, _, _, _, _, _, _) => {
            h4285
        }
    };
    let a4287 = 0u64;
    if if nat_eq(a4286, a4287) {
        let a4289 = match v1.clone() {
            Adt2::C0(h4288, _, _, _, _, _, _, _, _, _, _) => {
                h4288
            }
        };
        let a4290 = 26u64;
        nat_eq(a4289, a4290)
    } else {
        false
    } {
        let a4292 = match v0.clone() {
            Adt2::C0(_, _, _, h4291, _, _, _, _, _, _, _) => {
                h4291
            }
        };
        let a4293 = 1u64;
        if if if if if if if nat_eq(a4292, a4293) {
            let a4295 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4294, _, _, _, _, _, _) => {
                    h4294
                }
            };
            let a4296 = 1u64;
            nat_eq(a4295, a4296)
        } else {
            false
        } {
            let a4298 = match v0.clone() {
                Adt2::C0(_, h4297, _, _, _, _, _, _, _, _, _) => {
                    h4297
                }
            };
            let a4299 = 2147483647u64;
            nat_le(a4298, a4299)
        } else {
            false
        } {
            let a4302 = match v1.clone() {
                Adt2::C0(_, h4300, _, _, _, _, _, _, _, _, _) => {
                    h4300
                }
            };
            let a4303 = match v0.clone() {
                Adt2::C0(_, _, h4301, _, _, _, _, _, _, _, _) => {
                    h4301
                }
            };
            nat_eq(a4302, a4303)
        } else {
            false
        } {
            let a4305 = match v1.clone() {
                Adt2::C0(_, _, h4304, _, _, _, _, _, _, _, _) => {
                    h4304
                }
            };
            let a4306 = 1u64;
            nat_eq(a4305, a4306)
        } else {
            false
        } {
            let a4308 = match v1.clone() {
                Adt2::C0(_, _, _, h4307, _, _, _, _, _, _, _) => {
                    h4307
                }
            };
            let a4309 = 1u64;
            nat_eq(a4308, a4309)
        } else {
            false
        } {
            let a4311 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4310, _, _, _, _, _, _) => {
                    h4310
                }
            };
            let a4312 = 1u64;
            nat_eq(a4311, a4312)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4315 = match v0.clone() {
                    Adt2::C0(_, _, h4313, _, _, _, _, _, _, _, _) => {
                        h4313
                    }
                };
                let a4316 = f177(match v0.clone() {
                    Adt2::C0(_, h4314, _, _, _, _, _, _, _, _, _) => {
                        h4314
                    }
                })?;
                nat_mul(a4315, a4316)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f137(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4317 = v1;
    match m4317 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f182(f25(v0.clone(), 1u64))?;
            let a4319 = match v0.clone() {
                Adt2::C0(h4318, _, _, _, _, _, _, _, _, _, _) => {
                    h4318
                }
            };
            let a4320 = 26u64;
            if if if nat_eq(a4319, a4320) {
                let a4322 = match v3.clone() {
                    Adt2::C0(h4321, _, _, _, _, _, _, _, _, _, _) => {
                        h4321
                    }
                };
                let a4323 = 26u64;
                nat_eq(a4322, a4323)
            } else {
                false
            } {
                let a4325 = match v2.clone() {
                    Adt2::C0(h4324, _, _, _, _, _, _, _, _, _, _) => {
                        h4324
                    }
                };
                let a4326 = 27u64;
                nat_eq(a4325, a4326)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a4328 = match v2.clone() {
                        Adt2::C0(_, h4327, _, _, _, _, _, _, _, _, _) => {
                            h4327
                        }
                    };
                    let a4329 = 1u64;
                    nat_eq(a4328, a4329)
                } else {
                    false
                } {
                    let a4331 = match v2.clone() {
                        Adt2::C0(_, _, h4330, _, _, _, _, _, _, _, _) => {
                            h4330
                        }
                    };
                    let a4332 = 1u64;
                    nat_eq(a4331, a4332)
                } else {
                    false
                } {
                    let a4334 = match v2.clone() {
                        Adt2::C0(_, _, _, h4333, _, _, _, _, _, _, _) => {
                            h4333
                        }
                    };
                    let a4335 = 1u64;
                    nat_eq(a4334, a4335)
                } else {
                    false
                } {
                    let a4337 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4336, _, _, _, _, _, _) => {
                            h4336
                        }
                    };
                    let a4338 = 1u64;
                    nat_eq(a4337, a4338)
                } else {
                    false
                } {
                    let a4339 = v4;
                    let a4340 = 9223372036854775808u64;
                    nat_lt(a4339, a4340)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4343 = 2u64;
                        let a4344 = {
                            let a4341 = v4;
                            let a4342 = 2u64;
                            nat_mul(a4341, a4342)?
                        };
                        nat_add(a4343, a4344)?
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

pub fn f138(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4345, _, _, _, _, _, _, _, _, _, _) => {
            h4345
        }
    };
    let a4346 = v3;
    let a4347 = 0u64;
    if if nat_eq(a4346, a4347) {
        true
    } else {
        let a4348 = v0;
        let a4349 = 13u64;
        if nat_eq(a4348, a4349) {
            let a4350 = v3;
            let a4351 = 1u64;
            if nat_eq(a4350, a4351) {
                true
            } else {
                let a4352 = v3;
                let a4353 = 30u64;
                nat_eq(a4352, a4353)
            }
        } else {
            false
        }
    } {
        let a4355 = v3;
        let a4356 = match v2.clone() {
            Adt2::C0(h4354, _, _, _, _, _, _, _, _, _, _) => {
                h4354
            }
        };
        if if nat_eq(a4355, a4356) {
            let a4357 = v0;
            let a4358 = 13u64;
            if nat_eq(a4357, a4358) {
                let a4360 = match v2.clone() {
                    Adt2::C0(_, h4359, _, _, _, _, _, _, _, _, _) => {
                        h4359
                    }
                };
                let a4361 = 1u64;
                if if if nat_eq(a4360, a4361) {
                    let a4363 = match v2.clone() {
                        Adt2::C0(_, _, h4362, _, _, _, _, _, _, _, _) => {
                            h4362
                        }
                    };
                    let a4364 = 1u64;
                    nat_eq(a4363, a4364)
                } else {
                    false
                } {
                    let a4366 = match v2.clone() {
                        Adt2::C0(_, _, _, h4365, _, _, _, _, _, _, _) => {
                            h4365
                        }
                    };
                    let a4367 = 1u64;
                    nat_eq(a4366, a4367)
                } else {
                    false
                } {
                    let a4369 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4368, _, _, _, _, _, _) => {
                            h4368
                        }
                    };
                    let a4370 = 1u64;
                    nat_eq(a4369, a4370)
                } else {
                    false
                }
            } else {
                let a4372 = match v2.clone() {
                    Adt2::C0(_, h4371, _, _, _, _, _, _, _, _, _) => {
                        h4371
                    }
                };
                let a4373 = 1u64;
                if if if nat_eq(a4372, a4373) {
                    let a4376 = match v1.clone() {
                        Adt2::C0(_, _, h4374, _, _, _, _, _, _, _, _) => {
                            h4374
                        }
                    };
                    let a4377 = match v2.clone() {
                        Adt2::C0(_, _, h4375, _, _, _, _, _, _, _, _) => {
                            h4375
                        }
                    };
                    nat_eq(a4376, a4377)
                } else {
                    false
                } {
                    let a4380 = match v1.clone() {
                        Adt2::C0(_, _, _, h4378, _, _, _, _, _, _, _) => {
                            h4378
                        }
                    };
                    let a4381 = match v2.clone() {
                        Adt2::C0(_, _, _, h4379, _, _, _, _, _, _, _) => {
                            h4379
                        }
                    };
                    nat_eq(a4380, a4381)
                } else {
                    false
                } {
                    let a4384 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4382, _, _, _, _, _, _) => {
                            h4382
                        }
                    };
                    let a4385 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4383, _, _, _, _, _, _) => {
                            h4383
                        }
                    };
                    nat_eq(a4384, a4385)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt10::C4(f235(v4.clone(), v0, v3)?, {
                let a4386 = v3;
                let a4387 = 0u64;
                if nat_eq(a4386, a4387) {
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

pub fn f139(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4388 = v0;
    let a4389 = 89u64;
    if if nat_eq(a4388, a4389) {
        let a4390 = v1;
        let a4391 = 16u64;
        nat_eq(a4390, a4391)
    } else {
        false
    } {
        Ok(Some(24u64))
    } else {
        let a4392 = v0;
        let a4393 = 89u64;
        if if nat_eq(a4392, a4393) {
            let a4394 = v1;
            let a4395 = 14u64;
            nat_eq(a4394, a4395)
        } else {
            false
        } {
            Ok(Some(23u64))
        } else {
            let a4396 = v0;
            let a4397 = 89u64;
            if if nat_eq(a4396, a4397) {
                let a4398 = v1;
                let a4399 = 15u64;
                nat_eq(a4398, a4399)
            } else {
                false
            } {
                Ok(Some(22u64))
            } else {
                let a4400 = v0;
                let a4401 = 89u64;
                if if nat_eq(a4400, a4401) {
                    let a4402 = 8u64;
                    let a4403 = v1;
                    if nat_le(a4402, a4403) {
                        let a4404 = v1;
                        let a4405 = 9u64;
                        nat_le(a4404, a4405)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Some({
                        let a4406 = v1;
                        let a4407 = 12u64;
                        nat_add(a4406, a4407)?
                    }))
                } else {
                    let a4408 = v0;
                    let a4409 = 89u64;
                    if if nat_eq(a4408, a4409) {
                        let a4410 = v1;
                        let a4411 = 4u64;
                        nat_eq(a4410, a4411)
                    } else {
                        false
                    } {
                        Ok(Some(19u64))
                    } else {
                        let a4412 = 10u64;
                        let a4413 = v0;
                        if if nat_le(a4412, a4413) {
                            let a4414 = v0;
                            let a4415 = 12u64;
                            nat_le(a4414, a4415)
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4416 = v0;
                                let a4417 = 3u64;
                                nat_add(a4416, a4417)?
                            }))
                        } else {
                            let a4418 = v0;
                            let a4419 = 89u64;
                            if if nat_eq(a4418, a4419) {
                                let a4420 = v1;
                                let a4421 = 7u64;
                                nat_eq(a4420, a4421)
                            } else {
                                false
                            } {
                                Ok(Some(17u64))
                            } else {
                                let a4422 = v0;
                                let a4423 = 89u64;
                                if if nat_eq(a4422, a4423) {
                                    let a4424 = v1;
                                    let a4425 = 10u64;
                                    nat_eq(a4424, a4425)
                                } else {
                                    false
                                } {
                                    Ok(Some(18u64))
                                } else {
                                    let a4426 = v0;
                                    let a4427 = 89u64;
                                    if if nat_eq(a4426, a4427) {
                                        let a4428 = v1;
                                        let a4429 = 13u64;
                                        nat_eq(a4428, a4429)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(16u64))
                                    } else {
                                        let a4430 = v0;
                                        let a4431 = 8u64;
                                        if nat_eq(a4430, a4431) {
                                            Ok(Some(5u64))
                                        } else {
                                            let a4432 = v0;
                                            let a4433 = 9u64;
                                            if nat_eq(a4432, a4433) {
                                                Ok(Some(6u64))
                                            } else {
                                                let a4434 = v0;
                                                let a4435 = 89u64;
                                                if nat_eq(a4434, a4435) {
                                                    let a4436 = v1;
                                                    let a4437 = 0u64;
                                                    if nat_eq(a4436, a4437) {
                                                        Ok(Some(0u64))
                                                    } else {
                                                        let a4438 = v1;
                                                        let a4439 = 1u64;
                                                        if nat_eq(a4438, a4439) {
                                                            Ok(Some(1u64))
                                                        } else {
                                                            let a4440 = v1;
                                                            let a4441 = 2u64;
                                                            if nat_eq(a4440, a4441) {
                                                                Ok(Some(2u64))
                                                            } else {
                                                                let a4442 = v1;
                                                                let a4443 = 3u64;
                                                                if nat_eq(a4442, a4443) {
                                                                    Ok(Some(3u64))
                                                                } else {
                                                                    let a4444 = v1;
                                                                    let a4445 = 6u64;
                                                                    if nat_eq(a4444, a4445) {
                                                                        Ok(Some(4u64))
                                                                    } else {
                                                                        let a4446 = v1;
                                                                        let a4447 = 12u64;
                                                                        if nat_eq(a4446, a4447) {
                                                                            Ok(Some(7u64))
                                                                        } else {
                                                                            let a4448 = v1;
                                                                            let a4449 = 11u64;
                                                                            if nat_eq(a4448, a4449) {
                                                                                Ok(Some(8u64))
                                                                            } else {
                                                                                let a4450 = v1;
                                                                                let a4451 = 18u64;
                                                                                if nat_eq(a4450, a4451) {
                                                                                    Ok(Some(9u64))
                                                                                } else {
                                                                                    let a4452 = v1;
                                                                                    let a4453 = 19u64;
                                                                                    if nat_eq(a4452, a4453) {
                                                                                        Ok(Some(10u64))
                                                                                    } else {
                                                                                        let a4454 = v1;
                                                                                        let a4455 = 20u64;
                                                                                        if nat_eq(a4454, a4455) {
                                                                                            Ok(Some(11u64))
                                                                                        } else {
                                                                                            let a4456 = v1;
                                                                                            let a4457 = 21u64;
                                                                                            if nat_eq(a4456, a4457) {
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

pub fn f140(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4458 = f236(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4458 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4459 = v0;
            let a4460 = 2u64;
            if if if if nat_eq(a4459, a4460) {
                true
            } else {
                let a4461 = v0;
                let a4462 = 5u64;
                nat_eq(a4461, a4462)
            } {
                true
            } else {
                let a4463 = v0;
                let a4464 = 6u64;
                nat_eq(a4463, a4464)
            } {
                true
            } else {
                let a4465 = v0;
                let a4466 = 7u64;
                nat_eq(a4465, a4466)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4467 = v3;
                    match m4467 {
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

pub fn f141(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4469 = match v1.clone() {
        Adt2::C0(h4468, _, _, _, _, _, _, _, _, _, _) => {
            h4468
        }
    };
    let a4470 = 0u64;
    if if if if if nat_eq(a4469, a4470) {
        true
    } else {
        let a4472 = match v1.clone() {
            Adt2::C0(h4471, _, _, _, _, _, _, _, _, _, _) => {
                h4471
            }
        };
        let a4473 = 1u64;
        nat_eq(a4472, a4473)
    } {
        true
    } else {
        let a4475 = match v1.clone() {
            Adt2::C0(h4474, _, _, _, _, _, _, _, _, _, _) => {
                h4474
            }
        };
        let a4476 = 30u64;
        nat_eq(a4475, a4476)
    } {
        let a4479 = match v1.clone() {
            Adt2::C0(h4477, _, _, _, _, _, _, _, _, _, _) => {
                h4477
            }
        };
        let a4480 = match v2.clone() {
            Adt2::C0(h4478, _, _, _, _, _, _, _, _, _, _) => {
                h4478
            }
        };
        if nat_eq(a4479, a4480) {
            true
        } else {
            let a4482 = match v2.clone() {
                Adt2::C0(h4481, _, _, _, _, _, _, _, _, _, _) => {
                    h4481
                }
            };
            let a4483 = 0u64;
            nat_eq(a4482, a4483)
        }
    } else {
        false
    } {
        f21(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4485 = f23(v2.clone(), 1u64)?;
            let a4486 = f237(v0, match v2.clone() {
                Adt2::C0(h4484, _, _, _, _, _, _, _, _, _, _) => {
                    h4484
                }
            });
            nat_mul(a4485, a4486)?
        }, {
            let a4488 = match v2.clone() {
                Adt2::C0(h4487, _, _, _, _, _, _, _, _, _, _) => {
                    h4487
                }
            };
            let a4489 = 0u64;
            if nat_eq(a4488, a4489) {
                4u64
            } else {
                2u64
            }
        }, f25(v1.clone(), 1u64), f25(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h4490, _, _, _, _, _, _, _, _, _, _) => {
                h4490
            }
        }, match v2 {
            Adt2::C0(h4491, _, _, _, _, _, _, _, _, _, _) => {
                h4491
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f142(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4504 = {
        let a4500 = {
            let a4496 = {
                let a4492 = v0;
                let a4493 = v4;
                nat_mul(a4492, a4493)?
            };
            let a4497 = {
                let a4494 = v1;
                let a4495 = v5;
                nat_mul(a4494, a4495)?
            };
            nat_add(a4496, a4497)?
        };
        let a4501 = {
            let a4498 = v2;
            let a4499 = v6;
            nat_mul(a4498, a4499)?
        };
        nat_add(a4500, a4501)?
    };
    let a4505 = {
        let a4502 = v3;
        let a4503 = v7;
        nat_mul(a4502, a4503)?
    };
    nat_add(a4504, a4505)
}

pub fn f143(v0: u64) -> u64 {
    let a4506 = v0;
    let a4507 = 2147483648u64;
    if nat_lt(a4506, a4507) {
        0u64
    } else {
        let a4508 = 4294967296u64;
        let a4509 = v0;
        nat_sub(a4508, a4509)
    }
}

pub fn f144(v0: u64) -> u64 {
    let a4510 = v0;
    let a4511 = 2147483648u64;
    if nat_lt(a4510, a4511) {
        v0
    } else {
        0u64
    }
}

pub fn f145(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4512 = v7;
        let a4513 = 6u64;
        if nat_eq(a4512, a4513) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f90(v3, v4), f90(v5, v6), f91(v7, v8), f176(v1), f176(v9))
}

pub fn f146(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4514 = v0;
    match m4514 {
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
        Adt4::C13(v57, v58, v59, v60) => {
            Adt11::C4(v57, v58, v59, v60)
        }
        Adt4::C14(v61, _, _) => {
            let a4515 = 13u64;
            let a4516 = v61;
            if nat_le(a4515, a4516) {
                Adt11::C7(v61, v1, v2)
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
        Adt4::C27(v81) => {
            Adt11::C2(match v81.clone() {
                Adt5::C0(_, _, _, _, _, h4517, _, _) => {
                    h4517
                }
            }, match v81 {
                Adt5::C0(_, _, _, _, _, _, _, h4518) => {
                    h4518
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
    }
}

pub fn f147(v0: Adt4) -> u64 {
    let m4519 = v0;
    match m4519 {
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
        Adt4::C14(v59, _, _) => {
            v59
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
    }
}

pub fn f148(v0: Adt4, v1: u64) -> u64 {
    let m4520 = v0;
    match m4520 {
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
        Adt4::C16(_, v64) => {
            v64
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v67, _) => {
            let a4521 = v1;
            let a4522 = 6u64;
            if nat_eq(a4521, a4522) {
                v67
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
        Adt4::C26(v81, _, _, _) => {
            let a4531 = v1;
            let a4532 = 6u64;
            if if if nat_eq(a4531, a4532) {
                true
            } else {
                let a4533 = v1;
                let a4534 = 7u64;
                nat_eq(a4533, a4534)
            } {
                let a4535 = v81;
                let a4536 = 30u64;
                nat_eq(a4535, a4536)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v80) => {
            match v80 {
                Adt5::C0(_, _, _, _, _, _, _, h4530) => {
                    h4530
                }
            }
        }
        Adt4::C28(v76) => {
            let a4523 = v1;
            let a4524 = 6u64;
            if nat_eq(a4523, a4524) {
                match v76 {
                    Adt7::C0(_, _, _, _, h4525) => {
                        h4525
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v79) => {
            let a4526 = v1;
            let a4527 = 6u64;
            if if nat_eq(a4526, a4527) {
                true
            } else {
                let a4528 = v1;
                let a4529 = 7u64;
                nat_eq(a4528, a4529)
            } {
                v79
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
    }
}

pub fn f149(v0: Adt4, v1: u64) -> Adt11 {
    let a4537 = v1;
    let a4538 = 4u64;
    if nat_lt(a4537, a4538) {
        Adt11::C0(v1)
    } else {
        let a4539 = v1;
        let a4540 = 4u64;
        if nat_eq(a4539, a4540) {
            Adt11::C1(0u64)
        } else {
            let a4541 = v1;
            let a4542 = 5u64;
            if nat_eq(a4541, a4542) {
                Adt11::C1(3u64)
            } else {
                let a4543 = v1;
                let a4544 = 6u64;
                if nat_eq(a4543, a4544) {
                    Adt11::C2(f148(v0, v1), 0u64, false)
                } else {
                    let a4545 = v1;
                    let a4546 = 7u64;
                    if nat_eq(a4545, a4546) {
                        Adt11::C2(0u64, f148(v0, v1), false)
                    } else {
                        let a4547 = v1;
                        let a4548 = 16u64;
                        if nat_lt(a4547, a4548) {
                            Adt11::C1({
                                let a4549 = v1;
                                let a4550 = 8u64;
                                nat_sub(a4549, a4550)
                            })
                        } else {
                            let a4551 = v1;
                            let a4552 = 20u64;
                            if nat_lt(a4551, a4552) {
                                Adt11::C5({
                                    let a4553 = v1;
                                    let a4554 = 16u64;
                                    nat_sub(a4553, a4554)
                                })
                            } else {
                                let a4555 = v1;
                                let a4556 = 20u64;
                                if nat_eq(a4555, a4556) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4557 = v1;
                                    let a4558 = 21u64;
                                    if nat_eq(a4557, a4558) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4559 = v1;
                                        let a4560 = 22u64;
                                        if nat_eq(a4559, a4560) {
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

pub fn f150(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4578 = {
        let a4573 = f238(v2, {
            let a4564 = {
                let a4561 = v0;
                let a4562 = v1;
                let a4563 = 0u64;
                nat_quot(a4561, a4562, a4563)
            };
            let a4565 = v2;
            let a4566 = 0u64;
            nat_rem(a4564, a4565, a4566)
        }, {
            let a4570 = {
                let a4567 = v0;
                let a4568 = v1;
                let a4569 = 0u64;
                nat_quot(a4567, a4568, a4569)
            };
            let a4571 = v2;
            let a4572 = 0u64;
            nat_quot(a4570, a4571, a4572)
        })?;
        let a4574 = v1;
        nat_mul(a4573, a4574)?
    };
    let a4579 = {
        let a4575 = v0;
        let a4576 = v1;
        let a4577 = 0u64;
        nat_rem(a4575, a4576, a4577)
    };
    nat_add(a4578, a4579)
}

pub fn f151(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4581 = v0;
        let a4582 = match v2.clone() {
            Adt3::C0(h4580, _, _, _, _, _, _, _) => {
                h4580
            }
        };
        let a4583 = 0u64;
        nat_quot(a4581, a4582, a4583)
    };
    let v4: u64 = {
        let a4585 = v3;
        let a4586 = match v2.clone() {
            Adt3::C0(_, h4584, _, _, _, _, _, _) => {
                h4584
            }
        };
        let a4587 = 0u64;
        nat_quot(a4585, a4586, a4587)
    };
    f239({
        let a4589 = v0;
        let a4590 = match v2.clone() {
            Adt3::C0(h4588, _, _, _, _, _, _, _) => {
                h4588
            }
        };
        let a4591 = 0u64;
        nat_rem(a4589, a4590, a4591)
    }, {
        let a4593 = v3;
        let a4594 = match v2.clone() {
            Adt3::C0(_, h4592, _, _, _, _, _, _) => {
                h4592
            }
        };
        let a4595 = 0u64;
        nat_rem(a4593, a4594, a4595)
    }, {
        let a4597 = v4;
        let a4598 = match v2.clone() {
            Adt3::C0(_, _, h4596, _, _, _, _, _) => {
                h4596
            }
        };
        let a4599 = 0u64;
        nat_rem(a4597, a4598, a4599)
    }, {
        let a4601 = v4;
        let a4602 = match v2 {
            Adt3::C0(_, _, h4600, _, _, _, _, _) => {
                h4600
            }
        };
        let a4603 = 0u64;
        nat_quot(a4601, a4602, a4603)
    }, match v1.clone() {
        Adt3::C0(h4604, _, _, _, _, _, _, _) => {
            h4604
        }
    }, match v1.clone() {
        Adt3::C0(_, h4605, _, _, _, _, _, _) => {
            h4605
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4606, _, _, _, _, _) => {
            h4606
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4607, _, _, _, _) => {
            h4607
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4608, _, _, _) => {
            h4608
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4609, _, _) => {
            h4609
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4610, _) => {
            h4610
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4611) => {
            h4611
        }
    })
}

pub fn f152(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4613 = v0;
        let a4614 = match v3.clone() {
            Adt3::C0(h4612, _, _, _, _, _, _, _) => {
                h4612
            }
        };
        let a4615 = 0u64;
        nat_quot(a4613, a4614, a4615)
    };
    let v6: u64 = {
        let a4617 = v5;
        let a4618 = match v3.clone() {
            Adt3::C0(_, h4616, _, _, _, _, _, _) => {
                h4616
            }
        };
        let a4619 = 0u64;
        nat_quot(a4617, a4618, a4619)
    };
    let v7: u64 = {
        let a4621 = v0;
        let a4622 = match v3.clone() {
            Adt3::C0(h4620, _, _, _, _, _, _, _) => {
                h4620
            }
        };
        let a4623 = 0u64;
        nat_rem(a4621, a4622, a4623)
    };
    let v8: u64 = {
        let a4625 = v5;
        let a4626 = match v3.clone() {
            Adt3::C0(_, h4624, _, _, _, _, _, _) => {
                h4624
            }
        };
        let a4627 = 0u64;
        nat_rem(a4625, a4626, a4627)
    };
    let v9: u64 = {
        let a4629 = v6;
        let a4630 = match v3.clone() {
            Adt3::C0(_, _, h4628, _, _, _, _, _) => {
                h4628
            }
        };
        let a4631 = 0u64;
        nat_rem(a4629, a4630, a4631)
    };
    let v10: u64 = {
        let a4633 = v6;
        let a4634 = match v3.clone() {
            Adt3::C0(_, _, h4632, _, _, _, _, _) => {
                h4632
            }
        };
        let a4635 = 0u64;
        nat_quot(a4633, a4634, a4635)
    };
    let v11: u64 = {
        let a4636 = v4;
        let a4637 = 0u64;
        if nat_eq(a4636, a4637) {
            match v1.clone() {
                Adt3::C0(h4638, _, _, _, _, _, _, _) => {
                    h4638
                }
            }
        } else {
            let a4639 = v4;
            let a4640 = 1u64;
            if nat_eq(a4639, a4640) {
                match v1.clone() {
                    Adt3::C0(_, h4641, _, _, _, _, _, _) => {
                        h4641
                    }
                }
            } else {
                let a4642 = v4;
                let a4643 = 2u64;
                if nat_eq(a4642, a4643) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4644, _, _, _, _, _) => {
                            h4644
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4645, _, _, _, _) => {
                            h4645
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4652 = v11;
        let a4653 = {
            let a4646 = v4;
            let a4647 = 0u64;
            if nat_eq(a4646, a4647) {
                v7
            } else {
                let a4648 = v4;
                let a4649 = 1u64;
                if nat_eq(a4648, a4649) {
                    v8
                } else {
                    let a4650 = v4;
                    let a4651 = 2u64;
                    if nat_eq(a4650, a4651) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4652, a4653)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f142(if if v12 {
        let a4654 = v4;
        let a4655 = 0u64;
        nat_eq(a4654, a4655)
    } else {
        false
    } {
        let a4656 = v7;
        let a4657 = v11;
        nat_sub(a4656, a4657)
    } else {
        v7
    }, if if v12 {
        let a4658 = v4;
        let a4659 = 1u64;
        nat_eq(a4658, a4659)
    } else {
        false
    } {
        let a4660 = v8;
        let a4661 = v11;
        nat_sub(a4660, a4661)
    } else {
        v8
    }, if if v12 {
        let a4662 = v4;
        let a4663 = 2u64;
        nat_eq(a4662, a4663)
    } else {
        false
    } {
        let a4664 = v9;
        let a4665 = v11;
        nat_sub(a4664, a4665)
    } else {
        v9
    }, if if v12 {
        let a4666 = v4;
        let a4667 = 3u64;
        nat_eq(a4666, a4667)
    } else {
        false
    } {
        let a4668 = v10;
        let a4669 = v11;
        nat_sub(a4668, a4669)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4670, _, _, _) => {
            h4670
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4671, _, _) => {
            h4671
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4672, _) => {
            h4672
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4673) => {
            h4673
        }
    })?, f92(v0, v3)?))
}

pub fn f153(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f57(v1.clone(), f240(v8, v3, match v1.clone() {
            Adt3::C0(h4674, _, _, _, _, _, _, _) => {
                h4674
            }
        })?, f240(v9, v4, match v1.clone() {
            Adt3::C0(_, h4675, _, _, _, _, _, _) => {
                h4675
            }
        })?, f240(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4676, _, _, _, _, _) => {
                h4676
            }
        })?, f240(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4677, _, _, _, _) => {
                h4677
            }
        })?)?, f92(v0, v2)?))
    } else {
        if if if if f241(v8, v3, match v1.clone() {
            Adt3::C0(h4678, _, _, _, _, _, _, _) => {
                h4678
            }
        })? {
            f241(v9, v4, match v1.clone() {
                Adt3::C0(_, h4679, _, _, _, _, _, _) => {
                    h4679
                }
            })?
        } else {
            false
        } {
            f241(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4680, _, _, _, _, _) => {
                    h4680
                }
            })?
        } else {
            false
        } {
            f241(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4681, _, _, _, _) => {
                    h4681
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f57(v1, f242(v8, v3)?, f242(v9, v4)?, f242(v10, v5)?, f242(v11, v6)?)?, f92(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f92(v0, v2)?))
        }
    }
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f57(v1.clone(), f243(v4, v3, match v1 {
        Adt3::C0(h4682, _, _, _, _, _, _, _) => {
            h4682
        }
    })?, v5, v6, v7)?, f92(v0, v2)?))
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4683 = v0;
    let a4684 = v3;
    if nat_lt(a4683, a4684) {
        Ok(Adt9::C1(false, {
            let a4685 = v0;
            let a4686 = 4u64;
            nat_mul(a4685, a4686)?
        }, {
            let a4687 = v0;
            let a4688 = 4u64;
            nat_mul(a4687, a4688)?
        }))
    } else {
        let v6: u64 = {
            let a4689 = v0;
            let a4690 = v3;
            nat_sub(a4689, a4690)
        };
        let v7: u64 = f92(v6, v1)?;
        let v8: u64 = f244(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4691 = v0;
    let a4692 = v3;
    if nat_lt(a4691, a4692) {
        Ok(Adt9::C1(true, 0u64, f92(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4693 = v0;
            let a4694 = v3;
            nat_sub(a4693, a4694)
        };
        let v5: u64 = f151(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f92(v4, v1)?, 3u64, v5))
    }
}

pub fn f157(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4696 = v1;
        let a4697 = match v3.clone() {
            Adt3::C0(h4695, _, _, _, _, _, _, _) => {
                h4695
            }
        };
        let a4698 = 0u64;
        nat_rem(a4696, a4697, a4698)
    };
    let v5: u64 = {
        let a4704 = {
            let a4700 = v1;
            let a4701 = match v3.clone() {
                Adt3::C0(h4699, _, _, _, _, _, _, _) => {
                    h4699
                }
            };
            let a4702 = 0u64;
            nat_quot(a4700, a4701, a4702)
        };
        let a4705 = match v3.clone() {
            Adt3::C0(_, h4703, _, _, _, _, _, _) => {
                h4703
            }
        };
        let a4706 = 0u64;
        nat_rem(a4704, a4705, a4706)
    };
    let v6: bool = f245(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f151(v1, v2, v3.clone())?
    } else {
        0u64
    }, f92(v1, v3)?))
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4707 = v3;
        let a4708 = 8u64;
        nat_add(a4707, a4708)?
    }, 0u64, f92(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f151(v0, v5, v2.clone())?
    }, 3u64, f92(v0, v2)?))
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4709 = v4;
    let a4710 = 0u64;
    if if nat_eq(a4709, a4710) {
        f123(v3)
    } else {
        false
    } {
        let m4711 = f216(v3);
        match m4711 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4712, _) => {
                        h4712
                    }
                };
                Ok(Adt9::C5(v3, f92(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4713) => {
                        h4713
                    }
                }, f92({
                    let a4714 = v0;
                    let a4715 = v6;
                    nat_mul(a4714, a4715)?
                }, v2.clone())?, {
                    let a4721 = {
                        let a4719 = {
                            let a4716 = v6;
                            let a4717 = 1u64;
                            nat_sub(a4716, a4717)
                        };
                        let a4720 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4718, _, _, _) => {
                                h4718
                            }
                        };
                        nat_mul(a4719, a4720)?
                    };
                    let a4722 = 4u64;
                    nat_add(a4721, a4722)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4723, _, _, _) => {
                        h4723
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f92(v0, v1)?, f92(v0, v2)?))
    }
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4725 = v0;
        let a4726 = match v2.clone() {
            Adt3::C0(h4724, _, _, _, _, _, _, _) => {
                h4724
            }
        };
        let a4727 = 0u64;
        nat_rem(a4725, a4726, a4727)
    };
    let v7: u64 = {
        let a4729 = v0;
        let a4730 = match v2.clone() {
            Adt3::C0(h4728, _, _, _, _, _, _, _) => {
                h4728
            }
        };
        let a4731 = 0u64;
        nat_quot(a4729, a4730, a4731)
    };
    let v8: u64 = {
        let a4733 = v7;
        let a4734 = match v2.clone() {
            Adt3::C0(_, h4732, _, _, _, _, _, _) => {
                h4732
            }
        };
        let a4735 = 0u64;
        nat_rem(a4733, a4734, a4735)
    };
    let v9: u64 = {
        let a4737 = v7;
        let a4738 = match v2.clone() {
            Adt3::C0(_, h4736, _, _, _, _, _, _) => {
                h4736
            }
        };
        let a4739 = 0u64;
        nat_quot(a4737, a4738, a4739)
    };
    let v10: u64 = {
        let a4741 = v9;
        let a4742 = match v2.clone() {
            Adt3::C0(_, _, h4740, _, _, _, _, _) => {
                h4740
            }
        };
        let a4743 = 0u64;
        nat_rem(a4741, a4742, a4743)
    };
    let v11: u64 = {
        let a4745 = v9;
        let a4746 = match v2.clone() {
            Adt3::C0(_, _, h4744, _, _, _, _, _) => {
                h4744
            }
        };
        let a4747 = 0u64;
        nat_quot(a4745, a4746, a4747)
    };
    let v12: u64 = f92(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4756 = {
                let a4751 = {
                    let a4748 = v10;
                    let a4749 = v4;
                    let a4750 = 0u64;
                    nat_quot(a4748, a4749, a4750)
                };
                let a4752 = v5;
                nat_mul(a4751, a4752)?
            };
            let a4757 = {
                let a4753 = v8;
                let a4754 = v4;
                let a4755 = 0u64;
                nat_quot(a4753, a4754, a4755)
            };
            nat_add(a4756, a4757)?
        };
        let v14: u64 = {
            let a4777 = v6;
            let a4778 = {
                let a4775 = match v1.clone() {
                    Adt3::C0(h4758, _, _, _, _, _, _, _) => {
                        h4758
                    }
                };
                let a4776 = {
                    let a4773 = {
                        let a4759 = v8;
                        let a4760 = v4;
                        let a4761 = 0u64;
                        nat_rem(a4759, a4760, a4761)
                    };
                    let a4774 = {
                        let a4771 = match v1.clone() {
                            Adt3::C0(_, h4762, _, _, _, _, _, _) => {
                                h4762
                            }
                        };
                        let a4772 = {
                            let a4769 = {
                                let a4763 = v10;
                                let a4764 = v4;
                                let a4765 = 0u64;
                                nat_rem(a4763, a4764, a4765)
                            };
                            let a4770 = {
                                let a4767 = match v1.clone() {
                                    Adt3::C0(_, _, h4766, _, _, _, _, _) => {
                                        h4766
                                    }
                                };
                                let a4768 = v13;
                                nat_mul(a4767, a4768)?
                            };
                            nat_add(a4769, a4770)?
                        };
                        nat_mul(a4771, a4772)?
                    };
                    nat_add(a4773, a4774)?
                };
                nat_mul(a4775, a4776)?
            };
            nat_add(a4777, a4778)?
        };
        Ok(Adt9::C1(false, f92(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4784 = {
                let a4782 = {
                    let a4779 = v11;
                    let a4780 = v5;
                    let a4781 = 0u64;
                    nat_rem(a4779, a4780, a4781)
                };
                let a4783 = v4;
                nat_mul(a4782, a4783)?
            };
            let a4785 = v8;
            nat_add(a4784, a4785)?
        };
        let v16: u64 = {
            let a4791 = {
                let a4789 = {
                    let a4786 = v11;
                    let a4787 = v5;
                    let a4788 = 0u64;
                    nat_quot(a4786, a4787, a4788)
                };
                let a4790 = v4;
                nat_mul(a4789, a4790)?
            };
            let a4792 = v10;
            nat_add(a4791, a4792)?
        };
        let a4794 = v15;
        let a4795 = match v1.clone() {
            Adt3::C0(_, h4793, _, _, _, _, _, _) => {
                h4793
            }
        };
        if if nat_lt(a4794, a4795) {
            let a4797 = v16;
            let a4798 = match v1.clone() {
                Adt3::C0(_, _, h4796, _, _, _, _, _) => {
                    h4796
                }
            };
            nat_lt(a4797, a4798)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f92({
                let a4807 = v6;
                let a4808 = {
                    let a4805 = match v1.clone() {
                        Adt3::C0(h4799, _, _, _, _, _, _, _) => {
                            h4799
                        }
                    };
                    let a4806 = {
                        let a4803 = v15;
                        let a4804 = {
                            let a4801 = match v1.clone() {
                                Adt3::C0(_, h4800, _, _, _, _, _, _) => {
                                    h4800
                                }
                            };
                            let a4802 = v16;
                            nat_mul(a4801, a4802)?
                        };
                        nat_add(a4803, a4804)?
                    };
                    nat_mul(a4805, a4806)?
                };
                nat_add(a4807, a4808)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f237(23u64, v4);
    let v6: u64 = {
        let a4809 = v0;
        let a4810 = v5;
        let a4811 = 0u64;
        nat_rem(a4809, a4810, a4811)
    };
    let v7: u64 = {
        let a4812 = v0;
        let a4813 = v5;
        let a4814 = 0u64;
        nat_quot(a4812, a4813, a4814)
    };
    let v8: Adt13 = Adt13::C1(f92(v7, v2)?);
    let a4815 = v6;
    let a4816 = 0u64;
    if nat_eq(a4815, a4816) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f92(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f176(v3), 4u64))
    } else {
        let a4817 = v6;
        let a4818 = 1u64;
        if nat_eq(a4817, a4818) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4819 = v4;
                let a4820 = 0u64;
                if nat_eq(a4819, a4820) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f176(v4)))
        }
    }
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f92(v0, v2.clone())?;
    let v4: u64 = f92(v0, v1.clone())?;
    let a4825 = {
        let a4822 = v0;
        let a4823 = match v1 {
            Adt3::C0(h4821, _, _, _, _, _, _, _) => {
                h4821
            }
        };
        let a4824 = 0u64;
        nat_rem(a4822, a4823, a4824)
    };
    let a4826 = 0u64;
    if nat_eq(a4825, a4826) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f92({
            let a4827 = v0;
            let a4828 = 1u64;
            nat_sub(a4827, a4828)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4829 = v3;
    let a4830 = 13u64;
    if nat_eq(a4829, a4830) {
        f246(v0, v1, v4)
    } else {
        let v5: u64 = f247(match v1.clone() {
            Adt3::C0(h4831, _, _, _, _, _, _, _) => {
                h4831
            }
        }, {
            let a4832 = v3;
            let a4833 = 16u64;
            nat_eq(a4832, a4833)
        })?;
        let v6: u64 = {
            let a4834 = v0;
            let a4835 = v5;
            let a4836 = 0u64;
            nat_quot(a4834, a4835, a4836)
        };
        let v7: u64 = {
            let a4837 = v0;
            let a4838 = v5;
            let a4839 = 0u64;
            nat_rem(a4837, a4838, a4839)
        };
        let v8: u64 = {
            let a4843 = 1u64;
            let a4844 = {
                let a4841 = match v1.clone() {
                    Adt3::C0(h4840, _, _, _, _, _, _, _) => {
                        h4840
                    }
                };
                let a4842 = 2u64;
                nat_mul(a4841, a4842)?
            };
            nat_add(a4843, a4844)?
        };
        let a4845 = v7;
        let a4846 = v8;
        if nat_lt(a4845, a4846) {
            f248(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f92(v6, v2)?;
            let a4847 = v3;
            let a4848 = 14u64;
            if nat_eq(a4847, a4848) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4849 = v7;
                let a4850 = v8;
                if nat_eq(a4849, a4850) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4853 = v7;
                    let a4854 = {
                        let a4851 = v8;
                        let a4852 = 1u64;
                        nat_add(a4851, a4852)?
                    };
                    if nat_eq(a4853, a4854) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4855, _, _, _, _, _, _, _) => {
                                h4855
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
    let a4856 = v0;
    let a4857 = 0u64;
    if nat_eq(a4856, a4857) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4864 = {
            let a4858 = v0;
            let a4859 = 1u64;
            nat_add(a4858, a4859)?
        };
        let a4865 = {
            let a4862 = 2u64;
            let a4863 = {
                let a4860 = v3;
                let a4861 = 2u64;
                nat_mul(a4860, a4861)?
            };
            nat_add(a4862, a4863)?
        };
        if nat_eq(a4864, a4865) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4866 = v0;
                let a4867 = 1u64;
                nat_sub(a4866, a4867)
            };
            let v5: u64 = {
                let a4868 = v4;
                let a4869 = 2u64;
                let a4870 = 0u64;
                nat_quot(a4868, a4869, a4870)
            };
            let a4874 = {
                let a4871 = v4;
                let a4872 = 2u64;
                let a4873 = 0u64;
                nat_rem(a4871, a4872, a4873)
            };
            let a4875 = 0u64;
            if nat_eq(a4874, a4875) {
                Ok(Adt9::C4(17u64, 0u64, f92(v5, v1)?, 4u64, f92(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4876 = v0;
    let a4877 = f41(v1, v2, v3)?;
    if nat_lt(a4876, a4877) {
        let a4878 = v0;
        let a4879 = v1;
        if nat_lt(a4878, a4879) {
            Ok(Adt9::C1(false, {
                let a4880 = 4u64;
                let a4881 = v0;
                nat_mul(a4880, a4881)?
            }, {
                let a4882 = 4u64;
                let a4883 = v0;
                nat_mul(a4882, a4883)?
            }))
        } else {
            let v4: u64 = {
                let a4884 = v0;
                let a4885 = v1;
                nat_sub(a4884, a4885)
            };
            let v5: u64 = f249(v4, v2)?;
            let a4886 = v5;
            let a4887 = 2u64;
            if nat_lt(a4886, a4887) {
                Ok(Adt9::C4(6u64, {
                    let a4888 = v5;
                    let a4889 = 0u64;
                    if nat_eq(a4888, a4889) {
                        4u64
                    } else {
                        5u64
                    }
                }, f250(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f251(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f252(v4, v2)? {
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

pub fn f166(v0: u64, v1: u64) -> u64 {
    let a4890 = v0;
    let a4891 = 0u64;
    if if nat_eq(a4890, a4891) {
        true
    } else {
        let a4892 = v1;
        let a4893 = 0u64;
        nat_eq(a4892, a4893)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f167(v0: u64, v1: u64) -> Adt11 {
    let a4894 = v0;
    let a4895 = 43u64;
    if nat_eq(a4894, a4895) {
        Adt11::C8(3u64)
    } else {
        let a4896 = v1;
        let a4897 = 43u64;
        if nat_eq(a4896, a4897) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f168(v0: u64) -> R<u64> {
    let a4900 = 2u64;
    let a4901 = {
        let a4898 = 6u64;
        let a4899 = v0;
        nat_mul(a4898, a4899)?
    };
    nat_add(a4900, a4901)
}

pub fn f169(v0: u64, v1: bool) -> R<u64> {
    let a4904 = {
        let a4902 = v0;
        let a4903 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4902, a4903)?
    };
    let a4905 = 1u64;
    nat_add(a4904, a4905)
}

pub fn f170(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4906 = v0;
    let a4907 = 16u64;
    if nat_lt(a4906, a4907) {
        let v9: u64 = {
            let a4908 = v0;
            let a4909 = 4u64;
            let a4910 = 0u64;
            nat_quot(a4908, a4909, a4910)
        };
        let v10: u64 = {
            let a4915 = {
                let a4913 = 8u64;
                let a4914 = {
                    let a4911 = 4u64;
                    let a4912 = v1;
                    nat_mul(a4911, a4912)?
                };
                nat_add(a4913, a4914)?
            };
            let a4916 = v9;
            nat_add(a4915, a4916)?
        };
        let v11: u64 = f57(v6, v9, v1, v2, 0u64)?;
        let a4920 = {
            let a4917 = v0;
            let a4918 = 4u64;
            let a4919 = 0u64;
            nat_rem(a4917, a4918, a4919)
        };
        let a4921 = 0u64;
        if nat_eq(a4920, a4921) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f57(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f57(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4925 = {
                let a4922 = v0;
                let a4923 = 4u64;
                let a4924 = 0u64;
                nat_rem(a4922, a4923, a4924)
            };
            let a4926 = 1u64;
            if nat_eq(a4925, a4926) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f57(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4930 = {
                    let a4927 = v0;
                    let a4928 = 4u64;
                    let a4929 = 0u64;
                    nat_rem(a4927, a4928, a4929)
                };
                let a4931 = 2u64;
                if nat_eq(a4930, a4931) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4932 = v9;
                        let a4933 = 0u64;
                        if nat_eq(a4932, a4933) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4934 = v8;
                        let a4935 = 2u64;
                        if nat_eq(a4934, a4935) {
                            let a4936 = v9;
                            let a4937 = 0u64;
                            if nat_eq(a4936, a4937) {
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
        let a4938 = v0;
        let a4939 = 28u64;
        if nat_lt(a4938, a4939) {
            let v12: u64 = {
                let a4940 = v0;
                let a4941 = 16u64;
                nat_sub(a4940, a4941)
            };
            let v13: u64 = {
                let a4942 = v12;
                let a4943 = 3u64;
                let a4944 = 0u64;
                nat_quot(a4942, a4943, a4944)
            };
            let v14: u64 = f57(v6, v13, v1, v2, 0u64)?;
            let a4948 = {
                let a4945 = v12;
                let a4946 = 3u64;
                let a4947 = 0u64;
                nat_rem(a4945, a4946, a4947)
            };
            let a4949 = 0u64;
            if nat_eq(a4948, a4949) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4953 = {
                    let a4950 = v12;
                    let a4951 = 3u64;
                    let a4952 = 0u64;
                    nat_rem(a4950, a4951, a4952)
                };
                let a4954 = 1u64;
                if nat_eq(a4953, a4954) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4955 = v13;
                        let a4956 = 0u64;
                        if nat_eq(a4955, a4956) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4957 = v0;
            let a4958 = 28u64;
            if nat_eq(a4957, a4958) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4959 = v0;
                    let a4960 = 29u64;
                    nat_sub(a4959, a4960)
                };
                let v16: u64 = {
                    let a4961 = v15;
                    let a4962 = 2u64;
                    let a4963 = 0u64;
                    nat_quot(a4961, a4962, a4963)
                };
                let v17: u64 = f57(v6, v16, v1, v2, 0u64)?;
                let a4967 = {
                    let a4964 = v15;
                    let a4965 = 2u64;
                    let a4966 = 0u64;
                    nat_rem(a4964, a4965, a4966)
                };
                let a4968 = 0u64;
                if nat_eq(a4967, a4968) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f171(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4969 = v0;
        let a4970 = 9u64;
        let a4971 = 0u64;
        nat_rem(a4969, a4970, a4971)
    };
    let v5: u64 = {
        let a4975 = {
            let a4972 = v0;
            let a4973 = 9u64;
            let a4974 = 0u64;
            nat_quot(a4972, a4973, a4974)
        };
        let a4976 = 4u64;
        let a4977 = 0u64;
        nat_rem(a4975, a4976, a4977)
    };
    let v6: bool = {
        let a4984 = {
            let a4981 = {
                let a4978 = v0;
                let a4979 = 36u64;
                let a4980 = 0u64;
                nat_quot(a4978, a4979, a4980)
            };
            let a4982 = 2u64;
            let a4983 = 0u64;
            nat_rem(a4981, a4982, a4983)
        };
        let a4985 = 0u64;
        nat_eq(a4984, a4985)
    };
    let v7: u64 = {
        let a4986 = v4;
        let a4987 = 4u64;
        if nat_lt(a4986, a4987) {
            v4
        } else {
            let a4988 = v4;
            let a4989 = 5u64;
            nat_sub(a4988, a4989)
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
    let a4990 = v4;
    let a4991 = 4u64;
    if nat_lt(a4990, a4991) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4992 = v4;
            let a4993 = 0u64;
            if nat_eq(a4992, a4993) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4994 = v4;
        let a4995 = 4u64;
        if nat_eq(a4994, a4995) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f172(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a4998 = {
            let a4996 = 3u64;
            let a4997 = v1;
            nat_mul(a4996, a4997)?
        };
        let a4999 = 4u64;
        nat_add(a4998, a4999)
    } else {
        let a5000 = v1;
        let a5001 = 3u64;
        nat_add(a5000, a5001)
    }
}

pub fn f173(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5010 = f60(match v0.clone() {
        Adt7::C0(h5002, _, _, _, _) => {
            h5002
        }
    }, match v1.clone() {
        Adt3::C0(h5003, _, _, _, _, _, _, _) => {
            h5003
        }
    }, f58(v3, v2.clone(), 0u64), {
        let a5007 = v4;
        let a5008 = {
            let m5006 = match v0.clone() {
                Adt7::C0(h5005, _, _, _, _) => {
                    h5005
                }
            };
            match m5006 {
                Adt6::C0(h5004, _, _, _) => {
                    h5004
                }
            }
        };
        let a5009 = 0u64;
        nat_rem(a5007, a5008, a5009)
    })?;
    match m5010 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5019 = f60(match v0.clone() {
                Adt7::C0(_, h5011, _, _, _) => {
                    h5011
                }
            }, match v1.clone() {
                Adt3::C0(_, h5012, _, _, _, _, _, _) => {
                    h5012
                }
            }, f58(v3, v2.clone(), 1u64), {
                let a5016 = v4;
                let a5017 = {
                    let m5015 = match v0 {
                        Adt7::C0(h5014, _, _, _, _) => {
                            h5014
                        }
                    };
                    match m5015 {
                        Adt6::C0(h5013, _, _, _) => {
                            h5013
                        }
                    }
                };
                let a5018 = 0u64;
                nat_quot(a5016, a5017, a5018)
            })?;
            match m5019 {
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

pub fn f174(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5021 = v2;
        let a5022 = f144(match v0.clone() {
            Adt6::C0(_, _, h5020, _) => {
                h5020
            }
        });
        nat_add(a5021, a5022)?
    };
    let v5: u64 = {
        let a5027 = {
            let a5024 = v3;
            let a5025 = match v0.clone() {
                Adt6::C0(_, _, _, h5023) => {
                    h5023
                }
            };
            nat_mul(a5024, a5025)?
        };
        let a5028 = f143(match v0.clone() {
            Adt6::C0(_, _, h5026, _) => {
                h5026
            }
        });
        nat_add(a5027, a5028)?
    };
    let v6: u64 = {
        let a5029 = v4;
        let a5030 = v5;
        nat_sub(a5029, a5030)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5031, _, _) => {
            h5031
        }
    };
    let v8: u64 = {
        let a5032 = v6;
        let a5033 = v7;
        let a5034 = 0u64;
        nat_quot(a5032, a5033, a5034)
    };
    let a5035 = v5;
    let a5036 = v4;
    if if if nat_le(a5035, a5036) {
        let a5040 = {
            let a5037 = v6;
            let a5038 = v7;
            let a5039 = 0u64;
            nat_rem(a5037, a5038, a5039)
        };
        let a5041 = 0u64;
        nat_eq(a5040, a5041)
    } else {
        false
    } {
        let a5042 = v8;
        let a5043 = v1;
        nat_lt(a5042, a5043)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f175(v0: u64) -> R<u64> {
    let a5046 = v0;
    let a5047 = {
        let a5044 = v0;
        let a5045 = 2u64;
        nat_add(a5044, a5045)?
    };
    nat_mul(a5046, a5047)
}

pub fn f176(v0: u64) -> u64 {
    let m5048 = f216(v0);
    match m5048 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5050 = match v1.clone() {
                Adt19::C0(_, h5049, _) => {
                    h5049
                }
            };
            let a5051 = 1u64;
            if if nat_eq(a5050, a5051) {
                let a5053 = match v1.clone() {
                    Adt19::C0(_, _, h5052) => {
                        h5052
                    }
                };
                let a5054 = 1u64;
                if if if nat_eq(a5053, a5054) {
                    true
                } else {
                    let a5056 = match v1.clone() {
                        Adt19::C0(_, _, h5055) => {
                            h5055
                        }
                    };
                    let a5057 = 2u64;
                    nat_eq(a5056, a5057)
                } {
                    true
                } else {
                    let a5059 = match v1.clone() {
                        Adt19::C0(_, _, h5058) => {
                            h5058
                        }
                    };
                    let a5060 = 4u64;
                    nat_eq(a5059, a5060)
                } {
                    true
                } else {
                    let a5062 = match v1.clone() {
                        Adt19::C0(_, _, h5061) => {
                            h5061
                        }
                    };
                    let a5063 = 8u64;
                    nat_eq(a5062, a5063)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h5064) => {
                        h5064
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f177(v0: u64) -> R<u64> {
    let a5067 = 3u64;
    let a5068 = {
        let a5065 = v0;
        let a5066 = 3u64;
        nat_mul(a5065, a5066)?
    };
    nat_add(a5067, a5068)
}

pub fn f178(v0: u64, v1: u64) -> R<u64> {
    if f179(v0) {
        let a5069 = 2u64;
        let a5070 = v1;
        nat_mul(a5069, a5070)
    } else {
        let a5071 = v1;
        let a5072 = f253(v0);
        let a5073 = 0u64;
        Ok(nat_quot(a5071, a5072, a5073))
    }
}

pub fn f179(v0: u64) -> bool {
    let a5074 = v0;
    let a5075 = 0u64;
    if if nat_eq(a5074, a5075) {
        true
    } else {
        let a5076 = v0;
        let a5077 = 1u64;
        nat_eq(a5076, a5077)
    } {
        true
    } else {
        let a5078 = v0;
        let a5079 = 30u64;
        nat_eq(a5078, a5079)
    }
}

pub fn f180(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m5080 = f216(v0);
    match m5080 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f142(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h5081, _, _, _) => {
                    h5081
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h5082, _, _) => {
                    h5082
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h5083, _) => {
                    h5083
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h5084) => {
                    h5084
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h5085) => {
                    h5085
                }
            }, f142({
                let a5087 = v2;
                let a5088 = match v6.clone() {
                    Adt19::C0(_, h5086, _) => {
                        h5086
                    }
                };
                nat_mul(a5087, a5088)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h5089, _, _, _) => {
                    h5089
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h5090, _, _) => {
                    h5090
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h5091, _) => {
                    h5091
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h5092) => {
                    h5092
                }
            })?, match v6 {
                Adt19::C0(_, h5093, _) => {
                    h5093
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h5094, _, _, _) => {
                    h5094
                }
            }))
        }
    }
}

pub fn f181(v0: Adt3, v1: Adt3) -> R<u64> {
    let a5102 = {
        let a5095 = 2u64;
        let a5096 = f182(v1.clone())?;
        nat_mul(a5095, a5096)?
    };
    let a5103 = {
        let a5100 = f182(v1)?;
        let a5101 = {
            let a5098 = 1u64;
            let a5099 = match v0 {
                Adt3::C0(h5097, _, _, _, _, _, _, _) => {
                    h5097
                }
            };
            nat_add(a5098, a5099)?
        };
        nat_mul(a5100, a5101)?
    };
    nat_add(a5102, a5103)
}

pub fn f182(v0: Adt3) -> R<u64> {
    if f48(match v0.clone() {
        Adt3::C0(h5104, _, _, _, _, _, _, _) => {
            h5104
        }
    }, match v0.clone() {
        Adt3::C0(_, h5105, _, _, _, _, _, _) => {
            h5105
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h5106, _, _, _, _, _) => {
            h5106
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h5107, _, _, _, _) => {
            h5107
        }
    }) {
        Ok(0u64)
    } else {
        let a5116 = match v0.clone() {
            Adt3::C0(h5108, _, _, _, _, _, _, _) => {
                h5108
            }
        };
        let a5117 = {
            let a5114 = match v0.clone() {
                Adt3::C0(_, h5109, _, _, _, _, _, _) => {
                    h5109
                }
            };
            let a5115 = {
                let a5112 = match v0.clone() {
                    Adt3::C0(_, _, h5110, _, _, _, _, _) => {
                        h5110
                    }
                };
                let a5113 = match v0 {
                    Adt3::C0(_, _, _, h5111, _, _, _, _) => {
                        h5111
                    }
                };
                nat_mul(a5112, a5113)?
            };
            nat_mul(a5114, a5115)?
        };
        nat_mul(a5116, a5117)
    }
}

pub fn f183(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f92(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f184(v0: u64, v1: u64) -> bool {
    let a5118 = v1;
    let a5119 = 2147483647u64;
    if nat_le(a5118, a5119) {
        let a5120 = v1;
        let a5121 = v0;
        nat_lt(a5120, a5121)
    } else {
        false
    }
}

pub fn f185(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5122 = f216(v0);
    match m5122 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5124 = v2;
                let a5125 = match v7.clone() {
                    Adt19::C0(_, h5123, _) => {
                        h5123
                    }
                };
                nat_mul(a5124, a5125)?
            };
            let m5138 = f254(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5126, _, _, _, _, _, _, _) => {
                    h5126
                }
            }, match v5.clone() {
                Adt3::C0(_, h5127, _, _, _, _, _, _) => {
                    h5127
                }
            }, match v5 {
                Adt3::C0(_, _, h5128, _, _, _, _, _) => {
                    h5128
                }
            }, {
                let a5131 = match v3.clone() {
                    Adt3::C0(h5129, _, _, _, _, _, _, _) => {
                        h5129
                    }
                };
                let a5132 = match v7.clone() {
                    Adt19::C0(_, h5130, _) => {
                        h5130
                    }
                };
                nat_mul(a5131, a5132)?
            }, match v3.clone() {
                Adt3::C0(_, h5133, _, _, _, _, _, _) => {
                    h5133
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5134, _, _, _) => {
                    h5134
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5135, _, _) => {
                    h5135
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5136, _) => {
                    h5136
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5137) => {
                    h5137
                }
            })?;
            match m5138 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5139 = v0;
                    let a5140 = 0u64;
                    if if nat_eq(a5139, a5140) {
                        true
                    } else {
                        let a5141 = v0;
                        let a5142 = 26u64;
                        nat_eq(a5141, a5142)
                    } {
                        Ok(Adt9::C1(false, v9, f92({
                            let a5146 = {
                                let a5144 = v1;
                                let a5145 = match v4.clone() {
                                    Adt3::C0(h5143, _, _, _, _, _, _, _) => {
                                        h5143
                                    }
                                };
                                nat_mul(a5144, a5145)?
                            };
                            let a5147 = v8;
                            nat_add(a5146, a5147)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5148) => {
                                h5148
                            }
                        }, f92({
                            let a5152 = {
                                let a5150 = v1;
                                let a5151 = match v4.clone() {
                                    Adt3::C0(h5149, _, _, _, _, _, _, _) => {
                                        h5149
                                    }
                                };
                                nat_mul(a5150, a5151)?
                            };
                            let a5153 = v8;
                            nat_add(a5152, a5153)?
                        }, v4.clone())?, {
                            let a5160 = {
                                let a5158 = {
                                    let a5155 = match v7 {
                                        Adt19::C0(_, h5154, _) => {
                                            h5154
                                        }
                                    };
                                    let a5156 = 1u64;
                                    nat_sub(a5155, a5156)
                                };
                                let a5159 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5157, _, _, _) => {
                                        h5157
                                    }
                                };
                                nat_mul(a5158, a5159)?
                            };
                            let a5161 = 4u64;
                            nat_add(a5160, a5161)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5162, _, _, _) => {
                                h5162
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f186(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5163 = v1;
    let a5164 = 0u64;
    if if nat_eq(a5163, a5164) {
        true
    } else {
        let a5165 = v2;
        let a5166 = 0u64;
        nat_eq(a5165, a5166)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f187(v0: u64, v1: u64) -> bool {
    let a5167 = v0;
    let a5168 = 4294967295u64;
    if nat_eq(a5167, a5168) {
        true
    } else {
        let a5169 = v0;
        let a5170 = 2147483648u64;
        if nat_lt(a5169, a5170) {
            let a5171 = v0;
            let a5172 = v1;
            nat_lt(a5171, a5172)
        } else {
            false
        }
    }
}

pub fn f188(v0: Adt7) -> R<u64> {
    let a5184 = if match v0.clone() {
        Adt7::C0(_, _, _, h5173, _) => {
            h5173
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5185 = {
        let a5182 = {
            let a5180 = {
                let m5176 = match v0.clone() {
                    Adt7::C0(h5175, _, _, _, _) => {
                        h5175
                    }
                };
                match m5176 {
                    Adt6::C0(h5174, _, _, _) => {
                        h5174
                    }
                }
            };
            let a5181 = {
                let m5179 = match v0.clone() {
                    Adt7::C0(_, h5178, _, _, _) => {
                        h5178
                    }
                };
                match m5179 {
                    Adt6::C0(h5177, _, _, _) => {
                        h5177
                    }
                }
            };
            nat_mul(a5180, a5181)?
        };
        let a5183 = f189(v0)?;
        nat_mul(a5182, a5183)?
    };
    nat_add(a5184, a5185)
}

pub fn f189(v0: Adt7) -> R<u64> {
    let a5190 = if match v0.clone() {
        Adt7::C0(_, _, _, h5186, _) => {
            h5186
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5191 = {
        let a5188 = match v0 {
            Adt7::C0(_, _, _, _, h5187) => {
                h5187
            }
        };
        let a5189 = 1u64;
        if nat_eq(a5188, a5189) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5190, a5191)
}

pub fn f190(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5196 = {
            let a5193 = v2;
            let a5194 = match v0.clone() {
                Adt6::C0(_, h5192, _, _) => {
                    h5192
                }
            };
            nat_mul(a5193, a5194)?
        };
        let a5197 = f143(match v0.clone() {
            Adt6::C0(_, _, h5195, _) => {
                h5195
            }
        });
        nat_add(a5196, a5197)?
    };
    let v4: u64 = f144(match v0.clone() {
        Adt6::C0(_, _, h5198, _) => {
            h5198
        }
    });
    let v5: u64 = {
        let a5202 = {
            let a5200 = v3;
            let a5201 = match v0 {
                Adt6::C0(h5199, _, _, _) => {
                    h5199
                }
            };
            nat_add(a5200, a5201)?
        };
        let a5203 = v4;
        nat_sub(a5202, a5203)
    };
    let a5208 = {
        let a5204 = v5;
        let a5205 = v1;
        if nat_lt(a5204, a5205) {
            v5
        } else {
            v1
        }
    };
    let a5209 = {
        let a5206 = v3;
        let a5207 = v4;
        nat_sub(a5206, a5207)
    };
    Ok(nat_sub(a5208, a5209))
}

pub fn f191(v0: u64) -> R<u64> {
    let a5212 = 2u64;
    let a5213 = {
        let a5210 = v0;
        let a5211 = 2u64;
        nat_mul(a5210, a5211)?
    };
    nat_add(a5212, a5213)
}

pub fn f192(v0: Adt6) -> R<u64> {
    let a5226 = 2u64;
    let a5227 = {
        let a5224 = 2u64;
        let a5225 = {
            let a5221 = {
                let a5218 = match v0.clone() {
                    Adt6::C0(h5214, _, _, _) => {
                        h5214
                    }
                };
                let a5219 = {
                    let a5216 = match v0.clone() {
                        Adt6::C0(_, h5215, _, _) => {
                            h5215
                        }
                    };
                    let a5217 = 1u64;
                    nat_sub(a5216, a5217)
                };
                nat_add(a5218, a5219)?
            };
            let a5222 = match v0 {
                Adt6::C0(_, h5220, _, _) => {
                    h5220
                }
            };
            let a5223 = 0u64;
            nat_quot(a5221, a5222, a5223)
        };
        nat_mul(a5224, a5225)?
    };
    nat_add(a5226, a5227)
}

pub fn f193(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5228 = v0;
    match m5228 {
        Adt16::C0(v19) => {
            f255(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5229 = v20;
            let a5230 = 66u64;
            if nat_eq(a5229, a5230) {
                f256(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f257(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f194(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5237 = v3;
        let a5238 = {
            let a5234 = f92({
                let a5231 = v0;
                let a5232 = 2u64;
                let a5233 = 0u64;
                nat_quot(a5231, a5232, a5233)
            }, v1)?;
            let a5235 = v2;
            let a5236 = 0u64;
            nat_quot(a5234, a5235, a5236)
        };
        nat_add(a5237, a5238)?
    };
    let a5242 = {
        let a5239 = v0;
        let a5240 = 2u64;
        let a5241 = 0u64;
        nat_rem(a5239, a5240, a5241)
    };
    let a5243 = 0u64;
    if nat_eq(a5242, a5243) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5244 = v4;
        let a5245 = 0u64;
        if nat_eq(a5244, a5245) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f195(v0: u64, v1: u64) -> R<u64> {
    let a5248 = 8u64;
    let a5249 = {
        let a5246 = f258(v0);
        let a5247 = v1;
        nat_mul(a5246, a5247)?
    };
    nat_add(a5248, a5249)
}

pub fn f196(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5250, _, _, _, _, _, _, _) => {
            h5250
        }
    };
    let v7: u64 = f258(v3);
    let v8: u64 = {
        let a5253 = 1u64;
        let a5254 = {
            let a5251 = v6;
            let a5252 = v7;
            nat_mul(a5251, a5252)?
        };
        nat_add(a5253, a5254)?
    };
    let a5255 = v0;
    let a5256 = 0u64;
    if nat_eq(a5255, a5256) {
        Ok(f259(0u64))
    } else {
        let a5257 = v0;
        let a5258 = v8;
        if nat_lt(a5257, a5258) {
            let v9: u64 = {
                let a5259 = v0;
                let a5260 = 1u64;
                nat_sub(a5259, a5260)
            };
            let v10: u64 = {
                let a5261 = v9;
                let a5262 = v7;
                let a5263 = 0u64;
                nat_rem(a5261, a5262, a5263)
            };
            let v11: u64 = f92({
                let a5269 = {
                    let a5264 = v1;
                    let a5265 = v6;
                    nat_mul(a5264, a5265)?
                };
                let a5270 = {
                    let a5266 = v9;
                    let a5267 = v7;
                    let a5268 = 0u64;
                    nat_quot(a5266, a5267, a5268)
                };
                nat_add(a5269, a5270)?
            }, v2)?;
            let a5271 = v3;
            let a5272 = 30u64;
            if nat_eq(a5271, a5272) {
                let a5273 = v10;
                let a5274 = 0u64;
                if nat_eq(a5273, a5274) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5275 = v10;
                    let a5276 = 1u64;
                    if nat_eq(a5275, a5276) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5277 = v10;
                        let a5278 = 2u64;
                        if nat_eq(a5277, a5278) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5279 = v10;
                let a5280 = 0u64;
                if nat_eq(a5279, a5280) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5281 = v10;
                    let a5282 = 1u64;
                    if nat_eq(a5281, a5282) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5283 = v0;
                let a5284 = v8;
                nat_sub(a5283, a5284)
            };
            let a5285 = v12;
            let a5286 = 0u64;
            if nat_eq(a5285, a5286) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5287 = v12;
                let a5288 = 1u64;
                if nat_eq(a5287, a5288) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5289 = v12;
                    let a5290 = 2u64;
                    if nat_eq(a5289, a5290) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5291 = v12;
                        let a5292 = 3u64;
                        if nat_eq(a5291, a5292) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5293 = v12;
                            let a5294 = 4u64;
                            if nat_eq(a5293, a5294) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5295 = v12;
                                let a5296 = 5u64;
                                if nat_eq(a5295, a5296) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5297 = v12;
                                    let a5298 = 6u64;
                                    if nat_eq(a5297, a5298) {
                                        if f260(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5299 = 3u64;
                                                let a5300 = v1;
                                                nat_add(a5299, a5300)?
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

pub fn f197(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5301 = v0;
        let a5302 = f103(v3);
        let a5303 = 0u64;
        nat_quot(a5301, a5302, a5303)
    };
    let v5: u64 = {
        let a5308 = 3u64;
        let a5309 = {
            let a5305 = v4;
            let a5306 = match v1.clone() {
                Adt3::C0(h5304, _, _, _, _, _, _, _) => {
                    h5304
                }
            };
            let a5307 = 0u64;
            nat_quot(a5305, a5306, a5307)
        };
        nat_add(a5308, a5309)?
    };
    let v6: u64 = f92(v4, v1)?;
    let v7: u64 = f92(v4, v2)?;
    let a5310 = v3;
    let a5311 = 30u64;
    if nat_eq(a5310, a5311) {
        let a5315 = {
            let a5312 = v0;
            let a5313 = 3u64;
            let a5314 = 0u64;
            nat_rem(a5312, a5313, a5314)
        };
        let a5316 = 0u64;
        if nat_eq(a5315, a5316) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5320 = {
                let a5317 = v0;
                let a5318 = 3u64;
                let a5319 = 0u64;
                nat_rem(a5317, a5318, a5319)
            };
            let a5321 = 1u64;
            if nat_eq(a5320, a5321) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f198(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5322 = v0;
        let a5323 = 8388608u64;
        let a5324 = 0u64;
        nat_quot(a5322, a5323, a5324)
    };
    let a5325 = v0;
    let a5326 = 2147483648u64;
    if if if nat_lt(a5325, a5326) {
        let a5327 = 127u64;
        let a5328 = v1;
        nat_le(a5327, a5328)
    } else {
        false
    } {
        let a5329 = v1;
        let a5330 = 190u64;
        nat_lt(a5329, a5330)
    } else {
        false
    } {
        let v2: u64 = {
            let a5334 = 8388608u64;
            let a5335 = {
                let a5331 = v0;
                let a5332 = 8388608u64;
                let a5333 = 0u64;
                nat_rem(a5331, a5332, a5333)
            };
            nat_add(a5334, a5335)?
        };
        let a5336 = v1;
        let a5337 = 150u64;
        if nat_lt(a5336, a5337) {
            let v3: u64 = f261({
                let a5338 = 150u64;
                let a5339 = v1;
                nat_sub(a5338, a5339)
            });
            let a5343 = {
                let a5340 = v2;
                let a5341 = v3;
                let a5342 = 0u64;
                nat_rem(a5340, a5341, a5342)
            };
            let a5344 = 0u64;
            if nat_eq(a5343, a5344) {
                let a5345 = v2;
                let a5346 = v3;
                let a5347 = 0u64;
                Ok(nat_quot(a5345, a5346, a5347))
            } else {
                Ok(0u64)
            }
        } else {
            let a5350 = f261({
                let a5348 = v1;
                let a5349 = 150u64;
                nat_sub(a5348, a5349)
            });
            let a5351 = v2;
            nat_mul(a5350, a5351)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f199(v0: Adt14, v1: Adt14) -> bool {
    let a5354 = match v0.clone() {
        Adt14::C0(_, h5352, _, _, _, _, _, _, _, _, _) => {
            h5352
        }
    };
    let a5355 = match v1.clone() {
        Adt14::C0(_, h5353, _, _, _, _, _, _, _, _, _) => {
            h5353
        }
    };
    if if if nat_eq(a5354, a5355) {
        let a5358 = match v0.clone() {
            Adt14::C0(_, _, h5356, _, _, _, _, _, _, _, _) => {
                h5356
            }
        };
        let a5359 = match v1.clone() {
            Adt14::C0(_, _, h5357, _, _, _, _, _, _, _, _) => {
                h5357
            }
        };
        nat_eq(a5358, a5359)
    } else {
        false
    } {
        let a5362 = match v0.clone() {
            Adt14::C0(_, _, _, h5360, _, _, _, _, _, _, _) => {
                h5360
            }
        };
        let a5363 = match v1.clone() {
            Adt14::C0(_, _, _, h5361, _, _, _, _, _, _, _) => {
                h5361
            }
        };
        nat_eq(a5362, a5363)
    } else {
        false
    } {
        let a5366 = match v0 {
            Adt14::C0(_, _, _, _, h5364, _, _, _, _, _, _) => {
                h5364
            }
        };
        let a5367 = match v1 {
            Adt14::C0(_, _, _, _, h5365, _, _, _, _, _, _) => {
                h5365
            }
        };
        nat_eq(a5366, a5367)
    } else {
        false
    }
}

pub fn f200(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5368 = v0;
    let a5369 = 0u64;
    if if if nat_eq(a5368, a5369) {
        true
    } else {
        let a5370 = v1;
        let a5371 = 0u64;
        nat_eq(a5370, a5371)
    } {
        true
    } else {
        let a5372 = v2;
        let a5373 = 0u64;
        nat_eq(a5372, a5373)
    } {
        true
    } else {
        let a5374 = v3;
        let a5375 = 0u64;
        nat_eq(a5374, a5375)
    }
}

pub fn f201(v0: Adt14, v1: Adt14) -> bool {
    if f207(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5376, _, _, _, _, _) => {
            h5376
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5377, _, _, _, _, _) => {
            h5377
        }
    }) {
        let a5379 = match v0 {
            Adt14::C0(_, h5378, _, _, _, _, _, _, _, _, _) => {
                h5378
            }
        };
        let a5380 = 2147483647u64;
        nat_le(a5379, a5380)
    } else {
        true
    }
}

pub fn f202(v0: Adt14) -> bool {
    let a5385 = {
        let a5382 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5381) => {
                h5381
            }
        };
        let a5383 = 4u64;
        let a5384 = 0u64;
        nat_rem(a5382, a5383, a5384)
    };
    let a5386 = 0u64;
    if if nat_eq(a5385, a5386) {
        f262(match v0.clone() {
            Adt14::C0(_, h5387, _, _, _, _, _, _, _, _, _) => {
                h5387
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5388, _, _, _, _, _) => {
                h5388
            }
        })
    } else {
        false
    } {
        f99(match v0.clone() {
            Adt14::C0(_, _, h5389, _, _, _, _, _, _, _, _) => {
                h5389
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5390, _, _, _, _, _, _, _) => {
                h5390
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5391, _, _, _, _, _, _) => {
                h5391
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5392, _, _, _, _) => {
                h5392
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5393, _, _, _) => {
                h5393
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5394, _, _) => {
                h5394
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f203(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5396 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5395, _) => {
            h5395
        }
    };
    let a5397 = 0u64;
    if nat_eq(a5396, a5397) {
        Ok(true)
    } else {
        let a5399 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5398, _) => {
                h5398
            }
        };
        let a5400 = 0u64;
        if nat_eq(a5399, a5400) {
            Ok(true)
        } else {
            let a5402 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5401) => {
                    h5401
                }
            };
            let a5403 = 0u64;
            if nat_eq(a5402, a5403) {
                Ok(true)
            } else {
                let a5405 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5404) => {
                        h5404
                    }
                };
                let a5406 = 0u64;
                if nat_eq(a5405, a5406) {
                    Ok(true)
                } else {
                    let a5412 = {
                        let a5409 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5407) => {
                                h5407
                            }
                        };
                        let a5410 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5408, _) => {
                                h5408
                            }
                        };
                        nat_add(a5409, a5410)?
                    };
                    let a5413 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5411) => {
                            h5411
                        }
                    };
                    if nat_le(a5412, a5413) {
                        Ok(true)
                    } else {
                        let a5419 = {
                            let a5416 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5414) => {
                                    h5414
                                }
                            };
                            let a5417 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5415, _) => {
                                    h5415
                                }
                            };
                            nat_add(a5416, a5417)?
                        };
                        let a5420 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5418) => {
                                h5418
                            }
                        };
                        Ok(nat_le(a5419, a5420))
                    }
                }
            }
        }
    }
}

pub fn f204(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f199(v0.clone(), v1.clone()) {
        let a5423 = match v0.clone() {
            Adt14::C0(h5421, _, _, _, _, _, _, _, _, _, _) => {
                h5421
            }
        };
        let a5424 = match v1.clone() {
            Adt14::C0(h5422, _, _, _, _, _, _, _, _, _, _) => {
                h5422
            }
        };
        nat_eq(a5423, a5424)
    } else {
        false
    } {
        let a5427 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5425) => {
                h5425
            }
        };
        let a5428 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5426) => {
                h5426
            }
        };
        nat_eq(a5427, a5428)
    } else {
        false
    } {
        let a5431 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5429, _, _, _, _, _) => {
                h5429
            }
        };
        let a5432 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5430, _, _, _, _, _) => {
                h5430
            }
        };
        nat_eq(a5431, a5432)
    } else {
        false
    } {
        let a5435 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5433, _, _, _, _) => {
                h5433
            }
        };
        let a5436 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5434, _, _, _, _) => {
                h5434
            }
        };
        nat_eq(a5435, a5436)
    } else {
        false
    } {
        let a5439 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5437, _, _, _) => {
                h5437
            }
        };
        let a5440 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5438, _, _, _) => {
                h5438
            }
        };
        nat_eq(a5439, a5440)
    } else {
        false
    } {
        let a5443 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5441, _, _) => {
                h5441
            }
        };
        let a5444 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5442, _, _) => {
                h5442
            }
        };
        nat_eq(a5443, a5444)
    } else {
        false
    }
}

pub fn f205(v0: Adt14) -> R<bool> {
    if f263(match v0.clone() {
        Adt14::C0(_, h5445, _, _, _, _, _, _, _, _, _) => {
            h5445
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5446, _, _, _, _, _) => {
            h5446
        }
    }) {
        f100(f264(match v0.clone() {
            Adt14::C0(_, h5447, _, _, _, _, _, _, _, _, _) => {
                h5447
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5448, _, _, _, _, _) => {
                h5448
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5449, _, _, _, _, _, _, _, _) => {
                h5449
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5450, _, _, _, _, _, _, _) => {
                h5450
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5451, _, _, _, _, _, _) => {
                h5451
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5452, _, _, _, _) => {
                h5452
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5453, _, _, _) => {
                h5453
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5454, _, _) => {
                h5454
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f206(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5455 = v1;
    let a5456 = 0u64;
    if if nat_eq(a5455, a5456) {
        true
    } else {
        let a5457 = v2;
        let a5458 = 0u64;
        nat_eq(a5457, a5458)
    } {
        Ok(0u64)
    } else {
        let a5461 = f101(v1, v2, v3)?;
        let a5462 = {
            let a5459 = v2;
            let a5460 = f265(v0, v1)?;
            nat_mul(a5459, a5460)?
        };
        nat_add(a5461, a5462)
    }
}

pub fn f207(v0: u64, v1: u64) -> bool {
    let a5463 = v0;
    let a5464 = 4u64;
    if nat_eq(a5463, a5464) {
        let a5465 = v1;
        let a5466 = 4u64;
        nat_eq(a5465, a5466)
    } else {
        false
    }
}

pub fn f208(v0: u64) -> R<u64> {
    let a5467 = 6u64;
    let a5468 = v0;
    nat_add(a5467, a5468)
}

pub fn f209(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5469, _, _, _, _, _, _, _, _, _, _) => {
            h5469
        }
    }, match v0.clone() {
        Adt14::C0(_, h5470, _, _, _, _, _, _, _, _, _) => {
            h5470
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5471, _, _, _, _, _, _, _, _) => {
            h5471
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5472, _, _, _, _, _, _, _) => {
            h5472
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5473, _, _, _, _, _, _) => {
            h5473
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5474, _, _, _, _) => {
            h5474
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5475, _, _, _) => {
            h5475
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5476, _, _) => {
            h5476
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5477, _) => {
            h5477
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5478) => {
            h5478
        }
    })
}

pub fn f210(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5479 = v0;
    let a5480 = 2147483647u64;
    if if if nat_le(a5479, a5480) {
        let a5481 = 0u64;
        let a5482 = v3;
        if nat_lt(a5481, a5482) {
            let a5483 = v3;
            let a5484 = 2147483647u64;
            nat_le(a5483, a5484)
        } else {
            false
        }
    } else {
        false
    } {
        let a5489 = {
            let a5487 = {
                let a5485 = v2;
                let a5486 = v3;
                nat_add(a5485, a5486)?
            };
            let a5488 = 1u64;
            nat_sub(a5487, a5488)
        };
        let a5490 = 2147483647u64;
        nat_le(a5489, a5490)
    } else {
        false
    } {
        let a5495 = {
            let a5493 = {
                let a5491 = v0;
                let a5492 = v1;
                nat_mul(a5491, a5492)?
            };
            let a5494 = f266(v2, v3)?;
            nat_mul(a5493, a5494)?
        };
        let a5496 = 9223372036854775807u64;
        Ok(nat_le(a5495, a5496))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Adt14) -> R<u64> {
    let a5527 = {
        let a5519 = {
            let a5511 = {
                let a5503 = 4u64;
                let a5504 = {
                    let a5501 = {
                        let a5498 = match v0.clone() {
                            Adt14::C0(_, h5497, _, _, _, _, _, _, _, _, _) => {
                                h5497
                            }
                        };
                        let a5499 = 1u64;
                        nat_sub(a5498, a5499)
                    };
                    let a5502 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5500, _, _, _, _, _) => {
                            h5500
                        }
                    };
                    nat_mul(a5501, a5502)?
                };
                nat_add(a5503, a5504)?
            };
            let a5512 = {
                let a5509 = {
                    let a5506 = match v0.clone() {
                        Adt14::C0(_, _, h5505, _, _, _, _, _, _, _, _) => {
                            h5505
                        }
                    };
                    let a5507 = 1u64;
                    nat_sub(a5506, a5507)
                };
                let a5510 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5508, _, _, _, _) => {
                        h5508
                    }
                };
                nat_mul(a5509, a5510)?
            };
            nat_add(a5511, a5512)?
        };
        let a5520 = {
            let a5517 = {
                let a5514 = match v0.clone() {
                    Adt14::C0(_, _, _, h5513, _, _, _, _, _, _, _) => {
                        h5513
                    }
                };
                let a5515 = 1u64;
                nat_sub(a5514, a5515)
            };
            let a5518 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5516, _, _, _) => {
                    h5516
                }
            };
            nat_mul(a5517, a5518)?
        };
        nat_add(a5519, a5520)?
    };
    let a5528 = {
        let a5525 = {
            let a5522 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5521, _, _, _, _, _, _) => {
                    h5521
                }
            };
            let a5523 = 1u64;
            nat_sub(a5522, a5523)
        };
        let a5526 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5524, _, _) => {
                h5524
            }
        };
        nat_mul(a5525, a5526)?
    };
    nat_add(a5527, a5528)
}

pub fn f212(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5529 = v0;
    let a5530 = 0u64;
    if if if if nat_eq(a5529, a5530) {
        true
    } else {
        let a5531 = v1;
        let a5532 = 0u64;
        nat_eq(a5531, a5532)
    } {
        true
    } else {
        let a5533 = v2;
        let a5534 = 0u64;
        nat_eq(a5533, a5534)
    } {
        true
    } else {
        let a5535 = v3;
        let a5536 = 0u64;
        nat_eq(a5535, a5536)
    } {
        Ok(0u64)
    } else {
        let a5537 = f267(v0, v1, v2, v3, v5)?;
        let a5538 = f268(v0, v1, v2, v3, v4)?;
        nat_add(a5537, a5538)
    }
}

pub fn f213(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5539 = v0;
    let a5540 = 0u64;
    if if nat_eq(a5539, a5540) {
        true
    } else {
        let a5541 = v1;
        let a5542 = 0u64;
        nat_eq(a5541, a5542)
    } {
        Ok(0u64)
    } else {
        let a5545 = f101(v0, v1, v2)?;
        let a5546 = {
            let a5543 = v1;
            let a5544 = f269(v0)?;
            nat_mul(a5543, a5544)?
        };
        nat_add(a5545, a5546)
    }
}

pub fn f214(v0: u64, v1: u64, v2: u64) -> bool {
    let a5547 = v0;
    let a5548 = 1u64;
    if nat_le(a5547, a5548) {
        true
    } else {
        let a5552 = {
            let a5549 = v1;
            let a5550 = v2;
            let a5551 = 1u64;
            nat_rem(a5549, a5550, a5551)
        };
        let a5553 = 0u64;
        nat_eq(a5552, a5553)
    }
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5554 = v1;
    let a5555 = 1u64;
    if if if nat_le(a5554, a5555) {
        true
    } else {
        let a5556 = v0;
        let a5557 = v2;
        nat_le(a5556, a5557)
    } {
        let a5558 = v3;
        let a5559 = 1u64;
        if nat_le(a5558, a5559) {
            true
        } else {
            let a5566 = {
                let a5564 = v0;
                let a5565 = {
                    let a5562 = {
                        let a5560 = v1;
                        let a5561 = 1u64;
                        nat_sub(a5560, a5561)
                    };
                    let a5563 = v2;
                    nat_mul(a5562, a5563)?
                };
                nat_add(a5564, a5565)?
            };
            let a5567 = v4;
            nat_le(a5566, a5567)
        }
    } else {
        false
    } {
        let a5568 = v5;
        let a5569 = 1u64;
        if nat_le(a5568, a5569) {
            Ok(true)
        } else {
            let a5582 = {
                let a5580 = {
                    let a5574 = v0;
                    let a5575 = {
                        let a5572 = {
                            let a5570 = v1;
                            let a5571 = 1u64;
                            nat_sub(a5570, a5571)
                        };
                        let a5573 = v2;
                        nat_mul(a5572, a5573)?
                    };
                    nat_add(a5574, a5575)?
                };
                let a5581 = {
                    let a5578 = {
                        let a5576 = v3;
                        let a5577 = 1u64;
                        nat_sub(a5576, a5577)
                    };
                    let a5579 = v4;
                    nat_mul(a5578, a5579)?
                };
                nat_add(a5580, a5581)?
            };
            let a5583 = v6;
            Ok(nat_le(a5582, a5583))
        }
    } else {
        Ok(false)
    }
}

pub fn f216(v0: u64) -> Adt18 {
    let a5584 = v0;
    let a5585 = 0u64;
    if nat_eq(a5584, a5585) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5586 = v0;
        let a5587 = 1u64;
        if nat_eq(a5586, a5587) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5588 = v0;
            let a5589 = 2u64;
            if nat_eq(a5588, a5589) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5590 = v0;
                let a5591 = 3u64;
                if nat_eq(a5590, a5591) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5592 = v0;
                    let a5593 = 6u64;
                    if nat_eq(a5592, a5593) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5594 = v0;
                        let a5595 = 7u64;
                        if nat_eq(a5594, a5595) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5596 = v0;
                            let a5597 = 8u64;
                            if nat_eq(a5596, a5597) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5598 = v0;
                                let a5599 = 9u64;
                                if nat_eq(a5598, a5599) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5600 = v0;
                                    let a5601 = 10u64;
                                    if nat_eq(a5600, a5601) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5602 = v0;
                                        let a5603 = 11u64;
                                        if nat_eq(a5602, a5603) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5604 = v0;
                                            let a5605 = 12u64;
                                            if nat_eq(a5604, a5605) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5606 = v0;
                                                let a5607 = 13u64;
                                                if nat_eq(a5606, a5607) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5608 = v0;
                                                    let a5609 = 14u64;
                                                    if nat_eq(a5608, a5609) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5610 = v0;
                                                        let a5611 = 15u64;
                                                        if nat_eq(a5610, a5611) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5612 = v0;
                                                            let a5613 = 16u64;
                                                            if nat_eq(a5612, a5613) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5614 = v0;
                                                                let a5615 = 17u64;
                                                                if nat_eq(a5614, a5615) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5616 = v0;
                                                                    let a5617 = 18u64;
                                                                    if nat_eq(a5616, a5617) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5618 = v0;
                                                                        let a5619 = 19u64;
                                                                        if nat_eq(a5618, a5619) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5620 = v0;
                                                                            let a5621 = 20u64;
                                                                            if nat_eq(a5620, a5621) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5622 = v0;
                                                                                let a5623 = 21u64;
                                                                                if nat_eq(a5622, a5623) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5624 = v0;
                                                                                    let a5625 = 22u64;
                                                                                    if nat_eq(a5624, a5625) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5626 = v0;
                                                                                        let a5627 = 23u64;
                                                                                        if nat_eq(a5626, a5627) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5628 = v0;
                                                                                            let a5629 = 24u64;
                                                                                            if nat_eq(a5628, a5629) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5630 = v0;
                                                                                                let a5631 = 25u64;
                                                                                                if nat_eq(a5630, a5631) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5632 = v0;
                                                                                                    let a5633 = 26u64;
                                                                                                    if nat_eq(a5632, a5633) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5634 = v0;
                                                                                                        let a5635 = 27u64;
                                                                                                        if nat_eq(a5634, a5635) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5636 = v0;
                                                                                                            let a5637 = 28u64;
                                                                                                            if nat_eq(a5636, a5637) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5638 = v0;
                                                                                                                let a5639 = 29u64;
                                                                                                                if nat_eq(a5638, a5639) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5640 = v0;
                                                                                                                    let a5641 = 30u64;
                                                                                                                    if nat_eq(a5640, a5641) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5642 = v0;
                                                                                                                        let a5643 = 34u64;
                                                                                                                        if nat_eq(a5642, a5643) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5644 = v0;
                                                                                                                            let a5645 = 35u64;
                                                                                                                            if nat_eq(a5644, a5645) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5646 = v0;
                                                                                                                                let a5647 = 39u64;
                                                                                                                                if nat_eq(a5646, a5647) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5648 = v0;
                                                                                                                                    let a5649 = 40u64;
                                                                                                                                    if nat_eq(a5648, a5649) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5650 = v0;
                                                                                                                                        let a5651 = 41u64;
                                                                                                                                        if nat_eq(a5650, a5651) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5652 = v0;
                                                                                                                                            let a5653 = 42u64;
                                                                                                                                            if nat_eq(a5652, a5653) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5654 = v0;
                                                                                                                                                let a5655 = 43u64;
                                                                                                                                                if nat_eq(a5654, a5655) {
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

pub fn f217(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5657 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5656, _, _, _, _, _, _) => {
                    h5656
                }
            };
            let a5658 = 1u64;
            if if if if if nat_le(a5657, a5658) {
                true
            } else {
                let a5660 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5659, _, _) => {
                        h5659
                    }
                };
                let a5661 = v1;
                nat_eq(a5660, a5661)
            } {
                let a5663 = match v0.clone() {
                    Adt2::C0(_, h5662, _, _, _, _, _, _, _, _, _) => {
                        h5662
                    }
                };
                let a5664 = 1u64;
                if nat_le(a5663, a5664) {
                    true
                } else {
                    let a5669 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5665, _, _, _, _, _) => {
                            h5665
                        }
                    };
                    let a5670 = {
                        let a5667 = v1;
                        let a5668 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5666, _, _, _, _, _, _) => {
                                h5666
                            }
                        };
                        nat_mul(a5667, a5668)?
                    };
                    nat_eq(a5669, a5670)
                }
            } else {
                false
            } {
                let a5672 = match v0.clone() {
                    Adt2::C0(_, _, h5671, _, _, _, _, _, _, _, _) => {
                        h5671
                    }
                };
                let a5673 = 1u64;
                if nat_le(a5672, a5673) {
                    true
                } else {
                    let a5681 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5674, _, _, _, _) => {
                            h5674
                        }
                    };
                    let a5682 = {
                        let a5679 = {
                            let a5676 = v1;
                            let a5677 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5675, _, _, _, _, _, _) => {
                                    h5675
                                }
                            };
                            nat_mul(a5676, a5677)?
                        };
                        let a5680 = match v0.clone() {
                            Adt2::C0(_, h5678, _, _, _, _, _, _, _, _, _) => {
                                h5678
                            }
                        };
                        nat_mul(a5679, a5680)?
                    };
                    nat_eq(a5681, a5682)
                }
            } else {
                false
            } {
                let a5684 = match v0.clone() {
                    Adt2::C0(_, _, _, h5683, _, _, _, _, _, _, _) => {
                        h5683
                    }
                };
                let a5685 = 1u64;
                if nat_le(a5684, a5685) {
                    true
                } else {
                    let a5696 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5686, _, _, _) => {
                            h5686
                        }
                    };
                    let a5697 = {
                        let a5694 = {
                            let a5691 = {
                                let a5688 = v1;
                                let a5689 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5687, _, _, _, _, _, _) => {
                                        h5687
                                    }
                                };
                                nat_mul(a5688, a5689)?
                            };
                            let a5692 = match v0.clone() {
                                Adt2::C0(_, h5690, _, _, _, _, _, _, _, _, _) => {
                                    h5690
                                }
                            };
                            nat_mul(a5691, a5692)?
                        };
                        let a5695 = match v0.clone() {
                            Adt2::C0(_, _, h5693, _, _, _, _, _, _, _, _) => {
                                h5693
                            }
                        };
                        nat_mul(a5694, a5695)?
                    };
                    nat_eq(a5696, a5697)
                }
            } else {
                false
            } {
                let a5711 = {
                    let a5708 = {
                        let a5705 = {
                            let a5702 = {
                                let a5699 = v1;
                                let a5700 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5698, _, _, _, _, _, _) => {
                                        h5698
                                    }
                                };
                                nat_mul(a5699, a5700)?
                            };
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
                let a5712 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5710, _) => {
                        h5710
                    }
                };
                Ok(nat_le(a5711, a5712))
            } else {
                Ok(false)
            }
        } else {
            let a5714 = match v0.clone() {
                Adt2::C0(_, _, _, h5713, _, _, _, _, _, _, _) => {
                    h5713
                }
            };
            let a5715 = 1u64;
            if if if if if nat_le(a5714, a5715) {
                true
            } else {
                let a5717 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5716, _, _, _) => {
                        h5716
                    }
                };
                let a5718 = v1;
                nat_eq(a5717, a5718)
            } {
                let a5720 = match v0.clone() {
                    Adt2::C0(_, h5719, _, _, _, _, _, _, _, _, _) => {
                        h5719
                    }
                };
                let a5721 = 1u64;
                if nat_le(a5720, a5721) {
                    true
                } else {
                    let a5726 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5722, _, _, _, _, _) => {
                            h5722
                        }
                    };
                    let a5727 = {
                        let a5724 = v1;
                        let a5725 = match v0.clone() {
                            Adt2::C0(_, _, _, h5723, _, _, _, _, _, _, _) => {
                                h5723
                            }
                        };
                        nat_mul(a5724, a5725)?
                    };
                    nat_eq(a5726, a5727)
                }
            } else {
                false
            } {
                let a5729 = match v0.clone() {
                    Adt2::C0(_, _, h5728, _, _, _, _, _, _, _, _) => {
                        h5728
                    }
                };
                let a5730 = 1u64;
                if nat_le(a5729, a5730) {
                    true
                } else {
                    let a5738 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5731, _, _, _, _) => {
                            h5731
                        }
                    };
                    let a5739 = {
                        let a5736 = {
                            let a5733 = v1;
                            let a5734 = match v0.clone() {
                                Adt2::C0(_, _, _, h5732, _, _, _, _, _, _, _) => {
                                    h5732
                                }
                            };
                            nat_mul(a5733, a5734)?
                        };
                        let a5737 = match v0.clone() {
                            Adt2::C0(_, h5735, _, _, _, _, _, _, _, _, _) => {
                                h5735
                            }
                        };
                        nat_mul(a5736, a5737)?
                    };
                    nat_eq(a5738, a5739)
                }
            } else {
                false
            } {
                let a5741 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5740, _, _, _, _, _, _) => {
                        h5740
                    }
                };
                let a5742 = 1u64;
                if nat_le(a5741, a5742) {
                    true
                } else {
                    let a5753 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5743, _, _) => {
                            h5743
                        }
                    };
                    let a5754 = {
                        let a5751 = {
                            let a5748 = {
                                let a5745 = v1;
                                let a5746 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5744, _, _, _, _, _, _, _) => {
                                        h5744
                                    }
                                };
                                nat_mul(a5745, a5746)?
                            };
                            let a5749 = match v0.clone() {
                                Adt2::C0(_, h5747, _, _, _, _, _, _, _, _, _) => {
                                    h5747
                                }
                            };
                            nat_mul(a5748, a5749)?
                        };
                        let a5752 = match v0.clone() {
                            Adt2::C0(_, _, h5750, _, _, _, _, _, _, _, _) => {
                                h5750
                            }
                        };
                        nat_mul(a5751, a5752)?
                    };
                    nat_eq(a5753, a5754)
                }
            } else {
                false
            } {
                let a5768 = {
                    let a5765 = {
                        let a5762 = {
                            let a5759 = {
                                let a5756 = v1;
                                let a5757 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5755, _, _, _, _, _, _, _) => {
                                        h5755
                                    }
                                };
                                nat_mul(a5756, a5757)?
                            };
                            let a5760 = match v0.clone() {
                                Adt2::C0(_, h5758, _, _, _, _, _, _, _, _, _) => {
                                    h5758
                                }
                            };
                            nat_mul(a5759, a5760)?
                        };
                        let a5763 = match v0.clone() {
                            Adt2::C0(_, _, h5761, _, _, _, _, _, _, _, _) => {
                                h5761
                            }
                        };
                        nat_mul(a5762, a5763)?
                    };
                    let a5766 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5764, _, _, _, _, _, _) => {
                            h5764
                        }
                    };
                    nat_mul(a5765, a5766)?
                };
                let a5769 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5767, _) => {
                        h5767
                    }
                };
                Ok(nat_le(a5768, a5769))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5771 = match v0.clone() {
            Adt2::C0(_, h5770, _, _, _, _, _, _, _, _, _) => {
                h5770
            }
        };
        let a5772 = 1u64;
        if if if if if nat_le(a5771, a5772) {
            true
        } else {
            let a5774 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5773, _, _, _, _, _) => {
                    h5773
                }
            };
            let a5775 = v1;
            nat_eq(a5774, a5775)
        } {
            let a5777 = match v0.clone() {
                Adt2::C0(_, _, h5776, _, _, _, _, _, _, _, _) => {
                    h5776
                }
            };
            let a5778 = 1u64;
            if nat_le(a5777, a5778) {
                true
            } else {
                let a5783 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5779, _, _, _, _) => {
                        h5779
                    }
                };
                let a5784 = {
                    let a5781 = v1;
                    let a5782 = match v0.clone() {
                        Adt2::C0(_, h5780, _, _, _, _, _, _, _, _, _) => {
                            h5780
                        }
                    };
                    nat_mul(a5781, a5782)?
                };
                nat_eq(a5783, a5784)
            }
        } else {
            false
        } {
            let a5786 = match v0.clone() {
                Adt2::C0(_, _, _, h5785, _, _, _, _, _, _, _) => {
                    h5785
                }
            };
            let a5787 = 1u64;
            if nat_le(a5786, a5787) {
                true
            } else {
                let a5795 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5788, _, _, _) => {
                        h5788
                    }
                };
                let a5796 = {
                    let a5793 = {
                        let a5790 = v1;
                        let a5791 = match v0.clone() {
                            Adt2::C0(_, h5789, _, _, _, _, _, _, _, _, _) => {
                                h5789
                            }
                        };
                        nat_mul(a5790, a5791)?
                    };
                    let a5794 = match v0.clone() {
                        Adt2::C0(_, _, h5792, _, _, _, _, _, _, _, _) => {
                            h5792
                        }
                    };
                    nat_mul(a5793, a5794)?
                };
                nat_eq(a5795, a5796)
            }
        } else {
            false
        } {
            let a5798 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5797, _, _, _, _, _, _) => {
                    h5797
                }
            };
            let a5799 = 1u64;
            if nat_le(a5798, a5799) {
                true
            } else {
                let a5810 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5800, _, _) => {
                        h5800
                    }
                };
                let a5811 = {
                    let a5808 = {
                        let a5805 = {
                            let a5802 = v1;
                            let a5803 = match v0.clone() {
                                Adt2::C0(_, h5801, _, _, _, _, _, _, _, _, _) => {
                                    h5801
                                }
                            };
                            nat_mul(a5802, a5803)?
                        };
                        let a5806 = match v0.clone() {
                            Adt2::C0(_, _, h5804, _, _, _, _, _, _, _, _) => {
                                h5804
                            }
                        };
                        nat_mul(a5805, a5806)?
                    };
                    let a5809 = match v0.clone() {
                        Adt2::C0(_, _, _, h5807, _, _, _, _, _, _, _) => {
                            h5807
                        }
                    };
                    nat_mul(a5808, a5809)?
                };
                nat_eq(a5810, a5811)
            }
        } else {
            false
        } {
            let a5825 = {
                let a5822 = {
                    let a5819 = {
                        let a5816 = {
                            let a5813 = v1;
                            let a5814 = match v0.clone() {
                                Adt2::C0(_, h5812, _, _, _, _, _, _, _, _, _) => {
                                    h5812
                                }
                            };
                            nat_mul(a5813, a5814)?
                        };
                        let a5817 = match v0.clone() {
                            Adt2::C0(_, _, h5815, _, _, _, _, _, _, _, _) => {
                                h5815
                            }
                        };
                        nat_mul(a5816, a5817)?
                    };
                    let a5820 = match v0.clone() {
                        Adt2::C0(_, _, _, h5818, _, _, _, _, _, _, _) => {
                            h5818
                        }
                    };
                    nat_mul(a5819, a5820)?
                };
                let a5823 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5821, _, _, _, _, _, _) => {
                        h5821
                    }
                };
                nat_mul(a5822, a5823)?
            };
            let a5826 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5824, _) => {
                    h5824
                }
            };
            Ok(nat_le(a5825, a5826))
        } else {
            Ok(false)
        }
    }
}

pub fn f218(v0: Adt2, v1: u64) -> R<bool> {
    let a5828 = match v0.clone() {
        Adt2::C0(h5827, _, _, _, _, _, _, _, _, _, _) => {
            h5827
        }
    };
    let a5829 = 0u64;
    if if nat_eq(a5828, a5829) {
        f38(v0.clone())?
    } else {
        false
    } {
        let a5830 = v1;
        let a5831 = 1u64;
        if nat_le(a5830, a5831) {
            Ok(true)
        } else {
            let a5836 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5832, _, _, _, _) => {
                    h5832
                }
            };
            let a5837 = {
                let a5834 = 4u64;
                let a5835 = match v0 {
                    Adt2::C0(_, h5833, _, _, _, _, _, _, _, _, _) => {
                        h5833
                    }
                };
                nat_mul(a5834, a5835)?
            };
            Ok(nat_eq(a5836, a5837))
        }
    } else {
        Ok(false)
    }
}

pub fn f219(v0: Adt2, v1: u64) -> R<bool> {
    let a5839 = match v0.clone() {
        Adt2::C0(_, _, _, h5838, _, _, _, _, _, _, _) => {
            h5838
        }
    };
    let a5840 = 1u64;
    if if if nat_eq(a5839, a5840) {
        let a5842 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5841, _, _, _, _, _, _) => {
                h5841
            }
        };
        let a5843 = 1u64;
        nat_eq(a5842, a5843)
    } else {
        false
    } {
        let a5845 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5844, _, _, _, _, _) => {
                h5844
            }
        };
        let a5846 = v1;
        nat_eq(a5845, a5846)
    } else {
        false
    } {
        let a5848 = match v0.clone() {
            Adt2::C0(_, _, h5847, _, _, _, _, _, _, _, _) => {
                h5847
            }
        };
        let a5849 = 1u64;
        if nat_eq(a5848, a5849) {
            Ok(true)
        } else {
            let a5854 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5850, _, _, _, _) => {
                    h5850
                }
            };
            let a5855 = {
                let a5852 = match v0 {
                    Adt2::C0(_, h5851, _, _, _, _, _, _, _, _, _) => {
                        h5851
                    }
                };
                let a5853 = v1;
                nat_mul(a5852, a5853)?
            };
            Ok(nat_eq(a5854, a5855))
        }
    } else {
        Ok(false)
    }
}

pub fn f220(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5859 = {
            let a5856 = v2;
            let a5857 = 1u64;
            nat_sub(a5856, a5857)
        };
        let a5860 = match v0.clone() {
            Adt6::C0(_, h5858, _, _) => {
                h5858
            }
        };
        nat_mul(a5859, a5860)?
    };
    let a5862 = 0u64;
    let a5863 = match v0.clone() {
        Adt6::C0(h5861, _, _, _) => {
            h5861
        }
    };
    if if if if if if if if nat_lt(a5862, a5863) {
        let a5865 = match v0.clone() {
            Adt6::C0(h5864, _, _, _) => {
                h5864
            }
        };
        let a5866 = 2147483648u64;
        nat_lt(a5865, a5866)
    } else {
        false
    } {
        let a5868 = 0u64;
        let a5869 = match v0.clone() {
            Adt6::C0(_, h5867, _, _) => {
                h5867
            }
        };
        nat_lt(a5868, a5869)
    } else {
        false
    } {
        let a5871 = match v0.clone() {
            Adt6::C0(_, h5870, _, _) => {
                h5870
            }
        };
        let a5872 = 2147483648u64;
        nat_lt(a5871, a5872)
    } else {
        false
    } {
        let a5873 = v1;
        let a5874 = 2147483648u64;
        nat_lt(a5873, a5874)
    } else {
        false
    } {
        let a5875 = 0u64;
        let a5876 = v2;
        nat_lt(a5875, a5876)
    } else {
        false
    } {
        let a5877 = v3;
        let a5878 = 2147483648u64;
        nat_lt(a5877, a5878)
    } else {
        false
    } {
        let a5890 = {
            let a5885 = {
                let a5882 = v3;
                let a5883 = {
                    let a5880 = match v0.clone() {
                        Adt6::C0(h5879, _, _, _) => {
                            h5879
                        }
                    };
                    let a5881 = 1u64;
                    nat_sub(a5880, a5881)
                };
                nat_add(a5882, a5883)?
            };
            let a5886 = f143(match v0.clone() {
                Adt6::C0(_, _, h5884, _) => {
                    h5884
                }
            });
            nat_add(a5885, a5886)?
        };
        let a5891 = {
            let a5888 = 2147483647u64;
            let a5889 = f144(match v0.clone() {
                Adt6::C0(_, _, h5887, _) => {
                    h5887
                }
            });
            nat_add(a5888, a5889)?
        };
        nat_le(a5890, a5891)
    } else {
        false
    } {
        let a5893 = match v0 {
            Adt6::C0(_, _, h5892, _) => {
                h5892
            }
        };
        let a5894 = 2147483648u64;
        Ok(!nat_eq(a5893, a5894))
    } else {
        Ok(false)
    }
}

pub fn f221(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f270(match v0.clone() {
        Adt2::C0(h5895, _, _, _, _, _, _, _, _, _, _) => {
            h5895
        }
    }, match v0.clone() {
        Adt2::C0(_, h5896, _, _, _, _, _, _, _, _, _) => {
            h5896
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5897, _, _, _, _, _, _, _, _) => {
            h5897
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5898, _, _, _, _, _, _, _) => {
            h5898
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5899, _, _, _, _, _, _) => {
            h5899
        }
    }, match v1.clone() {
        Adt2::C0(h5900, _, _, _, _, _, _, _, _, _, _) => {
            h5900
        }
    }, match v1.clone() {
        Adt2::C0(_, h5901, _, _, _, _, _, _, _, _, _) => {
            h5901
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5902, _, _, _, _, _, _, _, _) => {
            h5902
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5903, _, _, _, _, _, _, _) => {
            h5903
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5904, _, _, _, _, _, _) => {
            h5904
        }
    }, match v2.clone() {
        Adt2::C0(h5905, _, _, _, _, _, _, _, _, _, _) => {
            h5905
        }
    }, match v2.clone() {
        Adt2::C0(_, h5906, _, _, _, _, _, _, _, _, _) => {
            h5906
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5907, _, _, _, _, _, _, _, _) => {
            h5907
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5908, _, _, _, _, _, _, _) => {
            h5908
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5909, _, _, _, _, _, _) => {
            h5909
        }
    }) {
        let m5911 = f216(match v0.clone() {
            Adt2::C0(h5910, _, _, _, _, _, _, _, _, _, _) => {
                h5910
            }
        });
        match m5911 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5917 = {
                    let a5914 = match v0.clone() {
                        Adt2::C0(_, h5912, _, _, _, _, _, _, _, _, _) => {
                            h5912
                        }
                    };
                    let a5915 = match v3.clone() {
                        Adt19::C0(_, h5913, _) => {
                            h5913
                        }
                    };
                    let a5916 = 0u64;
                    nat_rem(a5914, a5915, a5916)
                };
                let a5918 = 0u64;
                if if nat_eq(a5917, a5918) {
                    if f271(match v0.clone() {
                        Adt2::C0(_, h5919, _, _, _, _, _, _, _, _, _) => {
                            h5919
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5920, _, _, _, _, _, _, _, _, _) => {
                            h5920
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5921, _, _, _, _, _, _, _, _) => {
                            h5921
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5922, _, _, _, _, _, _, _) => {
                            h5922
                        }
                    }) {
                        true
                    } else {
                        let a5924 = 4u64;
                        let a5925 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5923, _, _, _, _, _) => {
                                h5923
                            }
                        };
                        if if nat_le(a5924, a5925) {
                            let a5927 = match v3.clone() {
                                Adt19::C0(_, h5926, _) => {
                                    h5926
                                }
                            };
                            let a5928 = 1u64;
                            if nat_eq(a5927, a5928) {
                                true
                            } else {
                                let a5931 = match v3.clone() {
                                    Adt19::C0(_, _, h5929) => {
                                        h5929
                                    }
                                };
                                let a5932 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5930, _, _, _, _, _) => {
                                        h5930
                                    }
                                };
                                nat_le(a5931, a5932)
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
                        Adt19::C0(_, h5933, _) => {
                            h5933
                        }
                    });
                    Ok(Adt10::C4(if f271(match v0.clone() {
                        Adt2::C0(_, h5934, _, _, _, _, _, _, _, _, _) => {
                            h5934
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5935, _, _, _, _, _, _, _, _, _) => {
                            h5935
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5936, _, _, _, _, _, _, _, _) => {
                            h5936
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5937, _, _, _, _, _, _, _) => {
                            h5937
                        }
                    }) {
                        0u64
                    } else {
                        f181(v4.clone(), f25(v1.clone(), 1u64))?
                    }, 4u64, v4, f25(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5938, _, _, _, _, _, _, _, _, _, _) => {
                            h5938
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

pub fn f222(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f223(match v0.clone() {
        Adt2::C0(h5939, _, _, _, _, _, _, _, _, _, _) => {
            h5939
        }
    }) {
        let a5945 = {
            let a5942 = match v0.clone() {
                Adt2::C0(_, h5940, _, _, _, _, _, _, _, _, _) => {
                    h5940
                }
            };
            let a5943 = f253(match v0.clone() {
                Adt2::C0(h5941, _, _, _, _, _, _, _, _, _, _) => {
                    h5941
                }
            });
            let a5944 = 1u64;
            nat_rem(a5942, a5943, a5944)
        };
        let a5946 = 0u64;
        nat_eq(a5945, a5946)
    } else {
        false
    } {
        let a5948 = match v1.clone() {
            Adt2::C0(h5947, _, _, _, _, _, _, _, _, _, _) => {
                h5947
            }
        };
        let a5949 = 26u64;
        nat_eq(a5948, a5949)
    } else {
        false
    } {
        let a5951 = match v2.clone() {
            Adt2::C0(h5950, _, _, _, _, _, _, _, _, _, _) => {
                h5950
            }
        };
        let a5952 = 0u64;
        nat_eq(a5951, a5952)
    } else {
        false
    } {
        let a5955 = match v0.clone() {
            Adt2::C0(_, h5953, _, _, _, _, _, _, _, _, _) => {
                h5953
            }
        };
        let a5956 = match v2.clone() {
            Adt2::C0(_, h5954, _, _, _, _, _, _, _, _, _) => {
                h5954
            }
        };
        nat_eq(a5955, a5956)
    } else {
        false
    } {
        let a5959 = match v1.clone() {
            Adt2::C0(_, h5957, _, _, _, _, _, _, _, _, _) => {
                h5957
            }
        };
        let a5960 = match v0.clone() {
            Adt2::C0(_, _, h5958, _, _, _, _, _, _, _, _) => {
                h5958
            }
        };
        nat_le(a5959, a5960)
    } else {
        false
    } {
        let a5962 = match v0.clone() {
            Adt2::C0(_, _, _, h5961, _, _, _, _, _, _, _) => {
                h5961
            }
        };
        let a5963 = 1u64;
        nat_eq(a5962, a5963)
    } else {
        false
    } {
        let a5965 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5964, _, _, _, _, _, _) => {
                h5964
            }
        };
        let a5966 = 1u64;
        nat_eq(a5965, a5966)
    } else {
        false
    } {
        let a5968 = match v2.clone() {
            Adt2::C0(_, _, _, h5967, _, _, _, _, _, _, _) => {
                h5967
            }
        };
        let a5969 = 1u64;
        nat_eq(a5968, a5969)
    } else {
        false
    } {
        let a5971 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5970, _, _, _, _, _, _) => {
                h5970
            }
        };
        let a5972 = 1u64;
        nat_eq(a5971, a5972)
    } else {
        false
    } {
        let a5974 = match v1.clone() {
            Adt2::C0(_, _, h5973, _, _, _, _, _, _, _, _) => {
                h5973
            }
        };
        let a5975 = 1u64;
        nat_eq(a5974, a5975)
    } else {
        false
    } {
        let a5977 = match v1.clone() {
            Adt2::C0(_, _, _, h5976, _, _, _, _, _, _, _) => {
                h5976
            }
        };
        let a5978 = 1u64;
        nat_eq(a5977, a5978)
    } else {
        false
    } {
        let a5980 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5979, _, _, _, _, _, _) => {
                h5979
            }
        };
        let a5981 = 1u64;
        nat_eq(a5980, a5981)
    } else {
        false
    } {
        if f48(match v2.clone() {
            Adt2::C0(_, h5982, _, _, _, _, _, _, _, _, _) => {
                h5982
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5983, _, _, _, _, _, _, _, _) => {
                h5983
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5984, _, _, _, _, _, _, _) => {
                h5984
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5985, _, _, _, _, _, _) => {
                h5985
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5987 = match v1.clone() {
                Adt2::C0(_, h5986, _, _, _, _, _, _, _, _, _) => {
                    h5986
                }
            };
            let a5988 = 0u64;
            if nat_eq(a5987, a5988) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5990 = match v0.clone() {
                    Adt2::C0(_, h5989, _, _, _, _, _, _, _, _, _) => {
                        h5989
                    }
                };
                let a5991 = 2147483647u64;
                if if if if if if nat_le(a5990, a5991) {
                    let a5993 = match v1.clone() {
                        Adt2::C0(_, h5992, _, _, _, _, _, _, _, _, _) => {
                            h5992
                        }
                    };
                    let a5994 = 2147483647u64;
                    nat_le(a5993, a5994)
                } else {
                    false
                } {
                    let a5996 = 4u64;
                    let a5997 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5995, _, _, _, _, _) => {
                            h5995
                        }
                    };
                    nat_le(a5996, a5997)
                } else {
                    false
                } {
                    let a6003 = {
                        let a6000 = match v2.clone() {
                            Adt2::C0(_, h5998, _, _, _, _, _, _, _, _, _) => {
                                h5998
                            }
                        };
                        let a6001 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5999, _, _, _, _, _) => {
                                h5999
                            }
                        };
                        nat_mul(a6000, a6001)?
                    };
                    let a6004 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6002, _, _, _, _) => {
                            h6002
                        }
                    };
                    nat_le(a6003, a6004)
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
                        let a6017 = {
                            let a6008 = {
                                let a6006 = 2u64;
                                let a6007 = match v1.clone() {
                                    Adt2::C0(_, h6005, _, _, _, _, _, _, _, _, _) => {
                                        h6005
                                    }
                                };
                                nat_mul(a6006, a6007)?
                            };
                            let a6009 = f23(v2.clone(), 1u64)?;
                            nat_add(a6008, a6009)?
                        };
                        let a6018 = {
                            let a6015 = match v1.clone() {
                                Adt2::C0(_, h6010, _, _, _, _, _, _, _, _, _) => {
                                    h6010
                                }
                            };
                            let a6016 = {
                                let a6013 = 1u64;
                                let a6014 = f178(match v0.clone() {
                                    Adt2::C0(h6011, _, _, _, _, _, _, _, _, _, _) => {
                                        h6011
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h6012, _, _, _, _, _, _, _, _, _) => {
                                        h6012
                                    }
                                })?;
                                nat_add(a6013, a6014)?
                            };
                            nat_mul(a6015, a6016)?
                        };
                        nat_add(a6017, a6018)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h6019, _, _, _, _, _, _, _, _, _) => {
                            h6019
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6020, _, _, _, _, _, _, _, _, _) => {
                            h6020
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6021, _, _, _, _, _) => {
                            h6021
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6022, _, _, _, _) => {
                            h6022
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6023, _, _, _) => {
                            h6023
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6024, _, _) => {
                            h6024
                        }
                    }), f25(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h6025, _, _, _, _, _, _, _, _, _, _) => {
                            h6025
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

pub fn f223(v0: u64) -> bool {
    let a6026 = v0;
    let a6027 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a6026, a6027) {
        true
    } else {
        let a6028 = v0;
        let a6029 = 1u64;
        nat_eq(a6028, a6029)
    } {
        true
    } else {
        let a6030 = v0;
        let a6031 = 2u64;
        nat_eq(a6030, a6031)
    } {
        true
    } else {
        let a6032 = v0;
        let a6033 = 3u64;
        nat_eq(a6032, a6033)
    } {
        true
    } else {
        let a6034 = v0;
        let a6035 = 6u64;
        nat_eq(a6034, a6035)
    } {
        true
    } else {
        let a6036 = v0;
        let a6037 = 7u64;
        nat_eq(a6036, a6037)
    } {
        true
    } else {
        let a6038 = v0;
        let a6039 = 8u64;
        nat_eq(a6038, a6039)
    } {
        true
    } else {
        let a6040 = v0;
        let a6041 = 10u64;
        nat_eq(a6040, a6041)
    } {
        true
    } else {
        let a6042 = v0;
        let a6043 = 11u64;
        nat_eq(a6042, a6043)
    } {
        true
    } else {
        let a6044 = v0;
        let a6045 = 12u64;
        nat_eq(a6044, a6045)
    } {
        true
    } else {
        let a6046 = v0;
        let a6047 = 13u64;
        nat_eq(a6046, a6047)
    } {
        true
    } else {
        let a6048 = v0;
        let a6049 = 14u64;
        nat_eq(a6048, a6049)
    } {
        true
    } else {
        let a6050 = v0;
        let a6051 = 16u64;
        nat_eq(a6050, a6051)
    } {
        true
    } else {
        let a6052 = v0;
        let a6053 = 17u64;
        nat_eq(a6052, a6053)
    } {
        true
    } else {
        let a6054 = v0;
        let a6055 = 18u64;
        nat_eq(a6054, a6055)
    } {
        true
    } else {
        let a6056 = v0;
        let a6057 = 19u64;
        nat_eq(a6056, a6057)
    } {
        true
    } else {
        let a6058 = v0;
        let a6059 = 20u64;
        nat_eq(a6058, a6059)
    } {
        true
    } else {
        let a6060 = v0;
        let a6061 = 21u64;
        nat_eq(a6060, a6061)
    } {
        true
    } else {
        let a6062 = v0;
        let a6063 = 22u64;
        nat_eq(a6062, a6063)
    } {
        true
    } else {
        let a6064 = v0;
        let a6065 = 23u64;
        nat_eq(a6064, a6065)
    } {
        true
    } else {
        let a6066 = v0;
        let a6067 = 26u64;
        nat_eq(a6066, a6067)
    } {
        true
    } else {
        let a6068 = v0;
        let a6069 = 29u64;
        nat_eq(a6068, a6069)
    } {
        true
    } else {
        let a6070 = v0;
        let a6071 = 30u64;
        nat_eq(a6070, a6071)
    } {
        true
    } else {
        let a6072 = v0;
        let a6073 = 34u64;
        nat_eq(a6072, a6073)
    } {
        true
    } else {
        let a6074 = v0;
        let a6075 = 35u64;
        nat_eq(a6074, a6075)
    } {
        true
    } else {
        let a6076 = v0;
        let a6077 = 39u64;
        nat_eq(a6076, a6077)
    } {
        true
    } else {
        let a6078 = v0;
        let a6079 = 40u64;
        nat_eq(a6078, a6079)
    } {
        true
    } else {
        let a6080 = v0;
        let a6081 = 41u64;
        nat_eq(a6080, a6081)
    } {
        true
    } else {
        let a6082 = v0;
        let a6083 = 42u64;
        nat_eq(a6082, a6083)
    } {
        true
    } else {
        let a6084 = v0;
        let a6085 = 43u64;
        nat_eq(a6084, a6085)
    }
}

pub fn f224(v0: u64) -> bool {
    if f179(v0) {
        true
    } else {
        let a6086 = v0;
        let a6087 = 43u64;
        nat_eq(a6086, a6087)
    }
}

pub fn f225(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a6090 = match v1.clone() {
        Adt2::C0(h6088, _, _, _, _, _, _, _, _, _, _) => {
            h6088
        }
    };
    let a6091 = match v3.clone() {
        Adt2::C0(h6089, _, _, _, _, _, _, _, _, _, _) => {
            h6089
        }
    };
    if if nat_eq(a6090, a6091) {
        let a6092 = v0;
        let a6093 = 33u64;
        if nat_eq(a6092, a6093) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m6095 = f216(match v1.clone() {
            Adt2::C0(h6094, _, _, _, _, _, _, _, _, _, _) => {
                h6094
            }
        });
        match m6095 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h6096, _) => {
                        h6096
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a6101 = {
                    let a6098 = match v1.clone() {
                        Adt2::C0(_, h6097, _, _, _, _, _, _, _, _, _) => {
                            h6097
                        }
                    };
                    let a6099 = v5;
                    let a6100 = 1u64;
                    nat_rem(a6098, a6099, a6100)
                };
                let a6102 = 0u64;
                if if if nat_eq(a6101, a6102) {
                    let a6107 = {
                        let a6104 = match v3.clone() {
                            Adt2::C0(_, h6103, _, _, _, _, _, _, _, _, _) => {
                                h6103
                            }
                        };
                        let a6105 = v5;
                        let a6106 = 1u64;
                        nat_rem(a6104, a6105, a6106)
                    };
                    let a6108 = 0u64;
                    nat_eq(a6107, a6108)
                } else {
                    false
                } {
                    let a6109 = v6;
                    let a6110 = f23(v3.clone(), v5)?;
                    nat_eq(a6109, a6110)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6111) => {
                            h6111
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

pub fn f226(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f48(v4, v5, v6, v7) {
        f48(v0, v1, v2, v3)
    } else {
        let a6115 = {
            let a6112 = v0;
            let a6113 = v4;
            let a6114 = 1u64;
            nat_rem(a6112, a6113, a6114)
        };
        let a6116 = 0u64;
        if if if nat_eq(a6115, a6116) {
            let a6120 = {
                let a6117 = v1;
                let a6118 = v5;
                let a6119 = 1u64;
                nat_rem(a6117, a6118, a6119)
            };
            let a6121 = 0u64;
            nat_eq(a6120, a6121)
        } else {
            false
        } {
            let a6125 = {
                let a6122 = v2;
                let a6123 = v6;
                let a6124 = 1u64;
                nat_rem(a6122, a6123, a6124)
            };
            let a6126 = 0u64;
            nat_eq(a6125, a6126)
        } else {
            false
        } {
            let a6130 = {
                let a6127 = v3;
                let a6128 = v7;
                let a6129 = 1u64;
                nat_rem(a6127, a6128, a6129)
            };
            let a6131 = 0u64;
            nat_eq(a6130, a6131)
        } else {
            false
        }
    }
}

pub fn f227(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6132 = v0;
    let a6133 = v1;
    if nat_eq(a6132, a6133) {
        let a6136 = v4;
        let a6137 = {
            let a6134 = v2;
            let a6135 = v3;
            nat_add(a6134, a6135)?
        };
        Ok(nat_eq(a6136, a6137))
    } else {
        let a6138 = v2;
        let a6139 = v3;
        if nat_eq(a6138, a6139) {
            let a6140 = v4;
            let a6141 = v2;
            Ok(nat_eq(a6140, a6141))
        } else {
            Ok(false)
        }
    }
}

pub fn f228(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6144 = {
            let a6142 = v0;
            let a6143 = f144(v1);
            nat_add(a6142, a6143)?
        };
        let a6145 = f144(v2);
        nat_add(a6144, a6145)?
    };
    let v5: u64 = {
        let a6146 = f143(v1);
        let a6147 = f143(v2);
        nat_add(a6146, a6147)?
    };
    let a6148 = v1;
    let a6149 = 4294967295u64;
    if if nat_le(a6148, a6149) {
        let a6150 = v2;
        let a6151 = 4294967295u64;
        nat_le(a6150, a6151)
    } else {
        false
    } {
        let a6152 = v5;
        let a6153 = v4;
        if nat_le(a6152, a6153) {
            let a6156 = v3;
            let a6157 = {
                let a6154 = v4;
                let a6155 = v5;
                nat_sub(a6154, a6155)
            };
            Ok(nat_eq(a6156, a6157))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f229(v0: u64, v1: u64) -> R<bool> {
    let a6158 = v1;
    let a6159 = 4294967295u64;
    if nat_le(a6158, a6159) {
        let a6162 = {
            let a6160 = f144(v1);
            let a6161 = f143(v1);
            nat_add(a6160, a6161)?
        };
        let a6163 = v0;
        Ok(nat_lt(a6162, a6163))
    } else {
        Ok(false)
    }
}

pub fn f230(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f272(v0.clone(), false)?;
    let v6: u64 = f272(v0, true)?;
    let a6164 = v6;
    let a6165 = v1;
    if if nat_le(a6164, a6165) {
        let a6172 = {
            let a6170 = {
                let a6166 = v1;
                let a6167 = v5;
                nat_add(a6166, a6167)?
            };
            let a6171 = {
                let a6168 = v3;
                let a6169 = 0u64;
                if nat_eq(a6168, a6169) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6170, a6171)?
        };
        let a6173 = v2;
        nat_le(a6172, a6173)
    } else {
        false
    } {
        if v4 {
            let a6178 = {
                let a6176 = {
                    let a6174 = v1;
                    let a6175 = v5;
                    nat_add(a6174, a6175)?
                };
                let a6177 = v6;
                nat_sub(a6176, a6177)
            };
            let a6179 = v2;
            Ok(nat_lt(a6178, a6179))
        } else {
            let a6184 = {
                let a6182 = {
                    let a6180 = v1;
                    let a6181 = v5;
                    nat_add(a6180, a6181)?
                };
                let a6183 = v6;
                nat_sub(a6182, a6183)
            };
            let a6185 = v2;
            Ok(nat_le(a6184, a6185))
        }
    } else {
        Ok(false)
    }
}

pub fn f231(v0: u64, v1: u64) -> bool {
    let a6186 = v0;
    let a6187 = 0u64;
    if if nat_eq(a6186, a6187) {
        true
    } else {
        let a6188 = v0;
        let a6189 = 1u64;
        nat_eq(a6188, a6189)
    } {
        let a6190 = v1;
        let a6191 = 4294967296u64;
        nat_lt(a6190, a6191)
    } else {
        false
    }
}

pub fn f232(v0: u64, v1: u64) -> R<u64> {
    let a6196 = {
        let a6194 = v0;
        let a6195 = {
            let a6192 = v1;
            let a6193 = 1u64;
            nat_sub(a6192, a6193)
        };
        nat_add(a6194, a6195)?
    };
    let a6197 = v1;
    let a6198 = 0u64;
    Ok(nat_quot(a6196, a6197, a6198))
}

pub fn f233(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f48(match v0.clone() {
        Adt2::C0(_, h6199, _, _, _, _, _, _, _, _, _) => {
            h6199
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6200, _, _, _, _, _, _, _, _) => {
            h6200
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6201, _, _, _, _, _, _, _) => {
            h6201
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6202, _, _, _, _, _, _) => {
            h6202
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6205 = match v1.clone() {
                Adt2::C0(_, h6203, _, _, _, _, _, _, _, _, _) => {
                    h6203
                }
            };
            let a6206 = match v1.clone() {
                Adt2::C0(_, _, h6204, _, _, _, _, _, _, _, _) => {
                    h6204
                }
            };
            nat_mul(a6205, a6206)?
        };
        let v5: u64 = {
            let a6208 = v4;
            let a6209 = match v1.clone() {
                Adt2::C0(_, _, _, h6207, _, _, _, _, _, _, _) => {
                    h6207
                }
            };
            nat_mul(a6208, a6209)?
        };
        let a6222 = {
            let a6220 = {
                let a6211 = match v0.clone() {
                    Adt2::C0(_, h6210, _, _, _, _, _, _, _, _, _) => {
                        h6210
                    }
                };
                let a6212 = 1u64;
                nat_sub(a6211, a6212)
            };
            let a6221 = {
                let a6218 = f273(match v0.clone() {
                    Adt2::C0(_, _, h6213, _, _, _, _, _, _, _, _) => {
                        h6213
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6214, _, _, _, _, _, _, _, _, _) => {
                        h6214
                    }
                })?;
                let a6219 = f273(match v0 {
                    Adt2::C0(_, _, _, h6215, _, _, _, _, _, _, _) => {
                        h6215
                    }
                }, v2, {
                    let a6216 = v3;
                    let a6217 = v5;
                    nat_mul(a6216, a6217)?
                }, v4)?;
                nat_add(a6218, a6219)?
            };
            nat_add(a6220, a6221)?
        };
        let a6223 = f23(v1, 1u64)?;
        Ok(nat_lt(a6222, a6223))
    }
}

pub fn f234(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6224 = v0;
    let a6225 = 2147483648u64;
    if if if if nat_lt(a6224, a6225) {
        let a6226 = v1;
        let a6227 = 2147483648u64;
        nat_lt(a6226, a6227)
    } else {
        false
    } {
        let a6230 = {
            let a6228 = v0;
            let a6229 = v1;
            nat_mul(a6228, a6229)?
        };
        let a6231 = 2147483648u64;
        nat_lt(a6230, a6231)
    } else {
        false
    } {
        let a6236 = {
            let a6234 = {
                let a6232 = v0;
                let a6233 = 1u64;
                nat_sub(a6232, a6233)
            };
            let a6235 = v2;
            nat_mul(a6234, a6235)?
        };
        let a6237 = 2147483648u64;
        nat_lt(a6236, a6237)
    } else {
        false
    } {
        let a6242 = {
            let a6240 = {
                let a6238 = v1;
                let a6239 = 1u64;
                nat_sub(a6238, a6239)
            };
            let a6241 = v2;
            nat_mul(a6240, a6241)?
        };
        let a6243 = 2147483648u64;
        Ok(nat_lt(a6242, a6243))
    } else {
        Ok(false)
    }
}

pub fn f235(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6244 = v1;
    let a6245 = 13u64;
    if if nat_eq(a6244, a6245) {
        f48(match v0.clone() {
            Adt3::C0(h6246, _, _, _, _, _, _, _) => {
                h6246
            }
        }, match v0.clone() {
            Adt3::C0(_, h6247, _, _, _, _, _, _) => {
                h6247
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6248, _, _, _, _, _) => {
                h6248
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6249, _, _, _, _) => {
                h6249
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6255 = match v0.clone() {
                Adt3::C0(_, h6250, _, _, _, _, _, _) => {
                    h6250
                }
            };
            let a6256 = {
                let a6253 = match v0.clone() {
                    Adt3::C0(_, _, h6251, _, _, _, _, _) => {
                        h6251
                    }
                };
                let a6254 = match v0.clone() {
                    Adt3::C0(_, _, _, h6252, _, _, _, _) => {
                        h6252
                    }
                };
                nat_mul(a6253, a6254)?
            };
            nat_mul(a6255, a6256)?
        };
        let a6257 = v1;
        let a6258 = 13u64;
        if nat_eq(a6257, a6258) {
            let a6267 = 2u64;
            let a6268 = if f274(v0.clone(), v2) {
                let a6260 = v3;
                let a6261 = f247(match v0 {
                    Adt3::C0(h6259, _, _, _, _, _, _, _) => {
                        h6259
                    }
                }, false)?;
                nat_mul(a6260, a6261)?
            } else {
                let a6265 = v3;
                let a6266 = {
                    let a6263 = match v0 {
                        Adt3::C0(h6262, _, _, _, _, _, _, _) => {
                            h6262
                        }
                    };
                    let a6264 = 2u64;
                    nat_mul(a6263, a6264)?
                };
                nat_mul(a6265, a6266)?
            };
            nat_add(a6267, a6268)
        } else {
            let a6272 = v3;
            let a6273 = f247(match v0 {
                Adt3::C0(h6269, _, _, _, _, _, _, _) => {
                    h6269
                }
            }, {
                let a6270 = v1;
                let a6271 = 16u64;
                nat_eq(a6270, a6271)
            })?;
            nat_mul(a6272, a6273)
        }
    }
}

pub fn f236(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6274 = v0;
    let a6275 = 30u64;
    if if nat_eq(a6274, a6275) {
        let a6278 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6276, _, _, _, _) => {
                h6276
            }
        };
        let a6279 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6277, _, _, _, _, _) => {
                h6277
            }
        };
        nat_lt(a6278, a6279)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6280 = v3;
        match m6280 {
            None => {
                f275(v0, v1, match v2.clone() {
                    Adt2::C0(h6281, _, _, _, _, _, _, _, _, _, _) => {
                        h6281
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6282, _, _, _, _, _, _, _, _, _, _) => {
                        h6282
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6283, _, _, _, _, _, _, _, _, _) => {
                        h6283
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6284, _, _, _, _, _, _, _, _) => {
                        h6284
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6285, _, _, _, _, _, _, _) => {
                        h6285
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6286, _, _, _, _, _, _) => {
                        h6286
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6287, _, _, _, _, _, _, _, _, _) => {
                        h6287
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6288, _, _, _, _, _, _, _, _) => {
                        h6288
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6289, _, _, _, _, _, _, _) => {
                        h6289
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6290, _, _, _, _, _, _) => {
                        h6290
                    }
                })
            }
            Some(v5) => {
                f275(v0, v1, match v2.clone() {
                    Adt2::C0(h6291, _, _, _, _, _, _, _, _, _, _) => {
                        h6291
                    }
                }, match v5.clone() {
                    Adt2::C0(h6292, _, _, _, _, _, _, _, _, _, _) => {
                        h6292
                    }
                }, match v4.clone() {
                    Adt2::C0(h6293, _, _, _, _, _, _, _, _, _, _) => {
                        h6293
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6294, _, _, _, _, _, _, _, _, _) => {
                        h6294
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6295, _, _, _, _, _, _, _, _) => {
                        h6295
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6296, _, _, _, _, _, _, _) => {
                        h6296
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6297, _, _, _, _, _, _) => {
                        h6297
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6298, _, _, _, _, _, _, _, _, _) => {
                        h6298
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6299, _, _, _, _, _, _, _, _) => {
                        h6299
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6300, _, _, _, _, _, _, _) => {
                        h6300
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6301, _, _, _, _, _, _) => {
                        h6301
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6302, _, _, _, _, _, _, _, _, _) => {
                        h6302
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6303, _, _, _, _, _, _, _, _) => {
                        h6303
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6304, _, _, _, _, _, _, _) => {
                        h6304
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6305, _, _, _, _, _, _) => {
                        h6305
                    }
                })
            }
        }
    }
}

pub fn f237(v0: u64, v1: u64) -> u64 {
    let a6306 = v0;
    let a6307 = 23u64;
    if nat_eq(a6306, a6307) {
        let a6308 = v1;
        let a6309 = 0u64;
        if nat_eq(a6308, a6309) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6314 = {
        let a6312 = {
            let a6310 = v0;
            let a6311 = v1;
            nat_sub(a6310, a6311)
        };
        let a6313 = 1u64;
        nat_sub(a6312, a6313)
    };
    let a6315 = v2;
    nat_add(a6314, a6315)
}

pub fn f239(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f142({
        let a6316 = v0;
        let a6317 = v4;
        let a6318 = 0u64;
        nat_rem(a6316, a6317, a6318)
    }, {
        let a6319 = v1;
        let a6320 = v5;
        let a6321 = 0u64;
        nat_rem(a6319, a6320, a6321)
    }, {
        let a6322 = v2;
        let a6323 = v6;
        let a6324 = 0u64;
        nat_rem(a6322, a6323, a6324)
    }, {
        let a6325 = v3;
        let a6326 = v7;
        let a6327 = 0u64;
        nat_rem(a6325, a6326, a6327)
    }, v8, v9, v10, v11)
}

pub fn f240(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6328 = v1;
    let a6329 = 2147483648u64;
    if nat_lt(a6328, a6329) {
        let a6340 = {
            let a6338 = {
                let a6333 = {
                    let a6330 = v0;
                    let a6331 = v2;
                    let a6332 = 0u64;
                    nat_rem(a6330, a6331, a6332)
                };
                let a6334 = v2;
                nat_add(a6333, a6334)?
            };
            let a6339 = {
                let a6335 = v1;
                let a6336 = v2;
                let a6337 = 0u64;
                nat_rem(a6335, a6336, a6337)
            };
            nat_sub(a6338, a6339)
        };
        let a6341 = v2;
        let a6342 = 0u64;
        Ok(nat_rem(a6340, a6341, a6342))
    } else {
        let a6345 = {
            let a6343 = v0;
            let a6344 = f143(v1);
            nat_add(a6343, a6344)?
        };
        let a6346 = v2;
        let a6347 = 0u64;
        Ok(nat_rem(a6345, a6346, a6347))
    }
}

pub fn f241(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6350 = f144(v1);
    let a6351 = {
        let a6348 = v0;
        let a6349 = f143(v1);
        nat_add(a6348, a6349)?
    };
    if nat_le(a6350, a6351) {
        let a6352 = f242(v0, v1)?;
        let a6353 = v2;
        Ok(nat_lt(a6352, a6353))
    } else {
        Ok(false)
    }
}

pub fn f242(v0: u64, v1: u64) -> R<u64> {
    let a6356 = {
        let a6354 = v0;
        let a6355 = f143(v1);
        nat_add(a6354, a6355)?
    };
    let a6357 = f144(v1);
    Ok(nat_sub(a6356, a6357))
}

pub fn f243(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6358 = v0;
    let a6359 = v1;
    if nat_lt(a6358, a6359) {
        let a6360 = v1;
        let a6361 = v0;
        Ok(nat_sub(a6360, a6361))
    } else {
        let a6364 = v0;
        let a6365 = {
            let a6362 = v1;
            let a6363 = v2;
            nat_add(a6362, a6363)?
        };
        if nat_lt(a6364, a6365) {
            let a6366 = v0;
            let a6367 = v1;
            Ok(nat_sub(a6366, a6367))
        } else {
            let a6374 = {
                let a6370 = 2u64;
                let a6371 = {
                    let a6368 = v2;
                    let a6369 = 1u64;
                    nat_sub(a6368, a6369)
                };
                nat_mul(a6370, a6371)?
            };
            let a6375 = {
                let a6372 = v0;
                let a6373 = v1;
                nat_sub(a6372, a6373)
            };
            Ok(nat_sub(a6374, a6375))
        }
    }
}

pub fn f244(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6378 = {
        let a6376 = v2;
        let a6377 = f276(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a6376, a6377)?
    };
    let a6379 = f276(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6378, a6379))
}

pub fn f245(v0: u64, v1: u64, v2: u64) -> bool {
    let a6380 = v0;
    let a6381 = 0u64;
    if nat_eq(a6380, a6381) {
        let a6382 = v2;
        let a6383 = v1;
        nat_le(a6382, a6383)
    } else {
        let a6384 = v0;
        let a6385 = 1u64;
        if nat_eq(a6384, a6385) {
            let a6386 = v2;
            let a6387 = v1;
            nat_lt(a6386, a6387)
        } else {
            let a6388 = v0;
            let a6389 = 2u64;
            if nat_eq(a6388, a6389) {
                let a6390 = v1;
                let a6391 = v2;
                nat_le(a6390, a6391)
            } else {
                let a6392 = v0;
                let a6393 = 3u64;
                if nat_eq(a6392, a6393) {
                    let a6394 = v1;
                    let a6395 = v2;
                    nat_lt(a6394, a6395)
                } else {
                    let a6396 = v1;
                    let a6397 = v2;
                    nat_eq(a6396, a6397)
                }
            }
        }
    }
}

pub fn f246(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6398 = v0;
    let a6399 = 0u64;
    if nat_eq(a6398, a6399) {
        Ok(f259(0u64))
    } else {
        let a6402 = {
            let a6400 = v0;
            let a6401 = 1u64;
            nat_add(a6400, a6401)?
        };
        let a6403 = f235(v1.clone(), 13u64, v2)?;
        if nat_eq(a6402, a6403) {
            Ok(Adt9::C4({
                let a6404 = v2;
                let a6405 = 0u64;
                if nat_eq(a6404, a6405) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6406 = v0;
                let a6407 = 1u64;
                nat_sub(a6406, a6407)
            };
            if f274(v1.clone(), v2) {
                let v4: u64 = f247(match v1.clone() {
                    Adt3::C0(h6408, _, _, _, _, _, _, _) => {
                        h6408
                    }
                }, false)?;
                let v5: u64 = {
                    let a6409 = v3;
                    let a6410 = v4;
                    let a6411 = 0u64;
                    nat_quot(a6409, a6410, a6411)
                };
                let v6: u64 = {
                    let a6412 = v3;
                    let a6413 = v4;
                    let a6414 = 0u64;
                    nat_rem(a6412, a6413, a6414)
                };
                let a6417 = {
                    let a6415 = v6;
                    let a6416 = 1u64;
                    nat_add(a6415, a6416)?
                };
                let a6418 = v4;
                if nat_eq(a6417, a6418) {
                    Ok(Adt9::C4({
                        let a6419 = v2;
                        let a6420 = 0u64;
                        if nat_eq(a6419, a6420) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f248(v6, v5, v1, v2)
                }
            } else {
                f277({
                    let a6421 = v3;
                    let a6422 = 2u64;
                    let a6423 = 0u64;
                    nat_quot(a6421, a6422, a6423)
                }, {
                    let a6424 = v3;
                    let a6425 = 2u64;
                    let a6426 = 0u64;
                    nat_rem(a6424, a6425, a6426)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f247(v0: u64, v1: bool) -> R<u64> {
    let a6429 = {
        let a6427 = v0;
        let a6428 = 2u64;
        nat_mul(a6427, a6428)?
    };
    let a6430 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6429, a6430)
}

pub fn f248(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6431 = v0;
    let a6432 = 0u64;
    if nat_eq(a6431, a6432) {
        Ok(f259(1u64))
    } else {
        f277({
            let a6441 = {
                let a6434 = v1;
                let a6435 = match v2.clone() {
                    Adt3::C0(h6433, _, _, _, _, _, _, _) => {
                        h6433
                    }
                };
                nat_mul(a6434, a6435)?
            };
            let a6442 = {
                let a6438 = {
                    let a6436 = v0;
                    let a6437 = 1u64;
                    nat_sub(a6436, a6437)
                };
                let a6439 = 2u64;
                let a6440 = 0u64;
                nat_quot(a6438, a6439, a6440)
            };
            nat_add(a6441, a6442)?
        }, {
            let a6445 = {
                let a6443 = v0;
                let a6444 = 1u64;
                nat_sub(a6443, a6444)
            };
            let a6446 = 2u64;
            let a6447 = 0u64;
            nat_rem(a6445, a6446, a6447)
        }, v2, v3, 1u64)
    }
}

pub fn f249(v0: u64, v1: u64) -> R<u64> {
    let a6448 = v0;
    let a6449 = f95(v1)?;
    let a6450 = 0u64;
    Ok(nat_rem(a6448, a6449, a6450))
}

pub fn f250(v0: u64, v1: u64) -> R<u64> {
    let a6451 = 4u64;
    let a6452 = f278(v0, v1)?;
    nat_mul(a6451, a6452)
}

pub fn f251(v0: u64, v1: u64) -> R<u64> {
    let a6453 = 4u64;
    let a6454 = f279(v0, v1)?;
    nat_mul(a6453, a6454)
}

pub fn f252(v0: u64, v1: u64) -> R<bool> {
    let a6460 = {
        let a6457 = {
            let a6455 = f249(v0, v1)?;
            let a6456 = 2u64;
            nat_sub(a6455, a6456)
        };
        let a6458 = 2u64;
        let a6459 = 0u64;
        nat_rem(a6457, a6458, a6459)
    };
    let a6461 = 0u64;
    Ok(nat_eq(a6460, a6461))
}

pub fn f253(v0: u64) -> u64 {
    let m6462 = f216(v0);
    match m6462 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6463, _) => {
                    h6463
                }
            }
        }
    }
}

pub fn f254(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6464 = f216(v0);
    match m6464 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f223(v0) {
                let a6465 = v1;
                let a6466 = v7;
                nat_lt(a6465, a6466)
            } else {
                false
            } {
                f184(v8, v2)
            } else {
                false
            } {
                let a6467 = v3;
                let a6468 = f280(v4, v5, v6)?;
                nat_lt(a6467, a6468)
            } else {
                false
            } {
                let a6473 = {
                    let a6470 = v7;
                    let a6471 = match v13.clone() {
                        Adt19::C0(_, h6469, _) => {
                            h6469
                        }
                    };
                    let a6472 = 0u64;
                    nat_rem(a6470, a6471, a6472)
                };
                let a6474 = 0u64;
                nat_eq(a6473, a6474)
            } else {
                false
            } {
                Ok(Some({
                    let a6503 = {
                        let a6493 = {
                            let a6483 = {
                                let a6479 = {
                                    let a6476 = v1;
                                    let a6477 = match v13 {
                                        Adt19::C0(_, h6475, _) => {
                                            h6475
                                        }
                                    };
                                    let a6478 = 0u64;
                                    nat_quot(a6476, a6477, a6478)
                                };
                                let a6480 = v9;
                                nat_mul(a6479, a6480)?
                            };
                            let a6484 = {
                                let a6481 = v2;
                                let a6482 = v10;
                                nat_mul(a6481, a6482)?
                            };
                            nat_add(a6483, a6484)?
                        };
                        let a6494 = {
                            let a6491 = {
                                let a6488 = {
                                    let a6485 = v3;
                                    let a6486 = v4;
                                    let a6487 = 0u64;
                                    nat_quot(a6485, a6486, a6487)
                                };
                                let a6489 = v5;
                                let a6490 = 0u64;
                                nat_rem(a6488, a6489, a6490)
                            };
                            let a6492 = v11;
                            nat_mul(a6491, a6492)?
                        };
                        nat_add(a6493, a6494)?
                    };
                    let a6504 = {
                        let a6501 = {
                            let a6498 = {
                                let a6495 = v3;
                                let a6496 = v4;
                                let a6497 = 0u64;
                                nat_quot(a6495, a6496, a6497)
                            };
                            let a6499 = v5;
                            let a6500 = 0u64;
                            nat_quot(a6498, a6499, a6500)
                        };
                        let a6502 = v12;
                        nat_mul(a6501, a6502)?
                    };
                    nat_add(a6503, a6504)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f255(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f281(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f256(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f282(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f257(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6505 = v0;
    let a6506 = 64u64;
    if nat_eq(a6505, a6506) {
        f283(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6507 = v0;
        let a6508 = 65u64;
        if nat_eq(a6507, a6508) {
            f284(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f258(v0: u64) -> u64 {
    let a6509 = v0;
    let a6510 = 30u64;
    if nat_eq(a6509, a6510) {
        4u64
    } else {
        3u64
    }
}

pub fn f259(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f260(v0: u64) -> bool {
    let a6511 = 0u64;
    let a6512 = v0;
    if nat_lt(a6511, a6512) {
        let a6513 = v0;
        let a6514 = 2139095040u64;
        nat_le(a6513, a6514)
    } else {
        false
    }
}

pub fn f261(v0: u64) -> u64 {
    let a6515 = v0;
    let a6516 = 40u64;
    if nat_lt(a6515, a6516) {
        let a6517 = v0;
        let a6518 = 20u64;
        if nat_lt(a6517, a6518) {
            let a6519 = v0;
            let a6520 = 10u64;
            if nat_lt(a6519, a6520) {
                let a6521 = v0;
                let a6522 = 5u64;
                if nat_lt(a6521, a6522) {
                    let a6523 = v0;
                    let a6524 = 2u64;
                    if nat_lt(a6523, a6524) {
                        let a6525 = v0;
                        let a6526 = 1u64;
                        if nat_lt(a6525, a6526) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6527 = v0;
                        let a6528 = 3u64;
                        if nat_lt(a6527, a6528) {
                            4u64
                        } else {
                            let a6529 = v0;
                            let a6530 = 4u64;
                            if nat_lt(a6529, a6530) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6531 = v0;
                    let a6532 = 7u64;
                    if nat_lt(a6531, a6532) {
                        let a6533 = v0;
                        let a6534 = 6u64;
                        if nat_lt(a6533, a6534) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6535 = v0;
                        let a6536 = 8u64;
                        if nat_lt(a6535, a6536) {
                            128u64
                        } else {
                            let a6537 = v0;
                            let a6538 = 9u64;
                            if nat_lt(a6537, a6538) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6539 = v0;
                let a6540 = 15u64;
                if nat_lt(a6539, a6540) {
                    let a6541 = v0;
                    let a6542 = 12u64;
                    if nat_lt(a6541, a6542) {
                        let a6543 = v0;
                        let a6544 = 11u64;
                        if nat_lt(a6543, a6544) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6545 = v0;
                        let a6546 = 13u64;
                        if nat_lt(a6545, a6546) {
                            4096u64
                        } else {
                            let a6547 = v0;
                            let a6548 = 14u64;
                            if nat_lt(a6547, a6548) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6549 = v0;
                    let a6550 = 17u64;
                    if nat_lt(a6549, a6550) {
                        let a6551 = v0;
                        let a6552 = 16u64;
                        if nat_lt(a6551, a6552) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6553 = v0;
                        let a6554 = 18u64;
                        if nat_lt(a6553, a6554) {
                            131072u64
                        } else {
                            let a6555 = v0;
                            let a6556 = 19u64;
                            if nat_lt(a6555, a6556) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6557 = v0;
            let a6558 = 30u64;
            if nat_lt(a6557, a6558) {
                let a6559 = v0;
                let a6560 = 25u64;
                if nat_lt(a6559, a6560) {
                    let a6561 = v0;
                    let a6562 = 22u64;
                    if nat_lt(a6561, a6562) {
                        let a6563 = v0;
                        let a6564 = 21u64;
                        if nat_lt(a6563, a6564) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6565 = v0;
                        let a6566 = 23u64;
                        if nat_lt(a6565, a6566) {
                            4194304u64
                        } else {
                            let a6567 = v0;
                            let a6568 = 24u64;
                            if nat_lt(a6567, a6568) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6569 = v0;
                    let a6570 = 27u64;
                    if nat_lt(a6569, a6570) {
                        let a6571 = v0;
                        let a6572 = 26u64;
                        if nat_lt(a6571, a6572) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6573 = v0;
                        let a6574 = 28u64;
                        if nat_lt(a6573, a6574) {
                            134217728u64
                        } else {
                            let a6575 = v0;
                            let a6576 = 29u64;
                            if nat_lt(a6575, a6576) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6577 = v0;
                let a6578 = 35u64;
                if nat_lt(a6577, a6578) {
                    let a6579 = v0;
                    let a6580 = 32u64;
                    if nat_lt(a6579, a6580) {
                        let a6581 = v0;
                        let a6582 = 31u64;
                        if nat_lt(a6581, a6582) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6583 = v0;
                        let a6584 = 33u64;
                        if nat_lt(a6583, a6584) {
                            4294967296u64
                        } else {
                            let a6585 = v0;
                            let a6586 = 34u64;
                            if nat_lt(a6585, a6586) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6587 = v0;
                    let a6588 = 37u64;
                    if nat_lt(a6587, a6588) {
                        let a6589 = v0;
                        let a6590 = 36u64;
                        if nat_lt(a6589, a6590) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6591 = v0;
                        let a6592 = 38u64;
                        if nat_lt(a6591, a6592) {
                            137438953472u64
                        } else {
                            let a6593 = v0;
                            let a6594 = 39u64;
                            if nat_lt(a6593, a6594) {
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

pub fn f262(v0: u64, v1: u64) -> bool {
    let a6595 = v0;
    let a6596 = 1u64;
    if nat_le(a6595, a6596) {
        true
    } else {
        let a6600 = {
            let a6597 = v1;
            let a6598 = 4u64;
            let a6599 = 0u64;
            nat_rem(a6597, a6598, a6599)
        };
        let a6601 = 0u64;
        nat_eq(a6600, a6601)
    }
}

pub fn f263(v0: u64, v1: u64) -> bool {
    let a6602 = v0;
    let a6603 = 1u64;
    if nat_le(a6602, a6603) {
        true
    } else {
        let a6604 = 4u64;
        let a6605 = v1;
        nat_le(a6604, a6605)
    }
}

pub fn f264(v0: u64, v1: u64) -> R<u64> {
    let a6606 = v0;
    let a6607 = 0u64;
    if nat_eq(a6606, a6607) {
        Ok(0u64)
    } else {
        let a6612 = 4u64;
        let a6613 = {
            let a6610 = {
                let a6608 = v0;
                let a6609 = 1u64;
                nat_sub(a6608, a6609)
            };
            let a6611 = v1;
            nat_mul(a6610, a6611)?
        };
        nat_add(a6612, a6613)
    }
}

pub fn f265(v0: bool, v1: u64) -> R<u64> {
    let a6618 = {
        let a6616 = {
            let a6614 = f285(v0, v1)?;
            let a6615 = f286(v0, v1)?;
            nat_add(a6614, a6615)?
        };
        let a6617 = f287(v0);
        nat_add(a6616, a6617)?
    };
    let a6619 = v1;
    nat_add(a6618, a6619)
}

pub fn f266(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f288(v0, v1)?;
    let v3: u64 = {
        let a6624 = {
            let a6622 = {
                let a6620 = v1;
                let a6621 = 1u64;
                nat_sub(a6620, a6621)
            };
            let a6623 = v2;
            nat_mul(a6622, a6623)?
        };
        let a6625 = v0;
        nat_sub(a6624, a6625)
    };
    let a6626 = v2;
    let a6627 = v3;
    if nat_lt(a6626, a6627) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f267(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6628 = v4;
    let a6629 = 0u64;
    if nat_eq(a6628, a6629) {
        Ok(0u64)
    } else {
        let a6636 = 2u64;
        let a6637 = {
            let a6634 = {
                let a6630 = v0;
                let a6631 = v1;
                nat_mul(a6630, a6631)?
            };
            let a6635 = {
                let a6632 = v2;
                let a6633 = v3;
                nat_mul(a6632, a6633)?
            };
            nat_mul(a6634, a6635)?
        };
        nat_mul(a6636, a6637)
    }
}

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6642 = v3;
    let a6643 = {
        let a6640 = {
            let a6638 = f289(v2, v4)?;
            let a6639 = f290(v0, v1, f288(v2, v4)?)?;
            nat_mul(a6638, a6639)?
        };
        let a6641 = f290(v0, v1, f291(v2, v4)?)?;
        nat_add(a6640, a6641)?
    };
    nat_mul(a6642, a6643)
}

pub fn f269(v0: u64) -> R<u64> {
    let a6646 = 13u64;
    let a6647 = {
        let a6644 = 9u64;
        let a6645 = v0;
        nat_mul(a6644, a6645)?
    };
    nat_add(a6646, a6647)
}

pub fn f270(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f292(v0, v1, v6, v7, v8) {
        let a6648 = v5;
        let a6649 = 26u64;
        nat_eq(a6648, a6649)
    } else {
        false
    } {
        let a6650 = v3;
        let a6651 = v7;
        nat_eq(a6650, a6651)
    } else {
        false
    } {
        let a6652 = v4;
        let a6653 = v8;
        nat_eq(a6652, a6653)
    } else {
        false
    } {
        let a6654 = v9;
        let a6655 = 1u64;
        nat_eq(a6654, a6655)
    } else {
        false
    } {
        let a6656 = v10;
        let a6657 = f293(v0);
        nat_eq(a6656, a6657)
    } else {
        false
    } {
        let a6658 = v11;
        let a6659 = v1;
        nat_eq(a6658, a6659)
    } else {
        false
    } {
        let a6660 = v12;
        let a6661 = v6;
        nat_eq(a6660, a6661)
    } else {
        false
    } {
        let a6662 = v13;
        let a6663 = v7;
        nat_eq(a6662, a6663)
    } else {
        false
    } {
        let a6664 = v14;
        let a6665 = v8;
        nat_eq(a6664, a6665)
    } else {
        false
    }
}

pub fn f271(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6666 = v0;
    let a6667 = 0u64;
    if if if nat_eq(a6666, a6667) {
        true
    } else {
        let a6668 = v1;
        let a6669 = 0u64;
        nat_eq(a6668, a6669)
    } {
        true
    } else {
        let a6670 = v2;
        let a6671 = 0u64;
        nat_eq(a6670, a6671)
    } {
        true
    } else {
        let a6672 = v3;
        let a6673 = 0u64;
        nat_eq(a6672, a6673)
    }
}

pub fn f272(v0: Adt3, v1: bool) -> R<u64> {
    f276(v0.clone(), {
        let a6675 = match v0.clone() {
            Adt3::C0(h6674, _, _, _, _, _, _, _) => {
                h6674
            }
        };
        let a6676 = 1u64;
        nat_sub(a6675, a6676)
    }, {
        let a6678 = match v0.clone() {
            Adt3::C0(_, h6677, _, _, _, _, _, _) => {
                h6677
            }
        };
        let a6679 = 1u64;
        nat_sub(a6678, a6679)
    }, {
        let a6681 = match v0.clone() {
            Adt3::C0(_, _, h6680, _, _, _, _, _) => {
                h6680
            }
        };
        let a6682 = 1u64;
        nat_sub(a6681, a6682)
    }, {
        let a6684 = match v0 {
            Adt3::C0(_, _, _, h6683, _, _, _, _) => {
                h6683
            }
        };
        let a6685 = 1u64;
        nat_sub(a6684, a6685)
    }, v1)
}

pub fn f273(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6686 = v0;
    let a6687 = 0u64;
    if nat_eq(a6686, a6687) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6688 = v0;
            let a6689 = 1u64;
            nat_sub(a6688, a6689)
        };
        let v5: u64 = {
            let a6690 = v4;
            let a6691 = v1;
            let a6692 = 0u64;
            nat_quot(a6690, a6691, a6692)
        };
        let v6: u64 = {
            let a6700 = {
                let a6693 = v5;
                let a6694 = v2;
                nat_mul(a6693, a6694)?
            };
            let a6701 = {
                let a6698 = {
                    let a6695 = v4;
                    let a6696 = v1;
                    let a6697 = 0u64;
                    nat_rem(a6695, a6696, a6697)
                };
                let a6699 = v3;
                nat_mul(a6698, a6699)?
            };
            nat_add(a6700, a6701)?
        };
        let a6702 = v5;
        let a6703 = 0u64;
        if nat_eq(a6702, a6703) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6712 = {
                    let a6706 = {
                        let a6704 = v5;
                        let a6705 = 1u64;
                        nat_sub(a6704, a6705)
                    };
                    let a6707 = v2;
                    nat_mul(a6706, a6707)?
                };
                let a6713 = {
                    let a6710 = {
                        let a6708 = v1;
                        let a6709 = 1u64;
                        nat_sub(a6708, a6709)
                    };
                    let a6711 = v3;
                    nat_mul(a6710, a6711)?
                };
                nat_add(a6712, a6713)?
            };
            let a6714 = v6;
            let a6715 = v7;
            if nat_lt(a6714, a6715) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f274(v0: Adt3, v1: u64) -> bool {
    let a6716 = v1;
    let a6717 = 0u64;
    if !nat_eq(a6716, a6717) {
        true
    } else {
        let a6719 = match v0 {
            Adt3::C0(_, _, _, _, h6718, _, _, _) => {
                h6718
            }
        };
        let a6720 = 4u64;
        nat_eq(a6719, a6720)
    }
}

pub fn f275(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6721 = f294(v0, v1);
    match m6721 {
        None => {
            if v5 {
                let m6722 = f295(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6722 {
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
                let a6723 = 4u64;
                let a6724 = v28;
                nat_le(a6723, a6724)
            };
            let a6725 = v2;
            let a6726 = 0u64;
            if if if if if nat_eq(a6725, a6726) {
                let a6727 = v4;
                let a6728 = 0u64;
                nat_eq(a6727, a6728)
            } else {
                false
            } {
                let a6729 = v6;
                let a6730 = v14;
                if nat_eq(a6729, a6730) {
                    let a6731 = v7;
                    let a6732 = v15;
                    nat_eq(a6731, a6732)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6733 = v8;
                let a6734 = v16;
                if nat_eq(a6733, a6734) {
                    let a6735 = v9;
                    let a6736 = v17;
                    nat_eq(a6735, a6736)
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
                        let a6737 = v3;
                        let a6738 = 0u64;
                        nat_eq(a6737, a6738)
                    } else {
                        false
                    } {
                        f226(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6743 = {
                        let a6739 = v6;
                        let a6740 = v7;
                        nat_mul(a6739, a6740)?
                    };
                    let a6744 = {
                        let a6741 = v8;
                        let a6742 = v9;
                        nat_mul(a6741, a6742)?
                    };
                    nat_mul(a6743, a6744)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f276(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6765 = {
        let a6759 = {
            let a6753 = {
                let a6747 = v1;
                let a6748 = if v5 {
                    f143(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6745, _, _, _) => {
                            h6745
                        }
                    })
                } else {
                    f144(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6746, _, _, _) => {
                            h6746
                        }
                    })
                };
                nat_mul(a6747, a6748)?
            };
            let a6754 = {
                let a6751 = v2;
                let a6752 = if v5 {
                    f143(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6749, _, _) => {
                            h6749
                        }
                    })
                } else {
                    f144(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6750, _, _) => {
                            h6750
                        }
                    })
                };
                nat_mul(a6751, a6752)?
            };
            nat_add(a6753, a6754)?
        };
        let a6760 = {
            let a6757 = v3;
            let a6758 = if v5 {
                f143(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6755, _) => {
                        h6755
                    }
                })
            } else {
                f144(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6756, _) => {
                        h6756
                    }
                })
            };
            nat_mul(a6757, a6758)?
        };
        nat_add(a6759, a6760)?
    };
    let a6766 = {
        let a6763 = v4;
        let a6764 = if v5 {
            f143(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6761) => {
                    h6761
                }
            })
        } else {
            f144(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6762) => {
                    h6762
                }
            })
        };
        nat_mul(a6763, a6764)?
    };
    nat_add(a6765, a6766)
}

pub fn f277(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6767 = v1;
    let a6768 = 0u64;
    if nat_eq(a6767, a6768) {
        Ok(Adt9::C4({
            let a6769 = v3;
            let a6770 = 0u64;
            if nat_eq(a6769, a6770) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f92(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6771 = v3;
            let a6772 = 0u64;
            if nat_eq(a6771, a6772) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f278(v0: u64, v1: u64) -> R<u64> {
    let a6773 = v0;
    let a6774 = f95(v1)?;
    let a6775 = 0u64;
    Ok(nat_quot(a6773, a6774, a6775))
}

pub fn f279(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f278(v0, v1)?;
    let v3: u64 = {
        let a6778 = {
            let a6776 = f249(v0, v1)?;
            let a6777 = 2u64;
            nat_sub(a6776, a6777)
        };
        let a6779 = 2u64;
        let a6780 = 0u64;
        nat_quot(a6778, a6779, a6780)
    };
    let v4: u64 = {
        let a6781 = v2;
        let a6782 = v1;
        nat_mul(a6781, a6782)?
    };
    if f252(v0, v1)? {
        let a6783 = v4;
        let a6784 = v3;
        nat_add(a6783, a6784)
    } else {
        let a6796 = {
            let a6792 = v4;
            let a6793 = {
                let a6790 = {
                    let a6785 = v1;
                    let a6786 = 1u64;
                    nat_sub(a6785, a6786)
                };
                let a6791 = {
                    let a6787 = v2;
                    let a6788 = v1;
                    let a6789 = 0u64;
                    nat_rem(a6787, a6788, a6789)
                };
                nat_mul(a6790, a6791)?
            };
            nat_sub(a6792, a6793)
        };
        let a6797 = {
            let a6794 = v3;
            let a6795 = v1;
            nat_mul(a6794, a6795)?
        };
        nat_add(a6796, a6797)
    }
}

pub fn f280(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6798 = v0;
    let a6799 = 0u64;
    if if if nat_eq(a6798, a6799) {
        true
    } else {
        let a6800 = v1;
        let a6801 = 0u64;
        nat_eq(a6800, a6801)
    } {
        true
    } else {
        let a6802 = v2;
        let a6803 = 0u64;
        nat_eq(a6802, a6803)
    } {
        Ok(0u64)
    } else {
        let a6806 = {
            let a6804 = v0;
            let a6805 = v1;
            nat_mul(a6804, a6805)?
        };
        let a6807 = v2;
        nat_mul(a6806, a6807)
    }
}

pub fn f281(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f104(match v1.clone() {
        Adt20::C0(h6808, _, _, _, _, _, _, _) => {
            h6808
        }
    })?;
    let v6: u64 = {
        let a6814 = match v1.clone() {
            Adt20::C0(_, h6809, _, _, _, _, _, _) => {
                h6809
            }
        };
        let a6815 = {
            let a6812 = match v1.clone() {
                Adt20::C0(_, _, h6810, _, _, _, _, _) => {
                    h6810
                }
            };
            let a6813 = match v1.clone() {
                Adt20::C0(_, _, _, h6811, _, _, _, _) => {
                    h6811
                }
            };
            nat_mul(a6812, a6813)?
        };
        nat_mul(a6814, a6815)?
    };
    let a6817 = v0;
    let a6818 = f53(match v1.clone() {
        Adt20::C0(h6816, _, _, _, _, _, _, _) => {
            h6816
        }
    }, v6)?;
    if nat_lt(a6817, a6818) {
        f296({
            let a6819 = v0;
            let a6820 = v5;
            let a6821 = 0u64;
            nat_rem(a6819, a6820, a6821)
        }, {
            let a6822 = v0;
            let a6823 = v5;
            let a6824 = 0u64;
            nat_quot(a6822, a6823, a6824)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f282(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6825 = v6;
        let a6826 = 6u64;
        nat_sub(a6825, a6826)
    };
    let v9: Adt20 = f297(v3);
    let a6828 = v0;
    let a6829 = f213(match v1.clone() {
        Adt20::C0(h6827, _, _, _, _, _, _, _) => {
            h6827
        }
    }, v5, v8)?;
    if nat_lt(a6828, a6829) {
        let v10: u64 = f101(match v1.clone() {
            Adt20::C0(h6830, _, _, _, _, _, _, _) => {
                h6830
            }
        }, v5, v8)?;
        let a6831 = v0;
        let a6832 = v10;
        if nat_lt(a6831, a6832) {
            f298(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6833 = v0;
                let a6834 = v10;
                nat_sub(a6833, a6834)
            };
            let v12: u64 = f269(match v1.clone() {
                Adt20::C0(h6835, _, _, _, _, _, _, _) => {
                    h6835
                }
            })?;
            f299({
                let a6836 = v11;
                let a6837 = v12;
                let a6838 = 0u64;
                nat_rem(a6836, a6837, a6838)
            }, {
                let a6839 = v11;
                let a6840 = v12;
                let a6841 = 0u64;
                nat_quot(a6839, a6840, a6841)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f283(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6842 = v5;
        let a6843 = 6u64;
        nat_sub(a6842, a6843)
    };
    let v8: bool = f207(match v1.clone() {
        Adt20::C0(_, _, _, _, h6844, _, _, _) => {
            h6844
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6845, _, _, _) => {
            h6845
        }
    });
    let a6847 = v0;
    let a6848 = f206(v8, match v1.clone() {
        Adt20::C0(h6846, _, _, _, _, _, _, _) => {
            h6846
        }
    }, v4, v7)?;
    if nat_lt(a6847, a6848) {
        let v9: u64 = f101(match v1.clone() {
            Adt20::C0(h6849, _, _, _, _, _, _, _) => {
                h6849
            }
        }, v4, v7)?;
        let a6850 = v0;
        let a6851 = v9;
        if nat_lt(a6850, a6851) {
            f298(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6852 = v0;
                let a6853 = v9;
                nat_sub(a6852, a6853)
            };
            let v11: u64 = f265(v8, match v1.clone() {
                Adt20::C0(h6854, _, _, _, _, _, _, _) => {
                    h6854
                }
            })?;
            f300({
                let a6855 = v10;
                let a6856 = v11;
                let a6857 = 0u64;
                nat_rem(a6855, a6856, a6857)
            }, {
                let a6858 = v10;
                let a6859 = v11;
                let a6860 = 0u64;
                nat_quot(a6858, a6859, a6860)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f284(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6861 = v5;
        let a6862 = 6u64;
        nat_sub(a6861, a6862)
    };
    let v8: Adt20 = f297(v2);
    let v9: u64 = f267(match v1.clone() {
        Adt20::C0(h6863, _, _, _, _, _, _, _) => {
            h6863
        }
    }, match v1.clone() {
        Adt20::C0(_, h6864, _, _, _, _, _, _) => {
            h6864
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6865, _, _, _, _, _) => {
            h6865
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6866, _, _, _, _) => {
            h6866
        }
    }, v7)?;
    let a6871 = v0;
    let a6872 = f212(match v1.clone() {
        Adt20::C0(h6867, _, _, _, _, _, _, _) => {
            h6867
        }
    }, match v1.clone() {
        Adt20::C0(_, h6868, _, _, _, _, _, _) => {
            h6868
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6869, _, _, _, _, _) => {
            h6869
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6870, _, _, _, _) => {
            h6870
        }
    }, v4, v7)?;
    if nat_lt(a6871, a6872) {
        let a6873 = v0;
        let a6874 = v9;
        if nat_lt(a6873, a6874) {
            f298(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f301({
                let a6875 = v0;
                let a6876 = v9;
                nat_sub(a6875, a6876)
            }, match v1.clone() {
                Adt20::C0(h6877, _, _, _, _, _, _, _) => {
                    h6877
                }
            }, match v1.clone() {
                Adt20::C0(_, h6878, _, _, _, _, _, _) => {
                    h6878
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6879, _, _, _, _, _) => {
                    h6879
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6880, _, _, _, _) => {
                    h6880
                }
            }, v4)?;
            f302(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f285(v0: bool, v1: u64) -> R<u64> {
    let a6883 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6884 = {
        let a6881 = f303(v0);
        let a6882 = v1;
        nat_mul(a6881, a6882)?
    };
    nat_add(a6883, a6884)
}

pub fn f286(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6892 = {
            let a6885 = 13u64;
            let a6886 = f304(v1);
            nat_mul(a6885, a6886)?
        };
        let a6893 = {
            let a6890 = 4u64;
            let a6891 = {
                let a6887 = v1;
                let a6888 = 4u64;
                let a6889 = 0u64;
                nat_rem(a6887, a6888, a6889)
            };
            nat_mul(a6890, a6891)?
        };
        nat_add(a6892, a6893)
    } else {
        let a6894 = 3u64;
        let a6895 = v1;
        nat_mul(a6894, a6895)
    }
}

pub fn f287(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f288(v0: u64, v1: u64) -> R<u64> {
    let a6900 = {
        let a6898 = {
            let a6896 = v0;
            let a6897 = v1;
            nat_add(a6896, a6897)?
        };
        let a6899 = 1u64;
        nat_sub(a6898, a6899)
    };
    let a6901 = v1;
    let a6902 = 0u64;
    Ok(nat_quot(a6900, a6901, a6902))
}

pub fn f289(v0: u64, v1: u64) -> R<u64> {
    let a6903 = v0;
    let a6904 = f288(v0, v1)?;
    let a6905 = 0u64;
    Ok(nat_quot(a6903, a6904, a6905))
}

pub fn f290(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6906 = v2;
    let a6907 = 0u64;
    if nat_eq(a6906, a6907) {
        Ok(0u64)
    } else {
        let a6912 = 11u64;
        let a6913 = {
            let a6910 = {
                let a6908 = v1;
                let a6909 = v2;
                nat_mul(a6908, a6909)?
            };
            let a6911 = f305(v0)?;
            nat_mul(a6910, a6911)?
        };
        nat_add(a6912, a6913)
    }
}

pub fn f291(v0: u64, v1: u64) -> R<u64> {
    let a6914 = v0;
    let a6915 = f288(v0, v1)?;
    let a6916 = 0u64;
    Ok(nat_rem(a6914, a6915, a6916))
}

pub fn f292(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f271(v1, v2, v3, v4) {
        f306(v0)
    } else {
        f223(v0)
    }
}

pub fn f293(v0: u64) -> u64 {
    let a6917 = v0;
    let a6918 = 26u64;
    if nat_eq(a6917, a6918) {
        26u64
    } else {
        0u64
    }
}

pub fn f294(v0: u64, v1: u64) -> Option<u64> {
    let a6919 = v0;
    let a6920 = 2u64;
    if nat_eq(a6919, a6920) {
        Some(0u64)
    } else {
        let a6921 = v0;
        let a6922 = 5u64;
        if nat_eq(a6921, a6922) {
            Some(1u64)
        } else {
            let a6923 = v0;
            let a6924 = 6u64;
            if nat_eq(a6923, a6924) {
                Some(2u64)
            } else {
                let a6925 = v0;
                let a6926 = 7u64;
                if nat_eq(a6925, a6926) {
                    Some(3u64)
                } else {
                    let a6927 = v0;
                    let a6928 = 9u64;
                    if nat_eq(a6927, a6928) {
                        Some(4u64)
                    } else {
                        let a6929 = v0;
                        let a6930 = 8u64;
                        if nat_eq(a6929, a6930) {
                            Some(5u64)
                        } else {
                            let a6931 = v0;
                            let a6932 = 89u64;
                            if nat_eq(a6931, a6932) {
                                let a6933 = v1;
                                let a6934 = 2u64;
                                if nat_eq(a6933, a6934) {
                                    Some(6u64)
                                } else {
                                    let a6935 = v1;
                                    let a6936 = 0u64;
                                    if nat_eq(a6935, a6936) {
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

pub fn f295(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6937 = v0;
    let a6938 = 30u64;
    if nat_eq(a6937, a6938) {
        let a6939 = v2;
        let a6940 = 0u64;
        if if if nat_eq(a6939, a6940) {
            true
        } else {
            let a6941 = v2;
            let a6942 = 1u64;
            nat_eq(a6941, a6942)
        } {
            let a6943 = v1;
            let a6944 = 0u64;
            if nat_eq(a6943, a6944) {
                true
            } else {
                f307(v1, v4)
            }
        } else {
            false
        } {
            f308(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f309(v1) {
            f309(v2)
        } else {
            false
        } {
            let a6945 = v1;
            let a6946 = 0u64;
            if if nat_eq(a6945, a6946) {
                true
            } else {
                let a6947 = v2;
                let a6948 = 0u64;
                nat_eq(a6947, a6948)
            } {
                true
            } else {
                let a6949 = v1;
                let a6950 = v2;
                nat_eq(a6949, a6950)
            }
        } else {
            false
        } {
            true
        } else {
            let a6951 = v1;
            let a6952 = 43u64;
            if nat_eq(a6951, a6952) {
                f309(v2)
            } else {
                false
            }
        } {
            f308(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f296(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6953, _, _, _, _, _, _, _) => {
            h6953
        }
    };
    let v7: u64 = {
        let a6956 = 1u64;
        let a6957 = {
            let a6954 = 3u64;
            let a6955 = v6;
            nat_mul(a6954, a6955)?
        };
        nat_add(a6956, a6957)?
    };
    let a6958 = v0;
    let a6959 = 0u64;
    if nat_eq(a6958, a6959) {
        Ok(f310(0u64))
    } else {
        let a6960 = v0;
        let a6961 = v7;
        if nat_lt(a6960, a6961) {
            let v8: u64 = {
                let a6962 = v0;
                let a6963 = 1u64;
                nat_sub(a6962, a6963)
            };
            let v9: u64 = {
                let a6969 = {
                    let a6964 = v1;
                    let a6965 = v6;
                    nat_mul(a6964, a6965)?
                };
                let a6970 = {
                    let a6966 = v8;
                    let a6967 = 3u64;
                    let a6968 = 0u64;
                    nat_quot(a6966, a6967, a6968)
                };
                nat_add(a6969, a6970)?
            };
            let v10: u64 = f311(v9, v2)?;
            let v11: u64 = {
                let a6971 = v8;
                let a6972 = 3u64;
                let a6973 = 0u64;
                nat_rem(a6971, a6972, a6973)
            };
            let a6974 = v11;
            let a6975 = 0u64;
            if nat_eq(a6974, a6975) {
                Ok(f312(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6976 = v11;
                let a6977 = 1u64;
                if nat_eq(a6976, a6977) {
                    Ok(f312(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f312(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6978 = v0;
            let a6979 = v7;
            if nat_eq(a6978, a6979) {
                Ok(f312(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6982 = v0;
                let a6983 = {
                    let a6980 = v7;
                    let a6981 = 1u64;
                    nat_add(a6980, a6981)?
                };
                if nat_eq(a6982, a6983) {
                    Ok(f312(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6986 = v0;
                    let a6987 = {
                        let a6984 = v7;
                        let a6985 = 2u64;
                        nat_add(a6984, a6985)?
                    };
                    if nat_eq(a6986, a6987) {
                        Ok(f312(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6990 = v0;
                        let a6991 = {
                            let a6988 = v7;
                            let a6989 = 3u64;
                            nat_add(a6988, a6989)?
                        };
                        if nat_eq(a6990, a6991) {
                            let a6992 = v5;
                            let a6993 = 1u64;
                            if if nat_eq(a6992, a6993) {
                                true
                            } else {
                                let a6994 = v5;
                                let a6995 = 2u64;
                                nat_eq(a6994, a6995)
                            } {
                                Ok(f312(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f312(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7002 = {
                                    let a6996 = v1;
                                    let a6997 = v6;
                                    nat_mul(a6996, a6997)?
                                };
                                let a7003 = {
                                    let a7000 = v0;
                                    let a7001 = {
                                        let a6998 = v7;
                                        let a6999 = 4u64;
                                        nat_add(a6998, a6999)?
                                    };
                                    nat_sub(a7000, a7001)
                                };
                                nat_add(a7002, a7003)?
                            };
                            Ok(f312(10u64, 0u64, f311(v12, v2)?, 1u64, 2u64, 3u64, f311(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f297(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h7004, _, _, _, _, _, _, _) => {
            h7004
        }
    }, match v0.clone() {
        Adt20::C0(_, h7005, _, _, _, _, _, _) => {
            h7005
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h7006, _, _, _, _, _) => {
            h7006
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h7007, _, _, _, _) => {
            h7007
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h7008, _, _) => {
            h7008
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h7009, _) => {
            h7009
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h7010) => {
            h7010
        }
    })
}

pub fn f298(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a7017 = v3;
        let a7018 = {
            let a7014 = f311({
                let a7011 = v0;
                let a7012 = 2u64;
                let a7013 = 0u64;
                nat_quot(a7011, a7012, a7013)
            }, v1)?;
            let a7015 = v2;
            let a7016 = 0u64;
            nat_quot(a7014, a7015, a7016)
        };
        nat_add(a7017, a7018)?
    };
    let a7022 = {
        let a7019 = v0;
        let a7020 = 2u64;
        let a7021 = 0u64;
        nat_rem(a7019, a7020, a7021)
    };
    let a7023 = 0u64;
    if nat_eq(a7022, a7023) {
        Ok(f312(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a7024 = v4;
        let a7025 = 0u64;
        if nat_eq(a7024, a7025) {
            Ok(f312(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f299(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f313(match v2.clone() {
        Adt20::C0(h7026, _, _, _, _, _, _, _) => {
            h7026
        }
    })?;
    let v7: u64 = f314(match v2.clone() {
        Adt20::C0(h7027, _, _, _, _, _, _, _) => {
            h7027
        }
    })?;
    let a7028 = v0;
    let a7029 = 2u64;
    if nat_lt(a7028, a7029) {
        Ok(f310(v0))
    } else {
        let a7030 = v0;
        let a7031 = v6;
        if nat_lt(a7030, a7031) {
            f315({
                let a7032 = v0;
                let a7033 = 2u64;
                nat_sub(a7032, a7033)
            }, v1, v2, v3)
        } else {
            let a7034 = v0;
            let a7035 = v7;
            if nat_lt(a7034, a7035) {
                Ok(f316({
                    let a7036 = v0;
                    let a7037 = v6;
                    nat_sub(a7036, a7037)
                }, match v2 {
                    Adt20::C0(h7038, _, _, _, _, _, _, _) => {
                        h7038
                    }
                }, v5))
            } else {
                let a7040 = v0;
                let a7041 = f269(match v2.clone() {
                    Adt20::C0(h7039, _, _, _, _, _, _, _) => {
                        h7039
                    }
                })?;
                if nat_lt(a7040, a7041) {
                    f317({
                        let a7042 = v0;
                        let a7043 = v7;
                        nat_sub(a7042, a7043)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f300(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f285(v5, match v2.clone() {
        Adt20::C0(h7044, _, _, _, _, _, _, _) => {
            h7044
        }
    })?;
    let v7: u64 = {
        let a7046 = v6;
        let a7047 = f286(v5, match v2.clone() {
            Adt20::C0(h7045, _, _, _, _, _, _, _) => {
                h7045
            }
        })?;
        nat_add(a7046, a7047)?
    };
    let v8: u64 = {
        let a7048 = v7;
        let a7049 = f287(v5);
        nat_add(a7048, a7049)?
    };
    let a7050 = v0;
    let a7051 = v6;
    if nat_lt(a7050, a7051) {
        f318(v0, v1, v2, v5)
    } else {
        let a7052 = v0;
        let a7053 = v7;
        if nat_lt(a7052, a7053) {
            f319({
                let a7054 = v0;
                let a7055 = v6;
                nat_sub(a7054, a7055)
            }, v1, v2, v3, v5)
        } else {
            let a7056 = v0;
            let a7057 = v8;
            if nat_lt(a7056, a7057) {
                Ok(f320({
                    let a7058 = v0;
                    let a7059 = v7;
                    nat_sub(a7058, a7059)
                }, match v2 {
                    Adt20::C0(h7060, _, _, _, _, _, _, _) => {
                        h7060
                    }
                }, v4, v5))
            } else {
                let a7064 = v0;
                let a7065 = {
                    let a7062 = v8;
                    let a7063 = match v2.clone() {
                        Adt20::C0(h7061, _, _, _, _, _, _, _) => {
                            h7061
                        }
                    };
                    nat_add(a7062, a7063)?
                };
                if nat_lt(a7064, a7065) {
                    let v9: u64 = f311({
                        let a7071 = {
                            let a7067 = v1;
                            let a7068 = match v2 {
                                Adt20::C0(h7066, _, _, _, _, _, _, _) => {
                                    h7066
                                }
                            };
                            nat_mul(a7067, a7068)?
                        };
                        let a7072 = {
                            let a7069 = v0;
                            let a7070 = v8;
                            nat_sub(a7069, a7070)
                        };
                        nat_add(a7071, a7072)?
                    }, v3)?;
                    Ok(f312(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f301(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f288(v3, v5)?;
    let v7: u64 = f290(v1, v2, v6)?;
    let v8: u64 = {
        let a7075 = {
            let a7073 = f289(v3, v5)?;
            let a7074 = v4;
            nat_mul(a7073, a7074)?
        };
        let a7076 = v7;
        nat_mul(a7075, a7076)?
    };
    let a7077 = v0;
    let a7078 = v8;
    if nat_lt(a7077, a7078) {
        let v9: u64 = {
            let a7079 = v0;
            let a7080 = v7;
            let a7081 = 0u64;
            nat_quot(a7079, a7080, a7081)
        };
        Ok(Adt21::C0({
            let a7085 = {
                let a7082 = v9;
                let a7083 = v4;
                let a7084 = 0u64;
                nat_quot(a7082, a7083, a7084)
            };
            let a7086 = v6;
            nat_mul(a7085, a7086)?
        }, v6, {
            let a7087 = v9;
            let a7088 = v4;
            let a7089 = 0u64;
            nat_rem(a7087, a7088, a7089)
        }, {
            let a7090 = v0;
            let a7091 = v7;
            let a7092 = 0u64;
            nat_rem(a7090, a7091, a7092)
        }))
    } else {
        let v10: u64 = {
            let a7093 = v0;
            let a7094 = v8;
            nat_sub(a7093, a7094)
        };
        let v11: u64 = f290(v1, v2, f291(v3, v5)?)?;
        Ok(Adt21::C0({
            let a7095 = f289(v3, v5)?;
            let a7096 = v6;
            nat_mul(a7095, a7096)?
        }, f291(v3, v5)?, {
            let a7097 = v10;
            let a7098 = v11;
            let a7099 = 0u64;
            nat_quot(a7097, a7098, a7099)
        }, {
            let a7100 = v10;
            let a7101 = v11;
            let a7102 = 0u64;
            nat_rem(a7100, a7101, a7102)
        }))
    }
}

pub fn f302(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7103) => {
            h7103
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7104, _, _, _, _, _, _, _) => {
            h7104
        }
    };
    let v6: u64 = {
        let a7107 = match v0.clone() {
            Adt21::C0(_, h7105, _, _) => {
                h7105
            }
        };
        let a7108 = match v1.clone() {
            Adt20::C0(_, h7106, _, _, _, _, _, _) => {
                h7106
            }
        };
        nat_mul(a7107, a7108)?
    };
    let v7: u64 = {
        let a7109 = v6;
        let a7110 = v5;
        nat_mul(a7109, a7110)?
    };
    let v8: u64 = {
        let a7113 = {
            let a7111 = 2u64;
            let a7112 = v5;
            nat_mul(a7111, a7112)?
        };
        let a7114 = 2u64;
        nat_add(a7113, a7114)?
    };
    let v9: u64 = {
        let a7117 = {
            let a7115 = 4u64;
            let a7116 = v5;
            nat_mul(a7115, a7116)?
        };
        let a7118 = 2u64;
        nat_add(a7117, a7118)?
    };
    let v10: u64 = {
        let a7121 = 1u64;
        let a7122 = {
            let a7119 = v6;
            let a7120 = v8;
            nat_mul(a7119, a7120)?
        };
        nat_add(a7121, a7122)?
    };
    let v11: u64 = {
        let a7123 = v10;
        let a7124 = 4u64;
        nat_add(a7123, a7124)?
    };
    let v12: u64 = {
        let a7127 = v11;
        let a7128 = {
            let a7125 = v6;
            let a7126 = v9;
            nat_mul(a7125, a7126)?
        };
        nat_add(a7127, a7128)?
    };
    let v13: u64 = {
        let a7129 = v12;
        let a7130 = 6u64;
        nat_add(a7129, a7130)?
    };
    let a7131 = v4;
    let a7132 = 0u64;
    if nat_eq(a7131, a7132) {
        Ok(f310(0u64))
    } else {
        let a7133 = v4;
        let a7134 = v10;
        if nat_lt(a7133, a7134) {
            let v14: u64 = {
                let a7135 = v4;
                let a7136 = 1u64;
                nat_sub(a7135, a7136)
            };
            f321(false, {
                let a7137 = v14;
                let a7138 = v8;
                let a7139 = 0u64;
                nat_rem(a7137, a7138, a7139)
            }, {
                let a7140 = v14;
                let a7141 = v8;
                let a7142 = 0u64;
                nat_quot(a7140, a7141, a7142)
            }, v0, v1, v2)
        } else {
            let a7143 = v4;
            let a7144 = v11;
            if nat_lt(a7143, a7144) {
                Ok(f322({
                    let a7145 = v4;
                    let a7146 = v10;
                    nat_sub(a7145, a7146)
                }, v7))
            } else {
                let a7147 = v4;
                let a7148 = v12;
                if nat_lt(a7147, a7148) {
                    let v15: u64 = {
                        let a7149 = v4;
                        let a7150 = v11;
                        nat_sub(a7149, a7150)
                    };
                    f321(true, {
                        let a7151 = v15;
                        let a7152 = v9;
                        let a7153 = 0u64;
                        nat_rem(a7151, a7152, a7153)
                    }, {
                        let a7154 = v15;
                        let a7155 = v9;
                        let a7156 = 0u64;
                        nat_quot(a7154, a7155, a7156)
                    }, v0, v1, v2)
                } else {
                    let a7157 = v4;
                    let a7158 = v13;
                    if nat_lt(a7157, a7158) {
                        Ok(f323({
                            let a7159 = v4;
                            let a7160 = v12;
                            nat_sub(a7159, a7160)
                        }, v7, v3))
                    } else {
                        let a7163 = v4;
                        let a7164 = {
                            let a7161 = v13;
                            let a7162 = v7;
                            nat_add(a7161, a7162)?
                        };
                        if nat_lt(a7163, a7164) {
                            let v16: u64 = {
                                let a7165 = v4;
                                let a7166 = v13;
                                nat_sub(a7165, a7166)
                            };
                            let v17: u64 = {
                                let a7167 = v16;
                                let a7168 = v5;
                                let a7169 = 0u64;
                                nat_quot(a7167, a7168, a7169)
                            };
                            let v18: u64 = {
                                let a7175 = {
                                    let a7170 = f324(v17, v0, v1)?;
                                    let a7171 = v5;
                                    nat_mul(a7170, a7171)?
                                };
                                let a7176 = {
                                    let a7172 = v16;
                                    let a7173 = v5;
                                    let a7174 = 0u64;
                                    nat_rem(a7172, a7173, a7174)
                                };
                                nat_add(a7175, a7176)?
                            };
                            let v19: u64 = f311(v18, v2)?;
                            Ok(f312(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f303(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f304(v0: u64) -> u64 {
    let a7177 = v0;
    let a7178 = 4u64;
    let a7179 = 0u64;
    nat_quot(a7177, a7178, a7179)
}

pub fn f305(v0: u64) -> R<u64> {
    let a7182 = {
        let a7180 = 7u64;
        let a7181 = v0;
        nat_mul(a7180, a7181)?
    };
    let a7183 = 4u64;
    nat_add(a7182, a7183)
}

pub fn f306(v0: u64) -> bool {
    let m7184 = f216(v0);
    match m7184 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f307(v0: u64, v1: u64) -> bool {
    let a7185 = v0;
    let a7186 = 1u64;
    if nat_eq(a7185, a7186) {
        true
    } else {
        let v2: u64 = f325(v0);
        let a7187 = 0u64;
        let a7188 = v2;
        if nat_lt(a7187, a7188) {
            let a7192 = {
                let a7189 = v1;
                let a7190 = v2;
                let a7191 = 1u64;
                nat_rem(a7189, a7190, a7191)
            };
            let a7193 = 0u64;
            nat_eq(a7192, a7193)
        } else {
            false
        }
    }
}

pub fn f308(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7194 = v0;
    let a7195 = 28u64;
    if if if if nat_eq(a7194, a7195) {
        let a7196 = v1;
        let a7197 = 0u64;
        nat_eq(a7196, a7197)
    } else {
        false
    } {
        let a7198 = v2;
        let a7199 = 0u64;
        nat_eq(a7198, a7199)
    } else {
        false
    } {
        let a7200 = v3;
        let a7201 = 0u64;
        nat_eq(a7200, a7201)
    } else {
        false
    } {
        let a7202 = v4;
        let a7203 = v8;
        if if if if if if if if nat_eq(a7202, a7203) {
            let a7204 = v5;
            let a7205 = v12;
            nat_eq(a7204, a7205)
        } else {
            false
        } {
            let a7206 = v9;
            let a7207 = v13;
            nat_eq(a7206, a7207)
        } else {
            false
        } {
            let a7208 = v10;
            let a7209 = v14;
            nat_eq(a7208, a7209)
        } else {
            false
        } {
            let a7210 = v11;
            let a7211 = v15;
            nat_eq(a7210, a7211)
        } else {
            false
        } {
            let a7212 = 0u64;
            let a7213 = v6;
            nat_lt(a7212, a7213)
        } else {
            false
        } {
            let a7214 = 0u64;
            let a7215 = v7;
            nat_lt(a7214, a7215)
        } else {
            false
        } {
            let a7219 = {
                let a7216 = v10;
                let a7217 = v6;
                let a7218 = 1u64;
                nat_rem(a7216, a7217, a7218)
            };
            let a7220 = 0u64;
            if nat_eq(a7219, a7220) {
                let a7224 = {
                    let a7221 = v11;
                    let a7222 = v7;
                    let a7223 = 1u64;
                    nat_rem(a7221, a7222, a7223)
                };
                let a7225 = 0u64;
                nat_eq(a7224, a7225)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7226 = v10;
                let a7227 = v6;
                let a7228 = 0u64;
                nat_quot(a7226, a7227, a7228)
            }, {
                let a7229 = v11;
                let a7230 = v7;
                let a7231 = 0u64;
                nat_quot(a7229, a7230, a7231)
            }, {
                let a7232 = v5;
                let a7233 = v4;
                nat_mul(a7232, a7233)?
            }, {
                let a7234 = v4;
                let a7235 = v9;
                nat_mul(a7234, a7235)?
            }, {
                let a7236 = v5;
                let a7237 = v9;
                nat_mul(a7236, a7237)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f309(v0: u64) -> bool {
    let a7238 = v0;
    let a7239 = 0u64;
    if if nat_eq(a7238, a7239) {
        true
    } else {
        let a7240 = v0;
        let a7241 = 1u64;
        nat_eq(a7240, a7241)
    } {
        true
    } else {
        let a7242 = v0;
        let a7243 = 30u64;
        nat_eq(a7242, a7243)
    }
}

pub fn f310(v0: u64) -> Adt17 {
    f312(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f311(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7245 = v0;
        let a7246 = match v1.clone() {
            Adt20::C0(h7244, _, _, _, _, _, _, _) => {
                h7244
            }
        };
        let a7247 = 0u64;
        nat_quot(a7245, a7246, a7247)
    };
    let v3: u64 = {
        let a7249 = v2;
        let a7250 = match v1.clone() {
            Adt20::C0(_, h7248, _, _, _, _, _, _) => {
                h7248
            }
        };
        let a7251 = 0u64;
        nat_quot(a7249, a7250, a7251)
    };
    f142({
        let a7253 = v0;
        let a7254 = match v1.clone() {
            Adt20::C0(h7252, _, _, _, _, _, _, _) => {
                h7252
            }
        };
        let a7255 = 0u64;
        nat_rem(a7253, a7254, a7255)
    }, {
        let a7257 = v2;
        let a7258 = match v1.clone() {
            Adt20::C0(_, h7256, _, _, _, _, _, _) => {
                h7256
            }
        };
        let a7259 = 0u64;
        nat_rem(a7257, a7258, a7259)
    }, {
        let a7261 = v3;
        let a7262 = match v1.clone() {
            Adt20::C0(_, _, h7260, _, _, _, _, _) => {
                h7260
            }
        };
        let a7263 = 0u64;
        nat_rem(a7261, a7262, a7263)
    }, {
        let a7265 = v3;
        let a7266 = match v1.clone() {
            Adt20::C0(_, _, h7264, _, _, _, _, _) => {
                h7264
            }
        };
        let a7267 = 0u64;
        nat_quot(a7265, a7266, a7267)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7268, _, _, _) => {
            h7268
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7269, _, _) => {
            h7269
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7270, _) => {
            h7270
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7271) => {
            h7271
        }
    })
}

pub fn f312(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7272 = v0;
    let a7273 = 4u64;
    if if if nat_le(a7272, a7273) {
        true
    } else {
        let a7274 = 9u64;
        let a7275 = v0;
        if nat_le(a7274, a7275) {
            let a7276 = v0;
            let a7277 = 12u64;
            nat_le(a7276, a7277)
        } else {
            false
        }
    } {
        true
    } else {
        let a7278 = v0;
        let a7279 = 16u64;
        if if nat_eq(a7278, a7279) {
            true
        } else {
            let a7280 = v0;
            let a7281 = 19u64;
            nat_eq(a7280, a7281)
        } {
            true
        } else {
            let a7282 = v0;
            let a7283 = 21u64;
            if nat_eq(a7282, a7283) {
                true
            } else {
                let a7284 = v0;
                let a7285 = 22u64;
                nat_eq(a7284, a7285)
            }
        }
    } {
        Adt17::C1({
            let a7286 = v0;
            let a7287 = 4u64;
            if if nat_lt(a7286, a7287) {
                true
            } else {
                let a7288 = 21u64;
                let a7289 = v0;
                nat_le(a7288, a7289)
            } {
                0u64
            } else {
                let a7290 = v0;
                let a7291 = 16u64;
                if if nat_eq(a7290, a7291) {
                    true
                } else {
                    let a7292 = v0;
                    let a7293 = 19u64;
                    nat_eq(a7292, a7293)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7294 = v0;
            let a7295 = 4u64;
            if nat_lt(a7294, a7295) {
                v0
            } else {
                let a7296 = v0;
                let a7297 = 4u64;
                if if nat_eq(a7296, a7297) {
                    true
                } else {
                    let a7298 = v0;
                    let a7299 = 16u64;
                    nat_eq(a7298, a7299)
                } {
                    0u64
                } else {
                    let a7300 = v0;
                    let a7301 = 19u64;
                    if nat_eq(a7300, a7301) {
                        3u64
                    } else {
                        let a7302 = 21u64;
                        let a7303 = v0;
                        if nat_le(a7302, a7303) {
                            let a7304 = v0;
                            let a7305 = 17u64;
                            nat_sub(a7304, a7305)
                        } else {
                            let a7306 = v0;
                            let a7307 = 8u64;
                            nat_sub(a7306, a7307)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f313(v0: u64) -> R<u64> {
    let a7310 = 2u64;
    let a7311 = {
        let a7308 = 6u64;
        let a7309 = v0;
        nat_mul(a7308, a7309)?
    };
    nat_add(a7310, a7311)
}

pub fn f314(v0: u64) -> R<u64> {
    let a7312 = f313(v0)?;
    let a7313 = 11u64;
    nat_add(a7312, a7313)
}

pub fn f315(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7314 = v0;
        let a7315 = 6u64;
        let a7316 = 0u64;
        nat_rem(a7314, a7315, a7316)
    };
    let v5: u64 = {
        let a7323 = {
            let a7318 = v1;
            let a7319 = match v2.clone() {
                Adt20::C0(h7317, _, _, _, _, _, _, _) => {
                    h7317
                }
            };
            nat_mul(a7318, a7319)?
        };
        let a7324 = {
            let a7320 = v0;
            let a7321 = 6u64;
            let a7322 = 0u64;
            nat_quot(a7320, a7321, a7322)
        };
        nat_add(a7323, a7324)?
    };
    let v6: u64 = f311(v5, v3)?;
    let v7: u64 = f311(v5, v2)?;
    let a7325 = v4;
    let a7326 = 0u64;
    if nat_eq(a7325, a7326) {
        Ok(f312(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7327 = v4;
        let a7328 = 1u64;
        if nat_eq(a7327, a7328) {
            Ok(f312(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7329 = v4;
            let a7330 = 2u64;
            if nat_eq(a7329, a7330) {
                Ok(f312(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7331 = v4;
                let a7332 = 3u64;
                if nat_eq(a7331, a7332) {
                    Ok(f312(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7333 = v4;
                    let a7334 = 4u64;
                    if nat_eq(a7333, a7334) {
                        Ok(f312(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f312(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f316(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7335 = v0;
    let a7336 = 0u64;
    if nat_eq(a7335, a7336) {
        f312(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7337 = v0;
        let a7338 = 1u64;
        if nat_eq(a7337, a7338) {
            f312(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7339 = v0;
            let a7340 = 2u64;
            if nat_eq(a7339, a7340) {
                f312(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7341 = v0;
                let a7342 = 3u64;
                if nat_eq(a7341, a7342) {
                    f312(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7343 = v0;
                    let a7344 = 4u64;
                    if nat_eq(a7343, a7344) {
                        f312(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7345 = v0;
                        let a7346 = 5u64;
                        if nat_eq(a7345, a7346) {
                            f312(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7347 = v0;
                            let a7348 = 6u64;
                            if nat_eq(a7347, a7348) {
                                f312(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7349 = v0;
                                let a7350 = 7u64;
                                if nat_eq(a7349, a7350) {
                                    f312(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7351 = v0;
                                    let a7352 = 8u64;
                                    if nat_eq(a7351, a7352) {
                                        f312(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7353 = v0;
                                        let a7354 = 9u64;
                                        if nat_eq(a7353, a7354) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7355 = v0;
                                            let a7356 = 10u64;
                                            if nat_eq(a7355, a7356) {
                                                f312(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f317(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7357 = v0;
        let a7358 = 3u64;
        let a7359 = 0u64;
        nat_rem(a7357, a7358, a7359)
    };
    let v6: u64 = {
        let a7366 = {
            let a7361 = v1;
            let a7362 = match v2.clone() {
                Adt20::C0(h7360, _, _, _, _, _, _, _) => {
                    h7360
                }
            };
            nat_mul(a7361, a7362)?
        };
        let a7367 = {
            let a7363 = v0;
            let a7364 = 3u64;
            let a7365 = 0u64;
            nat_quot(a7363, a7364, a7365)
        };
        nat_add(a7366, a7367)?
    };
    let v7: u64 = f311(v6, v3)?;
    let v8: u64 = f311(v6, v2)?;
    let v9: u64 = f311(v6, v4)?;
    let a7368 = v5;
    let a7369 = 0u64;
    if nat_eq(a7368, a7369) {
        Ok(f312(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7370 = v5;
        let a7371 = 1u64;
        if nat_eq(a7370, a7371) {
            Ok(f312(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f312(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f318(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7372, _, _, _, _, _, _, _) => {
            h7372
        }
    };
    let v5: u64 = {
        let a7375 = 1u64;
        let a7376 = {
            let a7373 = v4;
            let a7374 = f303(v3);
            nat_mul(a7373, a7374)?
        };
        nat_add(a7375, a7376)?
    };
    let a7377 = v0;
    let a7378 = 0u64;
    if nat_eq(a7377, a7378) {
        Ok(f310(0u64))
    } else {
        let a7379 = v0;
        let a7380 = v5;
        if nat_lt(a7379, a7380) {
            let v6: u64 = {
                let a7381 = v0;
                let a7382 = 1u64;
                nat_sub(a7381, a7382)
            };
            let v7: u64 = f311({
                let a7388 = {
                    let a7383 = v1;
                    let a7384 = v4;
                    nat_mul(a7383, a7384)?
                };
                let a7389 = {
                    let a7385 = v6;
                    let a7386 = f303(v3);
                    let a7387 = 0u64;
                    nat_quot(a7385, a7386, a7387)
                };
                nat_add(a7388, a7389)?
            }, v2)?;
            if v3 {
                let a7393 = {
                    let a7390 = v6;
                    let a7391 = 2u64;
                    let a7392 = 0u64;
                    nat_rem(a7390, a7391, a7392)
                };
                let a7394 = 0u64;
                if nat_eq(a7393, a7394) {
                    Ok(f312(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f312(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f312(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7395 = v0;
                let a7396 = v5;
                nat_sub(a7395, a7396)
            };
            let a7397 = v8;
            let a7398 = 0u64;
            if nat_eq(a7397, a7398) {
                Ok(f312(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7399 = v8;
                    let a7400 = 1u64;
                    nat_eq(a7399, a7400)
                } else {
                    false
                } {
                    Ok(f312(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7401 = v8;
                    let a7402 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7401, a7402) {
                        Ok(f312(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7403 = v8;
                        let a7404 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7403, a7404) {
                            Ok(f310(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f319(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7406 = 13u64;
        let a7407 = f304(match v2.clone() {
            Adt20::C0(h7405, _, _, _, _, _, _, _) => {
                h7405
            }
        });
        nat_mul(a7406, a7407)?
    } else {
        0u64
    };
    let a7408 = v0;
    let a7409 = v5;
    if nat_lt(a7408, a7409) {
        f326({
            let a7410 = v0;
            let a7411 = 13u64;
            let a7412 = 0u64;
            nat_rem(a7410, a7411, a7412)
        }, v1, {
            let a7413 = v0;
            let a7414 = 13u64;
            let a7415 = 0u64;
            nat_quot(a7413, a7414, a7415)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7416 = v0;
            let a7417 = v5;
            nat_sub(a7416, a7417)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f327({
            let a7418 = v6;
            let a7419 = v7;
            let a7420 = 0u64;
            nat_rem(a7418, a7419, a7420)
        }, {
            let a7430 = {
                let a7425 = {
                    let a7422 = v1;
                    let a7423 = match v2.clone() {
                        Adt20::C0(h7421, _, _, _, _, _, _, _) => {
                            h7421
                        }
                    };
                    nat_mul(a7422, a7423)?
                };
                let a7426 = if v4 {
                    f328(match v2.clone() {
                        Adt20::C0(h7424, _, _, _, _, _, _, _) => {
                            h7424
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7425, a7426)?
            };
            let a7431 = {
                let a7427 = v6;
                let a7428 = v7;
                let a7429 = 0u64;
                nat_quot(a7427, a7428, a7429)
            };
            nat_add(a7430, a7431)?
        }, v2, v3, v4)
    }
}

pub fn f320(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7432 = v0;
    let a7433 = 0u64;
    if nat_eq(a7432, a7433) {
        f312(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7434 = v0;
        let a7435 = 1u64;
        if nat_eq(a7434, a7435) {
            f312(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7436 = v0;
                let a7437 = 2u64;
                nat_eq(a7436, a7437)
            } else {
                false
            } {
                f312(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7438 = v0;
                let a7439 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7438, a7439) {
                    f312(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7440 = v0;
                    let a7441 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7440, a7441) {
                        f312(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7442 = v0;
                        let a7443 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7442, a7443) {
                            f312(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f321(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7447 = 1u64;
        let a7448 = {
            let a7445 = match v4.clone() {
                Adt20::C0(h7444, _, _, _, _, _, _, _) => {
                    h7444
                }
            };
            let a7446 = v6;
            nat_mul(a7445, a7446)?
        };
        nat_add(a7447, a7448)?
    };
    let a7449 = v1;
    let a7450 = 0u64;
    if nat_eq(a7449, a7450) {
        Ok(f310(1u64))
    } else {
        let a7451 = v1;
        let a7452 = v7;
        if nat_lt(a7451, a7452) {
            let v8: u64 = {
                let a7453 = v1;
                let a7454 = 1u64;
                nat_sub(a7453, a7454)
            };
            let v9: u64 = {
                let a7461 = {
                    let a7456 = f324(v2, v3, v4.clone())?;
                    let a7457 = match v4.clone() {
                        Adt20::C0(h7455, _, _, _, _, _, _, _) => {
                            h7455
                        }
                    };
                    nat_mul(a7456, a7457)?
                };
                let a7462 = {
                    let a7458 = v8;
                    let a7459 = v6;
                    let a7460 = 0u64;
                    nat_quot(a7458, a7459, a7460)
                };
                nat_add(a7461, a7462)?
            };
            let v10: u64 = f311(v9, v4)?;
            let v11: u64 = f311(v9, v5)?;
            let v12: u64 = {
                let a7463 = v8;
                let a7464 = v6;
                let a7465 = 0u64;
                nat_rem(a7463, a7464, a7465)
            };
            if v0 {
                let a7466 = v12;
                let a7467 = 0u64;
                if nat_eq(a7466, a7467) {
                    Ok(f312(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7468 = v12;
                    let a7469 = 1u64;
                    if nat_eq(a7468, a7469) {
                        Ok(f312(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7470 = v12;
                        let a7471 = 2u64;
                        if nat_eq(a7470, a7471) {
                            Ok(f312(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f312(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7472 = v12;
                let a7473 = 0u64;
                if nat_eq(a7472, a7473) {
                    Ok(f312(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f312(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7474 = v1;
            let a7475 = v7;
            if nat_eq(a7474, a7475) {
                Ok(f312(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f322(v0: u64, v1: u64) -> Adt17 {
    let a7476 = v0;
    let a7477 = 0u64;
    if nat_eq(a7476, a7477) {
        f312(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7478 = v0;
        let a7479 = 1u64;
        if nat_eq(a7478, a7479) {
            f312(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7480 = v0;
            let a7481 = 2u64;
            if nat_eq(a7480, a7481) {
                f312(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7482 = v0;
                let a7483 = 3u64;
                if nat_eq(a7482, a7483) {
                    f310(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f323(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7484 = v0;
    let a7485 = 0u64;
    if nat_eq(a7484, a7485) {
        f312(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7486 = v0;
        let a7487 = 1u64;
        if nat_eq(a7486, a7487) {
            f312(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7488 = v0;
            let a7489 = 2u64;
            if nat_eq(a7488, a7489) {
                f312(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7490 = v0;
                let a7491 = 3u64;
                if nat_eq(a7490, a7491) {
                    f312(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7492 = v0;
                    let a7493 = 4u64;
                    if nat_eq(a7492, a7493) {
                        f312(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7494 = v0;
                        let a7495 = 5u64;
                        if nat_eq(a7494, a7495) {
                            f312(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f324(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7506 = {
        let a7504 = {
            let a7501 = {
                let a7498 = match v1.clone() {
                    Adt21::C0(_, _, h7496, _) => {
                        h7496
                    }
                };
                let a7499 = match v2.clone() {
                    Adt20::C0(_, _, h7497, _, _, _, _, _) => {
                        h7497
                    }
                };
                nat_mul(a7498, a7499)?
            };
            let a7502 = match v1 {
                Adt21::C0(h7500, _, _, _) => {
                    h7500
                }
            };
            nat_add(a7501, a7502)?
        };
        let a7505 = match v2 {
            Adt20::C0(_, h7503, _, _, _, _, _, _) => {
                h7503
            }
        };
        nat_mul(a7504, a7505)?
    };
    let a7507 = v0;
    nat_add(a7506, a7507)
}

pub fn f325(v0: u64) -> u64 {
    if f223(v0) {
        let m7508 = f216(v0);
        match m7508 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7510 = 1u64;
                let a7511 = match v1.clone() {
                    Adt19::C0(_, h7509, _) => {
                        h7509
                    }
                };
                if if nat_lt(a7510, a7511) {
                    true
                } else {
                    let a7512 = v0;
                    let a7513 = 43u64;
                    nat_eq(a7512, a7513)
                } {
                    match v1 {
                        Adt19::C0(_, h7514, _) => {
                            h7514
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

pub fn f326(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7522 = {
            let a7520 = {
                let a7516 = v1;
                let a7517 = match v3.clone() {
                    Adt20::C0(h7515, _, _, _, _, _, _, _) => {
                        h7515
                    }
                };
                nat_mul(a7516, a7517)?
            };
            let a7521 = {
                let a7518 = v2;
                let a7519 = 4u64;
                nat_mul(a7518, a7519)?
            };
            nat_add(a7520, a7521)?
        };
        let a7523 = f329(v0);
        nat_add(a7522, a7523)?
    };
    let v6: u64 = f311(v5, v3)?;
    let v7: u64 = f311(v5, v4)?;
    let a7524 = v0;
    let a7525 = 0u64;
    if if if if nat_eq(a7524, a7525) {
        true
    } else {
        let a7526 = v0;
        let a7527 = 2u64;
        nat_eq(a7526, a7527)
    } {
        true
    } else {
        let a7528 = v0;
        let a7529 = 5u64;
        nat_eq(a7528, a7529)
    } {
        true
    } else {
        let a7530 = v0;
        let a7531 = 7u64;
        nat_eq(a7530, a7531)
    } {
        Ok(f312(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7532 = v0;
        let a7533 = 1u64;
        if if if if nat_eq(a7532, a7533) {
            true
        } else {
            let a7534 = v0;
            let a7535 = 3u64;
            nat_eq(a7534, a7535)
        } {
            true
        } else {
            let a7536 = v0;
            let a7537 = 6u64;
            nat_eq(a7536, a7537)
        } {
            true
        } else {
            let a7538 = v0;
            let a7539 = 8u64;
            nat_eq(a7538, a7539)
        } {
            Ok(f312(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7540 = v0;
                let a7541 = 1u64;
                if nat_eq(a7540, a7541) {
                    4u64
                } else {
                    let a7542 = v0;
                    let a7543 = 6u64;
                    if nat_eq(a7542, a7543) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7544 = v0;
            let a7545 = 4u64;
            if nat_eq(a7544, a7545) {
                Ok(f312(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7546 = v0;
                let a7547 = 9u64;
                if nat_eq(a7546, a7547) {
                    Ok(f312(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7548 = v0;
                    let a7549 = 10u64;
                    if nat_eq(a7548, a7549) {
                        Ok(f312(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7550 = v0;
                        let a7551 = 11u64;
                        if nat_eq(a7550, a7551) {
                            Ok(f312(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7552 = v0;
                            let a7553 = 12u64;
                            if nat_eq(a7552, a7553) {
                                Ok(f312(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f327(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f311(v1, v2)?;
    let v6: u64 = f311(v1, v3)?;
    let a7554 = v0;
    let a7555 = 0u64;
    if nat_eq(a7554, a7555) {
        Ok(f312(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7556 = v0;
        let a7557 = 1u64;
        if nat_eq(a7556, a7557) {
            Ok(f312(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7558 = v0;
                let a7559 = 2u64;
                if nat_eq(a7558, a7559) {
                    Ok(f312(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7560 = v0;
                    let a7561 = 3u64;
                    if nat_eq(a7560, a7561) {
                        Ok(f312(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7562 = v0;
                let a7563 = 2u64;
                if nat_eq(a7562, a7563) {
                    Ok(f312(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f328(v0: u64) -> R<u64> {
    let a7564 = 4u64;
    let a7565 = f304(v0);
    nat_mul(a7564, a7565)
}

pub fn f329(v0: u64) -> u64 {
    let a7566 = v0;
    let a7567 = 5u64;
    if nat_lt(a7566, a7567) {
        let a7568 = v0;
        let a7569 = 2u64;
        if nat_lt(a7568, a7569) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7570 = v0;
        let a7571 = 7u64;
        if nat_lt(a7570, a7571) {
            2u64
        } else {
            3u64
        }
    }
}

pub fn run(p0: Adt0) -> R<Adt9> {
    f0(p0)
}
