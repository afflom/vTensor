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
    C45(bool, u64, u64, Adt1),
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
                f79(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C19(v81, v82) => {
                f80(v81, v0, v1, v2, v82, v4)?
            }
            Adt4::C20(v83, v84) => {
                f81(v83, v0, v1, v2, v84, v4)?
            }
            Adt4::C21 => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C22(v85, v86) => {
                f82(v0, v1, v2, v85, v86, v4)?
            }
            Adt4::C23(v100, v101, v102, v103) => {
                f89(v0, v103, v100, v101, v102, v4)?
            }
            Adt4::C24(v99) => {
                if v5 {
                    f87(Adt16::C0(v99), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f63(v0, v1, v2, v3)?
            }
            Adt4::C26(v95, v96, v97, v98) => {
                let a970 = v95;
                let a971 = 64u64;
                if if if nat_eq(a970, a971) {
                    true
                } else {
                    let a972 = v95;
                    let a973 = 65u64;
                    nat_eq(a972, a973)
                } {
                    true
                } else {
                    let a974 = v95;
                    let a975 = 66u64;
                    nat_eq(a974, a975)
                } {
                    if v5 {
                        f87(Adt16::C1(v95, v96, v97, v98), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a976 = v95;
                    let a977 = 0u64;
                    if if nat_eq(a976, a977) {
                        true
                    } else {
                        let a978 = v95;
                        let a979 = 30u64;
                        nat_eq(a978, a979)
                    } {
                        f88(v0, v1, v2, v95, v96, v97, v98, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v94) => {
                f86(v0, v1, v2, v94)?
            }
            Adt4::C28(v90) => {
                f83(v0, v1, v2, v90, v4)?
            }
            Adt4::C29(v91) => {
                f84(v0, v1, v2, v91)?
            }
            Adt4::C30(v92, _) => {
                f85(v0, v1, v2, v92)?
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
            Adt4::C34(v61, v62, v63, v64) => {
                f78(v0, v1, v61, v2, v62, v63, v64)?
            }
            Adt4::C35(v60) => {
                f77(v0, v60, v1, v2)?
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
                f76(v0, v1, v2, v4)?
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
            Adt4::C45(v56, v57, v58, v59) => {
                f75(v0, v1, v2, v56, v57, v58, v59, v4)?
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
        Adt4::C25(v78, _, v80) => {
            Ok(Adt9::C6({
                let a986 = {
                    let a982 = 0u64;
                    let a983 = v78;
                    if nat_lt(a982, a983) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a987 = {
                    let a984 = 0u64;
                    let a985 = v80;
                    if nat_lt(a984, a985) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a986, a987)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v86, _, _, v89) => {
            Ok(Adt9::C6({
                let a988 = v86;
                let a989 = 66u64;
                if nat_eq(a988, a989) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v89))
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
        Adt4::C35(v51) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v51))
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
        Adt4::C45(_, _, _, _) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, 3u64))
        }
    }
}

pub fn f20(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a990 = v1;
    let a991 = 5u64;
    if if if if if nat_lt(a990, a991) {
        true
    } else {
        f90(v0, v1, v2, v3, v4)
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
        Adt9::C7(f91(v0, v1, v2, v3, v4), f92(v5, v6), f92(v7, v8), f93({
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
            let a1069 = f94({
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
                let a1081 = f94({
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
                let a1168 = f94({
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
        f95(Adt6::C0(1u64, match v0.clone() {
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
        f95(v0, v1)
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
        let a1261 = f96(v3);
        nat_mul(a1260, a1261)?
    };
    nat_mul(a1262, a1263)
}

pub fn f41(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1266 = v0;
    let a1267 = {
        let a1264 = f97(v1)?;
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
        f98(v1, v4, v6)
    } else {
        let a1281 = v0;
        let a1282 = 26u64;
        if nat_eq(a1281, a1282) {
            f99(v1, v2, v4, v6)
        } else {
            let a1283 = v0;
            let a1284 = 25u64;
            if nat_eq(a1283, a1284) {
                f100(v1, v3, v4, v5, v6)
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
        f101(match v0.clone() {
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
    f102({
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
            let a1351 = f103(v1, v2, v3)?;
            let a1352 = f104(v0, v1, v2)?;
            nat_add(a1351, a1352)?
        };
        let a1358 = {
            let a1355 = f105(v0);
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
        let a1374 = f106(v0)?;
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
                if f107(v0) {
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
                    f108(v1, v2, v4)?
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
                        f109(v1, v2, v7, v4)?
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
                            f110(v1, v2, v8, v4)?
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
                                f111(v2, v9, v4)?
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
                                    f112(v1, v2, v10, v4)?
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
                                        f113(v1, v2, v11, v4)?
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
                                            f114(v2, v12, v4)?
                                        }
                                    }
                                } else {
                                    let a1449 = v0;
                                    let a1450 = 44u64;
                                    if nat_eq(a1449, a1450) {
                                        f115(v1, v2, v3, v4)?
                                    } else {
                                        let a1451 = v0;
                                        let a1452 = 52u64;
                                        if nat_eq(a1451, a1452) {
                                            f116(v1, v2, v4)?
                                        } else {
                                            let a1453 = v0;
                                            let a1454 = 73u64;
                                            if nat_eq(a1453, a1454) {
                                                f117(v2, v3, v4)?
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
                                                            f118(false, {
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
                                                        f119(false, v1, v2, v3, v4)?
                                                    } else {
                                                        let a1464 = v0;
                                                        let a1465 = 51u64;
                                                        if nat_eq(a1464, a1465) {
                                                            f120(v1, v2, v3, v4)?
                                                        } else {
                                                            let a1466 = v0;
                                                            let a1467 = 39u64;
                                                            if nat_eq(a1466, a1467) {
                                                                f121(v2, v3, v4)?
                                                            } else {
                                                                let a1468 = v0;
                                                                let a1469 = 77u64;
                                                                if nat_eq(a1468, a1469) {
                                                                    f122(v2, v4)?
                                                                } else {
                                                                    let a1470 = v0;
                                                                    let a1471 = 40u64;
                                                                    if nat_eq(a1470, a1471) {
                                                                        f123(v2, v3, v4)?
                                                                    } else {
                                                                        if f124(v0) {
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
                                                                                        f125(v2, v14, v4)?
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
                                                                                    f126(match v2.clone() {
                                                                                        Adt2::C0(h1486, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1486
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f127(v2, v3, v4)?
                                                                                } else {
                                                                                    f128(v0, v2, {
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
                                                                                f129(v2, v4)?
                                                                            } else {
                                                                                let a1490 = v0;
                                                                                let a1491 = 21u64;
                                                                                if nat_eq(a1490, a1491) {
                                                                                    f130(v5, v2, v3, v4)?
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
                                                                                        f131(v0, v1, v2, v4)?
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
                                                                                            f132(v0, v1, v2, v3, v4)?
                                                                                        } else {
                                                                                            let a1502 = v0;
                                                                                            let a1503 = 20u64;
                                                                                            if nat_eq(a1502, a1503) {
                                                                                                f133(v2, v4)?
                                                                                            } else {
                                                                                                let a1504 = v0;
                                                                                                let a1505 = 42u64;
                                                                                                if nat_eq(a1504, a1505) {
                                                                                                    f134(4u64, v2, v4)?
                                                                                                } else {
                                                                                                    let a1506 = v0;
                                                                                                    let a1507 = 69u64;
                                                                                                    if nat_eq(a1506, a1507) {
                                                                                                        let a1508 = v5;
                                                                                                        let a1509 = 4u64;
                                                                                                        if nat_lt(a1508, a1509) {
                                                                                                            f134(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            Adt10::C1
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1510 = v0;
                                                                                                        let a1511 = 70u64;
                                                                                                        if nat_eq(a1510, a1511) {
                                                                                                            f135(v5, v2, v4)?
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
                                                                                                                f136({
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
                                                                                                                    f137(v0, v1, v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1524 = v0;
                                                                                                                    let a1525 = 15u64;
                                                                                                                    if nat_eq(a1524, a1525) {
                                                                                                                        f138(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1526 = v0;
                                                                                                                        let a1527 = 17u64;
                                                                                                                        if nat_eq(a1526, a1527) {
                                                                                                                            f139(v2, v4)?
                                                                                                                        } else {
                                                                                                                            let a1528 = v0;
                                                                                                                            let a1529 = 18u64;
                                                                                                                            if nat_eq(a1528, a1529) {
                                                                                                                                f140(v2, v3, v4)?
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
                                                                                                                                    f141(v0, v2, v4)?
                                                                                                                                } else {
                                                                                                                                    let m1536 = f142(v0, v5)?;
                                                                                                                                    match m1536 {
                                                                                                                                        None => {
                                                                                                                                            f143(v0, v5, v2, v3, v4)?
                                                                                                                                        }
                                                                                                                                        Some(v16) => {
                                                                                                                                            f144(v16, v1, v2, v4)?
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
    f145(v1, v2, v3, v4, match v0.clone() {
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
        let a1590 = f146(match v0.clone() {
            Adt6::C0(_, _, h1588, _) => {
                h1588
            }
        });
        nat_add(a1589, a1590)?
    };
    let v5: u64 = f147(match v0 {
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
            f148(f149(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f150(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f151(v0.clone(), v24);
            f148(f152(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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
            Ok(Adt9::C1(false, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f154(v0, v1, v2.clone())?, f94(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f155(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f156(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f157(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f156(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f158(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f159(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f160(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f94(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f161(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v60, v61) => {
            f162(v0, v1, v2, v60, v61)
        }
        Adt4::C12(v62, v63, v64) => {
            f163(v0, v1, v2, v62, v63, v64)
        }
        Adt4::C13(_, v66, _, _) => {
            Ok(Adt9::C3(v66, v66, f94(v0, v1)?, f94(v0, v2)?))
        }
        Adt4::C14(v69, v70, v71) => {
            let a1626 = v69;
            let a1627 = 23u64;
            if nat_eq(a1626, a1627) {
                f164(v0, v1, v2, v70, v71)
            } else {
                Ok(Adt9::C3(v70, v71, f94(v0, v1)?, f94(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f165(v0, v1, v2)
        }
        Adt4::C16(v72, v73) => {
            f166(v0, v1, v2, v72, v73)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v74, v75) => {
            f167(v0, v1, v75, v74)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f94(f153(v0, match v2.clone() {
                Adt3::C0(h1624, _, _, _, _, _, _, _) => {
                    h1624
                }
            }, match v2.clone() {
                Adt3::C0(_, h1625, _, _, _, _, _, _) => {
                    h1625
                }
            })?, v1)?, f94(v0, v2)?))
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
        Adt4::C25(v80, v81, v82) => {
            f168(v0, v80, v81, v82)
        }
        Adt4::C26(v88, _, _, _) => {
            let a1628 = v88;
            let a1629 = 64u64;
            if if if nat_eq(a1628, a1629) {
                true
            } else {
                let a1630 = v88;
                let a1631 = 65u64;
                nat_eq(a1630, a1631)
            } {
                true
            } else {
                let a1632 = v88;
                let a1633 = 66u64;
                nat_eq(a1632, a1633)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v87) => {
            f86(v0, v1, v2, v87)
        }
        Adt4::C28(v83) => {
            f83(v0, v1, v2, v83, 0u64)
        }
        Adt4::C29(v84) => {
            f84(v0, v1, v2, v84)
        }
        Adt4::C30(v85, _) => {
            f85(v0, v1, v2, v85)
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
        Adt4::C45(_, _, _, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f169(v3, v4);
    let v6: u64 = {
        let a1634 = v0;
        let a1635 = v5;
        let a1636 = 0u64;
        nat_quot(a1634, a1635, a1636)
    };
    let a1637 = v5;
    let a1638 = 1u64;
    if nat_eq(a1637, a1638) {
        Ok(f148(f170(v3, v4), v3, v4, 0u64, f94(v6, v1)?, 2u64, 0u64, 3u64, f94(v6, v2)?))
    } else {
        let a1642 = {
            let a1639 = v0;
            let a1640 = 2u64;
            let a1641 = 0u64;
            nat_rem(a1639, a1640, a1641)
        };
        let a1643 = 0u64;
        if nat_eq(a1642, a1643) {
            Ok(f148(f170(v3, 0u64), v3, 0u64, 0u64, f94(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f148(f170(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f94(v6, v2)?))
        }
    }
}

pub fn f65(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f171(v1)?;
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
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f94(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1694 = v5;
        let a1695 = 1u64;
        if nat_eq(a1694, a1695) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1696 = v5;
            let a1697 = 2u64;
            if nat_eq(a1696, a1697) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f94(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1698 = v5;
                let a1699 = 3u64;
                if nat_eq(a1698, a1699) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1700 = v5;
                    let a1701 = 4u64;
                    if nat_eq(a1700, a1701) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f94(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f94(v4, v3)?), 4u64, 4u64))
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
    let v7: u64 = f172(match v1.clone() {
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
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f94(v9, v3)?), 4u64, 4u64))
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
                }), Adt13::C1(f94(v8, v5)?), 4u64, 4u64))
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
                            Adt13::C1(f94(v8, v5)?)
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
        f173({
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
        f174({
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
        let v7: u64 = f94({
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
                    let a2168 = f147(v2);
                    let a2169 = f146(v2);
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
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f94(v0, v2)?), 4u64, 4u64))
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
        let v7: u64 = f175(match v3.clone() {
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
                                }), Adt12::C0(0u64, f94(v9, v1)?), Adt13::C1({
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
                            let m2274 = f176(v3, v2, v1, v9, {
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
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f94(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2291 = f176(v3, v2, v1, v9, {
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
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f94(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2310 = v8;
                let a2311 = 1u64;
                nat_sub(a2310, a2311)
            };
            let m2317 = f177(v3.clone(), match v1.clone() {
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
                    let m2323 = f177(v4, match v1.clone() {
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

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: Adt1, v7: u64) -> R<Adt9> {
    let v8: u64 = f178(v3, v5)?;
    let v9: u64 = {
        let a2328 = v0;
        let a2329 = v8;
        let a2330 = 0u64;
        nat_rem(a2328, a2329, a2330)
    };
    let v10: u64 = {
        let a2331 = v0;
        let a2332 = v8;
        let a2333 = 0u64;
        nat_quot(a2331, a2332, a2333)
    };
    let v11: Adt13 = Adt13::C1(f94(v10, v2)?);
    let v12: Adt13 = {
        let a2334 = v5;
        let a2335 = 0u64;
        if nat_eq(a2334, a2335) {
            v11.clone()
        } else {
            Adt13::C0(0u64)
        }
    };
    let a2336 = v9;
    let a2337 = 0u64;
    if nat_eq(a2336, a2337) {
        Ok(Adt9::C7(Adt11::C2(v4, 0u64, true), Adt12::C0(0u64, f94(v10, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f179(v4), 4u64))
    } else {
        let a2338 = v9;
        let a2339 = 1u64;
        if nat_eq(a2338, a2339) {
            Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
        } else {
            let a2340 = v9;
            let a2341 = if v3 {
                8u64
            } else {
                3u64
            };
            if nat_eq(a2340, a2341) {
                Ok(Adt9::C7(Adt11::C2(0u64, v5, true), Adt12::C1(0u64), Adt12::C2(0u64), v11, 4u64, f179(v5)))
            } else {
                if v3 {
                    Ok(f180(v9, v7, v6, v12))
                } else {
                    Ok(Adt9::C7({
                        let a2342 = v7;
                        let a2343 = 2u64;
                        if nat_eq(a2342, a2343) {
                            Adt11::C5(3u64)
                        } else {
                            Adt11::C3(0u64, 0u64, false)
                        }
                    }, Adt12::C1(0u64), Adt12::C2(13u64), v12, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f76(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2344, _, _, _, _, _, _, _) => {
            h2344
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2345, _, _, _, _, _, _, _) => {
            h2345
        }
    };
    let v6: u64 = f181(v4)?;
    let v7: u64 = {
        let a2346 = v0;
        let a2347 = v6;
        let a2348 = 0u64;
        nat_quot(a2346, a2347, a2348)
    };
    let v8: u64 = {
        let a2349 = v7;
        let a2350 = v5;
        let a2351 = 0u64;
        nat_quot(a2349, a2350, a2351)
    };
    let v9: u64 = {
        let a2352 = v7;
        let a2353 = v5;
        let a2354 = 0u64;
        nat_rem(a2352, a2353, a2354)
    };
    let v10: u64 = {
        let a2355 = v3;
        let a2356 = v4;
        let a2357 = 0u64;
        nat_rem(a2355, a2356, a2357)
    };
    let v11: u64 = {
        let a2362 = 4u64;
        let a2363 = {
            let a2360 = v8;
            let a2361 = {
                let a2358 = v4;
                let a2359 = v4;
                nat_mul(a2358, a2359)?
            };
            nat_mul(a2360, a2361)?
        };
        nat_mul(a2362, a2363)?
    };
    let v12: u64 = {
        let a2368 = 4u64;
        let a2369 = {
            let a2366 = v8;
            let a2367 = {
                let a2364 = v4;
                let a2365 = v5;
                nat_mul(a2364, a2365)?
            };
            nat_mul(a2366, a2367)?
        };
        nat_mul(a2368, a2369)?
    };
    let v13: u64 = {
        let a2373 = {
            let a2370 = v0;
            let a2371 = v6;
            let a2372 = 0u64;
            nat_rem(a2370, a2371, a2372)
        };
        let a2374 = f181(v10)?;
        nat_sub(a2373, a2374)
    };
    let a2377 = v13;
    let a2378 = {
        let a2375 = v10;
        let a2376 = 2u64;
        nat_mul(a2375, a2376)?
    };
    if nat_lt(a2377, a2378) {
        let v14: u64 = {
            let a2379 = v13;
            let a2380 = 2u64;
            let a2381 = 0u64;
            nat_quot(a2379, a2380, a2381)
        };
        let a2385 = {
            let a2382 = v13;
            let a2383 = 2u64;
            let a2384 = 0u64;
            nat_rem(a2382, a2383, a2384)
        };
        let a2386 = 0u64;
        if nat_eq(a2385, a2386) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2393 = v11;
                let a2394 = {
                    let a2391 = 4u64;
                    let a2392 = {
                        let a2389 = {
                            let a2387 = v10;
                            let a2388 = v4;
                            nat_mul(a2387, a2388)?
                        };
                        let a2390 = v14;
                        nat_add(a2389, a2390)?
                    };
                    nat_mul(a2391, a2392)?
                };
                nat_add(a2393, a2394)?
            }), Adt12::C3({
                let a2401 = v12;
                let a2402 = {
                    let a2399 = 4u64;
                    let a2400 = {
                        let a2397 = {
                            let a2395 = v14;
                            let a2396 = v5;
                            nat_mul(a2395, a2396)?
                        };
                        let a2398 = v9;
                        nat_add(a2397, a2398)?
                    };
                    nat_mul(a2399, a2400)?
                };
                nat_add(a2401, a2402)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2403 = v14;
                let a2404 = 0u64;
                if nat_eq(a2403, a2404) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2407 = v13;
        let a2408 = {
            let a2405 = v10;
            let a2406 = 2u64;
            nat_mul(a2405, a2406)?
        };
        if nat_eq(a2407, a2408) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2415 = v12;
                let a2416 = {
                    let a2413 = 4u64;
                    let a2414 = {
                        let a2411 = {
                            let a2409 = v10;
                            let a2410 = v5;
                            nat_mul(a2409, a2410)?
                        };
                        let a2412 = v9;
                        nat_add(a2411, a2412)?
                    };
                    nat_mul(a2413, a2414)?
                };
                nat_add(a2415, a2416)?
            }), {
                let a2417 = v10;
                let a2418 = 0u64;
                if nat_eq(a2417, a2418) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2423 = v13;
            let a2424 = {
                let a2421 = {
                    let a2419 = v10;
                    let a2420 = 2u64;
                    nat_mul(a2419, a2420)?
                };
                let a2422 = 1u64;
                nat_add(a2421, a2422)?
            };
            if nat_eq(a2423, a2424) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2431 = v11;
                    let a2432 = {
                        let a2429 = 4u64;
                        let a2430 = {
                            let a2427 = {
                                let a2425 = v10;
                                let a2426 = v4;
                                nat_mul(a2425, a2426)?
                            };
                            let a2428 = v10;
                            nat_add(a2427, a2428)?
                        };
                        nat_mul(a2429, a2430)?
                    };
                    nat_add(a2431, a2432)?
                }), Adt13::C1({
                    let a2439 = v12;
                    let a2440 = {
                        let a2437 = 4u64;
                        let a2438 = {
                            let a2435 = {
                                let a2433 = v10;
                                let a2434 = v5;
                                nat_mul(a2433, a2434)?
                            };
                            let a2436 = v9;
                            nat_add(a2435, a2436)?
                        };
                        nat_mul(a2437, a2438)?
                    };
                    nat_add(a2439, a2440)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2443 = {
                        let a2441 = v10;
                        let a2442 = 1u64;
                        nat_add(a2441, a2442)?
                    };
                    let a2444 = v4;
                    if nat_eq(a2443, a2444) {
                        0u64
                    } else {
                        let a2445 = v10;
                        let a2446 = 1u64;
                        nat_add(a2445, a2446)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f77(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2447 = v0;
    let a2448 = v1;
    if nat_lt(a2447, a2448) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f94(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2449 = v0;
            let a2450 = v1;
            nat_sub(a2449, a2450)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f94(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2457 = {
            let a2455 = 3u64;
            let a2456 = {
                let a2453 = match v4.clone() {
                    Adt6::C0(h2451, _, _, _) => {
                        h2451
                    }
                };
                let a2454 = match v5.clone() {
                    Adt6::C0(h2452, _, _, _) => {
                        h2452
                    }
                };
                nat_mul(a2453, a2454)?
            };
            nat_mul(a2455, a2456)?
        };
        let a2458 = 2u64;
        nat_add(a2457, a2458)?
    };
    let v8: u64 = {
        let a2459 = v0;
        let a2460 = v7;
        let a2461 = 0u64;
        nat_quot(a2459, a2460, a2461)
    };
    let v9: u64 = {
        let a2462 = v0;
        let a2463 = v7;
        let a2464 = 0u64;
        nat_rem(a2462, a2463, a2464)
    };
    let a2465 = v9;
    let a2466 = 0u64;
    if nat_eq(a2465, a2466) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2469 = {
            let a2467 = v9;
            let a2468 = 1u64;
            nat_add(a2467, a2468)?
        };
        let a2470 = v7;
        if nat_eq(a2469, a2470) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f94(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2471 = v9;
                let a2472 = 1u64;
                nat_sub(a2471, a2472)
            };
            let v11: u64 = {
                let a2473 = v10;
                let a2474 = 3u64;
                let a2475 = 0u64;
                nat_quot(a2473, a2474, a2475)
            };
            let v12: u64 = {
                let a2477 = v11;
                let a2478 = match v4.clone() {
                    Adt6::C0(h2476, _, _, _) => {
                        h2476
                    }
                };
                let a2479 = 0u64;
                nat_rem(a2477, a2478, a2479)
            };
            let v13: u64 = {
                let a2481 = v11;
                let a2482 = match v4.clone() {
                    Adt6::C0(h2480, _, _, _) => {
                        h2480
                    }
                };
                let a2483 = 0u64;
                nat_quot(a2481, a2482, a2483)
            };
            let v14: u64 = {
                let a2484 = v10;
                let a2485 = 3u64;
                let a2486 = 0u64;
                nat_rem(a2484, a2485, a2486)
            };
            let v15: u64 = f58(v8, v3.clone(), 2u64);
            let m2488 = f60(v4, match v2.clone() {
                Adt3::C0(h2487, _, _, _, _, _, _, _) => {
                    h2487
                }
            }, f58(v8, v3.clone(), 0u64), v12)?;
            match m2488 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2490 = f60(v5, match v2.clone() {
                        Adt3::C0(_, h2489, _, _, _, _, _, _) => {
                            h2489
                        }
                    }, f58(v8, v3.clone(), 1u64), v13)?;
                    match m2490 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2491 = v14;
                            let a2492 = 0u64;
                            if nat_eq(a2491, a2492) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f57(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f179(v6), 4u64))
                            } else {
                                let a2493 = v14;
                                let a2494 = 1u64;
                                if nat_eq(a2493, a2494) {
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

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f182(match v1.clone() {
        Adt3::C0(h2495, _, _, _, _, _, _, _) => {
            h2495
        }
    })?;
    let v5: u64 = {
        let a2496 = v0;
        let a2497 = v4;
        let a2498 = 0u64;
        nat_quot(a2496, a2497, a2498)
    };
    let v6: u64 = {
        let a2499 = v0;
        let a2500 = v4;
        let a2501 = 0u64;
        nat_rem(a2499, a2500, a2501)
    };
    let a2502 = v6;
    let a2503 = 0u64;
    if nat_eq(a2502, a2503) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2504 = v6;
        let a2505 = 1u64;
        if nat_eq(a2504, a2505) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2508 = {
                let a2506 = v6;
                let a2507 = 1u64;
                nat_add(a2506, a2507)?
            };
            let a2509 = v4;
            if nat_eq(a2508, a2509) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f94(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2510 = v6;
                    let a2511 = 2u64;
                    nat_sub(a2510, a2511)
                };
                let v8: u64 = {
                    let a2512 = v7;
                    let a2513 = 3u64;
                    let a2514 = 0u64;
                    nat_quot(a2512, a2513, a2514)
                };
                let v9: u64 = {
                    let a2515 = v7;
                    let a2516 = 3u64;
                    let a2517 = 0u64;
                    nat_rem(a2515, a2516, a2517)
                };
                let v10: u64 = f94({
                    let a2521 = {
                        let a2519 = v5;
                        let a2520 = match v1.clone() {
                            Adt3::C0(h2518, _, _, _, _, _, _, _) => {
                                h2518
                            }
                        };
                        nat_mul(a2519, a2520)?
                    };
                    let a2522 = v8;
                    nat_add(a2521, a2522)?
                }, v1)?;
                let a2523 = v9;
                let a2524 = 0u64;
                if nat_eq(a2523, a2524) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2525 = v9;
                    let a2526 = 1u64;
                    if nat_eq(a2525, a2526) {
                        let a2527 = v3;
                        let a2528 = 2u64;
                        if nat_eq(a2527, a2528) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2529 = v3;
                        let a2530 = 2u64;
                        if if nat_eq(a2529, a2530) {
                            true
                        } else {
                            let a2531 = v3;
                            let a2532 = 3u64;
                            nat_eq(a2531, a2532)
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

pub fn f80(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2534 = 2u64;
        let a2535 = match v2.clone() {
            Adt3::C0(_, h2533, _, _, _, _, _, _) => {
                h2533
            }
        };
        nat_mul(a2534, a2535)?
    };
    let a2536 = v1;
    let a2537 = v6;
    if nat_lt(a2536, a2537) {
        let a2541 = {
            let a2538 = v1;
            let a2539 = 2u64;
            let a2540 = 0u64;
            nat_rem(a2538, a2539, a2540)
        };
        let a2542 = 0u64;
        if nat_eq(a2541, a2542) {
            Ok(Adt9::C4(19u64, 4u64, f94({
                let a2543 = v1;
                let a2544 = 2u64;
                let a2545 = 0u64;
                nat_quot(a2543, a2544, a2545)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2546 = v5;
            let a2547 = 2147483648u64;
            if if nat_lt(a2546, a2547) {
                let a2549 = v5;
                let a2550 = match v3 {
                    Adt3::C0(_, h2548, _, _, _, _, _, _) => {
                        h2548
                    }
                };
                nat_lt(a2549, a2550)
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
            let a2553 = match v3.clone() {
                Adt3::C0(h2551, _, _, _, _, _, _, _) => {
                    h2551
                }
            };
            let a2554 = match v3.clone() {
                Adt3::C0(_, h2552, _, _, _, _, _, _) => {
                    h2552
                }
            };
            nat_mul(a2553, a2554)?
        };
        let a2557 = {
            let a2555 = v1;
            let a2556 = v6;
            nat_sub(a2555, a2556)
        };
        let a2558 = v7;
        if nat_lt(a2557, a2558) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f94({
                let a2559 = v1;
                let a2560 = v6;
                nat_sub(a2559, a2560)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2563 = {
                    let a2561 = v1;
                    let a2562 = v6;
                    nat_sub(a2561, a2562)
                };
                let a2564 = v7;
                nat_sub(a2563, a2564)
            };
            let v9: u64 = {
                let a2566 = 1u64;
                let a2567 = f183(v0, match v2.clone() {
                    Adt3::C0(h2565, _, _, _, _, _, _, _) => {
                        h2565
                    }
                })?;
                nat_add(a2566, a2567)?
            };
            let v10: u64 = {
                let a2568 = v8;
                let a2569 = v9;
                let a2570 = 0u64;
                nat_quot(a2568, a2569, a2570)
            };
            let v11: u64 = {
                let a2571 = v8;
                let a2572 = v9;
                let a2573 = 0u64;
                nat_rem(a2571, a2572, a2573)
            };
            let a2574 = v11;
            let a2575 = 0u64;
            if nat_eq(a2574, a2575) {
                Ok(Adt9::C4(19u64, 4u64, f94(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f184(v0) {
                    let a2579 = {
                        let a2576 = v11;
                        let a2577 = 2u64;
                        let a2578 = 0u64;
                        nat_rem(a2576, a2577, a2578)
                    };
                    let a2580 = 1u64;
                    if nat_eq(a2579, a2580) {
                        Ok(Adt9::C4(6u64, 0u64, f94({
                            let a2589 = {
                                let a2582 = v10;
                                let a2583 = match v2.clone() {
                                    Adt3::C0(h2581, _, _, _, _, _, _, _) => {
                                        h2581
                                    }
                                };
                                nat_mul(a2582, a2583)?
                            };
                            let a2590 = {
                                let a2586 = {
                                    let a2584 = v11;
                                    let a2585 = 1u64;
                                    nat_sub(a2584, a2585)
                                };
                                let a2587 = 2u64;
                                let a2588 = 0u64;
                                nat_quot(a2586, a2587, a2588)
                            };
                            nat_add(a2589, a2590)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f94({
                            let a2599 = {
                                let a2592 = v5;
                                let a2593 = match v2.clone() {
                                    Adt3::C0(h2591, _, _, _, _, _, _, _) => {
                                        h2591
                                    }
                                };
                                nat_mul(a2592, a2593)?
                            };
                            let a2600 = {
                                let a2596 = {
                                    let a2594 = v11;
                                    let a2595 = 1u64;
                                    nat_sub(a2594, a2595)
                                };
                                let a2597 = 2u64;
                                let a2598 = 0u64;
                                nat_quot(a2596, a2597, a2598)
                            };
                            nat_add(a2599, a2600)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f94({
                            let a2609 = {
                                let a2602 = v5;
                                let a2603 = match v2 {
                                    Adt3::C0(h2601, _, _, _, _, _, _, _) => {
                                        h2601
                                    }
                                };
                                nat_mul(a2602, a2603)?
                            };
                            let a2610 = {
                                let a2606 = {
                                    let a2604 = v11;
                                    let a2605 = 1u64;
                                    nat_sub(a2604, a2605)
                                };
                                let a2607 = 2u64;
                                let a2608 = 0u64;
                                nat_quot(a2606, a2607, a2608)
                            };
                            nat_add(a2609, a2610)?
                        }, v3)?))
                    }
                } else {
                    f185(v0, v10, {
                        let a2611 = v11;
                        let a2612 = 1u64;
                        nat_sub(a2611, a2612)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f81(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2613 = v1;
    let a2614 = f186(v2.clone(), v4.clone())?;
    if nat_lt(a2613, a2614) {
        let a2617 = v1;
        let a2618 = {
            let a2615 = 2u64;
            let a2616 = f187(v4.clone())?;
            nat_mul(a2615, a2616)?
        };
        if nat_lt(a2617, a2618) {
            let a2622 = {
                let a2619 = v1;
                let a2620 = 2u64;
                let a2621 = 0u64;
                nat_rem(a2619, a2620, a2621)
            };
            let a2623 = 0u64;
            if nat_eq(a2622, a2623) {
                f188({
                    let a2624 = v1;
                    let a2625 = 2u64;
                    let a2626 = 0u64;
                    nat_quot(a2624, a2625, a2626)
                }, v4)
            } else {
                if f189(match v2 {
                    Adt3::C0(_, h2627, _, _, _, _, _, _) => {
                        h2627
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2630 = v1;
                let a2631 = {
                    let a2628 = 2u64;
                    let a2629 = f187(v4.clone())?;
                    nat_mul(a2628, a2629)?
                };
                nat_sub(a2630, a2631)
            };
            let v7: u64 = {
                let a2635 = v6;
                let a2636 = {
                    let a2633 = 1u64;
                    let a2634 = match v2.clone() {
                        Adt3::C0(h2632, _, _, _, _, _, _, _) => {
                            h2632
                        }
                    };
                    nat_add(a2633, a2634)?
                };
                let a2637 = 0u64;
                nat_quot(a2635, a2636, a2637)
            };
            let v8: u64 = {
                let a2641 = v6;
                let a2642 = {
                    let a2639 = 1u64;
                    let a2640 = match v2.clone() {
                        Adt3::C0(h2638, _, _, _, _, _, _, _) => {
                            h2638
                        }
                    };
                    nat_add(a2639, a2640)?
                };
                let a2643 = 0u64;
                nat_rem(a2641, a2642, a2643)
            };
            let a2644 = v8;
            let a2645 = 0u64;
            if nat_eq(a2644, a2645) {
                f188(v7, v4)
            } else {
                f190(v0, v7, {
                    let a2646 = v8;
                    let a2647 = 1u64;
                    nat_sub(a2646, a2647)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f82(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2650 = match v4.clone() {
            Adt3::C0(h2648, _, _, _, _, _, _, _) => {
                h2648
            }
        };
        let a2651 = match v4.clone() {
            Adt3::C0(_, h2649, _, _, _, _, _, _) => {
                h2649
            }
        };
        nat_mul(a2650, a2651)?
    };
    let v7: u64 = {
        let a2652 = 2u64;
        let a2653 = v6;
        nat_mul(a2652, a2653)?
    };
    let v8: u64 = f191(match v3.clone() {
        Adt3::C0(_, h2654, _, _, _, _, _, _) => {
            h2654
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2655, _, _, _, _, _) => {
            h2655
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2656, _, _, _, _) => {
            h2656
        }
    });
    let a2657 = v0;
    let a2658 = v7;
    if nat_lt(a2657, a2658) {
        let a2662 = {
            let a2659 = v0;
            let a2660 = 2u64;
            let a2661 = 0u64;
            nat_rem(a2659, a2660, a2661)
        };
        let a2663 = 0u64;
        if nat_eq(a2662, a2663) {
            Ok(Adt9::C4(19u64, 5u64, f94({
                let a2664 = v0;
                let a2665 = 2u64;
                let a2666 = 0u64;
                nat_quot(a2664, a2665, a2666)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f192(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2667 = v0;
            let a2668 = v7;
            nat_sub(a2667, a2668)
        };
        let v10: u64 = {
            let a2672 = v9;
            let a2673 = {
                let a2670 = 1u64;
                let a2671 = match v1.clone() {
                    Adt3::C0(h2669, _, _, _, _, _, _, _) => {
                        h2669
                    }
                };
                nat_add(a2670, a2671)?
            };
            let a2674 = 0u64;
            nat_quot(a2672, a2673, a2674)
        };
        let v11: u64 = {
            let a2678 = v9;
            let a2679 = {
                let a2676 = 1u64;
                let a2677 = match v1.clone() {
                    Adt3::C0(h2675, _, _, _, _, _, _, _) => {
                        h2675
                    }
                };
                nat_add(a2676, a2677)?
            };
            let a2680 = 0u64;
            nat_rem(a2678, a2679, a2680)
        };
        let a2681 = v11;
        let a2682 = 0u64;
        if nat_eq(a2681, a2682) {
            Ok(Adt9::C4(19u64, 5u64, f94({
                let a2683 = v10;
                let a2684 = v6;
                let a2685 = 0u64;
                nat_rem(a2683, a2684, a2685)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f192(v5, v8) {
                let v12: u64 = {
                    let a2691 = {
                        let a2687 = v10;
                        let a2688 = match v1.clone() {
                            Adt3::C0(h2686, _, _, _, _, _, _, _) => {
                                h2686
                            }
                        };
                        nat_mul(a2687, a2688)?
                    };
                    let a2692 = {
                        let a2689 = v11;
                        let a2690 = 1u64;
                        nat_sub(a2689, a2690)
                    };
                    nat_add(a2691, a2692)?
                };
                let a2693 = v5;
                let a2694 = 4294967295u64;
                if nat_eq(a2693, a2694) {
                    Ok(Adt9::C1(false, f94(v12, v1)?, f94(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f94(v12, v1)?, 4u64, f94({
                        let a2700 = {
                            let a2696 = v5;
                            let a2697 = match v3.clone() {
                                Adt3::C0(h2695, _, _, _, _, _, _, _) => {
                                    h2695
                                }
                            };
                            nat_mul(a2696, a2697)?
                        };
                        let a2701 = {
                            let a2698 = v11;
                            let a2699 = 1u64;
                            nat_sub(a2698, a2699)
                        };
                        nat_add(a2700, a2701)?
                    }, v3)?, 3u64, f94(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f193(v3.clone())?;
    let v6: u64 = {
        let a2702 = v0;
        let a2703 = v5;
        let a2704 = 0u64;
        nat_quot(a2702, a2703, a2704)
    };
    let v7: u64 = {
        let a2705 = v0;
        let a2706 = v5;
        let a2707 = 0u64;
        nat_rem(a2705, a2706, a2707)
    };
    let v8: u64 = f194(v3.clone())?;
    let a2708 = v7;
    let a2709 = 0u64;
    if nat_eq(a2708, a2709) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2710, _) => {
                h2710
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2714 = v5;
        let a2715 = {
            let a2712 = v7;
            let a2713 = if match v3.clone() {
                Adt7::C0(_, _, _, h2711, _) => {
                    h2711
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2712, a2713)?
        };
        if nat_le(a2714, a2715) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2716, _) => {
                    h2716
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f94(v6, v2)?))
            } else {
                let a2719 = {
                    let a2717 = v7;
                    let a2718 = 2u64;
                    nat_add(a2717, a2718)?
                };
                let a2720 = v5;
                if nat_eq(a2719, a2720) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2721, _, _) => {
                            h2721
                        }
                    } {
                        let a2728 = {
                            let m2724 = match v3.clone() {
                                Adt7::C0(h2723, _, _, _, _) => {
                                    h2723
                                }
                            };
                            match m2724 {
                                Adt6::C0(h2722, _, _, _) => {
                                    h2722
                                }
                            }
                        };
                        let a2729 = {
                            let m2727 = match v3 {
                                Adt7::C0(_, h2726, _, _, _) => {
                                    h2726
                                }
                            };
                            match m2727 {
                                Adt6::C0(h2725, _, _, _) => {
                                    h2725
                                }
                            }
                        };
                        nat_mul(a2728, a2729)?
                    } else {
                        f195(match v3 {
                            Adt7::C0(h2730, _, _, _, _) => {
                                h2730
                            }
                        }, match v1 {
                            Adt3::C0(h2731, _, _, _, _, _, _, _) => {
                                h2731
                            }
                        }, f58(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2732 = v9;
                        let a2733 = 0u64;
                        if nat_eq(a2732, a2733) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f94(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2736 = {
                    let a2734 = v7;
                    let a2735 = 1u64;
                    nat_sub(a2734, a2735)
                };
                let a2737 = v8;
                let a2738 = 0u64;
                nat_rem(a2736, a2737, a2738)
            };
            let m2744 = f176(v3.clone(), v1, v2, v6, {
                let a2741 = {
                    let a2739 = v7;
                    let a2740 = 1u64;
                    nat_sub(a2739, a2740)
                };
                let a2742 = v8;
                let a2743 = 0u64;
                nat_quot(a2741, a2742, a2743)
            })?;
            match m2744 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2746 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2745) => {
                            h2745
                        }
                    };
                    let a2747 = 1u64;
                    if if nat_eq(a2746, a2747) {
                        let a2748 = v10;
                        let a2749 = 0u64;
                        nat_eq(a2748, a2749)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2750, _) => {
                                h2750
                            }
                        } {
                            let a2753 = {
                                let a2751 = v10;
                                let a2752 = 1u64;
                                nat_add(a2751, a2752)?
                            };
                            let a2754 = v8;
                            if nat_eq(a2753, a2754) {
                                let a2755 = v4;
                                let a2756 = 2u64;
                                if nat_eq(a2755, a2756) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2758 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2757) => {
                                                h2757
                                            }
                                        };
                                        let a2759 = 1u64;
                                        if nat_eq(a2758, a2759) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2761 = match v3 {
                                            Adt7::C0(_, _, _, _, h2760) => {
                                                h2760
                                            }
                                        };
                                        let a2762 = 1u64;
                                        if nat_eq(a2761, a2762) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2764 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2763) => {
                                            h2763
                                        }
                                    };
                                    let a2765 = 1u64;
                                    if nat_eq(a2764, a2765) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2767 = match v3 {
                                        Adt7::C0(_, _, _, _, h2766) => {
                                            h2766
                                        }
                                    };
                                    let a2768 = 1u64;
                                    if nat_eq(a2767, a2768) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2770 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2769) => {
                                        h2769
                                    }
                                };
                                let a2771 = 1u64;
                                if nat_eq(a2770, a2771) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2773 = match v3 {
                                    Adt7::C0(_, _, _, _, h2772) => {
                                        h2772
                                    }
                                };
                                let a2774 = 1u64;
                                if nat_eq(a2773, a2774) {
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

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f196(match v3.clone() {
        Adt3::C0(h2775, _, _, _, _, _, _, _) => {
            h2775
        }
    })?;
    let v5: u64 = {
        let a2776 = v0;
        let a2777 = v4;
        let a2778 = 0u64;
        nat_rem(a2776, a2777, a2778)
    };
    let v6: u64 = {
        let a2779 = v0;
        let a2780 = v4;
        let a2781 = 0u64;
        nat_quot(a2779, a2780, a2781)
    };
    let a2782 = v5;
    let a2783 = 0u64;
    if nat_eq(a2782, a2783) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2786 = {
            let a2784 = v5;
            let a2785 = 1u64;
            nat_add(a2784, a2785)?
        };
        let a2787 = v4;
        if nat_eq(a2786, a2787) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f94(v6, v2)?))
        } else {
            let a2791 = {
                let a2788 = v5;
                let a2789 = 2u64;
                let a2790 = 0u64;
                nat_rem(a2788, a2789, a2790)
            };
            let a2792 = 0u64;
            if nat_eq(a2791, a2792) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2795 = {
                        let a2793 = v5;
                        let a2794 = 1u64;
                        nat_sub(a2793, a2794)
                    };
                    let a2796 = 2u64;
                    let a2797 = 0u64;
                    nat_quot(a2795, a2796, a2797)
                };
                let v8: u64 = f58(v6, v2.clone(), 0u64);
                let v9: u64 = f58(v6, v2.clone(), 1u64);
                let v10: u64 = f58(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f57(v1, {
                    let a2798 = v9;
                    let a2799 = v7;
                    nat_add(a2798, a2799)?
                }, v8, v10, 0u64)?, 4u64, f57(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f85(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f197(v3.clone())?;
    let v5: u64 = {
        let a2800 = v0;
        let a2801 = v4;
        let a2802 = 0u64;
        nat_rem(a2800, a2801, a2802)
    };
    let v6: u64 = {
        let a2803 = v0;
        let a2804 = v4;
        let a2805 = 0u64;
        nat_quot(a2803, a2804, a2805)
    };
    let a2806 = v5;
    let a2807 = 0u64;
    if nat_eq(a2806, a2807) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2810 = {
            let a2808 = v5;
            let a2809 = 1u64;
            nat_add(a2808, a2809)?
        };
        let a2811 = v4;
        if nat_eq(a2810, a2811) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f94(v6, v2)?))
        } else {
            let a2815 = {
                let a2812 = v5;
                let a2813 = 2u64;
                let a2814 = 0u64;
                nat_rem(a2812, a2813, a2814)
            };
            let a2816 = 0u64;
            if nat_eq(a2815, a2816) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2818 = f58(v6, v2.clone(), 0u64);
                    let a2819 = match v3.clone() {
                        Adt6::C0(_, _, h2817, _) => {
                            h2817
                        }
                    };
                    nat_add(a2818, a2819)?
                };
                let v8: u64 = {
                    let a2827 = {
                        let a2824 = {
                            let a2821 = v7;
                            let a2822 = match v3.clone() {
                                Adt6::C0(_, h2820, _, _) => {
                                    h2820
                                }
                            };
                            nat_add(a2821, a2822)?
                        };
                        let a2825 = match v3.clone() {
                            Adt6::C0(h2823, _, _, _) => {
                                h2823
                            }
                        };
                        nat_sub(a2824, a2825)
                    };
                    let a2828 = match v3.clone() {
                        Adt6::C0(_, h2826, _, _) => {
                            h2826
                        }
                    };
                    let a2829 = 0u64;
                    nat_quot(a2827, a2828, a2829)
                };
                let v9: u64 = {
                    let a2835 = v8;
                    let a2836 = {
                        let a2832 = {
                            let a2830 = v5;
                            let a2831 = 1u64;
                            nat_sub(a2830, a2831)
                        };
                        let a2833 = 2u64;
                        let a2834 = 0u64;
                        nat_quot(a2832, a2833, a2834)
                    };
                    nat_add(a2835, a2836)?
                };
                let v10: u64 = {
                    let a2838 = v9;
                    let a2839 = match v3.clone() {
                        Adt6::C0(_, h2837, _, _) => {
                            h2837
                        }
                    };
                    nat_mul(a2838, a2839)?
                };
                let v11: u64 = {
                    let a2840 = v7;
                    let a2841 = v10;
                    nat_sub(a2840, a2841)
                };
                let a2843 = v9;
                let a2844 = match v1.clone() {
                    Adt3::C0(_, h2842, _, _, _, _, _, _) => {
                        h2842
                    }
                };
                if if if nat_lt(a2843, a2844) {
                    let a2845 = v10;
                    let a2846 = v7;
                    nat_le(a2845, a2846)
                } else {
                    false
                } {
                    let a2848 = v11;
                    let a2849 = match v3.clone() {
                        Adt6::C0(h2847, _, _, _) => {
                            h2847
                        }
                    };
                    nat_lt(a2848, a2849)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f57(v1, {
                        let a2853 = {
                            let a2851 = f58(v6, v2, 1u64);
                            let a2852 = match v3 {
                                Adt6::C0(h2850, _, _, _) => {
                                    h2850
                                }
                            };
                            nat_mul(a2851, a2852)?
                        };
                        let a2854 = v11;
                        nat_add(a2853, a2854)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f86(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2856 = v5;
        let a2857 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2855, _) => {
                h2855
            }
        };
        let a2858 = 0u64;
        nat_rem(a2856, a2857, a2858)
    };
    let v7: u64 = {
        let a2860 = v5;
        let a2861 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2859, _) => {
                h2859
            }
        };
        let a2862 = 0u64;
        nat_quot(a2860, a2861, a2862)
    };
    let v8: Option<u64> = f60(match v3.clone() {
        Adt5::C0(h2863, _, _, _, _, _, _, _) => {
            h2863
        }
    }, match v1.clone() {
        Adt3::C0(h2864, _, _, _, _, _, _, _) => {
            h2864
        }
    }, f58(v0, v2.clone(), 1u64), {
        let a2868 = v4;
        let a2869 = {
            let m2867 = match v3.clone() {
                Adt5::C0(h2866, _, _, _, _, _, _, _) => {
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
    let v9: u64 = {
        let a2874 = v4;
        let a2875 = {
            let m2873 = match v3.clone() {
                Adt5::C0(h2872, _, _, _, _, _, _, _) => {
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
    let v10: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, h2877, _, _, _, _, _, _) => {
            h2877
        }
    }, match v1.clone() {
        Adt3::C0(_, h2878, _, _, _, _, _, _) => {
            h2878
        }
    }, f58(v0, v2.clone(), 2u64), {
        let a2882 = v9;
        let a2883 = {
            let m2881 = match v3.clone() {
                Adt5::C0(_, h2880, _, _, _, _, _, _) => {
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
    let v11: u64 = {
        let a2888 = v9;
        let a2889 = {
            let m2887 = match v3.clone() {
                Adt5::C0(_, h2886, _, _, _, _, _, _) => {
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
    let v12: Option<u64> = f60(match v3.clone() {
        Adt5::C0(_, _, h2891, _, _, _, _, _) => {
            h2891
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2892, _, _, _, _, _) => {
            h2892
        }
    }, v6, {
        let a2896 = v11;
        let a2897 = {
            let m2895 = match v3.clone() {
                Adt5::C0(_, _, h2894, _, _, _, _, _) => {
                    h2894
                }
            };
            match m2895 {
                Adt6::C0(h2893, _, _, _) => {
                    h2893
                }
            }
        };
        let a2898 = 0u64;
        nat_rem(a2896, a2897, a2898)
    })?;
    let v13: u64 = {
        let a2902 = v11;
        let a2903 = {
            let m2901 = match v3.clone() {
                Adt5::C0(_, _, h2900, _, _, _, _, _) => {
                    h2900
                }
            };
            match m2901 {
                Adt6::C0(h2899, _, _, _) => {
                    h2899
                }
            }
        };
        let a2904 = 0u64;
        nat_quot(a2902, a2903, a2904)
    };
    let v17: Adt12 = {
        let m2905 = v8;
        match m2905 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2906 = v10;
                match m2906 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2907 = v12;
                        match m2907 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2916 = {
                                        let a2911 = f57(v1, v14, v15, v16, 0u64)?;
                                        let a2912 = {
                                            let a2909 = v7;
                                            let a2910 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2908, _, _, _) => {
                                                    h2908
                                                }
                                            };
                                            nat_mul(a2909, a2910)?
                                        };
                                        nat_add(a2911, a2912)?
                                    };
                                    let a2917 = {
                                        let a2914 = v13;
                                        let a2915 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2913, _, _, _, _) => {
                                                h2913
                                            }
                                        };
                                        nat_mul(a2914, a2915)?
                                    };
                                    nat_add(a2916, a2917)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2918, _, _) => {
            h2918
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2919) => {
            h2919
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f94(v0, v2)?), {
        let a2921 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2920, _, _) => {
                h2920
            }
        };
        let a2922 = 0u64;
        if nat_eq(a2921, a2922) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2924 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2923) => {
                h2923
            }
        };
        let a2925 = 0u64;
        if nat_eq(a2924, a2925) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f87(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2938 = f198(v0, v1, match v2.clone() {
        Adt3::C0(h2926, _, _, _, _, _, _, _) => {
            h2926
        }
    }, match v2.clone() {
        Adt3::C0(_, h2927, _, _, _, _, _, _) => {
            h2927
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2928, _, _, _, _, _) => {
            h2928
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2929, _, _, _, _) => {
            h2929
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2930, _, _, _) => {
            h2930
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2931, _, _) => {
            h2931
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2932, _) => {
            h2932
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2933) => {
            h2933
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2934, _, _, _) => {
            h2934
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2935, _, _) => {
            h2935
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2936, _) => {
            h2936
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2937) => {
            h2937
        }
    }, v4)?;
    match m2938 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f20(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2941 = v6;
        let a2942 = {
            let a2939 = 3u64;
            let a2940 = v5;
            nat_add(a2939, a2940)?
        };
        nat_sub(a2941, a2942)
    };
    let a2944 = v0;
    let a2945 = f50(v3, match v1.clone() {
        Adt3::C0(h2943, _, _, _, _, _, _, _) => {
            h2943
        }
    }, v5, v8)?;
    if nat_lt(a2944, a2945) {
        let v9: u64 = f103(match v1.clone() {
            Adt3::C0(h2946, _, _, _, _, _, _, _) => {
                h2946
            }
        }, v5, v8)?;
        let v10: u64 = f104(v3, match v1.clone() {
            Adt3::C0(h2947, _, _, _, _, _, _, _) => {
                h2947
            }
        }, v5)?;
        let a2948 = v0;
        let a2949 = v9;
        if nat_lt(a2948, a2949) {
            f199(v0, v2, f44(v3), {
                let a2950 = 3u64;
                let a2951 = v5;
                nat_add(a2950, a2951)?
            }, v7)
        } else {
            let a2954 = v0;
            let a2955 = {
                let a2952 = v9;
                let a2953 = v10;
                nat_add(a2952, a2953)?
            };
            if nat_lt(a2954, a2955) {
                let v11: u64 = {
                    let a2956 = v0;
                    let a2957 = v9;
                    nat_sub(a2956, a2957)
                };
                let v12: u64 = f200(v3, match v1.clone() {
                    Adt3::C0(h2958, _, _, _, _, _, _, _) => {
                        h2958
                    }
                })?;
                f201({
                    let a2959 = v11;
                    let a2960 = v12;
                    let a2961 = 0u64;
                    nat_rem(a2959, a2960, a2961)
                }, {
                    let a2962 = v11;
                    let a2963 = v12;
                    let a2964 = 0u64;
                    nat_quot(a2962, a2963, a2964)
                }, v1, v3, v4, v7)
            } else {
                f202({
                    let a2967 = v0;
                    let a2968 = {
                        let a2965 = v9;
                        let a2966 = v10;
                        nat_add(a2965, a2966)?
                    };
                    nat_sub(a2967, a2968)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2969 = v0;
    let a2970 = f36(v1)?;
    if nat_lt(a2969, a2970) {
        let a2971 = v0;
        let a2972 = 4u64;
        if nat_lt(a2971, a2972) {
            let a2973 = v0;
            let a2974 = 0u64;
            if nat_eq(a2973, a2974) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2975 = v0;
                let a2976 = 1u64;
                if nat_eq(a2975, a2976) {
                    let a2977 = v5;
                    let a2978 = 0u64;
                    if nat_eq(a2977, a2978) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2979 = v0;
                    let a2980 = 2u64;
                    if nat_eq(a2979, a2980) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2981 = v0;
            let a2982 = 4u64;
            if if nat_eq(a2981, a2982) {
                let a2983 = f203(v5)?;
                let a2984 = v1;
                !nat_eq(a2983, a2984)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2987 = {
                        let a2985 = v0;
                        let a2986 = 4u64;
                        nat_sub(a2985, a2986)
                    };
                    let a2988 = 3u64;
                    let a2989 = 0u64;
                    nat_quot(a2987, a2988, a2989)
                };
                let v7: u64 = {
                    let a2992 = {
                        let a2990 = v0;
                        let a2991 = 4u64;
                        nat_sub(a2990, a2991)
                    };
                    let a2993 = 3u64;
                    let a2994 = 0u64;
                    nat_rem(a2992, a2993, a2994)
                };
                let a2995 = v7;
                let a2996 = 0u64;
                if nat_eq(a2995, a2996) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2997 = v7;
                    let a2998 = 1u64;
                    if nat_eq(a2997, a2998) {
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

pub fn f90(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2999 = v1;
        let a3000 = 5u64;
        nat_eq(a2999, a3000)
    } else {
        false
    } {
        let a3001 = v2;
        let a3002 = 16u64;
        nat_eq(a3001, a3002)
    } else {
        false
    } {
        let a3003 = v3;
        let a3004 = 0u64;
        nat_eq(a3003, a3004)
    } else {
        false
    } {
        let a3005 = v4;
        let a3006 = 0u64;
        nat_eq(a3005, a3006)
    } else {
        false
    }
}

pub fn f91(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3007 = v1;
    let a3008 = 0u64;
    if nat_eq(a3007, a3008) {
        if v0 {
            let a3009 = v3;
            let a3010 = 0u64;
            if if if nat_eq(a3009, a3010) {
                let a3011 = v4;
                let a3012 = 43u64;
                nat_eq(a3011, a3012)
            } else {
                false
            } {
                true
            } else {
                let a3013 = v3;
                let a3014 = 43u64;
                if nat_eq(a3013, a3014) {
                    let a3015 = v4;
                    let a3016 = 0u64;
                    nat_eq(a3015, a3016)
                } else {
                    false
                }
            } {
                f170(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3017 = v1;
        let a3018 = 1u64;
        if nat_eq(a3017, a3018) {
            Adt11::C1(v2)
        } else {
            let a3019 = v1;
            let a3020 = 2u64;
            if nat_eq(a3019, a3020) {
                Adt11::C5(v2)
            } else {
                let a3021 = v1;
                let a3022 = 3u64;
                if nat_eq(a3021, a3022) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3023 = v1;
                    let a3024 = 4u64;
                    if nat_eq(a3023, a3024) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f90(v0, v1, v2, v3, v4) {
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

pub fn f92(v0: u64, v1: u64) -> Adt12 {
    let a3025 = v0;
    let a3026 = 0u64;
    if nat_eq(a3025, a3026) {
        Adt12::C0(0u64, v1)
    } else {
        let a3027 = v0;
        let a3028 = 1u64;
        if nat_eq(a3027, a3028) {
            Adt12::C1(v1)
        } else {
            let a3029 = v0;
            let a3030 = 2u64;
            if nat_eq(a3029, a3030) {
                Adt12::C2(v1)
            } else {
                let a3031 = v0;
                let a3032 = 3u64;
                if nat_eq(a3031, a3032) {
                    Adt12::C3(v1)
                } else {
                    let a3033 = v0;
                    let a3034 = 13u64;
                    if nat_lt(a3033, a3034) {
                        Adt12::C0({
                            let a3035 = v0;
                            let a3036 = 3u64;
                            nat_sub(a3035, a3036)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f93(v0: u64, v1: u64) -> Adt13 {
    let a3037 = v0;
    let a3038 = 1u64;
    if nat_eq(a3037, a3038) {
        Adt13::C0(v1)
    } else {
        let a3039 = v0;
        let a3040 = 3u64;
        if if nat_eq(a3039, a3040) {
            true
        } else {
            let a3041 = v0;
            let a3042 = 6u64;
            nat_eq(a3041, a3042)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f94(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a3044 = v0;
        let a3045 = match v1.clone() {
            Adt3::C0(h3043, _, _, _, _, _, _, _) => {
                h3043
            }
        };
        let a3046 = 0u64;
        nat_quot(a3044, a3045, a3046)
    };
    let v3: u64 = {
        let a3048 = v2;
        let a3049 = match v1.clone() {
            Adt3::C0(_, h3047, _, _, _, _, _, _) => {
                h3047
            }
        };
        let a3050 = 0u64;
        nat_quot(a3048, a3049, a3050)
    };
    f145({
        let a3052 = v0;
        let a3053 = match v1.clone() {
            Adt3::C0(h3051, _, _, _, _, _, _, _) => {
                h3051
            }
        };
        let a3054 = 0u64;
        nat_rem(a3052, a3053, a3054)
    }, {
        let a3056 = v2;
        let a3057 = match v1.clone() {
            Adt3::C0(_, h3055, _, _, _, _, _, _) => {
                h3055
            }
        };
        let a3058 = 0u64;
        nat_rem(a3056, a3057, a3058)
    }, {
        let a3060 = v3;
        let a3061 = match v1.clone() {
            Adt3::C0(_, _, h3059, _, _, _, _, _) => {
                h3059
            }
        };
        let a3062 = 0u64;
        nat_rem(a3060, a3061, a3062)
    }, {
        let a3064 = v3;
        let a3065 = match v1.clone() {
            Adt3::C0(_, _, h3063, _, _, _, _, _) => {
                h3063
            }
        };
        let a3066 = 0u64;
        nat_quot(a3064, a3065, a3066)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3067, _, _, _) => {
            h3067
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3068, _, _) => {
            h3068
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3069, _) => {
            h3069
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3070) => {
            h3070
        }
    })
}

pub fn f95(v0: Adt6, v1: u64) -> R<u64> {
    let a3072 = 0u64;
    let a3073 = match v0.clone() {
        Adt6::C0(h3071, _, _, _) => {
            h3071
        }
    };
    if if if if if nat_lt(a3072, a3073) {
        let a3075 = 0u64;
        let a3076 = match v0.clone() {
            Adt6::C0(_, h3074, _, _) => {
                h3074
            }
        };
        nat_lt(a3075, a3076)
    } else {
        false
    } {
        let a3078 = 0u64;
        let a3079 = match v0.clone() {
            Adt6::C0(_, _, _, h3077) => {
                h3077
            }
        };
        nat_lt(a3078, a3079)
    } else {
        false
    } {
        let a3081 = match v0.clone() {
            Adt6::C0(_, h3080, _, _) => {
                h3080
            }
        };
        let a3082 = 2147483648u64;
        nat_lt(a3081, a3082)
    } else {
        false
    } {
        let a3084 = match v0.clone() {
            Adt6::C0(_, _, _, h3083) => {
                h3083
            }
        };
        let a3085 = 2147483648u64;
        nat_lt(a3084, a3085)
    } else {
        false
    } {
        let v2: u64 = {
            let a3089 = v1;
            let a3090 = {
                let a3087 = 2u64;
                let a3088 = f147(match v0.clone() {
                    Adt6::C0(_, _, h3086, _) => {
                        h3086
                    }
                });
                nat_mul(a3087, a3088)?
            };
            nat_add(a3089, a3090)?
        };
        let v3: u64 = {
            let a3102 = {
                let a3097 = 1u64;
                let a3098 = {
                    let a3095 = match v0.clone() {
                        Adt6::C0(_, _, _, h3091) => {
                            h3091
                        }
                    };
                    let a3096 = {
                        let a3093 = match v0.clone() {
                            Adt6::C0(h3092, _, _, _) => {
                                h3092
                            }
                        };
                        let a3094 = 1u64;
                        nat_sub(a3093, a3094)
                    };
                    nat_mul(a3095, a3096)?
                };
                nat_add(a3097, a3098)?
            };
            let a3103 = {
                let a3100 = 2u64;
                let a3101 = f146(match v0.clone() {
                    Adt6::C0(_, _, h3099, _) => {
                        h3099
                    }
                });
                nat_mul(a3100, a3101)?
            };
            nat_add(a3102, a3103)?
        };
        let a3104 = v3;
        let a3105 = v2;
        if nat_le(a3104, a3105) {
            let a3112 = 1u64;
            let a3113 = {
                let a3109 = {
                    let a3106 = v2;
                    let a3107 = v3;
                    nat_sub(a3106, a3107)
                };
                let a3110 = match v0 {
                    Adt6::C0(_, h3108, _, _) => {
                        h3108
                    }
                };
                let a3111 = 0u64;
                nat_quot(a3109, a3110, a3111)
            };
            nat_add(a3112, a3113)
        } else {
            let a3117 = {
                let a3114 = v3;
                let a3115 = v2;
                nat_sub(a3114, a3115)
            };
            let a3118 = match v0 {
                Adt6::C0(_, h3116, _, _) => {
                    h3116
                }
            };
            if nat_lt(a3117, a3118) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f96(v0: u64) -> u64 {
    let v1: u64 = {
        let a3119 = v0;
        let a3120 = 4294967296u64;
        let a3121 = 0u64;
        nat_rem(a3119, a3120, a3121)
    };
    let a3122 = v1;
    let a3123 = 2147483648u64;
    if nat_lt(a3122, a3123) {
        v1
    } else {
        0u64
    }
}

pub fn f97(v0: u64) -> R<u64> {
    let a3126 = 2u64;
    let a3127 = {
        let a3124 = 2u64;
        let a3125 = v0;
        nat_mul(a3124, a3125)?
    };
    nat_add(a3126, a3127)
}

pub fn f98(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3129 = match v1.clone() {
        Adt14::C0(h3128, _, _, _, _, _, _, _, _, _, _) => {
            h3128
        }
    };
    let a3130 = 0u64;
    if if if if nat_eq(a3129, a3130) {
        let a3132 = match v2.clone() {
            Adt14::C0(h3131, _, _, _, _, _, _, _, _, _, _) => {
                h3131
            }
        };
        let a3133 = 0u64;
        nat_eq(a3132, a3133)
    } else {
        false
    } {
        f204(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f205(match v1.clone() {
            Adt14::C0(_, h3134, _, _, _, _, _, _, _, _, _) => {
                h3134
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3135, _, _, _, _, _, _, _, _) => {
                h3135
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3136, _, _, _, _, _, _, _) => {
                h3136
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3137, _, _, _, _, _, _) => {
                h3137
            }
        }) {
            true
        } else {
            if if if if f45(v0) {
                f206(v1.clone(), v2.clone())
            } else {
                false
            } {
                f207(v1.clone())
            } else {
                false
            } {
                f207(v2.clone())
            } else {
                false
            } {
                if f208(v1.clone(), v2.clone())? {
                    true
                } else {
                    f209(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f205(match v1.clone() {
            Adt14::C0(_, h3138, _, _, _, _, _, _, _, _, _) => {
                h3138
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3139, _, _, _, _, _, _, _, _) => {
                h3139
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3140, _, _, _, _, _, _, _) => {
                h3140
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3141, _, _, _, _, _, _) => {
                h3141
            }
        }) {
            0u64
        } else {
            let a3147 = match v1.clone() {
                Adt14::C0(_, _, h3142, _, _, _, _, _, _, _, _) => {
                    h3142
                }
            };
            let a3148 = {
                let a3145 = match v1.clone() {
                    Adt14::C0(_, _, _, h3143, _, _, _, _, _, _, _) => {
                        h3143
                    }
                };
                let a3146 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3144, _, _, _, _, _, _) => {
                        h3144
                    }
                };
                nat_mul(a3145, a3146)?
            };
            nat_mul(a3147, a3148)?
        };
        let v4: u64 = {
            let a3149 = v3;
            let a3150 = 0u64;
            if if nat_eq(a3149, a3150) {
                true
            } else {
                f210(v2.clone())?
            } {
                0u64
            } else {
                let a3152 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3151, _) => {
                        h3151
                    }
                };
                let a3153 = 4u64;
                let a3154 = 0u64;
                nat_quot(a3152, a3153, a3154)
            }
        };
        Ok(Adt15::C1(f211(f212(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3155, _, _, _, _, _) => {
                h3155
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3156, _, _, _, _, _) => {
                h3156
            }
        }), match v1 {
            Adt14::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                h3157
            }
        }, v3, v4)?, 4u64, 64u64, v3, f213(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f99(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f214(v3.clone());
    let a3159 = match v2.clone() {
        Adt14::C0(h3158, _, _, _, _, _, _, _, _, _, _) => {
            h3158
        }
    };
    let a3160 = 0u64;
    if if if if nat_eq(a3159, a3160) {
        let a3162 = match v3.clone() {
            Adt14::C0(h3161, _, _, _, _, _, _, _, _, _, _) => {
                h3161
            }
        };
        let a3163 = 0u64;
        nat_eq(a3162, a3163)
    } else {
        false
    } {
        f204(v2.clone(), v3)
    } else {
        false
    } {
        if f205(match v2.clone() {
            Adt14::C0(_, h3164, _, _, _, _, _, _, _, _, _) => {
                h3164
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3165, _, _, _, _, _, _, _, _) => {
                h3165
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3166, _, _, _, _, _, _, _) => {
                h3166
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3167, _, _, _, _, _, _) => {
                h3167
            }
        }) {
            true
        } else {
            let a3169 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3168, _, _, _, _, _) => {
                    h3168
                }
            };
            let a3170 = 4u64;
            if if if if if nat_eq(a3169, a3170) {
                f215(match v2.clone() {
                    Adt14::C0(_, h3171, _, _, _, _, _, _, _, _, _) => {
                        h3171
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3172, _, _, _, _, _, _, _, _) => {
                        h3172
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3173, _, _, _, _, _, _, _) => {
                        h3173
                    }
                }, v0)?
            } else {
                false
            } {
                f207(v2.clone())
            } else {
                false
            } {
                f207(v4.clone())
            } else {
                false
            } {
                let a3175 = f216(v4.clone())?;
                let a3176 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3174, _) => {
                        h3174
                    }
                };
                nat_le(a3175, a3176)
            } else {
                false
            } {
                if f208(v2.clone(), v4.clone())? {
                    true
                } else {
                    f209(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f205(match v2.clone() {
            Adt14::C0(_, h3177, _, _, _, _, _, _, _, _, _) => {
                h3177
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3178, _, _, _, _, _, _, _, _) => {
                h3178
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3179, _, _, _, _, _, _, _) => {
                h3179
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3180, _, _, _, _, _, _) => {
                h3180
            }
        }) {
            true
        } else {
            f210(v4.clone())?
        } {
            0u64
        } else {
            let a3182 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3181, _) => {
                    h3181
                }
            };
            let a3183 = 4u64;
            let a3184 = 0u64;
            nat_quot(a3182, a3183, a3184)
        };
        Ok(Adt15::C1(f217(match v2.clone() {
            Adt14::C0(_, h3185, _, _, _, _, _, _, _, _, _) => {
                h3185
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3186, _, _, _, _, _, _, _, _) => {
                h3186
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3187, _, _, _, _, _, _, _) => {
                h3187
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3188, _, _, _, _, _, _) => {
                h3188
            }
        }, v0, v5)?, 4u64, 65u64, v0, f213(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f100(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f214(v4.clone());
    if if if if if if if v1 {
        let a3190 = match v2.clone() {
            Adt14::C0(h3189, _, _, _, _, _, _, _, _, _, _) => {
                h3189
            }
        };
        let a3191 = 0u64;
        nat_eq(a3190, a3191)
    } else {
        false
    } {
        let a3193 = match v3.clone() {
            Adt14::C0(h3192, _, _, _, _, _, _, _, _, _, _) => {
                h3192
            }
        };
        let a3194 = 0u64;
        nat_eq(a3193, a3194)
    } else {
        false
    } {
        let a3196 = match v4.clone() {
            Adt14::C0(h3195, _, _, _, _, _, _, _, _, _, _) => {
                h3195
            }
        };
        let a3197 = 0u64;
        nat_eq(a3196, a3197)
    } else {
        false
    } {
        f204(v2.clone(), v3.clone())
    } else {
        false
    } {
        f204(v2.clone(), v4)
    } else {
        false
    } {
        if f205(match v2.clone() {
            Adt14::C0(_, h3198, _, _, _, _, _, _, _, _, _) => {
                h3198
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3199, _, _, _, _, _, _, _, _) => {
                h3199
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3200, _, _, _, _, _, _, _) => {
                h3200
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3201, _, _, _, _, _, _) => {
                h3201
            }
        }) {
            true
        } else {
            let a3203 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3202, _, _, _, _, _) => {
                    h3202
                }
            };
            let a3204 = 4u64;
            if if if if if if if nat_eq(a3203, a3204) {
                let a3206 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3205, _, _, _, _, _) => {
                        h3205
                    }
                };
                let a3207 = 4u64;
                nat_eq(a3206, a3207)
            } else {
                false
            } {
                f207(v2.clone())
            } else {
                false
            } {
                f207(v3.clone())
            } else {
                false
            } {
                f207(v5.clone())
            } else {
                false
            } {
                let a3209 = f216(v5.clone())?;
                let a3210 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3208, _) => {
                        h3208
                    }
                };
                nat_le(a3209, a3210)
            } else {
                false
            } {
                if f208(v2.clone(), v5.clone())? {
                    true
                } else {
                    f209(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f208(v3.clone(), v5.clone())? {
                    true
                } else {
                    f209(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f205(match v2.clone() {
            Adt14::C0(_, h3211, _, _, _, _, _, _, _, _, _) => {
                h3211
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3212, _, _, _, _, _, _, _, _) => {
                h3212
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3213, _, _, _, _, _, _, _) => {
                h3213
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3214, _, _, _, _, _, _) => {
                h3214
            }
        }) {
            0u64
        } else {
            let a3220 = match v2.clone() {
                Adt14::C0(_, _, h3215, _, _, _, _, _, _, _, _) => {
                    h3215
                }
            };
            let a3221 = {
                let a3218 = match v2.clone() {
                    Adt14::C0(_, _, _, h3216, _, _, _, _, _, _, _) => {
                        h3216
                    }
                };
                let a3219 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3217, _, _, _, _, _, _) => {
                        h3217
                    }
                };
                nat_mul(a3218, a3219)?
            };
            nat_mul(a3220, a3221)?
        };
        let v7: u64 = {
            let a3222 = v6;
            let a3223 = 0u64;
            if if nat_eq(a3222, a3223) {
                true
            } else {
                f210(v5.clone())?
            } {
                0u64
            } else {
                let a3225 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3224, _) => {
                        h3224
                    }
                };
                let a3226 = 4u64;
                let a3227 = 0u64;
                nat_quot(a3225, a3226, a3227)
            }
        };
        Ok(Adt15::C1(f218(match v2 {
            Adt14::C0(_, h3228, _, _, _, _, _, _, _, _, _) => {
                h3228
            }
        }, v6, v7)?, 4u64, 66u64, v6, f213(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f101(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f219(v0, v3, v6) {
        f219(v1, v4, v6)
    } else {
        false
    } {
        f219(v2, v5, v6)
    } else {
        false
    }
}

pub fn f102(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f220(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f220(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f220(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f220(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f220(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f220(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f103(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3229 = v2;
    let a3230 = 0u64;
    if nat_eq(a3229, a3230) {
        Ok(0u64)
    } else {
        let a3233 = 2u64;
        let a3234 = {
            let a3231 = v0;
            let a3232 = v1;
            nat_mul(a3231, a3232)?
        };
        nat_mul(a3233, a3234)
    }
}

pub fn f104(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3235 = v2;
    let a3236 = f200(v0, v1)?;
    nat_mul(a3235, a3236)
}

pub fn f105(v0: u64) -> u64 {
    let a3237 = v0;
    let a3238 = 30u64;
    if nat_eq(a3237, a3238) {
        3u64
    } else {
        1u64
    }
}

pub fn f106(v0: u64) -> R<u64> {
    let a3241 = 5u64;
    let a3242 = {
        let a3239 = 4u64;
        let a3240 = v0;
        nat_mul(a3239, a3240)?
    };
    nat_add(a3241, a3242)
}

pub fn f107(v0: u64) -> bool {
    let m3243 = f221(v0);
    match m3243 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3245 = 1u64;
            let a3246 = match v1 {
                Adt19::C0(_, h3244, _) => {
                    h3244
                }
            };
            nat_lt(a3245, a3246)
        }
    }
}

pub fn f108(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3247, _, _, _, _, _, _, _, _, _, _, _) => {
            h3247
        }
    };
    let v4: u64 = match v0 {
        Adt1::C0(_, h3248, _, _, _, _, _, _, _, _, _, _) => {
            h3248
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, h3249, _, _, _, _, _, _, _, _, _) => {
            h3249
        }
    };
    let a3251 = match v1.clone() {
        Adt2::C0(h3250, _, _, _, _, _, _, _, _, _, _) => {
            h3250
        }
    };
    let a3252 = 0u64;
    if if if if if if if if if if if if if nat_eq(a3251, a3252) {
        let a3254 = match v2.clone() {
            Adt2::C0(h3253, _, _, _, _, _, _, _, _, _, _) => {
                h3253
            }
        };
        let a3255 = 0u64;
        nat_eq(a3254, a3255)
    } else {
        false
    } {
        let a3257 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h3256, _, _, _, _, _) => {
                h3256
            }
        };
        let a3258 = 4u64;
        nat_eq(a3257, a3258)
    } else {
        false
    } {
        let a3260 = match v2.clone() {
            Adt2::C0(_, h3259, _, _, _, _, _, _, _, _, _) => {
                h3259
            }
        };
        let a3261 = v3;
        nat_eq(a3260, a3261)
    } else {
        false
    } {
        let a3263 = match v2.clone() {
            Adt2::C0(_, _, h3262, _, _, _, _, _, _, _, _) => {
                h3262
            }
        };
        let a3264 = v5;
        nat_eq(a3263, a3264)
    } else {
        false
    } {
        let a3266 = match v2.clone() {
            Adt2::C0(_, _, _, h3265, _, _, _, _, _, _, _) => {
                h3265
            }
        };
        let a3267 = 1u64;
        nat_eq(a3266, a3267)
    } else {
        false
    } {
        let a3269 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3268, _, _, _, _, _, _) => {
                h3268
            }
        };
        let a3270 = 1u64;
        nat_eq(a3269, a3270)
    } else {
        false
    } {
        let a3271 = v3;
        let a3272 = 2147483648u64;
        nat_lt(a3271, a3272)
    } else {
        false
    } {
        let a3273 = v4;
        let a3274 = 4294967296u64;
        nat_lt(a3273, a3274)
    } else {
        false
    } {
        f38(v2.clone())?
    } else {
        false
    } {
        let a3278 = {
            let a3275 = v5;
            let a3276 = 4u64;
            nat_mul(a3275, a3276)?
        };
        let a3279 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3277, _) => {
                h3277
            }
        };
        nat_le(a3278, a3279)
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
            let a3280 = v5;
            let a3281 = 0u64;
            if nat_eq(a3280, a3281) {
                0u64
            } else {
                f222(v3, v5)?
            }
        }, 4u64, f25(v1, 1u64), f25(v2, 1u64), Adt4::C44(v4)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f109(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f21(v2.clone(), v3.clone()) {
        let a3283 = match v3.clone() {
            Adt2::C0(h3282, _, _, _, _, _, _, _, _, _, _) => {
                h3282
            }
        };
        let a3284 = 0u64;
        nat_eq(a3283, a3284)
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
        let a3286 = match v2.clone() {
            Adt2::C0(h3285, _, _, _, _, _, _, _, _, _, _) => {
                h3285
            }
        };
        let a3287 = 1u64;
        nat_le(a3286, a3287)
    } else {
        false
    } {
        let a3289 = match v0.clone() {
            Adt1::C0(h3288, _, _, _, _, _, _, _, _, _, _, _) => {
                h3288
            }
        };
        let a3290 = 0u64;
        if nat_eq(a3289, a3290) {
            let a3292 = match v2.clone() {
                Adt2::C0(h3291, _, _, _, _, _, _, _, _, _, _) => {
                    h3291
                }
            };
            let a3293 = 0u64;
            if if nat_eq(a3292, a3293) {
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
        f118(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f110(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3295 = match v1.clone() {
        Adt2::C0(h3294, _, _, _, _, _, _, _, _, _, _) => {
            h3294
        }
    };
    let a3296 = 0u64;
    if if if if if if if nat_eq(a3295, a3296) {
        let a3298 = match v3.clone() {
            Adt2::C0(h3297, _, _, _, _, _, _, _, _, _, _) => {
                h3297
            }
        };
        let a3299 = 0u64;
        nat_eq(a3298, a3299)
    } else {
        false
    } {
        let a3301 = match v2.clone() {
            Adt2::C0(h3300, _, _, _, _, _, _, _, _, _, _) => {
                h3300
            }
        };
        let a3302 = 1u64;
        nat_le(a3301, a3302)
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
        let a3304 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3303, _, _, _, _, _) => {
                h3303
            }
        };
        let a3305 = 0u64;
        if nat_eq(a3304, a3305) {
            true
        } else {
            let a3307 = match v3.clone() {
                Adt2::C0(_, _, h3306, _, _, _, _, _, _, _, _) => {
                    h3306
                }
            };
            let a3308 = 1u64;
            if nat_le(a3307, a3308) {
                true
            } else {
                let a3313 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3309, _, _, _, _) => {
                        h3309
                    }
                };
                let a3314 = {
                    let a3311 = 4u64;
                    let a3312 = match v3.clone() {
                        Adt2::C0(_, h3310, _, _, _, _, _, _, _, _, _) => {
                            h3310
                        }
                    };
                    nat_mul(a3311, a3312)?
                };
                nat_eq(a3313, a3314)
            }
        }
    } else {
        false
    } {
        f119(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f111(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3316 = match v0.clone() {
        Adt2::C0(h3315, _, _, _, _, _, _, _, _, _, _) => {
            h3315
        }
    };
    let a3317 = 0u64;
    if if if if if if if if if if if if nat_eq(a3316, a3317) {
        f38(v0.clone())?
    } else {
        false
    } {
        f26(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3319 = match v1.clone() {
            Adt2::C0(h3318, _, _, _, _, _, _, _, _, _, _) => {
                h3318
            }
        };
        let a3320 = 0u64;
        if if nat_eq(a3319, a3320) {
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
        let a3322 = match v2.clone() {
            Adt2::C0(h3321, _, _, _, _, _, _, _, _, _, _) => {
                h3321
            }
        };
        let a3323 = 0u64;
        if if nat_eq(a3322, a3323) {
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
        let a3326 = match v0.clone() {
            Adt2::C0(_, h3324, _, _, _, _, _, _, _, _, _) => {
                h3324
            }
        };
        let a3327 = match v0.clone() {
            Adt2::C0(_, _, h3325, _, _, _, _, _, _, _, _) => {
                h3325
            }
        };
        nat_eq(a3326, a3327)
    } else {
        false
    } {
        let a3330 = match v0.clone() {
            Adt2::C0(_, _, h3328, _, _, _, _, _, _, _, _) => {
                h3328
            }
        };
        let a3331 = match v1.clone() {
            Adt2::C0(_, _, h3329, _, _, _, _, _, _, _, _) => {
                h3329
            }
        };
        nat_eq(a3330, a3331)
    } else {
        false
    } {
        let a3334 = match v0.clone() {
            Adt2::C0(_, _, _, h3332, _, _, _, _, _, _, _) => {
                h3332
            }
        };
        let a3335 = match v1.clone() {
            Adt2::C0(_, _, _, h3333, _, _, _, _, _, _, _) => {
                h3333
            }
        };
        nat_eq(a3334, a3335)
    } else {
        false
    } {
        let a3338 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3336, _, _, _, _, _, _) => {
                h3336
            }
        };
        let a3339 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3337, _, _, _, _, _, _) => {
                h3337
            }
        };
        nat_eq(a3338, a3339)
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
            let a3348 = {
                let a3345 = {
                    let a3342 = match v1.clone() {
                        Adt2::C0(_, _, _, h3340, _, _, _, _, _, _, _) => {
                            h3340
                        }
                    };
                    let a3343 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3341, _, _, _, _, _, _) => {
                            h3341
                        }
                    };
                    nat_mul(a3342, a3343)?
                };
                let a3346 = match v1 {
                    Adt2::C0(_, h3344, _, _, _, _, _, _, _, _, _) => {
                        h3344
                    }
                };
                nat_mul(a3345, a3346)?
            };
            let a3349 = f181(match v0.clone() {
                Adt2::C0(_, h3347, _, _, _, _, _, _, _, _, _) => {
                    h3347
                }
            })?;
            nat_mul(a3348, a3349)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f112(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3351 = 0u64;
        let a3352 = match v0.clone() {
            Adt1::C0(_, h3350, _, _, _, _, _, _, _, _, _, _) => {
                h3350
            }
        };
        nat_lt(a3351, a3352)
    };
    let a3354 = match v1.clone() {
        Adt2::C0(h3353, _, _, _, _, _, _, _, _, _, _) => {
            h3353
        }
    };
    let a3355 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3354, a3355) {
        let a3357 = match v2.clone() {
            Adt2::C0(h3356, _, _, _, _, _, _, _, _, _, _) => {
                h3356
            }
        };
        let a3358 = 0u64;
        nat_eq(a3357, a3358)
    } else {
        false
    } {
        let a3360 = match v3.clone() {
            Adt2::C0(h3359, _, _, _, _, _, _, _, _, _, _) => {
                h3359
            }
        };
        let a3361 = 0u64;
        nat_eq(a3360, a3361)
    } else {
        false
    } {
        let a3363 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3362, _, _, _, _, _, _) => {
                h3362
            }
        };
        let a3364 = 1u64;
        nat_eq(a3363, a3364)
    } else {
        false
    } {
        let a3366 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3365, _, _, _, _, _, _) => {
                h3365
            }
        };
        let a3367 = 1u64;
        nat_eq(a3366, a3367)
    } else {
        false
    } {
        let a3369 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3368, _, _, _, _, _, _) => {
                h3368
            }
        };
        let a3370 = 1u64;
        nat_eq(a3369, a3370)
    } else {
        false
    } {
        let a3372 = 0u64;
        let a3373 = match v1.clone() {
            Adt2::C0(_, _, h3371, _, _, _, _, _, _, _, _) => {
                h3371
            }
        };
        nat_lt(a3372, a3373)
    } else {
        false
    } {
        let a3375 = match v3.clone() {
            Adt2::C0(_, _, _, h3374, _, _, _, _, _, _, _) => {
                h3374
            }
        };
        let a3376 = 1u64;
        nat_eq(a3375, a3376)
    } else {
        false
    } {
        let a3379 = match v3.clone() {
            Adt2::C0(_, h3377, _, _, _, _, _, _, _, _, _) => {
                h3377
            }
        };
        let a3380 = match v1.clone() {
            Adt2::C0(_, h3378, _, _, _, _, _, _, _, _, _) => {
                h3378
            }
        };
        nat_eq(a3379, a3380)
    } else {
        false
    } {
        let a3383 = match v3.clone() {
            Adt2::C0(_, _, h3381, _, _, _, _, _, _, _, _) => {
                h3381
            }
        };
        let a3384 = match v1.clone() {
            Adt2::C0(_, _, _, h3382, _, _, _, _, _, _, _) => {
                h3382
            }
        };
        nat_eq(a3383, a3384)
    } else {
        false
    } {
        if v4 {
            f21(v1.clone(), v2.clone())
        } else {
            let a3387 = match v2.clone() {
                Adt2::C0(_, h3385, _, _, _, _, _, _, _, _, _) => {
                    h3385
                }
            };
            let a3388 = match v1.clone() {
                Adt2::C0(_, _, h3386, _, _, _, _, _, _, _, _) => {
                    h3386
                }
            };
            if if nat_eq(a3387, a3388) {
                let a3391 = match v2.clone() {
                    Adt2::C0(_, _, h3389, _, _, _, _, _, _, _, _) => {
                        h3389
                    }
                };
                let a3392 = match v1.clone() {
                    Adt2::C0(_, _, _, h3390, _, _, _, _, _, _, _) => {
                        h3390
                    }
                };
                nat_eq(a3391, a3392)
            } else {
                false
            } {
                let a3394 = match v2.clone() {
                    Adt2::C0(_, _, _, h3393, _, _, _, _, _, _, _) => {
                        h3393
                    }
                };
                let a3395 = 1u64;
                nat_eq(a3394, a3395)
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
            let a3397 = f23(v3.clone(), 1u64)?;
            let a3398 = f172(match v1.clone() {
                Adt2::C0(_, _, h3396, _, _, _, _, _, _, _, _) => {
                    h3396
                }
            }, v4)?;
            nat_mul(a3397, a3398)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C36(f25(v2, 1u64), v4, match v0 {
            Adt1::C0(h3399, _, _, _, _, _, _, _, _, _, _, _) => {
                h3399
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f113(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3400, _, _, _, _, _, _, _, _, _) => {
            h3400
        }
    }, match v0.clone() {
        Adt1::C0(h3401, _, _, _, _, _, _, _, _, _, _, _) => {
            h3401
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3402, _, _, _, _, _, _, _, _, _) => {
            h3402
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3403, _, _, _, _, _, _, _) => {
            h3403
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3404, _, _, _, _, _, _, _, _) => {
            h3404
        }
    }, match v0.clone() {
        Adt1::C0(_, h3405, _, _, _, _, _, _, _, _, _, _) => {
            h3405
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3406, _, _, _, _, _, _, _, _) => {
            h3406
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3407, _, _, _, _, _, _) => {
            h3407
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3408, _, _, _, _, _, _, _, _, _, _) => {
            h3408
        }
    };
    let v7: bool = {
        let a3411 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3409, _, _, _) => {
                h3409
            }
        };
        let a3412 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3410, _, _, _, _, _) => {
                h3410
            }
        };
        nat_lt(a3411, a3412)
    };
    let a3413 = v6;
    let a3414 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3413, a3414) {
        true
    } else {
        let a3415 = v6;
        let a3416 = 1u64;
        nat_eq(a3415, a3416)
    } {
        let a3418 = match v2.clone() {
            Adt2::C0(h3417, _, _, _, _, _, _, _, _, _, _) => {
                h3417
            }
        };
        let a3419 = 0u64;
        nat_eq(a3418, a3419)
    } else {
        false
    } {
        let a3421 = match v3.clone() {
            Adt2::C0(h3420, _, _, _, _, _, _, _, _, _, _) => {
                h3420
            }
        };
        let a3422 = 0u64;
        nat_eq(a3421, a3422)
    } else {
        false
    } {
        let a3424 = match v1.clone() {
            Adt2::C0(_, _, _, h3423, _, _, _, _, _, _, _) => {
                h3423
            }
        };
        let a3425 = 1u64;
        nat_eq(a3424, a3425)
    } else {
        false
    } {
        let a3428 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3426, _, _, _, _, _, _) => {
                h3426
            }
        };
        let a3429 = match v2.clone() {
            Adt2::C0(_, _, _, h3427, _, _, _, _, _, _, _) => {
                h3427
            }
        };
        nat_eq(a3428, a3429)
    } else {
        false
    } {
        let a3432 = match v3.clone() {
            Adt2::C0(_, _, _, h3430, _, _, _, _, _, _, _) => {
                h3430
            }
        };
        let a3433 = match v2.clone() {
            Adt2::C0(_, _, _, h3431, _, _, _, _, _, _, _) => {
                h3431
            }
        };
        nat_eq(a3432, a3433)
    } else {
        false
    } {
        let a3436 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3434, _, _, _, _, _, _) => {
                h3434
            }
        };
        let a3437 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3435, _, _, _, _, _, _) => {
                h3435
            }
        };
        nat_eq(a3436, a3437)
    } else {
        false
    } {
        let a3439 = 0u64;
        let a3440 = match v4.clone() {
            Adt6::C0(h3438, _, _, _) => {
                h3438
            }
        };
        if if if if if nat_lt(a3439, a3440) {
            let a3442 = 0u64;
            let a3443 = match v4.clone() {
                Adt6::C0(_, h3441, _, _) => {
                    h3441
                }
            };
            nat_lt(a3442, a3443)
        } else {
            false
        } {
            let a3445 = match v4.clone() {
                Adt6::C0(_, h3444, _, _) => {
                    h3444
                }
            };
            let a3446 = 2147483648u64;
            nat_lt(a3445, a3446)
        } else {
            false
        } {
            let a3448 = 0u64;
            let a3449 = match v4.clone() {
                Adt6::C0(_, _, _, h3447) => {
                    h3447
                }
            };
            nat_lt(a3448, a3449)
        } else {
            false
        } {
            let a3451 = match v4.clone() {
                Adt6::C0(_, _, _, h3450) => {
                    h3450
                }
            };
            let a3452 = 2147483648u64;
            nat_lt(a3451, a3452)
        } else {
            false
        } {
            let a3455 = match v3.clone() {
                Adt2::C0(_, h3453, _, _, _, _, _, _, _, _, _) => {
                    h3453
                }
            };
            let a3456 = f95(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3454, _, _, _, _, _, _, _, _, _) => {
                    h3454
                }
            })?;
            nat_eq(a3455, a3456)
        } else {
            false
        }
    } else {
        false
    } {
        let a3458 = 0u64;
        let a3459 = match v5.clone() {
            Adt6::C0(h3457, _, _, _) => {
                h3457
            }
        };
        if if if if if nat_lt(a3458, a3459) {
            let a3461 = 0u64;
            let a3462 = match v5.clone() {
                Adt6::C0(_, h3460, _, _) => {
                    h3460
                }
            };
            nat_lt(a3461, a3462)
        } else {
            false
        } {
            let a3464 = match v5.clone() {
                Adt6::C0(_, h3463, _, _) => {
                    h3463
                }
            };
            let a3465 = 2147483648u64;
            nat_lt(a3464, a3465)
        } else {
            false
        } {
            let a3467 = 0u64;
            let a3468 = match v5.clone() {
                Adt6::C0(_, _, _, h3466) => {
                    h3466
                }
            };
            nat_lt(a3467, a3468)
        } else {
            false
        } {
            let a3470 = match v5.clone() {
                Adt6::C0(_, _, _, h3469) => {
                    h3469
                }
            };
            let a3471 = 2147483648u64;
            nat_lt(a3470, a3471)
        } else {
            false
        } {
            let a3474 = match v3.clone() {
                Adt2::C0(_, _, h3472, _, _, _, _, _, _, _, _) => {
                    h3472
                }
            };
            let a3475 = f95(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3473, _, _, _, _, _, _, _, _) => {
                    h3473
                }
            })?;
            nat_eq(a3474, a3475)
        } else {
            false
        }
    } else {
        false
    } {
        f223(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f223(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f223(v1.clone(), {
            let a3476 = v6;
            let a3477 = 0u64;
            if nat_eq(a3476, a3477) {
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
            let a3486 = {
                let a3484 = {
                    let a3482 = 3u64;
                    let a3483 = {
                        let a3480 = match v4.clone() {
                            Adt6::C0(h3478, _, _, _) => {
                                h3478
                            }
                        };
                        let a3481 = match v5.clone() {
                            Adt6::C0(h3479, _, _, _) => {
                                h3479
                            }
                        };
                        nat_mul(a3480, a3481)?
                    };
                    nat_mul(a3482, a3483)?
                };
                let a3485 = 2u64;
                nat_add(a3484, a3485)?
            };
            let a3487 = f23(v3.clone(), 1u64)?;
            nat_mul(a3486, a3487)?
        }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C34(f25(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
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
            let a3488 = 6u64;
            let a3489 = f23(v0.clone(), 1u64)?;
            nat_mul(a3488, a3489)?
        }, 2u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C33(f25(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f115(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3490 = v2;
    match m3490 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3491, _, _, _, _, _, _, _, _, _) => {
                    h3491
                }
            };
            let v6: u64 = {
                let a3497 = match v1.clone() {
                    Adt2::C0(_, _, h3492, _, _, _, _, _, _, _, _) => {
                        h3492
                    }
                };
                let a3498 = {
                    let a3495 = match v1.clone() {
                        Adt2::C0(_, _, _, h3493, _, _, _, _, _, _, _) => {
                            h3493
                        }
                    };
                    let a3496 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3494, _, _, _, _, _, _) => {
                            h3494
                        }
                    };
                    nat_mul(a3495, a3496)?
                };
                nat_mul(a3497, a3498)?
            };
            if if if if if if if if if if f21(v1.clone(), v4.clone()) {
                f21(v1.clone(), v3.clone())
            } else {
                false
            } {
                f224(v1.clone(), v6)?
            } else {
                false
            } {
                f224(v4.clone(), v6)?
            } else {
                false
            } {
                f224(v3.clone(), v6)?
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
                let a3499 = v5;
                let a3500 = 2147483647u64;
                nat_le(a3499, a3500)
            } else {
                false
            } {
                let a3501 = v6;
                let a3502 = 2147483647u64;
                nat_le(a3501, a3502)
            } else {
                false
            } {
                let a3504 = match v0.clone() {
                    Adt1::C0(h3503, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3503
                    }
                };
                let a3505 = 4294967295u64;
                if nat_le(a3504, a3505) {
                    let a3507 = match v0.clone() {
                        Adt1::C0(_, h3506, _, _, _, _, _, _, _, _, _, _) => {
                            h3506
                        }
                    };
                    let a3508 = 4294967295u64;
                    nat_le(a3507, a3508)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3509 = v5;
                    let a3510 = 0u64;
                    if nat_eq(a3509, a3510) {
                        0u64
                    } else {
                        let a3511 = v6;
                        let a3512 = f171(v5)?;
                        nat_mul(a3511, a3512)?
                    }
                }, 4u64, f25(v1, 1u64), f25(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3513, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3513
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f116(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3514, _, _, _, _, _, _, _, _, _, _, _) => {
            h3514
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3515, _, _, _, _, _, _, _, _, _, _) => {
            h3515
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3516, _, _, _, _, _, _, _, _, _) => {
            h3516
        }
    };
    let v6: u64 = {
        let a3518 = match v1.clone() {
            Adt2::C0(_, h3517, _, _, _, _, _, _, _, _, _) => {
                h3517
            }
        };
        let a3519 = v4;
        let a3520 = 0u64;
        nat_quot(a3518, a3519, a3520)
    };
    let v7: u64 = {
        let a3522 = match v1.clone() {
            Adt2::C0(h3521, _, _, _, _, _, _, _, _, _, _) => {
                h3521
            }
        };
        let a3523 = 0u64;
        if nat_eq(a3522, a3523) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f184(match v1.clone() {
        Adt2::C0(h3524, _, _, _, _, _, _, _, _, _, _) => {
            h3524
        }
    }) {
        let a3527 = match v1.clone() {
            Adt2::C0(h3525, _, _, _, _, _, _, _, _, _, _) => {
                h3525
            }
        };
        let a3528 = match v2.clone() {
            Adt2::C0(h3526, _, _, _, _, _, _, _, _, _, _) => {
                h3526
            }
        };
        nat_eq(a3527, a3528)
    } else {
        false
    } {
        let a3529 = 0u64;
        let a3530 = v3;
        nat_lt(a3529, a3530)
    } else {
        false
    } {
        let a3531 = v3;
        let a3532 = 2147483648u64;
        nat_lt(a3531, a3532)
    } else {
        false
    } {
        let a3533 = 0u64;
        let a3534 = v4;
        nat_lt(a3533, a3534)
    } else {
        false
    } {
        let a3535 = v4;
        let a3536 = 2147483648u64;
        nat_lt(a3535, a3536)
    } else {
        false
    } {
        let a3537 = v5;
        let a3538 = 2147483648u64;
        nat_lt(a3537, a3538)
    } else {
        false
    } {
        let a3539 = 0u64;
        let a3540 = v6;
        nat_lt(a3539, a3540)
    } else {
        false
    } {
        let a3542 = 0u64;
        let a3543 = match v2.clone() {
            Adt2::C0(_, h3541, _, _, _, _, _, _, _, _, _) => {
                h3541
            }
        };
        nat_lt(a3542, a3543)
    } else {
        false
    } {
        let a3547 = match v1.clone() {
            Adt2::C0(_, h3544, _, _, _, _, _, _, _, _, _) => {
                h3544
            }
        };
        let a3548 = {
            let a3545 = v6;
            let a3546 = v4;
            nat_mul(a3545, a3546)?
        };
        nat_eq(a3547, a3548)
    } else {
        false
    } {
        let a3550 = match v2.clone() {
            Adt2::C0(_, _, h3549, _, _, _, _, _, _, _, _) => {
                h3549
            }
        };
        let a3551 = v4;
        nat_eq(a3550, a3551)
    } else {
        false
    } {
        let a3564 = {
            let a3557 = {
                let a3555 = match v2.clone() {
                    Adt2::C0(_, h3552, _, _, _, _, _, _, _, _, _) => {
                        h3552
                    }
                };
                let a3556 = {
                    let a3553 = v5;
                    let a3554 = 2u64;
                    nat_mul(a3553, a3554)?
                };
                nat_add(a3555, a3556)?
            };
            let a3558 = v3;
            nat_add(a3557, a3558)?
        };
        let a3565 = {
            let a3562 = {
                let a3560 = match v1.clone() {
                    Adt2::C0(_, _, h3559, _, _, _, _, _, _, _, _) => {
                        h3559
                    }
                };
                let a3561 = v3;
                nat_mul(a3560, a3561)?
            };
            let a3563 = v6;
            nat_add(a3562, a3563)?
        };
        nat_eq(a3564, a3565)
    } else {
        false
    } {
        f225(v1.clone(), v7)?
    } else {
        false
    } {
        f225(v2.clone(), v7)?
    } else {
        false
    } {
        f30(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3566 = f23(v2.clone(), 1u64)?;
            let a3567 = f197(v8.clone())?;
            nat_mul(a3566, a3567)?
        }, v7, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3568, _, _, _, _, _, _, _, _, _, _) => {
                h3568
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f117(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3569 = v1;
    match m3569 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3571 = match v0.clone() {
                Adt2::C0(h3570, _, _, _, _, _, _, _, _, _, _) => {
                    h3570
                }
            };
            let a3572 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3571, a3572) {
                let a3574 = match v3.clone() {
                    Adt2::C0(h3573, _, _, _, _, _, _, _, _, _, _) => {
                        h3573
                    }
                };
                let a3575 = 0u64;
                nat_eq(a3574, a3575)
            } else {
                false
            } {
                let a3577 = match v2.clone() {
                    Adt2::C0(h3576, _, _, _, _, _, _, _, _, _, _) => {
                        h3576
                    }
                };
                let a3578 = 0u64;
                nat_eq(a3577, a3578)
            } else {
                false
            } {
                let a3580 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3579, _, _, _, _, _, _) => {
                        h3579
                    }
                };
                let a3581 = 1u64;
                nat_eq(a3580, a3581)
            } else {
                false
            } {
                let a3583 = match v3.clone() {
                    Adt2::C0(_, _, _, h3582, _, _, _, _, _, _, _) => {
                        h3582
                    }
                };
                let a3584 = 1u64;
                nat_eq(a3583, a3584)
            } else {
                false
            } {
                let a3586 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3585, _, _, _, _, _, _) => {
                        h3585
                    }
                };
                let a3587 = 1u64;
                nat_eq(a3586, a3587)
            } else {
                false
            } {
                let a3590 = match v0.clone() {
                    Adt2::C0(_, _, h3588, _, _, _, _, _, _, _, _) => {
                        h3588
                    }
                };
                let a3591 = match v3.clone() {
                    Adt2::C0(_, _, h3589, _, _, _, _, _, _, _, _) => {
                        h3589
                    }
                };
                nat_eq(a3590, a3591)
            } else {
                false
            } {
                let a3596 = match v3.clone() {
                    Adt2::C0(_, h3592, _, _, _, _, _, _, _, _, _) => {
                        h3592
                    }
                };
                let a3597 = {
                    let a3594 = match v0.clone() {
                        Adt2::C0(_, h3593, _, _, _, _, _, _, _, _, _) => {
                            h3593
                        }
                    };
                    let a3595 = 1u64;
                    nat_add(a3594, a3595)?
                };
                nat_le(a3596, a3597)
            } else {
                false
            } {
                let a3600 = match v2.clone() {
                    Adt2::C0(_, h3598, _, _, _, _, _, _, _, _, _) => {
                        h3598
                    }
                };
                let a3601 = match v0.clone() {
                    Adt2::C0(_, _, h3599, _, _, _, _, _, _, _, _) => {
                        h3599
                    }
                };
                nat_eq(a3600, a3601)
            } else {
                false
            } {
                let a3609 = match v2.clone() {
                    Adt2::C0(_, _, h3602, _, _, _, _, _, _, _, _) => {
                        h3602
                    }
                };
                let a3610 = {
                    let a3607 = {
                        let a3604 = match v0.clone() {
                            Adt2::C0(_, h3603, _, _, _, _, _, _, _, _, _) => {
                                h3603
                            }
                        };
                        let a3605 = 1u64;
                        nat_add(a3604, a3605)?
                    };
                    let a3608 = match v3.clone() {
                        Adt2::C0(_, h3606, _, _, _, _, _, _, _, _, _) => {
                            h3606
                        }
                    };
                    nat_sub(a3607, a3608)
                };
                nat_eq(a3609, a3610)
            } else {
                false
            } {
                let a3613 = match v2.clone() {
                    Adt2::C0(_, _, _, h3611, _, _, _, _, _, _, _) => {
                        h3611
                    }
                };
                let a3614 = match v0.clone() {
                    Adt2::C0(_, _, _, h3612, _, _, _, _, _, _, _) => {
                        h3612
                    }
                };
                nat_eq(a3613, a3614)
            } else {
                false
            } {
                let a3616 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3615, _, _, _, _, _, _) => {
                        h3615
                    }
                };
                let a3617 = 1u64;
                nat_eq(a3616, a3617)
            } else {
                false
            } {
                f38(v2.clone())?
            } else {
                false
            } {
                let a3619 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3618, _, _, _, _, _) => {
                        h3618
                    }
                };
                let a3620 = 4u64;
                nat_eq(a3619, a3620)
            } else {
                false
            } {
                let a3622 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3621, _, _, _, _, _) => {
                        h3621
                    }
                };
                let a3623 = 4u64;
                nat_eq(a3622, a3623)
            } else {
                false
            } {
                let a3628 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3624, _, _, _, _) => {
                        h3624
                    }
                };
                let a3629 = {
                    let a3626 = match v0.clone() {
                        Adt2::C0(_, h3625, _, _, _, _, _, _, _, _, _) => {
                            h3625
                        }
                    };
                    let a3627 = 4u64;
                    nat_mul(a3626, a3627)?
                };
                nat_eq(a3628, a3629)
            } else {
                false
            } {
                let a3634 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3630, _, _, _, _) => {
                        h3630
                    }
                };
                let a3635 = {
                    let a3632 = match v3.clone() {
                        Adt2::C0(_, h3631, _, _, _, _, _, _, _, _, _) => {
                            h3631
                        }
                    };
                    let a3633 = 4u64;
                    nat_mul(a3632, a3633)?
                };
                nat_eq(a3634, a3635)
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
                    let a3637 = f23(v2.clone(), 1u64)?;
                    let a3638 = f196(match v3.clone() {
                        Adt2::C0(_, h3636, _, _, _, _, _, _, _, _, _) => {
                            h3636
                        }
                    })?;
                    nat_mul(a3637, a3638)?
                }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C29(f25(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f118(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h3639, _, _, _, _, _, _, _, _, _, _) => {
            h3639
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h3640, _, _, _, _, _, _, _, _) => {
                h3640
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h3641, _, _, _, _, _, _, _, _, _) => {
                h3641
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h3642, _, _, _, _, _, _) => {
                h3642
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h3643, _, _, _, _, _, _, _, _) => {
                h3643
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h3644, _, _, _, _, _, _, _, _, _) => {
                h3644
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h3645, _, _, _, _, _, _, _) => {
                h3645
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h3646, _, _, _, _, _) => {
                h3646
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3648 = match v2.clone() {
        Adt1::C0(h3647, _, _, _, _, _, _, _, _, _, _, _) => {
            h3647
        }
    };
    let a3649 = 1u64;
    if if if if if if if if if if if nat_le(a3648, a3649) {
        let a3651 = match v3.clone() {
            Adt2::C0(h3650, _, _, _, _, _, _, _, _, _, _) => {
                h3650
            }
        };
        let a3652 = 1u64;
        nat_le(a3651, a3652)
    } else {
        false
    } {
        let a3654 = match v4.clone() {
            Adt2::C0(h3653, _, _, _, _, _, _, _, _, _, _) => {
                h3653
            }
        };
        let a3655 = 0u64;
        nat_eq(a3654, a3655)
    } else {
        false
    } {
        let a3660 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h3656, _, _, _, _, _) => {
                h3656
            }
        };
        let a3661 = {
            let a3658 = match v3.clone() {
                Adt2::C0(h3657, _, _, _, _, _, _, _, _, _, _) => {
                    h3657
                }
            };
            let a3659 = 0u64;
            if nat_eq(a3658, a3659) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3660, a3661)
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
        f226(v5.clone(), match v3.clone() {
            Adt2::C0(_, h3662, _, _, _, _, _, _, _, _, _) => {
                h3662
            }
        }, match v4.clone() {
            Adt2::C0(_, h3663, _, _, _, _, _, _, _, _, _) => {
                h3663
            }
        })?
    } else {
        false
    } {
        if v1 {
            f226(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h3664, _, _, _, _, _, _, _, _) => {
                    h3664
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h3665, _, _, _, _, _, _, _, _) => {
                    h3665
                }
            })?
        } else {
            let a3668 = match v3.clone() {
                Adt2::C0(_, _, h3666, _, _, _, _, _, _, _, _) => {
                    h3666
                }
            };
            let a3669 = match v4.clone() {
                Adt2::C0(_, _, h3667, _, _, _, _, _, _, _, _) => {
                    h3667
                }
            };
            nat_eq(a3668, a3669)
        }
    } else {
        false
    } {
        let a3672 = match v3.clone() {
            Adt2::C0(_, _, _, h3670, _, _, _, _, _, _, _) => {
                h3670
            }
        };
        let a3673 = match v4.clone() {
            Adt2::C0(_, _, _, h3671, _, _, _, _, _, _, _) => {
                h3671
            }
        };
        nat_eq(a3672, a3673)
    } else {
        false
    } {
        let a3676 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3674, _, _, _, _, _, _) => {
                h3674
            }
        };
        let a3677 = match v4.clone() {
            Adt2::C0(_, _, _, _, h3675, _, _, _, _, _, _) => {
                h3675
            }
        };
        nat_eq(a3676, a3677)
    } else {
        false
    } {
        let a3682 = {
            let a3680 = match v5.clone() {
                Adt6::C0(h3678, _, _, _) => {
                    h3678
                }
            };
            let a3681 = match v6.clone() {
                Adt6::C0(h3679, _, _, _) => {
                    h3679
                }
            };
            nat_mul(a3680, a3681)?
        };
        let a3683 = 2147483648u64;
        nat_lt(a3682, a3683)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a3685 = match v2 {
                Adt1::C0(h3684, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3684
                }
            };
            let a3686 = 0u64;
            nat_eq(a3685, a3686)
        }, match v3.clone() {
            Adt2::C0(h3687, _, _, _, _, _, _, _, _, _, _) => {
                h3687
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a3699 = f23(v3.clone(), 1u64)?;
                let a3700 = {
                    let a3697 = f23(v4.clone(), 1u64)?;
                    let a3698 = f175(match v7.clone() {
                        Adt7::C0(_, _, _, h3688, _) => {
                            h3688
                        }
                    }, {
                        let a3695 = {
                            let m3691 = match v7.clone() {
                                Adt7::C0(h3690, _, _, _, _) => {
                                    h3690
                                }
                            };
                            match m3691 {
                                Adt6::C0(h3689, _, _, _) => {
                                    h3689
                                }
                            }
                        };
                        let a3696 = {
                            let m3694 = match v7.clone() {
                                Adt7::C0(_, h3693, _, _, _) => {
                                    h3693
                                }
                            };
                            match m3694 {
                                Adt6::C0(h3692, _, _, _) => {
                                    h3692
                                }
                            }
                        };
                        nat_mul(a3695, a3696)?
                    })?;
                    nat_mul(a3697, a3698)?
                };
                nat_add(a3699, a3700)?
            }, 4u64, f25(v4, 1u64), f25(v3.clone(), 1u64), Adt4::C43(v7, f23(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a3701 = f23(v4.clone(), 1u64)?;
                let a3702 = f193(v7.clone())?;
                nat_mul(a3701, a3702)?
            }, 4u64, f25(v3, 1u64), f25(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f119(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3703 = v3;
    match m3703 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3705 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3704, _, _, _, _, _) => {
                        h3704
                    }
                };
                let a3706 = 1u64;
                nat_eq(a3705, a3706)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h3707, _, _, _, _, _, _, _, _, _) => {
                    h3707
                }
            }, match v1.clone() {
                Adt1::C0(h3708, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3708
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3709, _, _, _, _, _, _, _, _, _) => {
                    h3709
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h3710, _, _, _, _, _, _, _) => {
                    h3710
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h3711, _, _, _, _, _, _, _, _) => {
                        h3711
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h3712, _, _, _, _, _, _, _, _, _, _) => {
                        h3712
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3713, _, _, _, _, _, _, _, _) => {
                        h3713
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3714, _, _, _, _, _, _) => {
                        h3714
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3715, _, _, _, _, _, _, _) => {
                        h3715
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h3716, _, _, _, _, _, _, _, _) => {
                        h3716
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h3717, _, _, _, _, _, _) => {
                        h3717
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3718, _, _, _, _, _, _, _) => {
                        h3718
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h3719, _, _, _, _, _, _, _, _) => {
                        h3719
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h3720, _, _, _, _, _, _, _, _, _, _) => {
                    h3720
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h3721, _, _, _, _, _, _, _, _, _, _) => {
                    h3721
                }
            };
            let v15: u64 = {
                let a3722 = v13;
                let a3723 = 0u64;
                if nat_eq(a3722, a3723) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a3725 = match v5.clone() {
                    Adt2::C0(_, h3724, _, _, _, _, _, _, _, _, _) => {
                        h3724
                    }
                };
                let a3726 = v15;
                nat_mul(a3725, a3726)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3727, _, _, _) => {
                        h3727
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3728, _, _, _, _) => {
                        h3728
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3729, _, _) => {
                        h3729
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3730, _, _, _) => {
                        h3730
                    }
                }
            };
            let a3732 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h3731, _, _, _, _, _) => {
                    h3731
                }
            };
            let a3733 = 1u64;
            if if if if if if if if if if nat_le(a3732, a3733) {
                let a3734 = v13;
                let a3735 = 0u64;
                if nat_eq(a3734, a3735) {
                    true
                } else {
                    let a3736 = v13;
                    let a3737 = 1u64;
                    if nat_eq(a3736, a3737) {
                        let a3738 = v14;
                        let a3739 = 1u64;
                        nat_eq(a3738, a3739)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3740 = v14;
                let a3741 = 0u64;
                if nat_eq(a3740, a3741) {
                    true
                } else {
                    let a3742 = v14;
                    let a3743 = 1u64;
                    nat_eq(a3742, a3743)
                }
            } else {
                false
            } {
                let a3745 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3744, _, _, _, _, _) => {
                        h3744
                    }
                };
                let a3746 = v15;
                nat_eq(a3745, a3746)
            } else {
                false
            } {
                let a3754 = match v4.clone() {
                    Adt2::C0(_, h3747, _, _, _, _, _, _, _, _, _) => {
                        h3747
                    }
                };
                let a3755 = {
                    let a3752 = v10;
                    let a3753 = {
                        let a3750 = match v7.clone() {
                            Adt6::C0(h3748, _, _, _) => {
                                h3748
                            }
                        };
                        let a3751 = match v8.clone() {
                            Adt6::C0(h3749, _, _, _) => {
                                h3749
                            }
                        };
                        nat_mul(a3750, a3751)?
                    };
                    nat_mul(a3752, a3753)?
                };
                nat_eq(a3754, a3755)
            } else {
                false
            } {
                let a3758 = match v4.clone() {
                    Adt2::C0(_, _, h3756, _, _, _, _, _, _, _, _) => {
                        h3756
                    }
                };
                let a3759 = f95(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h3757, _, _, _, _, _, _, _, _, _) => {
                        h3757
                    }
                })?;
                nat_eq(a3758, a3759)
            } else {
                false
            } {
                let a3761 = 0u64;
                let a3762 = match v4.clone() {
                    Adt2::C0(_, _, h3760, _, _, _, _, _, _, _, _) => {
                        h3760
                    }
                };
                nat_lt(a3761, a3762)
            } else {
                false
            } {
                if v6 {
                    let a3764 = match v2 {
                        Adt2::C0(_, _, _, h3763, _, _, _, _, _, _, _) => {
                            h3763
                        }
                    };
                    let a3765 = v10;
                    if if if nat_eq(a3764, a3765) {
                        let a3767 = match v4.clone() {
                            Adt2::C0(_, _, _, h3766, _, _, _, _, _, _, _) => {
                                h3766
                            }
                        };
                        let a3768 = f95(v8.clone(), v12)?;
                        nat_eq(a3767, a3768)
                    } else {
                        false
                    } {
                        let a3770 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3769, _, _, _, _, _, _) => {
                                h3769
                            }
                        };
                        let a3771 = v11;
                        nat_eq(a3770, a3771)
                    } else {
                        false
                    } {
                        let a3773 = 0u64;
                        let a3774 = match v4.clone() {
                            Adt2::C0(_, _, _, h3772, _, _, _, _, _, _, _) => {
                                h3772
                            }
                        };
                        nat_lt(a3773, a3774)
                    } else {
                        false
                    }
                } else {
                    let a3776 = match v2 {
                        Adt2::C0(_, _, h3775, _, _, _, _, _, _, _, _) => {
                            h3775
                        }
                    };
                    let a3777 = v10;
                    if if if nat_eq(a3776, a3777) {
                        let a3779 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h3778, _, _, _, _, _, _) => {
                                h3778
                            }
                        };
                        let a3780 = 1u64;
                        nat_eq(a3779, a3780)
                    } else {
                        false
                    } {
                        let a3782 = match v4.clone() {
                            Adt2::C0(_, _, _, h3781, _, _, _, _, _, _, _) => {
                                h3781
                            }
                        };
                        let a3783 = v11;
                        nat_eq(a3782, a3783)
                    } else {
                        false
                    } {
                        let a3785 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3784, _, _, _, _, _, _) => {
                                h3784
                            }
                        };
                        let a3786 = 1u64;
                        nat_eq(a3785, a3786)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f48(match v5.clone() {
                    Adt2::C0(_, h3787, _, _, _, _, _, _, _, _, _) => {
                        h3787
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a3807 = {
                        let a3804 = {
                            let a3802 = {
                                let a3796 = {
                                    let a3790 = {
                                        let a3788 = v11;
                                        let a3789 = 1u64;
                                        nat_sub(a3788, a3789)
                                    };
                                    let a3791 = v18;
                                    nat_mul(a3790, a3791)?
                                };
                                let a3797 = {
                                    let a3794 = {
                                        let a3792 = v10;
                                        let a3793 = 1u64;
                                        nat_sub(a3792, a3793)
                                    };
                                    let a3795 = v17;
                                    nat_mul(a3794, a3795)?
                                };
                                nat_add(a3796, a3797)?
                            };
                            let a3803 = {
                                let a3800 = {
                                    let a3798 = v12;
                                    let a3799 = 1u64;
                                    nat_sub(a3798, a3799)
                                };
                                let a3801 = v16;
                                nat_mul(a3800, a3801)?
                            };
                            nat_add(a3802, a3803)?
                        };
                        let a3805 = v16;
                        nat_add(a3804, a3805)?
                    };
                    let a3808 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3806, _) => {
                            h3806
                        }
                    };
                    nat_le(a3807, a3808)
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
                        Adt2::C0(_, h3809, _, _, _, _, _, _, _, _, _) => {
                            h3809
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3810, _, _, _, _, _, _, _, _) => {
                            h3810
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h3811, _, _, _, _, _, _, _) => {
                            h3811
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h3812, _, _, _, _, _) => {
                            h3812
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3813, _, _, _, _) => {
                            h3813
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3814, _, _, _) => {
                            h3814
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a3821 = f23(v5.clone(), 1u64)?;
                        let a3822 = {
                            let a3819 = {
                                let a3817 = match v7.clone() {
                                    Adt6::C0(h3815, _, _, _) => {
                                        h3815
                                    }
                                };
                                let a3818 = match v8.clone() {
                                    Adt6::C0(h3816, _, _, _) => {
                                        h3816
                                    }
                                };
                                nat_mul(a3817, a3818)?
                            };
                            let a3820 = 2u64;
                            nat_add(a3819, a3820)?
                        };
                        nat_mul(a3821, a3822)?
                    }, 4u64, v19, f37(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f23(v4, 1u64)?, {
                        let a3823 = v14;
                        let a3824 = 0u64;
                        if nat_eq(a3823, a3824) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h3825, _, _, _, _, _, _, _, _, _) => {
                            h3825
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f120(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3826 = v2;
    match m3826 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h3827, _, _, _, _, _, _, _, _, _) => {
                    h3827
                }
            }, match v0.clone() {
                Adt1::C0(h3828, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3828
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h3829, _, _, _, _, _, _, _, _) => {
                    h3829
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h3830, _, _, _, _, _) => {
                    h3830
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h3831, _, _, _, _, _, _, _, _) => {
                    h3831
                }
            }, match v0.clone() {
                Adt1::C0(_, h3832, _, _, _, _, _, _, _, _, _, _) => {
                    h3832
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h3833, _, _, _, _, _, _, _) => {
                    h3833
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h3834, _, _, _, _) => {
                    h3834
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h3835, _, _, _, _, _, _, _) => {
                    h3835
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h3836, _, _, _, _, _, _, _, _, _) => {
                    h3836
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h3837, _, _, _, _, _, _) => {
                    h3837
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h3838, _, _, _) => {
                    h3838
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h3839, _, _) => {
                    h3839
                }
            };
            let v9: u64 = f95(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h3840, _, _, _, _, _, _, _) => {
                    h3840
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h3841, _, _, _, _, _, _, _, _, _, _) => {
                    h3841
                }
            };
            let v11: u64 = {
                let a3849 = v8;
                let a3850 = {
                    let a3847 = match v5.clone() {
                        Adt6::C0(h3842, _, _, _) => {
                            h3842
                        }
                    };
                    let a3848 = {
                        let a3845 = match v6.clone() {
                            Adt6::C0(h3843, _, _, _) => {
                                h3843
                            }
                        };
                        let a3846 = match v7.clone() {
                            Adt6::C0(h3844, _, _, _) => {
                                h3844
                            }
                        };
                        nat_mul(a3845, a3846)?
                    };
                    nat_mul(a3847, a3848)?
                };
                nat_mul(a3849, a3850)?
            };
            let a3852 = match v4.clone() {
                Adt2::C0(h3851, _, _, _, _, _, _, _, _, _, _) => {
                    h3851
                }
            };
            let a3853 = 0u64;
            if if if if if if if if if if if if if nat_eq(a3852, a3853) {
                let a3854 = v10;
                let a3855 = 0u64;
                if nat_eq(a3854, a3855) {
                    true
                } else {
                    let a3856 = v10;
                    let a3857 = 1u64;
                    nat_eq(a3856, a3857)
                }
            } else {
                false
            } {
                let a3858 = 0u64;
                let a3859 = v8;
                nat_lt(a3858, a3859)
            } else {
                false
            } {
                let a3860 = v8;
                let a3861 = 2147483648u64;
                nat_lt(a3860, a3861)
            } else {
                false
            } {
                let a3863 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h3862, _, _, _, _, _) => {
                        h3862
                    }
                };
                let a3864 = 4u64;
                nat_eq(a3863, a3864)
            } else {
                false
            } {
                let a3865 = 0u64;
                let a3866 = v9;
                nat_lt(a3865, a3866)
            } else {
                false
            } {
                let a3868 = 0u64;
                let a3869 = match v3.clone() {
                    Adt2::C0(_, _, h3867, _, _, _, _, _, _, _, _) => {
                        h3867
                    }
                };
                nat_lt(a3868, a3869)
            } else {
                false
            } {
                let a3871 = 0u64;
                let a3872 = match v3.clone() {
                    Adt2::C0(_, _, _, h3870, _, _, _, _, _, _, _) => {
                        h3870
                    }
                };
                nat_lt(a3871, a3872)
            } else {
                false
            } {
                let a3874 = match v3.clone() {
                    Adt2::C0(_, h3873, _, _, _, _, _, _, _, _, _) => {
                        h3873
                    }
                };
                let a3875 = v11;
                nat_eq(a3874, a3875)
            } else {
                false
            } {
                let a3878 = match v3.clone() {
                    Adt2::C0(_, _, h3876, _, _, _, _, _, _, _, _) => {
                        h3876
                    }
                };
                let a3879 = f95(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3877, _, _, _, _, _, _, _, _, _) => {
                        h3877
                    }
                })?;
                nat_eq(a3878, a3879)
            } else {
                false
            } {
                let a3882 = match v3.clone() {
                    Adt2::C0(_, _, _, h3880, _, _, _, _, _, _, _) => {
                        h3880
                    }
                };
                let a3883 = f95(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3881, _, _, _, _, _, _, _, _) => {
                        h3881
                    }
                })?;
                nat_eq(a3882, a3883)
            } else {
                false
            } {
                let a3891 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3884, _, _, _, _, _, _) => {
                        h3884
                    }
                };
                let a3892 = {
                    let a3889 = v9;
                    let a3890 = {
                        let a3886 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3885, _, _, _, _, _, _) => {
                                h3885
                            }
                        };
                        let a3887 = v8;
                        let a3888 = 0u64;
                        nat_quot(a3886, a3887, a3888)
                    };
                    nat_mul(a3889, a3890)?
                };
                nat_eq(a3891, a3892)
            } else {
                false
            } {
                f30(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a3893 = v10;
                    let a3894 = 0u64;
                    if nat_eq(a3893, a3894) {
                        4u64
                    } else {
                        2u64
                    }
                }, f25(v4.clone(), 1u64), f25(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3895, _, _) => {
                        h3895
                    }
                }, {
                    let a3897 = v8;
                    let a3898 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3896, _, _) => {
                            h3896
                        }
                    };
                    nat_mul(a3897, a3898)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f121(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3899 = v1;
    match m3899 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f227(v0, v3, v2)
        }
    }
}

pub fn f122(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3901 = match v0.clone() {
        Adt2::C0(h3900, _, _, _, _, _, _, _, _, _, _) => {
            h3900
        }
    };
    let a3902 = 1u64;
    if if nat_eq(a3901, a3902) {
        true
    } else {
        let a3904 = match v0.clone() {
            Adt2::C0(h3903, _, _, _, _, _, _, _, _, _, _) => {
                h3903
            }
        };
        let a3905 = 30u64;
        nat_eq(a3904, a3905)
    } {
        let a3907 = match v1.clone() {
            Adt2::C0(h3906, _, _, _, _, _, _, _, _, _, _) => {
                h3906
            }
        };
        let a3908 = 1u64;
        if if if if if if if if if if if nat_eq(a3907, a3908) {
            let a3911 = match v0.clone() {
                Adt2::C0(_, h3909, _, _, _, _, _, _, _, _, _) => {
                    h3909
                }
            };
            let a3912 = match v1.clone() {
                Adt2::C0(_, h3910, _, _, _, _, _, _, _, _, _) => {
                    h3910
                }
            };
            nat_eq(a3911, a3912)
        } else {
            false
        } {
            let a3914 = 0u64;
            let a3915 = match v1.clone() {
                Adt2::C0(_, _, h3913, _, _, _, _, _, _, _, _) => {
                    h3913
                }
            };
            nat_lt(a3914, a3915)
        } else {
            false
        } {
            let a3918 = match v1.clone() {
                Adt2::C0(_, _, h3916, _, _, _, _, _, _, _, _) => {
                    h3916
                }
            };
            let a3919 = match v1.clone() {
                Adt2::C0(_, _, _, h3917, _, _, _, _, _, _, _) => {
                    h3917
                }
            };
            nat_eq(a3918, a3919)
        } else {
            false
        } {
            let a3921 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3920, _, _, _, _, _, _) => {
                    h3920
                }
            };
            let a3922 = 1u64;
            nat_eq(a3921, a3922)
        } else {
            false
        } {
            let a3930 = match v0.clone() {
                Adt2::C0(_, _, h3923, _, _, _, _, _, _, _, _) => {
                    h3923
                }
            };
            let a3931 = {
                let a3928 = {
                    let a3926 = match v1.clone() {
                        Adt2::C0(_, _, h3924, _, _, _, _, _, _, _, _) => {
                            h3924
                        }
                    };
                    let a3927 = match v1.clone() {
                        Adt2::C0(_, _, h3925, _, _, _, _, _, _, _, _) => {
                            h3925
                        }
                    };
                    nat_add(a3926, a3927)?
                };
                let a3929 = 1u64;
                nat_sub(a3928, a3929)
            };
            nat_eq(a3930, a3931)
        } else {
            false
        } {
            let a3933 = 0u64;
            let a3934 = match v0.clone() {
                Adt2::C0(_, _, _, h3932, _, _, _, _, _, _, _) => {
                    h3932
                }
            };
            nat_lt(a3933, a3934)
        } else {
            false
        } {
            let a3936 = 0u64;
            let a3937 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3935, _, _, _, _, _, _) => {
                    h3935
                }
            };
            nat_lt(a3936, a3937)
        } else {
            false
        } {
            let a3939 = 2u64;
            let a3940 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3938, _, _, _, _, _) => {
                    h3938
                }
            };
            nat_le(a3939, a3940)
        } else {
            false
        } {
            let a3942 = 2u64;
            let a3943 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3941, _, _, _, _, _) => {
                    h3941
                }
            };
            nat_le(a3942, a3943)
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

pub fn f123(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3944 = v1;
    match m3944 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f228(v0, v3, v2)
        }
    }
}

pub fn f124(v0: u64) -> bool {
    let a3945 = v0;
    let a3946 = 1u64;
    if if nat_eq(a3945, a3946) {
        true
    } else {
        let a3947 = v0;
        let a3948 = 33u64;
        nat_eq(a3947, a3948)
    } {
        true
    } else {
        let a3949 = v0;
        let a3950 = 34u64;
        nat_eq(a3949, a3950)
    }
}

pub fn f125(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f47(v1, v2.clone()) {
        let a3951 = v3;
        let a3952 = f23(v2.clone(), 1u64)?;
        nat_eq(a3951, a3952)
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
        let a3953 = v3;
        let a3954 = 0u64;
        if nat_eq(a3953, a3954) {
            true
        } else {
            let a3956 = 4u64;
            let a3957 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3955, _, _, _, _, _) => {
                    h3955
                }
            };
            nat_le(a3956, a3957)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3958 = 2u64;
            let a3959 = v3;
            nat_mul(a3958, a3959)?
        }, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f126(v0: u64) -> bool {
    if f229(v0) {
        !f230(v0)
    } else {
        false
    }
}

pub fn f127(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3960 = v1;
    match m3960 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3962 = f221(match v0.clone() {
                Adt2::C0(h3961, _, _, _, _, _, _, _, _, _, _) => {
                    h3961
                }
            });
            match m3962 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3963, _) => {
                            h3963
                        }
                    };
                    if if if if if if f47(v3, v2.clone()) {
                        f30(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3964 = f23(v0.clone(), 1u64)?;
                        let a3965 = f23(v2.clone(), 1u64)?;
                        nat_eq(a3964, a3965)
                    } else {
                        false
                    } {
                        let a3970 = {
                            let a3967 = match v0.clone() {
                                Adt2::C0(_, h3966, _, _, _, _, _, _, _, _, _) => {
                                    h3966
                                }
                            };
                            let a3968 = v5;
                            let a3969 = 1u64;
                            nat_rem(a3967, a3968, a3969)
                        };
                        let a3971 = 0u64;
                        if nat_eq(a3970, a3971) {
                            let a3974 = match v4 {
                                Adt19::C0(_, _, h3972) => {
                                    h3972
                                }
                            };
                            let a3975 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3973, _, _, _, _, _) => {
                                    h3973
                                }
                            };
                            nat_le(a3974, a3975)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3976 = v5;
                        let a3977 = 1u64;
                        if nat_eq(a3976, a3977) {
                            let a3979 = 4u64;
                            let a3980 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3978, _, _, _, _, _) => {
                                    h3978
                                }
                            };
                            nat_le(a3979, a3980)
                        } else {
                            let a3982 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3981, _, _, _, _, _) => {
                                    h3981
                                }
                            };
                            let a3983 = 4u64;
                            nat_eq(a3982, a3983)
                        }
                    } else {
                        false
                    } {
                        let a3988 = {
                            let a3985 = match v2.clone() {
                                Adt2::C0(_, h3984, _, _, _, _, _, _, _, _, _) => {
                                    h3984
                                }
                            };
                            let a3986 = v5;
                            let a3987 = 1u64;
                            nat_rem(a3985, a3986, a3987)
                        };
                        let a3989 = 0u64;
                        if nat_eq(a3988, a3989) {
                            true
                        } else {
                            f38(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f25(v0.clone(), v5), f25(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3990, _, _, _, _, _, _, _, _, _, _) => {
                                h3990
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

pub fn f128(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3993 = match v1.clone() {
        Adt2::C0(h3991, _, _, _, _, _, _, _, _, _, _) => {
            h3991
        }
    };
    let a3994 = match v3.clone() {
        Adt2::C0(h3992, _, _, _, _, _, _, _, _, _, _) => {
            h3992
        }
    };
    if nat_eq(a3993, a3994) {
        f231(v0, v1, v2, v3)
    } else {
        if if if if f230(match v1.clone() {
            Adt2::C0(h3995, _, _, _, _, _, _, _, _, _, _) => {
                h3995
            }
        }) {
            f230(match v3.clone() {
                Adt2::C0(h3996, _, _, _, _, _, _, _, _, _, _) => {
                    h3996
                }
            })
        } else {
            false
        } {
            let a3997 = v0;
            let a3998 = 33u64;
            if nat_eq(a3997, a3998) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3999 = f23(v1.clone(), 1u64)?;
            let a4000 = f23(v3.clone(), 1u64)?;
            nat_eq(a3999, a4000)
        } else {
            false
        } {
            let m4002 = f221(match v3.clone() {
                Adt2::C0(h4001, _, _, _, _, _, _, _, _, _, _) => {
                    h4001
                }
            });
            match m4002 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h4003, _, _, _, _, _, _, _, _, _, _) => {
                            h4003
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h4004, _, _, _, _, _, _, _, _, _, _) => {
                            h4004
                        }
                    };
                    let v7: bool = {
                        let a4005 = v5;
                        let a4006 = 43u64;
                        if nat_eq(a4005, a4006) {
                            true
                        } else {
                            let a4007 = v6;
                            let a4008 = 43u64;
                            nat_eq(a4007, a4008)
                        }
                    };
                    Ok(Adt10::C4({
                        let a4009 = f23(v3.clone(), 1u64)?;
                        let a4010 = if v7 {
                            f169(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a4009, a4010)?
                    }, match v4 {
                        Adt19::C0(_, _, h4011) => {
                            h4011
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

pub fn f129(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4013 = match v0.clone() {
        Adt2::C0(h4012, _, _, _, _, _, _, _, _, _, _) => {
            h4012
        }
    };
    let a4014 = 0u64;
    if if if if nat_eq(a4013, a4014) {
        true
    } else {
        let a4016 = match v0.clone() {
            Adt2::C0(h4015, _, _, _, _, _, _, _, _, _, _) => {
                h4015
            }
        };
        let a4017 = 1u64;
        nat_eq(a4016, a4017)
    } {
        true
    } else {
        let a4019 = match v0.clone() {
            Adt2::C0(h4018, _, _, _, _, _, _, _, _, _, _) => {
                h4018
            }
        };
        let a4020 = 25u64;
        if nat_eq(a4019, a4020) {
            true
        } else {
            let a4022 = match v0.clone() {
                Adt2::C0(h4021, _, _, _, _, _, _, _, _, _, _) => {
                    h4021
                }
            };
            let a4023 = 26u64;
            nat_eq(a4022, a4023)
        }
    } {
        true
    } else {
        let a4025 = match v0.clone() {
            Adt2::C0(h4024, _, _, _, _, _, _, _, _, _, _) => {
                h4024
            }
        };
        let a4026 = 30u64;
        nat_eq(a4025, a4026)
    } {
        let a4029 = match v0.clone() {
            Adt2::C0(h4027, _, _, _, _, _, _, _, _, _, _) => {
                h4027
            }
        };
        let a4030 = match v1.clone() {
            Adt2::C0(h4028, _, _, _, _, _, _, _, _, _, _) => {
                h4028
            }
        };
        if if nat_eq(a4029, a4030) {
            f232(match v1.clone() {
                Adt2::C0(_, h4031, _, _, _, _, _, _, _, _, _) => {
                    h4031
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h4032, _, _, _, _, _, _, _, _) => {
                    h4032
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h4033, _, _, _, _, _, _, _) => {
                    h4033
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h4034, _, _, _, _, _, _) => {
                    h4034
                }
            }, match v0.clone() {
                Adt2::C0(_, h4035, _, _, _, _, _, _, _, _, _) => {
                    h4035
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h4036, _, _, _, _, _, _, _, _) => {
                    h4036
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h4037, _, _, _, _, _, _, _) => {
                    h4037
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h4038, _, _, _, _, _, _) => {
                    h4038
                }
            })
        } else {
            false
        } {
            let m4040 = f221(match v0.clone() {
                Adt2::C0(h4039, _, _, _, _, _, _, _, _, _, _) => {
                    h4039
                }
            });
            match m4040 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h4041) => {
                            h4041
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

pub fn f130(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4042 = v2;
    match m4042 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a4043 = v0;
            let a4044 = 4u64;
            if if if nat_lt(a4043, a4044) {
                let a4047 = match v1.clone() {
                    Adt2::C0(h4045, _, _, _, _, _, _, _, _, _, _) => {
                        h4045
                    }
                };
                let a4048 = match v4.clone() {
                    Adt2::C0(h4046, _, _, _, _, _, _, _, _, _, _) => {
                        h4046
                    }
                };
                if nat_eq(a4047, a4048) {
                    let a4051 = match v1.clone() {
                        Adt2::C0(h4049, _, _, _, _, _, _, _, _, _, _) => {
                            h4049
                        }
                    };
                    let a4052 = match v3.clone() {
                        Adt2::C0(h4050, _, _, _, _, _, _, _, _, _, _) => {
                            h4050
                        }
                    };
                    nat_eq(a4051, a4052)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f233(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h4053, _, _, _, _, _, _, _, _, _) => {
                        h4053
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4054, _, _, _, _, _, _, _, _, _) => {
                        h4054
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h4055, _, _, _, _, _, _, _, _, _) => {
                        h4055
                    }
                })? {
                    f233(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h4056, _, _, _, _, _, _, _, _) => {
                            h4056
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4057, _, _, _, _, _, _, _, _) => {
                            h4057
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h4058, _, _, _, _, _, _, _, _) => {
                            h4058
                        }
                    })?
                } else {
                    false
                } {
                    f233(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h4059, _, _, _, _, _, _, _) => {
                            h4059
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h4060, _, _, _, _, _, _, _) => {
                            h4060
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h4061, _, _, _, _, _, _, _) => {
                            h4061
                        }
                    })?
                } else {
                    false
                } {
                    f233(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h4062, _, _, _, _, _, _) => {
                            h4062
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h4063, _, _, _, _, _, _) => {
                            h4063
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h4064, _, _, _, _, _, _) => {
                            h4064
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m4066 = f221(match v1.clone() {
                    Adt2::C0(h4065, _, _, _, _, _, _, _, _, _, _) => {
                        h4065
                    }
                });
                match m4066 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h4067, _) => {
                                h4067
                            }
                        };
                        let a4072 = {
                            let a4069 = match v1.clone() {
                                Adt2::C0(_, h4068, _, _, _, _, _, _, _, _, _) => {
                                    h4068
                                }
                            };
                            let a4070 = v6;
                            let a4071 = 1u64;
                            nat_rem(a4069, a4070, a4071)
                        };
                        let a4073 = 0u64;
                        if if if nat_eq(a4072, a4073) {
                            let a4078 = {
                                let a4075 = match v4.clone() {
                                    Adt2::C0(_, h4074, _, _, _, _, _, _, _, _, _) => {
                                        h4074
                                    }
                                };
                                let a4076 = v6;
                                let a4077 = 1u64;
                                nat_rem(a4075, a4076, a4077)
                            };
                            let a4079 = 0u64;
                            nat_eq(a4078, a4079)
                        } else {
                            false
                        } {
                            let a4084 = {
                                let a4081 = match v3.clone() {
                                    Adt2::C0(_, h4080, _, _, _, _, _, _, _, _, _) => {
                                        h4080
                                    }
                                };
                                let a4082 = v6;
                                let a4083 = 1u64;
                                nat_rem(a4081, a4082, a4083)
                            };
                            let a4085 = 0u64;
                            nat_eq(a4084, a4085)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h4086) => {
                                    h4086
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

pub fn f131(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4088 = match v2.clone() {
        Adt2::C0(h4087, _, _, _, _, _, _, _, _, _, _) => {
            h4087
        }
    };
    let a4089 = 0u64;
    if if nat_eq(a4088, a4089) {
        let a4091 = match v3.clone() {
            Adt2::C0(h4090, _, _, _, _, _, _, _, _, _, _) => {
                h4090
            }
        };
        let a4092 = 0u64;
        nat_eq(a4091, a4092)
    } else {
        false
    } {
        let a4093 = v0;
        let a4094 = 61u64;
        if nat_eq(a4093, a4094) {
            let v4: bool = {
                let a4096 = 0u64;
                let a4097 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4095, _, _, _) => {
                        h4095
                    }
                };
                nat_lt(a4096, a4097)
            };
            if if if if if if f234(match v2.clone() {
                Adt2::C0(_, h4098, _, _, _, _, _, _, _, _, _) => {
                    h4098
                }
            }, match v1.clone() {
                Adt1::C0(h4099, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4099
                }
            }, match v1.clone() {
                Adt1::C0(_, h4100, _, _, _, _, _, _, _, _, _, _) => {
                    h4100
                }
            }, match v3.clone() {
                Adt2::C0(_, h4101, _, _, _, _, _, _, _, _, _) => {
                    h4101
                }
            })? {
                f234(match v2.clone() {
                    Adt2::C0(_, _, h4102, _, _, _, _, _, _, _, _) => {
                        h4102
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4103, _, _, _, _, _, _, _, _, _) => {
                        h4103
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4104, _, _, _, _, _, _, _, _) => {
                        h4104
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h4105, _, _, _, _, _, _, _, _) => {
                        h4105
                    }
                })?
            } else {
                false
            } {
                f234(match v2.clone() {
                    Adt2::C0(_, _, _, h4106, _, _, _, _, _, _, _) => {
                        h4106
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4107, _, _, _, _, _, _, _) => {
                        h4107
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4108, _, _, _, _, _, _) => {
                        h4108
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h4109, _, _, _, _, _, _, _) => {
                        h4109
                    }
                })?
            } else {
                false
            } {
                f234(match v2.clone() {
                    Adt2::C0(_, _, _, _, h4110, _, _, _, _, _, _) => {
                        h4110
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4111, _, _, _, _, _) => {
                        h4111
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h4112, _, _, _, _) => {
                        h4112
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h4113, _, _, _, _, _, _) => {
                        h4113
                    }
                })?
            } else {
                false
            } {
                let a4115 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4114, _, _, _) => {
                        h4114
                    }
                };
                let a4116 = 4294967295u64;
                nat_le(a4115, a4116)
            } else {
                false
            } {
                if v4 {
                    !f48(match v2.clone() {
                        Adt2::C0(_, h4117, _, _, _, _, _, _, _, _, _) => {
                            h4117
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h4118, _, _, _, _, _, _, _, _) => {
                            h4118
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h4119, _, _, _, _, _, _, _) => {
                            h4119
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h4120, _, _, _, _, _, _) => {
                            h4120
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h4121, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4121
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4122, _, _, _, _, _, _, _, _, _) => {
                        h4122
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4123, _, _, _, _, _, _, _) => {
                        h4123
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h4124, _, _, _, _, _) => {
                        h4124
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a4125 = v0;
            let a4126 = 62u64;
            if nat_eq(a4125, a4126) {
                let a4128 = match v1.clone() {
                    Adt1::C0(h4127, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4127
                    }
                };
                let a4129 = 2147483648u64;
                if if if if if nat_lt(a4128, a4129) {
                    let a4131 = match v1.clone() {
                        Adt1::C0(_, h4130, _, _, _, _, _, _, _, _, _, _) => {
                            h4130
                        }
                    };
                    let a4132 = 2147483648u64;
                    nat_lt(a4131, a4132)
                } else {
                    false
                } {
                    let a4135 = match v1.clone() {
                        Adt1::C0(h4133, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4133
                        }
                    };
                    let a4136 = match v2.clone() {
                        Adt2::C0(_, h4134, _, _, _, _, _, _, _, _, _) => {
                            h4134
                        }
                    };
                    nat_lt(a4135, a4136)
                } else {
                    false
                } {
                    let a4139 = match v1.clone() {
                        Adt1::C0(_, h4137, _, _, _, _, _, _, _, _, _, _) => {
                            h4137
                        }
                    };
                    let a4140 = match v2.clone() {
                        Adt2::C0(_, h4138, _, _, _, _, _, _, _, _, _) => {
                            h4138
                        }
                    };
                    nat_lt(a4139, a4140)
                } else {
                    false
                } {
                    let a4143 = match v2.clone() {
                        Adt2::C0(_, _, h4141, _, _, _, _, _, _, _, _) => {
                            h4141
                        }
                    };
                    let a4144 = match v3.clone() {
                        Adt2::C0(_, _, h4142, _, _, _, _, _, _, _, _) => {
                            h4142
                        }
                    };
                    if if if nat_eq(a4143, a4144) {
                        let a4147 = match v2.clone() {
                            Adt2::C0(_, _, _, h4145, _, _, _, _, _, _, _) => {
                                h4145
                            }
                        };
                        let a4148 = match v3.clone() {
                            Adt2::C0(_, _, _, h4146, _, _, _, _, _, _, _) => {
                                h4146
                            }
                        };
                        nat_eq(a4147, a4148)
                    } else {
                        false
                    } {
                        let a4151 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4149, _, _, _, _, _, _) => {
                                h4149
                            }
                        };
                        let a4152 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4150, _, _, _, _, _, _) => {
                                h4150
                            }
                        };
                        nat_eq(a4151, a4152)
                    } else {
                        false
                    } {
                        let a4161 = match v3.clone() {
                            Adt2::C0(_, h4153, _, _, _, _, _, _, _, _, _) => {
                                h4153
                            }
                        };
                        let a4162 = {
                            let a4159 = {
                                let a4156 = match v2.clone() {
                                    Adt2::C0(_, h4154, _, _, _, _, _, _, _, _, _) => {
                                        h4154
                                    }
                                };
                                let a4157 = match v1.clone() {
                                    Adt1::C0(h4155, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4155
                                    }
                                };
                                nat_add(a4156, a4157)?
                            };
                            let a4160 = match v1.clone() {
                                Adt1::C0(_, h4158, _, _, _, _, _, _, _, _, _, _) => {
                                    h4158
                                }
                            };
                            nat_add(a4159, a4160)?
                        };
                        nat_eq(a4161, a4162)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4163, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4163
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4164 = v0;
                let a4165 = 63u64;
                if nat_eq(a4164, a4165) {
                    if if if if if f235(match v2.clone() {
                        Adt2::C0(_, h4166, _, _, _, _, _, _, _, _, _) => {
                            h4166
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4167, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4167
                        }
                    })? {
                        f235(match v2.clone() {
                            Adt2::C0(_, _, h4168, _, _, _, _, _, _, _, _) => {
                                h4168
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4169, _, _, _, _, _, _, _, _, _, _) => {
                                h4169
                            }
                        })?
                    } else {
                        false
                    } {
                        f235(match v2.clone() {
                            Adt2::C0(_, _, _, h4170, _, _, _, _, _, _, _) => {
                                h4170
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4171, _, _, _, _, _, _, _, _, _) => {
                                h4171
                            }
                        })?
                    } else {
                        false
                    } {
                        f235(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4172, _, _, _, _, _, _) => {
                                h4172
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4173, _, _, _, _, _, _, _, _) => {
                                h4173
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4176 = match v2.clone() {
                            Adt2::C0(_, h4174, _, _, _, _, _, _, _, _, _) => {
                                h4174
                            }
                        };
                        let a4177 = match v3.clone() {
                            Adt2::C0(_, h4175, _, _, _, _, _, _, _, _, _) => {
                                h4175
                            }
                        };
                        if if if nat_eq(a4176, a4177) {
                            let a4180 = match v2.clone() {
                                Adt2::C0(_, _, h4178, _, _, _, _, _, _, _, _) => {
                                    h4178
                                }
                            };
                            let a4181 = match v3.clone() {
                                Adt2::C0(_, _, h4179, _, _, _, _, _, _, _, _) => {
                                    h4179
                                }
                            };
                            nat_eq(a4180, a4181)
                        } else {
                            false
                        } {
                            let a4184 = match v2.clone() {
                                Adt2::C0(_, _, _, h4182, _, _, _, _, _, _, _) => {
                                    h4182
                                }
                            };
                            let a4185 = match v3.clone() {
                                Adt2::C0(_, _, _, h4183, _, _, _, _, _, _, _) => {
                                    h4183
                                }
                            };
                            nat_eq(a4184, a4185)
                        } else {
                            false
                        } {
                            let a4188 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4186, _, _, _, _, _, _) => {
                                    h4186
                                }
                            };
                            let a4189 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4187, _, _, _, _, _, _) => {
                                    h4187
                                }
                            };
                            nat_eq(a4188, a4189)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f25(v2, 1u64), f25(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4190, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4190
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4191, _, _, _, _, _, _, _, _, _, _) => {
                                h4191
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4192, _, _, _, _, _, _, _, _, _) => {
                                h4192
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4193, _, _, _, _, _, _, _, _) => {
                                h4193
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

pub fn f132(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4194 = v3;
    match m4194 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4195 = v0;
                let a4196 = 4u64;
                nat_eq(a4195, a4196)
            };
            let v7: bool = {
                let a4198 = 0u64;
                let a4199 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4197, _, _, _, _, _, _, _) => {
                        h4197
                    }
                };
                nat_lt(a4198, a4199)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4200, _, _, _, _, _, _, _, _, _) => {
                    h4200
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4201, _, _, _, _, _, _, _, _) => {
                    h4201
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4202, _, _, _, _, _, _, _) => {
                    h4202
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4203, _, _, _, _, _, _) => {
                    h4203
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4204, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4204
                }
            }, match v1.clone() {
                Adt1::C0(_, h4205, _, _, _, _, _, _, _, _, _, _) => {
                    h4205
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4206, _, _, _, _, _, _, _, _, _) => {
                    h4206
                }
            });
            if if if if if if if if if if if if v6 {
                let a4208 = match v2.clone() {
                    Adt2::C0(h4207, _, _, _, _, _, _, _, _, _, _) => {
                        h4207
                    }
                };
                let a4209 = 0u64;
                if nat_eq(a4208, a4209) {
                    let a4211 = match v5.clone() {
                        Adt2::C0(h4210, _, _, _, _, _, _, _, _, _, _) => {
                            h4210
                        }
                    };
                    let a4212 = 0u64;
                    nat_eq(a4211, a4212)
                } else {
                    false
                }
            } else {
                let a4214 = match v2.clone() {
                    Adt2::C0(h4213, _, _, _, _, _, _, _, _, _, _) => {
                        h4213
                    }
                };
                let a4215 = 0u64;
                if if nat_eq(a4214, a4215) {
                    true
                } else {
                    let a4217 = match v2.clone() {
                        Adt2::C0(h4216, _, _, _, _, _, _, _, _, _, _) => {
                            h4216
                        }
                    };
                    let a4218 = 26u64;
                    nat_eq(a4217, a4218)
                } {
                    let a4220 = match v5.clone() {
                        Adt2::C0(h4219, _, _, _, _, _, _, _, _, _, _) => {
                            h4219
                        }
                    };
                    let a4221 = 0u64;
                    if nat_eq(a4220, a4221) {
                        true
                    } else {
                        let a4223 = match v5.clone() {
                            Adt2::C0(h4222, _, _, _, _, _, _, _, _, _, _) => {
                                h4222
                            }
                        };
                        let a4224 = 26u64;
                        nat_eq(a4223, a4224)
                    }
                } else {
                    false
                }
            } {
                let a4227 = match v2.clone() {
                    Adt2::C0(h4225, _, _, _, _, _, _, _, _, _, _) => {
                        h4225
                    }
                };
                let a4228 = match v4.clone() {
                    Adt2::C0(h4226, _, _, _, _, _, _, _, _, _, _) => {
                        h4226
                    }
                };
                nat_eq(a4227, a4228)
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
                let a4230 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4229, _, _, _, _, _) => {
                        h4229
                    }
                };
                let a4231 = 4u64;
                nat_eq(a4230, a4231)
            } else {
                false
            } {
                let a4232 = v8;
                let a4233 = f23(v2.clone(), 1u64)?;
                nat_le(a4232, a4233)
            } else {
                false
            } {
                let a4235 = match v1.clone() {
                    Adt1::C0(_, _, _, h4234, _, _, _, _, _, _, _, _) => {
                        h4234
                    }
                };
                let a4236 = 1073741824u64;
                nat_lt(a4235, a4236)
            } else {
                false
            } {
                let a4238 = match v1.clone() {
                    Adt1::C0(h4237, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4237
                    }
                };
                let a4239 = 4294967295u64;
                if if if if nat_le(a4238, a4239) {
                    let a4241 = match v1.clone() {
                        Adt1::C0(_, h4240, _, _, _, _, _, _, _, _, _, _) => {
                            h4240
                        }
                    };
                    let a4242 = 4294967295u64;
                    nat_le(a4241, a4242)
                } else {
                    false
                } {
                    let a4244 = match v1.clone() {
                        Adt1::C0(_, _, h4243, _, _, _, _, _, _, _, _, _) => {
                            h4243
                        }
                    };
                    let a4245 = 4294967295u64;
                    nat_le(a4244, a4245)
                } else {
                    false
                } {
                    let a4247 = match v1.clone() {
                        Adt1::C0(_, _, _, h4246, _, _, _, _, _, _, _, _) => {
                            h4246
                        }
                    };
                    let a4248 = 4294967295u64;
                    nat_le(a4247, a4248)
                } else {
                    false
                } {
                    let a4250 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4249, _, _, _, _, _, _, _) => {
                            h4249
                        }
                    };
                    let a4251 = 4294967295u64;
                    nat_le(a4250, a4251)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4254 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4252) => {
                            h4252
                        }
                    };
                    let a4255 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4253) => {
                            h4253
                        }
                    };
                    nat_eq(a4254, a4255)
                } else {
                    true
                }
            } else {
                false
            } {
                f236(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4256, _, _, _, _, _, _, _, _) => {
                        h4256
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4257, _) => {
                        h4257
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
                    let a4258 = v10;
                    let a4259 = v8;
                    nat_add(a4258, a4259)?
                }, 4u64, f25(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4260, _, _, _, _, _, _, _, _) => {
                        h4260
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f133(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4262 = match v0.clone() {
        Adt2::C0(h4261, _, _, _, _, _, _, _, _, _, _) => {
            h4261
        }
    };
    let a4263 = 0u64;
    if if if nat_eq(a4262, a4263) {
        let a4265 = match v1.clone() {
            Adt2::C0(h4264, _, _, _, _, _, _, _, _, _, _) => {
                h4264
            }
        };
        let a4266 = 0u64;
        nat_eq(a4265, a4266)
    } else {
        false
    } {
        f232(match v0.clone() {
            Adt2::C0(_, h4267, _, _, _, _, _, _, _, _, _) => {
                h4267
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4268, _, _, _, _, _, _, _, _) => {
                h4268
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4269, _, _, _, _, _, _, _) => {
                h4269
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4270, _, _, _, _, _, _) => {
                h4270
            }
        }, match v1.clone() {
            Adt2::C0(_, h4271, _, _, _, _, _, _, _, _, _) => {
                h4271
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4272, _, _, _, _, _, _, _, _) => {
                h4272
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4273, _, _, _, _, _, _, _) => {
                h4273
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4274, _, _, _, _, _, _) => {
                h4274
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4275 = v2;
            let a4276 = f23(v0.clone(), 1u64)?;
            nat_add(a4275, a4276)?
        }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f134(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4278 = match v1.clone() {
        Adt2::C0(h4277, _, _, _, _, _, _, _, _, _, _) => {
            h4277
        }
    };
    let a4279 = 0u64;
    if if if nat_eq(a4278, a4279) {
        true
    } else {
        let a4281 = match v1.clone() {
            Adt2::C0(h4280, _, _, _, _, _, _, _, _, _, _) => {
                h4280
            }
        };
        let a4282 = 1u64;
        nat_eq(a4281, a4282)
    } {
        true
    } else {
        let a4284 = match v1.clone() {
            Adt2::C0(h4283, _, _, _, _, _, _, _, _, _, _) => {
                h4283
            }
        };
        let a4285 = 30u64;
        nat_eq(a4284, a4285)
    } {
        let a4288 = match v1.clone() {
            Adt2::C0(h4286, _, _, _, _, _, _, _, _, _, _) => {
                h4286
            }
        };
        let a4289 = match v2.clone() {
            Adt2::C0(h4287, _, _, _, _, _, _, _, _, _, _) => {
                h4287
            }
        };
        if if if if if nat_eq(a4288, a4289) {
            let a4292 = match v1.clone() {
                Adt2::C0(_, h4290, _, _, _, _, _, _, _, _, _) => {
                    h4290
                }
            };
            let a4293 = match v2.clone() {
                Adt2::C0(_, h4291, _, _, _, _, _, _, _, _, _) => {
                    h4291
                }
            };
            nat_eq(a4292, a4293)
        } else {
            false
        } {
            let a4296 = match v1.clone() {
                Adt2::C0(_, _, _, h4294, _, _, _, _, _, _, _) => {
                    h4294
                }
            };
            let a4297 = match v2.clone() {
                Adt2::C0(_, _, _, h4295, _, _, _, _, _, _, _) => {
                    h4295
                }
            };
            nat_eq(a4296, a4297)
        } else {
            false
        } {
            let a4300 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4298, _, _, _, _, _, _) => {
                    h4298
                }
            };
            let a4301 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4299, _, _, _, _, _, _) => {
                    h4299
                }
            };
            nat_eq(a4300, a4301)
        } else {
            false
        } {
            let a4302 = v0;
            let a4303 = 4u64;
            if nat_eq(a4302, a4303) {
                let a4305 = match v1.clone() {
                    Adt2::C0(_, _, h4304, _, _, _, _, _, _, _, _) => {
                        h4304
                    }
                };
                let a4306 = 1u64;
                if nat_eq(a4305, a4306) {
                    let a4309 = match v1.clone() {
                        Adt2::C0(_, h4307, _, _, _, _, _, _, _, _, _) => {
                            h4307
                        }
                    };
                    let a4310 = match v2.clone() {
                        Adt2::C0(_, _, h4308, _, _, _, _, _, _, _, _) => {
                            h4308
                        }
                    };
                    nat_eq(a4309, a4310)
                } else {
                    false
                }
            } else {
                let a4311 = v0;
                let a4312 = 4u64;
                if if nat_lt(a4311, a4312) {
                    let a4315 = match v1.clone() {
                        Adt2::C0(_, h4313, _, _, _, _, _, _, _, _, _) => {
                            h4313
                        }
                    };
                    let a4316 = match v1.clone() {
                        Adt2::C0(_, _, h4314, _, _, _, _, _, _, _, _) => {
                            h4314
                        }
                    };
                    nat_eq(a4315, a4316)
                } else {
                    false
                } {
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
                }
            }
        } else {
            false
        } {
            let m4322 = f221(match v1.clone() {
                Adt2::C0(h4321, _, _, _, _, _, _, _, _, _, _) => {
                    h4321
                }
            });
            match m4322 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4323) => {
                            h4323
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

pub fn f135(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f237(match v1.clone() {
        Adt2::C0(h4324, _, _, _, _, _, _, _, _, _, _) => {
            h4324
        }
    }, v0) {
        let a4327 = match v1.clone() {
            Adt2::C0(h4325, _, _, _, _, _, _, _, _, _, _) => {
                h4325
            }
        };
        let a4328 = match v2.clone() {
            Adt2::C0(h4326, _, _, _, _, _, _, _, _, _, _) => {
                h4326
            }
        };
        nat_eq(a4327, a4328)
    } else {
        false
    } {
        let a4331 = match v1.clone() {
            Adt2::C0(_, h4329, _, _, _, _, _, _, _, _, _) => {
                h4329
            }
        };
        let a4332 = match v2.clone() {
            Adt2::C0(_, h4330, _, _, _, _, _, _, _, _, _) => {
                h4330
            }
        };
        if if if nat_eq(a4331, a4332) {
            let a4335 = match v1.clone() {
                Adt2::C0(_, _, h4333, _, _, _, _, _, _, _, _) => {
                    h4333
                }
            };
            let a4336 = match v2.clone() {
                Adt2::C0(_, _, h4334, _, _, _, _, _, _, _, _) => {
                    h4334
                }
            };
            nat_eq(a4335, a4336)
        } else {
            false
        } {
            let a4339 = match v1.clone() {
                Adt2::C0(_, _, _, h4337, _, _, _, _, _, _, _) => {
                    h4337
                }
            };
            let a4340 = match v2.clone() {
                Adt2::C0(_, _, _, h4338, _, _, _, _, _, _, _) => {
                    h4338
                }
            };
            nat_eq(a4339, a4340)
        } else {
            false
        } {
            let a4343 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4341, _, _, _, _, _, _) => {
                    h4341
                }
            };
            let a4344 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4342, _, _, _, _, _, _) => {
                    h4342
                }
            };
            nat_eq(a4343, a4344)
        } else {
            false
        }
    } else {
        false
    } {
        let m4346 = f221(match v1.clone() {
            Adt2::C0(h4345, _, _, _, _, _, _, _, _, _, _) => {
                h4345
            }
        });
        match m4346 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4347) => {
                        h4347
                    }
                }, f25(v1.clone(), 1u64), f25(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4348, _, _, _, _, _, _, _, _, _, _) => {
                        h4348
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f136(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4349, _, _, _, _, _, _, _, _, _, _, _) => {
                h4349
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4350, _, _, _, _, _, _, _, _, _) => {
                h4350
            }
        }
    };
    let a4351 = 0u64;
    let a4352 = v4;
    if if nat_lt(a4351, a4352) {
        let a4353 = v4;
        let a4354 = 2147483648u64;
        nat_lt(a4353, a4354)
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
        let v7: u64 = f238(match v5.clone() {
            Adt2::C0(_, _, h4355, _, _, _, _, _, _, _, _) => {
                h4355
            }
        }, v4)?;
        let v8: u64 = f238(match v5.clone() {
            Adt2::C0(_, _, _, h4356, _, _, _, _, _, _, _) => {
                h4356
            }
        }, v4)?;
        let a4358 = match v5.clone() {
            Adt2::C0(h4357, _, _, _, _, _, _, _, _, _, _) => {
                h4357
            }
        };
        let a4359 = 0u64;
        if if if if if nat_eq(a4358, a4359) {
            let a4361 = match v6.clone() {
                Adt2::C0(h4360, _, _, _, _, _, _, _, _, _, _) => {
                    h4360
                }
            };
            let a4362 = 0u64;
            nat_eq(a4361, a4362)
        } else {
            false
        } {
            let a4364 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4363, _, _, _, _, _, _) => {
                    h4363
                }
            };
            let a4365 = 1u64;
            nat_eq(a4364, a4365)
        } else {
            false
        } {
            let a4368 = match v5.clone() {
                Adt2::C0(_, h4366, _, _, _, _, _, _, _, _, _) => {
                    h4366
                }
            };
            let a4369 = match v6.clone() {
                Adt2::C0(_, h4367, _, _, _, _, _, _, _, _, _) => {
                    h4367
                }
            };
            nat_eq(a4368, a4369)
        } else {
            false
        } {
            if v0 {
                let a4371 = match v5.clone() {
                    Adt2::C0(_, _, h4370, _, _, _, _, _, _, _, _) => {
                        h4370
                    }
                };
                let a4372 = 2147483648u64;
                if if if nat_lt(a4371, a4372) {
                    let a4374 = match v5.clone() {
                        Adt2::C0(_, _, _, h4373, _, _, _, _, _, _, _) => {
                            h4373
                        }
                    };
                    let a4375 = 2147483648u64;
                    nat_lt(a4374, a4375)
                } else {
                    false
                } {
                    let a4378 = {
                        let a4376 = v7;
                        let a4377 = v8;
                        nat_mul(a4376, a4377)?
                    };
                    let a4379 = 2147483648u64;
                    nat_le(a4378, a4379)
                } else {
                    false
                } {
                    f239(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4381 = match v1.clone() {
                    Adt1::C0(h4380, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4380
                    }
                };
                let a4382 = v7;
                if if if if nat_eq(a4381, a4382) {
                    let a4384 = match v1 {
                        Adt1::C0(_, h4383, _, _, _, _, _, _, _, _, _, _) => {
                            h4383
                        }
                    };
                    let a4385 = v8;
                    nat_eq(a4384, a4385)
                } else {
                    false
                } {
                    f240(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4387 = match v6.clone() {
                        Adt2::C0(_, _, h4386, _, _, _, _, _, _, _, _) => {
                            h4386
                        }
                    };
                    let a4388 = v4;
                    if nat_eq(a4387, a4388) {
                        let a4390 = match v6.clone() {
                            Adt2::C0(_, _, _, h4389, _, _, _, _, _, _, _) => {
                                h4389
                            }
                        };
                        let a4391 = v4;
                        nat_eq(a4390, a4391)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4395 = match v6 {
                        Adt2::C0(_, _, _, _, h4392, _, _, _, _, _, _) => {
                            h4392
                        }
                    };
                    let a4396 = {
                        let a4393 = v7;
                        let a4394 = v8;
                        nat_mul(a4393, a4394)?
                    };
                    nat_eq(a4395, a4396)
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

pub fn f137(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4398 = match v2.clone() {
        Adt2::C0(h4397, _, _, _, _, _, _, _, _, _, _) => {
            h4397
        }
    };
    let a4399 = 0u64;
    if if if if if nat_eq(a4398, a4399) {
        true
    } else {
        let a4401 = match v2.clone() {
            Adt2::C0(h4400, _, _, _, _, _, _, _, _, _, _) => {
                h4400
            }
        };
        let a4402 = 1u64;
        nat_eq(a4401, a4402)
    } {
        true
    } else {
        let a4404 = match v2.clone() {
            Adt2::C0(h4403, _, _, _, _, _, _, _, _, _, _) => {
                h4403
            }
        };
        let a4405 = 30u64;
        nat_eq(a4404, a4405)
    } {
        let a4408 = match v2.clone() {
            Adt2::C0(h4406, _, _, _, _, _, _, _, _, _, _) => {
                h4406
            }
        };
        let a4409 = match v3.clone() {
            Adt2::C0(h4407, _, _, _, _, _, _, _, _, _, _) => {
                h4407
            }
        };
        nat_eq(a4408, a4409)
    } else {
        false
    } {
        if if f21(v2.clone(), v3.clone()) {
            let a4411 = match v1.clone() {
                Adt1::C0(h4410, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4410
                }
            };
            let a4412 = 4294967296u64;
            nat_lt(a4411, a4412)
        } else {
            false
        } {
            let a4414 = match v1.clone() {
                Adt1::C0(_, h4413, _, _, _, _, _, _, _, _, _, _) => {
                    h4413
                }
            };
            let a4415 = 4294967296u64;
            nat_lt(a4414, a4415)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4417 = match v2.clone() {
                Adt2::C0(h4416, _, _, _, _, _, _, _, _, _, _) => {
                    h4416
                }
            };
            let a4418 = 0u64;
            if nat_eq(a4417, a4418) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3, 1u64), Adt4::C13({
            let a4419 = v0;
            let a4420 = 31u64;
            if nat_eq(a4419, a4420) {
                0u64
            } else {
                let a4421 = v0;
                let a4422 = 47u64;
                if nat_eq(a4421, a4422) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4423, _, _, _, _, _, _, _, _, _, _) => {
                h4423
            }
        }, match v1.clone() {
            Adt1::C0(h4424, _, _, _, _, _, _, _, _, _, _, _) => {
                h4424
            }
        }, match v1 {
            Adt1::C0(_, h4425, _, _, _, _, _, _, _, _, _, _) => {
                h4425
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f138(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4427 = match v0.clone() {
        Adt2::C0(h4426, _, _, _, _, _, _, _, _, _, _) => {
            h4426
        }
    };
    let a4428 = 0u64;
    if if if nat_eq(a4427, a4428) {
        let a4430 = match v1.clone() {
            Adt2::C0(h4429, _, _, _, _, _, _, _, _, _, _) => {
                h4429
            }
        };
        let a4431 = 0u64;
        nat_eq(a4430, a4431)
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

pub fn f139(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4433 = match v0.clone() {
        Adt2::C0(h4432, _, _, _, _, _, _, _, _, _, _) => {
            h4432
        }
    };
    let a4434 = 0u64;
    if if nat_eq(a4433, a4434) {
        let a4436 = match v1.clone() {
            Adt2::C0(h4435, _, _, _, _, _, _, _, _, _, _) => {
                h4435
            }
        };
        let a4437 = 26u64;
        nat_eq(a4436, a4437)
    } else {
        false
    } {
        let a4439 = match v0.clone() {
            Adt2::C0(_, _, _, h4438, _, _, _, _, _, _, _) => {
                h4438
            }
        };
        let a4440 = 1u64;
        if if if if if if if nat_eq(a4439, a4440) {
            let a4442 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4441, _, _, _, _, _, _) => {
                    h4441
                }
            };
            let a4443 = 1u64;
            nat_eq(a4442, a4443)
        } else {
            false
        } {
            let a4445 = match v0.clone() {
                Adt2::C0(_, h4444, _, _, _, _, _, _, _, _, _) => {
                    h4444
                }
            };
            let a4446 = 2147483647u64;
            nat_le(a4445, a4446)
        } else {
            false
        } {
            let a4449 = match v1.clone() {
                Adt2::C0(_, h4447, _, _, _, _, _, _, _, _, _) => {
                    h4447
                }
            };
            let a4450 = match v0.clone() {
                Adt2::C0(_, _, h4448, _, _, _, _, _, _, _, _) => {
                    h4448
                }
            };
            nat_eq(a4449, a4450)
        } else {
            false
        } {
            let a4452 = match v1.clone() {
                Adt2::C0(_, _, h4451, _, _, _, _, _, _, _, _) => {
                    h4451
                }
            };
            let a4453 = 1u64;
            nat_eq(a4452, a4453)
        } else {
            false
        } {
            let a4455 = match v1.clone() {
                Adt2::C0(_, _, _, h4454, _, _, _, _, _, _, _) => {
                    h4454
                }
            };
            let a4456 = 1u64;
            nat_eq(a4455, a4456)
        } else {
            false
        } {
            let a4458 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4457, _, _, _, _, _, _) => {
                    h4457
                }
            };
            let a4459 = 1u64;
            nat_eq(a4458, a4459)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4462 = match v0.clone() {
                    Adt2::C0(_, _, h4460, _, _, _, _, _, _, _, _) => {
                        h4460
                    }
                };
                let a4463 = f182(match v0.clone() {
                    Adt2::C0(_, h4461, _, _, _, _, _, _, _, _, _) => {
                        h4461
                    }
                })?;
                nat_mul(a4462, a4463)?
            }, 4u64, f25(v0, 1u64), f25(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f140(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4464 = v1;
    match m4464 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f187(f25(v0.clone(), 1u64))?;
            let a4466 = match v0.clone() {
                Adt2::C0(h4465, _, _, _, _, _, _, _, _, _, _) => {
                    h4465
                }
            };
            let a4467 = 26u64;
            if if if nat_eq(a4466, a4467) {
                let a4469 = match v3.clone() {
                    Adt2::C0(h4468, _, _, _, _, _, _, _, _, _, _) => {
                        h4468
                    }
                };
                let a4470 = 26u64;
                nat_eq(a4469, a4470)
            } else {
                false
            } {
                let a4472 = match v2.clone() {
                    Adt2::C0(h4471, _, _, _, _, _, _, _, _, _, _) => {
                        h4471
                    }
                };
                let a4473 = 27u64;
                nat_eq(a4472, a4473)
            } else {
                false
            } {
                if if if if if if f21(v0.clone(), v3.clone()) {
                    let a4475 = match v2.clone() {
                        Adt2::C0(_, h4474, _, _, _, _, _, _, _, _, _) => {
                            h4474
                        }
                    };
                    let a4476 = 1u64;
                    nat_eq(a4475, a4476)
                } else {
                    false
                } {
                    let a4478 = match v2.clone() {
                        Adt2::C0(_, _, h4477, _, _, _, _, _, _, _, _) => {
                            h4477
                        }
                    };
                    let a4479 = 1u64;
                    nat_eq(a4478, a4479)
                } else {
                    false
                } {
                    let a4481 = match v2.clone() {
                        Adt2::C0(_, _, _, h4480, _, _, _, _, _, _, _) => {
                            h4480
                        }
                    };
                    let a4482 = 1u64;
                    nat_eq(a4481, a4482)
                } else {
                    false
                } {
                    let a4484 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4483, _, _, _, _, _, _) => {
                            h4483
                        }
                    };
                    let a4485 = 1u64;
                    nat_eq(a4484, a4485)
                } else {
                    false
                } {
                    let a4486 = v4;
                    let a4487 = 9223372036854775808u64;
                    nat_lt(a4486, a4487)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4490 = 2u64;
                        let a4491 = {
                            let a4488 = v4;
                            let a4489 = 2u64;
                            nat_mul(a4488, a4489)?
                        };
                        nat_add(a4490, a4491)?
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

pub fn f141(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4492, _, _, _, _, _, _, _, _, _, _) => {
            h4492
        }
    };
    let a4493 = v3;
    let a4494 = 0u64;
    if if nat_eq(a4493, a4494) {
        true
    } else {
        let a4495 = v0;
        let a4496 = 13u64;
        if nat_eq(a4495, a4496) {
            let a4497 = v3;
            let a4498 = 1u64;
            if nat_eq(a4497, a4498) {
                true
            } else {
                let a4499 = v3;
                let a4500 = 30u64;
                nat_eq(a4499, a4500)
            }
        } else {
            false
        }
    } {
        let a4502 = v3;
        let a4503 = match v2.clone() {
            Adt2::C0(h4501, _, _, _, _, _, _, _, _, _, _) => {
                h4501
            }
        };
        if if nat_eq(a4502, a4503) {
            let a4504 = v0;
            let a4505 = 13u64;
            if nat_eq(a4504, a4505) {
                let a4507 = match v2.clone() {
                    Adt2::C0(_, h4506, _, _, _, _, _, _, _, _, _) => {
                        h4506
                    }
                };
                let a4508 = 1u64;
                if if if nat_eq(a4507, a4508) {
                    let a4510 = match v2.clone() {
                        Adt2::C0(_, _, h4509, _, _, _, _, _, _, _, _) => {
                            h4509
                        }
                    };
                    let a4511 = 1u64;
                    nat_eq(a4510, a4511)
                } else {
                    false
                } {
                    let a4513 = match v2.clone() {
                        Adt2::C0(_, _, _, h4512, _, _, _, _, _, _, _) => {
                            h4512
                        }
                    };
                    let a4514 = 1u64;
                    nat_eq(a4513, a4514)
                } else {
                    false
                } {
                    let a4516 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4515, _, _, _, _, _, _) => {
                            h4515
                        }
                    };
                    let a4517 = 1u64;
                    nat_eq(a4516, a4517)
                } else {
                    false
                }
            } else {
                let a4519 = match v2.clone() {
                    Adt2::C0(_, h4518, _, _, _, _, _, _, _, _, _) => {
                        h4518
                    }
                };
                let a4520 = 1u64;
                if if if nat_eq(a4519, a4520) {
                    let a4523 = match v1.clone() {
                        Adt2::C0(_, _, h4521, _, _, _, _, _, _, _, _) => {
                            h4521
                        }
                    };
                    let a4524 = match v2.clone() {
                        Adt2::C0(_, _, h4522, _, _, _, _, _, _, _, _) => {
                            h4522
                        }
                    };
                    nat_eq(a4523, a4524)
                } else {
                    false
                } {
                    let a4527 = match v1.clone() {
                        Adt2::C0(_, _, _, h4525, _, _, _, _, _, _, _) => {
                            h4525
                        }
                    };
                    let a4528 = match v2.clone() {
                        Adt2::C0(_, _, _, h4526, _, _, _, _, _, _, _) => {
                            h4526
                        }
                    };
                    nat_eq(a4527, a4528)
                } else {
                    false
                } {
                    let a4531 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4529, _, _, _, _, _, _) => {
                            h4529
                        }
                    };
                    let a4532 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4530, _, _, _, _, _, _) => {
                            h4530
                        }
                    };
                    nat_eq(a4531, a4532)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f25(v1, 1u64);
            Ok(Adt10::C4(f241(v4.clone(), v0, v3)?, {
                let a4533 = v3;
                let a4534 = 0u64;
                if nat_eq(a4533, a4534) {
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

pub fn f142(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4535 = v0;
    let a4536 = 89u64;
    if if nat_eq(a4535, a4536) {
        let a4537 = v1;
        let a4538 = 5u64;
        nat_eq(a4537, a4538)
    } else {
        false
    } {
        Ok(Some(25u64))
    } else {
        let a4539 = v0;
        let a4540 = 89u64;
        if if nat_eq(a4539, a4540) {
            let a4541 = v1;
            let a4542 = 17u64;
            nat_eq(a4541, a4542)
        } else {
            false
        } {
            Ok(Some(26u64))
        } else {
            let a4543 = v0;
            let a4544 = 89u64;
            if if nat_eq(a4543, a4544) {
                let a4545 = v1;
                let a4546 = 16u64;
                nat_eq(a4545, a4546)
            } else {
                false
            } {
                Ok(Some(24u64))
            } else {
                let a4547 = v0;
                let a4548 = 89u64;
                if if nat_eq(a4547, a4548) {
                    let a4549 = v1;
                    let a4550 = 14u64;
                    nat_eq(a4549, a4550)
                } else {
                    false
                } {
                    Ok(Some(23u64))
                } else {
                    let a4551 = v0;
                    let a4552 = 89u64;
                    if if nat_eq(a4551, a4552) {
                        let a4553 = v1;
                        let a4554 = 15u64;
                        nat_eq(a4553, a4554)
                    } else {
                        false
                    } {
                        Ok(Some(22u64))
                    } else {
                        let a4555 = v0;
                        let a4556 = 89u64;
                        if if nat_eq(a4555, a4556) {
                            let a4557 = 8u64;
                            let a4558 = v1;
                            if nat_le(a4557, a4558) {
                                let a4559 = v1;
                                let a4560 = 9u64;
                                nat_le(a4559, a4560)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4561 = v1;
                                let a4562 = 12u64;
                                nat_add(a4561, a4562)?
                            }))
                        } else {
                            let a4563 = v0;
                            let a4564 = 89u64;
                            if if nat_eq(a4563, a4564) {
                                let a4565 = v1;
                                let a4566 = 4u64;
                                nat_eq(a4565, a4566)
                            } else {
                                false
                            } {
                                Ok(Some(19u64))
                            } else {
                                let a4567 = 10u64;
                                let a4568 = v0;
                                if if nat_le(a4567, a4568) {
                                    let a4569 = v0;
                                    let a4570 = 12u64;
                                    nat_le(a4569, a4570)
                                } else {
                                    false
                                } {
                                    Ok(Some({
                                        let a4571 = v0;
                                        let a4572 = 3u64;
                                        nat_add(a4571, a4572)?
                                    }))
                                } else {
                                    let a4573 = v0;
                                    let a4574 = 89u64;
                                    if if nat_eq(a4573, a4574) {
                                        let a4575 = v1;
                                        let a4576 = 7u64;
                                        nat_eq(a4575, a4576)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(17u64))
                                    } else {
                                        let a4577 = v0;
                                        let a4578 = 89u64;
                                        if if nat_eq(a4577, a4578) {
                                            let a4579 = v1;
                                            let a4580 = 10u64;
                                            nat_eq(a4579, a4580)
                                        } else {
                                            false
                                        } {
                                            Ok(Some(18u64))
                                        } else {
                                            let a4581 = v0;
                                            let a4582 = 89u64;
                                            if if nat_eq(a4581, a4582) {
                                                let a4583 = v1;
                                                let a4584 = 13u64;
                                                nat_eq(a4583, a4584)
                                            } else {
                                                false
                                            } {
                                                Ok(Some(16u64))
                                            } else {
                                                let a4585 = v0;
                                                let a4586 = 8u64;
                                                if nat_eq(a4585, a4586) {
                                                    Ok(Some(5u64))
                                                } else {
                                                    let a4587 = v0;
                                                    let a4588 = 9u64;
                                                    if nat_eq(a4587, a4588) {
                                                        Ok(Some(6u64))
                                                    } else {
                                                        let a4589 = v0;
                                                        let a4590 = 89u64;
                                                        if nat_eq(a4589, a4590) {
                                                            let a4591 = v1;
                                                            let a4592 = 0u64;
                                                            if nat_eq(a4591, a4592) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                let a4593 = v1;
                                                                let a4594 = 1u64;
                                                                if nat_eq(a4593, a4594) {
                                                                    Ok(Some(1u64))
                                                                } else {
                                                                    let a4595 = v1;
                                                                    let a4596 = 2u64;
                                                                    if nat_eq(a4595, a4596) {
                                                                        Ok(Some(2u64))
                                                                    } else {
                                                                        let a4597 = v1;
                                                                        let a4598 = 3u64;
                                                                        if nat_eq(a4597, a4598) {
                                                                            Ok(Some(3u64))
                                                                        } else {
                                                                            let a4599 = v1;
                                                                            let a4600 = 6u64;
                                                                            if nat_eq(a4599, a4600) {
                                                                                Ok(Some(4u64))
                                                                            } else {
                                                                                let a4601 = v1;
                                                                                let a4602 = 12u64;
                                                                                if nat_eq(a4601, a4602) {
                                                                                    Ok(Some(7u64))
                                                                                } else {
                                                                                    let a4603 = v1;
                                                                                    let a4604 = 11u64;
                                                                                    if nat_eq(a4603, a4604) {
                                                                                        Ok(Some(8u64))
                                                                                    } else {
                                                                                        let a4605 = v1;
                                                                                        let a4606 = 18u64;
                                                                                        if nat_eq(a4605, a4606) {
                                                                                            Ok(Some(9u64))
                                                                                        } else {
                                                                                            let a4607 = v1;
                                                                                            let a4608 = 19u64;
                                                                                            if nat_eq(a4607, a4608) {
                                                                                                Ok(Some(10u64))
                                                                                            } else {
                                                                                                let a4609 = v1;
                                                                                                let a4610 = 20u64;
                                                                                                if nat_eq(a4609, a4610) {
                                                                                                    Ok(Some(11u64))
                                                                                                } else {
                                                                                                    let a4611 = v1;
                                                                                                    let a4612 = 21u64;
                                                                                                    if nat_eq(a4611, a4612) {
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
    }
}

pub fn f143(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4613 = f242(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4613 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4614 = v0;
            let a4615 = 2u64;
            if if if if nat_eq(a4614, a4615) {
                true
            } else {
                let a4616 = v0;
                let a4617 = 5u64;
                nat_eq(a4616, a4617)
            } {
                true
            } else {
                let a4618 = v0;
                let a4619 = 6u64;
                nat_eq(a4618, a4619)
            } {
                true
            } else {
                let a4620 = v0;
                let a4621 = 7u64;
                nat_eq(a4620, a4621)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f25(v2.clone(), 1u64), f25(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4622 = v3;
                    match m4622 {
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

pub fn f144(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4624 = match v2.clone() {
        Adt2::C0(h4623, _, _, _, _, _, _, _, _, _, _) => {
            h4623
        }
    };
    let a4625 = 0u64;
    if if if if if nat_eq(a4624, a4625) {
        true
    } else {
        let a4627 = match v2.clone() {
            Adt2::C0(h4626, _, _, _, _, _, _, _, _, _, _) => {
                h4626
            }
        };
        let a4628 = 1u64;
        nat_eq(a4627, a4628)
    } {
        true
    } else {
        let a4630 = match v2.clone() {
            Adt2::C0(h4629, _, _, _, _, _, _, _, _, _, _) => {
                h4629
            }
        };
        let a4631 = 30u64;
        nat_eq(a4630, a4631)
    } {
        let a4634 = match v2.clone() {
            Adt2::C0(h4632, _, _, _, _, _, _, _, _, _, _) => {
                h4632
            }
        };
        let a4635 = match v3.clone() {
            Adt2::C0(h4633, _, _, _, _, _, _, _, _, _, _) => {
                h4633
            }
        };
        if nat_eq(a4634, a4635) {
            true
        } else {
            let a4637 = match v3.clone() {
                Adt2::C0(h4636, _, _, _, _, _, _, _, _, _, _) => {
                    h4636
                }
            };
            let a4638 = 0u64;
            nat_eq(a4637, a4638)
        }
    } else {
        false
    } {
        f21(v2.clone(), v3.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4645 = f23(v3.clone(), 1u64)?;
            let a4646 = {
                let a4639 = 25u64;
                let a4640 = v0;
                if nat_le(a4639, a4640) {
                    f178({
                        let a4641 = v0;
                        let a4642 = 26u64;
                        nat_eq(a4641, a4642)
                    }, match v3.clone() {
                        Adt2::C0(h4643, _, _, _, _, _, _, _, _, _, _) => {
                            h4643
                        }
                    })?
                } else {
                    f243(v0, match v3.clone() {
                        Adt2::C0(h4644, _, _, _, _, _, _, _, _, _, _) => {
                            h4644
                        }
                    })
                }
            };
            nat_mul(a4645, a4646)?
        }, {
            let a4648 = match v3.clone() {
                Adt2::C0(h4647, _, _, _, _, _, _, _, _, _, _) => {
                    h4647
                }
            };
            let a4649 = 0u64;
            if nat_eq(a4648, a4649) {
                4u64
            } else {
                2u64
            }
        }, f25(v2.clone(), 1u64), f25(v3.clone(), 1u64), {
            let a4650 = 25u64;
            let a4651 = v0;
            if nat_le(a4650, a4651) {
                Adt4::C45({
                    let a4652 = v0;
                    let a4653 = 26u64;
                    nat_eq(a4652, a4653)
                }, match v2 {
                    Adt2::C0(h4654, _, _, _, _, _, _, _, _, _, _) => {
                        h4654
                    }
                }, match v3 {
                    Adt2::C0(h4655, _, _, _, _, _, _, _, _, _, _) => {
                        h4655
                    }
                }, v1)
            } else {
                Adt4::C14(v0, match v2 {
                    Adt2::C0(h4656, _, _, _, _, _, _, _, _, _, _) => {
                        h4656
                    }
                }, match v3 {
                    Adt2::C0(h4657, _, _, _, _, _, _, _, _, _, _) => {
                        h4657
                    }
                })
            }
        }))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f145(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4670 = {
        let a4666 = {
            let a4662 = {
                let a4658 = v0;
                let a4659 = v4;
                nat_mul(a4658, a4659)?
            };
            let a4663 = {
                let a4660 = v1;
                let a4661 = v5;
                nat_mul(a4660, a4661)?
            };
            nat_add(a4662, a4663)?
        };
        let a4667 = {
            let a4664 = v2;
            let a4665 = v6;
            nat_mul(a4664, a4665)?
        };
        nat_add(a4666, a4667)?
    };
    let a4671 = {
        let a4668 = v3;
        let a4669 = v7;
        nat_mul(a4668, a4669)?
    };
    nat_add(a4670, a4671)
}

pub fn f146(v0: u64) -> u64 {
    let a4672 = v0;
    let a4673 = 2147483648u64;
    if nat_lt(a4672, a4673) {
        0u64
    } else {
        let a4674 = 4294967296u64;
        let a4675 = v0;
        nat_sub(a4674, a4675)
    }
}

pub fn f147(v0: u64) -> u64 {
    let a4676 = v0;
    let a4677 = 2147483648u64;
    if nat_lt(a4676, a4677) {
        v0
    } else {
        0u64
    }
}

pub fn f148(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4678 = v7;
        let a4679 = 6u64;
        if nat_eq(a4678, a4679) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f92(v3, v4), f92(v5, v6), f93(v7, v8), f179(v1), f179(v9))
}

pub fn f149(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4680 = v0;
    match m4680 {
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
        Adt4::C13(v62, v63, v64, v65) => {
            Adt11::C4(v62, v63, v64, v65)
        }
        Adt4::C14(v66, _, _) => {
            let a4681 = 13u64;
            let a4682 = v66;
            if nat_le(a4681, a4682) {
                Adt11::C7(v66, v1, v2)
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
        Adt4::C27(v86) => {
            Adt11::C2(match v86.clone() {
                Adt5::C0(_, _, _, _, _, h4683, _, _) => {
                    h4683
                }
            }, match v86 {
                Adt5::C0(_, _, _, _, _, _, _, h4684) => {
                    h4684
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
        Adt4::C45(_, _, _, _) => {
            Adt11::C6
        }
    }
}

pub fn f150(v0: Adt4) -> u64 {
    let m4685 = v0;
    match m4685 {
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
        Adt4::C14(v64, _, _) => {
            v64
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
        Adt4::C45(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f151(v0: Adt4, v1: u64) -> u64 {
    let m4686 = v0;
    match m4686 {
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
        Adt4::C16(_, v69) => {
            v69
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v72, _) => {
            let a4687 = v1;
            let a4688 = 6u64;
            if nat_eq(a4687, a4688) {
                v72
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
        Adt4::C26(v86, _, _, _) => {
            let a4697 = v1;
            let a4698 = 6u64;
            if if if nat_eq(a4697, a4698) {
                true
            } else {
                let a4699 = v1;
                let a4700 = 7u64;
                nat_eq(a4699, a4700)
            } {
                let a4701 = v86;
                let a4702 = 30u64;
                nat_eq(a4701, a4702)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v85) => {
            match v85 {
                Adt5::C0(_, _, _, _, _, _, _, h4696) => {
                    h4696
                }
            }
        }
        Adt4::C28(v81) => {
            let a4689 = v1;
            let a4690 = 6u64;
            if nat_eq(a4689, a4690) {
                match v81 {
                    Adt7::C0(_, _, _, _, h4691) => {
                        h4691
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v84) => {
            let a4692 = v1;
            let a4693 = 6u64;
            if if nat_eq(a4692, a4693) {
                true
            } else {
                let a4694 = v1;
                let a4695 = 7u64;
                nat_eq(a4694, a4695)
            } {
                v84
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
        Adt4::C45(_, _, _, _) => {
            0u64
        }
    }
}

pub fn f152(v0: Adt4, v1: u64) -> Adt11 {
    let a4703 = v1;
    let a4704 = 4u64;
    if nat_lt(a4703, a4704) {
        Adt11::C0(v1)
    } else {
        let a4705 = v1;
        let a4706 = 4u64;
        if nat_eq(a4705, a4706) {
            Adt11::C1(0u64)
        } else {
            let a4707 = v1;
            let a4708 = 5u64;
            if nat_eq(a4707, a4708) {
                Adt11::C1(3u64)
            } else {
                let a4709 = v1;
                let a4710 = 6u64;
                if nat_eq(a4709, a4710) {
                    Adt11::C2(f151(v0, v1), 0u64, false)
                } else {
                    let a4711 = v1;
                    let a4712 = 7u64;
                    if nat_eq(a4711, a4712) {
                        Adt11::C2(0u64, f151(v0, v1), false)
                    } else {
                        let a4713 = v1;
                        let a4714 = 16u64;
                        if nat_lt(a4713, a4714) {
                            Adt11::C1({
                                let a4715 = v1;
                                let a4716 = 8u64;
                                nat_sub(a4715, a4716)
                            })
                        } else {
                            let a4717 = v1;
                            let a4718 = 20u64;
                            if nat_lt(a4717, a4718) {
                                Adt11::C5({
                                    let a4719 = v1;
                                    let a4720 = 16u64;
                                    nat_sub(a4719, a4720)
                                })
                            } else {
                                let a4721 = v1;
                                let a4722 = 20u64;
                                if nat_eq(a4721, a4722) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4723 = v1;
                                    let a4724 = 21u64;
                                    if nat_eq(a4723, a4724) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4725 = v1;
                                        let a4726 = 22u64;
                                        if nat_eq(a4725, a4726) {
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

pub fn f153(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4744 = {
        let a4739 = f244(v2, {
            let a4730 = {
                let a4727 = v0;
                let a4728 = v1;
                let a4729 = 0u64;
                nat_quot(a4727, a4728, a4729)
            };
            let a4731 = v2;
            let a4732 = 0u64;
            nat_rem(a4730, a4731, a4732)
        }, {
            let a4736 = {
                let a4733 = v0;
                let a4734 = v1;
                let a4735 = 0u64;
                nat_quot(a4733, a4734, a4735)
            };
            let a4737 = v2;
            let a4738 = 0u64;
            nat_quot(a4736, a4737, a4738)
        })?;
        let a4740 = v1;
        nat_mul(a4739, a4740)?
    };
    let a4745 = {
        let a4741 = v0;
        let a4742 = v1;
        let a4743 = 0u64;
        nat_rem(a4741, a4742, a4743)
    };
    nat_add(a4744, a4745)
}

pub fn f154(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4747 = v0;
        let a4748 = match v2.clone() {
            Adt3::C0(h4746, _, _, _, _, _, _, _) => {
                h4746
            }
        };
        let a4749 = 0u64;
        nat_quot(a4747, a4748, a4749)
    };
    let v4: u64 = {
        let a4751 = v3;
        let a4752 = match v2.clone() {
            Adt3::C0(_, h4750, _, _, _, _, _, _) => {
                h4750
            }
        };
        let a4753 = 0u64;
        nat_quot(a4751, a4752, a4753)
    };
    f245({
        let a4755 = v0;
        let a4756 = match v2.clone() {
            Adt3::C0(h4754, _, _, _, _, _, _, _) => {
                h4754
            }
        };
        let a4757 = 0u64;
        nat_rem(a4755, a4756, a4757)
    }, {
        let a4759 = v3;
        let a4760 = match v2.clone() {
            Adt3::C0(_, h4758, _, _, _, _, _, _) => {
                h4758
            }
        };
        let a4761 = 0u64;
        nat_rem(a4759, a4760, a4761)
    }, {
        let a4763 = v4;
        let a4764 = match v2.clone() {
            Adt3::C0(_, _, h4762, _, _, _, _, _) => {
                h4762
            }
        };
        let a4765 = 0u64;
        nat_rem(a4763, a4764, a4765)
    }, {
        let a4767 = v4;
        let a4768 = match v2 {
            Adt3::C0(_, _, h4766, _, _, _, _, _) => {
                h4766
            }
        };
        let a4769 = 0u64;
        nat_quot(a4767, a4768, a4769)
    }, match v1.clone() {
        Adt3::C0(h4770, _, _, _, _, _, _, _) => {
            h4770
        }
    }, match v1.clone() {
        Adt3::C0(_, h4771, _, _, _, _, _, _) => {
            h4771
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4772, _, _, _, _, _) => {
            h4772
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4773, _, _, _, _) => {
            h4773
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4774, _, _, _) => {
            h4774
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4775, _, _) => {
            h4775
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4776, _) => {
            h4776
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4777) => {
            h4777
        }
    })
}

pub fn f155(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4779 = v0;
        let a4780 = match v3.clone() {
            Adt3::C0(h4778, _, _, _, _, _, _, _) => {
                h4778
            }
        };
        let a4781 = 0u64;
        nat_quot(a4779, a4780, a4781)
    };
    let v6: u64 = {
        let a4783 = v5;
        let a4784 = match v3.clone() {
            Adt3::C0(_, h4782, _, _, _, _, _, _) => {
                h4782
            }
        };
        let a4785 = 0u64;
        nat_quot(a4783, a4784, a4785)
    };
    let v7: u64 = {
        let a4787 = v0;
        let a4788 = match v3.clone() {
            Adt3::C0(h4786, _, _, _, _, _, _, _) => {
                h4786
            }
        };
        let a4789 = 0u64;
        nat_rem(a4787, a4788, a4789)
    };
    let v8: u64 = {
        let a4791 = v5;
        let a4792 = match v3.clone() {
            Adt3::C0(_, h4790, _, _, _, _, _, _) => {
                h4790
            }
        };
        let a4793 = 0u64;
        nat_rem(a4791, a4792, a4793)
    };
    let v9: u64 = {
        let a4795 = v6;
        let a4796 = match v3.clone() {
            Adt3::C0(_, _, h4794, _, _, _, _, _) => {
                h4794
            }
        };
        let a4797 = 0u64;
        nat_rem(a4795, a4796, a4797)
    };
    let v10: u64 = {
        let a4799 = v6;
        let a4800 = match v3.clone() {
            Adt3::C0(_, _, h4798, _, _, _, _, _) => {
                h4798
            }
        };
        let a4801 = 0u64;
        nat_quot(a4799, a4800, a4801)
    };
    let v11: u64 = {
        let a4802 = v4;
        let a4803 = 0u64;
        if nat_eq(a4802, a4803) {
            match v1.clone() {
                Adt3::C0(h4804, _, _, _, _, _, _, _) => {
                    h4804
                }
            }
        } else {
            let a4805 = v4;
            let a4806 = 1u64;
            if nat_eq(a4805, a4806) {
                match v1.clone() {
                    Adt3::C0(_, h4807, _, _, _, _, _, _) => {
                        h4807
                    }
                }
            } else {
                let a4808 = v4;
                let a4809 = 2u64;
                if nat_eq(a4808, a4809) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4810, _, _, _, _, _) => {
                            h4810
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4811, _, _, _, _) => {
                            h4811
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4818 = v11;
        let a4819 = {
            let a4812 = v4;
            let a4813 = 0u64;
            if nat_eq(a4812, a4813) {
                v7
            } else {
                let a4814 = v4;
                let a4815 = 1u64;
                if nat_eq(a4814, a4815) {
                    v8
                } else {
                    let a4816 = v4;
                    let a4817 = 2u64;
                    if nat_eq(a4816, a4817) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4818, a4819)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f145(if if v12 {
        let a4820 = v4;
        let a4821 = 0u64;
        nat_eq(a4820, a4821)
    } else {
        false
    } {
        let a4822 = v7;
        let a4823 = v11;
        nat_sub(a4822, a4823)
    } else {
        v7
    }, if if v12 {
        let a4824 = v4;
        let a4825 = 1u64;
        nat_eq(a4824, a4825)
    } else {
        false
    } {
        let a4826 = v8;
        let a4827 = v11;
        nat_sub(a4826, a4827)
    } else {
        v8
    }, if if v12 {
        let a4828 = v4;
        let a4829 = 2u64;
        nat_eq(a4828, a4829)
    } else {
        false
    } {
        let a4830 = v9;
        let a4831 = v11;
        nat_sub(a4830, a4831)
    } else {
        v9
    }, if if v12 {
        let a4832 = v4;
        let a4833 = 3u64;
        nat_eq(a4832, a4833)
    } else {
        false
    } {
        let a4834 = v10;
        let a4835 = v11;
        nat_sub(a4834, a4835)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h4836, _, _, _) => {
            h4836
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h4837, _, _) => {
            h4837
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h4838, _) => {
            h4838
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h4839) => {
            h4839
        }
    })?, f94(v0, v3)?))
}

pub fn f156(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f58(v0, v2.clone(), 0u64);
    let v9: u64 = f58(v0, v2.clone(), 1u64);
    let v10: u64 = f58(v0, v2.clone(), 2u64);
    let v11: u64 = f58(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f57(v1.clone(), f246(v8, v3, match v1.clone() {
            Adt3::C0(h4840, _, _, _, _, _, _, _) => {
                h4840
            }
        })?, f246(v9, v4, match v1.clone() {
            Adt3::C0(_, h4841, _, _, _, _, _, _) => {
                h4841
            }
        })?, f246(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h4842, _, _, _, _, _) => {
                h4842
            }
        })?, f246(v11, v6, match v1 {
            Adt3::C0(_, _, _, h4843, _, _, _, _) => {
                h4843
            }
        })?)?, f94(v0, v2)?))
    } else {
        if if if if f247(v8, v3, match v1.clone() {
            Adt3::C0(h4844, _, _, _, _, _, _, _) => {
                h4844
            }
        })? {
            f247(v9, v4, match v1.clone() {
                Adt3::C0(_, h4845, _, _, _, _, _, _) => {
                    h4845
                }
            })?
        } else {
            false
        } {
            f247(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h4846, _, _, _, _, _) => {
                    h4846
                }
            })?
        } else {
            false
        } {
            f247(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h4847, _, _, _, _) => {
                    h4847
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f57(v1, f248(v8, v3)?, f248(v9, v4)?, f248(v10, v5)?, f248(v11, v6)?)?, f94(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f94(v0, v2)?))
        }
    }
}

pub fn f157(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f58(v0, v2.clone(), 0u64);
    let v5: u64 = f58(v0, v2.clone(), 1u64);
    let v6: u64 = f58(v0, v2.clone(), 2u64);
    let v7: u64 = f58(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f57(v1.clone(), f249(v4, v3, match v1 {
        Adt3::C0(h4848, _, _, _, _, _, _, _) => {
            h4848
        }
    })?, v5, v6, v7)?, f94(v0, v2)?))
}

pub fn f158(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a4849 = v0;
    let a4850 = v3;
    if nat_lt(a4849, a4850) {
        Ok(Adt9::C1(false, {
            let a4851 = v0;
            let a4852 = 4u64;
            nat_mul(a4851, a4852)?
        }, {
            let a4853 = v0;
            let a4854 = 4u64;
            nat_mul(a4853, a4854)?
        }))
    } else {
        let v6: u64 = {
            let a4855 = v0;
            let a4856 = v3;
            nat_sub(a4855, a4856)
        };
        let v7: u64 = f94(v6, v1)?;
        let v8: u64 = f250(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4857 = v0;
    let a4858 = v3;
    if nat_lt(a4857, a4858) {
        Ok(Adt9::C1(true, 0u64, f94(v0, v2)?))
    } else {
        let v4: u64 = {
            let a4859 = v0;
            let a4860 = v3;
            nat_sub(a4859, a4860)
        };
        let v5: u64 = f154(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f94(v4, v1)?, 3u64, v5))
    }
}

pub fn f160(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a4862 = v1;
        let a4863 = match v3.clone() {
            Adt3::C0(h4861, _, _, _, _, _, _, _) => {
                h4861
            }
        };
        let a4864 = 0u64;
        nat_rem(a4862, a4863, a4864)
    };
    let v5: u64 = {
        let a4870 = {
            let a4866 = v1;
            let a4867 = match v3.clone() {
                Adt3::C0(h4865, _, _, _, _, _, _, _) => {
                    h4865
                }
            };
            let a4868 = 0u64;
            nat_quot(a4866, a4867, a4868)
        };
        let a4871 = match v3.clone() {
            Adt3::C0(_, h4869, _, _, _, _, _, _) => {
                h4869
            }
        };
        let a4872 = 0u64;
        nat_rem(a4870, a4871, a4872)
    };
    let v6: bool = f251(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f154(v1, v2, v3.clone())?
    } else {
        0u64
    }, f94(v1, v3)?))
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a4873 = v3;
        let a4874 = 8u64;
        nat_add(a4873, a4874)?
    }, 0u64, f94(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f154(v0, v5, v2.clone())?
    }, 3u64, f94(v0, v2)?))
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4875 = v4;
    let a4876 = 0u64;
    if if nat_eq(a4875, a4876) {
        f126(v3)
    } else {
        false
    } {
        let m4877 = f221(v3);
        match m4877 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h4878, _) => {
                        h4878
                    }
                };
                Ok(Adt9::C5(v3, f94(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h4879) => {
                        h4879
                    }
                }, f94({
                    let a4880 = v0;
                    let a4881 = v6;
                    nat_mul(a4880, a4881)?
                }, v2.clone())?, {
                    let a4887 = {
                        let a4885 = {
                            let a4882 = v6;
                            let a4883 = 1u64;
                            nat_sub(a4882, a4883)
                        };
                        let a4886 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h4884, _, _, _) => {
                                h4884
                            }
                        };
                        nat_mul(a4885, a4886)?
                    };
                    let a4888 = 4u64;
                    nat_add(a4887, a4888)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h4889, _, _, _) => {
                        h4889
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f94(v0, v1)?, f94(v0, v2)?))
    }
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a4891 = v0;
        let a4892 = match v2.clone() {
            Adt3::C0(h4890, _, _, _, _, _, _, _) => {
                h4890
            }
        };
        let a4893 = 0u64;
        nat_rem(a4891, a4892, a4893)
    };
    let v7: u64 = {
        let a4895 = v0;
        let a4896 = match v2.clone() {
            Adt3::C0(h4894, _, _, _, _, _, _, _) => {
                h4894
            }
        };
        let a4897 = 0u64;
        nat_quot(a4895, a4896, a4897)
    };
    let v8: u64 = {
        let a4899 = v7;
        let a4900 = match v2.clone() {
            Adt3::C0(_, h4898, _, _, _, _, _, _) => {
                h4898
            }
        };
        let a4901 = 0u64;
        nat_rem(a4899, a4900, a4901)
    };
    let v9: u64 = {
        let a4903 = v7;
        let a4904 = match v2.clone() {
            Adt3::C0(_, h4902, _, _, _, _, _, _) => {
                h4902
            }
        };
        let a4905 = 0u64;
        nat_quot(a4903, a4904, a4905)
    };
    let v10: u64 = {
        let a4907 = v9;
        let a4908 = match v2.clone() {
            Adt3::C0(_, _, h4906, _, _, _, _, _) => {
                h4906
            }
        };
        let a4909 = 0u64;
        nat_rem(a4907, a4908, a4909)
    };
    let v11: u64 = {
        let a4911 = v9;
        let a4912 = match v2.clone() {
            Adt3::C0(_, _, h4910, _, _, _, _, _) => {
                h4910
            }
        };
        let a4913 = 0u64;
        nat_quot(a4911, a4912, a4913)
    };
    let v12: u64 = f94(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4922 = {
                let a4917 = {
                    let a4914 = v10;
                    let a4915 = v4;
                    let a4916 = 0u64;
                    nat_quot(a4914, a4915, a4916)
                };
                let a4918 = v5;
                nat_mul(a4917, a4918)?
            };
            let a4923 = {
                let a4919 = v8;
                let a4920 = v4;
                let a4921 = 0u64;
                nat_quot(a4919, a4920, a4921)
            };
            nat_add(a4922, a4923)?
        };
        let v14: u64 = {
            let a4943 = v6;
            let a4944 = {
                let a4941 = match v1.clone() {
                    Adt3::C0(h4924, _, _, _, _, _, _, _) => {
                        h4924
                    }
                };
                let a4942 = {
                    let a4939 = {
                        let a4925 = v8;
                        let a4926 = v4;
                        let a4927 = 0u64;
                        nat_rem(a4925, a4926, a4927)
                    };
                    let a4940 = {
                        let a4937 = match v1.clone() {
                            Adt3::C0(_, h4928, _, _, _, _, _, _) => {
                                h4928
                            }
                        };
                        let a4938 = {
                            let a4935 = {
                                let a4929 = v10;
                                let a4930 = v4;
                                let a4931 = 0u64;
                                nat_rem(a4929, a4930, a4931)
                            };
                            let a4936 = {
                                let a4933 = match v1.clone() {
                                    Adt3::C0(_, _, h4932, _, _, _, _, _) => {
                                        h4932
                                    }
                                };
                                let a4934 = v13;
                                nat_mul(a4933, a4934)?
                            };
                            nat_add(a4935, a4936)?
                        };
                        nat_mul(a4937, a4938)?
                    };
                    nat_add(a4939, a4940)?
                };
                nat_mul(a4941, a4942)?
            };
            nat_add(a4943, a4944)?
        };
        Ok(Adt9::C1(false, f94(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4950 = {
                let a4948 = {
                    let a4945 = v11;
                    let a4946 = v5;
                    let a4947 = 0u64;
                    nat_rem(a4945, a4946, a4947)
                };
                let a4949 = v4;
                nat_mul(a4948, a4949)?
            };
            let a4951 = v8;
            nat_add(a4950, a4951)?
        };
        let v16: u64 = {
            let a4957 = {
                let a4955 = {
                    let a4952 = v11;
                    let a4953 = v5;
                    let a4954 = 0u64;
                    nat_quot(a4952, a4953, a4954)
                };
                let a4956 = v4;
                nat_mul(a4955, a4956)?
            };
            let a4958 = v10;
            nat_add(a4957, a4958)?
        };
        let a4960 = v15;
        let a4961 = match v1.clone() {
            Adt3::C0(_, h4959, _, _, _, _, _, _) => {
                h4959
            }
        };
        if if nat_lt(a4960, a4961) {
            let a4963 = v16;
            let a4964 = match v1.clone() {
                Adt3::C0(_, _, h4962, _, _, _, _, _) => {
                    h4962
                }
            };
            nat_lt(a4963, a4964)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f94({
                let a4973 = v6;
                let a4974 = {
                    let a4971 = match v1.clone() {
                        Adt3::C0(h4965, _, _, _, _, _, _, _) => {
                            h4965
                        }
                    };
                    let a4972 = {
                        let a4969 = v15;
                        let a4970 = {
                            let a4967 = match v1.clone() {
                                Adt3::C0(_, h4966, _, _, _, _, _, _) => {
                                    h4966
                                }
                            };
                            let a4968 = v16;
                            nat_mul(a4967, a4968)?
                        };
                        nat_add(a4969, a4970)?
                    };
                    nat_mul(a4971, a4972)?
                };
                nat_add(a4973, a4974)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f243(23u64, v4);
    let v6: u64 = {
        let a4975 = v0;
        let a4976 = v5;
        let a4977 = 0u64;
        nat_rem(a4975, a4976, a4977)
    };
    let v7: u64 = {
        let a4978 = v0;
        let a4979 = v5;
        let a4980 = 0u64;
        nat_quot(a4978, a4979, a4980)
    };
    let v8: Adt13 = Adt13::C1(f94(v7, v2)?);
    let a4981 = v6;
    let a4982 = 0u64;
    if nat_eq(a4981, a4982) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f94(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f179(v3), 4u64))
    } else {
        let a4983 = v6;
        let a4984 = 1u64;
        if nat_eq(a4983, a4984) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4985 = v4;
                let a4986 = 0u64;
                if nat_eq(a4985, a4986) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f179(v4)))
        }
    }
}

pub fn f165(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f94(v0, v2.clone())?;
    let v4: u64 = f94(v0, v1.clone())?;
    let a4991 = {
        let a4988 = v0;
        let a4989 = match v1 {
            Adt3::C0(h4987, _, _, _, _, _, _, _) => {
                h4987
            }
        };
        let a4990 = 0u64;
        nat_rem(a4988, a4989, a4990)
    };
    let a4992 = 0u64;
    if nat_eq(a4991, a4992) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f94({
            let a4993 = v0;
            let a4994 = 1u64;
            nat_sub(a4993, a4994)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4995 = v3;
    let a4996 = 13u64;
    if nat_eq(a4995, a4996) {
        f252(v0, v1, v4)
    } else {
        let v5: u64 = f253(match v1.clone() {
            Adt3::C0(h4997, _, _, _, _, _, _, _) => {
                h4997
            }
        }, {
            let a4998 = v3;
            let a4999 = 16u64;
            nat_eq(a4998, a4999)
        })?;
        let v6: u64 = {
            let a5000 = v0;
            let a5001 = v5;
            let a5002 = 0u64;
            nat_quot(a5000, a5001, a5002)
        };
        let v7: u64 = {
            let a5003 = v0;
            let a5004 = v5;
            let a5005 = 0u64;
            nat_rem(a5003, a5004, a5005)
        };
        let v8: u64 = {
            let a5009 = 1u64;
            let a5010 = {
                let a5007 = match v1.clone() {
                    Adt3::C0(h5006, _, _, _, _, _, _, _) => {
                        h5006
                    }
                };
                let a5008 = 2u64;
                nat_mul(a5007, a5008)?
            };
            nat_add(a5009, a5010)?
        };
        let a5011 = v7;
        let a5012 = v8;
        if nat_lt(a5011, a5012) {
            f254(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f94(v6, v2)?;
            let a5013 = v3;
            let a5014 = 14u64;
            if nat_eq(a5013, a5014) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a5015 = v7;
                let a5016 = v8;
                if nat_eq(a5015, a5016) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5019 = v7;
                    let a5020 = {
                        let a5017 = v8;
                        let a5018 = 1u64;
                        nat_add(a5017, a5018)?
                    };
                    if nat_eq(a5019, a5020) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h5021, _, _, _, _, _, _, _) => {
                                h5021
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

pub fn f167(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5022 = v0;
    let a5023 = 0u64;
    if nat_eq(a5022, a5023) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a5030 = {
            let a5024 = v0;
            let a5025 = 1u64;
            nat_add(a5024, a5025)?
        };
        let a5031 = {
            let a5028 = 2u64;
            let a5029 = {
                let a5026 = v3;
                let a5027 = 2u64;
                nat_mul(a5026, a5027)?
            };
            nat_add(a5028, a5029)?
        };
        if nat_eq(a5030, a5031) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a5032 = v0;
                let a5033 = 1u64;
                nat_sub(a5032, a5033)
            };
            let v5: u64 = {
                let a5034 = v4;
                let a5035 = 2u64;
                let a5036 = 0u64;
                nat_quot(a5034, a5035, a5036)
            };
            let a5040 = {
                let a5037 = v4;
                let a5038 = 2u64;
                let a5039 = 0u64;
                nat_rem(a5037, a5038, a5039)
            };
            let a5041 = 0u64;
            if nat_eq(a5040, a5041) {
                Ok(Adt9::C4(17u64, 0u64, f94(v5, v1)?, 4u64, f94(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f168(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a5042 = v0;
    let a5043 = f41(v1, v2, v3)?;
    if nat_lt(a5042, a5043) {
        let a5044 = v0;
        let a5045 = v1;
        if nat_lt(a5044, a5045) {
            Ok(Adt9::C1(false, {
                let a5046 = 4u64;
                let a5047 = v0;
                nat_mul(a5046, a5047)?
            }, {
                let a5048 = 4u64;
                let a5049 = v0;
                nat_mul(a5048, a5049)?
            }))
        } else {
            let v4: u64 = {
                let a5050 = v0;
                let a5051 = v1;
                nat_sub(a5050, a5051)
            };
            let v5: u64 = f255(v4, v2)?;
            let a5052 = v5;
            let a5053 = 2u64;
            if nat_lt(a5052, a5053) {
                Ok(Adt9::C4(6u64, {
                    let a5054 = v5;
                    let a5055 = 0u64;
                    if nat_eq(a5054, a5055) {
                        4u64
                    } else {
                        5u64
                    }
                }, f256(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f257(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f258(v4, v2)? {
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

pub fn f169(v0: u64, v1: u64) -> u64 {
    let a5056 = v0;
    let a5057 = 0u64;
    if if nat_eq(a5056, a5057) {
        true
    } else {
        let a5058 = v1;
        let a5059 = 0u64;
        nat_eq(a5058, a5059)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f170(v0: u64, v1: u64) -> Adt11 {
    let a5060 = v0;
    let a5061 = 43u64;
    if nat_eq(a5060, a5061) {
        Adt11::C8(3u64)
    } else {
        let a5062 = v1;
        let a5063 = 43u64;
        if nat_eq(a5062, a5063) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f171(v0: u64) -> R<u64> {
    let a5066 = 2u64;
    let a5067 = {
        let a5064 = 6u64;
        let a5065 = v0;
        nat_mul(a5064, a5065)?
    };
    nat_add(a5066, a5067)
}

pub fn f172(v0: u64, v1: bool) -> R<u64> {
    let a5070 = {
        let a5068 = v0;
        let a5069 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a5068, a5069)?
    };
    let a5071 = 1u64;
    nat_add(a5070, a5071)
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a5072 = v0;
    let a5073 = 16u64;
    if nat_lt(a5072, a5073) {
        let v9: u64 = {
            let a5074 = v0;
            let a5075 = 4u64;
            let a5076 = 0u64;
            nat_quot(a5074, a5075, a5076)
        };
        let v10: u64 = {
            let a5081 = {
                let a5079 = 8u64;
                let a5080 = {
                    let a5077 = 4u64;
                    let a5078 = v1;
                    nat_mul(a5077, a5078)?
                };
                nat_add(a5079, a5080)?
            };
            let a5082 = v9;
            nat_add(a5081, a5082)?
        };
        let v11: u64 = f57(v6, v9, v1, v2, 0u64)?;
        let a5086 = {
            let a5083 = v0;
            let a5084 = 4u64;
            let a5085 = 0u64;
            nat_rem(a5083, a5084, a5085)
        };
        let a5087 = 0u64;
        if nat_eq(a5086, a5087) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f57(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f57(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a5091 = {
                let a5088 = v0;
                let a5089 = 4u64;
                let a5090 = 0u64;
                nat_rem(a5088, a5089, a5090)
            };
            let a5092 = 1u64;
            if nat_eq(a5091, a5092) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f57(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a5096 = {
                    let a5093 = v0;
                    let a5094 = 4u64;
                    let a5095 = 0u64;
                    nat_rem(a5093, a5094, a5095)
                };
                let a5097 = 2u64;
                if nat_eq(a5096, a5097) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a5098 = v9;
                        let a5099 = 0u64;
                        if nat_eq(a5098, a5099) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a5100 = v8;
                        let a5101 = 2u64;
                        if nat_eq(a5100, a5101) {
                            let a5102 = v9;
                            let a5103 = 0u64;
                            if nat_eq(a5102, a5103) {
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
        let a5104 = v0;
        let a5105 = 28u64;
        if nat_lt(a5104, a5105) {
            let v12: u64 = {
                let a5106 = v0;
                let a5107 = 16u64;
                nat_sub(a5106, a5107)
            };
            let v13: u64 = {
                let a5108 = v12;
                let a5109 = 3u64;
                let a5110 = 0u64;
                nat_quot(a5108, a5109, a5110)
            };
            let v14: u64 = f57(v6, v13, v1, v2, 0u64)?;
            let a5114 = {
                let a5111 = v12;
                let a5112 = 3u64;
                let a5113 = 0u64;
                nat_rem(a5111, a5112, a5113)
            };
            let a5115 = 0u64;
            if nat_eq(a5114, a5115) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a5119 = {
                    let a5116 = v12;
                    let a5117 = 3u64;
                    let a5118 = 0u64;
                    nat_rem(a5116, a5117, a5118)
                };
                let a5120 = 1u64;
                if nat_eq(a5119, a5120) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a5121 = v13;
                        let a5122 = 0u64;
                        if nat_eq(a5121, a5122) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a5123 = v0;
            let a5124 = 28u64;
            if nat_eq(a5123, a5124) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a5125 = v0;
                    let a5126 = 29u64;
                    nat_sub(a5125, a5126)
                };
                let v16: u64 = {
                    let a5127 = v15;
                    let a5128 = 2u64;
                    let a5129 = 0u64;
                    nat_quot(a5127, a5128, a5129)
                };
                let v17: u64 = f57(v6, v16, v1, v2, 0u64)?;
                let a5133 = {
                    let a5130 = v15;
                    let a5131 = 2u64;
                    let a5132 = 0u64;
                    nat_rem(a5130, a5131, a5132)
                };
                let a5134 = 0u64;
                if nat_eq(a5133, a5134) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f174(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5135 = v0;
        let a5136 = 9u64;
        let a5137 = 0u64;
        nat_rem(a5135, a5136, a5137)
    };
    let v5: u64 = {
        let a5141 = {
            let a5138 = v0;
            let a5139 = 9u64;
            let a5140 = 0u64;
            nat_quot(a5138, a5139, a5140)
        };
        let a5142 = 4u64;
        let a5143 = 0u64;
        nat_rem(a5141, a5142, a5143)
    };
    let v6: bool = {
        let a5150 = {
            let a5147 = {
                let a5144 = v0;
                let a5145 = 36u64;
                let a5146 = 0u64;
                nat_quot(a5144, a5145, a5146)
            };
            let a5148 = 2u64;
            let a5149 = 0u64;
            nat_rem(a5147, a5148, a5149)
        };
        let a5151 = 0u64;
        nat_eq(a5150, a5151)
    };
    let v7: u64 = {
        let a5152 = v4;
        let a5153 = 4u64;
        if nat_lt(a5152, a5153) {
            v4
        } else {
            let a5154 = v4;
            let a5155 = 5u64;
            nat_sub(a5154, a5155)
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
    let a5156 = v4;
    let a5157 = 4u64;
    if nat_lt(a5156, a5157) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a5158 = v4;
            let a5159 = 0u64;
            if nat_eq(a5158, a5159) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a5160 = v4;
        let a5161 = 4u64;
        if nat_eq(a5160, a5161) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f175(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5164 = {
            let a5162 = 3u64;
            let a5163 = v1;
            nat_mul(a5162, a5163)?
        };
        let a5165 = 4u64;
        nat_add(a5164, a5165)
    } else {
        let a5166 = v1;
        let a5167 = 3u64;
        nat_add(a5166, a5167)
    }
}

pub fn f176(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5176 = f60(match v0.clone() {
        Adt7::C0(h5168, _, _, _, _) => {
            h5168
        }
    }, match v1.clone() {
        Adt3::C0(h5169, _, _, _, _, _, _, _) => {
            h5169
        }
    }, f58(v3, v2.clone(), 0u64), {
        let a5173 = v4;
        let a5174 = {
            let m5172 = match v0.clone() {
                Adt7::C0(h5171, _, _, _, _) => {
                    h5171
                }
            };
            match m5172 {
                Adt6::C0(h5170, _, _, _) => {
                    h5170
                }
            }
        };
        let a5175 = 0u64;
        nat_rem(a5173, a5174, a5175)
    })?;
    match m5176 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5185 = f60(match v0.clone() {
                Adt7::C0(_, h5177, _, _, _) => {
                    h5177
                }
            }, match v1.clone() {
                Adt3::C0(_, h5178, _, _, _, _, _, _) => {
                    h5178
                }
            }, f58(v3, v2.clone(), 1u64), {
                let a5182 = v4;
                let a5183 = {
                    let m5181 = match v0 {
                        Adt7::C0(h5180, _, _, _, _) => {
                            h5180
                        }
                    };
                    match m5181 {
                        Adt6::C0(h5179, _, _, _) => {
                            h5179
                        }
                    }
                };
                let a5184 = 0u64;
                nat_quot(a5182, a5183, a5184)
            })?;
            match m5185 {
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

pub fn f177(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5187 = v2;
        let a5188 = f147(match v0.clone() {
            Adt6::C0(_, _, h5186, _) => {
                h5186
            }
        });
        nat_add(a5187, a5188)?
    };
    let v5: u64 = {
        let a5193 = {
            let a5190 = v3;
            let a5191 = match v0.clone() {
                Adt6::C0(_, _, _, h5189) => {
                    h5189
                }
            };
            nat_mul(a5190, a5191)?
        };
        let a5194 = f146(match v0.clone() {
            Adt6::C0(_, _, h5192, _) => {
                h5192
            }
        });
        nat_add(a5193, a5194)?
    };
    let v6: u64 = {
        let a5195 = v4;
        let a5196 = v5;
        nat_sub(a5195, a5196)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5197, _, _) => {
            h5197
        }
    };
    let v8: u64 = {
        let a5198 = v6;
        let a5199 = v7;
        let a5200 = 0u64;
        nat_quot(a5198, a5199, a5200)
    };
    let a5201 = v5;
    let a5202 = v4;
    if if if nat_le(a5201, a5202) {
        let a5206 = {
            let a5203 = v6;
            let a5204 = v7;
            let a5205 = 0u64;
            nat_rem(a5203, a5204, a5205)
        };
        let a5207 = 0u64;
        nat_eq(a5206, a5207)
    } else {
        false
    } {
        let a5208 = v8;
        let a5209 = v1;
        nat_lt(a5208, a5209)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f178(v0: bool, v1: u64) -> R<u64> {
    let a5212 = if v0 {
        8u64
    } else {
        3u64
    };
    let a5213 = {
        let a5210 = v1;
        let a5211 = 0u64;
        if nat_eq(a5210, a5211) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a5212, a5213)
}

pub fn f179(v0: u64) -> u64 {
    let m5214 = f221(v0);
    match m5214 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5216 = match v1.clone() {
                Adt19::C0(_, h5215, _) => {
                    h5215
                }
            };
            let a5217 = 1u64;
            if if nat_eq(a5216, a5217) {
                let a5219 = match v1.clone() {
                    Adt19::C0(_, _, h5218) => {
                        h5218
                    }
                };
                let a5220 = 1u64;
                if if if nat_eq(a5219, a5220) {
                    true
                } else {
                    let a5222 = match v1.clone() {
                        Adt19::C0(_, _, h5221) => {
                            h5221
                        }
                    };
                    let a5223 = 2u64;
                    nat_eq(a5222, a5223)
                } {
                    true
                } else {
                    let a5225 = match v1.clone() {
                        Adt19::C0(_, _, h5224) => {
                            h5224
                        }
                    };
                    let a5226 = 4u64;
                    nat_eq(a5225, a5226)
                } {
                    true
                } else {
                    let a5228 = match v1.clone() {
                        Adt19::C0(_, _, h5227) => {
                            h5227
                        }
                    };
                    let a5229 = 8u64;
                    nat_eq(a5228, a5229)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h5230) => {
                        h5230
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f180(v0: u64, v1: u64, v2: Adt1, v3: Adt13) -> Adt9 {
    let a5231 = v0;
    let a5232 = 2u64;
    if nat_eq(a5231, a5232) {
        let a5233 = v1;
        let a5234 = 2u64;
        if nat_eq(a5233, a5234) {
            Adt9::C7(Adt11::C5(3u64), Adt12::C2(5u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64)
        } else {
            Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(match v2 {
                Adt1::C0(_, _, _, _, h5235, _, _, _, _, _, _, _) => {
                    h5235
                }
            }), Adt13::C0(2u64), 4u64, 4u64)
        }
    } else {
        let a5236 = v0;
        let a5237 = 3u64;
        if nat_eq(a5236, a5237) {
            let a5238 = v1;
            let a5239 = 5u64;
            if nat_eq(a5238, a5239) {
                Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                    Adt1::C0(_, _, h5240, _, _, _, _, _, _, _, _, _) => {
                        h5240
                    }
                }), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                Adt9::C7(Adt11::C3(0u64, 0u64, false), {
                    let a5241 = v1;
                    let a5242 = 0u64;
                    if if if nat_eq(a5241, a5242) {
                        true
                    } else {
                        let a5243 = v1;
                        let a5244 = 3u64;
                        nat_eq(a5243, a5244)
                    } {
                        true
                    } else {
                        let a5245 = v1;
                        let a5246 = 1u64;
                        if nat_eq(a5245, a5246) {
                            let a5248 = match v2.clone() {
                                Adt1::C0(_, _, _, _, h5247, _, _, _, _, _, _, _) => {
                                    h5247
                                }
                            };
                            let a5249 = 2147483648u64;
                            nat_lt(a5248, a5249)
                        } else {
                            false
                        }
                    } {
                        Adt12::C1(0u64)
                    } else {
                        Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, _, h5250, _, _, _, _, _, _, _) => {
                                h5250
                            }
                        })
                    }
                }, Adt12::C2(13u64), Adt13::C0(1u64), 4u64, 4u64)
            }
        } else {
            let a5251 = v0;
            let a5252 = 4u64;
            if nat_eq(a5251, a5252) {
                Adt9::C7({
                    let a5253 = v1;
                    let a5254 = 5u64;
                    if nat_eq(a5253, a5254) {
                        Adt11::C1(2u64)
                    } else {
                        Adt11::C1(1u64)
                    }
                }, Adt12::C1(1u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                let a5255 = v0;
                let a5256 = 5u64;
                if nat_eq(a5255, a5256) {
                    let a5257 = v1;
                    let a5258 = 5u64;
                    if nat_eq(a5257, a5258) {
                        Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64)
                    } else {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C2(match v2 {
                            Adt1::C0(_, h5259, _, _, _, _, _, _, _, _, _, _) => {
                                h5259
                            }
                        }), Adt13::C0(1u64), 4u64, 4u64)
                    }
                } else {
                    let a5260 = v0;
                    let a5261 = 6u64;
                    if nat_eq(a5260, a5261) {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, h5262, _, _, _, _, _, _, _, _) => {
                                h5262
                            }
                        }), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64)
                    } else {
                        Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C1(0u64), v3, 4u64, 4u64)
                    }
                }
            }
        }
    }
}

pub fn f181(v0: u64) -> R<u64> {
    let a5265 = v0;
    let a5266 = {
        let a5263 = v0;
        let a5264 = 2u64;
        nat_add(a5263, a5264)?
    };
    nat_mul(a5265, a5266)
}

pub fn f182(v0: u64) -> R<u64> {
    let a5269 = 3u64;
    let a5270 = {
        let a5267 = v0;
        let a5268 = 3u64;
        nat_mul(a5267, a5268)?
    };
    nat_add(a5269, a5270)
}

pub fn f183(v0: u64, v1: u64) -> R<u64> {
    if f184(v0) {
        let a5271 = 2u64;
        let a5272 = v1;
        nat_mul(a5271, a5272)
    } else {
        let a5273 = v1;
        let a5274 = f259(v0);
        let a5275 = 0u64;
        Ok(nat_quot(a5273, a5274, a5275))
    }
}

pub fn f184(v0: u64) -> bool {
    let a5276 = v0;
    let a5277 = 0u64;
    if if nat_eq(a5276, a5277) {
        true
    } else {
        let a5278 = v0;
        let a5279 = 1u64;
        nat_eq(a5278, a5279)
    } {
        true
    } else {
        let a5280 = v0;
        let a5281 = 30u64;
        nat_eq(a5280, a5281)
    }
}

pub fn f185(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m5282 = f221(v0);
    match m5282 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f145(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h5283, _, _, _) => {
                    h5283
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h5284, _, _) => {
                    h5284
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h5285, _) => {
                    h5285
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h5286) => {
                    h5286
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h5287) => {
                    h5287
                }
            }, f145({
                let a5289 = v2;
                let a5290 = match v6.clone() {
                    Adt19::C0(_, h5288, _) => {
                        h5288
                    }
                };
                nat_mul(a5289, a5290)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h5291, _, _, _) => {
                    h5291
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h5292, _, _) => {
                    h5292
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h5293, _) => {
                    h5293
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h5294) => {
                    h5294
                }
            })?, match v6 {
                Adt19::C0(_, h5295, _) => {
                    h5295
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h5296, _, _, _) => {
                    h5296
                }
            }))
        }
    }
}

pub fn f186(v0: Adt3, v1: Adt3) -> R<u64> {
    let a5304 = {
        let a5297 = 2u64;
        let a5298 = f187(v1.clone())?;
        nat_mul(a5297, a5298)?
    };
    let a5305 = {
        let a5302 = f187(v1)?;
        let a5303 = {
            let a5300 = 1u64;
            let a5301 = match v0 {
                Adt3::C0(h5299, _, _, _, _, _, _, _) => {
                    h5299
                }
            };
            nat_add(a5300, a5301)?
        };
        nat_mul(a5302, a5303)?
    };
    nat_add(a5304, a5305)
}

pub fn f187(v0: Adt3) -> R<u64> {
    if f48(match v0.clone() {
        Adt3::C0(h5306, _, _, _, _, _, _, _) => {
            h5306
        }
    }, match v0.clone() {
        Adt3::C0(_, h5307, _, _, _, _, _, _) => {
            h5307
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h5308, _, _, _, _, _) => {
            h5308
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h5309, _, _, _, _) => {
            h5309
        }
    }) {
        Ok(0u64)
    } else {
        let a5318 = match v0.clone() {
            Adt3::C0(h5310, _, _, _, _, _, _, _) => {
                h5310
            }
        };
        let a5319 = {
            let a5316 = match v0.clone() {
                Adt3::C0(_, h5311, _, _, _, _, _, _) => {
                    h5311
                }
            };
            let a5317 = {
                let a5314 = match v0.clone() {
                    Adt3::C0(_, _, h5312, _, _, _, _, _) => {
                        h5312
                    }
                };
                let a5315 = match v0 {
                    Adt3::C0(_, _, _, h5313, _, _, _, _) => {
                        h5313
                    }
                };
                nat_mul(a5314, a5315)?
            };
            nat_mul(a5316, a5317)?
        };
        nat_mul(a5318, a5319)
    }
}

pub fn f188(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f94(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f189(v0: u64, v1: u64) -> bool {
    let a5320 = v1;
    let a5321 = 2147483647u64;
    if nat_le(a5320, a5321) {
        let a5322 = v1;
        let a5323 = v0;
        nat_lt(a5322, a5323)
    } else {
        false
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5324 = f221(v0);
    match m5324 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5326 = v2;
                let a5327 = match v7.clone() {
                    Adt19::C0(_, h5325, _) => {
                        h5325
                    }
                };
                nat_mul(a5326, a5327)?
            };
            let m5340 = f260(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5328, _, _, _, _, _, _, _) => {
                    h5328
                }
            }, match v5.clone() {
                Adt3::C0(_, h5329, _, _, _, _, _, _) => {
                    h5329
                }
            }, match v5 {
                Adt3::C0(_, _, h5330, _, _, _, _, _) => {
                    h5330
                }
            }, {
                let a5333 = match v3.clone() {
                    Adt3::C0(h5331, _, _, _, _, _, _, _) => {
                        h5331
                    }
                };
                let a5334 = match v7.clone() {
                    Adt19::C0(_, h5332, _) => {
                        h5332
                    }
                };
                nat_mul(a5333, a5334)?
            }, match v3.clone() {
                Adt3::C0(_, h5335, _, _, _, _, _, _) => {
                    h5335
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5336, _, _, _) => {
                    h5336
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5337, _, _) => {
                    h5337
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5338, _) => {
                    h5338
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5339) => {
                    h5339
                }
            })?;
            match m5340 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5341 = v0;
                    let a5342 = 0u64;
                    if if nat_eq(a5341, a5342) {
                        true
                    } else {
                        let a5343 = v0;
                        let a5344 = 26u64;
                        nat_eq(a5343, a5344)
                    } {
                        Ok(Adt9::C1(false, v9, f94({
                            let a5348 = {
                                let a5346 = v1;
                                let a5347 = match v4.clone() {
                                    Adt3::C0(h5345, _, _, _, _, _, _, _) => {
                                        h5345
                                    }
                                };
                                nat_mul(a5346, a5347)?
                            };
                            let a5349 = v8;
                            nat_add(a5348, a5349)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5350) => {
                                h5350
                            }
                        }, f94({
                            let a5354 = {
                                let a5352 = v1;
                                let a5353 = match v4.clone() {
                                    Adt3::C0(h5351, _, _, _, _, _, _, _) => {
                                        h5351
                                    }
                                };
                                nat_mul(a5352, a5353)?
                            };
                            let a5355 = v8;
                            nat_add(a5354, a5355)?
                        }, v4.clone())?, {
                            let a5362 = {
                                let a5360 = {
                                    let a5357 = match v7 {
                                        Adt19::C0(_, h5356, _) => {
                                            h5356
                                        }
                                    };
                                    let a5358 = 1u64;
                                    nat_sub(a5357, a5358)
                                };
                                let a5361 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5359, _, _, _) => {
                                        h5359
                                    }
                                };
                                nat_mul(a5360, a5361)?
                            };
                            let a5363 = 4u64;
                            nat_add(a5362, a5363)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5364, _, _, _) => {
                                h5364
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f191(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5365 = v1;
    let a5366 = 0u64;
    if if nat_eq(a5365, a5366) {
        true
    } else {
        let a5367 = v2;
        let a5368 = 0u64;
        nat_eq(a5367, a5368)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f192(v0: u64, v1: u64) -> bool {
    let a5369 = v0;
    let a5370 = 4294967295u64;
    if nat_eq(a5369, a5370) {
        true
    } else {
        let a5371 = v0;
        let a5372 = 2147483648u64;
        if nat_lt(a5371, a5372) {
            let a5373 = v0;
            let a5374 = v1;
            nat_lt(a5373, a5374)
        } else {
            false
        }
    }
}

pub fn f193(v0: Adt7) -> R<u64> {
    let a5386 = if match v0.clone() {
        Adt7::C0(_, _, _, h5375, _) => {
            h5375
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5387 = {
        let a5384 = {
            let a5382 = {
                let m5378 = match v0.clone() {
                    Adt7::C0(h5377, _, _, _, _) => {
                        h5377
                    }
                };
                match m5378 {
                    Adt6::C0(h5376, _, _, _) => {
                        h5376
                    }
                }
            };
            let a5383 = {
                let m5381 = match v0.clone() {
                    Adt7::C0(_, h5380, _, _, _) => {
                        h5380
                    }
                };
                match m5381 {
                    Adt6::C0(h5379, _, _, _) => {
                        h5379
                    }
                }
            };
            nat_mul(a5382, a5383)?
        };
        let a5385 = f194(v0)?;
        nat_mul(a5384, a5385)?
    };
    nat_add(a5386, a5387)
}

pub fn f194(v0: Adt7) -> R<u64> {
    let a5392 = if match v0.clone() {
        Adt7::C0(_, _, _, h5388, _) => {
            h5388
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5393 = {
        let a5390 = match v0 {
            Adt7::C0(_, _, _, _, h5389) => {
                h5389
            }
        };
        let a5391 = 1u64;
        if nat_eq(a5390, a5391) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5392, a5393)
}

pub fn f195(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5398 = {
            let a5395 = v2;
            let a5396 = match v0.clone() {
                Adt6::C0(_, h5394, _, _) => {
                    h5394
                }
            };
            nat_mul(a5395, a5396)?
        };
        let a5399 = f146(match v0.clone() {
            Adt6::C0(_, _, h5397, _) => {
                h5397
            }
        });
        nat_add(a5398, a5399)?
    };
    let v4: u64 = f147(match v0.clone() {
        Adt6::C0(_, _, h5400, _) => {
            h5400
        }
    });
    let v5: u64 = {
        let a5404 = {
            let a5402 = v3;
            let a5403 = match v0 {
                Adt6::C0(h5401, _, _, _) => {
                    h5401
                }
            };
            nat_add(a5402, a5403)?
        };
        let a5405 = v4;
        nat_sub(a5404, a5405)
    };
    let a5410 = {
        let a5406 = v5;
        let a5407 = v1;
        if nat_lt(a5406, a5407) {
            v5
        } else {
            v1
        }
    };
    let a5411 = {
        let a5408 = v3;
        let a5409 = v4;
        nat_sub(a5408, a5409)
    };
    Ok(nat_sub(a5410, a5411))
}

pub fn f196(v0: u64) -> R<u64> {
    let a5414 = 2u64;
    let a5415 = {
        let a5412 = v0;
        let a5413 = 2u64;
        nat_mul(a5412, a5413)?
    };
    nat_add(a5414, a5415)
}

pub fn f197(v0: Adt6) -> R<u64> {
    let a5428 = 2u64;
    let a5429 = {
        let a5426 = 2u64;
        let a5427 = {
            let a5423 = {
                let a5420 = match v0.clone() {
                    Adt6::C0(h5416, _, _, _) => {
                        h5416
                    }
                };
                let a5421 = {
                    let a5418 = match v0.clone() {
                        Adt6::C0(_, h5417, _, _) => {
                            h5417
                        }
                    };
                    let a5419 = 1u64;
                    nat_sub(a5418, a5419)
                };
                nat_add(a5420, a5421)?
            };
            let a5424 = match v0 {
                Adt6::C0(_, h5422, _, _) => {
                    h5422
                }
            };
            let a5425 = 0u64;
            nat_quot(a5423, a5424, a5425)
        };
        nat_mul(a5426, a5427)?
    };
    nat_add(a5428, a5429)
}

pub fn f198(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5430 = v0;
    match m5430 {
        Adt16::C0(v19) => {
            f261(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5431 = v20;
            let a5432 = 66u64;
            if nat_eq(a5431, a5432) {
                f262(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f263(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f199(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5439 = v3;
        let a5440 = {
            let a5436 = f94({
                let a5433 = v0;
                let a5434 = 2u64;
                let a5435 = 0u64;
                nat_quot(a5433, a5434, a5435)
            }, v1)?;
            let a5437 = v2;
            let a5438 = 0u64;
            nat_quot(a5436, a5437, a5438)
        };
        nat_add(a5439, a5440)?
    };
    let a5444 = {
        let a5441 = v0;
        let a5442 = 2u64;
        let a5443 = 0u64;
        nat_rem(a5441, a5442, a5443)
    };
    let a5445 = 0u64;
    if nat_eq(a5444, a5445) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5446 = v4;
        let a5447 = 0u64;
        if nat_eq(a5446, a5447) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f200(v0: u64, v1: u64) -> R<u64> {
    let a5450 = 8u64;
    let a5451 = {
        let a5448 = f264(v0);
        let a5449 = v1;
        nat_mul(a5448, a5449)?
    };
    nat_add(a5450, a5451)
}

pub fn f201(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5452, _, _, _, _, _, _, _) => {
            h5452
        }
    };
    let v7: u64 = f264(v3);
    let v8: u64 = {
        let a5455 = 1u64;
        let a5456 = {
            let a5453 = v6;
            let a5454 = v7;
            nat_mul(a5453, a5454)?
        };
        nat_add(a5455, a5456)?
    };
    let a5457 = v0;
    let a5458 = 0u64;
    if nat_eq(a5457, a5458) {
        Ok(f265(0u64))
    } else {
        let a5459 = v0;
        let a5460 = v8;
        if nat_lt(a5459, a5460) {
            let v9: u64 = {
                let a5461 = v0;
                let a5462 = 1u64;
                nat_sub(a5461, a5462)
            };
            let v10: u64 = {
                let a5463 = v9;
                let a5464 = v7;
                let a5465 = 0u64;
                nat_rem(a5463, a5464, a5465)
            };
            let v11: u64 = f94({
                let a5471 = {
                    let a5466 = v1;
                    let a5467 = v6;
                    nat_mul(a5466, a5467)?
                };
                let a5472 = {
                    let a5468 = v9;
                    let a5469 = v7;
                    let a5470 = 0u64;
                    nat_quot(a5468, a5469, a5470)
                };
                nat_add(a5471, a5472)?
            }, v2)?;
            let a5473 = v3;
            let a5474 = 30u64;
            if nat_eq(a5473, a5474) {
                let a5475 = v10;
                let a5476 = 0u64;
                if nat_eq(a5475, a5476) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5477 = v10;
                    let a5478 = 1u64;
                    if nat_eq(a5477, a5478) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5479 = v10;
                        let a5480 = 2u64;
                        if nat_eq(a5479, a5480) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5481 = v10;
                let a5482 = 0u64;
                if nat_eq(a5481, a5482) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5483 = v10;
                    let a5484 = 1u64;
                    if nat_eq(a5483, a5484) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5485 = v0;
                let a5486 = v8;
                nat_sub(a5485, a5486)
            };
            let a5487 = v12;
            let a5488 = 0u64;
            if nat_eq(a5487, a5488) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5489 = v12;
                let a5490 = 1u64;
                if nat_eq(a5489, a5490) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5491 = v12;
                    let a5492 = 2u64;
                    if nat_eq(a5491, a5492) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5493 = v12;
                        let a5494 = 3u64;
                        if nat_eq(a5493, a5494) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5495 = v12;
                            let a5496 = 4u64;
                            if nat_eq(a5495, a5496) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5497 = v12;
                                let a5498 = 5u64;
                                if nat_eq(a5497, a5498) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5499 = v12;
                                    let a5500 = 6u64;
                                    if nat_eq(a5499, a5500) {
                                        if f266(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5501 = 3u64;
                                                let a5502 = v1;
                                                nat_add(a5501, a5502)?
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

pub fn f202(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5503 = v0;
        let a5504 = f105(v3);
        let a5505 = 0u64;
        nat_quot(a5503, a5504, a5505)
    };
    let v5: u64 = {
        let a5510 = 3u64;
        let a5511 = {
            let a5507 = v4;
            let a5508 = match v1.clone() {
                Adt3::C0(h5506, _, _, _, _, _, _, _) => {
                    h5506
                }
            };
            let a5509 = 0u64;
            nat_quot(a5507, a5508, a5509)
        };
        nat_add(a5510, a5511)?
    };
    let v6: u64 = f94(v4, v1)?;
    let v7: u64 = f94(v4, v2)?;
    let a5512 = v3;
    let a5513 = 30u64;
    if nat_eq(a5512, a5513) {
        let a5517 = {
            let a5514 = v0;
            let a5515 = 3u64;
            let a5516 = 0u64;
            nat_rem(a5514, a5515, a5516)
        };
        let a5518 = 0u64;
        if nat_eq(a5517, a5518) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5522 = {
                let a5519 = v0;
                let a5520 = 3u64;
                let a5521 = 0u64;
                nat_rem(a5519, a5520, a5521)
            };
            let a5523 = 1u64;
            if nat_eq(a5522, a5523) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f203(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5524 = v0;
        let a5525 = 8388608u64;
        let a5526 = 0u64;
        nat_quot(a5524, a5525, a5526)
    };
    let a5527 = v0;
    let a5528 = 2147483648u64;
    if if if nat_lt(a5527, a5528) {
        let a5529 = 127u64;
        let a5530 = v1;
        nat_le(a5529, a5530)
    } else {
        false
    } {
        let a5531 = v1;
        let a5532 = 190u64;
        nat_lt(a5531, a5532)
    } else {
        false
    } {
        let v2: u64 = {
            let a5536 = 8388608u64;
            let a5537 = {
                let a5533 = v0;
                let a5534 = 8388608u64;
                let a5535 = 0u64;
                nat_rem(a5533, a5534, a5535)
            };
            nat_add(a5536, a5537)?
        };
        let a5538 = v1;
        let a5539 = 150u64;
        if nat_lt(a5538, a5539) {
            let v3: u64 = f267({
                let a5540 = 150u64;
                let a5541 = v1;
                nat_sub(a5540, a5541)
            });
            let a5545 = {
                let a5542 = v2;
                let a5543 = v3;
                let a5544 = 0u64;
                nat_rem(a5542, a5543, a5544)
            };
            let a5546 = 0u64;
            if nat_eq(a5545, a5546) {
                let a5547 = v2;
                let a5548 = v3;
                let a5549 = 0u64;
                Ok(nat_quot(a5547, a5548, a5549))
            } else {
                Ok(0u64)
            }
        } else {
            let a5552 = f267({
                let a5550 = v1;
                let a5551 = 150u64;
                nat_sub(a5550, a5551)
            });
            let a5553 = v2;
            nat_mul(a5552, a5553)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f204(v0: Adt14, v1: Adt14) -> bool {
    let a5556 = match v0.clone() {
        Adt14::C0(_, h5554, _, _, _, _, _, _, _, _, _) => {
            h5554
        }
    };
    let a5557 = match v1.clone() {
        Adt14::C0(_, h5555, _, _, _, _, _, _, _, _, _) => {
            h5555
        }
    };
    if if if nat_eq(a5556, a5557) {
        let a5560 = match v0.clone() {
            Adt14::C0(_, _, h5558, _, _, _, _, _, _, _, _) => {
                h5558
            }
        };
        let a5561 = match v1.clone() {
            Adt14::C0(_, _, h5559, _, _, _, _, _, _, _, _) => {
                h5559
            }
        };
        nat_eq(a5560, a5561)
    } else {
        false
    } {
        let a5564 = match v0.clone() {
            Adt14::C0(_, _, _, h5562, _, _, _, _, _, _, _) => {
                h5562
            }
        };
        let a5565 = match v1.clone() {
            Adt14::C0(_, _, _, h5563, _, _, _, _, _, _, _) => {
                h5563
            }
        };
        nat_eq(a5564, a5565)
    } else {
        false
    } {
        let a5568 = match v0 {
            Adt14::C0(_, _, _, _, h5566, _, _, _, _, _, _) => {
                h5566
            }
        };
        let a5569 = match v1 {
            Adt14::C0(_, _, _, _, h5567, _, _, _, _, _, _) => {
                h5567
            }
        };
        nat_eq(a5568, a5569)
    } else {
        false
    }
}

pub fn f205(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5570 = v0;
    let a5571 = 0u64;
    if if if nat_eq(a5570, a5571) {
        true
    } else {
        let a5572 = v1;
        let a5573 = 0u64;
        nat_eq(a5572, a5573)
    } {
        true
    } else {
        let a5574 = v2;
        let a5575 = 0u64;
        nat_eq(a5574, a5575)
    } {
        true
    } else {
        let a5576 = v3;
        let a5577 = 0u64;
        nat_eq(a5576, a5577)
    }
}

pub fn f206(v0: Adt14, v1: Adt14) -> bool {
    if f212(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5578, _, _, _, _, _) => {
            h5578
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5579, _, _, _, _, _) => {
            h5579
        }
    }) {
        let a5581 = match v0 {
            Adt14::C0(_, h5580, _, _, _, _, _, _, _, _, _) => {
                h5580
            }
        };
        let a5582 = 2147483647u64;
        nat_le(a5581, a5582)
    } else {
        true
    }
}

pub fn f207(v0: Adt14) -> bool {
    let a5587 = {
        let a5584 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5583) => {
                h5583
            }
        };
        let a5585 = 4u64;
        let a5586 = 0u64;
        nat_rem(a5584, a5585, a5586)
    };
    let a5588 = 0u64;
    if if nat_eq(a5587, a5588) {
        f268(match v0.clone() {
            Adt14::C0(_, h5589, _, _, _, _, _, _, _, _, _) => {
                h5589
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5590, _, _, _, _, _) => {
                h5590
            }
        })
    } else {
        false
    } {
        f101(match v0.clone() {
            Adt14::C0(_, _, h5591, _, _, _, _, _, _, _, _) => {
                h5591
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5592, _, _, _, _, _, _, _) => {
                h5592
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5593, _, _, _, _, _, _) => {
                h5593
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5594, _, _, _, _) => {
                h5594
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5595, _, _, _) => {
                h5595
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5596, _, _) => {
                h5596
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f208(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5598 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5597, _) => {
            h5597
        }
    };
    let a5599 = 0u64;
    if nat_eq(a5598, a5599) {
        Ok(true)
    } else {
        let a5601 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5600, _) => {
                h5600
            }
        };
        let a5602 = 0u64;
        if nat_eq(a5601, a5602) {
            Ok(true)
        } else {
            let a5604 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5603) => {
                    h5603
                }
            };
            let a5605 = 0u64;
            if nat_eq(a5604, a5605) {
                Ok(true)
            } else {
                let a5607 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5606) => {
                        h5606
                    }
                };
                let a5608 = 0u64;
                if nat_eq(a5607, a5608) {
                    Ok(true)
                } else {
                    let a5614 = {
                        let a5611 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5609) => {
                                h5609
                            }
                        };
                        let a5612 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5610, _) => {
                                h5610
                            }
                        };
                        nat_add(a5611, a5612)?
                    };
                    let a5615 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5613) => {
                            h5613
                        }
                    };
                    if nat_le(a5614, a5615) {
                        Ok(true)
                    } else {
                        let a5621 = {
                            let a5618 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5616) => {
                                    h5616
                                }
                            };
                            let a5619 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5617, _) => {
                                    h5617
                                }
                            };
                            nat_add(a5618, a5619)?
                        };
                        let a5622 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5620) => {
                                h5620
                            }
                        };
                        Ok(nat_le(a5621, a5622))
                    }
                }
            }
        }
    }
}

pub fn f209(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f204(v0.clone(), v1.clone()) {
        let a5625 = match v0.clone() {
            Adt14::C0(h5623, _, _, _, _, _, _, _, _, _, _) => {
                h5623
            }
        };
        let a5626 = match v1.clone() {
            Adt14::C0(h5624, _, _, _, _, _, _, _, _, _, _) => {
                h5624
            }
        };
        nat_eq(a5625, a5626)
    } else {
        false
    } {
        let a5629 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5627) => {
                h5627
            }
        };
        let a5630 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5628) => {
                h5628
            }
        };
        nat_eq(a5629, a5630)
    } else {
        false
    } {
        let a5633 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5631, _, _, _, _, _) => {
                h5631
            }
        };
        let a5634 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5632, _, _, _, _, _) => {
                h5632
            }
        };
        nat_eq(a5633, a5634)
    } else {
        false
    } {
        let a5637 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5635, _, _, _, _) => {
                h5635
            }
        };
        let a5638 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5636, _, _, _, _) => {
                h5636
            }
        };
        nat_eq(a5637, a5638)
    } else {
        false
    } {
        let a5641 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5639, _, _, _) => {
                h5639
            }
        };
        let a5642 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5640, _, _, _) => {
                h5640
            }
        };
        nat_eq(a5641, a5642)
    } else {
        false
    } {
        let a5645 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5643, _, _) => {
                h5643
            }
        };
        let a5646 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5644, _, _) => {
                h5644
            }
        };
        nat_eq(a5645, a5646)
    } else {
        false
    }
}

pub fn f210(v0: Adt14) -> R<bool> {
    if f269(match v0.clone() {
        Adt14::C0(_, h5647, _, _, _, _, _, _, _, _, _) => {
            h5647
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5648, _, _, _, _, _) => {
            h5648
        }
    }) {
        f102(f270(match v0.clone() {
            Adt14::C0(_, h5649, _, _, _, _, _, _, _, _, _) => {
                h5649
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5650, _, _, _, _, _) => {
                h5650
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5651, _, _, _, _, _, _, _, _) => {
                h5651
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5652, _, _, _, _, _, _, _) => {
                h5652
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5653, _, _, _, _, _, _) => {
                h5653
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5654, _, _, _, _) => {
                h5654
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5655, _, _, _) => {
                h5655
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5656, _, _) => {
                h5656
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f211(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5657 = v1;
    let a5658 = 0u64;
    if if nat_eq(a5657, a5658) {
        true
    } else {
        let a5659 = v2;
        let a5660 = 0u64;
        nat_eq(a5659, a5660)
    } {
        Ok(0u64)
    } else {
        let a5663 = f103(v1, v2, v3)?;
        let a5664 = {
            let a5661 = v2;
            let a5662 = f271(v0, v1)?;
            nat_mul(a5661, a5662)?
        };
        nat_add(a5663, a5664)
    }
}

pub fn f212(v0: u64, v1: u64) -> bool {
    let a5665 = v0;
    let a5666 = 4u64;
    if nat_eq(a5665, a5666) {
        let a5667 = v1;
        let a5668 = 4u64;
        nat_eq(a5667, a5668)
    } else {
        false
    }
}

pub fn f213(v0: u64) -> R<u64> {
    let a5669 = 6u64;
    let a5670 = v0;
    nat_add(a5669, a5670)
}

pub fn f214(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5671, _, _, _, _, _, _, _, _, _, _) => {
            h5671
        }
    }, match v0.clone() {
        Adt14::C0(_, h5672, _, _, _, _, _, _, _, _, _) => {
            h5672
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5673, _, _, _, _, _, _, _, _) => {
            h5673
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5674, _, _, _, _, _, _, _) => {
            h5674
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5675, _, _, _, _, _, _) => {
            h5675
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5676, _, _, _, _) => {
            h5676
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5677, _, _, _) => {
            h5677
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5678, _, _) => {
            h5678
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5679, _) => {
            h5679
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5680) => {
            h5680
        }
    })
}

pub fn f215(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5681 = v0;
    let a5682 = 2147483647u64;
    if if if nat_le(a5681, a5682) {
        let a5683 = 0u64;
        let a5684 = v3;
        if nat_lt(a5683, a5684) {
            let a5685 = v3;
            let a5686 = 2147483647u64;
            nat_le(a5685, a5686)
        } else {
            false
        }
    } else {
        false
    } {
        let a5691 = {
            let a5689 = {
                let a5687 = v2;
                let a5688 = v3;
                nat_add(a5687, a5688)?
            };
            let a5690 = 1u64;
            nat_sub(a5689, a5690)
        };
        let a5692 = 2147483647u64;
        nat_le(a5691, a5692)
    } else {
        false
    } {
        let a5697 = {
            let a5695 = {
                let a5693 = v0;
                let a5694 = v1;
                nat_mul(a5693, a5694)?
            };
            let a5696 = f272(v2, v3)?;
            nat_mul(a5695, a5696)?
        };
        let a5698 = 9223372036854775807u64;
        Ok(nat_le(a5697, a5698))
    } else {
        Ok(false)
    }
}

pub fn f216(v0: Adt14) -> R<u64> {
    let a5729 = {
        let a5721 = {
            let a5713 = {
                let a5705 = 4u64;
                let a5706 = {
                    let a5703 = {
                        let a5700 = match v0.clone() {
                            Adt14::C0(_, h5699, _, _, _, _, _, _, _, _, _) => {
                                h5699
                            }
                        };
                        let a5701 = 1u64;
                        nat_sub(a5700, a5701)
                    };
                    let a5704 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5702, _, _, _, _, _) => {
                            h5702
                        }
                    };
                    nat_mul(a5703, a5704)?
                };
                nat_add(a5705, a5706)?
            };
            let a5714 = {
                let a5711 = {
                    let a5708 = match v0.clone() {
                        Adt14::C0(_, _, h5707, _, _, _, _, _, _, _, _) => {
                            h5707
                        }
                    };
                    let a5709 = 1u64;
                    nat_sub(a5708, a5709)
                };
                let a5712 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5710, _, _, _, _) => {
                        h5710
                    }
                };
                nat_mul(a5711, a5712)?
            };
            nat_add(a5713, a5714)?
        };
        let a5722 = {
            let a5719 = {
                let a5716 = match v0.clone() {
                    Adt14::C0(_, _, _, h5715, _, _, _, _, _, _, _) => {
                        h5715
                    }
                };
                let a5717 = 1u64;
                nat_sub(a5716, a5717)
            };
            let a5720 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5718, _, _, _) => {
                    h5718
                }
            };
            nat_mul(a5719, a5720)?
        };
        nat_add(a5721, a5722)?
    };
    let a5730 = {
        let a5727 = {
            let a5724 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5723, _, _, _, _, _, _) => {
                    h5723
                }
            };
            let a5725 = 1u64;
            nat_sub(a5724, a5725)
        };
        let a5728 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5726, _, _) => {
                h5726
            }
        };
        nat_mul(a5727, a5728)?
    };
    nat_add(a5729, a5730)
}

pub fn f217(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5731 = v0;
    let a5732 = 0u64;
    if if if if nat_eq(a5731, a5732) {
        true
    } else {
        let a5733 = v1;
        let a5734 = 0u64;
        nat_eq(a5733, a5734)
    } {
        true
    } else {
        let a5735 = v2;
        let a5736 = 0u64;
        nat_eq(a5735, a5736)
    } {
        true
    } else {
        let a5737 = v3;
        let a5738 = 0u64;
        nat_eq(a5737, a5738)
    } {
        Ok(0u64)
    } else {
        let a5739 = f273(v0, v1, v2, v3, v5)?;
        let a5740 = f274(v0, v1, v2, v3, v4)?;
        nat_add(a5739, a5740)
    }
}

pub fn f218(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5741 = v0;
    let a5742 = 0u64;
    if if nat_eq(a5741, a5742) {
        true
    } else {
        let a5743 = v1;
        let a5744 = 0u64;
        nat_eq(a5743, a5744)
    } {
        Ok(0u64)
    } else {
        let a5747 = f103(v0, v1, v2)?;
        let a5748 = {
            let a5745 = v1;
            let a5746 = f275(v0)?;
            nat_mul(a5745, a5746)?
        };
        nat_add(a5747, a5748)
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64) -> bool {
    let a5749 = v0;
    let a5750 = 1u64;
    if nat_le(a5749, a5750) {
        true
    } else {
        let a5754 = {
            let a5751 = v1;
            let a5752 = v2;
            let a5753 = 1u64;
            nat_rem(a5751, a5752, a5753)
        };
        let a5755 = 0u64;
        nat_eq(a5754, a5755)
    }
}

pub fn f220(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5756 = v1;
    let a5757 = 1u64;
    if if if nat_le(a5756, a5757) {
        true
    } else {
        let a5758 = v0;
        let a5759 = v2;
        nat_le(a5758, a5759)
    } {
        let a5760 = v3;
        let a5761 = 1u64;
        if nat_le(a5760, a5761) {
            true
        } else {
            let a5768 = {
                let a5766 = v0;
                let a5767 = {
                    let a5764 = {
                        let a5762 = v1;
                        let a5763 = 1u64;
                        nat_sub(a5762, a5763)
                    };
                    let a5765 = v2;
                    nat_mul(a5764, a5765)?
                };
                nat_add(a5766, a5767)?
            };
            let a5769 = v4;
            nat_le(a5768, a5769)
        }
    } else {
        false
    } {
        let a5770 = v5;
        let a5771 = 1u64;
        if nat_le(a5770, a5771) {
            Ok(true)
        } else {
            let a5784 = {
                let a5782 = {
                    let a5776 = v0;
                    let a5777 = {
                        let a5774 = {
                            let a5772 = v1;
                            let a5773 = 1u64;
                            nat_sub(a5772, a5773)
                        };
                        let a5775 = v2;
                        nat_mul(a5774, a5775)?
                    };
                    nat_add(a5776, a5777)?
                };
                let a5783 = {
                    let a5780 = {
                        let a5778 = v3;
                        let a5779 = 1u64;
                        nat_sub(a5778, a5779)
                    };
                    let a5781 = v4;
                    nat_mul(a5780, a5781)?
                };
                nat_add(a5782, a5783)?
            };
            let a5785 = v6;
            Ok(nat_le(a5784, a5785))
        }
    } else {
        Ok(false)
    }
}

pub fn f221(v0: u64) -> Adt18 {
    let a5786 = v0;
    let a5787 = 0u64;
    if nat_eq(a5786, a5787) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5788 = v0;
        let a5789 = 1u64;
        if nat_eq(a5788, a5789) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5790 = v0;
            let a5791 = 2u64;
            if nat_eq(a5790, a5791) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5792 = v0;
                let a5793 = 3u64;
                if nat_eq(a5792, a5793) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5794 = v0;
                    let a5795 = 6u64;
                    if nat_eq(a5794, a5795) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5796 = v0;
                        let a5797 = 7u64;
                        if nat_eq(a5796, a5797) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5798 = v0;
                            let a5799 = 8u64;
                            if nat_eq(a5798, a5799) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5800 = v0;
                                let a5801 = 9u64;
                                if nat_eq(a5800, a5801) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5802 = v0;
                                    let a5803 = 10u64;
                                    if nat_eq(a5802, a5803) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5804 = v0;
                                        let a5805 = 11u64;
                                        if nat_eq(a5804, a5805) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5806 = v0;
                                            let a5807 = 12u64;
                                            if nat_eq(a5806, a5807) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5808 = v0;
                                                let a5809 = 13u64;
                                                if nat_eq(a5808, a5809) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5810 = v0;
                                                    let a5811 = 14u64;
                                                    if nat_eq(a5810, a5811) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5812 = v0;
                                                        let a5813 = 15u64;
                                                        if nat_eq(a5812, a5813) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5814 = v0;
                                                            let a5815 = 16u64;
                                                            if nat_eq(a5814, a5815) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5816 = v0;
                                                                let a5817 = 17u64;
                                                                if nat_eq(a5816, a5817) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5818 = v0;
                                                                    let a5819 = 18u64;
                                                                    if nat_eq(a5818, a5819) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5820 = v0;
                                                                        let a5821 = 19u64;
                                                                        if nat_eq(a5820, a5821) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5822 = v0;
                                                                            let a5823 = 20u64;
                                                                            if nat_eq(a5822, a5823) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5824 = v0;
                                                                                let a5825 = 21u64;
                                                                                if nat_eq(a5824, a5825) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a5826 = v0;
                                                                                    let a5827 = 22u64;
                                                                                    if nat_eq(a5826, a5827) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a5828 = v0;
                                                                                        let a5829 = 23u64;
                                                                                        if nat_eq(a5828, a5829) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a5830 = v0;
                                                                                            let a5831 = 24u64;
                                                                                            if nat_eq(a5830, a5831) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a5832 = v0;
                                                                                                let a5833 = 25u64;
                                                                                                if nat_eq(a5832, a5833) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a5834 = v0;
                                                                                                    let a5835 = 26u64;
                                                                                                    if nat_eq(a5834, a5835) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a5836 = v0;
                                                                                                        let a5837 = 27u64;
                                                                                                        if nat_eq(a5836, a5837) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a5838 = v0;
                                                                                                            let a5839 = 28u64;
                                                                                                            if nat_eq(a5838, a5839) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a5840 = v0;
                                                                                                                let a5841 = 29u64;
                                                                                                                if nat_eq(a5840, a5841) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a5842 = v0;
                                                                                                                    let a5843 = 30u64;
                                                                                                                    if nat_eq(a5842, a5843) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a5844 = v0;
                                                                                                                        let a5845 = 34u64;
                                                                                                                        if nat_eq(a5844, a5845) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a5846 = v0;
                                                                                                                            let a5847 = 35u64;
                                                                                                                            if nat_eq(a5846, a5847) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a5848 = v0;
                                                                                                                                let a5849 = 39u64;
                                                                                                                                if nat_eq(a5848, a5849) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a5850 = v0;
                                                                                                                                    let a5851 = 40u64;
                                                                                                                                    if nat_eq(a5850, a5851) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a5852 = v0;
                                                                                                                                        let a5853 = 41u64;
                                                                                                                                        if nat_eq(a5852, a5853) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a5854 = v0;
                                                                                                                                            let a5855 = 42u64;
                                                                                                                                            if nat_eq(a5854, a5855) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a5856 = v0;
                                                                                                                                                let a5857 = 43u64;
                                                                                                                                                if nat_eq(a5856, a5857) {
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

pub fn f222(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a5858 = v0;
        let a5859 = 2u64;
        let a5860 = 0u64;
        nat_quot(a5858, a5859, a5860)
    };
    let a5876 = {
        let a5861 = v2;
        let a5862 = 0u64;
        if nat_eq(a5861, a5862) {
            0u64
        } else {
            let a5869 = 7u64;
            let a5870 = {
                let a5867 = v2;
                let a5868 = {
                    let a5865 = 5u64;
                    let a5866 = {
                        let a5863 = 3u64;
                        let a5864 = v1;
                        nat_mul(a5863, a5864)?
                    };
                    nat_add(a5865, a5866)?
                };
                nat_mul(a5867, a5868)?
            };
            nat_add(a5869, a5870)?
        }
    };
    let a5877 = {
        let a5874 = {
            let a5871 = v0;
            let a5872 = 2u64;
            let a5873 = 0u64;
            nat_rem(a5871, a5872, a5873)
        };
        let a5875 = 1u64;
        if nat_eq(a5874, a5875) {
            v1
        } else {
            0u64
        }
    };
    nat_add(a5876, a5877)
}

pub fn f223(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a5879 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5878, _, _, _, _, _, _) => {
                    h5878
                }
            };
            let a5880 = 1u64;
            if if if if if nat_le(a5879, a5880) {
                true
            } else {
                let a5882 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5881, _, _) => {
                        h5881
                    }
                };
                let a5883 = v1;
                nat_eq(a5882, a5883)
            } {
                let a5885 = match v0.clone() {
                    Adt2::C0(_, h5884, _, _, _, _, _, _, _, _, _) => {
                        h5884
                    }
                };
                let a5886 = 1u64;
                if nat_le(a5885, a5886) {
                    true
                } else {
                    let a5891 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5887, _, _, _, _, _) => {
                            h5887
                        }
                    };
                    let a5892 = {
                        let a5889 = v1;
                        let a5890 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h5888, _, _, _, _, _, _) => {
                                h5888
                            }
                        };
                        nat_mul(a5889, a5890)?
                    };
                    nat_eq(a5891, a5892)
                }
            } else {
                false
            } {
                let a5894 = match v0.clone() {
                    Adt2::C0(_, _, h5893, _, _, _, _, _, _, _, _) => {
                        h5893
                    }
                };
                let a5895 = 1u64;
                if nat_le(a5894, a5895) {
                    true
                } else {
                    let a5903 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5896, _, _, _, _) => {
                            h5896
                        }
                    };
                    let a5904 = {
                        let a5901 = {
                            let a5898 = v1;
                            let a5899 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h5897, _, _, _, _, _, _) => {
                                    h5897
                                }
                            };
                            nat_mul(a5898, a5899)?
                        };
                        let a5902 = match v0.clone() {
                            Adt2::C0(_, h5900, _, _, _, _, _, _, _, _, _) => {
                                h5900
                            }
                        };
                        nat_mul(a5901, a5902)?
                    };
                    nat_eq(a5903, a5904)
                }
            } else {
                false
            } {
                let a5906 = match v0.clone() {
                    Adt2::C0(_, _, _, h5905, _, _, _, _, _, _, _) => {
                        h5905
                    }
                };
                let a5907 = 1u64;
                if nat_le(a5906, a5907) {
                    true
                } else {
                    let a5918 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5908, _, _, _) => {
                            h5908
                        }
                    };
                    let a5919 = {
                        let a5916 = {
                            let a5913 = {
                                let a5910 = v1;
                                let a5911 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5909, _, _, _, _, _, _) => {
                                        h5909
                                    }
                                };
                                nat_mul(a5910, a5911)?
                            };
                            let a5914 = match v0.clone() {
                                Adt2::C0(_, h5912, _, _, _, _, _, _, _, _, _) => {
                                    h5912
                                }
                            };
                            nat_mul(a5913, a5914)?
                        };
                        let a5917 = match v0.clone() {
                            Adt2::C0(_, _, h5915, _, _, _, _, _, _, _, _) => {
                                h5915
                            }
                        };
                        nat_mul(a5916, a5917)?
                    };
                    nat_eq(a5918, a5919)
                }
            } else {
                false
            } {
                let a5933 = {
                    let a5930 = {
                        let a5927 = {
                            let a5924 = {
                                let a5921 = v1;
                                let a5922 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h5920, _, _, _, _, _, _) => {
                                        h5920
                                    }
                                };
                                nat_mul(a5921, a5922)?
                            };
                            let a5925 = match v0.clone() {
                                Adt2::C0(_, h5923, _, _, _, _, _, _, _, _, _) => {
                                    h5923
                                }
                            };
                            nat_mul(a5924, a5925)?
                        };
                        let a5928 = match v0.clone() {
                            Adt2::C0(_, _, h5926, _, _, _, _, _, _, _, _) => {
                                h5926
                            }
                        };
                        nat_mul(a5927, a5928)?
                    };
                    let a5931 = match v0.clone() {
                        Adt2::C0(_, _, _, h5929, _, _, _, _, _, _, _) => {
                            h5929
                        }
                    };
                    nat_mul(a5930, a5931)?
                };
                let a5934 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5932, _) => {
                        h5932
                    }
                };
                Ok(nat_le(a5933, a5934))
            } else {
                Ok(false)
            }
        } else {
            let a5936 = match v0.clone() {
                Adt2::C0(_, _, _, h5935, _, _, _, _, _, _, _) => {
                    h5935
                }
            };
            let a5937 = 1u64;
            if if if if if nat_le(a5936, a5937) {
                true
            } else {
                let a5939 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5938, _, _, _) => {
                        h5938
                    }
                };
                let a5940 = v1;
                nat_eq(a5939, a5940)
            } {
                let a5942 = match v0.clone() {
                    Adt2::C0(_, h5941, _, _, _, _, _, _, _, _, _) => {
                        h5941
                    }
                };
                let a5943 = 1u64;
                if nat_le(a5942, a5943) {
                    true
                } else {
                    let a5948 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5944, _, _, _, _, _) => {
                            h5944
                        }
                    };
                    let a5949 = {
                        let a5946 = v1;
                        let a5947 = match v0.clone() {
                            Adt2::C0(_, _, _, h5945, _, _, _, _, _, _, _) => {
                                h5945
                            }
                        };
                        nat_mul(a5946, a5947)?
                    };
                    nat_eq(a5948, a5949)
                }
            } else {
                false
            } {
                let a5951 = match v0.clone() {
                    Adt2::C0(_, _, h5950, _, _, _, _, _, _, _, _) => {
                        h5950
                    }
                };
                let a5952 = 1u64;
                if nat_le(a5951, a5952) {
                    true
                } else {
                    let a5960 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5953, _, _, _, _) => {
                            h5953
                        }
                    };
                    let a5961 = {
                        let a5958 = {
                            let a5955 = v1;
                            let a5956 = match v0.clone() {
                                Adt2::C0(_, _, _, h5954, _, _, _, _, _, _, _) => {
                                    h5954
                                }
                            };
                            nat_mul(a5955, a5956)?
                        };
                        let a5959 = match v0.clone() {
                            Adt2::C0(_, h5957, _, _, _, _, _, _, _, _, _) => {
                                h5957
                            }
                        };
                        nat_mul(a5958, a5959)?
                    };
                    nat_eq(a5960, a5961)
                }
            } else {
                false
            } {
                let a5963 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5962, _, _, _, _, _, _) => {
                        h5962
                    }
                };
                let a5964 = 1u64;
                if nat_le(a5963, a5964) {
                    true
                } else {
                    let a5975 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5965, _, _) => {
                            h5965
                        }
                    };
                    let a5976 = {
                        let a5973 = {
                            let a5970 = {
                                let a5967 = v1;
                                let a5968 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5966, _, _, _, _, _, _, _) => {
                                        h5966
                                    }
                                };
                                nat_mul(a5967, a5968)?
                            };
                            let a5971 = match v0.clone() {
                                Adt2::C0(_, h5969, _, _, _, _, _, _, _, _, _) => {
                                    h5969
                                }
                            };
                            nat_mul(a5970, a5971)?
                        };
                        let a5974 = match v0.clone() {
                            Adt2::C0(_, _, h5972, _, _, _, _, _, _, _, _) => {
                                h5972
                            }
                        };
                        nat_mul(a5973, a5974)?
                    };
                    nat_eq(a5975, a5976)
                }
            } else {
                false
            } {
                let a5990 = {
                    let a5987 = {
                        let a5984 = {
                            let a5981 = {
                                let a5978 = v1;
                                let a5979 = match v0.clone() {
                                    Adt2::C0(_, _, _, h5977, _, _, _, _, _, _, _) => {
                                        h5977
                                    }
                                };
                                nat_mul(a5978, a5979)?
                            };
                            let a5982 = match v0.clone() {
                                Adt2::C0(_, h5980, _, _, _, _, _, _, _, _, _) => {
                                    h5980
                                }
                            };
                            nat_mul(a5981, a5982)?
                        };
                        let a5985 = match v0.clone() {
                            Adt2::C0(_, _, h5983, _, _, _, _, _, _, _, _) => {
                                h5983
                            }
                        };
                        nat_mul(a5984, a5985)?
                    };
                    let a5988 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h5986, _, _, _, _, _, _) => {
                            h5986
                        }
                    };
                    nat_mul(a5987, a5988)?
                };
                let a5991 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5989, _) => {
                        h5989
                    }
                };
                Ok(nat_le(a5990, a5991))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5993 = match v0.clone() {
            Adt2::C0(_, h5992, _, _, _, _, _, _, _, _, _) => {
                h5992
            }
        };
        let a5994 = 1u64;
        if if if if if nat_le(a5993, a5994) {
            true
        } else {
            let a5996 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5995, _, _, _, _, _) => {
                    h5995
                }
            };
            let a5997 = v1;
            nat_eq(a5996, a5997)
        } {
            let a5999 = match v0.clone() {
                Adt2::C0(_, _, h5998, _, _, _, _, _, _, _, _) => {
                    h5998
                }
            };
            let a6000 = 1u64;
            if nat_le(a5999, a6000) {
                true
            } else {
                let a6005 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h6001, _, _, _, _) => {
                        h6001
                    }
                };
                let a6006 = {
                    let a6003 = v1;
                    let a6004 = match v0.clone() {
                        Adt2::C0(_, h6002, _, _, _, _, _, _, _, _, _) => {
                            h6002
                        }
                    };
                    nat_mul(a6003, a6004)?
                };
                nat_eq(a6005, a6006)
            }
        } else {
            false
        } {
            let a6008 = match v0.clone() {
                Adt2::C0(_, _, _, h6007, _, _, _, _, _, _, _) => {
                    h6007
                }
            };
            let a6009 = 1u64;
            if nat_le(a6008, a6009) {
                true
            } else {
                let a6017 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6010, _, _, _) => {
                        h6010
                    }
                };
                let a6018 = {
                    let a6015 = {
                        let a6012 = v1;
                        let a6013 = match v0.clone() {
                            Adt2::C0(_, h6011, _, _, _, _, _, _, _, _, _) => {
                                h6011
                            }
                        };
                        nat_mul(a6012, a6013)?
                    };
                    let a6016 = match v0.clone() {
                        Adt2::C0(_, _, h6014, _, _, _, _, _, _, _, _) => {
                            h6014
                        }
                    };
                    nat_mul(a6015, a6016)?
                };
                nat_eq(a6017, a6018)
            }
        } else {
            false
        } {
            let a6020 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6019, _, _, _, _, _, _) => {
                    h6019
                }
            };
            let a6021 = 1u64;
            if nat_le(a6020, a6021) {
                true
            } else {
                let a6032 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6022, _, _) => {
                        h6022
                    }
                };
                let a6033 = {
                    let a6030 = {
                        let a6027 = {
                            let a6024 = v1;
                            let a6025 = match v0.clone() {
                                Adt2::C0(_, h6023, _, _, _, _, _, _, _, _, _) => {
                                    h6023
                                }
                            };
                            nat_mul(a6024, a6025)?
                        };
                        let a6028 = match v0.clone() {
                            Adt2::C0(_, _, h6026, _, _, _, _, _, _, _, _) => {
                                h6026
                            }
                        };
                        nat_mul(a6027, a6028)?
                    };
                    let a6031 = match v0.clone() {
                        Adt2::C0(_, _, _, h6029, _, _, _, _, _, _, _) => {
                            h6029
                        }
                    };
                    nat_mul(a6030, a6031)?
                };
                nat_eq(a6032, a6033)
            }
        } else {
            false
        } {
            let a6047 = {
                let a6044 = {
                    let a6041 = {
                        let a6038 = {
                            let a6035 = v1;
                            let a6036 = match v0.clone() {
                                Adt2::C0(_, h6034, _, _, _, _, _, _, _, _, _) => {
                                    h6034
                                }
                            };
                            nat_mul(a6035, a6036)?
                        };
                        let a6039 = match v0.clone() {
                            Adt2::C0(_, _, h6037, _, _, _, _, _, _, _, _) => {
                                h6037
                            }
                        };
                        nat_mul(a6038, a6039)?
                    };
                    let a6042 = match v0.clone() {
                        Adt2::C0(_, _, _, h6040, _, _, _, _, _, _, _) => {
                            h6040
                        }
                    };
                    nat_mul(a6041, a6042)?
                };
                let a6045 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6043, _, _, _, _, _, _) => {
                        h6043
                    }
                };
                nat_mul(a6044, a6045)?
            };
            let a6048 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h6046, _) => {
                    h6046
                }
            };
            Ok(nat_le(a6047, a6048))
        } else {
            Ok(false)
        }
    }
}

pub fn f224(v0: Adt2, v1: u64) -> R<bool> {
    let a6050 = match v0.clone() {
        Adt2::C0(h6049, _, _, _, _, _, _, _, _, _, _) => {
            h6049
        }
    };
    let a6051 = 0u64;
    if if nat_eq(a6050, a6051) {
        f38(v0.clone())?
    } else {
        false
    } {
        let a6052 = v1;
        let a6053 = 1u64;
        if nat_le(a6052, a6053) {
            Ok(true)
        } else {
            let a6058 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6054, _, _, _, _) => {
                    h6054
                }
            };
            let a6059 = {
                let a6056 = 4u64;
                let a6057 = match v0 {
                    Adt2::C0(_, h6055, _, _, _, _, _, _, _, _, _) => {
                        h6055
                    }
                };
                nat_mul(a6056, a6057)?
            };
            Ok(nat_eq(a6058, a6059))
        }
    } else {
        Ok(false)
    }
}

pub fn f225(v0: Adt2, v1: u64) -> R<bool> {
    let a6061 = match v0.clone() {
        Adt2::C0(_, _, _, h6060, _, _, _, _, _, _, _) => {
            h6060
        }
    };
    let a6062 = 1u64;
    if if if nat_eq(a6061, a6062) {
        let a6064 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6063, _, _, _, _, _, _) => {
                h6063
            }
        };
        let a6065 = 1u64;
        nat_eq(a6064, a6065)
    } else {
        false
    } {
        let a6067 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h6066, _, _, _, _, _) => {
                h6066
            }
        };
        let a6068 = v1;
        nat_eq(a6067, a6068)
    } else {
        false
    } {
        let a6070 = match v0.clone() {
            Adt2::C0(_, _, h6069, _, _, _, _, _, _, _, _) => {
                h6069
            }
        };
        let a6071 = 1u64;
        if nat_eq(a6070, a6071) {
            Ok(true)
        } else {
            let a6076 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6072, _, _, _, _) => {
                    h6072
                }
            };
            let a6077 = {
                let a6074 = match v0 {
                    Adt2::C0(_, h6073, _, _, _, _, _, _, _, _, _) => {
                        h6073
                    }
                };
                let a6075 = v1;
                nat_mul(a6074, a6075)?
            };
            Ok(nat_eq(a6076, a6077))
        }
    } else {
        Ok(false)
    }
}

pub fn f226(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a6081 = {
            let a6078 = v2;
            let a6079 = 1u64;
            nat_sub(a6078, a6079)
        };
        let a6082 = match v0.clone() {
            Adt6::C0(_, h6080, _, _) => {
                h6080
            }
        };
        nat_mul(a6081, a6082)?
    };
    let a6084 = 0u64;
    let a6085 = match v0.clone() {
        Adt6::C0(h6083, _, _, _) => {
            h6083
        }
    };
    if if if if if if if if nat_lt(a6084, a6085) {
        let a6087 = match v0.clone() {
            Adt6::C0(h6086, _, _, _) => {
                h6086
            }
        };
        let a6088 = 2147483648u64;
        nat_lt(a6087, a6088)
    } else {
        false
    } {
        let a6090 = 0u64;
        let a6091 = match v0.clone() {
            Adt6::C0(_, h6089, _, _) => {
                h6089
            }
        };
        nat_lt(a6090, a6091)
    } else {
        false
    } {
        let a6093 = match v0.clone() {
            Adt6::C0(_, h6092, _, _) => {
                h6092
            }
        };
        let a6094 = 2147483648u64;
        nat_lt(a6093, a6094)
    } else {
        false
    } {
        let a6095 = v1;
        let a6096 = 2147483648u64;
        nat_lt(a6095, a6096)
    } else {
        false
    } {
        let a6097 = 0u64;
        let a6098 = v2;
        nat_lt(a6097, a6098)
    } else {
        false
    } {
        let a6099 = v3;
        let a6100 = 2147483648u64;
        nat_lt(a6099, a6100)
    } else {
        false
    } {
        let a6112 = {
            let a6107 = {
                let a6104 = v3;
                let a6105 = {
                    let a6102 = match v0.clone() {
                        Adt6::C0(h6101, _, _, _) => {
                            h6101
                        }
                    };
                    let a6103 = 1u64;
                    nat_sub(a6102, a6103)
                };
                nat_add(a6104, a6105)?
            };
            let a6108 = f146(match v0.clone() {
                Adt6::C0(_, _, h6106, _) => {
                    h6106
                }
            });
            nat_add(a6107, a6108)?
        };
        let a6113 = {
            let a6110 = 2147483647u64;
            let a6111 = f147(match v0.clone() {
                Adt6::C0(_, _, h6109, _) => {
                    h6109
                }
            });
            nat_add(a6110, a6111)?
        };
        nat_le(a6112, a6113)
    } else {
        false
    } {
        let a6115 = match v0 {
            Adt6::C0(_, _, h6114, _) => {
                h6114
            }
        };
        let a6116 = 2147483648u64;
        Ok(!nat_eq(a6115, a6116))
    } else {
        Ok(false)
    }
}

pub fn f227(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f276(match v0.clone() {
        Adt2::C0(h6117, _, _, _, _, _, _, _, _, _, _) => {
            h6117
        }
    }, match v0.clone() {
        Adt2::C0(_, h6118, _, _, _, _, _, _, _, _, _) => {
            h6118
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6119, _, _, _, _, _, _, _, _) => {
            h6119
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6120, _, _, _, _, _, _, _) => {
            h6120
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6121, _, _, _, _, _, _) => {
            h6121
        }
    }, match v1.clone() {
        Adt2::C0(h6122, _, _, _, _, _, _, _, _, _, _) => {
            h6122
        }
    }, match v1.clone() {
        Adt2::C0(_, h6123, _, _, _, _, _, _, _, _, _) => {
            h6123
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h6124, _, _, _, _, _, _, _, _) => {
            h6124
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h6125, _, _, _, _, _, _, _) => {
            h6125
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h6126, _, _, _, _, _, _) => {
            h6126
        }
    }, match v2.clone() {
        Adt2::C0(h6127, _, _, _, _, _, _, _, _, _, _) => {
            h6127
        }
    }, match v2.clone() {
        Adt2::C0(_, h6128, _, _, _, _, _, _, _, _, _) => {
            h6128
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h6129, _, _, _, _, _, _, _, _) => {
            h6129
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h6130, _, _, _, _, _, _, _) => {
            h6130
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h6131, _, _, _, _, _, _) => {
            h6131
        }
    }) {
        let m6133 = f221(match v0.clone() {
            Adt2::C0(h6132, _, _, _, _, _, _, _, _, _, _) => {
                h6132
            }
        });
        match m6133 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a6139 = {
                    let a6136 = match v0.clone() {
                        Adt2::C0(_, h6134, _, _, _, _, _, _, _, _, _) => {
                            h6134
                        }
                    };
                    let a6137 = match v3.clone() {
                        Adt19::C0(_, h6135, _) => {
                            h6135
                        }
                    };
                    let a6138 = 0u64;
                    nat_rem(a6136, a6137, a6138)
                };
                let a6140 = 0u64;
                if if nat_eq(a6139, a6140) {
                    if f277(match v0.clone() {
                        Adt2::C0(_, h6141, _, _, _, _, _, _, _, _, _) => {
                            h6141
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6142, _, _, _, _, _, _, _, _, _) => {
                            h6142
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6143, _, _, _, _, _, _, _, _) => {
                            h6143
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6144, _, _, _, _, _, _, _) => {
                            h6144
                        }
                    }) {
                        true
                    } else {
                        let a6146 = 4u64;
                        let a6147 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6145, _, _, _, _, _) => {
                                h6145
                            }
                        };
                        if if nat_le(a6146, a6147) {
                            let a6149 = match v3.clone() {
                                Adt19::C0(_, h6148, _) => {
                                    h6148
                                }
                            };
                            let a6150 = 1u64;
                            if nat_eq(a6149, a6150) {
                                true
                            } else {
                                let a6153 = match v3.clone() {
                                    Adt19::C0(_, _, h6151) => {
                                        h6151
                                    }
                                };
                                let a6154 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h6152, _, _, _, _, _) => {
                                        h6152
                                    }
                                };
                                nat_le(a6153, a6154)
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
                        Adt19::C0(_, h6155, _) => {
                            h6155
                        }
                    });
                    Ok(Adt10::C4(if f277(match v0.clone() {
                        Adt2::C0(_, h6156, _, _, _, _, _, _, _, _, _) => {
                            h6156
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6157, _, _, _, _, _, _, _, _, _) => {
                            h6157
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6158, _, _, _, _, _, _, _, _) => {
                            h6158
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6159, _, _, _, _, _, _, _) => {
                            h6159
                        }
                    }) {
                        0u64
                    } else {
                        f186(v4.clone(), f25(v1.clone(), 1u64))?
                    }, 4u64, v4, f25(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h6160, _, _, _, _, _, _, _, _, _, _) => {
                            h6160
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

pub fn f228(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f229(match v0.clone() {
        Adt2::C0(h6161, _, _, _, _, _, _, _, _, _, _) => {
            h6161
        }
    }) {
        let a6167 = {
            let a6164 = match v0.clone() {
                Adt2::C0(_, h6162, _, _, _, _, _, _, _, _, _) => {
                    h6162
                }
            };
            let a6165 = f259(match v0.clone() {
                Adt2::C0(h6163, _, _, _, _, _, _, _, _, _, _) => {
                    h6163
                }
            });
            let a6166 = 1u64;
            nat_rem(a6164, a6165, a6166)
        };
        let a6168 = 0u64;
        nat_eq(a6167, a6168)
    } else {
        false
    } {
        let a6170 = match v1.clone() {
            Adt2::C0(h6169, _, _, _, _, _, _, _, _, _, _) => {
                h6169
            }
        };
        let a6171 = 26u64;
        nat_eq(a6170, a6171)
    } else {
        false
    } {
        let a6173 = match v2.clone() {
            Adt2::C0(h6172, _, _, _, _, _, _, _, _, _, _) => {
                h6172
            }
        };
        let a6174 = 0u64;
        nat_eq(a6173, a6174)
    } else {
        false
    } {
        let a6177 = match v0.clone() {
            Adt2::C0(_, h6175, _, _, _, _, _, _, _, _, _) => {
                h6175
            }
        };
        let a6178 = match v2.clone() {
            Adt2::C0(_, h6176, _, _, _, _, _, _, _, _, _) => {
                h6176
            }
        };
        nat_eq(a6177, a6178)
    } else {
        false
    } {
        let a6181 = match v1.clone() {
            Adt2::C0(_, h6179, _, _, _, _, _, _, _, _, _) => {
                h6179
            }
        };
        let a6182 = match v0.clone() {
            Adt2::C0(_, _, h6180, _, _, _, _, _, _, _, _) => {
                h6180
            }
        };
        nat_le(a6181, a6182)
    } else {
        false
    } {
        let a6184 = match v0.clone() {
            Adt2::C0(_, _, _, h6183, _, _, _, _, _, _, _) => {
                h6183
            }
        };
        let a6185 = 1u64;
        nat_eq(a6184, a6185)
    } else {
        false
    } {
        let a6187 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6186, _, _, _, _, _, _) => {
                h6186
            }
        };
        let a6188 = 1u64;
        nat_eq(a6187, a6188)
    } else {
        false
    } {
        let a6190 = match v2.clone() {
            Adt2::C0(_, _, _, h6189, _, _, _, _, _, _, _) => {
                h6189
            }
        };
        let a6191 = 1u64;
        nat_eq(a6190, a6191)
    } else {
        false
    } {
        let a6193 = match v2.clone() {
            Adt2::C0(_, _, _, _, h6192, _, _, _, _, _, _) => {
                h6192
            }
        };
        let a6194 = 1u64;
        nat_eq(a6193, a6194)
    } else {
        false
    } {
        let a6196 = match v1.clone() {
            Adt2::C0(_, _, h6195, _, _, _, _, _, _, _, _) => {
                h6195
            }
        };
        let a6197 = 1u64;
        nat_eq(a6196, a6197)
    } else {
        false
    } {
        let a6199 = match v1.clone() {
            Adt2::C0(_, _, _, h6198, _, _, _, _, _, _, _) => {
                h6198
            }
        };
        let a6200 = 1u64;
        nat_eq(a6199, a6200)
    } else {
        false
    } {
        let a6202 = match v1.clone() {
            Adt2::C0(_, _, _, _, h6201, _, _, _, _, _, _) => {
                h6201
            }
        };
        let a6203 = 1u64;
        nat_eq(a6202, a6203)
    } else {
        false
    } {
        if f48(match v2.clone() {
            Adt2::C0(_, h6204, _, _, _, _, _, _, _, _, _) => {
                h6204
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h6205, _, _, _, _, _, _, _, _) => {
                h6205
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h6206, _, _, _, _, _, _, _) => {
                h6206
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h6207, _, _, _, _, _, _) => {
                h6207
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a6209 = match v1.clone() {
                Adt2::C0(_, h6208, _, _, _, _, _, _, _, _, _) => {
                    h6208
                }
            };
            let a6210 = 0u64;
            if nat_eq(a6209, a6210) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f25(v0, 1u64), f25(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a6212 = match v0.clone() {
                    Adt2::C0(_, h6211, _, _, _, _, _, _, _, _, _) => {
                        h6211
                    }
                };
                let a6213 = 2147483647u64;
                if if if if if if nat_le(a6212, a6213) {
                    let a6215 = match v1.clone() {
                        Adt2::C0(_, h6214, _, _, _, _, _, _, _, _, _) => {
                            h6214
                        }
                    };
                    let a6216 = 2147483647u64;
                    nat_le(a6215, a6216)
                } else {
                    false
                } {
                    let a6218 = 4u64;
                    let a6219 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h6217, _, _, _, _, _) => {
                            h6217
                        }
                    };
                    nat_le(a6218, a6219)
                } else {
                    false
                } {
                    let a6225 = {
                        let a6222 = match v2.clone() {
                            Adt2::C0(_, h6220, _, _, _, _, _, _, _, _, _) => {
                                h6220
                            }
                        };
                        let a6223 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6221, _, _, _, _, _) => {
                                h6221
                            }
                        };
                        nat_mul(a6222, a6223)?
                    };
                    let a6226 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6224, _, _, _, _) => {
                            h6224
                        }
                    };
                    nat_le(a6225, a6226)
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
                        let a6239 = {
                            let a6230 = {
                                let a6228 = 2u64;
                                let a6229 = match v1.clone() {
                                    Adt2::C0(_, h6227, _, _, _, _, _, _, _, _, _) => {
                                        h6227
                                    }
                                };
                                nat_mul(a6228, a6229)?
                            };
                            let a6231 = f23(v2.clone(), 1u64)?;
                            nat_add(a6230, a6231)?
                        };
                        let a6240 = {
                            let a6237 = match v1.clone() {
                                Adt2::C0(_, h6232, _, _, _, _, _, _, _, _, _) => {
                                    h6232
                                }
                            };
                            let a6238 = {
                                let a6235 = 1u64;
                                let a6236 = f183(match v0.clone() {
                                    Adt2::C0(h6233, _, _, _, _, _, _, _, _, _, _) => {
                                        h6233
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h6234, _, _, _, _, _, _, _, _, _) => {
                                        h6234
                                    }
                                })?;
                                nat_add(a6235, a6236)?
                            };
                            nat_mul(a6237, a6238)?
                        };
                        nat_add(a6239, a6240)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h6241, _, _, _, _, _, _, _, _, _) => {
                            h6241
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6242, _, _, _, _, _, _, _, _, _) => {
                            h6242
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6243, _, _, _, _, _) => {
                            h6243
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6244, _, _, _, _) => {
                            h6244
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6245, _, _, _) => {
                            h6245
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6246, _, _) => {
                            h6246
                        }
                    }), f25(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h6247, _, _, _, _, _, _, _, _, _, _) => {
                            h6247
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

pub fn f229(v0: u64) -> bool {
    let a6248 = v0;
    let a6249 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a6248, a6249) {
        true
    } else {
        let a6250 = v0;
        let a6251 = 1u64;
        nat_eq(a6250, a6251)
    } {
        true
    } else {
        let a6252 = v0;
        let a6253 = 2u64;
        nat_eq(a6252, a6253)
    } {
        true
    } else {
        let a6254 = v0;
        let a6255 = 3u64;
        nat_eq(a6254, a6255)
    } {
        true
    } else {
        let a6256 = v0;
        let a6257 = 6u64;
        nat_eq(a6256, a6257)
    } {
        true
    } else {
        let a6258 = v0;
        let a6259 = 7u64;
        nat_eq(a6258, a6259)
    } {
        true
    } else {
        let a6260 = v0;
        let a6261 = 8u64;
        nat_eq(a6260, a6261)
    } {
        true
    } else {
        let a6262 = v0;
        let a6263 = 10u64;
        nat_eq(a6262, a6263)
    } {
        true
    } else {
        let a6264 = v0;
        let a6265 = 11u64;
        nat_eq(a6264, a6265)
    } {
        true
    } else {
        let a6266 = v0;
        let a6267 = 12u64;
        nat_eq(a6266, a6267)
    } {
        true
    } else {
        let a6268 = v0;
        let a6269 = 13u64;
        nat_eq(a6268, a6269)
    } {
        true
    } else {
        let a6270 = v0;
        let a6271 = 14u64;
        nat_eq(a6270, a6271)
    } {
        true
    } else {
        let a6272 = v0;
        let a6273 = 16u64;
        nat_eq(a6272, a6273)
    } {
        true
    } else {
        let a6274 = v0;
        let a6275 = 17u64;
        nat_eq(a6274, a6275)
    } {
        true
    } else {
        let a6276 = v0;
        let a6277 = 18u64;
        nat_eq(a6276, a6277)
    } {
        true
    } else {
        let a6278 = v0;
        let a6279 = 19u64;
        nat_eq(a6278, a6279)
    } {
        true
    } else {
        let a6280 = v0;
        let a6281 = 20u64;
        nat_eq(a6280, a6281)
    } {
        true
    } else {
        let a6282 = v0;
        let a6283 = 21u64;
        nat_eq(a6282, a6283)
    } {
        true
    } else {
        let a6284 = v0;
        let a6285 = 22u64;
        nat_eq(a6284, a6285)
    } {
        true
    } else {
        let a6286 = v0;
        let a6287 = 23u64;
        nat_eq(a6286, a6287)
    } {
        true
    } else {
        let a6288 = v0;
        let a6289 = 26u64;
        nat_eq(a6288, a6289)
    } {
        true
    } else {
        let a6290 = v0;
        let a6291 = 29u64;
        nat_eq(a6290, a6291)
    } {
        true
    } else {
        let a6292 = v0;
        let a6293 = 30u64;
        nat_eq(a6292, a6293)
    } {
        true
    } else {
        let a6294 = v0;
        let a6295 = 34u64;
        nat_eq(a6294, a6295)
    } {
        true
    } else {
        let a6296 = v0;
        let a6297 = 35u64;
        nat_eq(a6296, a6297)
    } {
        true
    } else {
        let a6298 = v0;
        let a6299 = 39u64;
        nat_eq(a6298, a6299)
    } {
        true
    } else {
        let a6300 = v0;
        let a6301 = 40u64;
        nat_eq(a6300, a6301)
    } {
        true
    } else {
        let a6302 = v0;
        let a6303 = 41u64;
        nat_eq(a6302, a6303)
    } {
        true
    } else {
        let a6304 = v0;
        let a6305 = 42u64;
        nat_eq(a6304, a6305)
    } {
        true
    } else {
        let a6306 = v0;
        let a6307 = 43u64;
        nat_eq(a6306, a6307)
    }
}

pub fn f230(v0: u64) -> bool {
    if f184(v0) {
        true
    } else {
        let a6308 = v0;
        let a6309 = 43u64;
        nat_eq(a6308, a6309)
    }
}

pub fn f231(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a6312 = match v1.clone() {
        Adt2::C0(h6310, _, _, _, _, _, _, _, _, _, _) => {
            h6310
        }
    };
    let a6313 = match v3.clone() {
        Adt2::C0(h6311, _, _, _, _, _, _, _, _, _, _) => {
            h6311
        }
    };
    if if nat_eq(a6312, a6313) {
        let a6314 = v0;
        let a6315 = 33u64;
        if nat_eq(a6314, a6315) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m6317 = f221(match v1.clone() {
            Adt2::C0(h6316, _, _, _, _, _, _, _, _, _, _) => {
                h6316
            }
        });
        match m6317 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h6318, _) => {
                        h6318
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a6323 = {
                    let a6320 = match v1.clone() {
                        Adt2::C0(_, h6319, _, _, _, _, _, _, _, _, _) => {
                            h6319
                        }
                    };
                    let a6321 = v5;
                    let a6322 = 1u64;
                    nat_rem(a6320, a6321, a6322)
                };
                let a6324 = 0u64;
                if if if nat_eq(a6323, a6324) {
                    let a6329 = {
                        let a6326 = match v3.clone() {
                            Adt2::C0(_, h6325, _, _, _, _, _, _, _, _, _) => {
                                h6325
                            }
                        };
                        let a6327 = v5;
                        let a6328 = 1u64;
                        nat_rem(a6326, a6327, a6328)
                    };
                    let a6330 = 0u64;
                    nat_eq(a6329, a6330)
                } else {
                    false
                } {
                    let a6331 = v6;
                    let a6332 = f23(v3.clone(), v5)?;
                    nat_eq(a6331, a6332)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6333) => {
                            h6333
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

pub fn f232(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f48(v4, v5, v6, v7) {
        f48(v0, v1, v2, v3)
    } else {
        let a6337 = {
            let a6334 = v0;
            let a6335 = v4;
            let a6336 = 1u64;
            nat_rem(a6334, a6335, a6336)
        };
        let a6338 = 0u64;
        if if if nat_eq(a6337, a6338) {
            let a6342 = {
                let a6339 = v1;
                let a6340 = v5;
                let a6341 = 1u64;
                nat_rem(a6339, a6340, a6341)
            };
            let a6343 = 0u64;
            nat_eq(a6342, a6343)
        } else {
            false
        } {
            let a6347 = {
                let a6344 = v2;
                let a6345 = v6;
                let a6346 = 1u64;
                nat_rem(a6344, a6345, a6346)
            };
            let a6348 = 0u64;
            nat_eq(a6347, a6348)
        } else {
            false
        } {
            let a6352 = {
                let a6349 = v3;
                let a6350 = v7;
                let a6351 = 1u64;
                nat_rem(a6349, a6350, a6351)
            };
            let a6353 = 0u64;
            nat_eq(a6352, a6353)
        } else {
            false
        }
    }
}

pub fn f233(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6354 = v0;
    let a6355 = v1;
    if nat_eq(a6354, a6355) {
        let a6358 = v4;
        let a6359 = {
            let a6356 = v2;
            let a6357 = v3;
            nat_add(a6356, a6357)?
        };
        Ok(nat_eq(a6358, a6359))
    } else {
        let a6360 = v2;
        let a6361 = v3;
        if nat_eq(a6360, a6361) {
            let a6362 = v4;
            let a6363 = v2;
            Ok(nat_eq(a6362, a6363))
        } else {
            Ok(false)
        }
    }
}

pub fn f234(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6366 = {
            let a6364 = v0;
            let a6365 = f147(v1);
            nat_add(a6364, a6365)?
        };
        let a6367 = f147(v2);
        nat_add(a6366, a6367)?
    };
    let v5: u64 = {
        let a6368 = f146(v1);
        let a6369 = f146(v2);
        nat_add(a6368, a6369)?
    };
    let a6370 = v1;
    let a6371 = 4294967295u64;
    if if nat_le(a6370, a6371) {
        let a6372 = v2;
        let a6373 = 4294967295u64;
        nat_le(a6372, a6373)
    } else {
        false
    } {
        let a6374 = v5;
        let a6375 = v4;
        if nat_le(a6374, a6375) {
            let a6378 = v3;
            let a6379 = {
                let a6376 = v4;
                let a6377 = v5;
                nat_sub(a6376, a6377)
            };
            Ok(nat_eq(a6378, a6379))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f235(v0: u64, v1: u64) -> R<bool> {
    let a6380 = v1;
    let a6381 = 4294967295u64;
    if nat_le(a6380, a6381) {
        let a6384 = {
            let a6382 = f147(v1);
            let a6383 = f146(v1);
            nat_add(a6382, a6383)?
        };
        let a6385 = v0;
        Ok(nat_lt(a6384, a6385))
    } else {
        Ok(false)
    }
}

pub fn f236(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f278(v0.clone(), false)?;
    let v6: u64 = f278(v0, true)?;
    let a6386 = v6;
    let a6387 = v1;
    if if nat_le(a6386, a6387) {
        let a6394 = {
            let a6392 = {
                let a6388 = v1;
                let a6389 = v5;
                nat_add(a6388, a6389)?
            };
            let a6393 = {
                let a6390 = v3;
                let a6391 = 0u64;
                if nat_eq(a6390, a6391) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6392, a6393)?
        };
        let a6395 = v2;
        nat_le(a6394, a6395)
    } else {
        false
    } {
        if v4 {
            let a6400 = {
                let a6398 = {
                    let a6396 = v1;
                    let a6397 = v5;
                    nat_add(a6396, a6397)?
                };
                let a6399 = v6;
                nat_sub(a6398, a6399)
            };
            let a6401 = v2;
            Ok(nat_lt(a6400, a6401))
        } else {
            let a6406 = {
                let a6404 = {
                    let a6402 = v1;
                    let a6403 = v5;
                    nat_add(a6402, a6403)?
                };
                let a6405 = v6;
                nat_sub(a6404, a6405)
            };
            let a6407 = v2;
            Ok(nat_le(a6406, a6407))
        }
    } else {
        Ok(false)
    }
}

pub fn f237(v0: u64, v1: u64) -> bool {
    let a6408 = v0;
    let a6409 = 0u64;
    if if nat_eq(a6408, a6409) {
        true
    } else {
        let a6410 = v0;
        let a6411 = 1u64;
        nat_eq(a6410, a6411)
    } {
        let a6412 = v1;
        let a6413 = 4294967296u64;
        nat_lt(a6412, a6413)
    } else {
        false
    }
}

pub fn f238(v0: u64, v1: u64) -> R<u64> {
    let a6418 = {
        let a6416 = v0;
        let a6417 = {
            let a6414 = v1;
            let a6415 = 1u64;
            nat_sub(a6414, a6415)
        };
        nat_add(a6416, a6417)?
    };
    let a6419 = v1;
    let a6420 = 0u64;
    Ok(nat_quot(a6418, a6419, a6420))
}

pub fn f239(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f48(match v0.clone() {
        Adt2::C0(_, h6421, _, _, _, _, _, _, _, _, _) => {
            h6421
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6422, _, _, _, _, _, _, _, _) => {
            h6422
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6423, _, _, _, _, _, _, _) => {
            h6423
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6424, _, _, _, _, _, _) => {
            h6424
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6427 = match v1.clone() {
                Adt2::C0(_, h6425, _, _, _, _, _, _, _, _, _) => {
                    h6425
                }
            };
            let a6428 = match v1.clone() {
                Adt2::C0(_, _, h6426, _, _, _, _, _, _, _, _) => {
                    h6426
                }
            };
            nat_mul(a6427, a6428)?
        };
        let v5: u64 = {
            let a6430 = v4;
            let a6431 = match v1.clone() {
                Adt2::C0(_, _, _, h6429, _, _, _, _, _, _, _) => {
                    h6429
                }
            };
            nat_mul(a6430, a6431)?
        };
        let a6444 = {
            let a6442 = {
                let a6433 = match v0.clone() {
                    Adt2::C0(_, h6432, _, _, _, _, _, _, _, _, _) => {
                        h6432
                    }
                };
                let a6434 = 1u64;
                nat_sub(a6433, a6434)
            };
            let a6443 = {
                let a6440 = f279(match v0.clone() {
                    Adt2::C0(_, _, h6435, _, _, _, _, _, _, _, _) => {
                        h6435
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6436, _, _, _, _, _, _, _, _, _) => {
                        h6436
                    }
                })?;
                let a6441 = f279(match v0 {
                    Adt2::C0(_, _, _, h6437, _, _, _, _, _, _, _) => {
                        h6437
                    }
                }, v2, {
                    let a6438 = v3;
                    let a6439 = v5;
                    nat_mul(a6438, a6439)?
                }, v4)?;
                nat_add(a6440, a6441)?
            };
            nat_add(a6442, a6443)?
        };
        let a6445 = f23(v1, 1u64)?;
        Ok(nat_lt(a6444, a6445))
    }
}

pub fn f240(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6446 = v0;
    let a6447 = 2147483648u64;
    if if if if nat_lt(a6446, a6447) {
        let a6448 = v1;
        let a6449 = 2147483648u64;
        nat_lt(a6448, a6449)
    } else {
        false
    } {
        let a6452 = {
            let a6450 = v0;
            let a6451 = v1;
            nat_mul(a6450, a6451)?
        };
        let a6453 = 2147483648u64;
        nat_lt(a6452, a6453)
    } else {
        false
    } {
        let a6458 = {
            let a6456 = {
                let a6454 = v0;
                let a6455 = 1u64;
                nat_sub(a6454, a6455)
            };
            let a6457 = v2;
            nat_mul(a6456, a6457)?
        };
        let a6459 = 2147483648u64;
        nat_lt(a6458, a6459)
    } else {
        false
    } {
        let a6464 = {
            let a6462 = {
                let a6460 = v1;
                let a6461 = 1u64;
                nat_sub(a6460, a6461)
            };
            let a6463 = v2;
            nat_mul(a6462, a6463)?
        };
        let a6465 = 2147483648u64;
        Ok(nat_lt(a6464, a6465))
    } else {
        Ok(false)
    }
}

pub fn f241(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6466 = v1;
    let a6467 = 13u64;
    if if nat_eq(a6466, a6467) {
        f48(match v0.clone() {
            Adt3::C0(h6468, _, _, _, _, _, _, _) => {
                h6468
            }
        }, match v0.clone() {
            Adt3::C0(_, h6469, _, _, _, _, _, _) => {
                h6469
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6470, _, _, _, _, _) => {
                h6470
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6471, _, _, _, _) => {
                h6471
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6477 = match v0.clone() {
                Adt3::C0(_, h6472, _, _, _, _, _, _) => {
                    h6472
                }
            };
            let a6478 = {
                let a6475 = match v0.clone() {
                    Adt3::C0(_, _, h6473, _, _, _, _, _) => {
                        h6473
                    }
                };
                let a6476 = match v0.clone() {
                    Adt3::C0(_, _, _, h6474, _, _, _, _) => {
                        h6474
                    }
                };
                nat_mul(a6475, a6476)?
            };
            nat_mul(a6477, a6478)?
        };
        let a6479 = v1;
        let a6480 = 13u64;
        if nat_eq(a6479, a6480) {
            let a6489 = 2u64;
            let a6490 = if f280(v0.clone(), v2) {
                let a6482 = v3;
                let a6483 = f253(match v0 {
                    Adt3::C0(h6481, _, _, _, _, _, _, _) => {
                        h6481
                    }
                }, false)?;
                nat_mul(a6482, a6483)?
            } else {
                let a6487 = v3;
                let a6488 = {
                    let a6485 = match v0 {
                        Adt3::C0(h6484, _, _, _, _, _, _, _) => {
                            h6484
                        }
                    };
                    let a6486 = 2u64;
                    nat_mul(a6485, a6486)?
                };
                nat_mul(a6487, a6488)?
            };
            nat_add(a6489, a6490)
        } else {
            let a6494 = v3;
            let a6495 = f253(match v0 {
                Adt3::C0(h6491, _, _, _, _, _, _, _) => {
                    h6491
                }
            }, {
                let a6492 = v1;
                let a6493 = 16u64;
                nat_eq(a6492, a6493)
            })?;
            nat_mul(a6494, a6495)
        }
    }
}

pub fn f242(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6496 = v0;
    let a6497 = 30u64;
    if if nat_eq(a6496, a6497) {
        let a6500 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6498, _, _, _, _) => {
                h6498
            }
        };
        let a6501 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6499, _, _, _, _, _) => {
                h6499
            }
        };
        nat_lt(a6500, a6501)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6502 = v3;
        match m6502 {
            None => {
                f281(v0, v1, match v2.clone() {
                    Adt2::C0(h6503, _, _, _, _, _, _, _, _, _, _) => {
                        h6503
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6504, _, _, _, _, _, _, _, _, _, _) => {
                        h6504
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6505, _, _, _, _, _, _, _, _, _) => {
                        h6505
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6506, _, _, _, _, _, _, _, _) => {
                        h6506
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6507, _, _, _, _, _, _, _) => {
                        h6507
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6508, _, _, _, _, _, _) => {
                        h6508
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6509, _, _, _, _, _, _, _, _, _) => {
                        h6509
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6510, _, _, _, _, _, _, _, _) => {
                        h6510
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6511, _, _, _, _, _, _, _) => {
                        h6511
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6512, _, _, _, _, _, _) => {
                        h6512
                    }
                })
            }
            Some(v5) => {
                f281(v0, v1, match v2.clone() {
                    Adt2::C0(h6513, _, _, _, _, _, _, _, _, _, _) => {
                        h6513
                    }
                }, match v5.clone() {
                    Adt2::C0(h6514, _, _, _, _, _, _, _, _, _, _) => {
                        h6514
                    }
                }, match v4.clone() {
                    Adt2::C0(h6515, _, _, _, _, _, _, _, _, _, _) => {
                        h6515
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6516, _, _, _, _, _, _, _, _, _) => {
                        h6516
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6517, _, _, _, _, _, _, _, _) => {
                        h6517
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6518, _, _, _, _, _, _, _) => {
                        h6518
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6519, _, _, _, _, _, _) => {
                        h6519
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6520, _, _, _, _, _, _, _, _, _) => {
                        h6520
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6521, _, _, _, _, _, _, _, _) => {
                        h6521
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6522, _, _, _, _, _, _, _) => {
                        h6522
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6523, _, _, _, _, _, _) => {
                        h6523
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6524, _, _, _, _, _, _, _, _, _) => {
                        h6524
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6525, _, _, _, _, _, _, _, _) => {
                        h6525
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6526, _, _, _, _, _, _, _) => {
                        h6526
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6527, _, _, _, _, _, _) => {
                        h6527
                    }
                })
            }
        }
    }
}

pub fn f243(v0: u64, v1: u64) -> u64 {
    let a6528 = v0;
    let a6529 = 23u64;
    if nat_eq(a6528, a6529) {
        let a6530 = v1;
        let a6531 = 0u64;
        if nat_eq(a6530, a6531) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f244(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6536 = {
        let a6534 = {
            let a6532 = v0;
            let a6533 = v1;
            nat_sub(a6532, a6533)
        };
        let a6535 = 1u64;
        nat_sub(a6534, a6535)
    };
    let a6537 = v2;
    nat_add(a6536, a6537)
}

pub fn f245(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f145({
        let a6538 = v0;
        let a6539 = v4;
        let a6540 = 0u64;
        nat_rem(a6538, a6539, a6540)
    }, {
        let a6541 = v1;
        let a6542 = v5;
        let a6543 = 0u64;
        nat_rem(a6541, a6542, a6543)
    }, {
        let a6544 = v2;
        let a6545 = v6;
        let a6546 = 0u64;
        nat_rem(a6544, a6545, a6546)
    }, {
        let a6547 = v3;
        let a6548 = v7;
        let a6549 = 0u64;
        nat_rem(a6547, a6548, a6549)
    }, v8, v9, v10, v11)
}

pub fn f246(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6550 = v1;
    let a6551 = 2147483648u64;
    if nat_lt(a6550, a6551) {
        let a6562 = {
            let a6560 = {
                let a6555 = {
                    let a6552 = v0;
                    let a6553 = v2;
                    let a6554 = 0u64;
                    nat_rem(a6552, a6553, a6554)
                };
                let a6556 = v2;
                nat_add(a6555, a6556)?
            };
            let a6561 = {
                let a6557 = v1;
                let a6558 = v2;
                let a6559 = 0u64;
                nat_rem(a6557, a6558, a6559)
            };
            nat_sub(a6560, a6561)
        };
        let a6563 = v2;
        let a6564 = 0u64;
        Ok(nat_rem(a6562, a6563, a6564))
    } else {
        let a6567 = {
            let a6565 = v0;
            let a6566 = f146(v1);
            nat_add(a6565, a6566)?
        };
        let a6568 = v2;
        let a6569 = 0u64;
        Ok(nat_rem(a6567, a6568, a6569))
    }
}

pub fn f247(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6572 = f147(v1);
    let a6573 = {
        let a6570 = v0;
        let a6571 = f146(v1);
        nat_add(a6570, a6571)?
    };
    if nat_le(a6572, a6573) {
        let a6574 = f248(v0, v1)?;
        let a6575 = v2;
        Ok(nat_lt(a6574, a6575))
    } else {
        Ok(false)
    }
}

pub fn f248(v0: u64, v1: u64) -> R<u64> {
    let a6578 = {
        let a6576 = v0;
        let a6577 = f146(v1);
        nat_add(a6576, a6577)?
    };
    let a6579 = f147(v1);
    Ok(nat_sub(a6578, a6579))
}

pub fn f249(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6580 = v0;
    let a6581 = v1;
    if nat_lt(a6580, a6581) {
        let a6582 = v1;
        let a6583 = v0;
        Ok(nat_sub(a6582, a6583))
    } else {
        let a6586 = v0;
        let a6587 = {
            let a6584 = v1;
            let a6585 = v2;
            nat_add(a6584, a6585)?
        };
        if nat_lt(a6586, a6587) {
            let a6588 = v0;
            let a6589 = v1;
            Ok(nat_sub(a6588, a6589))
        } else {
            let a6596 = {
                let a6592 = 2u64;
                let a6593 = {
                    let a6590 = v2;
                    let a6591 = 1u64;
                    nat_sub(a6590, a6591)
                };
                nat_mul(a6592, a6593)?
            };
            let a6597 = {
                let a6594 = v0;
                let a6595 = v1;
                nat_sub(a6594, a6595)
            };
            Ok(nat_sub(a6596, a6597))
        }
    }
}

pub fn f250(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6600 = {
        let a6598 = v2;
        let a6599 = f282(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1.clone(), 3u64), false)?;
        nat_add(a6598, a6599)?
    };
    let a6601 = f282(v1.clone(), f58(v0, v1.clone(), 0u64), f58(v0, v1.clone(), 1u64), f58(v0, v1.clone(), 2u64), f58(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6600, a6601))
}

pub fn f251(v0: u64, v1: u64, v2: u64) -> bool {
    let a6602 = v0;
    let a6603 = 0u64;
    if nat_eq(a6602, a6603) {
        let a6604 = v2;
        let a6605 = v1;
        nat_le(a6604, a6605)
    } else {
        let a6606 = v0;
        let a6607 = 1u64;
        if nat_eq(a6606, a6607) {
            let a6608 = v2;
            let a6609 = v1;
            nat_lt(a6608, a6609)
        } else {
            let a6610 = v0;
            let a6611 = 2u64;
            if nat_eq(a6610, a6611) {
                let a6612 = v1;
                let a6613 = v2;
                nat_le(a6612, a6613)
            } else {
                let a6614 = v0;
                let a6615 = 3u64;
                if nat_eq(a6614, a6615) {
                    let a6616 = v1;
                    let a6617 = v2;
                    nat_lt(a6616, a6617)
                } else {
                    let a6618 = v1;
                    let a6619 = v2;
                    nat_eq(a6618, a6619)
                }
            }
        }
    }
}

pub fn f252(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6620 = v0;
    let a6621 = 0u64;
    if nat_eq(a6620, a6621) {
        Ok(f265(0u64))
    } else {
        let a6624 = {
            let a6622 = v0;
            let a6623 = 1u64;
            nat_add(a6622, a6623)?
        };
        let a6625 = f241(v1.clone(), 13u64, v2)?;
        if nat_eq(a6624, a6625) {
            Ok(Adt9::C4({
                let a6626 = v2;
                let a6627 = 0u64;
                if nat_eq(a6626, a6627) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6628 = v0;
                let a6629 = 1u64;
                nat_sub(a6628, a6629)
            };
            if f280(v1.clone(), v2) {
                let v4: u64 = f253(match v1.clone() {
                    Adt3::C0(h6630, _, _, _, _, _, _, _) => {
                        h6630
                    }
                }, false)?;
                let v5: u64 = {
                    let a6631 = v3;
                    let a6632 = v4;
                    let a6633 = 0u64;
                    nat_quot(a6631, a6632, a6633)
                };
                let v6: u64 = {
                    let a6634 = v3;
                    let a6635 = v4;
                    let a6636 = 0u64;
                    nat_rem(a6634, a6635, a6636)
                };
                let a6639 = {
                    let a6637 = v6;
                    let a6638 = 1u64;
                    nat_add(a6637, a6638)?
                };
                let a6640 = v4;
                if nat_eq(a6639, a6640) {
                    Ok(Adt9::C4({
                        let a6641 = v2;
                        let a6642 = 0u64;
                        if nat_eq(a6641, a6642) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f254(v6, v5, v1, v2)
                }
            } else {
                f283({
                    let a6643 = v3;
                    let a6644 = 2u64;
                    let a6645 = 0u64;
                    nat_quot(a6643, a6644, a6645)
                }, {
                    let a6646 = v3;
                    let a6647 = 2u64;
                    let a6648 = 0u64;
                    nat_rem(a6646, a6647, a6648)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f253(v0: u64, v1: bool) -> R<u64> {
    let a6651 = {
        let a6649 = v0;
        let a6650 = 2u64;
        nat_mul(a6649, a6650)?
    };
    let a6652 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6651, a6652)
}

pub fn f254(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6653 = v0;
    let a6654 = 0u64;
    if nat_eq(a6653, a6654) {
        Ok(f265(1u64))
    } else {
        f283({
            let a6663 = {
                let a6656 = v1;
                let a6657 = match v2.clone() {
                    Adt3::C0(h6655, _, _, _, _, _, _, _) => {
                        h6655
                    }
                };
                nat_mul(a6656, a6657)?
            };
            let a6664 = {
                let a6660 = {
                    let a6658 = v0;
                    let a6659 = 1u64;
                    nat_sub(a6658, a6659)
                };
                let a6661 = 2u64;
                let a6662 = 0u64;
                nat_quot(a6660, a6661, a6662)
            };
            nat_add(a6663, a6664)?
        }, {
            let a6667 = {
                let a6665 = v0;
                let a6666 = 1u64;
                nat_sub(a6665, a6666)
            };
            let a6668 = 2u64;
            let a6669 = 0u64;
            nat_rem(a6667, a6668, a6669)
        }, v2, v3, 1u64)
    }
}

pub fn f255(v0: u64, v1: u64) -> R<u64> {
    let a6670 = v0;
    let a6671 = f97(v1)?;
    let a6672 = 0u64;
    Ok(nat_rem(a6670, a6671, a6672))
}

pub fn f256(v0: u64, v1: u64) -> R<u64> {
    let a6673 = 4u64;
    let a6674 = f284(v0, v1)?;
    nat_mul(a6673, a6674)
}

pub fn f257(v0: u64, v1: u64) -> R<u64> {
    let a6675 = 4u64;
    let a6676 = f285(v0, v1)?;
    nat_mul(a6675, a6676)
}

pub fn f258(v0: u64, v1: u64) -> R<bool> {
    let a6682 = {
        let a6679 = {
            let a6677 = f255(v0, v1)?;
            let a6678 = 2u64;
            nat_sub(a6677, a6678)
        };
        let a6680 = 2u64;
        let a6681 = 0u64;
        nat_rem(a6679, a6680, a6681)
    };
    let a6683 = 0u64;
    Ok(nat_eq(a6682, a6683))
}

pub fn f259(v0: u64) -> u64 {
    let m6684 = f221(v0);
    match m6684 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6685, _) => {
                    h6685
                }
            }
        }
    }
}

pub fn f260(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6686 = f221(v0);
    match m6686 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f229(v0) {
                let a6687 = v1;
                let a6688 = v7;
                nat_lt(a6687, a6688)
            } else {
                false
            } {
                f189(v8, v2)
            } else {
                false
            } {
                let a6689 = v3;
                let a6690 = f286(v4, v5, v6)?;
                nat_lt(a6689, a6690)
            } else {
                false
            } {
                let a6695 = {
                    let a6692 = v7;
                    let a6693 = match v13.clone() {
                        Adt19::C0(_, h6691, _) => {
                            h6691
                        }
                    };
                    let a6694 = 0u64;
                    nat_rem(a6692, a6693, a6694)
                };
                let a6696 = 0u64;
                nat_eq(a6695, a6696)
            } else {
                false
            } {
                Ok(Some({
                    let a6725 = {
                        let a6715 = {
                            let a6705 = {
                                let a6701 = {
                                    let a6698 = v1;
                                    let a6699 = match v13 {
                                        Adt19::C0(_, h6697, _) => {
                                            h6697
                                        }
                                    };
                                    let a6700 = 0u64;
                                    nat_quot(a6698, a6699, a6700)
                                };
                                let a6702 = v9;
                                nat_mul(a6701, a6702)?
                            };
                            let a6706 = {
                                let a6703 = v2;
                                let a6704 = v10;
                                nat_mul(a6703, a6704)?
                            };
                            nat_add(a6705, a6706)?
                        };
                        let a6716 = {
                            let a6713 = {
                                let a6710 = {
                                    let a6707 = v3;
                                    let a6708 = v4;
                                    let a6709 = 0u64;
                                    nat_quot(a6707, a6708, a6709)
                                };
                                let a6711 = v5;
                                let a6712 = 0u64;
                                nat_rem(a6710, a6711, a6712)
                            };
                            let a6714 = v11;
                            nat_mul(a6713, a6714)?
                        };
                        nat_add(a6715, a6716)?
                    };
                    let a6726 = {
                        let a6723 = {
                            let a6720 = {
                                let a6717 = v3;
                                let a6718 = v4;
                                let a6719 = 0u64;
                                nat_quot(a6717, a6718, a6719)
                            };
                            let a6721 = v5;
                            let a6722 = 0u64;
                            nat_quot(a6720, a6721, a6722)
                        };
                        let a6724 = v12;
                        nat_mul(a6723, a6724)?
                    };
                    nat_add(a6725, a6726)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f287(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f262(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f288(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f263(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6727 = v0;
    let a6728 = 64u64;
    if nat_eq(a6727, a6728) {
        f289(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6729 = v0;
        let a6730 = 65u64;
        if nat_eq(a6729, a6730) {
            f290(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f264(v0: u64) -> u64 {
    let a6731 = v0;
    let a6732 = 30u64;
    if nat_eq(a6731, a6732) {
        4u64
    } else {
        3u64
    }
}

pub fn f265(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f266(v0: u64) -> bool {
    let a6733 = 0u64;
    let a6734 = v0;
    if nat_lt(a6733, a6734) {
        let a6735 = v0;
        let a6736 = 2139095040u64;
        nat_le(a6735, a6736)
    } else {
        false
    }
}

pub fn f267(v0: u64) -> u64 {
    let a6737 = v0;
    let a6738 = 40u64;
    if nat_lt(a6737, a6738) {
        let a6739 = v0;
        let a6740 = 20u64;
        if nat_lt(a6739, a6740) {
            let a6741 = v0;
            let a6742 = 10u64;
            if nat_lt(a6741, a6742) {
                let a6743 = v0;
                let a6744 = 5u64;
                if nat_lt(a6743, a6744) {
                    let a6745 = v0;
                    let a6746 = 2u64;
                    if nat_lt(a6745, a6746) {
                        let a6747 = v0;
                        let a6748 = 1u64;
                        if nat_lt(a6747, a6748) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6749 = v0;
                        let a6750 = 3u64;
                        if nat_lt(a6749, a6750) {
                            4u64
                        } else {
                            let a6751 = v0;
                            let a6752 = 4u64;
                            if nat_lt(a6751, a6752) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6753 = v0;
                    let a6754 = 7u64;
                    if nat_lt(a6753, a6754) {
                        let a6755 = v0;
                        let a6756 = 6u64;
                        if nat_lt(a6755, a6756) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6757 = v0;
                        let a6758 = 8u64;
                        if nat_lt(a6757, a6758) {
                            128u64
                        } else {
                            let a6759 = v0;
                            let a6760 = 9u64;
                            if nat_lt(a6759, a6760) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6761 = v0;
                let a6762 = 15u64;
                if nat_lt(a6761, a6762) {
                    let a6763 = v0;
                    let a6764 = 12u64;
                    if nat_lt(a6763, a6764) {
                        let a6765 = v0;
                        let a6766 = 11u64;
                        if nat_lt(a6765, a6766) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6767 = v0;
                        let a6768 = 13u64;
                        if nat_lt(a6767, a6768) {
                            4096u64
                        } else {
                            let a6769 = v0;
                            let a6770 = 14u64;
                            if nat_lt(a6769, a6770) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6771 = v0;
                    let a6772 = 17u64;
                    if nat_lt(a6771, a6772) {
                        let a6773 = v0;
                        let a6774 = 16u64;
                        if nat_lt(a6773, a6774) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6775 = v0;
                        let a6776 = 18u64;
                        if nat_lt(a6775, a6776) {
                            131072u64
                        } else {
                            let a6777 = v0;
                            let a6778 = 19u64;
                            if nat_lt(a6777, a6778) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6779 = v0;
            let a6780 = 30u64;
            if nat_lt(a6779, a6780) {
                let a6781 = v0;
                let a6782 = 25u64;
                if nat_lt(a6781, a6782) {
                    let a6783 = v0;
                    let a6784 = 22u64;
                    if nat_lt(a6783, a6784) {
                        let a6785 = v0;
                        let a6786 = 21u64;
                        if nat_lt(a6785, a6786) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6787 = v0;
                        let a6788 = 23u64;
                        if nat_lt(a6787, a6788) {
                            4194304u64
                        } else {
                            let a6789 = v0;
                            let a6790 = 24u64;
                            if nat_lt(a6789, a6790) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6791 = v0;
                    let a6792 = 27u64;
                    if nat_lt(a6791, a6792) {
                        let a6793 = v0;
                        let a6794 = 26u64;
                        if nat_lt(a6793, a6794) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6795 = v0;
                        let a6796 = 28u64;
                        if nat_lt(a6795, a6796) {
                            134217728u64
                        } else {
                            let a6797 = v0;
                            let a6798 = 29u64;
                            if nat_lt(a6797, a6798) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6799 = v0;
                let a6800 = 35u64;
                if nat_lt(a6799, a6800) {
                    let a6801 = v0;
                    let a6802 = 32u64;
                    if nat_lt(a6801, a6802) {
                        let a6803 = v0;
                        let a6804 = 31u64;
                        if nat_lt(a6803, a6804) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6805 = v0;
                        let a6806 = 33u64;
                        if nat_lt(a6805, a6806) {
                            4294967296u64
                        } else {
                            let a6807 = v0;
                            let a6808 = 34u64;
                            if nat_lt(a6807, a6808) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6809 = v0;
                    let a6810 = 37u64;
                    if nat_lt(a6809, a6810) {
                        let a6811 = v0;
                        let a6812 = 36u64;
                        if nat_lt(a6811, a6812) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6813 = v0;
                        let a6814 = 38u64;
                        if nat_lt(a6813, a6814) {
                            137438953472u64
                        } else {
                            let a6815 = v0;
                            let a6816 = 39u64;
                            if nat_lt(a6815, a6816) {
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

pub fn f268(v0: u64, v1: u64) -> bool {
    let a6817 = v0;
    let a6818 = 1u64;
    if nat_le(a6817, a6818) {
        true
    } else {
        let a6822 = {
            let a6819 = v1;
            let a6820 = 4u64;
            let a6821 = 0u64;
            nat_rem(a6819, a6820, a6821)
        };
        let a6823 = 0u64;
        nat_eq(a6822, a6823)
    }
}

pub fn f269(v0: u64, v1: u64) -> bool {
    let a6824 = v0;
    let a6825 = 1u64;
    if nat_le(a6824, a6825) {
        true
    } else {
        let a6826 = 4u64;
        let a6827 = v1;
        nat_le(a6826, a6827)
    }
}

pub fn f270(v0: u64, v1: u64) -> R<u64> {
    let a6828 = v0;
    let a6829 = 0u64;
    if nat_eq(a6828, a6829) {
        Ok(0u64)
    } else {
        let a6834 = 4u64;
        let a6835 = {
            let a6832 = {
                let a6830 = v0;
                let a6831 = 1u64;
                nat_sub(a6830, a6831)
            };
            let a6833 = v1;
            nat_mul(a6832, a6833)?
        };
        nat_add(a6834, a6835)
    }
}

pub fn f271(v0: bool, v1: u64) -> R<u64> {
    let a6840 = {
        let a6838 = {
            let a6836 = f291(v0, v1)?;
            let a6837 = f292(v0, v1)?;
            nat_add(a6836, a6837)?
        };
        let a6839 = f293(v0);
        nat_add(a6838, a6839)?
    };
    let a6841 = v1;
    nat_add(a6840, a6841)
}

pub fn f272(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f294(v0, v1)?;
    let v3: u64 = {
        let a6846 = {
            let a6844 = {
                let a6842 = v1;
                let a6843 = 1u64;
                nat_sub(a6842, a6843)
            };
            let a6845 = v2;
            nat_mul(a6844, a6845)?
        };
        let a6847 = v0;
        nat_sub(a6846, a6847)
    };
    let a6848 = v2;
    let a6849 = v3;
    if nat_lt(a6848, a6849) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f273(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6850 = v4;
    let a6851 = 0u64;
    if nat_eq(a6850, a6851) {
        Ok(0u64)
    } else {
        let a6858 = 2u64;
        let a6859 = {
            let a6856 = {
                let a6852 = v0;
                let a6853 = v1;
                nat_mul(a6852, a6853)?
            };
            let a6857 = {
                let a6854 = v2;
                let a6855 = v3;
                nat_mul(a6854, a6855)?
            };
            nat_mul(a6856, a6857)?
        };
        nat_mul(a6858, a6859)
    }
}

pub fn f274(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a6864 = v3;
    let a6865 = {
        let a6862 = {
            let a6860 = f295(v2, v4)?;
            let a6861 = f296(v0, v1, f294(v2, v4)?)?;
            nat_mul(a6860, a6861)?
        };
        let a6863 = f296(v0, v1, f297(v2, v4)?)?;
        nat_add(a6862, a6863)?
    };
    nat_mul(a6864, a6865)
}

pub fn f275(v0: u64) -> R<u64> {
    let a6868 = 13u64;
    let a6869 = {
        let a6866 = 9u64;
        let a6867 = v0;
        nat_mul(a6866, a6867)?
    };
    nat_add(a6868, a6869)
}

pub fn f276(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f298(v0, v1, v6, v7, v8) {
        let a6870 = v5;
        let a6871 = 26u64;
        nat_eq(a6870, a6871)
    } else {
        false
    } {
        let a6872 = v3;
        let a6873 = v7;
        nat_eq(a6872, a6873)
    } else {
        false
    } {
        let a6874 = v4;
        let a6875 = v8;
        nat_eq(a6874, a6875)
    } else {
        false
    } {
        let a6876 = v9;
        let a6877 = 1u64;
        nat_eq(a6876, a6877)
    } else {
        false
    } {
        let a6878 = v10;
        let a6879 = f299(v0);
        nat_eq(a6878, a6879)
    } else {
        false
    } {
        let a6880 = v11;
        let a6881 = v1;
        nat_eq(a6880, a6881)
    } else {
        false
    } {
        let a6882 = v12;
        let a6883 = v6;
        nat_eq(a6882, a6883)
    } else {
        false
    } {
        let a6884 = v13;
        let a6885 = v7;
        nat_eq(a6884, a6885)
    } else {
        false
    } {
        let a6886 = v14;
        let a6887 = v8;
        nat_eq(a6886, a6887)
    } else {
        false
    }
}

pub fn f277(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a6888 = v0;
    let a6889 = 0u64;
    if if if nat_eq(a6888, a6889) {
        true
    } else {
        let a6890 = v1;
        let a6891 = 0u64;
        nat_eq(a6890, a6891)
    } {
        true
    } else {
        let a6892 = v2;
        let a6893 = 0u64;
        nat_eq(a6892, a6893)
    } {
        true
    } else {
        let a6894 = v3;
        let a6895 = 0u64;
        nat_eq(a6894, a6895)
    }
}

pub fn f278(v0: Adt3, v1: bool) -> R<u64> {
    f282(v0.clone(), {
        let a6897 = match v0.clone() {
            Adt3::C0(h6896, _, _, _, _, _, _, _) => {
                h6896
            }
        };
        let a6898 = 1u64;
        nat_sub(a6897, a6898)
    }, {
        let a6900 = match v0.clone() {
            Adt3::C0(_, h6899, _, _, _, _, _, _) => {
                h6899
            }
        };
        let a6901 = 1u64;
        nat_sub(a6900, a6901)
    }, {
        let a6903 = match v0.clone() {
            Adt3::C0(_, _, h6902, _, _, _, _, _) => {
                h6902
            }
        };
        let a6904 = 1u64;
        nat_sub(a6903, a6904)
    }, {
        let a6906 = match v0 {
            Adt3::C0(_, _, _, h6905, _, _, _, _) => {
                h6905
            }
        };
        let a6907 = 1u64;
        nat_sub(a6906, a6907)
    }, v1)
}

pub fn f279(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a6908 = v0;
    let a6909 = 0u64;
    if nat_eq(a6908, a6909) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a6910 = v0;
            let a6911 = 1u64;
            nat_sub(a6910, a6911)
        };
        let v5: u64 = {
            let a6912 = v4;
            let a6913 = v1;
            let a6914 = 0u64;
            nat_quot(a6912, a6913, a6914)
        };
        let v6: u64 = {
            let a6922 = {
                let a6915 = v5;
                let a6916 = v2;
                nat_mul(a6915, a6916)?
            };
            let a6923 = {
                let a6920 = {
                    let a6917 = v4;
                    let a6918 = v1;
                    let a6919 = 0u64;
                    nat_rem(a6917, a6918, a6919)
                };
                let a6921 = v3;
                nat_mul(a6920, a6921)?
            };
            nat_add(a6922, a6923)?
        };
        let a6924 = v5;
        let a6925 = 0u64;
        if nat_eq(a6924, a6925) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a6934 = {
                    let a6928 = {
                        let a6926 = v5;
                        let a6927 = 1u64;
                        nat_sub(a6926, a6927)
                    };
                    let a6929 = v2;
                    nat_mul(a6928, a6929)?
                };
                let a6935 = {
                    let a6932 = {
                        let a6930 = v1;
                        let a6931 = 1u64;
                        nat_sub(a6930, a6931)
                    };
                    let a6933 = v3;
                    nat_mul(a6932, a6933)?
                };
                nat_add(a6934, a6935)?
            };
            let a6936 = v6;
            let a6937 = v7;
            if nat_lt(a6936, a6937) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f280(v0: Adt3, v1: u64) -> bool {
    let a6938 = v1;
    let a6939 = 0u64;
    if !nat_eq(a6938, a6939) {
        true
    } else {
        let a6941 = match v0 {
            Adt3::C0(_, _, _, _, h6940, _, _, _) => {
                h6940
            }
        };
        let a6942 = 4u64;
        nat_eq(a6941, a6942)
    }
}

pub fn f281(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m6943 = f300(v0, v1);
    match m6943 {
        None => {
            if v5 {
                let m6944 = f301(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m6944 {
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
                let a6945 = 4u64;
                let a6946 = v28;
                nat_le(a6945, a6946)
            };
            let a6947 = v2;
            let a6948 = 0u64;
            if if if if if nat_eq(a6947, a6948) {
                let a6949 = v4;
                let a6950 = 0u64;
                nat_eq(a6949, a6950)
            } else {
                false
            } {
                let a6951 = v6;
                let a6952 = v14;
                if nat_eq(a6951, a6952) {
                    let a6953 = v7;
                    let a6954 = v15;
                    nat_eq(a6953, a6954)
                } else {
                    false
                }
            } else {
                false
            } {
                let a6955 = v8;
                let a6956 = v16;
                if nat_eq(a6955, a6956) {
                    let a6957 = v9;
                    let a6958 = v17;
                    nat_eq(a6957, a6958)
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
                        let a6959 = v3;
                        let a6960 = 0u64;
                        nat_eq(a6959, a6960)
                    } else {
                        false
                    } {
                        f232(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a6965 = {
                        let a6961 = v6;
                        let a6962 = v7;
                        nat_mul(a6961, a6962)?
                    };
                    let a6966 = {
                        let a6963 = v8;
                        let a6964 = v9;
                        nat_mul(a6963, a6964)?
                    };
                    nat_mul(a6965, a6966)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f282(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6987 = {
        let a6981 = {
            let a6975 = {
                let a6969 = v1;
                let a6970 = if v5 {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6967, _, _, _) => {
                            h6967
                        }
                    })
                } else {
                    f147(match v0.clone() {
                        Adt3::C0(_, _, _, _, h6968, _, _, _) => {
                            h6968
                        }
                    })
                };
                nat_mul(a6969, a6970)?
            };
            let a6976 = {
                let a6973 = v2;
                let a6974 = if v5 {
                    f146(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6971, _, _) => {
                            h6971
                        }
                    })
                } else {
                    f147(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h6972, _, _) => {
                            h6972
                        }
                    })
                };
                nat_mul(a6973, a6974)?
            };
            nat_add(a6975, a6976)?
        };
        let a6982 = {
            let a6979 = v3;
            let a6980 = if v5 {
                f146(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6977, _) => {
                        h6977
                    }
                })
            } else {
                f147(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h6978, _) => {
                        h6978
                    }
                })
            };
            nat_mul(a6979, a6980)?
        };
        nat_add(a6981, a6982)?
    };
    let a6988 = {
        let a6985 = v4;
        let a6986 = if v5 {
            f146(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6983) => {
                    h6983
                }
            })
        } else {
            f147(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h6984) => {
                    h6984
                }
            })
        };
        nat_mul(a6985, a6986)?
    };
    nat_add(a6987, a6988)
}

pub fn f283(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6989 = v1;
    let a6990 = 0u64;
    if nat_eq(a6989, a6990) {
        Ok(Adt9::C4({
            let a6991 = v3;
            let a6992 = 0u64;
            if nat_eq(a6991, a6992) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f94(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6993 = v3;
            let a6994 = 0u64;
            if nat_eq(a6993, a6994) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f284(v0: u64, v1: u64) -> R<u64> {
    let a6995 = v0;
    let a6996 = f97(v1)?;
    let a6997 = 0u64;
    Ok(nat_quot(a6995, a6996, a6997))
}

pub fn f285(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f284(v0, v1)?;
    let v3: u64 = {
        let a7000 = {
            let a6998 = f255(v0, v1)?;
            let a6999 = 2u64;
            nat_sub(a6998, a6999)
        };
        let a7001 = 2u64;
        let a7002 = 0u64;
        nat_quot(a7000, a7001, a7002)
    };
    let v4: u64 = {
        let a7003 = v2;
        let a7004 = v1;
        nat_mul(a7003, a7004)?
    };
    if f258(v0, v1)? {
        let a7005 = v4;
        let a7006 = v3;
        nat_add(a7005, a7006)
    } else {
        let a7018 = {
            let a7014 = v4;
            let a7015 = {
                let a7012 = {
                    let a7007 = v1;
                    let a7008 = 1u64;
                    nat_sub(a7007, a7008)
                };
                let a7013 = {
                    let a7009 = v2;
                    let a7010 = v1;
                    let a7011 = 0u64;
                    nat_rem(a7009, a7010, a7011)
                };
                nat_mul(a7012, a7013)?
            };
            nat_sub(a7014, a7015)
        };
        let a7019 = {
            let a7016 = v3;
            let a7017 = v1;
            nat_mul(a7016, a7017)?
        };
        nat_add(a7018, a7019)
    }
}

pub fn f286(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7020 = v0;
    let a7021 = 0u64;
    if if if nat_eq(a7020, a7021) {
        true
    } else {
        let a7022 = v1;
        let a7023 = 0u64;
        nat_eq(a7022, a7023)
    } {
        true
    } else {
        let a7024 = v2;
        let a7025 = 0u64;
        nat_eq(a7024, a7025)
    } {
        Ok(0u64)
    } else {
        let a7028 = {
            let a7026 = v0;
            let a7027 = v1;
            nat_mul(a7026, a7027)?
        };
        let a7029 = v2;
        nat_mul(a7028, a7029)
    }
}

pub fn f287(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f106(match v1.clone() {
        Adt20::C0(h7030, _, _, _, _, _, _, _) => {
            h7030
        }
    })?;
    let v6: u64 = {
        let a7036 = match v1.clone() {
            Adt20::C0(_, h7031, _, _, _, _, _, _) => {
                h7031
            }
        };
        let a7037 = {
            let a7034 = match v1.clone() {
                Adt20::C0(_, _, h7032, _, _, _, _, _) => {
                    h7032
                }
            };
            let a7035 = match v1.clone() {
                Adt20::C0(_, _, _, h7033, _, _, _, _) => {
                    h7033
                }
            };
            nat_mul(a7034, a7035)?
        };
        nat_mul(a7036, a7037)?
    };
    let a7039 = v0;
    let a7040 = f53(match v1.clone() {
        Adt20::C0(h7038, _, _, _, _, _, _, _) => {
            h7038
        }
    }, v6)?;
    if nat_lt(a7039, a7040) {
        f302({
            let a7041 = v0;
            let a7042 = v5;
            let a7043 = 0u64;
            nat_rem(a7041, a7042, a7043)
        }, {
            let a7044 = v0;
            let a7045 = v5;
            let a7046 = 0u64;
            nat_quot(a7044, a7045, a7046)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f288(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a7047 = v6;
        let a7048 = 6u64;
        nat_sub(a7047, a7048)
    };
    let v9: Adt20 = f303(v3);
    let a7050 = v0;
    let a7051 = f218(match v1.clone() {
        Adt20::C0(h7049, _, _, _, _, _, _, _) => {
            h7049
        }
    }, v5, v8)?;
    if nat_lt(a7050, a7051) {
        let v10: u64 = f103(match v1.clone() {
            Adt20::C0(h7052, _, _, _, _, _, _, _) => {
                h7052
            }
        }, v5, v8)?;
        let a7053 = v0;
        let a7054 = v10;
        if nat_lt(a7053, a7054) {
            f304(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a7055 = v0;
                let a7056 = v10;
                nat_sub(a7055, a7056)
            };
            let v12: u64 = f275(match v1.clone() {
                Adt20::C0(h7057, _, _, _, _, _, _, _) => {
                    h7057
                }
            })?;
            f305({
                let a7058 = v11;
                let a7059 = v12;
                let a7060 = 0u64;
                nat_rem(a7058, a7059, a7060)
            }, {
                let a7061 = v11;
                let a7062 = v12;
                let a7063 = 0u64;
                nat_quot(a7061, a7062, a7063)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f289(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7064 = v5;
        let a7065 = 6u64;
        nat_sub(a7064, a7065)
    };
    let v8: bool = f212(match v1.clone() {
        Adt20::C0(_, _, _, _, h7066, _, _, _) => {
            h7066
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h7067, _, _, _) => {
            h7067
        }
    });
    let a7069 = v0;
    let a7070 = f211(v8, match v1.clone() {
        Adt20::C0(h7068, _, _, _, _, _, _, _) => {
            h7068
        }
    }, v4, v7)?;
    if nat_lt(a7069, a7070) {
        let v9: u64 = f103(match v1.clone() {
            Adt20::C0(h7071, _, _, _, _, _, _, _) => {
                h7071
            }
        }, v4, v7)?;
        let a7072 = v0;
        let a7073 = v9;
        if nat_lt(a7072, a7073) {
            f304(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a7074 = v0;
                let a7075 = v9;
                nat_sub(a7074, a7075)
            };
            let v11: u64 = f271(v8, match v1.clone() {
                Adt20::C0(h7076, _, _, _, _, _, _, _) => {
                    h7076
                }
            })?;
            f306({
                let a7077 = v10;
                let a7078 = v11;
                let a7079 = 0u64;
                nat_rem(a7077, a7078, a7079)
            }, {
                let a7080 = v10;
                let a7081 = v11;
                let a7082 = 0u64;
                nat_quot(a7080, a7081, a7082)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f290(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7083 = v5;
        let a7084 = 6u64;
        nat_sub(a7083, a7084)
    };
    let v8: Adt20 = f303(v2);
    let v9: u64 = f273(match v1.clone() {
        Adt20::C0(h7085, _, _, _, _, _, _, _) => {
            h7085
        }
    }, match v1.clone() {
        Adt20::C0(_, h7086, _, _, _, _, _, _) => {
            h7086
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7087, _, _, _, _, _) => {
            h7087
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7088, _, _, _, _) => {
            h7088
        }
    }, v7)?;
    let a7093 = v0;
    let a7094 = f217(match v1.clone() {
        Adt20::C0(h7089, _, _, _, _, _, _, _) => {
            h7089
        }
    }, match v1.clone() {
        Adt20::C0(_, h7090, _, _, _, _, _, _) => {
            h7090
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7091, _, _, _, _, _) => {
            h7091
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7092, _, _, _, _) => {
            h7092
        }
    }, v4, v7)?;
    if nat_lt(a7093, a7094) {
        let a7095 = v0;
        let a7096 = v9;
        if nat_lt(a7095, a7096) {
            f304(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f307({
                let a7097 = v0;
                let a7098 = v9;
                nat_sub(a7097, a7098)
            }, match v1.clone() {
                Adt20::C0(h7099, _, _, _, _, _, _, _) => {
                    h7099
                }
            }, match v1.clone() {
                Adt20::C0(_, h7100, _, _, _, _, _, _) => {
                    h7100
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h7101, _, _, _, _, _) => {
                    h7101
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h7102, _, _, _, _) => {
                    h7102
                }
            }, v4)?;
            f308(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f291(v0: bool, v1: u64) -> R<u64> {
    let a7105 = if v0 {
        5u64
    } else {
        4u64
    };
    let a7106 = {
        let a7103 = f309(v0);
        let a7104 = v1;
        nat_mul(a7103, a7104)?
    };
    nat_add(a7105, a7106)
}

pub fn f292(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a7114 = {
            let a7107 = 13u64;
            let a7108 = f310(v1);
            nat_mul(a7107, a7108)?
        };
        let a7115 = {
            let a7112 = 4u64;
            let a7113 = {
                let a7109 = v1;
                let a7110 = 4u64;
                let a7111 = 0u64;
                nat_rem(a7109, a7110, a7111)
            };
            nat_mul(a7112, a7113)?
        };
        nat_add(a7114, a7115)
    } else {
        let a7116 = 3u64;
        let a7117 = v1;
        nat_mul(a7116, a7117)
    }
}

pub fn f293(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f294(v0: u64, v1: u64) -> R<u64> {
    let a7122 = {
        let a7120 = {
            let a7118 = v0;
            let a7119 = v1;
            nat_add(a7118, a7119)?
        };
        let a7121 = 1u64;
        nat_sub(a7120, a7121)
    };
    let a7123 = v1;
    let a7124 = 0u64;
    Ok(nat_quot(a7122, a7123, a7124))
}

pub fn f295(v0: u64, v1: u64) -> R<u64> {
    let a7125 = v0;
    let a7126 = f294(v0, v1)?;
    let a7127 = 0u64;
    Ok(nat_quot(a7125, a7126, a7127))
}

pub fn f296(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7128 = v2;
    let a7129 = 0u64;
    if nat_eq(a7128, a7129) {
        Ok(0u64)
    } else {
        let a7134 = 11u64;
        let a7135 = {
            let a7132 = {
                let a7130 = v1;
                let a7131 = v2;
                nat_mul(a7130, a7131)?
            };
            let a7133 = f311(v0)?;
            nat_mul(a7132, a7133)?
        };
        nat_add(a7134, a7135)
    }
}

pub fn f297(v0: u64, v1: u64) -> R<u64> {
    let a7136 = v0;
    let a7137 = f294(v0, v1)?;
    let a7138 = 0u64;
    Ok(nat_rem(a7136, a7137, a7138))
}

pub fn f298(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f277(v1, v2, v3, v4) {
        f312(v0)
    } else {
        f229(v0)
    }
}

pub fn f299(v0: u64) -> u64 {
    let a7139 = v0;
    let a7140 = 26u64;
    if nat_eq(a7139, a7140) {
        26u64
    } else {
        0u64
    }
}

pub fn f300(v0: u64, v1: u64) -> Option<u64> {
    let a7141 = v0;
    let a7142 = 2u64;
    if nat_eq(a7141, a7142) {
        Some(0u64)
    } else {
        let a7143 = v0;
        let a7144 = 5u64;
        if nat_eq(a7143, a7144) {
            Some(1u64)
        } else {
            let a7145 = v0;
            let a7146 = 6u64;
            if nat_eq(a7145, a7146) {
                Some(2u64)
            } else {
                let a7147 = v0;
                let a7148 = 7u64;
                if nat_eq(a7147, a7148) {
                    Some(3u64)
                } else {
                    let a7149 = v0;
                    let a7150 = 9u64;
                    if nat_eq(a7149, a7150) {
                        Some(4u64)
                    } else {
                        let a7151 = v0;
                        let a7152 = 8u64;
                        if nat_eq(a7151, a7152) {
                            Some(5u64)
                        } else {
                            let a7153 = v0;
                            let a7154 = 89u64;
                            if nat_eq(a7153, a7154) {
                                let a7155 = v1;
                                let a7156 = 2u64;
                                if nat_eq(a7155, a7156) {
                                    Some(6u64)
                                } else {
                                    let a7157 = v1;
                                    let a7158 = 0u64;
                                    if nat_eq(a7157, a7158) {
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

pub fn f301(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7159 = v0;
    let a7160 = 30u64;
    if nat_eq(a7159, a7160) {
        let a7161 = v2;
        let a7162 = 0u64;
        if if if nat_eq(a7161, a7162) {
            true
        } else {
            let a7163 = v2;
            let a7164 = 1u64;
            nat_eq(a7163, a7164)
        } {
            let a7165 = v1;
            let a7166 = 0u64;
            if nat_eq(a7165, a7166) {
                true
            } else {
                f313(v1, v4)
            }
        } else {
            false
        } {
            f314(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f315(v1) {
            f315(v2)
        } else {
            false
        } {
            let a7167 = v1;
            let a7168 = 0u64;
            if if nat_eq(a7167, a7168) {
                true
            } else {
                let a7169 = v2;
                let a7170 = 0u64;
                nat_eq(a7169, a7170)
            } {
                true
            } else {
                let a7171 = v1;
                let a7172 = v2;
                nat_eq(a7171, a7172)
            }
        } else {
            false
        } {
            true
        } else {
            let a7173 = v1;
            let a7174 = 43u64;
            if nat_eq(a7173, a7174) {
                f315(v2)
            } else {
                false
            }
        } {
            f314(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f302(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h7175, _, _, _, _, _, _, _) => {
            h7175
        }
    };
    let v7: u64 = {
        let a7178 = 1u64;
        let a7179 = {
            let a7176 = 3u64;
            let a7177 = v6;
            nat_mul(a7176, a7177)?
        };
        nat_add(a7178, a7179)?
    };
    let a7180 = v0;
    let a7181 = 0u64;
    if nat_eq(a7180, a7181) {
        Ok(f316(0u64))
    } else {
        let a7182 = v0;
        let a7183 = v7;
        if nat_lt(a7182, a7183) {
            let v8: u64 = {
                let a7184 = v0;
                let a7185 = 1u64;
                nat_sub(a7184, a7185)
            };
            let v9: u64 = {
                let a7191 = {
                    let a7186 = v1;
                    let a7187 = v6;
                    nat_mul(a7186, a7187)?
                };
                let a7192 = {
                    let a7188 = v8;
                    let a7189 = 3u64;
                    let a7190 = 0u64;
                    nat_quot(a7188, a7189, a7190)
                };
                nat_add(a7191, a7192)?
            };
            let v10: u64 = f317(v9, v2)?;
            let v11: u64 = {
                let a7193 = v8;
                let a7194 = 3u64;
                let a7195 = 0u64;
                nat_rem(a7193, a7194, a7195)
            };
            let a7196 = v11;
            let a7197 = 0u64;
            if nat_eq(a7196, a7197) {
                Ok(f318(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a7198 = v11;
                let a7199 = 1u64;
                if nat_eq(a7198, a7199) {
                    Ok(f318(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f318(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a7200 = v0;
            let a7201 = v7;
            if nat_eq(a7200, a7201) {
                Ok(f318(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a7204 = v0;
                let a7205 = {
                    let a7202 = v7;
                    let a7203 = 1u64;
                    nat_add(a7202, a7203)?
                };
                if nat_eq(a7204, a7205) {
                    Ok(f318(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7208 = v0;
                    let a7209 = {
                        let a7206 = v7;
                        let a7207 = 2u64;
                        nat_add(a7206, a7207)?
                    };
                    if nat_eq(a7208, a7209) {
                        Ok(f318(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a7212 = v0;
                        let a7213 = {
                            let a7210 = v7;
                            let a7211 = 3u64;
                            nat_add(a7210, a7211)?
                        };
                        if nat_eq(a7212, a7213) {
                            let a7214 = v5;
                            let a7215 = 1u64;
                            if if nat_eq(a7214, a7215) {
                                true
                            } else {
                                let a7216 = v5;
                                let a7217 = 2u64;
                                nat_eq(a7216, a7217)
                            } {
                                Ok(f318(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f318(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7224 = {
                                    let a7218 = v1;
                                    let a7219 = v6;
                                    nat_mul(a7218, a7219)?
                                };
                                let a7225 = {
                                    let a7222 = v0;
                                    let a7223 = {
                                        let a7220 = v7;
                                        let a7221 = 4u64;
                                        nat_add(a7220, a7221)?
                                    };
                                    nat_sub(a7222, a7223)
                                };
                                nat_add(a7224, a7225)?
                            };
                            Ok(f318(10u64, 0u64, f317(v12, v2)?, 1u64, 2u64, 3u64, f317(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f303(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h7226, _, _, _, _, _, _, _) => {
            h7226
        }
    }, match v0.clone() {
        Adt20::C0(_, h7227, _, _, _, _, _, _) => {
            h7227
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h7228, _, _, _, _, _) => {
            h7228
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h7229, _, _, _, _) => {
            h7229
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h7230, _, _) => {
            h7230
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h7231, _) => {
            h7231
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h7232) => {
            h7232
        }
    })
}

pub fn f304(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a7239 = v3;
        let a7240 = {
            let a7236 = f317({
                let a7233 = v0;
                let a7234 = 2u64;
                let a7235 = 0u64;
                nat_quot(a7233, a7234, a7235)
            }, v1)?;
            let a7237 = v2;
            let a7238 = 0u64;
            nat_quot(a7236, a7237, a7238)
        };
        nat_add(a7239, a7240)?
    };
    let a7244 = {
        let a7241 = v0;
        let a7242 = 2u64;
        let a7243 = 0u64;
        nat_rem(a7241, a7242, a7243)
    };
    let a7245 = 0u64;
    if nat_eq(a7244, a7245) {
        Ok(f318(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a7246 = v4;
        let a7247 = 0u64;
        if nat_eq(a7246, a7247) {
            Ok(f318(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f305(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f319(match v2.clone() {
        Adt20::C0(h7248, _, _, _, _, _, _, _) => {
            h7248
        }
    })?;
    let v7: u64 = f320(match v2.clone() {
        Adt20::C0(h7249, _, _, _, _, _, _, _) => {
            h7249
        }
    })?;
    let a7250 = v0;
    let a7251 = 2u64;
    if nat_lt(a7250, a7251) {
        Ok(f316(v0))
    } else {
        let a7252 = v0;
        let a7253 = v6;
        if nat_lt(a7252, a7253) {
            f321({
                let a7254 = v0;
                let a7255 = 2u64;
                nat_sub(a7254, a7255)
            }, v1, v2, v3)
        } else {
            let a7256 = v0;
            let a7257 = v7;
            if nat_lt(a7256, a7257) {
                Ok(f322({
                    let a7258 = v0;
                    let a7259 = v6;
                    nat_sub(a7258, a7259)
                }, match v2 {
                    Adt20::C0(h7260, _, _, _, _, _, _, _) => {
                        h7260
                    }
                }, v5))
            } else {
                let a7262 = v0;
                let a7263 = f275(match v2.clone() {
                    Adt20::C0(h7261, _, _, _, _, _, _, _) => {
                        h7261
                    }
                })?;
                if nat_lt(a7262, a7263) {
                    f323({
                        let a7264 = v0;
                        let a7265 = v7;
                        nat_sub(a7264, a7265)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f306(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f291(v5, match v2.clone() {
        Adt20::C0(h7266, _, _, _, _, _, _, _) => {
            h7266
        }
    })?;
    let v7: u64 = {
        let a7268 = v6;
        let a7269 = f292(v5, match v2.clone() {
            Adt20::C0(h7267, _, _, _, _, _, _, _) => {
                h7267
            }
        })?;
        nat_add(a7268, a7269)?
    };
    let v8: u64 = {
        let a7270 = v7;
        let a7271 = f293(v5);
        nat_add(a7270, a7271)?
    };
    let a7272 = v0;
    let a7273 = v6;
    if nat_lt(a7272, a7273) {
        f324(v0, v1, v2, v5)
    } else {
        let a7274 = v0;
        let a7275 = v7;
        if nat_lt(a7274, a7275) {
            f325({
                let a7276 = v0;
                let a7277 = v6;
                nat_sub(a7276, a7277)
            }, v1, v2, v3, v5)
        } else {
            let a7278 = v0;
            let a7279 = v8;
            if nat_lt(a7278, a7279) {
                Ok(f326({
                    let a7280 = v0;
                    let a7281 = v7;
                    nat_sub(a7280, a7281)
                }, match v2 {
                    Adt20::C0(h7282, _, _, _, _, _, _, _) => {
                        h7282
                    }
                }, v4, v5))
            } else {
                let a7286 = v0;
                let a7287 = {
                    let a7284 = v8;
                    let a7285 = match v2.clone() {
                        Adt20::C0(h7283, _, _, _, _, _, _, _) => {
                            h7283
                        }
                    };
                    nat_add(a7284, a7285)?
                };
                if nat_lt(a7286, a7287) {
                    let v9: u64 = f317({
                        let a7293 = {
                            let a7289 = v1;
                            let a7290 = match v2 {
                                Adt20::C0(h7288, _, _, _, _, _, _, _) => {
                                    h7288
                                }
                            };
                            nat_mul(a7289, a7290)?
                        };
                        let a7294 = {
                            let a7291 = v0;
                            let a7292 = v8;
                            nat_sub(a7291, a7292)
                        };
                        nat_add(a7293, a7294)?
                    }, v3)?;
                    Ok(f318(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f307(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f294(v3, v5)?;
    let v7: u64 = f296(v1, v2, v6)?;
    let v8: u64 = {
        let a7297 = {
            let a7295 = f295(v3, v5)?;
            let a7296 = v4;
            nat_mul(a7295, a7296)?
        };
        let a7298 = v7;
        nat_mul(a7297, a7298)?
    };
    let a7299 = v0;
    let a7300 = v8;
    if nat_lt(a7299, a7300) {
        let v9: u64 = {
            let a7301 = v0;
            let a7302 = v7;
            let a7303 = 0u64;
            nat_quot(a7301, a7302, a7303)
        };
        Ok(Adt21::C0({
            let a7307 = {
                let a7304 = v9;
                let a7305 = v4;
                let a7306 = 0u64;
                nat_quot(a7304, a7305, a7306)
            };
            let a7308 = v6;
            nat_mul(a7307, a7308)?
        }, v6, {
            let a7309 = v9;
            let a7310 = v4;
            let a7311 = 0u64;
            nat_rem(a7309, a7310, a7311)
        }, {
            let a7312 = v0;
            let a7313 = v7;
            let a7314 = 0u64;
            nat_rem(a7312, a7313, a7314)
        }))
    } else {
        let v10: u64 = {
            let a7315 = v0;
            let a7316 = v8;
            nat_sub(a7315, a7316)
        };
        let v11: u64 = f296(v1, v2, f297(v3, v5)?)?;
        Ok(Adt21::C0({
            let a7317 = f295(v3, v5)?;
            let a7318 = v6;
            nat_mul(a7317, a7318)?
        }, f297(v3, v5)?, {
            let a7319 = v10;
            let a7320 = v11;
            let a7321 = 0u64;
            nat_quot(a7319, a7320, a7321)
        }, {
            let a7322 = v10;
            let a7323 = v11;
            let a7324 = 0u64;
            nat_rem(a7322, a7323, a7324)
        }))
    }
}

pub fn f308(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7325) => {
            h7325
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7326, _, _, _, _, _, _, _) => {
            h7326
        }
    };
    let v6: u64 = {
        let a7329 = match v0.clone() {
            Adt21::C0(_, h7327, _, _) => {
                h7327
            }
        };
        let a7330 = match v1.clone() {
            Adt20::C0(_, h7328, _, _, _, _, _, _) => {
                h7328
            }
        };
        nat_mul(a7329, a7330)?
    };
    let v7: u64 = {
        let a7331 = v6;
        let a7332 = v5;
        nat_mul(a7331, a7332)?
    };
    let v8: u64 = {
        let a7335 = {
            let a7333 = 2u64;
            let a7334 = v5;
            nat_mul(a7333, a7334)?
        };
        let a7336 = 2u64;
        nat_add(a7335, a7336)?
    };
    let v9: u64 = {
        let a7339 = {
            let a7337 = 4u64;
            let a7338 = v5;
            nat_mul(a7337, a7338)?
        };
        let a7340 = 2u64;
        nat_add(a7339, a7340)?
    };
    let v10: u64 = {
        let a7343 = 1u64;
        let a7344 = {
            let a7341 = v6;
            let a7342 = v8;
            nat_mul(a7341, a7342)?
        };
        nat_add(a7343, a7344)?
    };
    let v11: u64 = {
        let a7345 = v10;
        let a7346 = 4u64;
        nat_add(a7345, a7346)?
    };
    let v12: u64 = {
        let a7349 = v11;
        let a7350 = {
            let a7347 = v6;
            let a7348 = v9;
            nat_mul(a7347, a7348)?
        };
        nat_add(a7349, a7350)?
    };
    let v13: u64 = {
        let a7351 = v12;
        let a7352 = 6u64;
        nat_add(a7351, a7352)?
    };
    let a7353 = v4;
    let a7354 = 0u64;
    if nat_eq(a7353, a7354) {
        Ok(f316(0u64))
    } else {
        let a7355 = v4;
        let a7356 = v10;
        if nat_lt(a7355, a7356) {
            let v14: u64 = {
                let a7357 = v4;
                let a7358 = 1u64;
                nat_sub(a7357, a7358)
            };
            f327(false, {
                let a7359 = v14;
                let a7360 = v8;
                let a7361 = 0u64;
                nat_rem(a7359, a7360, a7361)
            }, {
                let a7362 = v14;
                let a7363 = v8;
                let a7364 = 0u64;
                nat_quot(a7362, a7363, a7364)
            }, v0, v1, v2)
        } else {
            let a7365 = v4;
            let a7366 = v11;
            if nat_lt(a7365, a7366) {
                Ok(f328({
                    let a7367 = v4;
                    let a7368 = v10;
                    nat_sub(a7367, a7368)
                }, v7))
            } else {
                let a7369 = v4;
                let a7370 = v12;
                if nat_lt(a7369, a7370) {
                    let v15: u64 = {
                        let a7371 = v4;
                        let a7372 = v11;
                        nat_sub(a7371, a7372)
                    };
                    f327(true, {
                        let a7373 = v15;
                        let a7374 = v9;
                        let a7375 = 0u64;
                        nat_rem(a7373, a7374, a7375)
                    }, {
                        let a7376 = v15;
                        let a7377 = v9;
                        let a7378 = 0u64;
                        nat_quot(a7376, a7377, a7378)
                    }, v0, v1, v2)
                } else {
                    let a7379 = v4;
                    let a7380 = v13;
                    if nat_lt(a7379, a7380) {
                        Ok(f329({
                            let a7381 = v4;
                            let a7382 = v12;
                            nat_sub(a7381, a7382)
                        }, v7, v3))
                    } else {
                        let a7385 = v4;
                        let a7386 = {
                            let a7383 = v13;
                            let a7384 = v7;
                            nat_add(a7383, a7384)?
                        };
                        if nat_lt(a7385, a7386) {
                            let v16: u64 = {
                                let a7387 = v4;
                                let a7388 = v13;
                                nat_sub(a7387, a7388)
                            };
                            let v17: u64 = {
                                let a7389 = v16;
                                let a7390 = v5;
                                let a7391 = 0u64;
                                nat_quot(a7389, a7390, a7391)
                            };
                            let v18: u64 = {
                                let a7397 = {
                                    let a7392 = f330(v17, v0, v1)?;
                                    let a7393 = v5;
                                    nat_mul(a7392, a7393)?
                                };
                                let a7398 = {
                                    let a7394 = v16;
                                    let a7395 = v5;
                                    let a7396 = 0u64;
                                    nat_rem(a7394, a7395, a7396)
                                };
                                nat_add(a7397, a7398)?
                            };
                            let v19: u64 = f317(v18, v2)?;
                            Ok(f318(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f309(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f310(v0: u64) -> u64 {
    let a7399 = v0;
    let a7400 = 4u64;
    let a7401 = 0u64;
    nat_quot(a7399, a7400, a7401)
}

pub fn f311(v0: u64) -> R<u64> {
    let a7404 = {
        let a7402 = 7u64;
        let a7403 = v0;
        nat_mul(a7402, a7403)?
    };
    let a7405 = 4u64;
    nat_add(a7404, a7405)
}

pub fn f312(v0: u64) -> bool {
    let m7406 = f221(v0);
    match m7406 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f313(v0: u64, v1: u64) -> bool {
    let a7407 = v0;
    let a7408 = 1u64;
    if nat_eq(a7407, a7408) {
        true
    } else {
        let v2: u64 = f331(v0);
        let a7409 = 0u64;
        let a7410 = v2;
        if nat_lt(a7409, a7410) {
            let a7414 = {
                let a7411 = v1;
                let a7412 = v2;
                let a7413 = 1u64;
                nat_rem(a7411, a7412, a7413)
            };
            let a7415 = 0u64;
            nat_eq(a7414, a7415)
        } else {
            false
        }
    }
}

pub fn f314(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7416 = v0;
    let a7417 = 28u64;
    if if if if nat_eq(a7416, a7417) {
        let a7418 = v1;
        let a7419 = 0u64;
        nat_eq(a7418, a7419)
    } else {
        false
    } {
        let a7420 = v2;
        let a7421 = 0u64;
        nat_eq(a7420, a7421)
    } else {
        false
    } {
        let a7422 = v3;
        let a7423 = 0u64;
        nat_eq(a7422, a7423)
    } else {
        false
    } {
        let a7424 = v4;
        let a7425 = v8;
        if if if if if if if if nat_eq(a7424, a7425) {
            let a7426 = v5;
            let a7427 = v12;
            nat_eq(a7426, a7427)
        } else {
            false
        } {
            let a7428 = v9;
            let a7429 = v13;
            nat_eq(a7428, a7429)
        } else {
            false
        } {
            let a7430 = v10;
            let a7431 = v14;
            nat_eq(a7430, a7431)
        } else {
            false
        } {
            let a7432 = v11;
            let a7433 = v15;
            nat_eq(a7432, a7433)
        } else {
            false
        } {
            let a7434 = 0u64;
            let a7435 = v6;
            nat_lt(a7434, a7435)
        } else {
            false
        } {
            let a7436 = 0u64;
            let a7437 = v7;
            nat_lt(a7436, a7437)
        } else {
            false
        } {
            let a7441 = {
                let a7438 = v10;
                let a7439 = v6;
                let a7440 = 1u64;
                nat_rem(a7438, a7439, a7440)
            };
            let a7442 = 0u64;
            if nat_eq(a7441, a7442) {
                let a7446 = {
                    let a7443 = v11;
                    let a7444 = v7;
                    let a7445 = 1u64;
                    nat_rem(a7443, a7444, a7445)
                };
                let a7447 = 0u64;
                nat_eq(a7446, a7447)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7448 = v10;
                let a7449 = v6;
                let a7450 = 0u64;
                nat_quot(a7448, a7449, a7450)
            }, {
                let a7451 = v11;
                let a7452 = v7;
                let a7453 = 0u64;
                nat_quot(a7451, a7452, a7453)
            }, {
                let a7454 = v5;
                let a7455 = v4;
                nat_mul(a7454, a7455)?
            }, {
                let a7456 = v4;
                let a7457 = v9;
                nat_mul(a7456, a7457)?
            }, {
                let a7458 = v5;
                let a7459 = v9;
                nat_mul(a7458, a7459)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f315(v0: u64) -> bool {
    let a7460 = v0;
    let a7461 = 0u64;
    if if nat_eq(a7460, a7461) {
        true
    } else {
        let a7462 = v0;
        let a7463 = 1u64;
        nat_eq(a7462, a7463)
    } {
        true
    } else {
        let a7464 = v0;
        let a7465 = 30u64;
        nat_eq(a7464, a7465)
    }
}

pub fn f316(v0: u64) -> Adt17 {
    f318(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f317(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7467 = v0;
        let a7468 = match v1.clone() {
            Adt20::C0(h7466, _, _, _, _, _, _, _) => {
                h7466
            }
        };
        let a7469 = 0u64;
        nat_quot(a7467, a7468, a7469)
    };
    let v3: u64 = {
        let a7471 = v2;
        let a7472 = match v1.clone() {
            Adt20::C0(_, h7470, _, _, _, _, _, _) => {
                h7470
            }
        };
        let a7473 = 0u64;
        nat_quot(a7471, a7472, a7473)
    };
    f145({
        let a7475 = v0;
        let a7476 = match v1.clone() {
            Adt20::C0(h7474, _, _, _, _, _, _, _) => {
                h7474
            }
        };
        let a7477 = 0u64;
        nat_rem(a7475, a7476, a7477)
    }, {
        let a7479 = v2;
        let a7480 = match v1.clone() {
            Adt20::C0(_, h7478, _, _, _, _, _, _) => {
                h7478
            }
        };
        let a7481 = 0u64;
        nat_rem(a7479, a7480, a7481)
    }, {
        let a7483 = v3;
        let a7484 = match v1.clone() {
            Adt20::C0(_, _, h7482, _, _, _, _, _) => {
                h7482
            }
        };
        let a7485 = 0u64;
        nat_rem(a7483, a7484, a7485)
    }, {
        let a7487 = v3;
        let a7488 = match v1.clone() {
            Adt20::C0(_, _, h7486, _, _, _, _, _) => {
                h7486
            }
        };
        let a7489 = 0u64;
        nat_quot(a7487, a7488, a7489)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7490, _, _, _) => {
            h7490
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7491, _, _) => {
            h7491
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7492, _) => {
            h7492
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7493) => {
            h7493
        }
    })
}

pub fn f318(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7494 = v0;
    let a7495 = 4u64;
    if if if nat_le(a7494, a7495) {
        true
    } else {
        let a7496 = 9u64;
        let a7497 = v0;
        if nat_le(a7496, a7497) {
            let a7498 = v0;
            let a7499 = 12u64;
            nat_le(a7498, a7499)
        } else {
            false
        }
    } {
        true
    } else {
        let a7500 = v0;
        let a7501 = 16u64;
        if if nat_eq(a7500, a7501) {
            true
        } else {
            let a7502 = v0;
            let a7503 = 19u64;
            nat_eq(a7502, a7503)
        } {
            true
        } else {
            let a7504 = v0;
            let a7505 = 21u64;
            if nat_eq(a7504, a7505) {
                true
            } else {
                let a7506 = v0;
                let a7507 = 22u64;
                nat_eq(a7506, a7507)
            }
        }
    } {
        Adt17::C1({
            let a7508 = v0;
            let a7509 = 4u64;
            if if nat_lt(a7508, a7509) {
                true
            } else {
                let a7510 = 21u64;
                let a7511 = v0;
                nat_le(a7510, a7511)
            } {
                0u64
            } else {
                let a7512 = v0;
                let a7513 = 16u64;
                if if nat_eq(a7512, a7513) {
                    true
                } else {
                    let a7514 = v0;
                    let a7515 = 19u64;
                    nat_eq(a7514, a7515)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7516 = v0;
            let a7517 = 4u64;
            if nat_lt(a7516, a7517) {
                v0
            } else {
                let a7518 = v0;
                let a7519 = 4u64;
                if if nat_eq(a7518, a7519) {
                    true
                } else {
                    let a7520 = v0;
                    let a7521 = 16u64;
                    nat_eq(a7520, a7521)
                } {
                    0u64
                } else {
                    let a7522 = v0;
                    let a7523 = 19u64;
                    if nat_eq(a7522, a7523) {
                        3u64
                    } else {
                        let a7524 = 21u64;
                        let a7525 = v0;
                        if nat_le(a7524, a7525) {
                            let a7526 = v0;
                            let a7527 = 17u64;
                            nat_sub(a7526, a7527)
                        } else {
                            let a7528 = v0;
                            let a7529 = 8u64;
                            nat_sub(a7528, a7529)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f319(v0: u64) -> R<u64> {
    let a7532 = 2u64;
    let a7533 = {
        let a7530 = 6u64;
        let a7531 = v0;
        nat_mul(a7530, a7531)?
    };
    nat_add(a7532, a7533)
}

pub fn f320(v0: u64) -> R<u64> {
    let a7534 = f319(v0)?;
    let a7535 = 11u64;
    nat_add(a7534, a7535)
}

pub fn f321(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7536 = v0;
        let a7537 = 6u64;
        let a7538 = 0u64;
        nat_rem(a7536, a7537, a7538)
    };
    let v5: u64 = {
        let a7545 = {
            let a7540 = v1;
            let a7541 = match v2.clone() {
                Adt20::C0(h7539, _, _, _, _, _, _, _) => {
                    h7539
                }
            };
            nat_mul(a7540, a7541)?
        };
        let a7546 = {
            let a7542 = v0;
            let a7543 = 6u64;
            let a7544 = 0u64;
            nat_quot(a7542, a7543, a7544)
        };
        nat_add(a7545, a7546)?
    };
    let v6: u64 = f317(v5, v3)?;
    let v7: u64 = f317(v5, v2)?;
    let a7547 = v4;
    let a7548 = 0u64;
    if nat_eq(a7547, a7548) {
        Ok(f318(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7549 = v4;
        let a7550 = 1u64;
        if nat_eq(a7549, a7550) {
            Ok(f318(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7551 = v4;
            let a7552 = 2u64;
            if nat_eq(a7551, a7552) {
                Ok(f318(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7553 = v4;
                let a7554 = 3u64;
                if nat_eq(a7553, a7554) {
                    Ok(f318(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7555 = v4;
                    let a7556 = 4u64;
                    if nat_eq(a7555, a7556) {
                        Ok(f318(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f318(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f322(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7557 = v0;
    let a7558 = 0u64;
    if nat_eq(a7557, a7558) {
        f318(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7559 = v0;
        let a7560 = 1u64;
        if nat_eq(a7559, a7560) {
            f318(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7561 = v0;
            let a7562 = 2u64;
            if nat_eq(a7561, a7562) {
                f318(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7563 = v0;
                let a7564 = 3u64;
                if nat_eq(a7563, a7564) {
                    f318(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7565 = v0;
                    let a7566 = 4u64;
                    if nat_eq(a7565, a7566) {
                        f318(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7567 = v0;
                        let a7568 = 5u64;
                        if nat_eq(a7567, a7568) {
                            f318(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7569 = v0;
                            let a7570 = 6u64;
                            if nat_eq(a7569, a7570) {
                                f318(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7571 = v0;
                                let a7572 = 7u64;
                                if nat_eq(a7571, a7572) {
                                    f318(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7573 = v0;
                                    let a7574 = 8u64;
                                    if nat_eq(a7573, a7574) {
                                        f318(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7575 = v0;
                                        let a7576 = 9u64;
                                        if nat_eq(a7575, a7576) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7577 = v0;
                                            let a7578 = 10u64;
                                            if nat_eq(a7577, a7578) {
                                                f318(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f323(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7579 = v0;
        let a7580 = 3u64;
        let a7581 = 0u64;
        nat_rem(a7579, a7580, a7581)
    };
    let v6: u64 = {
        let a7588 = {
            let a7583 = v1;
            let a7584 = match v2.clone() {
                Adt20::C0(h7582, _, _, _, _, _, _, _) => {
                    h7582
                }
            };
            nat_mul(a7583, a7584)?
        };
        let a7589 = {
            let a7585 = v0;
            let a7586 = 3u64;
            let a7587 = 0u64;
            nat_quot(a7585, a7586, a7587)
        };
        nat_add(a7588, a7589)?
    };
    let v7: u64 = f317(v6, v3)?;
    let v8: u64 = f317(v6, v2)?;
    let v9: u64 = f317(v6, v4)?;
    let a7590 = v5;
    let a7591 = 0u64;
    if nat_eq(a7590, a7591) {
        Ok(f318(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7592 = v5;
        let a7593 = 1u64;
        if nat_eq(a7592, a7593) {
            Ok(f318(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f318(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f324(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7594, _, _, _, _, _, _, _) => {
            h7594
        }
    };
    let v5: u64 = {
        let a7597 = 1u64;
        let a7598 = {
            let a7595 = v4;
            let a7596 = f309(v3);
            nat_mul(a7595, a7596)?
        };
        nat_add(a7597, a7598)?
    };
    let a7599 = v0;
    let a7600 = 0u64;
    if nat_eq(a7599, a7600) {
        Ok(f316(0u64))
    } else {
        let a7601 = v0;
        let a7602 = v5;
        if nat_lt(a7601, a7602) {
            let v6: u64 = {
                let a7603 = v0;
                let a7604 = 1u64;
                nat_sub(a7603, a7604)
            };
            let v7: u64 = f317({
                let a7610 = {
                    let a7605 = v1;
                    let a7606 = v4;
                    nat_mul(a7605, a7606)?
                };
                let a7611 = {
                    let a7607 = v6;
                    let a7608 = f309(v3);
                    let a7609 = 0u64;
                    nat_quot(a7607, a7608, a7609)
                };
                nat_add(a7610, a7611)?
            }, v2)?;
            if v3 {
                let a7615 = {
                    let a7612 = v6;
                    let a7613 = 2u64;
                    let a7614 = 0u64;
                    nat_rem(a7612, a7613, a7614)
                };
                let a7616 = 0u64;
                if nat_eq(a7615, a7616) {
                    Ok(f318(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f318(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f318(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7617 = v0;
                let a7618 = v5;
                nat_sub(a7617, a7618)
            };
            let a7619 = v8;
            let a7620 = 0u64;
            if nat_eq(a7619, a7620) {
                Ok(f318(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7621 = v8;
                    let a7622 = 1u64;
                    nat_eq(a7621, a7622)
                } else {
                    false
                } {
                    Ok(f318(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7623 = v8;
                    let a7624 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7623, a7624) {
                        Ok(f318(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7625 = v8;
                        let a7626 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7625, a7626) {
                            Ok(f316(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f325(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7628 = 13u64;
        let a7629 = f310(match v2.clone() {
            Adt20::C0(h7627, _, _, _, _, _, _, _) => {
                h7627
            }
        });
        nat_mul(a7628, a7629)?
    } else {
        0u64
    };
    let a7630 = v0;
    let a7631 = v5;
    if nat_lt(a7630, a7631) {
        f332({
            let a7632 = v0;
            let a7633 = 13u64;
            let a7634 = 0u64;
            nat_rem(a7632, a7633, a7634)
        }, v1, {
            let a7635 = v0;
            let a7636 = 13u64;
            let a7637 = 0u64;
            nat_quot(a7635, a7636, a7637)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7638 = v0;
            let a7639 = v5;
            nat_sub(a7638, a7639)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f333({
            let a7640 = v6;
            let a7641 = v7;
            let a7642 = 0u64;
            nat_rem(a7640, a7641, a7642)
        }, {
            let a7652 = {
                let a7647 = {
                    let a7644 = v1;
                    let a7645 = match v2.clone() {
                        Adt20::C0(h7643, _, _, _, _, _, _, _) => {
                            h7643
                        }
                    };
                    nat_mul(a7644, a7645)?
                };
                let a7648 = if v4 {
                    f334(match v2.clone() {
                        Adt20::C0(h7646, _, _, _, _, _, _, _) => {
                            h7646
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7647, a7648)?
            };
            let a7653 = {
                let a7649 = v6;
                let a7650 = v7;
                let a7651 = 0u64;
                nat_quot(a7649, a7650, a7651)
            };
            nat_add(a7652, a7653)?
        }, v2, v3, v4)
    }
}

pub fn f326(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7654 = v0;
    let a7655 = 0u64;
    if nat_eq(a7654, a7655) {
        f318(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7656 = v0;
        let a7657 = 1u64;
        if nat_eq(a7656, a7657) {
            f318(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7658 = v0;
                let a7659 = 2u64;
                nat_eq(a7658, a7659)
            } else {
                false
            } {
                f318(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7660 = v0;
                let a7661 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7660, a7661) {
                    f318(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7662 = v0;
                    let a7663 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7662, a7663) {
                        f318(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7664 = v0;
                        let a7665 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7664, a7665) {
                            f318(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f327(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7669 = 1u64;
        let a7670 = {
            let a7667 = match v4.clone() {
                Adt20::C0(h7666, _, _, _, _, _, _, _) => {
                    h7666
                }
            };
            let a7668 = v6;
            nat_mul(a7667, a7668)?
        };
        nat_add(a7669, a7670)?
    };
    let a7671 = v1;
    let a7672 = 0u64;
    if nat_eq(a7671, a7672) {
        Ok(f316(1u64))
    } else {
        let a7673 = v1;
        let a7674 = v7;
        if nat_lt(a7673, a7674) {
            let v8: u64 = {
                let a7675 = v1;
                let a7676 = 1u64;
                nat_sub(a7675, a7676)
            };
            let v9: u64 = {
                let a7683 = {
                    let a7678 = f330(v2, v3, v4.clone())?;
                    let a7679 = match v4.clone() {
                        Adt20::C0(h7677, _, _, _, _, _, _, _) => {
                            h7677
                        }
                    };
                    nat_mul(a7678, a7679)?
                };
                let a7684 = {
                    let a7680 = v8;
                    let a7681 = v6;
                    let a7682 = 0u64;
                    nat_quot(a7680, a7681, a7682)
                };
                nat_add(a7683, a7684)?
            };
            let v10: u64 = f317(v9, v4)?;
            let v11: u64 = f317(v9, v5)?;
            let v12: u64 = {
                let a7685 = v8;
                let a7686 = v6;
                let a7687 = 0u64;
                nat_rem(a7685, a7686, a7687)
            };
            if v0 {
                let a7688 = v12;
                let a7689 = 0u64;
                if nat_eq(a7688, a7689) {
                    Ok(f318(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7690 = v12;
                    let a7691 = 1u64;
                    if nat_eq(a7690, a7691) {
                        Ok(f318(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7692 = v12;
                        let a7693 = 2u64;
                        if nat_eq(a7692, a7693) {
                            Ok(f318(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f318(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7694 = v12;
                let a7695 = 0u64;
                if nat_eq(a7694, a7695) {
                    Ok(f318(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f318(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7696 = v1;
            let a7697 = v7;
            if nat_eq(a7696, a7697) {
                Ok(f318(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f328(v0: u64, v1: u64) -> Adt17 {
    let a7698 = v0;
    let a7699 = 0u64;
    if nat_eq(a7698, a7699) {
        f318(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7700 = v0;
        let a7701 = 1u64;
        if nat_eq(a7700, a7701) {
            f318(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7702 = v0;
            let a7703 = 2u64;
            if nat_eq(a7702, a7703) {
                f318(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7704 = v0;
                let a7705 = 3u64;
                if nat_eq(a7704, a7705) {
                    f316(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f329(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7706 = v0;
    let a7707 = 0u64;
    if nat_eq(a7706, a7707) {
        f318(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7708 = v0;
        let a7709 = 1u64;
        if nat_eq(a7708, a7709) {
            f318(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7710 = v0;
            let a7711 = 2u64;
            if nat_eq(a7710, a7711) {
                f318(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7712 = v0;
                let a7713 = 3u64;
                if nat_eq(a7712, a7713) {
                    f318(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7714 = v0;
                    let a7715 = 4u64;
                    if nat_eq(a7714, a7715) {
                        f318(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7716 = v0;
                        let a7717 = 5u64;
                        if nat_eq(a7716, a7717) {
                            f318(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f330(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7728 = {
        let a7726 = {
            let a7723 = {
                let a7720 = match v1.clone() {
                    Adt21::C0(_, _, h7718, _) => {
                        h7718
                    }
                };
                let a7721 = match v2.clone() {
                    Adt20::C0(_, _, h7719, _, _, _, _, _) => {
                        h7719
                    }
                };
                nat_mul(a7720, a7721)?
            };
            let a7724 = match v1 {
                Adt21::C0(h7722, _, _, _) => {
                    h7722
                }
            };
            nat_add(a7723, a7724)?
        };
        let a7727 = match v2 {
            Adt20::C0(_, h7725, _, _, _, _, _, _) => {
                h7725
            }
        };
        nat_mul(a7726, a7727)?
    };
    let a7729 = v0;
    nat_add(a7728, a7729)
}

pub fn f331(v0: u64) -> u64 {
    if f229(v0) {
        let m7730 = f221(v0);
        match m7730 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7732 = 1u64;
                let a7733 = match v1.clone() {
                    Adt19::C0(_, h7731, _) => {
                        h7731
                    }
                };
                if if nat_lt(a7732, a7733) {
                    true
                } else {
                    let a7734 = v0;
                    let a7735 = 43u64;
                    nat_eq(a7734, a7735)
                } {
                    match v1 {
                        Adt19::C0(_, h7736, _) => {
                            h7736
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

pub fn f332(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7744 = {
            let a7742 = {
                let a7738 = v1;
                let a7739 = match v3.clone() {
                    Adt20::C0(h7737, _, _, _, _, _, _, _) => {
                        h7737
                    }
                };
                nat_mul(a7738, a7739)?
            };
            let a7743 = {
                let a7740 = v2;
                let a7741 = 4u64;
                nat_mul(a7740, a7741)?
            };
            nat_add(a7742, a7743)?
        };
        let a7745 = f335(v0);
        nat_add(a7744, a7745)?
    };
    let v6: u64 = f317(v5, v3)?;
    let v7: u64 = f317(v5, v4)?;
    let a7746 = v0;
    let a7747 = 0u64;
    if if if if nat_eq(a7746, a7747) {
        true
    } else {
        let a7748 = v0;
        let a7749 = 2u64;
        nat_eq(a7748, a7749)
    } {
        true
    } else {
        let a7750 = v0;
        let a7751 = 5u64;
        nat_eq(a7750, a7751)
    } {
        true
    } else {
        let a7752 = v0;
        let a7753 = 7u64;
        nat_eq(a7752, a7753)
    } {
        Ok(f318(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7754 = v0;
        let a7755 = 1u64;
        if if if if nat_eq(a7754, a7755) {
            true
        } else {
            let a7756 = v0;
            let a7757 = 3u64;
            nat_eq(a7756, a7757)
        } {
            true
        } else {
            let a7758 = v0;
            let a7759 = 6u64;
            nat_eq(a7758, a7759)
        } {
            true
        } else {
            let a7760 = v0;
            let a7761 = 8u64;
            nat_eq(a7760, a7761)
        } {
            Ok(f318(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7762 = v0;
                let a7763 = 1u64;
                if nat_eq(a7762, a7763) {
                    4u64
                } else {
                    let a7764 = v0;
                    let a7765 = 6u64;
                    if nat_eq(a7764, a7765) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7766 = v0;
            let a7767 = 4u64;
            if nat_eq(a7766, a7767) {
                Ok(f318(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7768 = v0;
                let a7769 = 9u64;
                if nat_eq(a7768, a7769) {
                    Ok(f318(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7770 = v0;
                    let a7771 = 10u64;
                    if nat_eq(a7770, a7771) {
                        Ok(f318(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7772 = v0;
                        let a7773 = 11u64;
                        if nat_eq(a7772, a7773) {
                            Ok(f318(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7774 = v0;
                            let a7775 = 12u64;
                            if nat_eq(a7774, a7775) {
                                Ok(f318(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f333(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f317(v1, v2)?;
    let v6: u64 = f317(v1, v3)?;
    let a7776 = v0;
    let a7777 = 0u64;
    if nat_eq(a7776, a7777) {
        Ok(f318(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7778 = v0;
        let a7779 = 1u64;
        if nat_eq(a7778, a7779) {
            Ok(f318(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7780 = v0;
                let a7781 = 2u64;
                if nat_eq(a7780, a7781) {
                    Ok(f318(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7782 = v0;
                    let a7783 = 3u64;
                    if nat_eq(a7782, a7783) {
                        Ok(f318(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7784 = v0;
                let a7785 = 2u64;
                if nat_eq(a7784, a7785) {
                    Ok(f318(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f334(v0: u64) -> R<u64> {
    let a7786 = 4u64;
    let a7787 = f310(v0);
    nat_mul(a7786, a7787)
}

pub fn f335(v0: u64) -> u64 {
    let a7788 = v0;
    let a7789 = 5u64;
    if nat_lt(a7788, a7789) {
        let a7790 = v0;
        let a7791 = 2u64;
        if nat_lt(a7790, a7791) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7792 = v0;
        let a7793 = 7u64;
        if nat_lt(a7792, a7793) {
            2u64
        } else {
            3u64
        }
    }
}
