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
    C46(u64, u64),
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
            let a3 = 80u64;
            if nat_eq(a2, a3) {
                Ok(Adt9::C0(f1(match v2 {
                    Adt1::C0(h4, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4
                    }
                }, v3, v4, v5, v6, v7, v8, v11)?))
            } else {
                let a5 = v1;
                let a6 = 82u64;
                if if if if if nat_eq(a5, a6) {
                    true
                } else {
                    let a7 = v1;
                    let a8 = 50u64;
                    nat_eq(a7, a8)
                } {
                    true
                } else {
                    let a9 = v1;
                    let a10 = 59u64;
                    nat_eq(a9, a10)
                } {
                    true
                } else {
                    let a11 = v1;
                    let a12 = 65u64;
                    nat_eq(a11, a12)
                } {
                    if if if f2(v5.clone()) {
                        true
                    } else {
                        f2(v6.clone())
                    } {
                        true
                    } else {
                        f2(v7.clone())
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
                    let a13 = v1;
                    let a14 = 94u64;
                    if if nat_eq(a13, a14) {
                        true
                    } else {
                        let a15 = v1;
                        let a16 = 95u64;
                        nat_eq(a15, a16)
                    } {
                        Ok(Adt9::C0(f3({
                            let a17 = v1;
                            let a18 = 95u64;
                            nat_eq(a17, a18)
                        }, v3, v4, v5, {
                            let m19 = v6;
                            match m19 {
                                None => {
                                    let m20 = v7;
                                    match m20 {
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
                        let a21 = v1;
                        let a22 = 96u64;
                        if if nat_eq(a21, a22) {
                            true
                        } else {
                            let a23 = v1;
                            let a24 = 97u64;
                            nat_eq(a23, a24)
                        } {
                            Ok(Adt9::C0(f4({
                                let a25 = v1;
                                let a26 = 96u64;
                                nat_eq(a25, a26)
                            }, v9, v10, v3, v4, v5, v6, v7, v8, v11)?))
                        } else {
                            let v15: bool = {
                                let m27 = v7;
                                match m27 {
                                    None => {
                                        v8
                                    }
                                    Some(_) => {
                                        true
                                    }
                                }
                            };
                            let a28 = v1;
                            let a29 = 85u64;
                            if nat_eq(a28, a29) {
                                Ok(Adt9::C0(f5(v2, v3, v4, v5, {
                                    let m30 = v6;
                                    match m30 {
                                        None => {
                                            v15
                                        }
                                        Some(_) => {
                                            true
                                        }
                                    }
                                }, v11)?))
                            } else {
                                let a31 = v1;
                                let a32 = 87u64;
                                if nat_eq(a31, a32) {
                                    Ok(Adt9::C0(f6(v3, v4, v5, v6, v15, v11)?))
                                } else {
                                    let a33 = v1;
                                    let a34 = 64u64;
                                    if nat_eq(a33, a34) {
                                        Ok(Adt9::C0(f7(v2, v11)?))
                                    } else {
                                        let m35 = v3;
                                        match m35 {
                                            None => {
                                                Ok(Adt9::C0(Adt10::C1))
                                            }
                                            Some(v17) => {
                                                let a36 = v1;
                                                let a37 = 48u64;
                                                if if nat_eq(a36, a37) {
                                                    true
                                                } else {
                                                    let a38 = v1;
                                                    let a39 = 56u64;
                                                    nat_eq(a38, a39)
                                                } {
                                                    Ok(Adt9::C0(f8({
                                                        let a40 = v1;
                                                        let a41 = 56u64;
                                                        nat_eq(a40, a41)
                                                    }, v2, v17, v4, v11)?))
                                                } else {
                                                    let a42 = v1;
                                                    let a43 = 53u64;
                                                    if if nat_eq(a42, a43) {
                                                        true
                                                    } else {
                                                        let a44 = v1;
                                                        let a45 = 54u64;
                                                        nat_eq(a44, a45)
                                                    } {
                                                        Ok(Adt9::C0(f9({
                                                            let a46 = v1;
                                                            let a47 = 54u64;
                                                            nat_eq(a46, a47)
                                                        }, v2, v17, v4, v11)?))
                                                    } else {
                                                        let a48 = v1;
                                                        let a49 = 78u64;
                                                        if nat_eq(a48, a49) {
                                                            Ok(Adt9::C0(f10(v2, v17, v4, v5, v11)?))
                                                        } else {
                                                            let a50 = v1;
                                                            let a51 = 23u64;
                                                            if if if nat_eq(a50, a51) {
                                                                true
                                                            } else {
                                                                let a52 = v1;
                                                                let a53 = 26u64;
                                                                nat_eq(a52, a53)
                                                            } {
                                                                true
                                                            } else {
                                                                let a54 = v1;
                                                                let a55 = 25u64;
                                                                nat_eq(a54, a55)
                                                            } {
                                                                Ok(Adt9::C0(f11(v1, v2, v17, v4, v11)?))
                                                            } else {
                                                                let a56 = v1;
                                                                let a57 = 24u64;
                                                                if nat_eq(a56, a57) {
                                                                    Ok(Adt9::C0(f12(v2, v17, v11)?))
                                                                } else {
                                                                    let a58 = v1;
                                                                    let a59 = 27u64;
                                                                    if nat_eq(a58, a59) {
                                                                        Ok(Adt9::C0(f13(v2, v17, v11)?))
                                                                    } else {
                                                                        let a60 = v1;
                                                                        let a61 = 3u64;
                                                                        if nat_eq(a60, a61) {
                                                                            Ok(Adt9::C0(f14(v17, v4, v5, v11)?))
                                                                        } else {
                                                                            f15(v1, v2, v17, v4, v11)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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
            f18(v23, v24, v25, v26, v27)
        }
        Adt0::C2(v38) => {
            f20(v38)
        }
        Adt0::C3(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51) => {
            Ok(f21(v39, v40, v41, v42, v43, v44, v45, v46, v47, v48, v49, v50, v51))
        }
        Adt0::C4(v18, v19, v20, v21) => {
            let a63 = 0u64;
            let a64 = match v18.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h62, _, _, _, _, _, _, _) => {
                    h62
                }
            };
            if nat_lt(a63, a64) {
                f16(v18, v19, v20, v21)
            } else {
                f17(v18, v19, v20, v21)
            }
        }
        Adt0::C5(v22) => {
            Ok(Adt9::C9({
                let a65 = 3240099840u64;
                let a66 = v22;
                if if nat_le(a65, a66) {
                    let a67 = v22;
                    let a68 = 4286578688u64;
                    nat_le(a67, a68)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a69 = 1092616192u64;
                    let a70 = v22;
                    if if nat_le(a69, a70) {
                        let a71 = v22;
                        let a72 = 2139095040u64;
                        nat_le(a71, a72)
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
            f19(v28, v29, v30, v31, v32, true, v33, v34, v35, v36, v37)
        }
    }
}

pub fn f1(v0: u64, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: bool, v7: Adt2) -> R<Adt10> {
    if v6 {
        Ok(Adt10::C1)
    } else {
        let m73 = v1;
        match m73 {
            None => {
                Ok(Adt10::C1)
            }
            Some(v8) => {
                let m74 = v2;
                match m74 {
                    None => {
                        Ok(Adt10::C1)
                    }
                    Some(v9) => {
                        let m75 = v3;
                        match m75 {
                            None => {
                                Ok(Adt10::C1)
                            }
                            Some(v10) => {
                                let m76 = v4;
                                match m76 {
                                    None => {
                                        Ok(Adt10::C1)
                                    }
                                    Some(v11) => {
                                        let m77 = v5;
                                        match m77 {
                                            None => {
                                                Ok(Adt10::C1)
                                            }
                                            Some(v12) => {
                                                let v13: u64 = match v8.clone() {
                                                    Adt2::C0(_, h78, _, _, _, _, _, _, _, _, _) => {
                                                        h78
                                                    }
                                                };
                                                let v14: u64 = match v8.clone() {
                                                    Adt2::C0(_, _, h79, _, _, _, _, _, _, _, _) => {
                                                        h79
                                                    }
                                                };
                                                let v15: u64 = match v8.clone() {
                                                    Adt2::C0(_, _, _, h80, _, _, _, _, _, _, _) => {
                                                        h80
                                                    }
                                                };
                                                let v16: u64 = match v12.clone() {
                                                    Adt2::C0(_, _, h81, _, _, _, _, _, _, _, _) => {
                                                        h81
                                                    }
                                                };
                                                let v17: u64 = {
                                                    let a82 = v13;
                                                    let a83 = v14;
                                                    nat_mul(a82, a83)?
                                                };
                                                let v18: u64 = {
                                                    let a84 = v17;
                                                    let a85 = v15;
                                                    nat_mul(a84, a85)?
                                                };
                                                let v19: u64 = {
                                                    let a88 = {
                                                        let a86 = v17;
                                                        let a87 = v13;
                                                        nat_mul(a86, a87)?
                                                    };
                                                    let a89 = v16;
                                                    nat_mul(a88, a89)?
                                                };
                                                let a90 = 0u64;
                                                let a91 = v14;
                                                if if if if if if if if if if if if if if if if if nat_lt(a90, a91) {
                                                    let a92 = 0u64;
                                                    let a93 = v15;
                                                    nat_lt(a92, a93)
                                                } else {
                                                    false
                                                } {
                                                    let a94 = 0u64;
                                                    let a95 = v16;
                                                    nat_lt(a94, a95)
                                                } else {
                                                    false
                                                } {
                                                    let a99 = {
                                                        let a96 = v15;
                                                        let a97 = v16;
                                                        let a98 = 0u64;
                                                        nat_rem(a96, a97, a98)
                                                    };
                                                    let a100 = 0u64;
                                                    nat_eq(a99, a100)
                                                } else {
                                                    false
                                                } {
                                                    let a102 = match v7.clone() {
                                                        Adt2::C0(h101, _, _, _, _, _, _, _, _, _, _) => {
                                                            h101
                                                        }
                                                    };
                                                    let a103 = 0u64;
                                                    nat_eq(a102, a103)
                                                } else {
                                                    false
                                                } {
                                                    f22(v7.clone())?
                                                } else {
                                                    false
                                                } {
                                                    let a105 = match v7.clone() {
                                                        Adt2::C0(_, h104, _, _, _, _, _, _, _, _, _) => {
                                                            h104
                                                        }
                                                    };
                                                    let a106 = v17;
                                                    nat_eq(a105, a106)
                                                } else {
                                                    false
                                                } {
                                                    let a112 = match v7.clone() {
                                                        Adt2::C0(_, _, h107, _, _, _, _, _, _, _, _) => {
                                                            h107
                                                        }
                                                    };
                                                    let a113 = {
                                                        let a110 = v15;
                                                        let a111 = {
                                                            let a108 = v13;
                                                            let a109 = v16;
                                                            nat_mul(a108, a109)?
                                                        };
                                                        nat_add(a110, a111)?
                                                    };
                                                    nat_eq(a112, a113)
                                                } else {
                                                    false
                                                } {
                                                    let a115 = match v7.clone() {
                                                        Adt2::C0(_, _, _, h114, _, _, _, _, _, _, _) => {
                                                            h114
                                                        }
                                                    };
                                                    let a116 = 1u64;
                                                    nat_eq(a115, a116)
                                                } else {
                                                    false
                                                } {
                                                    let a118 = match v7.clone() {
                                                        Adt2::C0(_, _, _, _, h117, _, _, _, _, _, _) => {
                                                            h117
                                                        }
                                                    };
                                                    let a119 = 1u64;
                                                    nat_eq(a118, a119)
                                                } else {
                                                    false
                                                } {
                                                    let a120 = f23(v12.clone(), 1u64)?;
                                                    let a121 = v19;
                                                    nat_eq(a120, a121)
                                                } else {
                                                    false
                                                } {
                                                    f24(v8.clone(), v7.clone(), v18)?
                                                } else {
                                                    false
                                                } {
                                                    f25(v9, v8.clone(), v7.clone(), v18)?
                                                } else {
                                                    false
                                                } {
                                                    f25(v10, v8.clone(), v7.clone(), v18)?
                                                } else {
                                                    false
                                                } {
                                                    f25(v11, v8.clone(), v7.clone(), v18)?
                                                } else {
                                                    false
                                                } {
                                                    f24(v12, v7.clone(), v19)?
                                                } else {
                                                    false
                                                } {
                                                    let a127 = {
                                                        let a124 = 4u64;
                                                        let a125 = {
                                                            let a122 = v18;
                                                            let a123 = v19;
                                                            nat_add(a122, a123)?
                                                        };
                                                        nat_mul(a124, a125)?
                                                    };
                                                    let a128 = match v7.clone() {
                                                        Adt2::C0(_, _, _, _, _, _, _, _, _, h126, _) => {
                                                            h126
                                                        }
                                                    };
                                                    nat_le(a127, a128)
                                                } else {
                                                    false
                                                } {
                                                    Ok(Adt10::C4(f26(v13, v18)?, 4u64, f27(v8, 1u64), f27(v7, 1u64), Adt4::C46(v0, v16)))
                                                } else {
                                                    Ok(Adt10::C1)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn f2(v0: Option<Adt2>) -> bool {
    let m129 = v0;
    match m129 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    }
}

pub fn f3(v0: bool, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m130 = v1.clone();
        match m130 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v10: Adt2 = if v0 {
        let m131 = v2.clone();
        match m131 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    } else {
        let m132 = v1.clone();
        match m132 {
            None => {
                v5.clone()
            }
            Some(v9) => {
                v9
            }
        }
    };
    let v13: Adt2 = if v0 {
        let m133 = v3.clone();
        match m133 {
            None => {
                v5.clone()
            }
            Some(v11) => {
                v11
            }
        }
    } else {
        let m134 = v2.clone();
        match m134 {
            None => {
                v5.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v14: u64 = {
        let a140 = {
            let a137 = match v10.clone() {
                Adt2::C0(_, _, h135, _, _, _, _, _, _, _, _) => {
                    h135
                }
            };
            let a138 = match v10.clone() {
                Adt2::C0(_, _, _, h136, _, _, _, _, _, _, _) => {
                    h136
                }
            };
            nat_mul(a137, a138)?
        };
        let a141 = match v10.clone() {
            Adt2::C0(_, _, _, _, h139, _, _, _, _, _, _) => {
                h139
            }
        };
        nat_mul(a140, a141)?
    };
    let m142 = v1;
    if if if if if if if if if if match m142 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m143 = v2;
        match m143 {
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
            let m144 = v3;
            match m144 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            }
        } else {
            let m145 = v3;
            !match m145 {
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
        let a146 = 0u64;
        let a147 = v14;
        nat_lt(a146, a147)
    } else {
        false
    } {
        let a149 = 0u64;
        let a150 = match v10.clone() {
            Adt2::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                h148
            }
        };
        nat_lt(a149, a150)
    } else {
        false
    } {
        f28(v10.clone(), v13.clone())
    } else {
        false
    } {
        f29(v10.clone(), v14, v0)?
    } else {
        false
    } {
        f29(v13.clone(), v14, v0)?
    } else {
        false
    } {
        if v0 {
            let a152 = match v7.clone() {
                Adt2::C0(h151, _, _, _, _, _, _, _, _, _, _) => {
                    h151
                }
            };
            let a153 = 0u64;
            if if if if nat_eq(a152, a153) {
                let a154 = f23(v7.clone(), 1u64)?;
                let a155 = 1u64;
                nat_eq(a154, a155)
            } else {
                false
            } {
                let a157 = 4u64;
                let a158 = match v7 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h156, _) => {
                        h156
                    }
                };
                nat_le(a157, a158)
            } else {
                false
            } {
                f28(v10.clone(), v5.clone())
            } else {
                false
            } {
                f29(v5.clone(), v14, true)?
            } else {
                false
            }
        } else {
            let a160 = match v5.clone() {
                Adt2::C0(h159, _, _, _, _, _, _, _, _, _, _) => {
                    h159
                }
            };
            let a161 = 0u64;
            if if nat_eq(a160, a161) {
                let a162 = f23(v5.clone(), 1u64)?;
                let a163 = 1u64;
                nat_eq(a162, a163)
            } else {
                false
            } {
                let a165 = 4u64;
                let a166 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h164, _) => {
                        h164
                    }
                };
                nat_le(a165, a166)
            } else {
                false
            }
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a172 = {
                let a170 = 2u64;
                let a171 = {
                    let a168 = v14;
                    let a169 = f30(v0, match v10.clone() {
                        Adt2::C0(_, h167, _, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    })?;
                    nat_mul(a168, a169)?
                };
                nat_add(a170, a171)?
            };
            let a173 = if v0 {
                0u64
            } else {
                1u64
            };
            nat_add(a172, a173)?
        }, 4u64, f27(v10, 1u64), f27(v5, 1u64), Adt4::C40(v0, match v13 {
            Adt2::C0(_, _, _, _, _, _, h174, _, _, _, _) => {
                h174
            }
        }, v14)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: bool, v1: u64, v2: bool, v3: Option<Adt2>, v4: Option<Adt2>, v5: Option<Adt2>, v6: Option<Adt2>, v7: Option<Adt2>, v8: bool, v9: Adt2) -> R<Adt10> {
    let v11: Adt2 = {
        let m175 = v3.clone();
        match m175 {
            None => {
                v9.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v13: Adt2 = {
        let m176 = v4.clone();
        match m176 {
            None => {
                v9.clone()
            }
            Some(v12) => {
                v12
            }
        }
    };
    let v15: Adt2 = {
        let m177 = v5.clone();
        match m177 {
            None => {
                v9.clone()
            }
            Some(v14) => {
                v14
            }
        }
    };
    let v17: Adt2 = {
        let m178 = v6.clone();
        match m178 {
            None => {
                v9.clone()
            }
            Some(v16) => {
                v16
            }
        }
    };
    let v20: Adt2 = if v0 {
        let m179 = v7.clone();
        match m179 {
            None => {
                v9.clone()
            }
            Some(v18) => {
                v18
            }
        }
    } else {
        let m180 = v5.clone();
        match m180 {
            None => {
                v9.clone()
            }
            Some(v19) => {
                v19
            }
        }
    };
    let m181 = v3;
    if if if if if if if if if if if match m181 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m182 = v4;
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
        let m183 = v5;
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
        if v0 {
            let m184 = v6;
            if match m184 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m185 = v7;
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
            }
        } else {
            let m186 = v6;
            if !match m186 {
                None => {
                    false
                }
                Some(_) => {
                    true
                }
            } {
                let m187 = v7;
                !match m187 {
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
            let a194 = {
                let a191 = {
                    let a188 = v1;
                    let a189 = 4u64;
                    let a190 = 0u64;
                    nat_quot(a188, a189, a190)
                };
                let a192 = 2u64;
                let a193 = 0u64;
                nat_rem(a191, a192, a193)
            };
            let a195 = 1u64;
            nat_eq(a194, a195)
        } else {
            false
        } {
            let a197 = match v11.clone() {
                Adt2::C0(h196, _, _, _, _, _, _, _, _, _, _) => {
                    h196
                }
            };
            let a198 = 0u64;
            nat_eq(a197, a198)
        } else {
            false
        } {
            let a200 = match v11.clone() {
                Adt2::C0(_, _, _, _, _, h199, _, _, _, _, _) => {
                    h199
                }
            };
            let a201 = 4u64;
            nat_eq(a200, a201)
        } else {
            false
        } {
            f31(v11.clone(), f23(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f32(v11.clone(), v9.clone())?
        } else {
            false
        } {
            let a203 = match v11.clone() {
                Adt2::C0(_, h202, _, _, _, _, _, _, _, _, _) => {
                    h202
                }
            };
            let a204 = 1u64;
            if nat_le(a203, a204) {
                true
            } else {
                let a207 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, h205, _, _, _, _, _) => {
                        h205
                    }
                };
                let a208 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, h206, _, _, _, _, _) => {
                        h206
                    }
                };
                nat_eq(a207, a208)
            }
        } else {
            false
        } {
            let a210 = match v11.clone() {
                Adt2::C0(_, _, h209, _, _, _, _, _, _, _, _) => {
                    h209
                }
            };
            let a211 = 1u64;
            if nat_le(a210, a211) {
                true
            } else {
                let a214 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, h212, _, _, _, _) => {
                        h212
                    }
                };
                let a215 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, h213, _, _, _, _) => {
                        h213
                    }
                };
                nat_eq(a214, a215)
            }
        } else {
            false
        } {
            let a217 = match v11.clone() {
                Adt2::C0(_, _, _, h216, _, _, _, _, _, _, _) => {
                    h216
                }
            };
            let a218 = 1u64;
            if nat_le(a217, a218) {
                true
            } else {
                let a221 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h219, _, _, _) => {
                        h219
                    }
                };
                let a222 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h220, _, _, _) => {
                        h220
                    }
                };
                nat_eq(a221, a222)
            }
        } else {
            false
        } {
            let a224 = match v11.clone() {
                Adt2::C0(_, _, _, _, h223, _, _, _, _, _, _) => {
                    h223
                }
            };
            let a225 = 1u64;
            if nat_le(a224, a225) {
                true
            } else {
                let a228 = match v11.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h226, _, _) => {
                        h226
                    }
                };
                let a229 = match v9.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h227, _, _) => {
                        h227
                    }
                };
                nat_eq(a228, a229)
            }
        } else {
            false
        }
    } else {
        false
    } {
        f32(v11.clone(), v13)?
    } else {
        false
    } {
        if v0 {
            if f32(v11.clone(), v15)? {
                f32(v11.clone(), v17)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        let a231 = match v20.clone() {
            Adt2::C0(h230, _, _, _, _, _, _, _, _, _, _) => {
                h230
            }
        };
        let a232 = 0u64;
        nat_eq(a231, a232)
    } else {
        false
    } {
        let a233 = f23(v20.clone(), 1u64)?;
        let a234 = if v0 {
            7u64
        } else {
            2u64
        };
        nat_eq(a233, a234)
    } else {
        false
    } {
        let a236 = if v0 {
            28u64
        } else {
            8u64
        };
        let a237 = match v20 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h235, _) => {
                h235
            }
        };
        nat_le(a236, a237)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a240 = f33(v0);
            let a241 = {
                let a238 = f23(v11.clone(), 1u64)?;
                let a239 = f34(v0);
                nat_mul(a238, a239)?
            };
            nat_add(a240, a241)?
        }, 4u64, f27(v11, 1u64), f27(v9, 1u64), Adt4::C39(v0)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f5(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m242 = v1.clone();
        match m242 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m243 = v2.clone();
        match m243 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m244 = v3.clone();
        match m244 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h245, _, _, _, _, _, _, _, _, _, _) => {
            h245
        }
    };
    let m246 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m246 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m247 = v2;
        match m247 {
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
        let m248 = v3;
        match m248 {
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
        let a250 = match v7.clone() {
            Adt2::C0(h249, _, _, _, _, _, _, _, _, _, _) => {
                h249
            }
        };
        let a251 = 0u64;
        if if nat_eq(a250, a251) {
            f31(v7.clone(), f23(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a253 = match v9.clone() {
            Adt2::C0(h252, _, _, _, _, _, _, _, _, _, _) => {
                h252
            }
        };
        let a254 = 0u64;
        if if nat_eq(a253, a254) {
            f31(v9.clone(), f23(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a256 = match v11.clone() {
            Adt2::C0(h255, _, _, _, _, _, _, _, _, _, _) => {
                h255
            }
        };
        let a257 = 0u64;
        if if nat_eq(a256, a257) {
            f31(v11.clone(), f23(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a259 = match v7.clone() {
            Adt2::C0(_, h258, _, _, _, _, _, _, _, _, _) => {
                h258
            }
        };
        let a260 = 24u64;
        nat_eq(a259, a260)
    } else {
        false
    } {
        let a262 = match v7.clone() {
            Adt2::C0(_, _, _, h261, _, _, _, _, _, _, _) => {
                h261
            }
        };
        let a263 = 1u64;
        nat_eq(a262, a263)
    } else {
        false
    } {
        let a265 = match v7.clone() {
            Adt2::C0(_, _, _, _, h264, _, _, _, _, _, _) => {
                h264
            }
        };
        let a266 = 1u64;
        nat_eq(a265, a266)
    } else {
        false
    } {
        let a268 = match v11.clone() {
            Adt2::C0(_, h267, _, _, _, _, _, _, _, _, _) => {
                h267
            }
        };
        let a269 = 24u64;
        nat_eq(a268, a269)
    } else {
        false
    } {
        let a271 = match v11.clone() {
            Adt2::C0(_, _, h270, _, _, _, _, _, _, _, _) => {
                h270
            }
        };
        let a272 = 1u64;
        nat_eq(a271, a272)
    } else {
        false
    } {
        let a274 = match v11.clone() {
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
            Adt2::C0(_, _, _, _, h276, _, _, _, _, _, _) => {
                h276
            }
        };
        let a278 = 1u64;
        nat_eq(a277, a278)
    } else {
        false
    } {
        let a280 = match v9.clone() {
            Adt2::C0(_, _, h279, _, _, _, _, _, _, _, _) => {
                h279
            }
        };
        let a281 = 1u64;
        nat_eq(a280, a281)
    } else {
        false
    } {
        let a283 = match v9.clone() {
            Adt2::C0(_, _, _, h282, _, _, _, _, _, _, _) => {
                h282
            }
        };
        let a284 = 1u64;
        nat_eq(a283, a284)
    } else {
        false
    } {
        let a286 = match v9.clone() {
            Adt2::C0(_, _, _, _, h285, _, _, _, _, _, _) => {
                h285
            }
        };
        let a287 = 1u64;
        nat_eq(a286, a287)
    } else {
        false
    } {
        let a289 = 3u64;
        let a290 = match v9.clone() {
            Adt2::C0(_, h288, _, _, _, _, _, _, _, _, _) => {
                h288
            }
        };
        nat_le(a289, a290)
    } else {
        false
    } {
        let a292 = match v5.clone() {
            Adt2::C0(_, h291, _, _, _, _, _, _, _, _, _) => {
                h291
            }
        };
        let a293 = 4u64;
        nat_eq(a292, a293)
    } else {
        false
    } {
        let a295 = match v5.clone() {
            Adt2::C0(_, _, h294, _, _, _, _, _, _, _, _) => {
                h294
            }
        };
        let a296 = 4u64;
        nat_eq(a295, a296)
    } else {
        false
    } {
        let a299 = match v5.clone() {
            Adt2::C0(_, _, _, h297, _, _, _, _, _, _, _) => {
                h297
            }
        };
        let a300 = match v7.clone() {
            Adt2::C0(_, _, h298, _, _, _, _, _, _, _, _) => {
                h298
            }
        };
        nat_eq(a299, a300)
    } else {
        false
    } {
        let a302 = match v5.clone() {
            Adt2::C0(_, _, _, _, h301, _, _, _, _, _, _) => {
                h301
            }
        };
        let a303 = 1u64;
        nat_eq(a302, a303)
    } else {
        false
    } {
        f36(v5.clone())?
    } else {
        false
    } {
        let a304 = 0u64;
        let a305 = v12;
        nat_lt(a304, a305)
    } else {
        false
    } {
        let a306 = v12;
        let a307 = 2147483648u64;
        nat_lt(a306, a307)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a309 = match v7.clone() {
                Adt2::C0(_, _, h308, _, _, _, _, _, _, _, _) => {
                    h308
                }
            };
            let a310 = f37(v12)?;
            nat_mul(a309, a310)?
        }, 4u64, f27(v7, 1u64), f27(v5, 1u64), Adt4::C38(f27(v9, 1u64), f27(v11, 1u64), match v0 {
            Adt1::C0(h311, _, _, _, _, _, _, _, _, _, _, _) => {
                h311
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f6(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m312 = v0.clone();
        match m312 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m313 = v1.clone();
        match m313 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m314 = v2.clone();
        match m314 {
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
        let m315 = v0;
        match m315 {
            None => {
                false
            }
            Some(_) => {
                let m316 = v1;
                match m316 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m317 = v2;
                        match m317 {
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
        let a319 = match v7.clone() {
            Adt2::C0(h318, _, _, _, _, _, _, _, _, _, _) => {
                h318
            }
        };
        let a320 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a319, a320) {
            let a322 = match v7.clone() {
                Adt2::C0(_, _, _, _, h321, _, _, _, _, _, _) => {
                    h321
                }
            };
            let a323 = 1u64;
            nat_eq(a322, a323)
        } else {
            false
        } {
            f31(v7.clone(), f23(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f35(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a325 = match v9.clone() {
                Adt2::C0(h324, _, _, _, _, _, _, _, _, _, _) => {
                    h324
                }
            };
            let a326 = 0u64;
            if if if nat_eq(a325, a326) {
                let a328 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h327, _, _, _, _, _, _) => {
                        h327
                    }
                };
                let a329 = 1u64;
                nat_eq(a328, a329)
            } else {
                false
            } {
                f31(v9.clone(), f23(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f35(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a331 = match v11.clone() {
                Adt2::C0(h330, _, _, _, _, _, _, _, _, _, _) => {
                    h330
                }
            };
            let a332 = 0u64;
            if if if nat_eq(a331, a332) {
                let a334 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h333, _, _, _, _, _, _) => {
                        h333
                    }
                };
                let a335 = 1u64;
                nat_eq(a334, a335)
            } else {
                false
            } {
                f31(v11.clone(), f23(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f35(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a337 = match v7.clone() {
                Adt2::C0(_, _, _, h336, _, _, _, _, _, _, _) => {
                    h336
                }
            };
            let a338 = 1u64;
            nat_eq(a337, a338)
        } else {
            false
        } {
            let a340 = match v11.clone() {
                Adt2::C0(_, _, _, h339, _, _, _, _, _, _, _) => {
                    h339
                }
            };
            let a341 = 1u64;
            nat_eq(a340, a341)
        } else {
            false
        } {
            let a344 = match v9.clone() {
                Adt2::C0(_, h342, _, _, _, _, _, _, _, _, _) => {
                    h342
                }
            };
            let a345 = match v7.clone() {
                Adt2::C0(_, h343, _, _, _, _, _, _, _, _, _) => {
                    h343
                }
            };
            nat_eq(a344, a345)
        } else {
            false
        } {
            let a348 = match v9.clone() {
                Adt2::C0(_, _, _, h346, _, _, _, _, _, _, _) => {
                    h346
                }
            };
            let a349 = match v7.clone() {
                Adt2::C0(_, _, h347, _, _, _, _, _, _, _, _) => {
                    h347
                }
            };
            nat_eq(a348, a349)
        } else {
            false
        } {
            let a352 = match v11.clone() {
                Adt2::C0(_, h350, _, _, _, _, _, _, _, _, _) => {
                    h350
                }
            };
            let a353 = match v9.clone() {
                Adt2::C0(_, _, h351, _, _, _, _, _, _, _, _) => {
                    h351
                }
            };
            nat_eq(a352, a353)
        } else {
            false
        } {
            let a356 = match v11.clone() {
                Adt2::C0(_, _, h354, _, _, _, _, _, _, _, _) => {
                    h354
                }
            };
            let a357 = match v7.clone() {
                Adt2::C0(_, _, h355, _, _, _, _, _, _, _, _) => {
                    h355
                }
            };
            nat_eq(a356, a357)
        } else {
            false
        } {
            let a359 = 0u64;
            let a360 = match v9.clone() {
                Adt2::C0(_, _, h358, _, _, _, _, _, _, _, _) => {
                    h358
                }
            };
            if nat_lt(a359, a360) {
                f28(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f36(v5.clone())?
        } else {
            false
        } {
            let m361 = v3.clone();
            match m361 {
                None => {
                    true
                }
                Some(v15) => {
                    let a363 = match v15.clone() {
                        Adt2::C0(h362, _, _, _, _, _, _, _, _, _, _) => {
                            h362
                        }
                    };
                    let a364 = 0u64;
                    if if if if if if nat_eq(a363, a364) {
                        let a366 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h365, _, _, _, _, _, _) => {
                                h365
                            }
                        };
                        let a367 = 1u64;
                        nat_eq(a366, a367)
                    } else {
                        false
                    } {
                        f31(v15.clone(), f23(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f35(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a370 = match v15.clone() {
                            Adt2::C0(_, h368, _, _, _, _, _, _, _, _, _) => {
                                h368
                            }
                        };
                        let a371 = match v9.clone() {
                            Adt2::C0(_, _, h369, _, _, _, _, _, _, _, _) => {
                                h369
                            }
                        };
                        nat_eq(a370, a371)
                    } else {
                        false
                    } {
                        let a374 = match v15.clone() {
                            Adt2::C0(_, _, h372, _, _, _, _, _, _, _, _) => {
                                h372
                            }
                        };
                        let a375 = match v9.clone() {
                            Adt2::C0(_, _, h373, _, _, _, _, _, _, _, _) => {
                                h373
                            }
                        };
                        nat_eq(a374, a375)
                    } else {
                        false
                    } {
                        let a378 = match v15 {
                            Adt2::C0(_, _, _, h376, _, _, _, _, _, _, _) => {
                                h376
                            }
                        };
                        let a379 = match v7.clone() {
                            Adt2::C0(_, _, h377, _, _, _, _, _, _, _, _) => {
                                h377
                            }
                        };
                        nat_eq(a378, a379)
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
            let m380 = v3;
            match m380 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f27(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a382 = f23(v5.clone(), 1u64)?;
            let a383 = f38(match v9.clone() {
                Adt2::C0(_, _, h381, _, _, _, _, _, _, _, _) => {
                    h381
                }
            }, v17.clone())?;
            nat_mul(a382, a383)?
        }, 4u64, f27(v7, 1u64), f27(v5, 1u64), Adt4::C37(f27(v9, 1u64), f27(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f7(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f23(v1.clone(), 1u64)?;
    let a385 = match v1.clone() {
        Adt2::C0(h384, _, _, _, _, _, _, _, _, _, _) => {
            h384
        }
    };
    let a386 = 0u64;
    if if if if if if if if nat_eq(a385, a386) {
        let a388 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h387, _, _, _, _, _) => {
                h387
            }
        };
        let a389 = 4u64;
        nat_eq(a388, a389)
    } else {
        false
    } {
        f39(match v0.clone() {
            Adt1::C0(h390, _, _, _, _, _, _, _, _, _, _, _) => {
                h390
            }
        })
    } else {
        false
    } {
        f39(match v0.clone() {
            Adt1::C0(_, h391, _, _, _, _, _, _, _, _, _, _) => {
                h391
            }
        })
    } else {
        false
    } {
        let a393 = 0u64;
        let a394 = match v0.clone() {
            Adt1::C0(_, _, h392, _, _, _, _, _, _, _, _, _) => {
                h392
            }
        };
        nat_lt(a393, a394)
    } else {
        false
    } {
        let a396 = match v0.clone() {
            Adt1::C0(_, _, h395, _, _, _, _, _, _, _, _, _) => {
                h395
            }
        };
        let a397 = 2139095040u64;
        nat_lt(a396, a397)
    } else {
        false
    } {
        let a398 = v2;
        let a399 = 9223372036854775808u64;
        nat_lt(a398, a399)
    } else {
        false
    } {
        let a401 = f40(v2)?;
        let a402 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h400, _) => {
                h400
            }
        };
        nat_le(a401, a402)
    } else {
        false
    } {
        Ok(Adt10::C4(f41(v2)?, 4u64, f27(v1.clone(), 1u64), f27(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h403, _, _, _, _, _, _, _, _, _, _, _) => {
                h403
            }
        }, match v0.clone() {
            Adt1::C0(_, h404, _, _, _, _, _, _, _, _, _, _) => {
                h404
            }
        }, match v0 {
            Adt1::C0(_, _, h405, _, _, _, _, _, _, _, _, _) => {
                h405
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f8(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m406 = v3;
    match m406 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f42(v0, v2.clone());
            let v7: Adt3 = f42(v0, v5.clone());
            let v8: Adt3 = f42(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h407, _, _, _, _, _, _, _) => {
                    h407
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h408, _, _, _, _, _, _) => {
                    h408
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h409, _, _, _, _, _) => {
                    h409
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h410, _, _, _, _) => {
                    h410
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h411, _, _, _, _, _, _, _) => {
                    h411
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h412, _, _, _, _, _, _) => {
                    h412
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h413, _, _, _, _, _) => {
                    h413
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h414, _, _, _, _, _, _, _, _, _, _, _) => {
                    h414
                }
            };
            let v17: u64 = {
                let a419 = v13;
                let a420 = {
                    let a417 = {
                        let a415 = v9;
                        let a416 = 1u64;
                        nat_sub(a415, a416)
                    };
                    let a418 = v16;
                    nat_mul(a417, a418)?
                };
                nat_add(a419, a420)?
            };
            let v18: u64 = {
                let a425 = v14;
                let a426 = {
                    let a423 = {
                        let a421 = v10;
                        let a422 = 1u64;
                        nat_sub(a421, a422)
                    };
                    let a424 = v16;
                    nat_mul(a423, a424)?
                };
                nat_add(a425, a426)?
            };
            let v19: u64 = {
                let a429 = {
                    let a427 = v9;
                    let a428 = v10;
                    nat_mul(a427, a428)?
                };
                let a430 = v12;
                nat_mul(a429, a430)?
            };
            let v20: u64 = {
                let a433 = {
                    let a431 = v13;
                    let a432 = v14;
                    nat_mul(a431, a432)?
                };
                let a434 = v15;
                nat_mul(a433, a434)?
            };
            let v21: u64 = {
                let a435 = v19;
                let a436 = 32u64;
                if nat_lt(a435, a436) {
                    v19
                } else {
                    32u64
                }
            };
            let a438 = match v2.clone() {
                Adt2::C0(h437, _, _, _, _, _, _, _, _, _, _) => {
                    h437
                }
            };
            let a439 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a438, a439) {
                true
            } else {
                let a441 = match v2.clone() {
                    Adt2::C0(h440, _, _, _, _, _, _, _, _, _, _) => {
                        h440
                    }
                };
                let a442 = 1u64;
                nat_eq(a441, a442)
            } {
                let a444 = match v5.clone() {
                    Adt2::C0(h443, _, _, _, _, _, _, _, _, _, _) => {
                        h443
                    }
                };
                let a445 = 0u64;
                nat_eq(a444, a445)
            } else {
                false
            } {
                let a447 = match v4.clone() {
                    Adt2::C0(h446, _, _, _, _, _, _, _, _, _, _) => {
                        h446
                    }
                };
                let a448 = 0u64;
                nat_eq(a447, a448)
            } else {
                false
            } {
                let a449 = v16;
                let a450 = 2147483648u64;
                nat_lt(a449, a450)
            } else {
                false
            } {
                let a451 = 0u64;
                let a452 = v9;
                nat_lt(a451, a452)
            } else {
                false
            } {
                let a453 = 0u64;
                let a454 = v10;
                nat_lt(a453, a454)
            } else {
                false
            } {
                let a455 = 0u64;
                let a456 = v11;
                nat_lt(a455, a456)
            } else {
                false
            } {
                let a457 = 0u64;
                let a458 = v12;
                nat_lt(a457, a458)
            } else {
                false
            } {
                let a459 = 0u64;
                let a460 = v13;
                nat_lt(a459, a460)
            } else {
                false
            } {
                let a461 = 0u64;
                let a462 = v14;
                nat_lt(a461, a462)
            } else {
                false
            } {
                let a463 = 0u64;
                let a464 = v15;
                nat_lt(a463, a464)
            } else {
                false
            } {
                let a465 = 0u64;
                let a466 = v16;
                nat_lt(a465, a466)
            } else {
                false
            } {
                let a468 = match v6.clone() {
                    Adt3::C0(_, _, _, h467, _, _, _, _) => {
                        h467
                    }
                };
                let a469 = v11;
                nat_eq(a468, a469)
            } else {
                false
            } {
                let a471 = match v8.clone() {
                    Adt3::C0(h470, _, _, _, _, _, _, _) => {
                        h470
                    }
                };
                let a472 = v17;
                nat_eq(a471, a472)
            } else {
                false
            } {
                let a474 = match v8.clone() {
                    Adt3::C0(_, h473, _, _, _, _, _, _) => {
                        h473
                    }
                };
                let a475 = v18;
                nat_eq(a474, a475)
            } else {
                false
            } {
                let a477 = match v8.clone() {
                    Adt3::C0(_, _, h476, _, _, _, _, _) => {
                        h476
                    }
                };
                let a478 = v15;
                nat_eq(a477, a478)
            } else {
                false
            } {
                let a480 = match v8.clone() {
                    Adt3::C0(_, _, _, h479, _, _, _, _) => {
                        h479
                    }
                };
                let a481 = v12;
                nat_eq(a480, a481)
            } else {
                false
            } {
                f22(v4.clone())?
            } else {
                false
            } {
                let a486 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h482, _, _, _, _, _) => {
                        h482
                    }
                };
                let a487 = {
                    let a484 = match v2.clone() {
                        Adt2::C0(h483, _, _, _, _, _, _, _, _, _, _) => {
                            h483
                        }
                    };
                    let a485 = 1u64;
                    if nat_eq(a484, a485) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a486, a487)
            } else {
                false
            } {
                let a489 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h488, _, _, _, _, _) => {
                        h488
                    }
                };
                let a490 = 4u64;
                nat_eq(a489, a490)
            } else {
                false
            } {
                f35(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f35(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a495 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h491, _, _, _, _) => {
                            h491
                        }
                    };
                    let a496 = {
                        let a493 = v13;
                        let a494 = match v2 {
                            Adt2::C0(_, _, _, _, _, h492, _, _, _, _, _) => {
                                h492
                            }
                        };
                        nat_mul(a493, a494)?
                    };
                    nat_eq(a495, a496)
                } else {
                    let a498 = match v1.clone() {
                        Adt1::C0(_, h497, _, _, _, _, _, _, _, _, _, _) => {
                            h497
                        }
                    };
                    let a499 = 0u64;
                    if if if if if if nat_eq(a498, a499) {
                        let a501 = match v1 {
                            Adt1::C0(_, _, h500, _, _, _, _, _, _, _, _, _) => {
                                h500
                            }
                        };
                        let a502 = 1u64;
                        nat_eq(a501, a502)
                    } else {
                        false
                    } {
                        let a504 = match v2 {
                            Adt2::C0(_, _, _, _, h503, _, _, _, _, _, _) => {
                                h503
                            }
                        };
                        let a505 = 1u64;
                        nat_eq(a504, a505)
                    } else {
                        false
                    } {
                        let a507 = match v5.clone() {
                            Adt2::C0(_, _, _, h506, _, _, _, _, _, _, _) => {
                                h506
                            }
                        };
                        let a508 = 1u64;
                        nat_eq(a507, a508)
                    } else {
                        false
                    } {
                        let a510 = match v5 {
                            Adt2::C0(_, _, _, _, h509, _, _, _, _, _, _) => {
                                h509
                            }
                        };
                        let a511 = 1u64;
                        nat_eq(a510, a511)
                    } else {
                        false
                    } {
                        let a513 = match v4.clone() {
                            Adt2::C0(_, _, _, h512, _, _, _, _, _, _, _) => {
                                h512
                            }
                        };
                        let a514 = 1u64;
                        nat_eq(a513, a514)
                    } else {
                        false
                    } {
                        let a516 = match v4 {
                            Adt2::C0(_, _, _, _, h515, _, _, _, _, _, _) => {
                                h515
                            }
                        };
                        let a517 = 1u64;
                        nat_eq(a516, a517)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a518 = v11;
                    let a519 = v20;
                    nat_mul(a518, a519)?
                }, v21, {
                    let a520 = v21;
                    let a521 = v11;
                    nat_mul(a520, a521)?
                }, {
                    let a522 = v21;
                    let a523 = v20;
                    nat_mul(a522, a523)?
                }, {
                    let a528 = {
                        let a524 = v17;
                        let a525 = v18;
                        nat_mul(a524, a525)?
                    };
                    let a529 = {
                        let a526 = v15;
                        let a527 = v12;
                        nat_mul(a526, a527)?
                    };
                    nat_mul(a528, a529)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f9(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m530 = v3;
    match m530 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h531, _, _, _, _, _, _, _, _, _) => {
                    h531
                }
            }, match v1.clone() {
                Adt1::C0(h532, _, _, _, _, _, _, _, _, _, _, _) => {
                    h532
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h533, _, _, _, _, _, _, _, _) => {
                        h533
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h534, _, _, _, _, _, _, _, _, _) => {
                        h534
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h535, _, _, _, _, _) => {
                        h535
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h536, _, _, _, _, _, _, _) => {
                        h536
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h537, _, _, _, _, _, _, _, _) => {
                    h537
                }
            }, match v1.clone() {
                Adt1::C0(_, h538, _, _, _, _, _, _, _, _, _, _) => {
                    h538
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h539, _, _, _, _, _, _, _) => {
                        h539
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h540, _, _, _, _, _, _, _, _) => {
                        h540
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h541, _, _, _, _) => {
                        h541
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h542, _, _, _, _, _, _) => {
                        h542
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h543, _, _, _, _, _, _, _) => {
                        h543
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h544, _, _, _, _, _, _, _, _, _) => {
                        h544
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h545, _, _, _, _, _, _) => {
                        h545
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h546, _, _, _) => {
                        h546
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f43(v6.clone(), match v5.clone() {
                Adt2::C0(_, h547, _, _, _, _, _, _, _, _, _) => {
                    h547
                }
            })?;
            let v10: u64 = f43(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h548, _, _, _, _, _, _, _, _) => {
                    h548
                }
            })?;
            let v11: u64 = if v0 {
                f43(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h549, _, _, _, _, _, _, _) => {
                        h549
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h550, _, _) => {
                        h550
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h551, _, _, _, _, _, _, _) => {
                        h551
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h552, _) => {
                        h552
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h553, _, _, _, _, _, _) => {
                        h553
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h554) => {
                        h554
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h555, _, _, _, _, _, _) => {
                        h555
                    }
                }
            };
            let v15: u64 = {
                let a560 = {
                    let a556 = v9;
                    let a557 = v10;
                    nat_mul(a556, a557)?
                };
                let a561 = {
                    let a558 = v11;
                    let a559 = v13;
                    nat_mul(a558, a559)?
                };
                nat_mul(a560, a561)?
            };
            let v16: u64 = {
                let a569 = {
                    let a564 = match v6.clone() {
                        Adt6::C0(h562, _, _, _) => {
                            h562
                        }
                    };
                    let a565 = match v7.clone() {
                        Adt6::C0(h563, _, _, _) => {
                            h563
                        }
                    };
                    nat_mul(a564, a565)?
                };
                let a570 = {
                    let a567 = match v8.clone() {
                        Adt6::C0(h566, _, _, _) => {
                            h566
                        }
                    };
                    let a568 = v12;
                    nat_mul(a567, a568)?
                };
                nat_mul(a569, a570)?
            };
            let v17: u64 = {
                let a571 = v15;
                let a572 = 32u64;
                if nat_lt(a571, a572) {
                    v15
                } else {
                    32u64
                }
            };
            let a574 = match v2.clone() {
                Adt2::C0(h573, _, _, _, _, _, _, _, _, _, _) => {
                    h573
                }
            };
            let a575 = 0u64;
            if if if if if if if if if if if nat_eq(a574, a575) {
                true
            } else {
                let a577 = match v2.clone() {
                    Adt2::C0(h576, _, _, _, _, _, _, _, _, _, _) => {
                        h576
                    }
                };
                let a578 = 1u64;
                nat_eq(a577, a578)
            } {
                let a580 = match v5.clone() {
                    Adt2::C0(h579, _, _, _, _, _, _, _, _, _, _) => {
                        h579
                    }
                };
                let a581 = 0u64;
                nat_eq(a580, a581)
            } else {
                false
            } {
                let a583 = match v4.clone() {
                    Adt2::C0(h582, _, _, _, _, _, _, _, _, _, _) => {
                        h582
                    }
                };
                let a584 = 0u64;
                nat_eq(a583, a584)
            } else {
                false
            } {
                let a586 = match v4.clone() {
                    Adt2::C0(_, h585, _, _, _, _, _, _, _, _, _) => {
                        h585
                    }
                };
                let a587 = v9;
                nat_eq(a586, a587)
            } else {
                false
            } {
                let a589 = match v4.clone() {
                    Adt2::C0(_, _, h588, _, _, _, _, _, _, _, _) => {
                        h588
                    }
                };
                let a590 = v10;
                nat_eq(a589, a590)
            } else {
                false
            } {
                if v0 {
                    let a591 = 0u64;
                    let a592 = v12;
                    if if if if if if if nat_lt(a591, a592) {
                        let a593 = v12;
                        let a594 = 2147483648u64;
                        nat_lt(a593, a594)
                    } else {
                        false
                    } {
                        let a595 = v14;
                        let a596 = 2147483648u64;
                        nat_lt(a595, a596)
                    } else {
                        false
                    } {
                        let a597 = v13;
                        let a598 = 2147483648u64;
                        nat_lt(a597, a598)
                    } else {
                        false
                    } {
                        let a602 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h599, _, _, _, _, _, _) => {
                                h599
                            }
                        };
                        let a603 = {
                            let a600 = v12;
                            let a601 = v14;
                            nat_mul(a600, a601)?
                        };
                        nat_eq(a602, a603)
                    } else {
                        false
                    } {
                        let a607 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h604, _, _, _, _, _, _) => {
                                h604
                            }
                        };
                        let a608 = {
                            let a605 = v12;
                            let a606 = v13;
                            nat_mul(a605, a606)?
                        };
                        nat_eq(a607, a608)
                    } else {
                        false
                    } {
                        let a610 = match v4.clone() {
                            Adt2::C0(_, _, _, h609, _, _, _, _, _, _, _) => {
                                h609
                            }
                        };
                        let a611 = v11;
                        nat_eq(a610, a611)
                    } else {
                        false
                    } {
                        let a615 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h612, _, _, _, _, _, _) => {
                                h612
                            }
                        };
                        let a616 = {
                            let a613 = v14;
                            let a614 = v13;
                            nat_mul(a613, a614)?
                        };
                        nat_eq(a615, a616)
                    } else {
                        false
                    }
                } else {
                    let a618 = match v2.clone() {
                        Adt2::C0(_, _, _, h617, _, _, _, _, _, _, _) => {
                            h617
                        }
                    };
                    let a619 = v12;
                    if if if nat_eq(a618, a619) {
                        let a621 = match v4.clone() {
                            Adt2::C0(_, _, _, h620, _, _, _, _, _, _, _) => {
                                h620
                            }
                        };
                        let a622 = v14;
                        nat_eq(a621, a622)
                    } else {
                        false
                    } {
                        let a624 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h623, _, _, _, _, _, _) => {
                                h623
                            }
                        };
                        let a625 = v13;
                        nat_eq(a624, a625)
                    } else {
                        false
                    } {
                        let a626 = 0u64;
                        let a627 = v16;
                        nat_lt(a626, a627)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a628 = 0u64;
                let a629 = v14;
                nat_lt(a628, a629)
            } else {
                false
            } {
                let a630 = 0u64;
                let a631 = v15;
                nat_lt(a630, a631)
            } else {
                false
            } {
                f35(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f35(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a632 = v16;
                    let a633 = v14;
                    nat_mul(a632, a633)?
                }, v17, {
                    let a634 = v17;
                    let a635 = v16;
                    nat_mul(a634, a635)?
                }, {
                    let a636 = v17;
                    let a637 = v14;
                    nat_mul(a636, a637)?
                }, 0u64, f27(v2, 1u64), f27(v5, 1u64), f27(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f10(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m638 = v2;
    match m638 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m639 = v3;
            match m639 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a641 = match v1.clone() {
                        Adt2::C0(h640, _, _, _, _, _, _, _, _, _, _) => {
                            h640
                        }
                    };
                    let a642 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a641, a642) {
                        let a644 = match v5.clone() {
                            Adt2::C0(h643, _, _, _, _, _, _, _, _, _, _) => {
                                h643
                            }
                        };
                        let a645 = 0u64;
                        nat_eq(a644, a645)
                    } else {
                        false
                    } {
                        let a647 = match v6.clone() {
                            Adt2::C0(h646, _, _, _, _, _, _, _, _, _, _) => {
                                h646
                            }
                        };
                        let a648 = 0u64;
                        nat_eq(a647, a648)
                    } else {
                        false
                    } {
                        let a650 = match v4.clone() {
                            Adt2::C0(h649, _, _, _, _, _, _, _, _, _, _) => {
                                h649
                            }
                        };
                        let a651 = 0u64;
                        nat_eq(a650, a651)
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
                        let a657 = match v1.clone() {
                            Adt2::C0(_, h652, _, _, _, _, _, _, _, _, _) => {
                                h652
                            }
                        };
                        let a658 = {
                            let a655 = match v5.clone() {
                                Adt2::C0(_, h653, _, _, _, _, _, _, _, _, _) => {
                                    h653
                                }
                            };
                            let a656 = match v5.clone() {
                                Adt2::C0(_, h654, _, _, _, _, _, _, _, _, _) => {
                                    h654
                                }
                            };
                            nat_mul(a655, a656)?
                        };
                        nat_eq(a657, a658)
                    } else {
                        false
                    } {
                        let a664 = match v1.clone() {
                            Adt2::C0(_, _, h659, _, _, _, _, _, _, _, _) => {
                                h659
                            }
                        };
                        let a665 = {
                            let a662 = match v5.clone() {
                                Adt2::C0(_, _, h660, _, _, _, _, _, _, _, _) => {
                                    h660
                                }
                            };
                            let a663 = match v5.clone() {
                                Adt2::C0(_, _, _, h661, _, _, _, _, _, _, _) => {
                                    h661
                                }
                            };
                            nat_mul(a662, a663)?
                        };
                        nat_eq(a664, a665)
                    } else {
                        false
                    } {
                        let a668 = match v1.clone() {
                            Adt2::C0(_, _, _, h666, _, _, _, _, _, _, _) => {
                                h666
                            }
                        };
                        let a669 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h667, _, _, _, _, _, _) => {
                                h667
                            }
                        };
                        nat_eq(a668, a669)
                    } else {
                        false
                    } {
                        let a671 = match v0.clone() {
                            Adt1::C0(h670, _, _, _, _, _, _, _, _, _, _, _) => {
                                h670
                            }
                        };
                        let a672 = 4294967296u64;
                        nat_lt(a671, a672)
                    } else {
                        false
                    } {
                        let a674 = match v0.clone() {
                            Adt1::C0(h673, _, _, _, _, _, _, _, _, _, _, _) => {
                                h673
                            }
                        };
                        let a675 = 0u64;
                        if nat_eq(a674, a675) {
                            if f35(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a678 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h676) => {
                                        h676
                                    }
                                };
                                let a679 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h677) => {
                                        h677
                                    }
                                };
                                nat_eq(a678, a679)
                            }
                        } else {
                            let a682 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h680) => {
                                    h680
                                }
                            };
                            let a683 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h681) => {
                                    h681
                                }
                            };
                            nat_eq(a682, a683)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a685 = match v4.clone() {
                                Adt2::C0(_, h684, _, _, _, _, _, _, _, _, _) => {
                                    h684
                                }
                            };
                            let a686 = 0u64;
                            if if if nat_eq(a685, a686) {
                                true
                            } else {
                                let a688 = match v4.clone() {
                                    Adt2::C0(_, _, h687, _, _, _, _, _, _, _, _) => {
                                        h687
                                    }
                                };
                                let a689 = 0u64;
                                nat_eq(a688, a689)
                            } {
                                true
                            } else {
                                let a691 = match v4.clone() {
                                    Adt2::C0(_, _, _, h690, _, _, _, _, _, _, _) => {
                                        h690
                                    }
                                };
                                let a692 = 0u64;
                                nat_eq(a691, a692)
                            } {
                                true
                            } else {
                                let a694 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h693, _, _, _, _, _, _) => {
                                        h693
                                    }
                                };
                                let a695 = 0u64;
                                nat_eq(a694, a695)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h696, _, _, _, _, _, _, _, _, _) => {
                                h696
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a698 = match v0 {
                                Adt1::C0(h697, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h697
                                }
                            };
                            let a699 = 0u64;
                            if nat_eq(a698, a699) {
                                f23(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f44(v8, match v5.clone() {
                                Adt2::C0(_, _, h700, _, _, _, _, _, _, _, _) => {
                                    h700
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h701, _, _, _, _, _, _, _) => {
                                    h701
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h702, _, _, _, _, _, _) => {
                                    h702
                                }
                            })?
                        };
                        Ok(Adt10::C4(f45(v9, v8, v10)?, 4u64, f27(v1, 1u64), f27(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f46(v2.clone());
    let v6: Adt14 = f46(v4.clone());
    let v8: Adt15 = {
        let m703 = v3;
        match m703 {
            None => {
                f47(v0, match v1.clone() {
                    Adt1::C0(h704, _, _, _, _, _, _, _, _, _, _, _) => {
                        h704
                    }
                }, match v1 {
                    Adt1::C0(_, h705, _, _, _, _, _, _, _, _, _, _) => {
                        h705
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f47(v0, match v1.clone() {
                    Adt1::C0(h706, _, _, _, _, _, _, _, _, _, _, _) => {
                        h706
                    }
                }, match v1 {
                    Adt1::C0(_, h707, _, _, _, _, _, _, _, _, _, _) => {
                        h707
                    }
                }, true, v5, f46(v7), v6)?
            }
        }
    };
    let m708 = v8;
    match m708 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f27(v2, 1u64), f27(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f12(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h709, _, _, _, _, _, _, _, _, _, _) => {
            h709
        }
    };
    let v4: u64 = f48(v3);
    let a711 = match v1.clone() {
        Adt2::C0(h710, _, _, _, _, _, _, _, _, _, _) => {
            h710
        }
    };
    let a712 = 0u64;
    if if if if if if if if nat_eq(a711, a712) {
        true
    } else {
        let a714 = match v1.clone() {
            Adt2::C0(h713, _, _, _, _, _, _, _, _, _, _) => {
                h713
            }
        };
        let a715 = 30u64;
        nat_eq(a714, a715)
    } {
        let a718 = match v1.clone() {
            Adt2::C0(h716, _, _, _, _, _, _, _, _, _, _) => {
                h716
            }
        };
        let a719 = match v2.clone() {
            Adt2::C0(h717, _, _, _, _, _, _, _, _, _, _) => {
                h717
            }
        };
        nat_eq(a718, a719)
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        f49(match v0.clone() {
            Adt1::C0(h720, _, _, _, _, _, _, _, _, _, _, _) => {
                h720
            }
        })
    } else {
        false
    } {
        f50(v1.clone(), v4)
    } else {
        false
    } {
        f50(v2.clone(), v4)
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f51(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f52(match v1.clone() {
            Adt2::C0(_, h721, _, _, _, _, _, _, _, _, _) => {
                h721
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h722, _, _, _, _, _, _, _, _) => {
                h722
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h723, _, _, _, _, _, _, _) => {
                h723
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h724, _, _, _, _, _, _) => {
                h724
            }
        }) {
            0u64
        } else {
            let a730 = match v1.clone() {
                Adt2::C0(_, _, h725, _, _, _, _, _, _, _, _) => {
                    h725
                }
            };
            let a731 = {
                let a728 = match v1.clone() {
                    Adt2::C0(_, _, _, h726, _, _, _, _, _, _, _) => {
                        h726
                    }
                };
                let a729 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h727, _, _, _, _, _, _) => {
                        h727
                    }
                };
                nat_mul(a728, a729)?
            };
            nat_mul(a730, a731)?
        };
        let v6: u64 = {
            let a732 = v5;
            let a733 = 0u64;
            if if nat_eq(a732, a733) {
                true
            } else {
                f53(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a735 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h734, _) => {
                        h734
                    }
                };
                let a736 = v4;
                let a737 = 0u64;
                nat_quot(a735, a736, a737)
            }
        };
        Ok(Adt10::C4(f54(v3, match v1.clone() {
            Adt2::C0(_, h738, _, _, _, _, _, _, _, _, _) => {
                h738
            }
        }, v5, v6)?, v4, f27(v1.clone(), 1u64), f27(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h739, _, _, _, _, _, _, _, _, _, _, _) => {
                h739
            }
        }, v5, f55(match v1 {
            Adt2::C0(_, h740, _, _, _, _, _, _, _, _, _) => {
                h740
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f13(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a742 = match v1.clone() {
        Adt2::C0(h741, _, _, _, _, _, _, _, _, _, _) => {
            h741
        }
    };
    let a743 = 0u64;
    if if if if if nat_eq(a742, a743) {
        let a745 = match v2.clone() {
            Adt2::C0(h744, _, _, _, _, _, _, _, _, _, _) => {
                h744
            }
        };
        let a746 = 0u64;
        nat_eq(a745, a746)
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        f56(match v0.clone() {
            Adt1::C0(h747, _, _, _, _, _, _, _, _, _, _, _) => {
                h747
            }
        })
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f51(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f27(v1, 1u64);
        let v4: u64 = {
            let a753 = match v3.clone() {
                Adt3::C0(_, h748, _, _, _, _, _, _) => {
                    h748
                }
            };
            let a754 = {
                let a751 = match v3.clone() {
                    Adt3::C0(_, _, h749, _, _, _, _, _) => {
                        h749
                    }
                };
                let a752 = match v3.clone() {
                    Adt3::C0(_, _, _, h750, _, _, _, _) => {
                        h750
                    }
                };
                nat_mul(a751, a752)?
            };
            nat_mul(a753, a754)?
        };
        Ok(Adt10::C4(f57(match v3.clone() {
            Adt3::C0(h755, _, _, _, _, _, _, _) => {
                h755
            }
        }, v4)?, 4u64, v3, f27(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h756, _, _, _, _, _, _, _, _, _, _, _) => {
                h756
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f14(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m757 = v1;
    match m757 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m758 = v2;
            match m758 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a760 = match v0.clone() {
                        Adt2::C0(h759, _, _, _, _, _, _, _, _, _, _) => {
                            h759
                        }
                    };
                    let a761 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a760, a761) {
                        let a763 = match v4.clone() {
                            Adt2::C0(h762, _, _, _, _, _, _, _, _, _, _) => {
                                h762
                            }
                        };
                        let a764 = 0u64;
                        nat_eq(a763, a764)
                    } else {
                        false
                    } {
                        let a766 = match v5.clone() {
                            Adt2::C0(h765, _, _, _, _, _, _, _, _, _, _) => {
                                h765
                            }
                        };
                        let a767 = 26u64;
                        nat_eq(a766, a767)
                    } else {
                        false
                    } {
                        let a769 = match v3.clone() {
                            Adt2::C0(h768, _, _, _, _, _, _, _, _, _, _) => {
                                h768
                            }
                        };
                        let a770 = 0u64;
                        nat_eq(a769, a770)
                    } else {
                        false
                    } {
                        f28(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a773 = match v0.clone() {
                            Adt2::C0(_, h771, _, _, _, _, _, _, _, _, _) => {
                                h771
                            }
                        };
                        let a774 = match v4.clone() {
                            Adt2::C0(_, h772, _, _, _, _, _, _, _, _, _) => {
                                h772
                            }
                        };
                        nat_eq(a773, a774)
                    } else {
                        false
                    } {
                        let a777 = match v0.clone() {
                            Adt2::C0(_, _, h775, _, _, _, _, _, _, _, _) => {
                                h775
                            }
                        };
                        let a778 = match v5.clone() {
                            Adt2::C0(_, h776, _, _, _, _, _, _, _, _, _) => {
                                h776
                            }
                        };
                        nat_eq(a777, a778)
                    } else {
                        false
                    } {
                        let a781 = match v0.clone() {
                            Adt2::C0(_, _, _, h779, _, _, _, _, _, _, _) => {
                                h779
                            }
                        };
                        let a782 = match v5.clone() {
                            Adt2::C0(_, _, h780, _, _, _, _, _, _, _, _) => {
                                h780
                            }
                        };
                        nat_eq(a781, a782)
                    } else {
                        false
                    } {
                        let a784 = 0u64;
                        let a785 = match v5.clone() {
                            Adt2::C0(_, _, _, h783, _, _, _, _, _, _, _) => {
                                h783
                            }
                        };
                        nat_lt(a784, a785)
                    } else {
                        false
                    } {
                        let a787 = 0u64;
                        let a788 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h786, _, _, _, _, _, _) => {
                                h786
                            }
                        };
                        nat_lt(a787, a788)
                    } else {
                        false
                    } {
                        let a790 = 4u64;
                        let a791 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h789, _, _, _, _, _) => {
                                h789
                            }
                        };
                        nat_le(a790, a791)
                    } else {
                        false
                    } {
                        let a793 = 4u64;
                        let a794 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h792, _, _, _, _, _) => {
                                h792
                            }
                        };
                        nat_le(a793, a794)
                    } else {
                        false
                    } {
                        let a796 = 4u64;
                        let a797 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h795, _, _, _, _, _) => {
                                h795
                            }
                        };
                        nat_le(a796, a797)
                    } else {
                        false
                    } {
                        let a799 = 4u64;
                        let a800 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h798, _, _, _, _, _) => {
                                h798
                            }
                        };
                        nat_le(a799, a800)
                    } else {
                        false
                    } {
                        f35(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f35(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f35(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f58(match v0.clone() {
                            Adt2::C0(_, h801, _, _, _, _, _, _, _, _, _) => {
                                h801
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h802, _, _, _, _, _, _, _, _) => {
                                h802
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h803, _, _, _, _, _, _, _) => {
                                h803
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h804, _, _, _, _, _, _) => {
                                h804
                            }
                        })?, 4u64, f27(v0, 1u64), f27(v3, 1u64), Adt4::C22(f27(v4, 1u64), f27(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f15(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a805 = v0;
    let a806 = 28u64;
    if if if nat_eq(a805, a806) {
        true
    } else {
        let a807 = v0;
        let a808 = 29u64;
        nat_eq(a807, a808)
    } {
        let a810 = 0u64;
        let a811 = match v1.clone() {
            Adt1::C0(h809, _, _, _, _, _, _, _, _, _, _, _) => {
                h809
            }
        };
        if nat_lt(a810, a811) {
            true
        } else {
            let a813 = 0u64;
            let a814 = match v1.clone() {
                Adt1::C0(_, _, _, h812, _, _, _, _, _, _, _, _) => {
                    h812
                }
            };
            nat_lt(a813, a814)
        }
    } else {
        false
    } {
        let m818 = f59(match v2.clone() {
            Adt2::C0(h815, _, _, _, _, _, _, _, _, _, _) => {
                h815
            }
        }, match v1.clone() {
            Adt1::C0(h816, _, _, _, _, _, _, _, _, _, _, _) => {
                h816
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h817, _, _, _, _, _, _, _, _) => {
                h817
            }
        });
        match m818 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f60(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f60(v0, v1, v2, v3, v4)
    }
}

pub fn f16(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h819, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h819
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h820, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h820
        }
    };
    let v6: u64 = {
        let a821 = v3;
        let a822 = v5;
        let a823 = 0u64;
        nat_rem(a821, a822, a823)
    };
    let a824 = v1;
    let a825 = 0u64;
    if nat_eq(a824, a825) {
        let a827 = v3;
        let a828 = match v0.clone() {
            Adt8::C0(_, _, _, h826, _, _, _, _, _, _, _, _, _, _, _) => {
                h826
            }
        };
        if nat_lt(a827, a828) {
            Ok(Adt9::C8(Some(f61(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h829, _, _, _, _, _, _) => {
                    h829
                }
            }, {
                let a833 = v6;
                let a834 = {
                    let m832 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h831, _, _, _, _, _, _) => {
                            h831
                        }
                    };
                    match m832 {
                        Adt3::C0(h830, _, _, _, _, _, _, _) => {
                            h830
                        }
                    }
                };
                let a835 = 0u64;
                nat_rem(a833, a834, a835)
            }, {
                let a845 = {
                    let a839 = v6;
                    let a840 = {
                        let m838 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h837, _, _, _, _, _, _) => {
                                h837
                            }
                        };
                        match m838 {
                            Adt3::C0(h836, _, _, _, _, _, _, _) => {
                                h836
                            }
                        }
                    };
                    let a841 = 0u64;
                    nat_quot(a839, a840, a841)
                };
                let a846 = {
                    let m844 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h843, _, _, _, _, _, _) => {
                            h843
                        }
                    };
                    match m844 {
                        Adt3::C0(_, h842, _, _, _, _, _, _) => {
                            h842
                        }
                    }
                };
                let a847 = 0u64;
                nat_rem(a845, a846, a847)
            }, {
                let a857 = {
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
                    nat_quot(a851, a852, a853)
                };
                let a858 = {
                    let m856 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h855, _, _, _, _, _, _) => {
                            h855
                        }
                    };
                    match m856 {
                        Adt3::C0(_, h854, _, _, _, _, _, _) => {
                            h854
                        }
                    }
                };
                let a859 = 0u64;
                nat_quot(a857, a858, a859)
            }, {
                let a860 = v3;
                let a861 = v5;
                let a862 = 0u64;
                nat_quot(a860, a861, a862)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a868 = v2;
            let a869 = {
                let a865 = v3;
                let a866 = {
                    let a863 = v1;
                    let a864 = 1u64;
                    if nat_eq(a863, a864) {
                        v4
                    } else {
                        v5
                    }
                };
                let a867 = 0u64;
                nat_quot(a865, a866, a867)
            };
            nat_add(a868, a869)?
        };
        let a871 = v7;
        let a872 = match v0.clone() {
            Adt8::C0(h870, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h870
            }
        };
        if nat_lt(a871, a872) {
            let v8: u64 = {
                let a876 = v7;
                let a877 = {
                    let m875 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h874, _, _, _, _, _) => {
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
                nat_rem(a876, a877, a878)
            };
            let v9: u64 = {
                let a882 = v7;
                let a883 = {
                    let m881 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h880, _, _, _, _, _) => {
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
            let v10: u64 = {
                let a888 = v9;
                let a889 = {
                    let m887 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h886, _, _, _, _, _) => {
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
                nat_rem(a888, a889, a890)
            };
            let v11: u64 = {
                let a894 = v9;
                let a895 = {
                    let m893 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h892, _, _, _, _, _) => {
                            h892
                        }
                    };
                    match m893 {
                        Adt3::C0(_, h891, _, _, _, _, _, _) => {
                            h891
                        }
                    }
                };
                let a896 = 0u64;
                nat_quot(a894, a895, a896)
            };
            let a897 = v1;
            let a898 = 1u64;
            if nat_eq(a897, a898) {
                Ok(Adt9::C8(Some(f61(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h899, _, _, _, _, _) => {
                        h899
                    }
                }, v8, v10, {
                    let a900 = v3;
                    let a901 = v4;
                    let a902 = 0u64;
                    nat_rem(a900, a901, a902)
                }, v11)?)))
            } else {
                let a903 = v1;
                let a904 = 2u64;
                if nat_eq(a903, a904) {
                    Ok(Adt9::C8(Some({
                        let a950 = f61(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h905, _, _, _, _) => {
                                h905
                            }
                        }, {
                            let a917 = {
                                let a909 = v8;
                                let a910 = {
                                    let m908 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h907, _, _, _) => {
                                            h907
                                        }
                                    };
                                    match m908 {
                                        Adt6::C0(_, h906, _, _) => {
                                            h906
                                        }
                                    }
                                };
                                nat_mul(a909, a910)?
                            };
                            let a918 = {
                                let a914 = v6;
                                let a915 = {
                                    let m913 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h912, _, _, _, _, _, _) => {
                                            h912
                                        }
                                    };
                                    match m913 {
                                        Adt3::C0(h911, _, _, _, _, _, _, _) => {
                                            h911
                                        }
                                    }
                                };
                                let a916 = 0u64;
                                nat_rem(a914, a915, a916)
                            };
                            nat_add(a917, a918)?
                        }, {
                            let a936 = {
                                let a922 = v10;
                                let a923 = {
                                    let m921 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h920, _, _, _) => {
                                            h920
                                        }
                                    };
                                    match m921 {
                                        Adt6::C0(_, h919, _, _) => {
                                            h919
                                        }
                                    }
                                };
                                nat_mul(a922, a923)?
                            };
                            let a937 = {
                                let a933 = {
                                    let a927 = v6;
                                    let a928 = {
                                        let m926 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h925, _, _, _, _, _, _) => {
                                                h925
                                            }
                                        };
                                        match m926 {
                                            Adt3::C0(h924, _, _, _, _, _, _, _) => {
                                                h924
                                            }
                                        }
                                    };
                                    let a929 = 0u64;
                                    nat_quot(a927, a928, a929)
                                };
                                let a934 = {
                                    let m932 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h931, _, _, _, _, _, _) => {
                                            h931
                                        }
                                    };
                                    match m932 {
                                        Adt3::C0(_, h930, _, _, _, _, _, _) => {
                                            h930
                                        }
                                    }
                                };
                                let a935 = 0u64;
                                nat_rem(a933, a934, a935)
                            };
                            nat_add(a936, a937)?
                        }, {
                            let a947 = {
                                let a941 = v6;
                                let a942 = {
                                    let m940 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h939, _, _, _, _, _, _) => {
                                            h939
                                        }
                                    };
                                    match m940 {
                                        Adt3::C0(h938, _, _, _, _, _, _, _) => {
                                            h938
                                        }
                                    }
                                };
                                let a943 = 0u64;
                                nat_quot(a941, a942, a943)
                            };
                            let a948 = {
                                let m946 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h945, _, _, _, _, _, _) => {
                                        h945
                                    }
                                };
                                match m946 {
                                    Adt3::C0(_, h944, _, _, _, _, _, _) => {
                                        h944
                                    }
                                }
                            };
                            let a949 = 0u64;
                            nat_quot(a947, a948, a949)
                        }, v11)?;
                        let a951 = 4u64;
                        let a952 = 0u64;
                        nat_quot(a950, a951, a952)
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

pub fn f17(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a953 = v1;
    let a954 = 0u64;
    if nat_eq(a953, a954) {
        let a956 = v3;
        let a957 = match v0.clone() {
            Adt8::C0(_, _, _, h955, _, _, _, _, _, _, _, _, _, _, _) => {
                h955
            }
        };
        if nat_lt(a956, a957) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h958, _, _, _, _, _, _) => {
                    h958
                }
            };
            let v5: u64 = {
                let a960 = v3;
                let a961 = match v0.clone() {
                    Adt8::C0(_, _, h959, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h959
                    }
                };
                let a962 = 0u64;
                nat_quot(a960, a961, a962)
            };
            Ok(Adt9::C8(Some(f61(v4.clone(), f62(v5, v4.clone(), 0u64), f62(v5, v4.clone(), 1u64), f62(v5, v4, 2u64), f63(v0.clone(), v5, {
                let a964 = v3;
                let a965 = match v0 {
                    Adt8::C0(_, _, h963, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h963
                    }
                };
                let a966 = 0u64;
                nat_rem(a964, a965, a966)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h967, _, _, _, _) => {
                h967
            }
        };
        let v7: u64 = {
            let a968 = v1;
            let a969 = 1u64;
            if nat_eq(a968, a969) {
                match v0.clone() {
                    Adt8::C0(_, h970, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h970
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h971, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h971
                    }
                }
            }
        };
        let v8: u64 = {
            let a975 = v2;
            let a976 = {
                let a972 = v3;
                let a973 = v7;
                let a974 = 0u64;
                nat_quot(a972, a973, a974)
            };
            nat_add(a975, a976)?
        };
        let v9: u64 = {
            let a977 = v3;
            let a978 = v7;
            let a979 = 0u64;
            nat_rem(a977, a978, a979)
        };
        let v10: u64 = {
            let a981 = v8;
            let a982 = match v6.clone() {
                Adt3::C0(h980, _, _, _, _, _, _, _) => {
                    h980
                }
            };
            let a983 = 0u64;
            nat_rem(a981, a982, a983)
        };
        let v11: u64 = {
            let a985 = v8;
            let a986 = match v6.clone() {
                Adt3::C0(h984, _, _, _, _, _, _, _) => {
                    h984
                }
            };
            let a987 = 0u64;
            nat_quot(a985, a986, a987)
        };
        let v12: u64 = {
            let a989 = v11;
            let a990 = match v6.clone() {
                Adt3::C0(_, h988, _, _, _, _, _, _) => {
                    h988
                }
            };
            let a991 = 0u64;
            nat_rem(a989, a990, a991)
        };
        let v13: u64 = {
            let a993 = v11;
            let a994 = match v6.clone() {
                Adt3::C0(_, h992, _, _, _, _, _, _) => {
                    h992
                }
            };
            let a995 = 0u64;
            nat_quot(a993, a994, a995)
        };
        let a997 = v8;
        let a998 = match v0.clone() {
            Adt8::C0(h996, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h996
            }
        };
        if nat_lt(a997, a998) {
            let a999 = v1;
            let a1000 = 1u64;
            if nat_eq(a999, a1000) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h1001, _, _, _, _, _, _) => {
                        h1001
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h1002, _, _, _, _, _) => {
                        h1002
                    }
                };
                let m1005 = f64(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h1003, _, _, _) => {
                        h1003
                    }
                }, match v15.clone() {
                    Adt3::C0(h1004, _, _, _, _, _, _, _) => {
                        h1004
                    }
                }, v10, f62(v9, v14.clone(), 0u64))?;
                match m1005 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m1008 = f64(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h1006, _, _) => {
                                h1006
                            }
                        }, match v15 {
                            Adt3::C0(_, h1007, _, _, _, _, _, _) => {
                                h1007
                            }
                        }, v12, f62(v9, v14, 1u64))?;
                        match m1008 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f65(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a1009 = v1;
                let a1010 = 2u64;
                if nat_eq(a1009, a1010) {
                    Ok(Adt9::C8(Some({
                        let a1012 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1011) => {
                                h1011
                            }
                        };
                        let a1013 = 0u64;
                        if nat_eq(a1012, a1013) {
                            f61(v6, v10, v12, v9, v13)?
                        } else {
                            f61(v6.clone(), v10, v12, {
                                let a1015 = v13;
                                let a1016 = match v6.clone() {
                                    Adt3::C0(_, _, h1014, _, _, _, _, _) => {
                                        h1014
                                    }
                                };
                                let a1017 = 0u64;
                                nat_rem(a1015, a1016, a1017)
                            }, {
                                let a1025 = {
                                    let a1023 = {
                                        let a1019 = v13;
                                        let a1020 = match v6 {
                                            Adt3::C0(_, _, h1018, _, _, _, _, _) => {
                                                h1018
                                            }
                                        };
                                        let a1021 = 0u64;
                                        nat_quot(a1019, a1020, a1021)
                                    };
                                    let a1024 = match v0 {
                                        Adt8::C0(_, _, h1022, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h1022
                                        }
                                    };
                                    nat_mul(a1023, a1024)?
                                };
                                let a1026 = v9;
                                nat_add(a1025, a1026)?
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

pub fn f18(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f19(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f19(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f66(v3.clone(), {
        let m1027 = v3.clone();
        match m1027 {
            Adt4::C0 => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f84(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C19(v83, v84) => {
                f85(v83, v0, v1, v2, v84, v4)?
            }
            Adt4::C20(v85, v86) => {
                f86(v85, v0, v1, v2, v86, v4)?
            }
            Adt4::C21 => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C22(v87, v88) => {
                f87(v0, v1, v2, v87, v88, v4)?
            }
            Adt4::C23(v102, v103, v104, v105) => {
                f94(v0, v105, v102, v103, v104, v4)?
            }
            Adt4::C24(v101) => {
                if v5 {
                    f92(Adt16::C0(v101), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f67(v0, v1, v2, v3)?
            }
            Adt4::C26(v97, v98, v99, v100) => {
                let a1029 = v97;
                let a1030 = 64u64;
                if if if nat_eq(a1029, a1030) {
                    true
                } else {
                    let a1031 = v97;
                    let a1032 = 65u64;
                    nat_eq(a1031, a1032)
                } {
                    true
                } else {
                    let a1033 = v97;
                    let a1034 = 66u64;
                    nat_eq(a1033, a1034)
                } {
                    if v5 {
                        f92(Adt16::C1(v97, v98, v99, v100), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a1035 = v97;
                    let a1036 = 0u64;
                    if if nat_eq(a1035, a1036) {
                        true
                    } else {
                        let a1037 = v97;
                        let a1038 = 30u64;
                        nat_eq(a1037, a1038)
                    } {
                        f93(v0, v1, v2, v97, v98, v99, v100, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v96) => {
                f91(v0, v1, v2, v96)?
            }
            Adt4::C28(v92) => {
                f88(v0, v1, v2, v92, v4)?
            }
            Adt4::C29(v93) => {
                f89(v0, v1, v2, v93)?
            }
            Adt4::C30(v94, _) => {
                f90(v0, v1, v2, v94)?
            }
            Adt4::C31(v33, v34) => {
                f68(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f69(v0, match v1 {
                    Adt3::C0(h1028, _, _, _, _, _, _, _) => {
                        h1028
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f70(v0, v1, v36, v2)?
            }
            Adt4::C34(v63, v64, v65, v66) => {
                f83(v0, v1, v63, v2, v64, v65, v66)?
            }
            Adt4::C35(v62) => {
                f82(v0, v62, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f71(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f72(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f73(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
            Adt4::C39(v47) => {
                f74(v0, v1, v47)?
            }
            Adt4::C40(v48, v49, v50) => {
                f75(v0, v1, v2, v48, v49, v50, v4)?
            }
            Adt4::C41 => {
                f81(v0, v1, v2, v4)?
            }
            Adt4::C42(v56, v57) => {
                f79(v0, v1, v2, v56, v57)?
            }
            Adt4::C43(v54, v55) => {
                f78(v0, v1, v2, v54, v55, v4)?
            }
            Adt4::C44(v53) => {
                f77(v0, v2, v53)?
            }
            Adt4::C45(v58, v59, v60, v61) => {
                f80(v0, v1, v2, v58, v59, v60, v61, v4)?
            }
            Adt4::C46(v51, v52) => {
                f76(v0, v1, v51, v52)?
            }
        }
    }))
}

pub fn f20(v0: Adt4) -> R<Adt9> {
    let m1039 = v0;
    match m1039 {
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
        Adt4::C25(v80, _, v82) => {
            Ok(Adt9::C6({
                let a1045 = {
                    let a1041 = 0u64;
                    let a1042 = v80;
                    if nat_lt(a1041, a1042) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a1046 = {
                    let a1043 = 0u64;
                    let a1044 = v82;
                    if nat_lt(a1043, a1044) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a1045, a1046)?
            }, 0u64, 0u64, false, 3u64))
        }
        Adt4::C26(v88, _, _, v91) => {
            Ok(Adt9::C6({
                let a1047 = v88;
                let a1048 = 66u64;
                if nat_eq(a1047, a1048) {
                    3u64
                } else {
                    1u64
                }
            }, 0u64, 0u64, false, v91))
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
        Adt4::C35(v53) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false, v53))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false, 3u64))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m1040 = v32;
                match m1040 {
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
        Adt4::C46(_, _) => {
            Ok(Adt9::C6(31u64, 0u64, 0u64, false, 3u64))
        }
    }
}

pub fn f21(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a1049 = v1;
    let a1050 = 5u64;
    if if if if if nat_lt(a1049, a1050) {
        true
    } else {
        f95(v0, v1, v2, v3, v4)
    } {
        let a1051 = v5;
        let a1052 = 5u64;
        nat_lt(a1051, a1052)
    } else {
        false
    } {
        let a1053 = v7;
        let a1054 = 5u64;
        nat_lt(a1053, a1054)
    } else {
        false
    } {
        let a1055 = v9;
        let a1056 = 2u64;
        if nat_lt(a1055, a1056) {
            true
        } else {
            let a1057 = v9;
            let a1058 = 3u64;
            nat_eq(a1057, a1058)
        }
    } else {
        false
    } {
        Adt9::C7(f96(v0, v1, v2, v3, v4), f97(v5, v6), f97(v7, v8), f98({
            let a1059 = v9;
            let a1060 = 0u64;
            if nat_eq(a1059, a1060) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f22(v0: Adt2) -> R<bool> {
    let a1062 = match v0.clone() {
        Adt2::C0(_, h1061, _, _, _, _, _, _, _, _, _) => {
            h1061
        }
    };
    let a1063 = 1u64;
    if if if if nat_eq(a1062, a1063) {
        true
    } else {
        let a1065 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1064, _, _, _, _, _) => {
                h1064
            }
        };
        let a1066 = 4u64;
        nat_eq(a1065, a1066)
    } {
        let a1068 = match v0.clone() {
            Adt2::C0(_, _, h1067, _, _, _, _, _, _, _, _) => {
                h1067
            }
        };
        let a1069 = 1u64;
        if nat_eq(a1068, a1069) {
            true
        } else {
            let a1074 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1070, _, _, _, _) => {
                    h1070
                }
            };
            let a1075 = {
                let a1072 = match v0.clone() {
                    Adt2::C0(_, h1071, _, _, _, _, _, _, _, _, _) => {
                        h1071
                    }
                };
                let a1073 = 4u64;
                nat_mul(a1072, a1073)?
            };
            nat_eq(a1074, a1075)
        }
    } else {
        false
    } {
        let a1077 = match v0.clone() {
            Adt2::C0(_, _, _, h1076, _, _, _, _, _, _, _) => {
                h1076
            }
        };
        let a1078 = 1u64;
        if nat_eq(a1077, a1078) {
            true
        } else {
            let a1086 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1079, _, _, _) => {
                    h1079
                }
            };
            let a1087 = {
                let a1084 = {
                    let a1082 = match v0.clone() {
                        Adt2::C0(_, h1080, _, _, _, _, _, _, _, _, _) => {
                            h1080
                        }
                    };
                    let a1083 = match v0.clone() {
                        Adt2::C0(_, _, h1081, _, _, _, _, _, _, _, _) => {
                            h1081
                        }
                    };
                    nat_mul(a1082, a1083)?
                };
                let a1085 = 4u64;
                nat_mul(a1084, a1085)?
            };
            nat_eq(a1086, a1087)
        }
    } else {
        false
    } {
        let a1089 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1088, _, _, _, _, _, _) => {
                h1088
            }
        };
        let a1090 = 1u64;
        if nat_eq(a1089, a1090) {
            Ok(true)
        } else {
            let a1101 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1091, _, _) => {
                    h1091
                }
            };
            let a1102 = {
                let a1099 = {
                    let a1097 = {
                        let a1094 = match v0.clone() {
                            Adt2::C0(_, h1092, _, _, _, _, _, _, _, _, _) => {
                                h1092
                            }
                        };
                        let a1095 = match v0.clone() {
                            Adt2::C0(_, _, h1093, _, _, _, _, _, _, _, _) => {
                                h1093
                            }
                        };
                        nat_mul(a1094, a1095)?
                    };
                    let a1098 = match v0 {
                        Adt2::C0(_, _, _, h1096, _, _, _, _, _, _, _) => {
                            h1096
                        }
                    };
                    nat_mul(a1097, a1098)?
                };
                let a1100 = 4u64;
                nat_mul(a1099, a1100)?
            };
            Ok(nat_eq(a1101, a1102))
        }
    } else {
        Ok(false)
    }
}

pub fn f23(v0: Adt2, v1: u64) -> R<u64> {
    let a1114 = {
        let a1108 = {
            let a1104 = match v0.clone() {
                Adt2::C0(_, h1103, _, _, _, _, _, _, _, _, _) => {
                    h1103
                }
            };
            let a1105 = v1;
            let a1106 = 0u64;
            nat_quot(a1104, a1105, a1106)
        };
        let a1109 = match v0.clone() {
            Adt2::C0(_, _, h1107, _, _, _, _, _, _, _, _) => {
                h1107
            }
        };
        nat_mul(a1108, a1109)?
    };
    let a1115 = {
        let a1112 = match v0.clone() {
            Adt2::C0(_, _, _, h1110, _, _, _, _, _, _, _) => {
                h1110
            }
        };
        let a1113 = match v0 {
            Adt2::C0(_, _, _, _, h1111, _, _, _, _, _, _) => {
                h1111
            }
        };
        nat_mul(a1112, a1113)?
    };
    nat_mul(a1114, a1115)
}

pub fn f24(v0: Adt2, v1: Adt2, v2: u64) -> R<bool> {
    let a1117 = match v0.clone() {
        Adt2::C0(h1116, _, _, _, _, _, _, _, _, _, _) => {
            h1116
        }
    };
    let a1118 = 0u64;
    if if if nat_eq(a1117, a1118) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a1122 = {
            let a1119 = 4u64;
            let a1120 = v2;
            nat_mul(a1119, a1120)?
        };
        let a1123 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1121, _) => {
                h1121
            }
        };
        nat_le(a1122, a1123)
    } else {
        false
    } {
        f35(v0, v1)
    } else {
        Ok(false)
    }
}

pub fn f25(v0: Adt2, v1: Adt2, v2: Adt2, v3: u64) -> R<bool> {
    let a1126 = match v0.clone() {
        Adt2::C0(_, h1124, _, _, _, _, _, _, _, _, _) => {
            h1124
        }
    };
    let a1127 = match v1.clone() {
        Adt2::C0(_, h1125, _, _, _, _, _, _, _, _, _) => {
            h1125
        }
    };
    if if if nat_eq(a1126, a1127) {
        let a1130 = match v0.clone() {
            Adt2::C0(_, _, h1128, _, _, _, _, _, _, _, _) => {
                h1128
            }
        };
        let a1131 = match v1.clone() {
            Adt2::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                h1129
            }
        };
        nat_eq(a1130, a1131)
    } else {
        false
    } {
        let a1134 = match v0.clone() {
            Adt2::C0(_, _, _, h1132, _, _, _, _, _, _, _) => {
                h1132
            }
        };
        let a1135 = match v1 {
            Adt2::C0(_, _, _, h1133, _, _, _, _, _, _, _) => {
                h1133
            }
        };
        nat_eq(a1134, a1135)
    } else {
        false
    } {
        f24(v0, v2, v3)
    } else {
        Ok(false)
    }
}

pub fn f26(v0: u64, v1: u64) -> R<u64> {
    let a1142 = v1;
    let a1143 = {
        let a1140 = v1;
        let a1141 = {
            let a1138 = 1u64;
            let a1139 = {
                let a1136 = 5u64;
                let a1137 = v0;
                nat_mul(a1136, a1137)?
            };
            nat_add(a1138, a1139)?
        };
        nat_mul(a1140, a1141)?
    };
    nat_add(a1142, a1143)
}

pub fn f27(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a1145 = match v0.clone() {
            Adt2::C0(_, h1144, _, _, _, _, _, _, _, _, _) => {
                h1144
            }
        };
        let a1146 = v1;
        let a1147 = 0u64;
        nat_quot(a1145, a1146, a1147)
    }, match v0.clone() {
        Adt2::C0(_, _, h1148, _, _, _, _, _, _, _, _) => {
            h1148
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1149, _, _, _, _, _, _, _) => {
            h1149
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1150, _, _, _, _, _, _) => {
            h1150
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1151, _, _, _, _, _) => {
            h1151
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1152, _, _, _, _) => {
            h1152
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1153, _, _, _) => {
            h1153
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1154, _, _) => {
            h1154
        }
    })
}

pub fn f28(v0: Adt2, v1: Adt2) -> bool {
    let a1157 = match v0.clone() {
        Adt2::C0(_, h1155, _, _, _, _, _, _, _, _, _) => {
            h1155
        }
    };
    let a1158 = match v1.clone() {
        Adt2::C0(_, h1156, _, _, _, _, _, _, _, _, _) => {
            h1156
        }
    };
    if if if nat_eq(a1157, a1158) {
        let a1161 = match v0.clone() {
            Adt2::C0(_, _, h1159, _, _, _, _, _, _, _, _) => {
                h1159
            }
        };
        let a1162 = match v1.clone() {
            Adt2::C0(_, _, h1160, _, _, _, _, _, _, _, _) => {
                h1160
            }
        };
        nat_eq(a1161, a1162)
    } else {
        false
    } {
        let a1165 = match v0.clone() {
            Adt2::C0(_, _, _, h1163, _, _, _, _, _, _, _) => {
                h1163
            }
        };
        let a1166 = match v1.clone() {
            Adt2::C0(_, _, _, h1164, _, _, _, _, _, _, _) => {
                h1164
            }
        };
        nat_eq(a1165, a1166)
    } else {
        false
    } {
        let a1169 = match v0 {
            Adt2::C0(_, _, _, _, h1167, _, _, _, _, _, _) => {
                h1167
            }
        };
        let a1170 = match v1 {
            Adt2::C0(_, _, _, _, h1168, _, _, _, _, _, _) => {
                h1168
            }
        };
        nat_eq(a1169, a1170)
    } else {
        false
    }
}

pub fn f29(v0: Adt2, v1: u64, v2: bool) -> R<bool> {
    let a1172 = match v0.clone() {
        Adt2::C0(h1171, _, _, _, _, _, _, _, _, _, _) => {
            h1171
        }
    };
    let a1173 = 0u64;
    if if if nat_eq(a1172, a1173) {
        let a1175 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1174, _, _, _, _, _) => {
                h1174
            }
        };
        let a1176 = 4u64;
        nat_eq(a1175, a1176)
    } else {
        false
    } {
        if v2 {
            f22(v0.clone())?
        } else {
            true
        }
    } else {
        false
    } {
        let a1188 = {
            let a1185 = {
                let a1180 = {
                    let a1177 = v1;
                    let a1178 = 1u64;
                    nat_sub(a1177, a1178)
                };
                let a1181 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1179, _, _, _, _) => {
                        h1179
                    }
                };
                nat_mul(a1180, a1181)?
            };
            let a1186 = {
                let a1183 = 4u64;
                let a1184 = match v0.clone() {
                    Adt2::C0(_, h1182, _, _, _, _, _, _, _, _, _) => {
                        h1182
                    }
                };
                nat_mul(a1183, a1184)?
            };
            nat_add(a1185, a1186)?
        };
        let a1189 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1187, _) => {
                h1187
            }
        };
        Ok(nat_le(a1188, a1189))
    } else {
        Ok(false)
    }
}

pub fn f30(v0: bool, v1: u64) -> R<u64> {
    let a1192 = if v0 {
        2u64
    } else {
        4u64
    };
    let a1193 = {
        let a1190 = if v0 {
            9u64
        } else {
            11u64
        };
        let a1191 = v1;
        nat_mul(a1190, a1191)?
    };
    nat_add(a1192, a1193)
}

pub fn f31(v0: Adt2, v1: u64) -> R<bool> {
    let a1194 = v1;
    let a1195 = 0u64;
    if nat_eq(a1194, a1195) {
        Ok(true)
    } else {
        let a1201 = {
            let a1198 = f99({
                let a1196 = v1;
                let a1197 = 1u64;
                nat_sub(a1196, a1197)
            }, f27(v0.clone(), 1u64))?;
            let a1199 = 4u64;
            nat_add(a1198, a1199)?
        };
        let a1202 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1200, _) => {
                h1200
            }
        };
        Ok(nat_le(a1201, a1202))
    }
}

pub fn f32(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1204 = match v1.clone() {
        Adt2::C0(h1203, _, _, _, _, _, _, _, _, _, _) => {
            h1203
        }
    };
    let a1205 = 0u64;
    if if nat_eq(a1204, a1205) {
        f28(v0.clone(), v1.clone())
    } else {
        false
    } {
        let a1206 = f23(v0.clone(), 1u64)?;
        let a1207 = 0u64;
        if nat_eq(a1206, a1207) {
            Ok(true)
        } else {
            let a1213 = {
                let a1210 = f99({
                    let a1208 = f23(v0.clone(), 1u64)?;
                    let a1209 = 1u64;
                    nat_sub(a1208, a1209)
                }, f27(v0, 1u64))?;
                let a1211 = 4u64;
                nat_add(a1210, a1211)?
            };
            let a1214 = match v1 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1212, _) => {
                    h1212
                }
            };
            Ok(nat_le(a1213, a1214))
        }
    } else {
        Ok(false)
    }
}

pub fn f33(v0: bool) -> u64 {
    if v0 {
        11u64
    } else {
        4u64
    }
}

pub fn f34(v0: bool) -> u64 {
    if v0 {
        15u64
    } else {
        3u64
    }
}

pub fn f35(v0: Adt2, v1: Adt2) -> R<bool> {
    let a1216 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1215, _) => {
            h1215
        }
    };
    let a1217 = 0u64;
    if nat_eq(a1216, a1217) {
        Ok(true)
    } else {
        let a1219 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h1218, _) => {
                h1218
            }
        };
        let a1220 = 0u64;
        if nat_eq(a1219, a1220) {
            Ok(true)
        } else {
            let a1222 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1221) => {
                    h1221
                }
            };
            let a1223 = 0u64;
            if nat_eq(a1222, a1223) {
                Ok(true)
            } else {
                let a1225 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1224) => {
                        h1224
                    }
                };
                let a1226 = 0u64;
                if nat_eq(a1225, a1226) {
                    Ok(true)
                } else {
                    let a1232 = {
                        let a1229 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1227) => {
                                h1227
                            }
                        };
                        let a1230 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h1228, _) => {
                                h1228
                            }
                        };
                        nat_add(a1229, a1230)?
                    };
                    let a1233 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1231) => {
                            h1231
                        }
                    };
                    if nat_le(a1232, a1233) {
                        Ok(true)
                    } else {
                        let a1239 = {
                            let a1236 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1234) => {
                                    h1234
                                }
                            };
                            let a1237 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h1235, _) => {
                                    h1235
                                }
                            };
                            nat_add(a1236, a1237)?
                        };
                        let a1240 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1238) => {
                                h1238
                            }
                        };
                        Ok(nat_le(a1239, a1240))
                    }
                }
            }
        }
    }
}

pub fn f36(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a1246 = match v0.clone() {
            Adt2::C0(_, _, h1241, _, _, _, _, _, _, _, _) => {
                h1241
            }
        };
        let a1247 = {
            let a1244 = match v0.clone() {
                Adt2::C0(_, _, _, h1242, _, _, _, _, _, _, _) => {
                    h1242
                }
            };
            let a1245 = match v0.clone() {
                Adt2::C0(_, _, _, _, h1243, _, _, _, _, _, _) => {
                    h1243
                }
            };
            nat_mul(a1244, a1245)?
        };
        nat_mul(a1246, a1247)?
    };
    let v2: u64 = f23(v0.clone(), 1u64)?;
    let a1249 = match v0.clone() {
        Adt2::C0(h1248, _, _, _, _, _, _, _, _, _, _) => {
            h1248
        }
    };
    let a1250 = 0u64;
    if if if if if if if nat_eq(a1249, a1250) {
        let a1252 = match v0.clone() {
            Adt2::C0(_, h1251, _, _, _, _, _, _, _, _, _) => {
                h1251
            }
        };
        let a1253 = 2147483647u64;
        nat_le(a1252, a1253)
    } else {
        false
    } {
        let a1254 = v1;
        let a1255 = 2147483647u64;
        nat_le(a1254, a1255)
    } else {
        false
    } {
        let a1257 = match v0.clone() {
            Adt2::C0(_, h1256, _, _, _, _, _, _, _, _, _) => {
                h1256
            }
        };
        let a1258 = 1u64;
        if nat_eq(a1257, a1258) {
            true
        } else {
            let a1260 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h1259, _, _, _, _, _) => {
                    h1259
                }
            };
            let a1261 = 4u64;
            nat_eq(a1260, a1261)
        }
    } else {
        false
    } {
        let a1262 = v1;
        let a1263 = 1u64;
        if nat_le(a1262, a1263) {
            true
        } else {
            let a1268 = {
                let a1265 = match v0.clone() {
                    Adt2::C0(_, h1264, _, _, _, _, _, _, _, _, _) => {
                        h1264
                    }
                };
                let a1266 = 4u64;
                nat_mul(a1265, a1266)?
            };
            let a1269 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1267, _, _, _, _) => {
                    h1267
                }
            };
            nat_le(a1268, a1269)
        }
    } else {
        false
    } {
        let a1271 = match v0.clone() {
            Adt2::C0(_, _, _, h1270, _, _, _, _, _, _, _) => {
                h1270
            }
        };
        let a1272 = 1u64;
        if nat_eq(a1271, a1272) {
            true
        } else {
            let a1278 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1273, _, _, _) => {
                    h1273
                }
            };
            let a1279 = {
                let a1276 = match v0.clone() {
                    Adt2::C0(_, _, h1274, _, _, _, _, _, _, _, _) => {
                        h1274
                    }
                };
                let a1277 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h1275, _, _, _, _) => {
                        h1275
                    }
                };
                nat_mul(a1276, a1277)?
            };
            nat_eq(a1278, a1279)
        }
    } else {
        false
    } {
        let a1281 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1280, _, _, _, _, _, _) => {
                h1280
            }
        };
        let a1282 = 1u64;
        if nat_eq(a1281, a1282) {
            true
        } else {
            let a1291 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1283, _, _) => {
                    h1283
                }
            };
            let a1292 = {
                let a1289 = match v0.clone() {
                    Adt2::C0(_, _, _, h1284, _, _, _, _, _, _, _) => {
                        h1284
                    }
                };
                let a1290 = {
                    let a1287 = match v0.clone() {
                        Adt2::C0(_, _, h1285, _, _, _, _, _, _, _, _) => {
                            h1285
                        }
                    };
                    let a1288 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h1286, _, _, _, _) => {
                            h1286
                        }
                    };
                    nat_mul(a1287, a1288)?
                };
                nat_mul(a1289, a1290)?
            };
            nat_eq(a1291, a1292)
        }
    } else {
        false
    } {
        let a1293 = v2;
        let a1294 = 0u64;
        if nat_eq(a1293, a1294) {
            Ok(true)
        } else {
            let a1300 = {
                let a1297 = f99({
                    let a1295 = v2;
                    let a1296 = 1u64;
                    nat_sub(a1295, a1296)
                }, f27(v0.clone(), 1u64))?;
                let a1298 = 4u64;
                nat_add(a1297, a1298)?
            };
            let a1301 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h1299, _) => {
                    h1299
                }
            };
            Ok(nat_le(a1300, a1301))
        }
    } else {
        Ok(false)
    }
}

pub fn f37(v0: u64) -> R<u64> {
    let a1308 = 148u64;
    let a1309 = {
        let a1306 = 36u64;
        let a1307 = {
            let a1304 = {
                let a1302 = 2u64;
                let a1303 = v0;
                nat_mul(a1302, a1303)?
            };
            let a1305 = 1u64;
            nat_sub(a1304, a1305)
        };
        nat_mul(a1306, a1307)?
    };
    nat_add(a1308, a1309)
}

pub fn f38(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1310 = v1;
    match m1310 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1313 = 1u64;
            let a1314 = {
                let a1311 = 2u64;
                let a1312 = v0;
                nat_mul(a1311, a1312)?
            };
            nat_add(a1313, a1314)
        }
    }
}

pub fn f39(v0: u64) -> bool {
    let a1315 = v0;
    let a1316 = 4294967296u64;
    if nat_lt(a1315, a1316) {
        let a1320 = {
            let a1317 = v0;
            let a1318 = 2147483648u64;
            let a1319 = 0u64;
            nat_rem(a1317, a1318, a1319)
        };
        let a1321 = 2139095040u64;
        nat_lt(a1320, a1321)
    } else {
        false
    }
}

pub fn f40(v0: u64) -> R<u64> {
    let a1322 = 4u64;
    let a1323 = v0;
    nat_mul(a1322, a1323)
}

pub fn f41(v0: u64) -> R<u64> {
    let a1324 = v0;
    let a1325 = 0u64;
    if nat_eq(a1324, a1325) {
        Ok(0u64)
    } else {
        let a1328 = 4u64;
        let a1329 = {
            let a1326 = 3u64;
            let a1327 = v0;
            nat_mul(a1326, a1327)?
        };
        nat_add(a1328, a1329)
    }
}

pub fn f42(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f27(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1330, _, _, _, _, _, _, _, _, _) => {
                h1330
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1331, _, _, _, _, _, _, _, _) => {
                h1331
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1332, _, _, _, _, _, _, _) => {
                h1332
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1333, _, _, _, _, _) => {
                h1333
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1334, _, _, _, _) => {
                h1334
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1335, _, _, _) => {
                h1335
            }
        })
    }
}

pub fn f43(v0: Adt6, v1: u64) -> R<u64> {
    let a1337 = match v0.clone() {
        Adt6::C0(h1336, _, _, _) => {
            h1336
        }
    };
    let a1338 = 0u64;
    if nat_eq(a1337, a1338) {
        f100(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1339, _, _) => {
                h1339
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1340, _) => {
                h1340
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1341) => {
                h1341
            }
        }), {
            let a1343 = v1;
            let a1344 = match v0 {
                Adt6::C0(_, _, _, h1342) => {
                    h1342
                }
            };
            nat_add(a1343, a1344)?
        })
    } else {
        f100(v0, v1)
    }
}

pub fn f44(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1349 = {
        let a1345 = v0;
        let a1346 = v1;
        nat_mul(a1345, a1346)?
    };
    let a1350 = {
        let a1347 = v2;
        let a1348 = f101(v3);
        nat_mul(a1347, a1348)?
    };
    nat_mul(a1349, a1350)
}

pub fn f45(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1353 = v0;
    let a1354 = {
        let a1351 = f102(v1)?;
        let a1352 = v2;
        nat_mul(a1351, a1352)?
    };
    nat_add(a1353, a1354)
}

pub fn f46(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1355, _, _, _, _, _, _, _, _, _, _) => {
            h1355
        }
    }, match v0.clone() {
        Adt2::C0(_, h1356, _, _, _, _, _, _, _, _, _) => {
            h1356
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1357, _, _, _, _, _, _, _, _) => {
            h1357
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1358, _, _, _, _, _, _, _) => {
            h1358
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1359, _, _, _, _, _, _) => {
            h1359
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1360, _, _, _, _, _) => {
            h1360
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1361, _, _, _, _) => {
            h1361
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1362, _, _, _) => {
            h1362
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1363, _, _) => {
            h1363
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1364, _) => {
            h1364
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1365) => {
            h1365
        }
    })
}

pub fn f47(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1366 = v0;
    let a1367 = 23u64;
    if nat_eq(a1366, a1367) {
        f103(v1, v4, v6)
    } else {
        let a1368 = v0;
        let a1369 = 26u64;
        if nat_eq(a1368, a1369) {
            f104(v1, v2, v4, v6)
        } else {
            let a1370 = v0;
            let a1371 = 25u64;
            if nat_eq(a1370, a1371) {
                f105(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f48(v0: u64) -> u64 {
    let a1372 = v0;
    let a1373 = 30u64;
    if nat_eq(a1372, a1373) {
        2u64
    } else {
        4u64
    }
}

pub fn f49(v0: u64) -> bool {
    let a1374 = v0;
    let a1375 = 2139095040u64;
    if nat_le(a1374, a1375) {
        true
    } else {
        let a1376 = v0;
        let a1377 = 2147483648u64;
        nat_eq(a1376, a1377)
    }
}

pub fn f50(v0: Adt2, v1: u64) -> bool {
    let a1379 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1378, _, _, _, _, _) => {
            h1378
        }
    };
    let a1380 = v1;
    if if nat_eq(a1379, a1380) {
        let a1385 = {
            let a1382 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1381) => {
                    h1381
                }
            };
            let a1383 = v1;
            let a1384 = 0u64;
            nat_rem(a1382, a1383, a1384)
        };
        let a1386 = 0u64;
        nat_eq(a1385, a1386)
    } else {
        false
    } {
        f106(match v0.clone() {
            Adt2::C0(_, _, h1387, _, _, _, _, _, _, _, _) => {
                h1387
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1388, _, _, _, _, _, _, _) => {
                h1388
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1389, _, _, _, _, _, _) => {
                h1389
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1390, _, _, _, _) => {
                h1390
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1391, _, _, _) => {
                h1391
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1392, _, _) => {
                h1392
            }
        }, v1)
    } else {
        false
    }
}

pub fn f51(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f28(v0.clone(), v1.clone()) {
        let a1395 = match v0.clone() {
            Adt2::C0(h1393, _, _, _, _, _, _, _, _, _, _) => {
                h1393
            }
        };
        let a1396 = match v1.clone() {
            Adt2::C0(h1394, _, _, _, _, _, _, _, _, _, _) => {
                h1394
            }
        };
        nat_eq(a1395, a1396)
    } else {
        false
    } {
        let a1399 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1397) => {
                h1397
            }
        };
        let a1400 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1398) => {
                h1398
            }
        };
        nat_eq(a1399, a1400)
    } else {
        false
    } {
        let a1403 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1401, _, _, _, _, _) => {
                h1401
            }
        };
        let a1404 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1402, _, _, _, _, _) => {
                h1402
            }
        };
        nat_eq(a1403, a1404)
    } else {
        false
    } {
        let a1407 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1405, _, _, _, _) => {
                h1405
            }
        };
        let a1408 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1406, _, _, _, _) => {
                h1406
            }
        };
        nat_eq(a1407, a1408)
    } else {
        false
    } {
        let a1411 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1409, _, _, _) => {
                h1409
            }
        };
        let a1412 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1410, _, _, _) => {
                h1410
            }
        };
        nat_eq(a1411, a1412)
    } else {
        false
    } {
        let a1415 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1413, _, _) => {
                h1413
            }
        };
        let a1416 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1414, _, _) => {
                h1414
            }
        };
        nat_eq(a1415, a1416)
    } else {
        false
    }
}

pub fn f52(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1417 = v0;
    let a1418 = 0u64;
    if if if nat_eq(a1417, a1418) {
        true
    } else {
        let a1419 = v1;
        let a1420 = 0u64;
        nat_eq(a1419, a1420)
    } {
        true
    } else {
        let a1421 = v2;
        let a1422 = 0u64;
        nat_eq(a1421, a1422)
    } {
        true
    } else {
        let a1423 = v3;
        let a1424 = 0u64;
        nat_eq(a1423, a1424)
    }
}

pub fn f53(v0: Adt2, v1: u64) -> R<bool> {
    f107({
        let a1426 = match v0.clone() {
            Adt2::C0(_, h1425, _, _, _, _, _, _, _, _, _) => {
                h1425
            }
        };
        let a1427 = v1;
        nat_mul(a1426, a1427)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1428, _, _, _, _, _, _, _, _) => {
            h1428
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1429, _, _, _, _, _, _, _) => {
            h1429
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1430, _, _, _, _, _, _) => {
            h1430
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1431, _, _, _, _) => {
            h1431
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1432, _, _, _) => {
            h1432
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1433, _, _) => {
            h1433
        }
    })
}

pub fn f54(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1434 = v1;
    let a1435 = 0u64;
    if if nat_eq(a1434, a1435) {
        true
    } else {
        let a1436 = v2;
        let a1437 = 0u64;
        nat_eq(a1436, a1437)
    } {
        Ok(0u64)
    } else {
        let a1444 = {
            let a1438 = f108(v1, v2, v3)?;
            let a1439 = f109(v0, v1, v2)?;
            nat_add(a1438, a1439)?
        };
        let a1445 = {
            let a1442 = f110(v0);
            let a1443 = {
                let a1440 = v1;
                let a1441 = v2;
                nat_mul(a1440, a1441)?
            };
            nat_mul(a1442, a1443)?
        };
        nat_add(a1444, a1445)
    }
}

pub fn f55(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1446 = v0;
    let a1447 = 0u64;
    if if nat_eq(a1446, a1447) {
        true
    } else {
        let a1448 = v1;
        let a1449 = 0u64;
        nat_eq(a1448, a1449)
    } {
        Ok(3u64)
    } else {
        let a1452 = {
            let a1450 = 3u64;
            let a1451 = v1;
            nat_add(a1450, a1451)?
        };
        let a1453 = v2;
        nat_add(a1452, a1453)
    }
}

pub fn f56(v0: u64) -> bool {
    let a1454 = v0;
    let a1455 = 2139095040u64;
    if nat_le(a1454, a1455) {
        true
    } else {
        let a1456 = v0;
        let a1457 = 2147483648u64;
        nat_eq(a1456, a1457)
    }
}

pub fn f57(v0: u64, v1: u64) -> R<u64> {
    let a1458 = v0;
    let a1459 = 0u64;
    if nat_eq(a1458, a1459) {
        Ok(0u64)
    } else {
        let a1460 = v1;
        let a1461 = f111(v0)?;
        nat_mul(a1460, a1461)
    }
}

pub fn f58(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1462 = v0;
    let a1463 = 0u64;
    if if if if nat_eq(a1462, a1463) {
        true
    } else {
        let a1464 = v1;
        let a1465 = 0u64;
        nat_eq(a1464, a1465)
    } {
        true
    } else {
        let a1466 = v2;
        let a1467 = 0u64;
        nat_eq(a1466, a1467)
    } {
        true
    } else {
        let a1468 = v3;
        let a1469 = 0u64;
        nat_eq(a1468, a1469)
    } {
        Ok(0u64)
    } else {
        let a1482 = {
            let a1472 = 2u64;
            let a1473 = {
                let a1470 = v1;
                let a1471 = v2;
                nat_mul(a1470, a1471)?
            };
            nat_mul(a1472, a1473)?
        };
        let a1483 = {
            let a1480 = {
                let a1476 = {
                    let a1474 = v1;
                    let a1475 = v2;
                    nat_mul(a1474, a1475)?
                };
                let a1477 = v3;
                nat_mul(a1476, a1477)?
            };
            let a1481 = {
                let a1478 = 1u64;
                let a1479 = v0;
                nat_add(a1478, a1479)?
            };
            nat_mul(a1480, a1481)?
        };
        nat_add(a1482, a1483)
    }
}

pub fn f59(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1484 = v1;
        let a1485 = 0u64;
        if if if nat_eq(a1484, a1485) {
            true
        } else {
            let a1486 = v1;
            let a1487 = 10u64;
            nat_eq(a1486, a1487)
        } {
            true
        } else {
            let a1488 = v1;
            let a1489 = 15u64;
            nat_eq(a1488, a1489)
        } {
            true
        } else {
            let a1490 = v1;
            let a1491 = 20u64;
            nat_eq(a1490, a1491)
        }
    };
    let v4: bool = {
        let a1492 = v2;
        let a1493 = 0u64;
        if if if if if nat_eq(a1492, a1493) {
            true
        } else {
            let a1494 = v2;
            let a1495 = 10u64;
            nat_eq(a1494, a1495)
        } {
            true
        } else {
            let a1496 = v2;
            let a1497 = 15u64;
            nat_eq(a1496, a1497)
        } {
            true
        } else {
            let a1498 = v2;
            let a1499 = 20u64;
            nat_eq(a1498, a1499)
        } {
            true
        } else {
            let a1500 = v2;
            let a1501 = 30u64;
            nat_eq(a1500, a1501)
        } {
            true
        } else {
            let a1502 = v2;
            let a1503 = 40u64;
            nat_eq(a1502, a1503)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1504 = v0;
            let a1505 = 30u64;
            if if nat_eq(a1504, a1505) {
                let a1506 = v2;
                let a1507 = 10u64;
                nat_eq(a1506, a1507)
            } else {
                false
            } {
                true
            } else {
                if f112(v0) {
                    let a1508 = v2;
                    let a1509 = 10u64;
                    if if nat_eq(a1508, a1509) {
                        true
                    } else {
                        let a1510 = v2;
                        let a1511 = 15u64;
                        nat_eq(a1510, a1511)
                    } {
                        true
                    } else {
                        let a1512 = v2;
                        let a1513 = 20u64;
                        nat_eq(a1512, a1513)
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

pub fn f60(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1514, _, _, _, _, _, _, _, _, _, _, _) => {
            h1514
        }
    };
    Ok(Adt9::C0({
        let a1515 = v0;
        let a1516 = 65u64;
        if nat_eq(a1515, a1516) {
            let m1517 = v3;
            match m1517 {
                None => {
                    f113(v1, v2, v4)?
                }
                Some(_) => {
                    Adt10::C1
                }
            }
        } else {
            let a1518 = v0;
            let a1519 = 59u64;
            if nat_eq(a1518, a1519) {
                let m1520 = v3;
                match m1520 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f114(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1521 = v0;
                let a1522 = 50u64;
                if nat_eq(a1521, a1522) {
                    let m1523 = v3;
                    match m1523 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f115(v1, v2, v8, v4)?
                        }
                    }
                } else {
                    let a1524 = v0;
                    let a1525 = 82u64;
                    if nat_eq(a1524, a1525) {
                        let m1526 = v3;
                        match m1526 {
                            None => {
                                Adt10::C1
                            }
                            Some(v9) => {
                                f116(v2, v9, v4)?
                            }
                        }
                    } else {
                        let a1527 = v0;
                        let a1528 = 86u64;
                        if nat_eq(a1527, a1528) {
                            let m1529 = v3;
                            match m1529 {
                                None => {
                                    Adt10::C1
                                }
                                Some(v10) => {
                                    f117(v1, v2, v10, v4)?
                                }
                            }
                        } else {
                            let a1530 = v0;
                            let a1531 = 55u64;
                            if nat_eq(a1530, a1531) {
                                let m1532 = v3;
                                match m1532 {
                                    None => {
                                        Adt10::C1
                                    }
                                    Some(v11) => {
                                        f118(v1, v2, v11, v4)?
                                    }
                                }
                            } else {
                                let a1533 = v0;
                                let a1534 = 22u64;
                                if nat_eq(a1533, a1534) {
                                    let m1535 = v3;
                                    match m1535 {
                                        None => {
                                            Adt10::C1
                                        }
                                        Some(v12) => {
                                            f119(v2, v12, v4)?
                                        }
                                    }
                                } else {
                                    let a1536 = v0;
                                    let a1537 = 44u64;
                                    if nat_eq(a1536, a1537) {
                                        f120(v1, v2, v3, v4)?
                                    } else {
                                        let a1538 = v0;
                                        let a1539 = 52u64;
                                        if nat_eq(a1538, a1539) {
                                            f121(v1, v2, v4)?
                                        } else {
                                            let a1540 = v0;
                                            let a1541 = 73u64;
                                            if nat_eq(a1540, a1541) {
                                                f122(v2, v3, v4)?
                                            } else {
                                                let a1542 = v0;
                                                let a1543 = 57u64;
                                                if if nat_eq(a1542, a1543) {
                                                    true
                                                } else {
                                                    let a1544 = v0;
                                                    let a1545 = 58u64;
                                                    nat_eq(a1544, a1545)
                                                } {
                                                    let m1546 = v3;
                                                    match m1546 {
                                                        None => {
                                                            f123(false, {
                                                                let a1547 = v0;
                                                                let a1548 = 58u64;
                                                                nat_eq(a1547, a1548)
                                                            }, v1, v2, v4)?
                                                        }
                                                        Some(_) => {
                                                            Adt10::C1
                                                        }
                                                    }
                                                } else {
                                                    let a1549 = v0;
                                                    let a1550 = 49u64;
                                                    if nat_eq(a1549, a1550) {
                                                        f124(false, v1, v2, v3, v4)?
                                                    } else {
                                                        let a1551 = v0;
                                                        let a1552 = 51u64;
                                                        if nat_eq(a1551, a1552) {
                                                            f125(v1, v2, v3, v4)?
                                                        } else {
                                                            let a1553 = v0;
                                                            let a1554 = 39u64;
                                                            if nat_eq(a1553, a1554) {
                                                                f126(v2, v3, v4)?
                                                            } else {
                                                                let a1555 = v0;
                                                                let a1556 = 77u64;
                                                                if nat_eq(a1555, a1556) {
                                                                    f127(v2, v4)?
                                                                } else {
                                                                    let a1557 = v0;
                                                                    let a1558 = 40u64;
                                                                    if nat_eq(a1557, a1558) {
                                                                        f128(v2, v3, v4)?
                                                                    } else {
                                                                        if f129(v0) {
                                                                            let a1559 = v0;
                                                                            let a1560 = 33u64;
                                                                            if if if nat_eq(a1559, a1560) {
                                                                                let a1562 = match v2.clone() {
                                                                                    Adt2::C0(h1561, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1561
                                                                                    }
                                                                                };
                                                                                let a1563 = 0u64;
                                                                                nat_eq(a1562, a1563)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let a1565 = match v4.clone() {
                                                                                    Adt2::C0(h1564, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        h1564
                                                                                    }
                                                                                };
                                                                                let a1566 = 26u64;
                                                                                nat_eq(a1565, a1566)
                                                                            } else {
                                                                                false
                                                                            } {
                                                                                let m1567 = v3;
                                                                                match m1567 {
                                                                                    None => {
                                                                                        Adt10::C1
                                                                                    }
                                                                                    Some(v14) => {
                                                                                        f130(v2, v14, v4)?
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                let a1568 = v0;
                                                                                let a1569 = 33u64;
                                                                                if if if nat_eq(a1568, a1569) {
                                                                                    let a1571 = match v4.clone() {
                                                                                        Adt2::C0(h1570, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1570
                                                                                        }
                                                                                    };
                                                                                    let a1572 = 0u64;
                                                                                    nat_eq(a1571, a1572)
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f131(match v2.clone() {
                                                                                        Adt2::C0(h1573, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            h1573
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    false
                                                                                } {
                                                                                    f132(v2, v3, v4)?
                                                                                } else {
                                                                                    f133(v0, v2, {
                                                                                        let m1574 = v3;
                                                                                        match m1574 {
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
                                                                            let a1575 = v0;
                                                                            let a1576 = 19u64;
                                                                            if nat_eq(a1575, a1576) {
                                                                                f134(v2, v4)?
                                                                            } else {
                                                                                let a1577 = v0;
                                                                                let a1578 = 21u64;
                                                                                if nat_eq(a1577, a1578) {
                                                                                    f135(v5, v2, v3, v4)?
                                                                                } else {
                                                                                    let a1579 = v0;
                                                                                    let a1580 = 61u64;
                                                                                    if if if nat_eq(a1579, a1580) {
                                                                                        true
                                                                                    } else {
                                                                                        let a1581 = v0;
                                                                                        let a1582 = 62u64;
                                                                                        nat_eq(a1581, a1582)
                                                                                    } {
                                                                                        true
                                                                                    } else {
                                                                                        let a1583 = v0;
                                                                                        let a1584 = 63u64;
                                                                                        nat_eq(a1583, a1584)
                                                                                    } {
                                                                                        f136(v0, v1, v2, v4)?
                                                                                    } else {
                                                                                        let a1585 = v0;
                                                                                        let a1586 = 4u64;
                                                                                        if if nat_eq(a1585, a1586) {
                                                                                            true
                                                                                        } else {
                                                                                            let a1587 = v0;
                                                                                            let a1588 = 32u64;
                                                                                            nat_eq(a1587, a1588)
                                                                                        } {
                                                                                            f137(v0, v1, v2, v3, v4)?
                                                                                        } else {
                                                                                            let a1589 = v0;
                                                                                            let a1590 = 20u64;
                                                                                            if nat_eq(a1589, a1590) {
                                                                                                f138(v2, v4)?
                                                                                            } else {
                                                                                                let a1591 = v0;
                                                                                                let a1592 = 42u64;
                                                                                                if nat_eq(a1591, a1592) {
                                                                                                    f139(4u64, v2, v4)?
                                                                                                } else {
                                                                                                    let a1593 = v0;
                                                                                                    let a1594 = 69u64;
                                                                                                    if nat_eq(a1593, a1594) {
                                                                                                        let a1595 = v5;
                                                                                                        let a1596 = 4u64;
                                                                                                        if nat_lt(a1595, a1596) {
                                                                                                            f139(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            Adt10::C1
                                                                                                        }
                                                                                                    } else {
                                                                                                        let a1597 = v0;
                                                                                                        let a1598 = 70u64;
                                                                                                        if nat_eq(a1597, a1598) {
                                                                                                            f140(v5, v2, v4)?
                                                                                                        } else {
                                                                                                            let a1599 = v0;
                                                                                                            let a1600 = 75u64;
                                                                                                            if if nat_eq(a1599, a1600) {
                                                                                                                true
                                                                                                            } else {
                                                                                                                let a1601 = v0;
                                                                                                                let a1602 = 76u64;
                                                                                                                nat_eq(a1601, a1602)
                                                                                                            } {
                                                                                                                f141({
                                                                                                                    let a1603 = v0;
                                                                                                                    let a1604 = 76u64;
                                                                                                                    nat_eq(a1603, a1604)
                                                                                                                }, v1, v2, v4)?
                                                                                                            } else {
                                                                                                                let a1605 = v0;
                                                                                                                let a1606 = 31u64;
                                                                                                                if if if nat_eq(a1605, a1606) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1607 = v0;
                                                                                                                    let a1608 = 47u64;
                                                                                                                    nat_eq(a1607, a1608)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1609 = v0;
                                                                                                                    let a1610 = 68u64;
                                                                                                                    nat_eq(a1609, a1610)
                                                                                                                } {
                                                                                                                    f142(v0, v1, v2, v4)?
                                                                                                                } else {
                                                                                                                    let a1611 = v0;
                                                                                                                    let a1612 = 15u64;
                                                                                                                    if nat_eq(a1611, a1612) {
                                                                                                                        f143(v2, v4)?
                                                                                                                    } else {
                                                                                                                        let a1613 = v0;
                                                                                                                        let a1614 = 17u64;
                                                                                                                        if nat_eq(a1613, a1614) {
                                                                                                                            f144(v2, v4)?
                                                                                                                        } else {
                                                                                                                            let a1615 = v0;
                                                                                                                            let a1616 = 18u64;
                                                                                                                            if nat_eq(a1615, a1616) {
                                                                                                                                f145(v2, v3, v4)?
                                                                                                                            } else {
                                                                                                                                let a1617 = v0;
                                                                                                                                let a1618 = 13u64;
                                                                                                                                if if if nat_eq(a1617, a1618) {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1619 = v0;
                                                                                                                                    let a1620 = 14u64;
                                                                                                                                    nat_eq(a1619, a1620)
                                                                                                                                } {
                                                                                                                                    true
                                                                                                                                } else {
                                                                                                                                    let a1621 = v0;
                                                                                                                                    let a1622 = 16u64;
                                                                                                                                    nat_eq(a1621, a1622)
                                                                                                                                } {
                                                                                                                                    f146(v0, v2, v4)?
                                                                                                                                } else {
                                                                                                                                    let m1623 = f147(v0, v5)?;
                                                                                                                                    match m1623 {
                                                                                                                                        None => {
                                                                                                                                            f148(v0, v5, v2, v3, v4)?
                                                                                                                                        }
                                                                                                                                        Some(v16) => {
                                                                                                                                            f149(v16, v1, v2, v4)?
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f61(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f150(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1624, _, _, _) => {
            h1624
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1625, _, _) => {
            h1625
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1626, _) => {
            h1626
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1627) => {
            h1627
        }
    })
}

pub fn f62(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1629 = v0;
        let a1630 = match v1.clone() {
            Adt3::C0(h1628, _, _, _, _, _, _, _) => {
                h1628
            }
        };
        let a1631 = 0u64;
        nat_quot(a1629, a1630, a1631)
    };
    let v4: u64 = {
        let a1633 = v3;
        let a1634 = match v1.clone() {
            Adt3::C0(_, h1632, _, _, _, _, _, _) => {
                h1632
            }
        };
        let a1635 = 0u64;
        nat_quot(a1633, a1634, a1635)
    };
    let a1636 = v2;
    let a1637 = 0u64;
    if nat_eq(a1636, a1637) {
        let a1639 = v0;
        let a1640 = match v1 {
            Adt3::C0(h1638, _, _, _, _, _, _, _) => {
                h1638
            }
        };
        let a1641 = 0u64;
        nat_rem(a1639, a1640, a1641)
    } else {
        let a1642 = v2;
        let a1643 = 1u64;
        if nat_eq(a1642, a1643) {
            let a1645 = v3;
            let a1646 = match v1 {
                Adt3::C0(_, h1644, _, _, _, _, _, _) => {
                    h1644
                }
            };
            let a1647 = 0u64;
            nat_rem(a1645, a1646, a1647)
        } else {
            let a1648 = v2;
            let a1649 = 2u64;
            if nat_eq(a1648, a1649) {
                let a1651 = v4;
                let a1652 = match v1 {
                    Adt3::C0(_, _, h1650, _, _, _, _, _) => {
                        h1650
                    }
                };
                let a1653 = 0u64;
                nat_rem(a1651, a1652, a1653)
            } else {
                let a1655 = v4;
                let a1656 = match v1 {
                    Adt3::C0(_, _, h1654, _, _, _, _, _) => {
                        h1654
                    }
                };
                let a1657 = 0u64;
                nat_quot(a1655, a1656, a1657)
            }
        }
    }
}

pub fn f63(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1659 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1658) => {
            h1658
        }
    };
    let a1660 = 0u64;
    if nat_eq(a1659, a1660) {
        Ok(v2)
    } else {
        let a1665 = {
            let a1662 = v2;
            let a1663 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1661) => {
                    h1661
                }
            };
            nat_mul(a1662, a1663)?
        };
        let a1666 = f62(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1664, _, _, _, _, _, _) => {
                h1664
            }
        }, 3u64);
        nat_add(a1665, a1666)
    }
}

pub fn f64(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1676 = {
            let a1673 = {
                let a1668 = v2;
                let a1669 = match v0.clone() {
                    Adt6::C0(_, h1667, _, _) => {
                        h1667
                    }
                };
                nat_mul(a1668, a1669)?
            };
            let a1674 = {
                let a1671 = v3;
                let a1672 = match v0.clone() {
                    Adt6::C0(_, _, _, h1670) => {
                        h1670
                    }
                };
                nat_mul(a1671, a1672)?
            };
            nat_add(a1673, a1674)?
        };
        let a1677 = f151(match v0.clone() {
            Adt6::C0(_, _, h1675, _) => {
                h1675
            }
        });
        nat_add(a1676, a1677)?
    };
    let v5: u64 = f152(match v0 {
        Adt6::C0(_, _, h1678, _) => {
            h1678
        }
    });
    let a1679 = v5;
    let a1680 = v4;
    if if nat_le(a1679, a1680) {
        let a1683 = {
            let a1681 = v4;
            let a1682 = v5;
            nat_sub(a1681, a1682)
        };
        let a1684 = v1;
        nat_lt(a1683, a1684)
    } else {
        false
    } {
        Ok(Some({
            let a1685 = v4;
            let a1686 = v5;
            nat_sub(a1685, a1686)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f65(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1687, _, _, _, _, _) => {
            h1687
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1688, _, _, _, _, _, _) => {
            h1688
        }
    };
    let v7: u64 = f62(v1, v6.clone(), 2u64);
    let a1690 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1689) => {
            h1689
        }
    };
    let a1691 = 0u64;
    if nat_eq(a1690, a1691) {
        Ok(Adt9::C8(Some(f61(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1694 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1693, _, _, _, _) => {
                    h1693
                }
            };
            match m1694 {
                Adt3::C0(_, _, h1692, _, _, _, _, _) => {
                    h1692
                }
            }
        };
        let m1700 = f64(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1695, _) => {
                h1695
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1696, _, _, _, _, _) => {
                h1696
            }
        }, {
            let a1697 = v2;
            let a1698 = v8;
            let a1699 = 0u64;
            nat_rem(a1697, a1698, a1699)
        }, v7)?;
        match m1700 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1707 = {
                        let a1705 = {
                            let a1701 = v2;
                            let a1702 = v8;
                            let a1703 = 0u64;
                            nat_quot(a1701, a1702, a1703)
                        };
                        let a1706 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1704) => {
                                h1704
                            }
                        };
                        nat_mul(a1705, a1706)?
                    };
                    let a1708 = f62(v1, v6, 3u64);
                    nat_add(a1707, a1708)?
                };
                Ok(Adt9::C8(Some(f61(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f66(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1709 = v1;
    match m1709 {
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
            f153(f154(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f155(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f156(v0.clone(), v24);
            f153(f157(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1710 = v3;
    match m1710 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f99(v0, v1)?, f99(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f159(v0, v1, v2.clone())?, f99(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f160(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f161(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f162(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f161(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f163(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f164(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f165(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f99(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f166(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v62, v63) => {
            f167(v0, v1, v2, v62, v63)
        }
        Adt4::C12(v64, v65, v66) => {
            f168(v0, v1, v2, v64, v65, v66)
        }
        Adt4::C13(_, v68, _, _) => {
            Ok(Adt9::C3(v68, v68, f99(v0, v1)?, f99(v0, v2)?))
        }
        Adt4::C14(v71, v72, v73) => {
            let a1713 = v71;
            let a1714 = 23u64;
            if nat_eq(a1713, a1714) {
                f169(v0, v1, v2, v72, v73)
            } else {
                Ok(Adt9::C3(v72, v73, f99(v0, v1)?, f99(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f170(v0, v1, v2)
        }
        Adt4::C16(v74, v75) => {
            f171(v0, v1, v2, v74, v75)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v76, v77) => {
            f172(v0, v1, v77, v76)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f99(f158(v0, match v2.clone() {
                Adt3::C0(h1711, _, _, _, _, _, _, _) => {
                    h1711
                }
            }, match v2.clone() {
                Adt3::C0(_, h1712, _, _, _, _, _, _) => {
                    h1712
                }
            })?, v1)?, f99(v0, v2)?))
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
        Adt4::C25(v82, v83, v84) => {
            f173(v0, v82, v83, v84)
        }
        Adt4::C26(v90, _, _, _) => {
            let a1715 = v90;
            let a1716 = 64u64;
            if if if nat_eq(a1715, a1716) {
                true
            } else {
                let a1717 = v90;
                let a1718 = 65u64;
                nat_eq(a1717, a1718)
            } {
                true
            } else {
                let a1719 = v90;
                let a1720 = 66u64;
                nat_eq(a1719, a1720)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v89) => {
            f91(v0, v1, v2, v89)
        }
        Adt4::C28(v85) => {
            f88(v0, v1, v2, v85, 0u64)
        }
        Adt4::C29(v86) => {
            f89(v0, v1, v2, v86)
        }
        Adt4::C30(v87, _) => {
            f90(v0, v1, v2, v87)
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
        Adt4::C46(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f174(v3, v4);
    let v6: u64 = {
        let a1721 = v0;
        let a1722 = v5;
        let a1723 = 0u64;
        nat_quot(a1721, a1722, a1723)
    };
    let a1724 = v5;
    let a1725 = 1u64;
    if nat_eq(a1724, a1725) {
        Ok(f153(f175(v3, v4), v3, v4, 0u64, f99(v6, v1)?, 2u64, 0u64, 3u64, f99(v6, v2)?))
    } else {
        let a1729 = {
            let a1726 = v0;
            let a1727 = 2u64;
            let a1728 = 0u64;
            nat_rem(a1726, a1727, a1728)
        };
        let a1730 = 0u64;
        if nat_eq(a1729, a1730) {
            Ok(f153(f175(v3, 0u64), v3, 0u64, 0u64, f99(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f153(f175(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f99(v6, v2)?))
        }
    }
}

pub fn f69(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f176(v1)?;
    let v4: u64 = {
        let a1731 = v0;
        let a1732 = v3;
        let a1733 = 0u64;
        nat_rem(a1731, a1732, a1733)
    };
    let v5: u64 = {
        let a1736 = {
            let a1734 = v1;
            let a1735 = 3u64;
            nat_mul(a1734, a1735)?
        };
        let a1737 = 1u64;
        nat_add(a1736, a1737)?
    };
    let a1738 = v4;
    let a1739 = 0u64;
    if nat_eq(a1738, a1739) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1740 = v4;
        let a1741 = v5;
        if nat_eq(a1740, a1741) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1746 = v4;
                let a1747 = {
                    let a1742 = v4;
                    let a1743 = v5;
                    if nat_lt(a1742, a1743) {
                        1u64
                    } else {
                        let a1744 = v5;
                        let a1745 = 1u64;
                        nat_add(a1744, a1745)?
                    }
                };
                nat_sub(a1746, a1747)
            };
            let v7: u64 = {
                let a1748 = v6;
                let a1749 = 3u64;
                let a1750 = 0u64;
                nat_rem(a1748, a1749, a1750)
            };
            let v8: u64 = {
                let a1761 = 4u64;
                let a1762 = {
                    let a1759 = {
                        let a1754 = {
                            let a1751 = v0;
                            let a1752 = v3;
                            let a1753 = 0u64;
                            nat_quot(a1751, a1752, a1753)
                        };
                        let a1755 = v1;
                        nat_mul(a1754, a1755)?
                    };
                    let a1760 = {
                        let a1756 = v6;
                        let a1757 = 3u64;
                        let a1758 = 0u64;
                        nat_quot(a1756, a1757, a1758)
                    };
                    nat_add(a1759, a1760)?
                };
                nat_mul(a1761, a1762)?
            };
            let a1763 = v4;
            let a1764 = v5;
            if nat_lt(a1763, a1764) {
                let a1765 = v7;
                let a1766 = 0u64;
                if nat_eq(a1765, a1766) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1767 = v7;
                    let a1768 = 1u64;
                    if nat_eq(a1767, a1768) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1769 = v7;
                let a1770 = 0u64;
                if nat_eq(a1769, a1770) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1771 = v7;
                    let a1772 = 1u64;
                    if nat_eq(a1771, a1772) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1773 = v0;
        let a1774 = 6u64;
        let a1775 = 0u64;
        nat_quot(a1773, a1774, a1775)
    };
    let v5: u64 = {
        let a1776 = v0;
        let a1777 = 6u64;
        let a1778 = 0u64;
        nat_rem(a1776, a1777, a1778)
    };
    let a1779 = v5;
    let a1780 = 0u64;
    if nat_eq(a1779, a1780) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f99(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1781 = v5;
        let a1782 = 1u64;
        if nat_eq(a1781, a1782) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1783 = v5;
            let a1784 = 2u64;
            if nat_eq(a1783, a1784) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f99(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1785 = v5;
                let a1786 = 3u64;
                if nat_eq(a1785, a1786) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1787 = v5;
                    let a1788 = 4u64;
                    if nat_eq(a1787, a1788) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f99(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f99(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f177(match v1.clone() {
        Adt3::C0(_, h1789, _, _, _, _, _, _) => {
            h1789
        }
    }, v4)?;
    let v8: u64 = {
        let a1790 = v0;
        let a1791 = v7;
        let a1792 = 0u64;
        nat_rem(a1790, a1791, a1792)
    };
    let v9: u64 = {
        let a1793 = v0;
        let a1794 = v7;
        let a1795 = 0u64;
        nat_quot(a1793, a1794, a1795)
    };
    let a1798 = {
        let a1796 = v8;
        let a1797 = 1u64;
        nat_add(a1796, a1797)?
    };
    let a1799 = v7;
    if nat_eq(a1798, a1799) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f99(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1800 = v8;
            let a1801 = v6;
            let a1802 = 0u64;
            nat_quot(a1800, a1801, a1802)
        };
        let v11: u64 = {
            let a1803 = v8;
            let a1804 = v6;
            let a1805 = 0u64;
            nat_rem(a1803, a1804, a1805)
        };
        let v12: u64 = {
            let a1807 = v9;
            let a1808 = match v1.clone() {
                Adt3::C0(h1806, _, _, _, _, _, _, _) => {
                    h1806
                }
            };
            let a1809 = 0u64;
            nat_rem(a1807, a1808, a1809)
        };
        let v13: u64 = {
            let a1811 = v9;
            let a1812 = match v1.clone() {
                Adt3::C0(h1810, _, _, _, _, _, _, _) => {
                    h1810
                }
            };
            let a1813 = 0u64;
            nat_quot(a1811, a1812, a1813)
        };
        let v14: u64 = if v4 {
            let a1825 = {
                let a1820 = {
                    let a1815 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1814, _, _, _) => {
                            h1814
                        }
                    };
                    let a1816 = v12;
                    nat_mul(a1815, a1816)?
                };
                let a1821 = {
                    let a1818 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1817, _, _) => {
                            h1817
                        }
                    };
                    let a1819 = v10;
                    nat_mul(a1818, a1819)?
                };
                nat_add(a1820, a1821)?
            };
            let a1826 = {
                let a1823 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1822, _) => {
                        h1822
                    }
                };
                let a1824 = v13;
                nat_mul(a1823, a1824)?
            };
            nat_add(a1825, a1826)?
        } else {
            let a1833 = {
                let a1828 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1827, _, _, _) => {
                        h1827
                    }
                };
                let a1829 = v10;
                nat_mul(a1828, a1829)?
            };
            let a1834 = {
                let a1831 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1830, _, _) => {
                        h1830
                    }
                };
                let a1832 = v13;
                nat_mul(a1831, a1832)?
            };
            nat_add(a1833, a1834)?
        };
        if if v4 {
            let a1835 = v11;
            let a1836 = 0u64;
            nat_eq(a1835, a1836)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1839 = {
                let a1837 = v11;
                let a1838 = 1u64;
                nat_add(a1837, a1838)?
            };
            let a1840 = v6;
            if nat_eq(a1839, a1840) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1841 = v10;
                    let a1842 = 0u64;
                    if nat_eq(a1841, a1842) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1854 = {
                        let a1849 = {
                            let a1844 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1843, _, _, _) => {
                                    h1843
                                }
                            };
                            let a1845 = v12;
                            nat_mul(a1844, a1845)?
                        };
                        let a1850 = {
                            let a1847 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1846, _, _) => {
                                    h1846
                                }
                            };
                            let a1848 = v10;
                            nat_mul(a1847, a1848)?
                        };
                        nat_add(a1849, a1850)?
                    };
                    let a1855 = {
                        let a1852 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1851, _) => {
                                h1851
                            }
                        };
                        let a1853 = v13;
                        nat_mul(a1852, a1853)?
                    };
                    nat_add(a1854, a1855)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f72(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f38(match v2.clone() {
        Adt3::C0(_, h1856, _, _, _, _, _, _) => {
            h1856
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1857 = v0;
        let a1858 = v6;
        let a1859 = 0u64;
        nat_rem(a1857, a1858, a1859)
    };
    let v8: u64 = {
        let a1860 = v0;
        let a1861 = v6;
        let a1862 = 0u64;
        nat_quot(a1860, a1861, a1862)
    };
    let v9: u64 = {
        let a1864 = v8;
        let a1865 = match v5.clone() {
            Adt3::C0(h1863, _, _, _, _, _, _, _) => {
                h1863
            }
        };
        let a1866 = 0u64;
        nat_rem(a1864, a1865, a1866)
    };
    let v10: u64 = {
        let a1868 = v8;
        let a1869 = match v5.clone() {
            Adt3::C0(h1867, _, _, _, _, _, _, _) => {
                h1867
            }
        };
        let a1870 = 0u64;
        nat_quot(a1868, a1869, a1870)
    };
    let v11: u64 = {
        let a1872 = v10;
        let a1873 = match v2.clone() {
            Adt3::C0(_, h1871, _, _, _, _, _, _) => {
                h1871
            }
        };
        let a1874 = 0u64;
        nat_rem(a1872, a1873, a1874)
    };
    let v12: u64 = {
        let a1876 = v10;
        let a1877 = match v2.clone() {
            Adt3::C0(_, h1875, _, _, _, _, _, _) => {
                h1875
            }
        };
        let a1878 = 0u64;
        nat_quot(a1876, a1877, a1878)
    };
    let a1879 = v7;
    let a1880 = 0u64;
    if nat_eq(a1879, a1880) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1887 = {
                let a1882 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1881, _, _, _) => {
                        h1881
                    }
                };
                let a1883 = v9;
                nat_mul(a1882, a1883)?
            };
            let a1888 = {
                let a1885 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1884, _, _) => {
                        h1884
                    }
                };
                let a1886 = v12;
                nat_mul(a1885, a1886)?
            };
            nat_add(a1887, a1888)?
        }), Adt12::C0(2u64, {
            let a1895 = {
                let a1890 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1889, _, _, _) => {
                        h1889
                    }
                };
                let a1891 = v11;
                nat_mul(a1890, a1891)?
            };
            let a1896 = {
                let a1893 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1892, _, _) => {
                        h1892
                    }
                };
                let a1894 = v12;
                nat_mul(a1893, a1894)?
            };
            nat_add(a1895, a1896)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1897 = v4;
        match m1897 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1909 = {
                        let a1904 = {
                            let a1899 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1898, _, _, _) => {
                                    h1898
                                }
                            };
                            let a1900 = v9;
                            nat_mul(a1899, a1900)?
                        };
                        let a1905 = {
                            let a1902 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1901, _, _) => {
                                    h1901
                                }
                            };
                            let a1903 = v11;
                            nat_mul(a1902, a1903)?
                        };
                        nat_add(a1904, a1905)?
                    };
                    let a1910 = {
                        let a1907 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1906, _) => {
                                h1906
                            }
                        };
                        let a1908 = v12;
                        nat_mul(a1907, a1908)?
                    };
                    nat_add(a1909, a1910)?
                }), Adt13::C1(f99(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1911 = v7;
                    let a1912 = 1u64;
                    nat_sub(a1911, a1912)
                };
                let v15: u64 = {
                    let a1913 = v14;
                    let a1914 = 2u64;
                    let a1915 = 0u64;
                    nat_quot(a1913, a1914, a1915)
                };
                let a1919 = {
                    let a1916 = v14;
                    let a1917 = 2u64;
                    let a1918 = 0u64;
                    nat_rem(a1916, a1917, a1918)
                };
                let a1920 = 0u64;
                if nat_eq(a1919, a1920) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1932 = {
                            let a1927 = {
                                let a1922 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1921, _, _, _) => {
                                        h1921
                                    }
                                };
                                let a1923 = v9;
                                nat_mul(a1922, a1923)?
                            };
                            let a1928 = {
                                let a1925 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1924, _, _) => {
                                        h1924
                                    }
                                };
                                let a1926 = v15;
                                nat_mul(a1925, a1926)?
                            };
                            nat_add(a1927, a1928)?
                        };
                        let a1933 = {
                            let a1930 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1929, _) => {
                                    h1929
                                }
                            };
                            let a1931 = v12;
                            nat_mul(a1930, a1931)?
                        };
                        nat_add(a1932, a1933)?
                    }), Adt12::C0(3u64, {
                        let a1945 = {
                            let a1940 = {
                                let a1935 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1934, _, _, _) => {
                                        h1934
                                    }
                                };
                                let a1936 = v11;
                                nat_mul(a1935, a1936)?
                            };
                            let a1941 = {
                                let a1938 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1937, _, _) => {
                                        h1937
                                    }
                                };
                                let a1939 = v15;
                                nat_mul(a1938, a1939)?
                            };
                            nat_add(a1940, a1941)?
                        };
                        let a1946 = {
                            let a1943 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1942, _) => {
                                    h1942
                                }
                            };
                            let a1944 = v12;
                            nat_mul(a1943, a1944)?
                        };
                        nat_add(a1945, a1946)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1949 = {
                            let a1947 = v7;
                            let a1948 = 1u64;
                            nat_add(a1947, a1948)?
                        };
                        let a1950 = v6;
                        if nat_eq(a1949, a1950) {
                            Adt13::C1(f99(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f37(v6)?;
    let v9: u64 = {
        let a1951 = v0;
        let a1952 = v8;
        let a1953 = 0u64;
        nat_rem(a1951, a1952, a1953)
    };
    let v10: u64 = {
        let a1954 = v0;
        let a1955 = v8;
        let a1956 = 0u64;
        nat_quot(a1954, a1955, a1956)
    };
    let a1957 = v9;
    let a1958 = 148u64;
    if nat_lt(a1957, a1958) {
        f178({
            let a1959 = v9;
            let a1960 = 37u64;
            let a1961 = 0u64;
            nat_rem(a1959, a1960, a1961)
        }, {
            let a1962 = v9;
            let a1963 = 37u64;
            let a1964 = 0u64;
            nat_quot(a1962, a1963, a1964)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f179({
            let a1965 = v9;
            let a1966 = 148u64;
            nat_sub(a1965, a1966)
        }, v10, v4, v5)
    }
}

pub fn f74(v0: u64, v1: Adt3, v2: bool) -> R<Adt9> {
    let v3: u64 = f33(v2);
    let a1967 = v0;
    let a1968 = v3;
    if nat_lt(a1967, a1968) {
        let a1969 = v0;
        let a1970 = if v2 {
            7u64
        } else {
            2u64
        };
        if nat_lt(a1969, a1970) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(if v2 {
                4u64
            } else {
                2u64
            }, {
                let a1971 = 4u64;
                let a1972 = v0;
                nat_mul(a1971, a1972)?
            }), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
        } else {
            let a1973 = v0;
            let a1974 = if v2 {
                7u64
            } else {
                2u64
            };
            if nat_eq(a1973, a1974) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(if v2 {
                    4u64
                } else {
                    1u64
                }), Adt13::C0(7u64), 4u64, 4u64))
            } else {
                let a1977 = v0;
                let a1978 = {
                    let a1975 = if v2 {
                        7u64
                    } else {
                        2u64
                    };
                    let a1976 = 1u64;
                    nat_add(a1975, a1976)?
                };
                if nat_eq(a1977, a1978) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(7u64), Adt13::C0(7u64), 4u64, 4u64))
                } else {
                    let a1979 = v0;
                    let a1980 = 9u64;
                    if nat_eq(a1979, a1980) {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(8u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(2u64), Adt13::C0(9u64), 4u64, 4u64))
                    }
                }
            }
        }
    } else {
        let v4: u64 = {
            let a1981 = v0;
            let a1982 = v3;
            nat_sub(a1981, a1982)
        };
        let v5: u64 = f34(v2);
        let v6: u64 = {
            let a1983 = v4;
            let a1984 = v5;
            let a1985 = 0u64;
            nat_rem(a1983, a1984, a1985)
        };
        let v7: u64 = f99({
            let a1986 = v4;
            let a1987 = v5;
            let a1988 = 0u64;
            nat_quot(a1986, a1987, a1988)
        }, v1)?;
        if v2 {
            let a1989 = v6;
            let a1990 = 7u64;
            if nat_lt(a1989, a1990) {
                let a1991 = v6;
                let a1992 = 3u64;
                if nat_lt(a1991, a1992) {
                    let a1993 = v6;
                    let a1994 = 1u64;
                    if nat_lt(a1993, a1994) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(1u64), Adt13::C0(10u64), 4u64, 4u64))
                    } else {
                        let a1995 = v6;
                        let a1996 = 2u64;
                        if nat_lt(a1995, a1996) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C1(8u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(2u64, v7), 4u64, 4u64))
                        }
                    }
                } else {
                    let a1997 = v6;
                    let a1998 = 5u64;
                    if nat_lt(a1997, a1998) {
                        let a1999 = v6;
                        let a2000 = 4u64;
                        if nat_lt(a1999, a2000) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(2u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v7), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2001 = v6;
                        let a2002 = 6u64;
                        if nat_lt(a2001, a2002) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(11u64), Adt12::C1(9u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C3(3u64, v7), 4u64, 4u64))
                        }
                    }
                }
            } else {
                let a2003 = v6;
                let a2004 = 11u64;
                if nat_lt(a2003, a2004) {
                    let a2005 = v6;
                    let a2006 = 9u64;
                    if nat_lt(a2005, a2006) {
                        let a2007 = v6;
                        let a2008 = 8u64;
                        if nat_lt(a2007, a2008) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, v7), Adt12::C1(5u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(3u64, v7), Adt12::C1(6u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    } else {
                        let a2009 = v6;
                        let a2010 = 10u64;
                        if nat_lt(a2009, a2010) {
                            Ok(Adt9::C7(Adt11::C1(4u64), Adt12::C1(11u64), Adt12::C2(0u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(11u64), Adt12::C1(3u64), Adt13::C0(11u64), 4u64, 4u64))
                        }
                    }
                } else {
                    let a2011 = v6;
                    let a2012 = 13u64;
                    if nat_lt(a2011, a2012) {
                        let a2013 = v6;
                        let a2014 = 12u64;
                        if nat_lt(a2013, a2014) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(10u64), Adt13::C0(10u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C0(10u64), 4u64, 4u64))
                        }
                    } else {
                        let a2015 = v6;
                        let a2016 = 14u64;
                        if nat_lt(a2015, a2016) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(11u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(11u64), Adt12::C1(10u64), Adt13::C1(v7), 4u64, 4u64))
                        }
                    }
                }
            }
        } else {
            let a2017 = v6;
            let a2018 = 1u64;
            if nat_lt(a2017, a2018) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, v7), Adt12::C1(7u64), Adt13::C0(10u64), 4u64, 4u64))
            } else {
                let a2019 = v6;
                let a2020 = 2u64;
                if nat_lt(a2019, a2020) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C0(1u64, v7), Adt13::C0(11u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(10u64), Adt12::C1(11u64), Adt13::C1(v7), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f75(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: u64) -> R<Adt9> {
    let v7: u64 = match v1.clone() {
        Adt3::C0(h2021, _, _, _, _, _, _, _) => {
            h2021
        }
    };
    let v8: u64 = f30(v3, v7)?;
    let v9: u64 = {
        let a2022 = v0;
        let a2023 = 2u64;
        nat_sub(a2022, a2023)
    };
    let v10: u64 = {
        let a2024 = v9;
        let a2025 = v8;
        let a2026 = 0u64;
        nat_quot(a2024, a2025, a2026)
    };
    let v11: u64 = {
        let a2027 = v9;
        let a2028 = v8;
        let a2029 = 0u64;
        nat_rem(a2027, a2028, a2029)
    };
    let a2030 = v0;
    let a2031 = 0u64;
    if nat_eq(a2030, a2031) {
        Ok(Adt9::C7(Adt11::C0(3u64), Adt12::C2(v5), Adt12::C2(0u64), Adt13::C0(6u64), 4u64, 4u64))
    } else {
        let a2032 = v0;
        let a2033 = 1u64;
        if nat_eq(a2032, a2033) {
            Ok(Adt9::C7(Adt11::C1(3u64), if v3 {
                Adt12::C0(0u64, 0u64)
            } else {
                Adt12::C2(3212836864u64)
            }, Adt12::C1(6u64), Adt13::C0(6u64), 4u64, 4u64))
        } else {
            let a2034 = v10;
            let a2035 = v5;
            if nat_eq(a2034, a2035) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(5u64), Adt12::C1(6u64), Adt13::C1(0u64), 4u64, 4u64))
            } else {
                let a2038 = v11;
                let a2039 = {
                    let a2036 = 2u64;
                    let a2037 = v7;
                    nat_mul(a2036, a2037)?
                };
                if nat_lt(a2038, a2039) {
                    let v12: u64 = {
                        let a2040 = v11;
                        let a2041 = 2u64;
                        let a2042 = 0u64;
                        nat_quot(a2040, a2041, a2042)
                    };
                    let a2046 = {
                        let a2043 = v11;
                        let a2044 = 2u64;
                        let a2045 = 0u64;
                        nat_rem(a2043, a2044, a2045)
                    };
                    let a2047 = 0u64;
                    if nat_eq(a2046, a2047) {
                        Ok(Adt9::C7(Adt11::C5(0u64), {
                            let a2048 = v12;
                            let a2049 = 0u64;
                            if nat_eq(a2048, a2049) {
                                Adt12::C2(4286578688u64)
                            } else {
                                Adt12::C1(0u64)
                            }
                        }, Adt12::C0(if v3 {
                            1u64
                        } else {
                            0u64
                        }, {
                            let a2055 = {
                                let a2051 = v10;
                                let a2052 = match v1 {
                                    Adt3::C0(_, _, _, _, _, h2050, _, _) => {
                                        h2050
                                    }
                                };
                                nat_mul(a2051, a2052)?
                            };
                            let a2056 = {
                                let a2053 = 4u64;
                                let a2054 = v12;
                                nat_mul(a2053, a2054)?
                            };
                            nat_add(a2055, a2056)?
                        }), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C5(3u64), {
                            let a2057 = v6;
                            let a2058 = 2u64;
                            if nat_eq(a2057, a2058) {
                                let a2059 = v12;
                                let a2060 = 0u64;
                                if nat_eq(a2059, a2060) {
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
                                    let a2066 = {
                                        let a2062 = v10;
                                        let a2063 = match v1 {
                                            Adt3::C0(_, _, _, _, _, h2061, _, _) => {
                                                h2061
                                            }
                                        };
                                        nat_mul(a2062, a2063)?
                                    };
                                    let a2067 = {
                                        let a2064 = 4u64;
                                        let a2065 = v12;
                                        nat_mul(a2064, a2065)?
                                    };
                                    nat_add(a2066, a2067)?
                                })
                            }
                        }, Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    }
                } else {
                    let a2070 = v11;
                    let a2071 = {
                        let a2068 = 6u64;
                        let a2069 = v7;
                        nat_mul(a2068, a2069)?
                    };
                    if nat_lt(a2070, a2071) {
                        let v13: u64 = {
                            let a2074 = v11;
                            let a2075 = {
                                let a2072 = 2u64;
                                let a2073 = v7;
                                nat_mul(a2072, a2073)?
                            };
                            nat_sub(a2074, a2075)
                        };
                        let v14: u64 = {
                            let a2076 = v13;
                            let a2077 = 4u64;
                            let a2078 = 0u64;
                            nat_quot(a2076, a2077, a2078)
                        };
                        let a2082 = {
                            let a2079 = v13;
                            let a2080 = 4u64;
                            let a2081 = 0u64;
                            nat_rem(a2079, a2080, a2081)
                        };
                        let a2083 = 2u64;
                        if nat_lt(a2082, a2083) {
                            let a2087 = {
                                let a2084 = v13;
                                let a2085 = 4u64;
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
                                        let a2093 = v14;
                                        nat_mul(a2092, a2093)?
                                    };
                                    nat_add(a2094, a2095)?
                                }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), if v3 {
                                    Adt13::C1({
                                        let a2101 = {
                                            let a2097 = v10;
                                            let a2098 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2096, _, _) => {
                                                    h2096
                                                }
                                            };
                                            nat_mul(a2097, a2098)?
                                        };
                                        let a2102 = {
                                            let a2099 = 4u64;
                                            let a2100 = v14;
                                            nat_mul(a2099, a2100)?
                                        };
                                        nat_add(a2101, a2102)?
                                    })
                                } else {
                                    Adt13::C0(3u64)
                                }, 4u64, 4u64))
                            }
                        } else {
                            let a2106 = {
                                let a2103 = v13;
                                let a2104 = 4u64;
                                let a2105 = 0u64;
                                nat_rem(a2103, a2104, a2105)
                            };
                            let a2107 = 3u64;
                            if nat_lt(a2106, a2107) {
                                Ok(Adt9::C7(Adt11::C0(1u64), if v3 {
                                    Adt12::C3({
                                        let a2113 = {
                                            let a2109 = v10;
                                            let a2110 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2108, _, _) => {
                                                    h2108
                                                }
                                            };
                                            nat_mul(a2109, a2110)?
                                        };
                                        let a2114 = {
                                            let a2111 = 4u64;
                                            let a2112 = v14;
                                            nat_mul(a2111, a2112)?
                                        };
                                        nat_add(a2113, a2114)?
                                    })
                                } else {
                                    Adt12::C1(3u64)
                                }, Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                    let a2115 = v14;
                                    let a2116 = 0u64;
                                    if nat_eq(a2115, a2116) {
                                        Adt12::C2(0u64)
                                    } else {
                                        Adt12::C1(1u64)
                                    }
                                }, Adt12::C1(3u64), Adt13::C0(1u64), 4u64, 4u64))
                            }
                        }
                    } else {
                        let a2121 = v11;
                        let a2122 = {
                            let a2119 = {
                                let a2117 = 6u64;
                                let a2118 = v7;
                                nat_mul(a2117, a2118)?
                            };
                            let a2120 = 2u64;
                            nat_add(a2119, a2120)?
                        };
                        if nat_lt(a2121, a2122) {
                            let a2125 = v11;
                            let a2126 = {
                                let a2123 = 6u64;
                                let a2124 = v7;
                                nat_mul(a2123, a2124)?
                            };
                            if nat_eq(a2125, a2126) {
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
                                    let a2131 = v11;
                                    let a2132 = {
                                        let a2129 = {
                                            let a2127 = 6u64;
                                            let a2128 = v7;
                                            nat_mul(a2127, a2128)?
                                        };
                                        let a2130 = 2u64;
                                        nat_add(a2129, a2130)?
                                    };
                                    nat_sub(a2131, a2132)
                                };
                                let v16: u64 = {
                                    let a2133 = v15;
                                    let a2134 = 3u64;
                                    let a2135 = 0u64;
                                    nat_quot(a2133, a2134, a2135)
                                };
                                let a2139 = {
                                    let a2136 = v15;
                                    let a2137 = 3u64;
                                    let a2138 = 0u64;
                                    nat_rem(a2136, a2137, a2138)
                                };
                                let a2140 = 1u64;
                                if nat_lt(a2139, a2140) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3({
                                        let a2146 = {
                                            let a2142 = v10;
                                            let a2143 = match v2 {
                                                Adt3::C0(_, _, _, _, _, h2141, _, _) => {
                                                    h2141
                                                }
                                            };
                                            nat_mul(a2142, a2143)?
                                        };
                                        let a2147 = {
                                            let a2144 = 4u64;
                                            let a2145 = v16;
                                            nat_mul(a2144, a2145)?
                                        };
                                        nat_add(a2146, a2147)?
                                    }), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                } else {
                                    let a2151 = {
                                        let a2148 = v15;
                                        let a2149 = 3u64;
                                        let a2150 = 0u64;
                                        nat_rem(a2148, a2149, a2150)
                                    };
                                    let a2152 = 2u64;
                                    if nat_lt(a2151, a2152) {
                                        Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                            2u64
                                        } else {
                                            1u64
                                        }, {
                                            let a2157 = {
                                                let a2153 = v10;
                                                let a2154 = v4;
                                                nat_mul(a2153, a2154)?
                                            };
                                            let a2158 = {
                                                let a2155 = 4u64;
                                                let a2156 = v16;
                                                nat_mul(a2155, a2156)?
                                            };
                                            nat_add(a2157, a2158)?
                                        }), Adt13::C0(3u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C1(6u64), Adt13::C1({
                                            let a2164 = {
                                                let a2160 = v10;
                                                let a2161 = match v2 {
                                                    Adt3::C0(_, _, _, _, _, h2159, _, _) => {
                                                        h2159
                                                    }
                                                };
                                                nat_mul(a2160, a2161)?
                                            };
                                            let a2165 = {
                                                let a2162 = 4u64;
                                                let a2163 = v16;
                                                nat_mul(a2162, a2163)?
                                            };
                                            nat_add(a2164, a2165)?
                                        }), 4u64, 4u64))
                                    }
                                }
                            } else {
                                let a2170 = v11;
                                let a2171 = {
                                    let a2168 = {
                                        let a2166 = 11u64;
                                        let a2167 = v7;
                                        nat_mul(a2166, a2167)?
                                    };
                                    let a2169 = 2u64;
                                    nat_add(a2168, a2169)?
                                };
                                if nat_lt(a2170, a2171) {
                                    let v17: u64 = {
                                        let a2176 = v11;
                                        let a2177 = {
                                            let a2174 = {
                                                let a2172 = 6u64;
                                                let a2173 = v7;
                                                nat_mul(a2172, a2173)?
                                            };
                                            let a2175 = 2u64;
                                            nat_add(a2174, a2175)?
                                        };
                                        nat_sub(a2176, a2177)
                                    };
                                    let v18: u64 = {
                                        let a2178 = v17;
                                        let a2179 = 5u64;
                                        let a2180 = 0u64;
                                        nat_quot(a2178, a2179, a2180)
                                    };
                                    let a2184 = {
                                        let a2181 = v17;
                                        let a2182 = 5u64;
                                        let a2183 = 0u64;
                                        nat_rem(a2181, a2182, a2183)
                                    };
                                    let a2185 = 2u64;
                                    if nat_lt(a2184, a2185) {
                                        let a2189 = {
                                            let a2186 = v17;
                                            let a2187 = 5u64;
                                            let a2188 = 0u64;
                                            nat_rem(a2186, a2187, a2188)
                                        };
                                        let a2190 = 1u64;
                                        if nat_lt(a2189, a2190) {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(if v3 {
                                                1u64
                                            } else {
                                                0u64
                                            }, {
                                                let a2196 = {
                                                    let a2192 = v10;
                                                    let a2193 = match v1 {
                                                        Adt3::C0(_, _, _, _, _, h2191, _, _) => {
                                                            h2191
                                                        }
                                                    };
                                                    nat_mul(a2192, a2193)?
                                                };
                                                let a2197 = {
                                                    let a2194 = 4u64;
                                                    let a2195 = v18;
                                                    nat_mul(a2194, a2195)?
                                                };
                                                nat_add(a2196, a2197)?
                                            }), Adt12::C1(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                                        }
                                    } else {
                                        let a2201 = {
                                            let a2198 = v17;
                                            let a2199 = 5u64;
                                            let a2200 = 0u64;
                                            nat_rem(a2198, a2199, a2200)
                                        };
                                        let a2202 = 3u64;
                                        if nat_lt(a2201, a2202) {
                                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(3u64), Adt12::C0(if v3 {
                                                2u64
                                            } else {
                                                1u64
                                            }, {
                                                let a2207 = {
                                                    let a2203 = v10;
                                                    let a2204 = v4;
                                                    nat_mul(a2203, a2204)?
                                                };
                                                let a2208 = {
                                                    let a2205 = 4u64;
                                                    let a2206 = v18;
                                                    nat_mul(a2205, a2206)?
                                                };
                                                nat_add(a2207, a2208)?
                                            }), Adt13::C0(3u64), 4u64, 4u64))
                                        } else {
                                            let a2212 = {
                                                let a2209 = v17;
                                                let a2210 = 5u64;
                                                let a2211 = 0u64;
                                                nat_rem(a2209, a2210, a2211)
                                            };
                                            let a2213 = 4u64;
                                            if nat_lt(a2212, a2213) {
                                                Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C0(0u64), {
                                                    let a2214 = v18;
                                                    let a2215 = 0u64;
                                                    if nat_eq(a2214, a2215) {
                                                        Adt12::C2(0u64)
                                                    } else {
                                                        Adt12::C1(4u64)
                                                    }
                                                }, Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                                            }
                                        }
                                    }
                                } else {
                                    let a2220 = v11;
                                    let a2221 = {
                                        let a2218 = {
                                            let a2216 = 11u64;
                                            let a2217 = v7;
                                            nat_mul(a2216, a2217)?
                                        };
                                        let a2219 = 2u64;
                                        nat_add(a2218, a2219)?
                                    };
                                    if nat_eq(a2220, a2221) {
                                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C0(4u64), 4u64, 4u64))
                                    } else {
                                        Ok(Adt9::C7(Adt11::C1(0u64), {
                                            let a2222 = v10;
                                            let a2223 = 0u64;
                                            if nat_eq(a2222, a2223) {
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

pub fn f76(v0: u64, v1: Adt3, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1.clone() {
        Adt3::C0(h2224, _, _, _, _, _, _, _) => {
            h2224
        }
    };
    let v5: u64 = {
        let a2226 = v4;
        let a2227 = match v1.clone() {
            Adt3::C0(_, h2225, _, _, _, _, _, _) => {
                h2225
            }
        };
        nat_mul(a2226, a2227)?
    };
    let v6: u64 = {
        let a2229 = v5;
        let a2230 = match v1.clone() {
            Adt3::C0(_, _, h2228, _, _, _, _, _) => {
                h2228
            }
        };
        nat_mul(a2229, a2230)?
    };
    let a2231 = v0;
    let a2232 = v6;
    if nat_lt(a2231, a2232) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2233 = 4u64;
            let a2234 = v0;
            nat_mul(a2233, a2234)?
        }), 4u64, 4u64))
    } else {
        let v7: u64 = {
            let a2235 = v0;
            let a2236 = v6;
            nat_sub(a2235, a2236)
        };
        let v8: u64 = {
            let a2239 = 1u64;
            let a2240 = {
                let a2237 = 5u64;
                let a2238 = v4;
                nat_mul(a2237, a2238)?
            };
            nat_add(a2239, a2240)?
        };
        let v9: u64 = {
            let a2241 = v7;
            let a2242 = v8;
            let a2243 = 0u64;
            nat_quot(a2241, a2242, a2243)
        };
        let v10: u64 = {
            let a2244 = v7;
            let a2245 = v8;
            let a2246 = 0u64;
            nat_rem(a2244, a2245, a2246)
        };
        let a2247 = v10;
        let a2248 = 0u64;
        if nat_eq(a2247, a2248) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(2u64, {
                let a2249 = 4u64;
                let a2250 = v9;
                nat_mul(a2249, a2250)?
            }), Adt12::C2(v2), Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let v11: u64 = {
                let a2251 = v10;
                let a2252 = 1u64;
                nat_sub(a2251, a2252)
            };
            let v12: u64 = {
                let a2253 = v11;
                let a2254 = 5u64;
                let a2255 = 0u64;
                nat_quot(a2253, a2254, a2255)
            };
            let v13: u64 = {
                let a2256 = v11;
                let a2257 = 5u64;
                let a2258 = 0u64;
                nat_rem(a2256, a2257, a2258)
            };
            let v14: u64 = {
                let a2259 = v9;
                let a2260 = v5;
                let a2261 = 0u64;
                nat_quot(a2259, a2260, a2261)
            };
            let v15: u64 = {
                let a2262 = v9;
                let a2263 = v5;
                let a2264 = 0u64;
                nat_rem(a2262, a2263, a2264)
            };
            let v16: u64 = {
                let a2266 = match v1 {
                    Adt3::C0(_, _, h2265, _, _, _, _, _) => {
                        h2265
                    }
                };
                let a2267 = v3;
                let a2268 = 0u64;
                nat_quot(a2266, a2267, a2268)
            };
            let v17: u64 = {
                let a2276 = 4u64;
                let a2277 = {
                    let a2274 = {
                        let a2272 = v9;
                        let a2273 = {
                            let a2269 = v15;
                            let a2270 = v4;
                            let a2271 = 0u64;
                            nat_rem(a2269, a2270, a2271)
                        };
                        nat_sub(a2272, a2273)
                    };
                    let a2275 = v12;
                    nat_add(a2274, a2275)?
                };
                nat_mul(a2276, a2277)?
            };
            let v18: u64 = {
                let a2289 = 4u64;
                let a2290 = {
                    let a2287 = {
                        let a2285 = {
                            let a2283 = {
                                let a2281 = {
                                    let a2278 = v14;
                                    let a2279 = v16;
                                    let a2280 = 0u64;
                                    nat_quot(a2278, a2279, a2280)
                                };
                                let a2282 = v5;
                                nat_mul(a2281, a2282)?
                            };
                            let a2284 = v15;
                            nat_add(a2283, a2284)?
                        };
                        let a2286 = v4;
                        nat_mul(a2285, a2286)?
                    };
                    let a2288 = v12;
                    nat_add(a2287, a2288)?
                };
                nat_mul(a2289, a2290)?
            };
            let v19: u64 = {
                let a2293 = {
                    let a2291 = 4u64;
                    let a2292 = v6;
                    nat_mul(a2291, a2292)?
                };
                let a2294 = v18;
                nat_add(a2293, a2294)?
            };
            let a2295 = v13;
            let a2296 = 0u64;
            if nat_eq(a2295, a2296) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v17), Adt12::C0(0u64, {
                    let a2297 = 4u64;
                    let a2298 = v9;
                    nat_mul(a2297, a2298)?
                }), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a2299 = v13;
                let a2300 = 1u64;
                if nat_eq(a2299, a2300) {
                    Ok(Adt9::C7(Adt11::C1(2u64), {
                        let a2304 = {
                            let a2301 = v14;
                            let a2302 = v16;
                            let a2303 = 0u64;
                            nat_rem(a2301, a2302, a2303)
                        };
                        let a2305 = 0u64;
                        if nat_eq(a2304, a2305) {
                            Adt12::C0(4u64, v18)
                        } else {
                            Adt12::C3(v19)
                        }
                    }, Adt12::C0(3u64, {
                        let a2306 = 4u64;
                        let a2307 = v9;
                        nat_mul(a2306, a2307)?
                    }), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    let a2308 = v13;
                    let a2309 = 2u64;
                    if nat_eq(a2308, a2309) {
                        Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C1(1u64), Adt13::C1(v19), 4u64, 4u64))
                    } else {
                        let a2310 = v13;
                        let a2311 = 3u64;
                        if nat_eq(a2310, a2311) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v19), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3(v17), Adt12::C1(2u64), Adt13::C1(v17), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f77(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let v3: u64 = match v1.clone() {
        Adt3::C0(h2312, _, _, _, _, _, _, _) => {
            h2312
        }
    };
    let v4: u64 = match v1 {
        Adt3::C0(_, h2313, _, _, _, _, _, _) => {
            h2313
        }
    };
    let v5: u64 = {
        let a2314 = v3;
        let a2315 = 2u64;
        let a2316 = 0u64;
        nat_quot(a2314, a2315, a2316)
    };
    let v6: u64 = {
        let a2319 = 5u64;
        let a2320 = {
            let a2317 = 3u64;
            let a2318 = v4;
            nat_mul(a2317, a2318)?
        };
        nat_add(a2319, a2320)?
    };
    let v7: u64 = {
        let a2321 = v5;
        let a2322 = 0u64;
        if nat_eq(a2321, a2322) {
            0u64
        } else {
            let a2325 = 7u64;
            let a2326 = {
                let a2323 = v5;
                let a2324 = v6;
                nat_mul(a2323, a2324)?
            };
            nat_add(a2325, a2326)?
        }
    };
    let a2327 = v7;
    let a2328 = v0;
    if nat_le(a2327, a2328) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1({
            let a2337 = 4u64;
            let a2338 = {
                let a2335 = {
                    let a2331 = {
                        let a2329 = v0;
                        let a2330 = v7;
                        nat_sub(a2329, a2330)
                    };
                    let a2332 = v3;
                    nat_mul(a2331, a2332)?
                };
                let a2336 = {
                    let a2333 = 2u64;
                    let a2334 = v5;
                    nat_mul(a2333, a2334)?
                };
                nat_add(a2335, a2336)?
            };
            nat_mul(a2337, a2338)?
        }), 4u64, 4u64))
    } else {
        let a2339 = v0;
        let a2340 = 7u64;
        if nat_lt(a2339, a2340) {
            let a2341 = v0;
            let a2342 = 0u64;
            if nat_eq(a2341, a2342) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2({
                    let a2343 = f152(v2);
                    let a2344 = f151(v2);
                    nat_add(a2343, a2344)?
                }), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2345 = v0;
                let a2346 = 1u64;
                if nat_eq(a2345, a2346) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2347 = v0;
                    let a2348 = 2u64;
                    if nat_eq(a2347, a2348) {
                        let a2349 = v2;
                        let a2350 = 2147483648u64;
                        if nat_lt(a2349, a2350) {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    } else {
                        let a2351 = v0;
                        let a2352 = 3u64;
                        if nat_eq(a2351, a2352) {
                            Ok(Adt9::C7(Adt11::C7(13u64, 0u64, 0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        } else {
                            let a2353 = v0;
                            let a2354 = 4u64;
                            if nat_eq(a2353, a2354) {
                                Ok(Adt9::C7(Adt11::C1(6u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                let a2355 = v0;
                                let a2356 = 5u64;
                                if nat_eq(a2355, a2356) {
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
                let a2357 = v0;
                let a2358 = 7u64;
                nat_sub(a2357, a2358)
            };
            let v9: u64 = {
                let a2359 = v8;
                let a2360 = v6;
                let a2361 = 0u64;
                nat_quot(a2359, a2360, a2361)
            };
            let v10: u64 = {
                let a2362 = v8;
                let a2363 = v6;
                let a2364 = 0u64;
                nat_rem(a2362, a2363, a2364)
            };
            let a2365 = v10;
            let a2366 = 5u64;
            if nat_lt(a2365, a2366) {
                let a2367 = v10;
                let a2368 = 0u64;
                if nat_eq(a2367, a2368) {
                    Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v9), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                } else {
                    let a2369 = v10;
                    let a2370 = 1u64;
                    if nat_eq(a2369, a2370) {
                        Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                    } else {
                        let a2371 = v10;
                        let a2372 = 2u64;
                        if nat_eq(a2371, a2372) {
                            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(3u64), Adt13::C0(3u64), 4u64, 4u64))
                        } else {
                            let a2373 = v10;
                            let a2374 = 3u64;
                            if nat_eq(a2373, a2374) {
                                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(3u64), Adt12::C1(1u64), Adt13::C0(3u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(3u64), Adt12::C2(0u64), Adt13::C0(3u64), 4u64, 4u64))
                            }
                        }
                    }
                }
            } else {
                let v11: u64 = {
                    let a2375 = v10;
                    let a2376 = 5u64;
                    nat_sub(a2375, a2376)
                };
                let v12: u64 = {
                    let a2377 = v11;
                    let a2378 = 3u64;
                    let a2379 = 0u64;
                    nat_quot(a2377, a2378, a2379)
                };
                let v13: u64 = {
                    let a2380 = v11;
                    let a2381 = 3u64;
                    let a2382 = 0u64;
                    nat_rem(a2380, a2381, a2382)
                };
                let a2383 = v13;
                let a2384 = 0u64;
                if nat_eq(a2383, a2384) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                        let a2385 = 4u64;
                        let a2386 = v12;
                        nat_mul(a2385, a2386)?
                    }), Adt12::C1(3u64), Adt13::C0(4u64), 4u64, 4u64))
                } else {
                    let a2387 = v13;
                    let a2388 = 1u64;
                    if nat_eq(a2387, a2388) {
                        Ok(Adt9::C7(Adt11::C7(15u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2393 = 4u64;
                            let a2394 = {
                                let a2391 = {
                                    let a2389 = v12;
                                    let a2390 = v3;
                                    nat_mul(a2389, a2390)?
                                };
                                let a2392 = v9;
                                nat_add(a2391, a2392)?
                            };
                            nat_mul(a2393, a2394)?
                        }), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C7(14u64, 0u64, 0u64), Adt12::C1(4u64), Adt12::C2(0u64), Adt13::C1({
                            let a2401 = 4u64;
                            let a2402 = {
                                let a2399 = {
                                    let a2395 = v12;
                                    let a2396 = v3;
                                    nat_mul(a2395, a2396)?
                                };
                                let a2400 = {
                                    let a2397 = v5;
                                    let a2398 = v9;
                                    nat_add(a2397, a2398)?
                                };
                                nat_add(a2399, a2400)?
                            };
                            nat_mul(a2401, a2402)?
                        }), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64, v5: u64) -> R<Adt9> {
    let a2403 = v0;
    let a2404 = v4;
    if nat_lt(a2403, a2404) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C1(f99(v0, v2)?), 4u64, 4u64))
    } else {
        let v6: u64 = {
            let a2411 = {
                let m2407 = match v3.clone() {
                    Adt7::C0(h2406, _, _, _, _) => {
                        h2406
                    }
                };
                match m2407 {
                    Adt6::C0(h2405, _, _, _) => {
                        h2405
                    }
                }
            };
            let a2412 = {
                let m2410 = match v3.clone() {
                    Adt7::C0(_, h2409, _, _, _) => {
                        h2409
                    }
                };
                match m2410 {
                    Adt6::C0(h2408, _, _, _) => {
                        h2408
                    }
                }
            };
            nat_mul(a2411, a2412)?
        };
        let v7: u64 = f180(match v3.clone() {
            Adt7::C0(_, _, _, h2413, _) => {
                h2413
            }
        }, v6)?;
        let v8: u64 = {
            let a2414 = v0;
            let a2415 = v4;
            nat_sub(a2414, a2415)
        };
        let v9: u64 = {
            let a2416 = v8;
            let a2417 = v7;
            let a2418 = 0u64;
            nat_quot(a2416, a2417, a2418)
        };
        let v10: u64 = {
            let a2419 = v8;
            let a2420 = v7;
            let a2421 = 0u64;
            nat_rem(a2419, a2420, a2421)
        };
        if match v3.clone() {
            Adt7::C0(_, _, _, h2422, _) => {
                h2422
            }
        } {
            let a2423 = v10;
            let a2424 = 0u64;
            if nat_eq(a2423, a2424) {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(4286578687u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2425 = v10;
                let a2426 = 1u64;
                if nat_eq(a2425, a2426) {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a2429 = {
                        let a2427 = v10;
                        let a2428 = 2u64;
                        nat_add(a2427, a2428)?
                    };
                    let a2430 = v7;
                    if nat_eq(a2429, a2430) {
                        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
                    } else {
                        let a2433 = {
                            let a2431 = v10;
                            let a2432 = 1u64;
                            nat_add(a2431, a2432)?
                        };
                        let a2434 = v7;
                        if nat_eq(a2433, a2434) {
                            let a2435 = v5;
                            let a2436 = 0u64;
                            if nat_eq(a2435, a2436) {
                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                            } else {
                                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C3({
                                    let a2437 = v5;
                                    let a2438 = 1u64;
                                    nat_sub(a2437, a2438)
                                }), Adt12::C0(0u64, f99(v9, v1)?), Adt13::C1({
                                    let a2439 = v5;
                                    let a2440 = 1u64;
                                    nat_sub(a2439, a2440)
                                }), 4u64, 4u64))
                            }
                        } else {
                            let v11: u64 = {
                                let a2441 = v10;
                                let a2442 = 2u64;
                                nat_sub(a2441, a2442)
                            };
                            let v12: u64 = {
                                let a2443 = v11;
                                let a2444 = 3u64;
                                let a2445 = 0u64;
                                nat_rem(a2443, a2444, a2445)
                            };
                            let m2449 = f181(v3, v2, v1, v9, {
                                let a2446 = v11;
                                let a2447 = 3u64;
                                let a2448 = 0u64;
                                nat_quot(a2446, a2447, a2448)
                            })?;
                            match m2449 {
                                None => {
                                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                }
                                Some(v13) => {
                                    let a2450 = v12;
                                    let a2451 = 0u64;
                                    if nat_eq(a2450, a2451) {
                                        Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C0(1u64, v13), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
                                    } else {
                                        let a2452 = 2u64;
                                        let a2453 = v5;
                                        if nat_le(a2452, a2453) {
                                            let a2454 = v12;
                                            let a2455 = 1u64;
                                            if nat_eq(a2454, a2455) {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C0(1u64, v13), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                                            } else {
                                                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                                                    let a2456 = v13;
                                                    let a2457 = 1u64;
                                                    nat_add(a2456, a2457)?
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
            let a2458 = v10;
            let a2459 = 0u64;
            if nat_eq(a2458, a2459) {
                Ok(Adt9::C7(Adt11::C0(5u64), Adt12::C2(v6), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                let a2460 = v10;
                let a2461 = 1u64;
                if nat_eq(a2460, a2461) {
                    Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                } else {
                    let a2462 = v10;
                    let a2463 = 2u64;
                    if nat_eq(a2462, a2463) {
                        Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C0(0u64, f99(v9, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        let m2466 = f181(v3, v2, v1, v9, {
                            let a2464 = v10;
                            let a2465 = 3u64;
                            nat_sub(a2464, a2465)
                        })?;
                        match m2466 {
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

pub fn f79(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6, v4: Adt6) -> R<Adt9> {
    let v5: u64 = {
        let a2469 = match v3.clone() {
            Adt6::C0(h2467, _, _, _) => {
                h2467
            }
        };
        let a2470 = match v4.clone() {
            Adt6::C0(h2468, _, _, _) => {
                h2468
            }
        };
        nat_mul(a2469, a2470)?
    };
    let v6: u64 = {
        let a2471 = v5;
        let a2472 = 2u64;
        nat_add(a2471, a2472)?
    };
    let v7: u64 = {
        let a2473 = v0;
        let a2474 = v6;
        let a2475 = 0u64;
        nat_quot(a2473, a2474, a2475)
    };
    let v8: u64 = {
        let a2476 = v0;
        let a2477 = v6;
        let a2478 = 0u64;
        nat_rem(a2476, a2477, a2478)
    };
    let a2479 = v8;
    let a2480 = 0u64;
    if nat_eq(a2479, a2480) {
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2483 = {
            let a2481 = v8;
            let a2482 = 1u64;
            nat_add(a2481, a2482)?
        };
        let a2484 = v6;
        if nat_eq(a2483, a2484) {
            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f99(v7, v2)?), 4u64, 4u64))
        } else {
            let v9: u64 = {
                let a2485 = v8;
                let a2486 = 1u64;
                nat_sub(a2485, a2486)
            };
            let m2492 = f182(v3.clone(), match v1.clone() {
                Adt3::C0(_, h2487, _, _, _, _, _, _) => {
                    h2487
                }
            }, f62(v7, v2.clone(), 0u64), {
                let a2489 = v9;
                let a2490 = match v3.clone() {
                    Adt6::C0(h2488, _, _, _) => {
                        h2488
                    }
                };
                let a2491 = 0u64;
                nat_rem(a2489, a2490, a2491)
            })?;
            match m2492 {
                None => {
                    Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v10) => {
                    let m2498 = f182(v4, match v1.clone() {
                        Adt3::C0(_, _, h2493, _, _, _, _, _) => {
                            h2493
                        }
                    }, f62(v7, v2.clone(), 1u64), {
                        let a2495 = v9;
                        let a2496 = match v3 {
                            Adt6::C0(h2494, _, _, _) => {
                                h2494
                            }
                        };
                        let a2497 = 0u64;
                        nat_quot(a2495, a2496, a2497)
                    })?;
                    match m2498 {
                        None => {
                            Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v11) => {
                            Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(0u64, f61(v1, {
                                let a2501 = {
                                    let a2499 = f62(v7, v2.clone(), 2u64);
                                    let a2500 = v5;
                                    nat_mul(a2499, a2500)?
                                };
                                let a2502 = v9;
                                nat_add(a2501, a2502)?
                            }, v10, v11, f62(v7, v2, 3u64))?), Adt13::C0(0u64), 4u64, 4u64))
                        }
                    }
                }
            }
        }
    }
}

pub fn f80(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64, v6: Adt1, v7: u64) -> R<Adt9> {
    let v8: u64 = f183(v3, v5)?;
    let v9: u64 = {
        let a2503 = v0;
        let a2504 = v8;
        let a2505 = 0u64;
        nat_rem(a2503, a2504, a2505)
    };
    let v10: u64 = {
        let a2506 = v0;
        let a2507 = v8;
        let a2508 = 0u64;
        nat_quot(a2506, a2507, a2508)
    };
    let v11: Adt13 = Adt13::C1(f99(v10, v2)?);
    let v12: Adt13 = {
        let a2509 = v5;
        let a2510 = 0u64;
        if nat_eq(a2509, a2510) {
            v11.clone()
        } else {
            Adt13::C0(0u64)
        }
    };
    let a2511 = v9;
    let a2512 = 0u64;
    if nat_eq(a2511, a2512) {
        Ok(Adt9::C7(Adt11::C2(v4, 0u64, true), Adt12::C0(0u64, f99(v10, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f184(v4), 4u64))
    } else {
        let a2513 = v9;
        let a2514 = 1u64;
        if nat_eq(a2513, a2514) {
            Ok(Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
        } else {
            let a2515 = v9;
            let a2516 = if v3 {
                8u64
            } else {
                3u64
            };
            if nat_eq(a2515, a2516) {
                Ok(Adt9::C7(Adt11::C2(0u64, v5, true), Adt12::C1(0u64), Adt12::C2(0u64), v11, 4u64, f184(v5)))
            } else {
                if v3 {
                    Ok(f185(v9, v7, v6, v12))
                } else {
                    Ok(Adt9::C7({
                        let a2517 = v7;
                        let a2518 = 2u64;
                        if nat_eq(a2517, a2518) {
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

pub fn f81(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = match v1 {
        Adt3::C0(h2519, _, _, _, _, _, _, _) => {
            h2519
        }
    };
    let v5: u64 = match v2 {
        Adt3::C0(h2520, _, _, _, _, _, _, _) => {
            h2520
        }
    };
    let v6: u64 = f186(v4)?;
    let v7: u64 = {
        let a2521 = v0;
        let a2522 = v6;
        let a2523 = 0u64;
        nat_quot(a2521, a2522, a2523)
    };
    let v8: u64 = {
        let a2524 = v7;
        let a2525 = v5;
        let a2526 = 0u64;
        nat_quot(a2524, a2525, a2526)
    };
    let v9: u64 = {
        let a2527 = v7;
        let a2528 = v5;
        let a2529 = 0u64;
        nat_rem(a2527, a2528, a2529)
    };
    let v10: u64 = {
        let a2530 = v3;
        let a2531 = v4;
        let a2532 = 0u64;
        nat_rem(a2530, a2531, a2532)
    };
    let v11: u64 = {
        let a2537 = 4u64;
        let a2538 = {
            let a2535 = v8;
            let a2536 = {
                let a2533 = v4;
                let a2534 = v4;
                nat_mul(a2533, a2534)?
            };
            nat_mul(a2535, a2536)?
        };
        nat_mul(a2537, a2538)?
    };
    let v12: u64 = {
        let a2543 = 4u64;
        let a2544 = {
            let a2541 = v8;
            let a2542 = {
                let a2539 = v4;
                let a2540 = v5;
                nat_mul(a2539, a2540)?
            };
            nat_mul(a2541, a2542)?
        };
        nat_mul(a2543, a2544)?
    };
    let v13: u64 = {
        let a2548 = {
            let a2545 = v0;
            let a2546 = v6;
            let a2547 = 0u64;
            nat_rem(a2545, a2546, a2547)
        };
        let a2549 = f186(v10)?;
        nat_sub(a2548, a2549)
    };
    let a2552 = v13;
    let a2553 = {
        let a2550 = v10;
        let a2551 = 2u64;
        nat_mul(a2550, a2551)?
    };
    if nat_lt(a2552, a2553) {
        let v14: u64 = {
            let a2554 = v13;
            let a2555 = 2u64;
            let a2556 = 0u64;
            nat_quot(a2554, a2555, a2556)
        };
        let a2560 = {
            let a2557 = v13;
            let a2558 = 2u64;
            let a2559 = 0u64;
            nat_rem(a2557, a2558, a2559)
        };
        let a2561 = 0u64;
        if nat_eq(a2560, a2561) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                let a2568 = v11;
                let a2569 = {
                    let a2566 = 4u64;
                    let a2567 = {
                        let a2564 = {
                            let a2562 = v10;
                            let a2563 = v4;
                            nat_mul(a2562, a2563)?
                        };
                        let a2565 = v14;
                        nat_add(a2564, a2565)?
                    };
                    nat_mul(a2566, a2567)?
                };
                nat_add(a2568, a2569)?
            }), Adt12::C3({
                let a2576 = v12;
                let a2577 = {
                    let a2574 = 4u64;
                    let a2575 = {
                        let a2572 = {
                            let a2570 = v14;
                            let a2571 = v5;
                            nat_mul(a2570, a2571)?
                        };
                        let a2573 = v9;
                        nat_add(a2572, a2573)?
                    };
                    nat_mul(a2574, a2575)?
                };
                nat_add(a2576, a2577)?
            }), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(0u64), {
                let a2578 = v14;
                let a2579 = 0u64;
                if nat_eq(a2578, a2579) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
        }
    } else {
        let a2582 = v13;
        let a2583 = {
            let a2580 = v10;
            let a2581 = 2u64;
            nat_mul(a2580, a2581)?
        };
        if nat_eq(a2582, a2583) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(1u64, {
                let a2590 = v12;
                let a2591 = {
                    let a2588 = 4u64;
                    let a2589 = {
                        let a2586 = {
                            let a2584 = v10;
                            let a2585 = v5;
                            nat_mul(a2584, a2585)?
                        };
                        let a2587 = v9;
                        nat_add(a2586, a2587)?
                    };
                    nat_mul(a2588, a2589)?
                };
                nat_add(a2590, a2591)?
            }), {
                let a2592 = v10;
                let a2593 = 0u64;
                if nat_eq(a2592, a2593) {
                    Adt12::C2(0u64)
                } else {
                    Adt12::C1(0u64)
                }
            }, Adt13::C0(0u64), 4u64, 4u64))
        } else {
            let a2598 = v13;
            let a2599 = {
                let a2596 = {
                    let a2594 = v10;
                    let a2595 = 2u64;
                    nat_mul(a2594, a2595)?
                };
                let a2597 = 1u64;
                nat_add(a2596, a2597)?
            };
            if nat_eq(a2598, a2599) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C1(0u64), Adt12::C0(0u64, {
                    let a2606 = v11;
                    let a2607 = {
                        let a2604 = 4u64;
                        let a2605 = {
                            let a2602 = {
                                let a2600 = v10;
                                let a2601 = v4;
                                nat_mul(a2600, a2601)?
                            };
                            let a2603 = v10;
                            nat_add(a2602, a2603)?
                        };
                        nat_mul(a2604, a2605)?
                    };
                    nat_add(a2606, a2607)?
                }), Adt13::C1({
                    let a2614 = v12;
                    let a2615 = {
                        let a2612 = 4u64;
                        let a2613 = {
                            let a2610 = {
                                let a2608 = v10;
                                let a2609 = v5;
                                nat_mul(a2608, a2609)?
                            };
                            let a2611 = v9;
                            nat_add(a2610, a2611)?
                        };
                        nat_mul(a2612, a2613)?
                    };
                    nat_add(a2614, a2615)?
                }), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C2({
                    let a2618 = {
                        let a2616 = v10;
                        let a2617 = 1u64;
                        nat_add(a2616, a2617)?
                    };
                    let a2619 = v4;
                    if nat_eq(a2618, a2619) {
                        0u64
                    } else {
                        let a2620 = v10;
                        let a2621 = 1u64;
                        nat_add(a2620, a2621)?
                    }
                }), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f82(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a2622 = v0;
    let a2623 = v1;
    if nat_lt(a2622, a2623) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f99(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a2624 = v0;
            let a2625 = v1;
            nat_sub(a2624, a2625)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f99(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f83(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a2632 = {
            let a2630 = 3u64;
            let a2631 = {
                let a2628 = match v4.clone() {
                    Adt6::C0(h2626, _, _, _) => {
                        h2626
                    }
                };
                let a2629 = match v5.clone() {
                    Adt6::C0(h2627, _, _, _) => {
                        h2627
                    }
                };
                nat_mul(a2628, a2629)?
            };
            nat_mul(a2630, a2631)?
        };
        let a2633 = 2u64;
        nat_add(a2632, a2633)?
    };
    let v8: u64 = {
        let a2634 = v0;
        let a2635 = v7;
        let a2636 = 0u64;
        nat_quot(a2634, a2635, a2636)
    };
    let v9: u64 = {
        let a2637 = v0;
        let a2638 = v7;
        let a2639 = 0u64;
        nat_rem(a2637, a2638, a2639)
    };
    let a2640 = v9;
    let a2641 = 0u64;
    if nat_eq(a2640, a2641) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a2644 = {
            let a2642 = v9;
            let a2643 = 1u64;
            nat_add(a2642, a2643)?
        };
        let a2645 = v7;
        if nat_eq(a2644, a2645) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f99(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a2646 = v9;
                let a2647 = 1u64;
                nat_sub(a2646, a2647)
            };
            let v11: u64 = {
                let a2648 = v10;
                let a2649 = 3u64;
                let a2650 = 0u64;
                nat_quot(a2648, a2649, a2650)
            };
            let v12: u64 = {
                let a2652 = v11;
                let a2653 = match v4.clone() {
                    Adt6::C0(h2651, _, _, _) => {
                        h2651
                    }
                };
                let a2654 = 0u64;
                nat_rem(a2652, a2653, a2654)
            };
            let v13: u64 = {
                let a2656 = v11;
                let a2657 = match v4.clone() {
                    Adt6::C0(h2655, _, _, _) => {
                        h2655
                    }
                };
                let a2658 = 0u64;
                nat_quot(a2656, a2657, a2658)
            };
            let v14: u64 = {
                let a2659 = v10;
                let a2660 = 3u64;
                let a2661 = 0u64;
                nat_rem(a2659, a2660, a2661)
            };
            let v15: u64 = f62(v8, v3.clone(), 2u64);
            let m2663 = f64(v4, match v2.clone() {
                Adt3::C0(h2662, _, _, _, _, _, _, _) => {
                    h2662
                }
            }, f62(v8, v3.clone(), 0u64), v12)?;
            match m2663 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m2665 = f64(v5, match v2.clone() {
                        Adt3::C0(_, h2664, _, _, _, _, _, _) => {
                            h2664
                        }
                    }, f62(v8, v3.clone(), 1u64), v13)?;
                    match m2665 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a2666 = v14;
                            let a2667 = 0u64;
                            if nat_eq(a2666, a2667) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f61(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f184(v6), 4u64))
                            } else {
                                let a2668 = v14;
                                let a2669 = 1u64;
                                if nat_eq(a2668, a2669) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f61(v2, v16, v17, v15, f62(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f84(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f187(match v1.clone() {
        Adt3::C0(h2670, _, _, _, _, _, _, _) => {
            h2670
        }
    })?;
    let v5: u64 = {
        let a2671 = v0;
        let a2672 = v4;
        let a2673 = 0u64;
        nat_quot(a2671, a2672, a2673)
    };
    let v6: u64 = {
        let a2674 = v0;
        let a2675 = v4;
        let a2676 = 0u64;
        nat_rem(a2674, a2675, a2676)
    };
    let a2677 = v6;
    let a2678 = 0u64;
    if nat_eq(a2677, a2678) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2679 = v6;
        let a2680 = 1u64;
        if nat_eq(a2679, a2680) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a2683 = {
                let a2681 = v6;
                let a2682 = 1u64;
                nat_add(a2681, a2682)?
            };
            let a2684 = v4;
            if nat_eq(a2683, a2684) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f99(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a2685 = v6;
                    let a2686 = 2u64;
                    nat_sub(a2685, a2686)
                };
                let v8: u64 = {
                    let a2687 = v7;
                    let a2688 = 3u64;
                    let a2689 = 0u64;
                    nat_quot(a2687, a2688, a2689)
                };
                let v9: u64 = {
                    let a2690 = v7;
                    let a2691 = 3u64;
                    let a2692 = 0u64;
                    nat_rem(a2690, a2691, a2692)
                };
                let v10: u64 = f99({
                    let a2696 = {
                        let a2694 = v5;
                        let a2695 = match v1.clone() {
                            Adt3::C0(h2693, _, _, _, _, _, _, _) => {
                                h2693
                            }
                        };
                        nat_mul(a2694, a2695)?
                    };
                    let a2697 = v8;
                    nat_add(a2696, a2697)?
                }, v1)?;
                let a2698 = v9;
                let a2699 = 0u64;
                if nat_eq(a2698, a2699) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a2700 = v9;
                    let a2701 = 1u64;
                    if nat_eq(a2700, a2701) {
                        let a2702 = v3;
                        let a2703 = 2u64;
                        if nat_eq(a2702, a2703) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a2704 = v3;
                        let a2705 = 2u64;
                        if if nat_eq(a2704, a2705) {
                            true
                        } else {
                            let a2706 = v3;
                            let a2707 = 3u64;
                            nat_eq(a2706, a2707)
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

pub fn f85(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2709 = 2u64;
        let a2710 = match v2.clone() {
            Adt3::C0(_, h2708, _, _, _, _, _, _) => {
                h2708
            }
        };
        nat_mul(a2709, a2710)?
    };
    let a2711 = v1;
    let a2712 = v6;
    if nat_lt(a2711, a2712) {
        let a2716 = {
            let a2713 = v1;
            let a2714 = 2u64;
            let a2715 = 0u64;
            nat_rem(a2713, a2714, a2715)
        };
        let a2717 = 0u64;
        if nat_eq(a2716, a2717) {
            Ok(Adt9::C4(19u64, 4u64, f99({
                let a2718 = v1;
                let a2719 = 2u64;
                let a2720 = 0u64;
                nat_quot(a2718, a2719, a2720)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a2721 = v5;
            let a2722 = 2147483648u64;
            if if nat_lt(a2721, a2722) {
                let a2724 = v5;
                let a2725 = match v3 {
                    Adt3::C0(_, h2723, _, _, _, _, _, _) => {
                        h2723
                    }
                };
                nat_lt(a2724, a2725)
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
            let a2728 = match v3.clone() {
                Adt3::C0(h2726, _, _, _, _, _, _, _) => {
                    h2726
                }
            };
            let a2729 = match v3.clone() {
                Adt3::C0(_, h2727, _, _, _, _, _, _) => {
                    h2727
                }
            };
            nat_mul(a2728, a2729)?
        };
        let a2732 = {
            let a2730 = v1;
            let a2731 = v6;
            nat_sub(a2730, a2731)
        };
        let a2733 = v7;
        if nat_lt(a2732, a2733) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f99({
                let a2734 = v1;
                let a2735 = v6;
                nat_sub(a2734, a2735)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a2738 = {
                    let a2736 = v1;
                    let a2737 = v6;
                    nat_sub(a2736, a2737)
                };
                let a2739 = v7;
                nat_sub(a2738, a2739)
            };
            let v9: u64 = {
                let a2741 = 1u64;
                let a2742 = f188(v0, match v2.clone() {
                    Adt3::C0(h2740, _, _, _, _, _, _, _) => {
                        h2740
                    }
                })?;
                nat_add(a2741, a2742)?
            };
            let v10: u64 = {
                let a2743 = v8;
                let a2744 = v9;
                let a2745 = 0u64;
                nat_quot(a2743, a2744, a2745)
            };
            let v11: u64 = {
                let a2746 = v8;
                let a2747 = v9;
                let a2748 = 0u64;
                nat_rem(a2746, a2747, a2748)
            };
            let a2749 = v11;
            let a2750 = 0u64;
            if nat_eq(a2749, a2750) {
                Ok(Adt9::C4(19u64, 4u64, f99(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f189(v0) {
                    let a2754 = {
                        let a2751 = v11;
                        let a2752 = 2u64;
                        let a2753 = 0u64;
                        nat_rem(a2751, a2752, a2753)
                    };
                    let a2755 = 1u64;
                    if nat_eq(a2754, a2755) {
                        Ok(Adt9::C4(6u64, 0u64, f99({
                            let a2764 = {
                                let a2757 = v10;
                                let a2758 = match v2.clone() {
                                    Adt3::C0(h2756, _, _, _, _, _, _, _) => {
                                        h2756
                                    }
                                };
                                nat_mul(a2757, a2758)?
                            };
                            let a2765 = {
                                let a2761 = {
                                    let a2759 = v11;
                                    let a2760 = 1u64;
                                    nat_sub(a2759, a2760)
                                };
                                let a2762 = 2u64;
                                let a2763 = 0u64;
                                nat_quot(a2761, a2762, a2763)
                            };
                            nat_add(a2764, a2765)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f99({
                            let a2774 = {
                                let a2767 = v5;
                                let a2768 = match v2.clone() {
                                    Adt3::C0(h2766, _, _, _, _, _, _, _) => {
                                        h2766
                                    }
                                };
                                nat_mul(a2767, a2768)?
                            };
                            let a2775 = {
                                let a2771 = {
                                    let a2769 = v11;
                                    let a2770 = 1u64;
                                    nat_sub(a2769, a2770)
                                };
                                let a2772 = 2u64;
                                let a2773 = 0u64;
                                nat_quot(a2771, a2772, a2773)
                            };
                            nat_add(a2774, a2775)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f99({
                            let a2784 = {
                                let a2777 = v5;
                                let a2778 = match v2 {
                                    Adt3::C0(h2776, _, _, _, _, _, _, _) => {
                                        h2776
                                    }
                                };
                                nat_mul(a2777, a2778)?
                            };
                            let a2785 = {
                                let a2781 = {
                                    let a2779 = v11;
                                    let a2780 = 1u64;
                                    nat_sub(a2779, a2780)
                                };
                                let a2782 = 2u64;
                                let a2783 = 0u64;
                                nat_quot(a2781, a2782, a2783)
                            };
                            nat_add(a2784, a2785)?
                        }, v3)?))
                    }
                } else {
                    f190(v0, v10, {
                        let a2786 = v11;
                        let a2787 = 1u64;
                        nat_sub(a2786, a2787)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f86(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a2788 = v1;
    let a2789 = f191(v2.clone(), v4.clone())?;
    if nat_lt(a2788, a2789) {
        let a2792 = v1;
        let a2793 = {
            let a2790 = 2u64;
            let a2791 = f192(v4.clone())?;
            nat_mul(a2790, a2791)?
        };
        if nat_lt(a2792, a2793) {
            let a2797 = {
                let a2794 = v1;
                let a2795 = 2u64;
                let a2796 = 0u64;
                nat_rem(a2794, a2795, a2796)
            };
            let a2798 = 0u64;
            if nat_eq(a2797, a2798) {
                f193({
                    let a2799 = v1;
                    let a2800 = 2u64;
                    let a2801 = 0u64;
                    nat_quot(a2799, a2800, a2801)
                }, v4)
            } else {
                if f194(match v2 {
                    Adt3::C0(_, h2802, _, _, _, _, _, _) => {
                        h2802
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a2805 = v1;
                let a2806 = {
                    let a2803 = 2u64;
                    let a2804 = f192(v4.clone())?;
                    nat_mul(a2803, a2804)?
                };
                nat_sub(a2805, a2806)
            };
            let v7: u64 = {
                let a2810 = v6;
                let a2811 = {
                    let a2808 = 1u64;
                    let a2809 = match v2.clone() {
                        Adt3::C0(h2807, _, _, _, _, _, _, _) => {
                            h2807
                        }
                    };
                    nat_add(a2808, a2809)?
                };
                let a2812 = 0u64;
                nat_quot(a2810, a2811, a2812)
            };
            let v8: u64 = {
                let a2816 = v6;
                let a2817 = {
                    let a2814 = 1u64;
                    let a2815 = match v2.clone() {
                        Adt3::C0(h2813, _, _, _, _, _, _, _) => {
                            h2813
                        }
                    };
                    nat_add(a2814, a2815)?
                };
                let a2818 = 0u64;
                nat_rem(a2816, a2817, a2818)
            };
            let a2819 = v8;
            let a2820 = 0u64;
            if nat_eq(a2819, a2820) {
                f193(v7, v4)
            } else {
                f195(v0, v7, {
                    let a2821 = v8;
                    let a2822 = 1u64;
                    nat_sub(a2821, a2822)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f87(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a2825 = match v4.clone() {
            Adt3::C0(h2823, _, _, _, _, _, _, _) => {
                h2823
            }
        };
        let a2826 = match v4.clone() {
            Adt3::C0(_, h2824, _, _, _, _, _, _) => {
                h2824
            }
        };
        nat_mul(a2825, a2826)?
    };
    let v7: u64 = {
        let a2827 = 2u64;
        let a2828 = v6;
        nat_mul(a2827, a2828)?
    };
    let v8: u64 = f196(match v3.clone() {
        Adt3::C0(_, h2829, _, _, _, _, _, _) => {
            h2829
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h2830, _, _, _, _, _) => {
            h2830
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h2831, _, _, _, _) => {
            h2831
        }
    });
    let a2832 = v0;
    let a2833 = v7;
    if nat_lt(a2832, a2833) {
        let a2837 = {
            let a2834 = v0;
            let a2835 = 2u64;
            let a2836 = 0u64;
            nat_rem(a2834, a2835, a2836)
        };
        let a2838 = 0u64;
        if nat_eq(a2837, a2838) {
            Ok(Adt9::C4(19u64, 5u64, f99({
                let a2839 = v0;
                let a2840 = 2u64;
                let a2841 = 0u64;
                nat_quot(a2839, a2840, a2841)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f197(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a2842 = v0;
            let a2843 = v7;
            nat_sub(a2842, a2843)
        };
        let v10: u64 = {
            let a2847 = v9;
            let a2848 = {
                let a2845 = 1u64;
                let a2846 = match v1.clone() {
                    Adt3::C0(h2844, _, _, _, _, _, _, _) => {
                        h2844
                    }
                };
                nat_add(a2845, a2846)?
            };
            let a2849 = 0u64;
            nat_quot(a2847, a2848, a2849)
        };
        let v11: u64 = {
            let a2853 = v9;
            let a2854 = {
                let a2851 = 1u64;
                let a2852 = match v1.clone() {
                    Adt3::C0(h2850, _, _, _, _, _, _, _) => {
                        h2850
                    }
                };
                nat_add(a2851, a2852)?
            };
            let a2855 = 0u64;
            nat_rem(a2853, a2854, a2855)
        };
        let a2856 = v11;
        let a2857 = 0u64;
        if nat_eq(a2856, a2857) {
            Ok(Adt9::C4(19u64, 5u64, f99({
                let a2858 = v10;
                let a2859 = v6;
                let a2860 = 0u64;
                nat_rem(a2858, a2859, a2860)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f197(v5, v8) {
                let v12: u64 = {
                    let a2866 = {
                        let a2862 = v10;
                        let a2863 = match v1.clone() {
                            Adt3::C0(h2861, _, _, _, _, _, _, _) => {
                                h2861
                            }
                        };
                        nat_mul(a2862, a2863)?
                    };
                    let a2867 = {
                        let a2864 = v11;
                        let a2865 = 1u64;
                        nat_sub(a2864, a2865)
                    };
                    nat_add(a2866, a2867)?
                };
                let a2868 = v5;
                let a2869 = 4294967295u64;
                if nat_eq(a2868, a2869) {
                    Ok(Adt9::C1(false, f99(v12, v1)?, f99(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f99(v12, v1)?, 4u64, f99({
                        let a2875 = {
                            let a2871 = v5;
                            let a2872 = match v3.clone() {
                                Adt3::C0(h2870, _, _, _, _, _, _, _) => {
                                    h2870
                                }
                            };
                            nat_mul(a2871, a2872)?
                        };
                        let a2876 = {
                            let a2873 = v11;
                            let a2874 = 1u64;
                            nat_sub(a2873, a2874)
                        };
                        nat_add(a2875, a2876)?
                    }, v3)?, 3u64, f99(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f88(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f198(v3.clone())?;
    let v6: u64 = {
        let a2877 = v0;
        let a2878 = v5;
        let a2879 = 0u64;
        nat_quot(a2877, a2878, a2879)
    };
    let v7: u64 = {
        let a2880 = v0;
        let a2881 = v5;
        let a2882 = 0u64;
        nat_rem(a2880, a2881, a2882)
    };
    let v8: u64 = f199(v3.clone())?;
    let a2883 = v7;
    let a2884 = 0u64;
    if nat_eq(a2883, a2884) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h2885, _) => {
                h2885
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2889 = v5;
        let a2890 = {
            let a2887 = v7;
            let a2888 = if match v3.clone() {
                Adt7::C0(_, _, _, h2886, _) => {
                    h2886
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a2887, a2888)?
        };
        if nat_le(a2889, a2890) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h2891, _) => {
                    h2891
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f99(v6, v2)?))
            } else {
                let a2894 = {
                    let a2892 = v7;
                    let a2893 = 2u64;
                    nat_add(a2892, a2893)?
                };
                let a2895 = v5;
                if nat_eq(a2894, a2895) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h2896, _, _) => {
                            h2896
                        }
                    } {
                        let a2903 = {
                            let m2899 = match v3.clone() {
                                Adt7::C0(h2898, _, _, _, _) => {
                                    h2898
                                }
                            };
                            match m2899 {
                                Adt6::C0(h2897, _, _, _) => {
                                    h2897
                                }
                            }
                        };
                        let a2904 = {
                            let m2902 = match v3 {
                                Adt7::C0(_, h2901, _, _, _) => {
                                    h2901
                                }
                            };
                            match m2902 {
                                Adt6::C0(h2900, _, _, _) => {
                                    h2900
                                }
                            }
                        };
                        nat_mul(a2903, a2904)?
                    } else {
                        f200(match v3 {
                            Adt7::C0(h2905, _, _, _, _) => {
                                h2905
                            }
                        }, match v1 {
                            Adt3::C0(h2906, _, _, _, _, _, _, _) => {
                                h2906
                            }
                        }, f62(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a2907 = v9;
                        let a2908 = 0u64;
                        if nat_eq(a2907, a2908) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f99(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a2911 = {
                    let a2909 = v7;
                    let a2910 = 1u64;
                    nat_sub(a2909, a2910)
                };
                let a2912 = v8;
                let a2913 = 0u64;
                nat_rem(a2911, a2912, a2913)
            };
            let m2919 = f181(v3.clone(), v1, v2, v6, {
                let a2916 = {
                    let a2914 = v7;
                    let a2915 = 1u64;
                    nat_sub(a2914, a2915)
                };
                let a2917 = v8;
                let a2918 = 0u64;
                nat_quot(a2916, a2917, a2918)
            })?;
            match m2919 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a2921 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h2920) => {
                            h2920
                        }
                    };
                    let a2922 = 1u64;
                    if if nat_eq(a2921, a2922) {
                        let a2923 = v10;
                        let a2924 = 0u64;
                        nat_eq(a2923, a2924)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2925, _) => {
                                h2925
                            }
                        } {
                            let a2928 = {
                                let a2926 = v10;
                                let a2927 = 1u64;
                                nat_add(a2926, a2927)?
                            };
                            let a2929 = v8;
                            if nat_eq(a2928, a2929) {
                                let a2930 = v4;
                                let a2931 = 2u64;
                                if nat_eq(a2930, a2931) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2933 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2932) => {
                                                h2932
                                            }
                                        };
                                        let a2934 = 1u64;
                                        if nat_eq(a2933, a2934) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2936 = match v3 {
                                            Adt7::C0(_, _, _, _, h2935) => {
                                                h2935
                                            }
                                        };
                                        let a2937 = 1u64;
                                        if nat_eq(a2936, a2937) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2939 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2938) => {
                                            h2938
                                        }
                                    };
                                    let a2940 = 1u64;
                                    if nat_eq(a2939, a2940) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2942 = match v3 {
                                        Adt7::C0(_, _, _, _, h2941) => {
                                            h2941
                                        }
                                    };
                                    let a2943 = 1u64;
                                    if nat_eq(a2942, a2943) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2945 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2944) => {
                                        h2944
                                    }
                                };
                                let a2946 = 1u64;
                                if nat_eq(a2945, a2946) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2948 = match v3 {
                                    Adt7::C0(_, _, _, _, h2947) => {
                                        h2947
                                    }
                                };
                                let a2949 = 1u64;
                                if nat_eq(a2948, a2949) {
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

pub fn f89(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f201(match v3.clone() {
        Adt3::C0(h2950, _, _, _, _, _, _, _) => {
            h2950
        }
    })?;
    let v5: u64 = {
        let a2951 = v0;
        let a2952 = v4;
        let a2953 = 0u64;
        nat_rem(a2951, a2952, a2953)
    };
    let v6: u64 = {
        let a2954 = v0;
        let a2955 = v4;
        let a2956 = 0u64;
        nat_quot(a2954, a2955, a2956)
    };
    let a2957 = v5;
    let a2958 = 0u64;
    if nat_eq(a2957, a2958) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2961 = {
            let a2959 = v5;
            let a2960 = 1u64;
            nat_add(a2959, a2960)?
        };
        let a2962 = v4;
        if nat_eq(a2961, a2962) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f99(v6, v2)?))
        } else {
            let a2966 = {
                let a2963 = v5;
                let a2964 = 2u64;
                let a2965 = 0u64;
                nat_rem(a2963, a2964, a2965)
            };
            let a2967 = 0u64;
            if nat_eq(a2966, a2967) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2970 = {
                        let a2968 = v5;
                        let a2969 = 1u64;
                        nat_sub(a2968, a2969)
                    };
                    let a2971 = 2u64;
                    let a2972 = 0u64;
                    nat_quot(a2970, a2971, a2972)
                };
                let v8: u64 = f62(v6, v2.clone(), 0u64);
                let v9: u64 = f62(v6, v2.clone(), 1u64);
                let v10: u64 = f62(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f61(v1, {
                    let a2973 = v9;
                    let a2974 = v7;
                    nat_add(a2973, a2974)?
                }, v8, v10, 0u64)?, 4u64, f61(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f90(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f202(v3.clone())?;
    let v5: u64 = {
        let a2975 = v0;
        let a2976 = v4;
        let a2977 = 0u64;
        nat_rem(a2975, a2976, a2977)
    };
    let v6: u64 = {
        let a2978 = v0;
        let a2979 = v4;
        let a2980 = 0u64;
        nat_quot(a2978, a2979, a2980)
    };
    let a2981 = v5;
    let a2982 = 0u64;
    if nat_eq(a2981, a2982) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2985 = {
            let a2983 = v5;
            let a2984 = 1u64;
            nat_add(a2983, a2984)?
        };
        let a2986 = v4;
        if nat_eq(a2985, a2986) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f99(v6, v2)?))
        } else {
            let a2990 = {
                let a2987 = v5;
                let a2988 = 2u64;
                let a2989 = 0u64;
                nat_rem(a2987, a2988, a2989)
            };
            let a2991 = 0u64;
            if nat_eq(a2990, a2991) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2993 = f62(v6, v2.clone(), 0u64);
                    let a2994 = match v3.clone() {
                        Adt6::C0(_, _, h2992, _) => {
                            h2992
                        }
                    };
                    nat_add(a2993, a2994)?
                };
                let v8: u64 = {
                    let a3002 = {
                        let a2999 = {
                            let a2996 = v7;
                            let a2997 = match v3.clone() {
                                Adt6::C0(_, h2995, _, _) => {
                                    h2995
                                }
                            };
                            nat_add(a2996, a2997)?
                        };
                        let a3000 = match v3.clone() {
                            Adt6::C0(h2998, _, _, _) => {
                                h2998
                            }
                        };
                        nat_sub(a2999, a3000)
                    };
                    let a3003 = match v3.clone() {
                        Adt6::C0(_, h3001, _, _) => {
                            h3001
                        }
                    };
                    let a3004 = 0u64;
                    nat_quot(a3002, a3003, a3004)
                };
                let v9: u64 = {
                    let a3010 = v8;
                    let a3011 = {
                        let a3007 = {
                            let a3005 = v5;
                            let a3006 = 1u64;
                            nat_sub(a3005, a3006)
                        };
                        let a3008 = 2u64;
                        let a3009 = 0u64;
                        nat_quot(a3007, a3008, a3009)
                    };
                    nat_add(a3010, a3011)?
                };
                let v10: u64 = {
                    let a3013 = v9;
                    let a3014 = match v3.clone() {
                        Adt6::C0(_, h3012, _, _) => {
                            h3012
                        }
                    };
                    nat_mul(a3013, a3014)?
                };
                let v11: u64 = {
                    let a3015 = v7;
                    let a3016 = v10;
                    nat_sub(a3015, a3016)
                };
                let a3018 = v9;
                let a3019 = match v1.clone() {
                    Adt3::C0(_, h3017, _, _, _, _, _, _) => {
                        h3017
                    }
                };
                if if if nat_lt(a3018, a3019) {
                    let a3020 = v10;
                    let a3021 = v7;
                    nat_le(a3020, a3021)
                } else {
                    false
                } {
                    let a3023 = v11;
                    let a3024 = match v3.clone() {
                        Adt6::C0(h3022, _, _, _) => {
                            h3022
                        }
                    };
                    nat_lt(a3023, a3024)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f61(v1, {
                        let a3028 = {
                            let a3026 = f62(v6, v2, 1u64);
                            let a3027 = match v3 {
                                Adt6::C0(h3025, _, _, _) => {
                                    h3025
                                }
                            };
                            nat_mul(a3026, a3027)?
                        };
                        let a3029 = v11;
                        nat_add(a3028, a3029)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f91(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f62(v0, v2.clone(), 0u64);
    let v5: u64 = f62(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a3031 = v5;
        let a3032 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3030, _) => {
                h3030
            }
        };
        let a3033 = 0u64;
        nat_rem(a3031, a3032, a3033)
    };
    let v7: u64 = {
        let a3035 = v5;
        let a3036 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h3034, _) => {
                h3034
            }
        };
        let a3037 = 0u64;
        nat_quot(a3035, a3036, a3037)
    };
    let v8: Option<u64> = f64(match v3.clone() {
        Adt5::C0(h3038, _, _, _, _, _, _, _) => {
            h3038
        }
    }, match v1.clone() {
        Adt3::C0(h3039, _, _, _, _, _, _, _) => {
            h3039
        }
    }, f62(v0, v2.clone(), 1u64), {
        let a3043 = v4;
        let a3044 = {
            let m3042 = match v3.clone() {
                Adt5::C0(h3041, _, _, _, _, _, _, _) => {
                    h3041
                }
            };
            match m3042 {
                Adt6::C0(h3040, _, _, _) => {
                    h3040
                }
            }
        };
        let a3045 = 0u64;
        nat_rem(a3043, a3044, a3045)
    })?;
    let v9: u64 = {
        let a3049 = v4;
        let a3050 = {
            let m3048 = match v3.clone() {
                Adt5::C0(h3047, _, _, _, _, _, _, _) => {
                    h3047
                }
            };
            match m3048 {
                Adt6::C0(h3046, _, _, _) => {
                    h3046
                }
            }
        };
        let a3051 = 0u64;
        nat_quot(a3049, a3050, a3051)
    };
    let v10: Option<u64> = f64(match v3.clone() {
        Adt5::C0(_, h3052, _, _, _, _, _, _) => {
            h3052
        }
    }, match v1.clone() {
        Adt3::C0(_, h3053, _, _, _, _, _, _) => {
            h3053
        }
    }, f62(v0, v2.clone(), 2u64), {
        let a3057 = v9;
        let a3058 = {
            let m3056 = match v3.clone() {
                Adt5::C0(_, h3055, _, _, _, _, _, _) => {
                    h3055
                }
            };
            match m3056 {
                Adt6::C0(h3054, _, _, _) => {
                    h3054
                }
            }
        };
        let a3059 = 0u64;
        nat_rem(a3057, a3058, a3059)
    })?;
    let v11: u64 = {
        let a3063 = v9;
        let a3064 = {
            let m3062 = match v3.clone() {
                Adt5::C0(_, h3061, _, _, _, _, _, _) => {
                    h3061
                }
            };
            match m3062 {
                Adt6::C0(h3060, _, _, _) => {
                    h3060
                }
            }
        };
        let a3065 = 0u64;
        nat_quot(a3063, a3064, a3065)
    };
    let v12: Option<u64> = f64(match v3.clone() {
        Adt5::C0(_, _, h3066, _, _, _, _, _) => {
            h3066
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3067, _, _, _, _, _) => {
            h3067
        }
    }, v6, {
        let a3071 = v11;
        let a3072 = {
            let m3070 = match v3.clone() {
                Adt5::C0(_, _, h3069, _, _, _, _, _) => {
                    h3069
                }
            };
            match m3070 {
                Adt6::C0(h3068, _, _, _) => {
                    h3068
                }
            }
        };
        let a3073 = 0u64;
        nat_rem(a3071, a3072, a3073)
    })?;
    let v13: u64 = {
        let a3077 = v11;
        let a3078 = {
            let m3076 = match v3.clone() {
                Adt5::C0(_, _, h3075, _, _, _, _, _) => {
                    h3075
                }
            };
            match m3076 {
                Adt6::C0(h3074, _, _, _) => {
                    h3074
                }
            }
        };
        let a3079 = 0u64;
        nat_quot(a3077, a3078, a3079)
    };
    let v17: Adt12 = {
        let m3080 = v8;
        match m3080 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m3081 = v10;
                match m3081 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m3082 = v12;
                        match m3082 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a3091 = {
                                        let a3086 = f61(v1, v14, v15, v16, 0u64)?;
                                        let a3087 = {
                                            let a3084 = v7;
                                            let a3085 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h3083, _, _, _) => {
                                                    h3083
                                                }
                                            };
                                            nat_mul(a3084, a3085)?
                                        };
                                        nat_add(a3086, a3087)?
                                    };
                                    let a3092 = {
                                        let a3089 = v13;
                                        let a3090 = match v3.clone() {
                                            Adt5::C0(_, _, _, h3088, _, _, _, _) => {
                                                h3088
                                            }
                                        };
                                        nat_mul(a3089, a3090)?
                                    };
                                    nat_add(a3091, a3092)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h3093, _, _) => {
            h3093
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h3094) => {
            h3094
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f99(v0, v2)?), {
        let a3096 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h3095, _, _) => {
                h3095
            }
        };
        let a3097 = 0u64;
        if nat_eq(a3096, a3097) {
            4u64
        } else {
            2u64
        }
    }, {
        let a3099 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h3098) => {
                h3098
            }
        };
        let a3100 = 0u64;
        if nat_eq(a3099, a3100) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f92(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m3113 = f203(v0, v1, match v2.clone() {
        Adt3::C0(h3101, _, _, _, _, _, _, _) => {
            h3101
        }
    }, match v2.clone() {
        Adt3::C0(_, h3102, _, _, _, _, _, _) => {
            h3102
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h3103, _, _, _, _, _) => {
            h3103
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h3104, _, _, _, _) => {
            h3104
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h3105, _, _, _) => {
            h3105
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h3106, _, _) => {
            h3106
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h3107, _) => {
            h3107
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h3108) => {
            h3108
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h3109, _, _, _) => {
            h3109
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h3110, _, _) => {
            h3110
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h3111, _) => {
            h3111
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h3112) => {
            h3112
        }
    }, v4)?;
    match m3113 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f21(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f93(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a3116 = v6;
        let a3117 = {
            let a3114 = 3u64;
            let a3115 = v5;
            nat_add(a3114, a3115)?
        };
        nat_sub(a3116, a3117)
    };
    let a3119 = v0;
    let a3120 = f54(v3, match v1.clone() {
        Adt3::C0(h3118, _, _, _, _, _, _, _) => {
            h3118
        }
    }, v5, v8)?;
    if nat_lt(a3119, a3120) {
        let v9: u64 = f108(match v1.clone() {
            Adt3::C0(h3121, _, _, _, _, _, _, _) => {
                h3121
            }
        }, v5, v8)?;
        let v10: u64 = f109(v3, match v1.clone() {
            Adt3::C0(h3122, _, _, _, _, _, _, _) => {
                h3122
            }
        }, v5)?;
        let a3123 = v0;
        let a3124 = v9;
        if nat_lt(a3123, a3124) {
            f204(v0, v2, f48(v3), {
                let a3125 = 3u64;
                let a3126 = v5;
                nat_add(a3125, a3126)?
            }, v7)
        } else {
            let a3129 = v0;
            let a3130 = {
                let a3127 = v9;
                let a3128 = v10;
                nat_add(a3127, a3128)?
            };
            if nat_lt(a3129, a3130) {
                let v11: u64 = {
                    let a3131 = v0;
                    let a3132 = v9;
                    nat_sub(a3131, a3132)
                };
                let v12: u64 = f205(v3, match v1.clone() {
                    Adt3::C0(h3133, _, _, _, _, _, _, _) => {
                        h3133
                    }
                })?;
                f206({
                    let a3134 = v11;
                    let a3135 = v12;
                    let a3136 = 0u64;
                    nat_rem(a3134, a3135, a3136)
                }, {
                    let a3137 = v11;
                    let a3138 = v12;
                    let a3139 = 0u64;
                    nat_quot(a3137, a3138, a3139)
                }, v1, v3, v4, v7)
            } else {
                f207({
                    let a3142 = v0;
                    let a3143 = {
                        let a3140 = v9;
                        let a3141 = v10;
                        nat_add(a3140, a3141)?
                    };
                    nat_sub(a3142, a3143)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f94(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a3144 = v0;
    let a3145 = f41(v1)?;
    if nat_lt(a3144, a3145) {
        let a3146 = v0;
        let a3147 = 4u64;
        if nat_lt(a3146, a3147) {
            let a3148 = v0;
            let a3149 = 0u64;
            if nat_eq(a3148, a3149) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a3150 = v0;
                let a3151 = 1u64;
                if nat_eq(a3150, a3151) {
                    let a3152 = v5;
                    let a3153 = 0u64;
                    if nat_eq(a3152, a3153) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a3154 = v0;
                    let a3155 = 2u64;
                    if nat_eq(a3154, a3155) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a3156 = v0;
            let a3157 = 4u64;
            if if nat_eq(a3156, a3157) {
                let a3158 = f208(v5)?;
                let a3159 = v1;
                !nat_eq(a3158, a3159)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a3162 = {
                        let a3160 = v0;
                        let a3161 = 4u64;
                        nat_sub(a3160, a3161)
                    };
                    let a3163 = 3u64;
                    let a3164 = 0u64;
                    nat_quot(a3162, a3163, a3164)
                };
                let v7: u64 = {
                    let a3167 = {
                        let a3165 = v0;
                        let a3166 = 4u64;
                        nat_sub(a3165, a3166)
                    };
                    let a3168 = 3u64;
                    let a3169 = 0u64;
                    nat_rem(a3167, a3168, a3169)
                };
                let a3170 = v7;
                let a3171 = 0u64;
                if nat_eq(a3170, a3171) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a3172 = v7;
                    let a3173 = 1u64;
                    if nat_eq(a3172, a3173) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f40(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f95(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a3174 = v1;
        let a3175 = 5u64;
        nat_eq(a3174, a3175)
    } else {
        false
    } {
        let a3176 = v2;
        let a3177 = 16u64;
        nat_eq(a3176, a3177)
    } else {
        false
    } {
        let a3178 = v3;
        let a3179 = 0u64;
        nat_eq(a3178, a3179)
    } else {
        false
    } {
        let a3180 = v4;
        let a3181 = 0u64;
        nat_eq(a3180, a3181)
    } else {
        false
    }
}

pub fn f96(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a3182 = v1;
    let a3183 = 0u64;
    if nat_eq(a3182, a3183) {
        if v0 {
            let a3184 = v3;
            let a3185 = 0u64;
            if if if nat_eq(a3184, a3185) {
                let a3186 = v4;
                let a3187 = 43u64;
                nat_eq(a3186, a3187)
            } else {
                false
            } {
                true
            } else {
                let a3188 = v3;
                let a3189 = 43u64;
                if nat_eq(a3188, a3189) {
                    let a3190 = v4;
                    let a3191 = 0u64;
                    nat_eq(a3190, a3191)
                } else {
                    false
                }
            } {
                f175(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a3192 = v1;
        let a3193 = 1u64;
        if nat_eq(a3192, a3193) {
            Adt11::C1(v2)
        } else {
            let a3194 = v1;
            let a3195 = 2u64;
            if nat_eq(a3194, a3195) {
                Adt11::C5(v2)
            } else {
                let a3196 = v1;
                let a3197 = 3u64;
                if nat_eq(a3196, a3197) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a3198 = v1;
                    let a3199 = 4u64;
                    if nat_eq(a3198, a3199) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f95(v0, v1, v2, v3, v4) {
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

pub fn f97(v0: u64, v1: u64) -> Adt12 {
    let a3200 = v0;
    let a3201 = 0u64;
    if nat_eq(a3200, a3201) {
        Adt12::C0(0u64, v1)
    } else {
        let a3202 = v0;
        let a3203 = 1u64;
        if nat_eq(a3202, a3203) {
            Adt12::C1(v1)
        } else {
            let a3204 = v0;
            let a3205 = 2u64;
            if nat_eq(a3204, a3205) {
                Adt12::C2(v1)
            } else {
                let a3206 = v0;
                let a3207 = 3u64;
                if nat_eq(a3206, a3207) {
                    Adt12::C3(v1)
                } else {
                    let a3208 = v0;
                    let a3209 = 13u64;
                    if nat_lt(a3208, a3209) {
                        Adt12::C0({
                            let a3210 = v0;
                            let a3211 = 3u64;
                            nat_sub(a3210, a3211)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f98(v0: u64, v1: u64) -> Adt13 {
    let a3212 = v0;
    let a3213 = 1u64;
    if nat_eq(a3212, a3213) {
        Adt13::C0(v1)
    } else {
        let a3214 = v0;
        let a3215 = 3u64;
        if if nat_eq(a3214, a3215) {
            true
        } else {
            let a3216 = v0;
            let a3217 = 6u64;
            nat_eq(a3216, a3217)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f99(v0: u64, v1: Adt3) -> R<u64> {
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
    f150({
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

pub fn f100(v0: Adt6, v1: u64) -> R<u64> {
    let a3247 = 0u64;
    let a3248 = match v0.clone() {
        Adt6::C0(h3246, _, _, _) => {
            h3246
        }
    };
    if if if if if nat_lt(a3247, a3248) {
        let a3250 = 0u64;
        let a3251 = match v0.clone() {
            Adt6::C0(_, h3249, _, _) => {
                h3249
            }
        };
        nat_lt(a3250, a3251)
    } else {
        false
    } {
        let a3253 = 0u64;
        let a3254 = match v0.clone() {
            Adt6::C0(_, _, _, h3252) => {
                h3252
            }
        };
        nat_lt(a3253, a3254)
    } else {
        false
    } {
        let a3256 = match v0.clone() {
            Adt6::C0(_, h3255, _, _) => {
                h3255
            }
        };
        let a3257 = 2147483648u64;
        nat_lt(a3256, a3257)
    } else {
        false
    } {
        let a3259 = match v0.clone() {
            Adt6::C0(_, _, _, h3258) => {
                h3258
            }
        };
        let a3260 = 2147483648u64;
        nat_lt(a3259, a3260)
    } else {
        false
    } {
        let v2: u64 = {
            let a3264 = v1;
            let a3265 = {
                let a3262 = 2u64;
                let a3263 = f152(match v0.clone() {
                    Adt6::C0(_, _, h3261, _) => {
                        h3261
                    }
                });
                nat_mul(a3262, a3263)?
            };
            nat_add(a3264, a3265)?
        };
        let v3: u64 = {
            let a3277 = {
                let a3272 = 1u64;
                let a3273 = {
                    let a3270 = match v0.clone() {
                        Adt6::C0(_, _, _, h3266) => {
                            h3266
                        }
                    };
                    let a3271 = {
                        let a3268 = match v0.clone() {
                            Adt6::C0(h3267, _, _, _) => {
                                h3267
                            }
                        };
                        let a3269 = 1u64;
                        nat_sub(a3268, a3269)
                    };
                    nat_mul(a3270, a3271)?
                };
                nat_add(a3272, a3273)?
            };
            let a3278 = {
                let a3275 = 2u64;
                let a3276 = f151(match v0.clone() {
                    Adt6::C0(_, _, h3274, _) => {
                        h3274
                    }
                });
                nat_mul(a3275, a3276)?
            };
            nat_add(a3277, a3278)?
        };
        let a3279 = v3;
        let a3280 = v2;
        if nat_le(a3279, a3280) {
            let a3287 = 1u64;
            let a3288 = {
                let a3284 = {
                    let a3281 = v2;
                    let a3282 = v3;
                    nat_sub(a3281, a3282)
                };
                let a3285 = match v0 {
                    Adt6::C0(_, h3283, _, _) => {
                        h3283
                    }
                };
                let a3286 = 0u64;
                nat_quot(a3284, a3285, a3286)
            };
            nat_add(a3287, a3288)
        } else {
            let a3292 = {
                let a3289 = v3;
                let a3290 = v2;
                nat_sub(a3289, a3290)
            };
            let a3293 = match v0 {
                Adt6::C0(_, h3291, _, _) => {
                    h3291
                }
            };
            if nat_lt(a3292, a3293) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f101(v0: u64) -> u64 {
    let v1: u64 = {
        let a3294 = v0;
        let a3295 = 4294967296u64;
        let a3296 = 0u64;
        nat_rem(a3294, a3295, a3296)
    };
    let a3297 = v1;
    let a3298 = 2147483648u64;
    if nat_lt(a3297, a3298) {
        v1
    } else {
        0u64
    }
}

pub fn f102(v0: u64) -> R<u64> {
    let a3301 = 2u64;
    let a3302 = {
        let a3299 = 2u64;
        let a3300 = v0;
        nat_mul(a3299, a3300)?
    };
    nat_add(a3301, a3302)
}

pub fn f103(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a3304 = match v1.clone() {
        Adt14::C0(h3303, _, _, _, _, _, _, _, _, _, _) => {
            h3303
        }
    };
    let a3305 = 0u64;
    if if if if nat_eq(a3304, a3305) {
        let a3307 = match v2.clone() {
            Adt14::C0(h3306, _, _, _, _, _, _, _, _, _, _) => {
                h3306
            }
        };
        let a3308 = 0u64;
        nat_eq(a3307, a3308)
    } else {
        false
    } {
        f209(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f210(match v1.clone() {
            Adt14::C0(_, h3309, _, _, _, _, _, _, _, _, _) => {
                h3309
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3310, _, _, _, _, _, _, _, _) => {
                h3310
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3311, _, _, _, _, _, _, _) => {
                h3311
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3312, _, _, _, _, _, _) => {
                h3312
            }
        }) {
            true
        } else {
            if if if if f49(v0) {
                f211(v1.clone(), v2.clone())
            } else {
                false
            } {
                f212(v1.clone())
            } else {
                false
            } {
                f212(v2.clone())
            } else {
                false
            } {
                if f213(v1.clone(), v2.clone())? {
                    true
                } else {
                    f214(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f210(match v1.clone() {
            Adt14::C0(_, h3313, _, _, _, _, _, _, _, _, _) => {
                h3313
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h3314, _, _, _, _, _, _, _, _) => {
                h3314
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h3315, _, _, _, _, _, _, _) => {
                h3315
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h3316, _, _, _, _, _, _) => {
                h3316
            }
        }) {
            0u64
        } else {
            let a3322 = match v1.clone() {
                Adt14::C0(_, _, h3317, _, _, _, _, _, _, _, _) => {
                    h3317
                }
            };
            let a3323 = {
                let a3320 = match v1.clone() {
                    Adt14::C0(_, _, _, h3318, _, _, _, _, _, _, _) => {
                        h3318
                    }
                };
                let a3321 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h3319, _, _, _, _, _, _) => {
                        h3319
                    }
                };
                nat_mul(a3320, a3321)?
            };
            nat_mul(a3322, a3323)?
        };
        let v4: u64 = {
            let a3324 = v3;
            let a3325 = 0u64;
            if if nat_eq(a3324, a3325) {
                true
            } else {
                f215(v2.clone())?
            } {
                0u64
            } else {
                let a3327 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3326, _) => {
                        h3326
                    }
                };
                let a3328 = 4u64;
                let a3329 = 0u64;
                nat_quot(a3327, a3328, a3329)
            }
        };
        Ok(Adt15::C1(f216(f217(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h3330, _, _, _, _, _) => {
                h3330
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h3331, _, _, _, _, _) => {
                h3331
            }
        }), match v1 {
            Adt14::C0(_, h3332, _, _, _, _, _, _, _, _, _) => {
                h3332
            }
        }, v3, v4)?, 4u64, 64u64, v3, f218(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f104(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f219(v3.clone());
    let a3334 = match v2.clone() {
        Adt14::C0(h3333, _, _, _, _, _, _, _, _, _, _) => {
            h3333
        }
    };
    let a3335 = 0u64;
    if if if if nat_eq(a3334, a3335) {
        let a3337 = match v3.clone() {
            Adt14::C0(h3336, _, _, _, _, _, _, _, _, _, _) => {
                h3336
            }
        };
        let a3338 = 0u64;
        nat_eq(a3337, a3338)
    } else {
        false
    } {
        f209(v2.clone(), v3)
    } else {
        false
    } {
        if f210(match v2.clone() {
            Adt14::C0(_, h3339, _, _, _, _, _, _, _, _, _) => {
                h3339
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3340, _, _, _, _, _, _, _, _) => {
                h3340
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3341, _, _, _, _, _, _, _) => {
                h3341
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3342, _, _, _, _, _, _) => {
                h3342
            }
        }) {
            true
        } else {
            let a3344 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3343, _, _, _, _, _) => {
                    h3343
                }
            };
            let a3345 = 4u64;
            if if if if if nat_eq(a3344, a3345) {
                f220(match v2.clone() {
                    Adt14::C0(_, h3346, _, _, _, _, _, _, _, _, _) => {
                        h3346
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h3347, _, _, _, _, _, _, _, _) => {
                        h3347
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h3348, _, _, _, _, _, _, _) => {
                        h3348
                    }
                }, v0)?
            } else {
                false
            } {
                f212(v2.clone())
            } else {
                false
            } {
                f212(v4.clone())
            } else {
                false
            } {
                let a3350 = f221(v4.clone())?;
                let a3351 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3349, _) => {
                        h3349
                    }
                };
                nat_le(a3350, a3351)
            } else {
                false
            } {
                if f213(v2.clone(), v4.clone())? {
                    true
                } else {
                    f214(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f210(match v2.clone() {
            Adt14::C0(_, h3352, _, _, _, _, _, _, _, _, _) => {
                h3352
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3353, _, _, _, _, _, _, _, _) => {
                h3353
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3354, _, _, _, _, _, _, _) => {
                h3354
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3355, _, _, _, _, _, _) => {
                h3355
            }
        }) {
            true
        } else {
            f215(v4.clone())?
        } {
            0u64
        } else {
            let a3357 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h3356, _) => {
                    h3356
                }
            };
            let a3358 = 4u64;
            let a3359 = 0u64;
            nat_quot(a3357, a3358, a3359)
        };
        Ok(Adt15::C1(f222(match v2.clone() {
            Adt14::C0(_, h3360, _, _, _, _, _, _, _, _, _) => {
                h3360
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3361, _, _, _, _, _, _, _, _) => {
                h3361
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3362, _, _, _, _, _, _, _) => {
                h3362
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h3363, _, _, _, _, _, _) => {
                h3363
            }
        }, v0, v5)?, 4u64, 65u64, v0, f218(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f105(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f219(v4.clone());
    if if if if if if if v1 {
        let a3365 = match v2.clone() {
            Adt14::C0(h3364, _, _, _, _, _, _, _, _, _, _) => {
                h3364
            }
        };
        let a3366 = 0u64;
        nat_eq(a3365, a3366)
    } else {
        false
    } {
        let a3368 = match v3.clone() {
            Adt14::C0(h3367, _, _, _, _, _, _, _, _, _, _) => {
                h3367
            }
        };
        let a3369 = 0u64;
        nat_eq(a3368, a3369)
    } else {
        false
    } {
        let a3371 = match v4.clone() {
            Adt14::C0(h3370, _, _, _, _, _, _, _, _, _, _) => {
                h3370
            }
        };
        let a3372 = 0u64;
        nat_eq(a3371, a3372)
    } else {
        false
    } {
        f209(v2.clone(), v3.clone())
    } else {
        false
    } {
        f209(v2.clone(), v4)
    } else {
        false
    } {
        if f210(match v2.clone() {
            Adt14::C0(_, h3373, _, _, _, _, _, _, _, _, _) => {
                h3373
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3374, _, _, _, _, _, _, _, _) => {
                h3374
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3375, _, _, _, _, _, _, _) => {
                h3375
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3376, _, _, _, _, _, _) => {
                h3376
            }
        }) {
            true
        } else {
            let a3378 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h3377, _, _, _, _, _) => {
                    h3377
                }
            };
            let a3379 = 4u64;
            if if if if if if if nat_eq(a3378, a3379) {
                let a3381 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h3380, _, _, _, _, _) => {
                        h3380
                    }
                };
                let a3382 = 4u64;
                nat_eq(a3381, a3382)
            } else {
                false
            } {
                f212(v2.clone())
            } else {
                false
            } {
                f212(v3.clone())
            } else {
                false
            } {
                f212(v5.clone())
            } else {
                false
            } {
                let a3384 = f221(v5.clone())?;
                let a3385 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3383, _) => {
                        h3383
                    }
                };
                nat_le(a3384, a3385)
            } else {
                false
            } {
                if f213(v2.clone(), v5.clone())? {
                    true
                } else {
                    f214(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f213(v3.clone(), v5.clone())? {
                    true
                } else {
                    f214(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f210(match v2.clone() {
            Adt14::C0(_, h3386, _, _, _, _, _, _, _, _, _) => {
                h3386
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h3387, _, _, _, _, _, _, _, _) => {
                h3387
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h3388, _, _, _, _, _, _, _) => {
                h3388
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h3389, _, _, _, _, _, _) => {
                h3389
            }
        }) {
            0u64
        } else {
            let a3395 = match v2.clone() {
                Adt14::C0(_, _, h3390, _, _, _, _, _, _, _, _) => {
                    h3390
                }
            };
            let a3396 = {
                let a3393 = match v2.clone() {
                    Adt14::C0(_, _, _, h3391, _, _, _, _, _, _, _) => {
                        h3391
                    }
                };
                let a3394 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h3392, _, _, _, _, _, _) => {
                        h3392
                    }
                };
                nat_mul(a3393, a3394)?
            };
            nat_mul(a3395, a3396)?
        };
        let v7: u64 = {
            let a3397 = v6;
            let a3398 = 0u64;
            if if nat_eq(a3397, a3398) {
                true
            } else {
                f215(v5.clone())?
            } {
                0u64
            } else {
                let a3400 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h3399, _) => {
                        h3399
                    }
                };
                let a3401 = 4u64;
                let a3402 = 0u64;
                nat_quot(a3400, a3401, a3402)
            }
        };
        Ok(Adt15::C1(f223(match v2 {
            Adt14::C0(_, h3403, _, _, _, _, _, _, _, _, _) => {
                h3403
            }
        }, v6, v7)?, 4u64, 66u64, v6, f218(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f106(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f224(v0, v3, v6) {
        f224(v1, v4, v6)
    } else {
        false
    } {
        f224(v2, v5, v6)
    } else {
        false
    }
}

pub fn f107(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f225(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f225(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f225(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f225(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f225(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f225(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f108(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3404 = v2;
    let a3405 = 0u64;
    if nat_eq(a3404, a3405) {
        Ok(0u64)
    } else {
        let a3408 = 2u64;
        let a3409 = {
            let a3406 = v0;
            let a3407 = v1;
            nat_mul(a3406, a3407)?
        };
        nat_mul(a3408, a3409)
    }
}

pub fn f109(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3410 = v2;
    let a3411 = f205(v0, v1)?;
    nat_mul(a3410, a3411)
}

pub fn f110(v0: u64) -> u64 {
    let a3412 = v0;
    let a3413 = 30u64;
    if nat_eq(a3412, a3413) {
        3u64
    } else {
        1u64
    }
}

pub fn f111(v0: u64) -> R<u64> {
    let a3416 = 5u64;
    let a3417 = {
        let a3414 = 4u64;
        let a3415 = v0;
        nat_mul(a3414, a3415)?
    };
    nat_add(a3416, a3417)
}

pub fn f112(v0: u64) -> bool {
    let m3418 = f226(v0);
    match m3418 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a3420 = 1u64;
            let a3421 = match v1 {
                Adt19::C0(_, h3419, _) => {
                    h3419
                }
            };
            nat_lt(a3420, a3421)
        }
    }
}

pub fn f113(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3422, _, _, _, _, _, _, _, _, _, _, _) => {
            h3422
        }
    };
    let v4: u64 = match v0 {
        Adt1::C0(_, h3423, _, _, _, _, _, _, _, _, _, _) => {
            h3423
        }
    };
    let v5: u64 = match v1.clone() {
        Adt2::C0(_, h3424, _, _, _, _, _, _, _, _, _) => {
            h3424
        }
    };
    let a3426 = match v1.clone() {
        Adt2::C0(h3425, _, _, _, _, _, _, _, _, _, _) => {
            h3425
        }
    };
    let a3427 = 0u64;
    if if if if if if if if if if if if if nat_eq(a3426, a3427) {
        let a3429 = match v2.clone() {
            Adt2::C0(h3428, _, _, _, _, _, _, _, _, _, _) => {
                h3428
            }
        };
        let a3430 = 0u64;
        nat_eq(a3429, a3430)
    } else {
        false
    } {
        let a3432 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h3431, _, _, _, _, _) => {
                h3431
            }
        };
        let a3433 = 4u64;
        nat_eq(a3432, a3433)
    } else {
        false
    } {
        let a3435 = match v2.clone() {
            Adt2::C0(_, h3434, _, _, _, _, _, _, _, _, _) => {
                h3434
            }
        };
        let a3436 = v3;
        nat_eq(a3435, a3436)
    } else {
        false
    } {
        let a3438 = match v2.clone() {
            Adt2::C0(_, _, h3437, _, _, _, _, _, _, _, _) => {
                h3437
            }
        };
        let a3439 = v5;
        nat_eq(a3438, a3439)
    } else {
        false
    } {
        let a3441 = match v2.clone() {
            Adt2::C0(_, _, _, h3440, _, _, _, _, _, _, _) => {
                h3440
            }
        };
        let a3442 = 1u64;
        nat_eq(a3441, a3442)
    } else {
        false
    } {
        let a3444 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3443, _, _, _, _, _, _) => {
                h3443
            }
        };
        let a3445 = 1u64;
        nat_eq(a3444, a3445)
    } else {
        false
    } {
        let a3446 = v3;
        let a3447 = 2147483648u64;
        nat_lt(a3446, a3447)
    } else {
        false
    } {
        let a3448 = v4;
        let a3449 = 4294967296u64;
        nat_lt(a3448, a3449)
    } else {
        false
    } {
        f22(v2.clone())?
    } else {
        false
    } {
        let a3453 = {
            let a3450 = v5;
            let a3451 = 4u64;
            nat_mul(a3450, a3451)?
        };
        let a3454 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h3452, _) => {
                h3452
            }
        };
        nat_le(a3453, a3454)
    } else {
        false
    } {
        f31(v2.clone(), f23(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f35(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3455 = v5;
            let a3456 = 0u64;
            if nat_eq(a3455, a3456) {
                0u64
            } else {
                f227(v3, v5)?
            }
        }, 4u64, f27(v1, 1u64), f27(v2, 1u64), Adt4::C44(v4)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f114(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    if if if if if if if if f28(v2.clone(), v3.clone()) {
        let a3458 = match v3.clone() {
            Adt2::C0(h3457, _, _, _, _, _, _, _, _, _, _) => {
                h3457
            }
        };
        let a3459 = 0u64;
        nat_eq(a3458, a3459)
    } else {
        false
    } {
        f22(v3.clone())?
    } else {
        false
    } {
        f31(v3.clone(), f23(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f35(v2.clone(), v3.clone())?
    } else {
        false
    } {
        let a3461 = match v2.clone() {
            Adt2::C0(h3460, _, _, _, _, _, _, _, _, _, _) => {
                h3460
            }
        };
        let a3462 = 1u64;
        nat_le(a3461, a3462)
    } else {
        false
    } {
        let a3464 = match v0.clone() {
            Adt1::C0(h3463, _, _, _, _, _, _, _, _, _, _, _) => {
                h3463
            }
        };
        let a3465 = 0u64;
        if nat_eq(a3464, a3465) {
            let a3467 = match v2.clone() {
                Adt2::C0(h3466, _, _, _, _, _, _, _, _, _, _) => {
                    h3466
                }
            };
            let a3468 = 0u64;
            if if nat_eq(a3467, a3468) {
                f22(v2.clone())?
            } else {
                false
            } {
                f31(v2.clone(), f23(v2, 1u64)?)?
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    } {
        f123(true, true, v0, v3, v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f115(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3470 = match v1.clone() {
        Adt2::C0(h3469, _, _, _, _, _, _, _, _, _, _) => {
            h3469
        }
    };
    let a3471 = 0u64;
    if if if if if if if nat_eq(a3470, a3471) {
        let a3473 = match v3.clone() {
            Adt2::C0(h3472, _, _, _, _, _, _, _, _, _, _) => {
                h3472
            }
        };
        let a3474 = 0u64;
        nat_eq(a3473, a3474)
    } else {
        false
    } {
        let a3476 = match v2.clone() {
            Adt2::C0(h3475, _, _, _, _, _, _, _, _, _, _) => {
                h3475
            }
        };
        let a3477 = 1u64;
        nat_le(a3476, a3477)
    } else {
        false
    } {
        f22(v1.clone())?
    } else {
        false
    } {
        f31(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v3.clone(), f23(v3.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3479 = match v0.clone() {
            Adt1::C0(_, _, _, _, _, _, h3478, _, _, _, _, _) => {
                h3478
            }
        };
        let a3480 = 0u64;
        if nat_eq(a3479, a3480) {
            true
        } else {
            let a3482 = match v3.clone() {
                Adt2::C0(_, _, h3481, _, _, _, _, _, _, _, _) => {
                    h3481
                }
            };
            let a3483 = 1u64;
            if nat_le(a3482, a3483) {
                true
            } else {
                let a3488 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3484, _, _, _, _) => {
                        h3484
                    }
                };
                let a3489 = {
                    let a3486 = 4u64;
                    let a3487 = match v3.clone() {
                        Adt2::C0(_, h3485, _, _, _, _, _, _, _, _, _) => {
                            h3485
                        }
                    };
                    nat_mul(a3486, a3487)?
                };
                nat_eq(a3488, a3489)
            }
        }
    } else {
        false
    } {
        f124(true, v0, v2, Some(v3), v1)
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f116(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3491 = match v0.clone() {
        Adt2::C0(h3490, _, _, _, _, _, _, _, _, _, _) => {
            h3490
        }
    };
    let a3492 = 0u64;
    if if if if if if if if if if if if nat_eq(a3491, a3492) {
        f22(v0.clone())?
    } else {
        false
    } {
        f31(v0.clone(), f23(v0.clone(), 1u64)?)?
    } else {
        false
    } {
        let a3494 = match v1.clone() {
            Adt2::C0(h3493, _, _, _, _, _, _, _, _, _, _) => {
                h3493
            }
        };
        let a3495 = 0u64;
        if if nat_eq(a3494, a3495) {
            f22(v1.clone())?
        } else {
            false
        } {
            f31(v1.clone(), f23(v1.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3497 = match v2.clone() {
            Adt2::C0(h3496, _, _, _, _, _, _, _, _, _, _) => {
                h3496
            }
        };
        let a3498 = 0u64;
        if if nat_eq(a3497, a3498) {
            f22(v2.clone())?
        } else {
            false
        } {
            f31(v2.clone(), f23(v2.clone(), 1u64)?)?
        } else {
            false
        }
    } else {
        false
    } {
        let a3501 = match v0.clone() {
            Adt2::C0(_, h3499, _, _, _, _, _, _, _, _, _) => {
                h3499
            }
        };
        let a3502 = match v0.clone() {
            Adt2::C0(_, _, h3500, _, _, _, _, _, _, _, _) => {
                h3500
            }
        };
        nat_eq(a3501, a3502)
    } else {
        false
    } {
        let a3505 = match v0.clone() {
            Adt2::C0(_, _, h3503, _, _, _, _, _, _, _, _) => {
                h3503
            }
        };
        let a3506 = match v1.clone() {
            Adt2::C0(_, _, h3504, _, _, _, _, _, _, _, _) => {
                h3504
            }
        };
        nat_eq(a3505, a3506)
    } else {
        false
    } {
        let a3509 = match v0.clone() {
            Adt2::C0(_, _, _, h3507, _, _, _, _, _, _, _) => {
                h3507
            }
        };
        let a3510 = match v1.clone() {
            Adt2::C0(_, _, _, h3508, _, _, _, _, _, _, _) => {
                h3508
            }
        };
        nat_eq(a3509, a3510)
    } else {
        false
    } {
        let a3513 = match v0.clone() {
            Adt2::C0(_, _, _, _, h3511, _, _, _, _, _, _) => {
                h3511
            }
        };
        let a3514 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3512, _, _, _, _, _, _) => {
                h3512
            }
        };
        nat_eq(a3513, a3514)
    } else {
        false
    } {
        f28(v1.clone(), v2.clone())
    } else {
        false
    } {
        f35(v0.clone(), v2.clone())?
    } else {
        false
    } {
        f35(v1.clone(), v2.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3523 = {
                let a3520 = {
                    let a3517 = match v1.clone() {
                        Adt2::C0(_, _, _, h3515, _, _, _, _, _, _, _) => {
                            h3515
                        }
                    };
                    let a3518 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3516, _, _, _, _, _, _) => {
                            h3516
                        }
                    };
                    nat_mul(a3517, a3518)?
                };
                let a3521 = match v1 {
                    Adt2::C0(_, h3519, _, _, _, _, _, _, _, _, _) => {
                        h3519
                    }
                };
                nat_mul(a3520, a3521)?
            };
            let a3524 = f186(match v0.clone() {
                Adt2::C0(_, h3522, _, _, _, _, _, _, _, _, _) => {
                    h3522
                }
            })?;
            nat_mul(a3523, a3524)?
        }, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C41))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f117(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a3526 = 0u64;
        let a3527 = match v0.clone() {
            Adt1::C0(_, h3525, _, _, _, _, _, _, _, _, _, _) => {
                h3525
            }
        };
        nat_lt(a3526, a3527)
    };
    let a3529 = match v1.clone() {
        Adt2::C0(h3528, _, _, _, _, _, _, _, _, _, _) => {
            h3528
        }
    };
    let a3530 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a3529, a3530) {
        let a3532 = match v2.clone() {
            Adt2::C0(h3531, _, _, _, _, _, _, _, _, _, _) => {
                h3531
            }
        };
        let a3533 = 0u64;
        nat_eq(a3532, a3533)
    } else {
        false
    } {
        let a3535 = match v3.clone() {
            Adt2::C0(h3534, _, _, _, _, _, _, _, _, _, _) => {
                h3534
            }
        };
        let a3536 = 0u64;
        nat_eq(a3535, a3536)
    } else {
        false
    } {
        let a3538 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3537, _, _, _, _, _, _) => {
                h3537
            }
        };
        let a3539 = 1u64;
        nat_eq(a3538, a3539)
    } else {
        false
    } {
        let a3541 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3540, _, _, _, _, _, _) => {
                h3540
            }
        };
        let a3542 = 1u64;
        nat_eq(a3541, a3542)
    } else {
        false
    } {
        let a3544 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3543, _, _, _, _, _, _) => {
                h3543
            }
        };
        let a3545 = 1u64;
        nat_eq(a3544, a3545)
    } else {
        false
    } {
        let a3547 = 0u64;
        let a3548 = match v1.clone() {
            Adt2::C0(_, _, h3546, _, _, _, _, _, _, _, _) => {
                h3546
            }
        };
        nat_lt(a3547, a3548)
    } else {
        false
    } {
        let a3550 = match v3.clone() {
            Adt2::C0(_, _, _, h3549, _, _, _, _, _, _, _) => {
                h3549
            }
        };
        let a3551 = 1u64;
        nat_eq(a3550, a3551)
    } else {
        false
    } {
        let a3554 = match v3.clone() {
            Adt2::C0(_, h3552, _, _, _, _, _, _, _, _, _) => {
                h3552
            }
        };
        let a3555 = match v1.clone() {
            Adt2::C0(_, h3553, _, _, _, _, _, _, _, _, _) => {
                h3553
            }
        };
        nat_eq(a3554, a3555)
    } else {
        false
    } {
        let a3558 = match v3.clone() {
            Adt2::C0(_, _, h3556, _, _, _, _, _, _, _, _) => {
                h3556
            }
        };
        let a3559 = match v1.clone() {
            Adt2::C0(_, _, _, h3557, _, _, _, _, _, _, _) => {
                h3557
            }
        };
        nat_eq(a3558, a3559)
    } else {
        false
    } {
        if v4 {
            f28(v1.clone(), v2.clone())
        } else {
            let a3562 = match v2.clone() {
                Adt2::C0(_, h3560, _, _, _, _, _, _, _, _, _) => {
                    h3560
                }
            };
            let a3563 = match v1.clone() {
                Adt2::C0(_, _, h3561, _, _, _, _, _, _, _, _) => {
                    h3561
                }
            };
            if if nat_eq(a3562, a3563) {
                let a3566 = match v2.clone() {
                    Adt2::C0(_, _, h3564, _, _, _, _, _, _, _, _) => {
                        h3564
                    }
                };
                let a3567 = match v1.clone() {
                    Adt2::C0(_, _, _, h3565, _, _, _, _, _, _, _) => {
                        h3565
                    }
                };
                nat_eq(a3566, a3567)
            } else {
                false
            } {
                let a3569 = match v2.clone() {
                    Adt2::C0(_, _, _, h3568, _, _, _, _, _, _, _) => {
                        h3568
                    }
                };
                let a3570 = 1u64;
                nat_eq(a3569, a3570)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f31(v1.clone(), f23(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f31(v2.clone(), f23(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f36(v3.clone())?
    } else {
        false
    } {
        f35(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f35(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3572 = f23(v3.clone(), 1u64)?;
            let a3573 = f177(match v1.clone() {
                Adt2::C0(_, _, h3571, _, _, _, _, _, _, _, _) => {
                    h3571
                }
            }, v4)?;
            nat_mul(a3572, a3573)?
        }, 4u64, f27(v1, 1u64), f27(v3, 1u64), Adt4::C36(f27(v2, 1u64), v4, match v0 {
            Adt1::C0(h3574, _, _, _, _, _, _, _, _, _, _, _) => {
                h3574
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f118(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h3575, _, _, _, _, _, _, _, _, _) => {
            h3575
        }
    }, match v0.clone() {
        Adt1::C0(h3576, _, _, _, _, _, _, _, _, _, _, _) => {
            h3576
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h3577, _, _, _, _, _, _, _, _, _) => {
            h3577
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h3578, _, _, _, _, _, _, _) => {
            h3578
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h3579, _, _, _, _, _, _, _, _) => {
            h3579
        }
    }, match v0.clone() {
        Adt1::C0(_, h3580, _, _, _, _, _, _, _, _, _, _) => {
            h3580
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h3581, _, _, _, _, _, _, _, _) => {
            h3581
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h3582, _, _, _, _, _, _) => {
            h3582
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h3583, _, _, _, _, _, _, _, _, _, _) => {
            h3583
        }
    };
    let v7: bool = {
        let a3586 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h3584, _, _, _) => {
                h3584
            }
        };
        let a3587 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h3585, _, _, _, _, _) => {
                h3585
            }
        };
        nat_lt(a3586, a3587)
    };
    let a3588 = v6;
    let a3589 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a3588, a3589) {
        true
    } else {
        let a3590 = v6;
        let a3591 = 1u64;
        nat_eq(a3590, a3591)
    } {
        let a3593 = match v2.clone() {
            Adt2::C0(h3592, _, _, _, _, _, _, _, _, _, _) => {
                h3592
            }
        };
        let a3594 = 0u64;
        nat_eq(a3593, a3594)
    } else {
        false
    } {
        let a3596 = match v3.clone() {
            Adt2::C0(h3595, _, _, _, _, _, _, _, _, _, _) => {
                h3595
            }
        };
        let a3597 = 0u64;
        nat_eq(a3596, a3597)
    } else {
        false
    } {
        let a3599 = match v1.clone() {
            Adt2::C0(_, _, _, h3598, _, _, _, _, _, _, _) => {
                h3598
            }
        };
        let a3600 = 1u64;
        nat_eq(a3599, a3600)
    } else {
        false
    } {
        let a3603 = match v1.clone() {
            Adt2::C0(_, _, _, _, h3601, _, _, _, _, _, _) => {
                h3601
            }
        };
        let a3604 = match v2.clone() {
            Adt2::C0(_, _, _, h3602, _, _, _, _, _, _, _) => {
                h3602
            }
        };
        nat_eq(a3603, a3604)
    } else {
        false
    } {
        let a3607 = match v3.clone() {
            Adt2::C0(_, _, _, h3605, _, _, _, _, _, _, _) => {
                h3605
            }
        };
        let a3608 = match v2.clone() {
            Adt2::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                h3606
            }
        };
        nat_eq(a3607, a3608)
    } else {
        false
    } {
        let a3611 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3609, _, _, _, _, _, _) => {
                h3609
            }
        };
        let a3612 = match v2.clone() {
            Adt2::C0(_, _, _, _, h3610, _, _, _, _, _, _) => {
                h3610
            }
        };
        nat_eq(a3611, a3612)
    } else {
        false
    } {
        let a3614 = 0u64;
        let a3615 = match v4.clone() {
            Adt6::C0(h3613, _, _, _) => {
                h3613
            }
        };
        if if if if if nat_lt(a3614, a3615) {
            let a3617 = 0u64;
            let a3618 = match v4.clone() {
                Adt6::C0(_, h3616, _, _) => {
                    h3616
                }
            };
            nat_lt(a3617, a3618)
        } else {
            false
        } {
            let a3620 = match v4.clone() {
                Adt6::C0(_, h3619, _, _) => {
                    h3619
                }
            };
            let a3621 = 2147483648u64;
            nat_lt(a3620, a3621)
        } else {
            false
        } {
            let a3623 = 0u64;
            let a3624 = match v4.clone() {
                Adt6::C0(_, _, _, h3622) => {
                    h3622
                }
            };
            nat_lt(a3623, a3624)
        } else {
            false
        } {
            let a3626 = match v4.clone() {
                Adt6::C0(_, _, _, h3625) => {
                    h3625
                }
            };
            let a3627 = 2147483648u64;
            nat_lt(a3626, a3627)
        } else {
            false
        } {
            let a3630 = match v3.clone() {
                Adt2::C0(_, h3628, _, _, _, _, _, _, _, _, _) => {
                    h3628
                }
            };
            let a3631 = f100(v4.clone(), match v2.clone() {
                Adt2::C0(_, h3629, _, _, _, _, _, _, _, _, _) => {
                    h3629
                }
            })?;
            nat_eq(a3630, a3631)
        } else {
            false
        }
    } else {
        false
    } {
        let a3633 = 0u64;
        let a3634 = match v5.clone() {
            Adt6::C0(h3632, _, _, _) => {
                h3632
            }
        };
        if if if if if nat_lt(a3633, a3634) {
            let a3636 = 0u64;
            let a3637 = match v5.clone() {
                Adt6::C0(_, h3635, _, _) => {
                    h3635
                }
            };
            nat_lt(a3636, a3637)
        } else {
            false
        } {
            let a3639 = match v5.clone() {
                Adt6::C0(_, h3638, _, _) => {
                    h3638
                }
            };
            let a3640 = 2147483648u64;
            nat_lt(a3639, a3640)
        } else {
            false
        } {
            let a3642 = 0u64;
            let a3643 = match v5.clone() {
                Adt6::C0(_, _, _, h3641) => {
                    h3641
                }
            };
            nat_lt(a3642, a3643)
        } else {
            false
        } {
            let a3645 = match v5.clone() {
                Adt6::C0(_, _, _, h3644) => {
                    h3644
                }
            };
            let a3646 = 2147483648u64;
            nat_lt(a3645, a3646)
        } else {
            false
        } {
            let a3649 = match v3.clone() {
                Adt2::C0(_, _, h3647, _, _, _, _, _, _, _, _) => {
                    h3647
                }
            };
            let a3650 = f100(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h3648, _, _, _, _, _, _, _, _) => {
                    h3648
                }
            })?;
            nat_eq(a3649, a3650)
        } else {
            false
        }
    } else {
        false
    } {
        f228(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f228(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f228(v1.clone(), {
            let a3651 = v6;
            let a3652 = 0u64;
            if nat_eq(a3651, a3652) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f35(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f35(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3661 = {
                let a3659 = {
                    let a3657 = 3u64;
                    let a3658 = {
                        let a3655 = match v4.clone() {
                            Adt6::C0(h3653, _, _, _) => {
                                h3653
                            }
                        };
                        let a3656 = match v5.clone() {
                            Adt6::C0(h3654, _, _, _) => {
                                h3654
                            }
                        };
                        nat_mul(a3655, a3656)?
                    };
                    nat_mul(a3657, a3658)?
                };
                let a3660 = 2u64;
                nat_add(a3659, a3660)?
            };
            let a3662 = f23(v3.clone(), 1u64)?;
            nat_mul(a3661, a3662)?
        }, 4u64, f27(v1, 1u64), f27(v3, 1u64), Adt4::C34(f27(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f119(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f28(v0.clone(), v1.clone()) {
        f28(v0.clone(), v2.clone())
    } else {
        false
    } {
        f36(v0.clone())?
    } else {
        false
    } {
        f36(v1.clone())?
    } else {
        false
    } {
        f36(v2.clone())?
    } else {
        false
    } {
        if f35(v0.clone(), v2.clone())? {
            true
        } else {
            f51(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f35(v1.clone(), v2.clone())? {
            true
        } else {
            f51(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3663 = 6u64;
            let a3664 = f23(v0.clone(), 1u64)?;
            nat_mul(a3663, a3664)?
        }, 2u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C33(f27(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f120(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3665 = v2;
    match m3665 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h3666, _, _, _, _, _, _, _, _, _) => {
                    h3666
                }
            };
            let v6: u64 = {
                let a3672 = match v1.clone() {
                    Adt2::C0(_, _, h3667, _, _, _, _, _, _, _, _) => {
                        h3667
                    }
                };
                let a3673 = {
                    let a3670 = match v1.clone() {
                        Adt2::C0(_, _, _, h3668, _, _, _, _, _, _, _) => {
                            h3668
                        }
                    };
                    let a3671 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3669, _, _, _, _, _, _) => {
                            h3669
                        }
                    };
                    nat_mul(a3670, a3671)?
                };
                nat_mul(a3672, a3673)?
            };
            if if if if if if if if if if f28(v1.clone(), v4.clone()) {
                f28(v1.clone(), v3.clone())
            } else {
                false
            } {
                f229(v1.clone(), v6)?
            } else {
                false
            } {
                f229(v4.clone(), v6)?
            } else {
                false
            } {
                f229(v3.clone(), v6)?
            } else {
                false
            } {
                if f35(v1.clone(), v3.clone())? {
                    true
                } else {
                    f51(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f35(v4.clone(), v3.clone())? {
                    true
                } else {
                    f51(v4, v3.clone())
                }
            } else {
                false
            } {
                let a3674 = v5;
                let a3675 = 2147483647u64;
                nat_le(a3674, a3675)
            } else {
                false
            } {
                let a3676 = v6;
                let a3677 = 2147483647u64;
                nat_le(a3676, a3677)
            } else {
                false
            } {
                let a3679 = match v0.clone() {
                    Adt1::C0(h3678, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3678
                    }
                };
                let a3680 = 4294967295u64;
                if nat_le(a3679, a3680) {
                    let a3682 = match v0.clone() {
                        Adt1::C0(_, h3681, _, _, _, _, _, _, _, _, _, _) => {
                            h3681
                        }
                    };
                    let a3683 = 4294967295u64;
                    nat_le(a3682, a3683)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3684 = v5;
                    let a3685 = 0u64;
                    if nat_eq(a3684, a3685) {
                        0u64
                    } else {
                        let a3686 = v6;
                        let a3687 = f176(v5)?;
                        nat_mul(a3686, a3687)?
                    }
                }, 4u64, f27(v1, 1u64), f27(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h3688, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3688
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f121(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h3689, _, _, _, _, _, _, _, _, _, _, _) => {
            h3689
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h3690, _, _, _, _, _, _, _, _, _, _) => {
            h3690
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h3691, _, _, _, _, _, _, _, _, _) => {
            h3691
        }
    };
    let v6: u64 = {
        let a3693 = match v1.clone() {
            Adt2::C0(_, h3692, _, _, _, _, _, _, _, _, _) => {
                h3692
            }
        };
        let a3694 = v4;
        let a3695 = 0u64;
        nat_quot(a3693, a3694, a3695)
    };
    let v7: u64 = {
        let a3697 = match v1.clone() {
            Adt2::C0(h3696, _, _, _, _, _, _, _, _, _, _) => {
                h3696
            }
        };
        let a3698 = 0u64;
        if nat_eq(a3697, a3698) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f189(match v1.clone() {
        Adt2::C0(h3699, _, _, _, _, _, _, _, _, _, _) => {
            h3699
        }
    }) {
        let a3702 = match v1.clone() {
            Adt2::C0(h3700, _, _, _, _, _, _, _, _, _, _) => {
                h3700
            }
        };
        let a3703 = match v2.clone() {
            Adt2::C0(h3701, _, _, _, _, _, _, _, _, _, _) => {
                h3701
            }
        };
        nat_eq(a3702, a3703)
    } else {
        false
    } {
        let a3704 = 0u64;
        let a3705 = v3;
        nat_lt(a3704, a3705)
    } else {
        false
    } {
        let a3706 = v3;
        let a3707 = 2147483648u64;
        nat_lt(a3706, a3707)
    } else {
        false
    } {
        let a3708 = 0u64;
        let a3709 = v4;
        nat_lt(a3708, a3709)
    } else {
        false
    } {
        let a3710 = v4;
        let a3711 = 2147483648u64;
        nat_lt(a3710, a3711)
    } else {
        false
    } {
        let a3712 = v5;
        let a3713 = 2147483648u64;
        nat_lt(a3712, a3713)
    } else {
        false
    } {
        let a3714 = 0u64;
        let a3715 = v6;
        nat_lt(a3714, a3715)
    } else {
        false
    } {
        let a3717 = 0u64;
        let a3718 = match v2.clone() {
            Adt2::C0(_, h3716, _, _, _, _, _, _, _, _, _) => {
                h3716
            }
        };
        nat_lt(a3717, a3718)
    } else {
        false
    } {
        let a3722 = match v1.clone() {
            Adt2::C0(_, h3719, _, _, _, _, _, _, _, _, _) => {
                h3719
            }
        };
        let a3723 = {
            let a3720 = v6;
            let a3721 = v4;
            nat_mul(a3720, a3721)?
        };
        nat_eq(a3722, a3723)
    } else {
        false
    } {
        let a3725 = match v2.clone() {
            Adt2::C0(_, _, h3724, _, _, _, _, _, _, _, _) => {
                h3724
            }
        };
        let a3726 = v4;
        nat_eq(a3725, a3726)
    } else {
        false
    } {
        let a3739 = {
            let a3732 = {
                let a3730 = match v2.clone() {
                    Adt2::C0(_, h3727, _, _, _, _, _, _, _, _, _) => {
                        h3727
                    }
                };
                let a3731 = {
                    let a3728 = v5;
                    let a3729 = 2u64;
                    nat_mul(a3728, a3729)?
                };
                nat_add(a3730, a3731)?
            };
            let a3733 = v3;
            nat_add(a3732, a3733)?
        };
        let a3740 = {
            let a3737 = {
                let a3735 = match v1.clone() {
                    Adt2::C0(_, _, h3734, _, _, _, _, _, _, _, _) => {
                        h3734
                    }
                };
                let a3736 = v3;
                nat_mul(a3735, a3736)?
            };
            let a3738 = v6;
            nat_add(a3737, a3738)?
        };
        nat_eq(a3739, a3740)
    } else {
        false
    } {
        f230(v1.clone(), v7)?
    } else {
        false
    } {
        f230(v2.clone(), v7)?
    } else {
        false
    } {
        f35(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a3741 = f23(v2.clone(), 1u64)?;
            let a3742 = f202(v8.clone())?;
            nat_mul(a3741, a3742)?
        }, v7, f27(v1.clone(), 1u64), f27(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h3743, _, _, _, _, _, _, _, _, _, _) => {
                h3743
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f122(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3744 = v1;
    match m3744 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a3746 = match v0.clone() {
                Adt2::C0(h3745, _, _, _, _, _, _, _, _, _, _) => {
                    h3745
                }
            };
            let a3747 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a3746, a3747) {
                let a3749 = match v3.clone() {
                    Adt2::C0(h3748, _, _, _, _, _, _, _, _, _, _) => {
                        h3748
                    }
                };
                let a3750 = 0u64;
                nat_eq(a3749, a3750)
            } else {
                false
            } {
                let a3752 = match v2.clone() {
                    Adt2::C0(h3751, _, _, _, _, _, _, _, _, _, _) => {
                        h3751
                    }
                };
                let a3753 = 0u64;
                nat_eq(a3752, a3753)
            } else {
                false
            } {
                let a3755 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h3754, _, _, _, _, _, _) => {
                        h3754
                    }
                };
                let a3756 = 1u64;
                nat_eq(a3755, a3756)
            } else {
                false
            } {
                let a3758 = match v3.clone() {
                    Adt2::C0(_, _, _, h3757, _, _, _, _, _, _, _) => {
                        h3757
                    }
                };
                let a3759 = 1u64;
                nat_eq(a3758, a3759)
            } else {
                false
            } {
                let a3761 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3760, _, _, _, _, _, _) => {
                        h3760
                    }
                };
                let a3762 = 1u64;
                nat_eq(a3761, a3762)
            } else {
                false
            } {
                let a3765 = match v0.clone() {
                    Adt2::C0(_, _, h3763, _, _, _, _, _, _, _, _) => {
                        h3763
                    }
                };
                let a3766 = match v3.clone() {
                    Adt2::C0(_, _, h3764, _, _, _, _, _, _, _, _) => {
                        h3764
                    }
                };
                nat_eq(a3765, a3766)
            } else {
                false
            } {
                let a3771 = match v3.clone() {
                    Adt2::C0(_, h3767, _, _, _, _, _, _, _, _, _) => {
                        h3767
                    }
                };
                let a3772 = {
                    let a3769 = match v0.clone() {
                        Adt2::C0(_, h3768, _, _, _, _, _, _, _, _, _) => {
                            h3768
                        }
                    };
                    let a3770 = 1u64;
                    nat_add(a3769, a3770)?
                };
                nat_le(a3771, a3772)
            } else {
                false
            } {
                let a3775 = match v2.clone() {
                    Adt2::C0(_, h3773, _, _, _, _, _, _, _, _, _) => {
                        h3773
                    }
                };
                let a3776 = match v0.clone() {
                    Adt2::C0(_, _, h3774, _, _, _, _, _, _, _, _) => {
                        h3774
                    }
                };
                nat_eq(a3775, a3776)
            } else {
                false
            } {
                let a3784 = match v2.clone() {
                    Adt2::C0(_, _, h3777, _, _, _, _, _, _, _, _) => {
                        h3777
                    }
                };
                let a3785 = {
                    let a3782 = {
                        let a3779 = match v0.clone() {
                            Adt2::C0(_, h3778, _, _, _, _, _, _, _, _, _) => {
                                h3778
                            }
                        };
                        let a3780 = 1u64;
                        nat_add(a3779, a3780)?
                    };
                    let a3783 = match v3.clone() {
                        Adt2::C0(_, h3781, _, _, _, _, _, _, _, _, _) => {
                            h3781
                        }
                    };
                    nat_sub(a3782, a3783)
                };
                nat_eq(a3784, a3785)
            } else {
                false
            } {
                let a3788 = match v2.clone() {
                    Adt2::C0(_, _, _, h3786, _, _, _, _, _, _, _) => {
                        h3786
                    }
                };
                let a3789 = match v0.clone() {
                    Adt2::C0(_, _, _, h3787, _, _, _, _, _, _, _) => {
                        h3787
                    }
                };
                nat_eq(a3788, a3789)
            } else {
                false
            } {
                let a3791 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h3790, _, _, _, _, _, _) => {
                        h3790
                    }
                };
                let a3792 = 1u64;
                nat_eq(a3791, a3792)
            } else {
                false
            } {
                f22(v2.clone())?
            } else {
                false
            } {
                let a3794 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h3793, _, _, _, _, _) => {
                        h3793
                    }
                };
                let a3795 = 4u64;
                nat_eq(a3794, a3795)
            } else {
                false
            } {
                let a3797 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h3796, _, _, _, _, _) => {
                        h3796
                    }
                };
                let a3798 = 4u64;
                nat_eq(a3797, a3798)
            } else {
                false
            } {
                let a3803 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3799, _, _, _, _) => {
                        h3799
                    }
                };
                let a3804 = {
                    let a3801 = match v0.clone() {
                        Adt2::C0(_, h3800, _, _, _, _, _, _, _, _, _) => {
                            h3800
                        }
                    };
                    let a3802 = 4u64;
                    nat_mul(a3801, a3802)?
                };
                nat_eq(a3803, a3804)
            } else {
                false
            } {
                let a3809 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3805, _, _, _, _) => {
                        h3805
                    }
                };
                let a3810 = {
                    let a3807 = match v3.clone() {
                        Adt2::C0(_, h3806, _, _, _, _, _, _, _, _, _) => {
                            h3806
                        }
                    };
                    let a3808 = 4u64;
                    nat_mul(a3807, a3808)?
                };
                nat_eq(a3809, a3810)
            } else {
                false
            } {
                f35(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f35(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a3812 = f23(v2.clone(), 1u64)?;
                    let a3813 = f201(match v3.clone() {
                        Adt2::C0(_, h3811, _, _, _, _, _, _, _, _, _) => {
                            h3811
                        }
                    })?;
                    nat_mul(a3812, a3813)?
                }, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C29(f27(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f123(v0: bool, v1: bool, v2: Adt1, v3: Adt2, v4: Adt2) -> R<Adt10> {
    let v5: Adt6 = Adt6::C0(match v2.clone() {
        Adt1::C0(_, h3814, _, _, _, _, _, _, _, _, _, _) => {
            h3814
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, h3815, _, _, _, _, _, _, _, _) => {
                h3815
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, h3816, _, _, _, _, _, _, _, _, _) => {
                h3816
            }
        }
    }, if v1 {
        match v2.clone() {
            Adt1::C0(_, _, _, _, _, h3817, _, _, _, _, _, _) => {
                h3817
            }
        }
    } else {
        match v2.clone() {
            Adt1::C0(_, _, _, h3818, _, _, _, _, _, _, _, _) => {
                h3818
            }
        }
    }, 1u64);
    let v6: Adt6 = if v1 {
        Adt6::C0(match v2.clone() {
            Adt1::C0(_, _, h3819, _, _, _, _, _, _, _, _, _) => {
                h3819
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, h3820, _, _, _, _, _, _, _) => {
                h3820
            }
        }, match v2.clone() {
            Adt1::C0(_, _, _, _, _, _, h3821, _, _, _, _, _) => {
                h3821
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a3823 = match v2.clone() {
        Adt1::C0(h3822, _, _, _, _, _, _, _, _, _, _, _) => {
            h3822
        }
    };
    let a3824 = 1u64;
    if if if if if if if if if if if nat_le(a3823, a3824) {
        let a3826 = match v3.clone() {
            Adt2::C0(h3825, _, _, _, _, _, _, _, _, _, _) => {
                h3825
            }
        };
        let a3827 = 1u64;
        nat_le(a3826, a3827)
    } else {
        false
    } {
        let a3829 = match v4.clone() {
            Adt2::C0(h3828, _, _, _, _, _, _, _, _, _, _) => {
                h3828
            }
        };
        let a3830 = 0u64;
        nat_eq(a3829, a3830)
    } else {
        false
    } {
        let a3835 = match v3.clone() {
            Adt2::C0(_, _, _, _, _, h3831, _, _, _, _, _) => {
                h3831
            }
        };
        let a3836 = {
            let a3833 = match v3.clone() {
                Adt2::C0(h3832, _, _, _, _, _, _, _, _, _, _) => {
                    h3832
                }
            };
            let a3834 = 0u64;
            if nat_eq(a3833, a3834) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a3835, a3836)
    } else {
        false
    } {
        f22(v4.clone())?
    } else {
        false
    } {
        f35(v3.clone(), v4.clone())?
    } else {
        false
    } {
        f231(v5.clone(), match v3.clone() {
            Adt2::C0(_, h3837, _, _, _, _, _, _, _, _, _) => {
                h3837
            }
        }, match v4.clone() {
            Adt2::C0(_, h3838, _, _, _, _, _, _, _, _, _) => {
                h3838
            }
        })?
    } else {
        false
    } {
        if v1 {
            f231(v6.clone(), match v3.clone() {
                Adt2::C0(_, _, h3839, _, _, _, _, _, _, _, _) => {
                    h3839
                }
            }, match v4.clone() {
                Adt2::C0(_, _, h3840, _, _, _, _, _, _, _, _) => {
                    h3840
                }
            })?
        } else {
            let a3843 = match v3.clone() {
                Adt2::C0(_, _, h3841, _, _, _, _, _, _, _, _) => {
                    h3841
                }
            };
            let a3844 = match v4.clone() {
                Adt2::C0(_, _, h3842, _, _, _, _, _, _, _, _) => {
                    h3842
                }
            };
            nat_eq(a3843, a3844)
        }
    } else {
        false
    } {
        let a3847 = match v3.clone() {
            Adt2::C0(_, _, _, h3845, _, _, _, _, _, _, _) => {
                h3845
            }
        };
        let a3848 = match v4.clone() {
            Adt2::C0(_, _, _, h3846, _, _, _, _, _, _, _) => {
                h3846
            }
        };
        nat_eq(a3847, a3848)
    } else {
        false
    } {
        let a3851 = match v3.clone() {
            Adt2::C0(_, _, _, _, h3849, _, _, _, _, _, _) => {
                h3849
            }
        };
        let a3852 = match v4.clone() {
            Adt2::C0(_, _, _, _, h3850, _, _, _, _, _, _) => {
                h3850
            }
        };
        nat_eq(a3851, a3852)
    } else {
        false
    } {
        let a3857 = {
            let a3855 = match v5.clone() {
                Adt6::C0(h3853, _, _, _) => {
                    h3853
                }
            };
            let a3856 = match v6.clone() {
                Adt6::C0(h3854, _, _, _) => {
                    h3854
                }
            };
            nat_mul(a3855, a3856)?
        };
        let a3858 = 2147483648u64;
        nat_lt(a3857, a3858)
    } else {
        false
    } {
        let v7: Adt7 = Adt7::C0(v5, v6, v1, {
            let a3860 = match v2 {
                Adt1::C0(h3859, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3859
                }
            };
            let a3861 = 0u64;
            nat_eq(a3860, a3861)
        }, match v3.clone() {
            Adt2::C0(h3862, _, _, _, _, _, _, _, _, _, _) => {
                h3862
            }
        });
        if v0 {
            Ok(Adt10::C4({
                let a3874 = f23(v3.clone(), 1u64)?;
                let a3875 = {
                    let a3872 = f23(v4.clone(), 1u64)?;
                    let a3873 = f180(match v7.clone() {
                        Adt7::C0(_, _, _, h3863, _) => {
                            h3863
                        }
                    }, {
                        let a3870 = {
                            let m3866 = match v7.clone() {
                                Adt7::C0(h3865, _, _, _, _) => {
                                    h3865
                                }
                            };
                            match m3866 {
                                Adt6::C0(h3864, _, _, _) => {
                                    h3864
                                }
                            }
                        };
                        let a3871 = {
                            let m3869 = match v7.clone() {
                                Adt7::C0(_, h3868, _, _, _) => {
                                    h3868
                                }
                            };
                            match m3869 {
                                Adt6::C0(h3867, _, _, _) => {
                                    h3867
                                }
                            }
                        };
                        nat_mul(a3870, a3871)?
                    })?;
                    nat_mul(a3872, a3873)?
                };
                nat_add(a3874, a3875)?
            }, 4u64, f27(v4, 1u64), f27(v3.clone(), 1u64), Adt4::C43(v7, f23(v3, 1u64)?)))
        } else {
            Ok(Adt10::C4({
                let a3876 = f23(v4.clone(), 1u64)?;
                let a3877 = f198(v7.clone())?;
                nat_mul(a3876, a3877)?
            }, 4u64, f27(v3, 1u64), f27(v4, 1u64), Adt4::C28(v7)))
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f124(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3878 = v3;
    match m3878 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3880 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3879, _, _, _, _, _) => {
                        h3879
                    }
                };
                let a3881 = 1u64;
                nat_eq(a3880, a3881)
            };
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h3882, _, _, _, _, _, _, _, _, _) => {
                    h3882
                }
            }, match v1.clone() {
                Adt1::C0(h3883, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3883
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3884, _, _, _, _, _, _, _, _, _) => {
                    h3884
                }
            }, match v1.clone() {
                Adt1::C0(_, _, _, _, h3885, _, _, _, _, _, _, _) => {
                    h3885
                }
            });
            let v8: Adt6 = if v6 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, h3886, _, _, _, _, _, _, _, _) => {
                        h3886
                    }
                }, match v1.clone() {
                    Adt1::C0(_, h3887, _, _, _, _, _, _, _, _, _, _) => {
                        h3887
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3888, _, _, _, _, _, _, _, _) => {
                        h3888
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3889, _, _, _, _, _, _) => {
                        h3889
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v10: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3890, _, _, _, _, _, _, _) => {
                        h3890
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, h3891, _, _, _, _, _, _, _, _) => {
                        h3891
                    }
                }
            };
            let v11: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h3892, _, _, _, _, _, _) => {
                        h3892
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h3893, _, _, _, _, _, _, _) => {
                        h3893
                    }
                }
            };
            let v12: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, h3894, _, _, _, _, _, _, _, _) => {
                        h3894
                    }
                }
            } else {
                1u64
            };
            let v13: u64 = match v5.clone() {
                Adt2::C0(h3895, _, _, _, _, _, _, _, _, _, _) => {
                    h3895
                }
            };
            let v14: u64 = match v4.clone() {
                Adt2::C0(h3896, _, _, _, _, _, _, _, _, _, _) => {
                    h3896
                }
            };
            let v15: u64 = {
                let a3897 = v13;
                let a3898 = 0u64;
                if nat_eq(a3897, a3898) {
                    4u64
                } else {
                    2u64
                }
            };
            let v16: u64 = {
                let a3900 = match v5.clone() {
                    Adt2::C0(_, h3899, _, _, _, _, _, _, _, _, _) => {
                        h3899
                    }
                };
                let a3901 = v15;
                nat_mul(a3900, a3901)?
            };
            let v17: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3902, _, _, _) => {
                        h3902
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, h3903, _, _, _, _) => {
                        h3903
                    }
                }
            };
            let v18: u64 = if v6 {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3904, _, _) => {
                        h3904
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h3905, _, _, _) => {
                        h3905
                    }
                }
            };
            let a3907 = match v1 {
                Adt1::C0(_, _, _, _, _, _, h3906, _, _, _, _, _) => {
                    h3906
                }
            };
            let a3908 = 1u64;
            if if if if if if if if if if nat_le(a3907, a3908) {
                let a3909 = v13;
                let a3910 = 0u64;
                if nat_eq(a3909, a3910) {
                    true
                } else {
                    let a3911 = v13;
                    let a3912 = 1u64;
                    if nat_eq(a3911, a3912) {
                        let a3913 = v14;
                        let a3914 = 1u64;
                        nat_eq(a3913, a3914)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a3915 = v14;
                let a3916 = 0u64;
                if nat_eq(a3915, a3916) {
                    true
                } else {
                    let a3917 = v14;
                    let a3918 = 1u64;
                    nat_eq(a3917, a3918)
                }
            } else {
                false
            } {
                let a3920 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3919, _, _, _, _, _) => {
                        h3919
                    }
                };
                let a3921 = v15;
                nat_eq(a3920, a3921)
            } else {
                false
            } {
                let a3929 = match v4.clone() {
                    Adt2::C0(_, h3922, _, _, _, _, _, _, _, _, _) => {
                        h3922
                    }
                };
                let a3930 = {
                    let a3927 = v10;
                    let a3928 = {
                        let a3925 = match v7.clone() {
                            Adt6::C0(h3923, _, _, _) => {
                                h3923
                            }
                        };
                        let a3926 = match v8.clone() {
                            Adt6::C0(h3924, _, _, _) => {
                                h3924
                            }
                        };
                        nat_mul(a3925, a3926)?
                    };
                    nat_mul(a3927, a3928)?
                };
                nat_eq(a3929, a3930)
            } else {
                false
            } {
                let a3933 = match v4.clone() {
                    Adt2::C0(_, _, h3931, _, _, _, _, _, _, _, _) => {
                        h3931
                    }
                };
                let a3934 = f100(v7.clone(), match v5.clone() {
                    Adt2::C0(_, h3932, _, _, _, _, _, _, _, _, _) => {
                        h3932
                    }
                })?;
                nat_eq(a3933, a3934)
            } else {
                false
            } {
                let a3936 = 0u64;
                let a3937 = match v4.clone() {
                    Adt2::C0(_, _, h3935, _, _, _, _, _, _, _, _) => {
                        h3935
                    }
                };
                nat_lt(a3936, a3937)
            } else {
                false
            } {
                if v6 {
                    let a3939 = match v2 {
                        Adt2::C0(_, _, _, h3938, _, _, _, _, _, _, _) => {
                            h3938
                        }
                    };
                    let a3940 = v10;
                    if if if nat_eq(a3939, a3940) {
                        let a3942 = match v4.clone() {
                            Adt2::C0(_, _, _, h3941, _, _, _, _, _, _, _) => {
                                h3941
                            }
                        };
                        let a3943 = f100(v8.clone(), v12)?;
                        nat_eq(a3942, a3943)
                    } else {
                        false
                    } {
                        let a3945 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3944, _, _, _, _, _, _) => {
                                h3944
                            }
                        };
                        let a3946 = v11;
                        nat_eq(a3945, a3946)
                    } else {
                        false
                    } {
                        let a3948 = 0u64;
                        let a3949 = match v4.clone() {
                            Adt2::C0(_, _, _, h3947, _, _, _, _, _, _, _) => {
                                h3947
                            }
                        };
                        nat_lt(a3948, a3949)
                    } else {
                        false
                    }
                } else {
                    let a3951 = match v2 {
                        Adt2::C0(_, _, h3950, _, _, _, _, _, _, _, _) => {
                            h3950
                        }
                    };
                    let a3952 = v10;
                    if if if nat_eq(a3951, a3952) {
                        let a3954 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h3953, _, _, _, _, _, _) => {
                                h3953
                            }
                        };
                        let a3955 = 1u64;
                        nat_eq(a3954, a3955)
                    } else {
                        false
                    } {
                        let a3957 = match v4.clone() {
                            Adt2::C0(_, _, _, h3956, _, _, _, _, _, _, _) => {
                                h3956
                            }
                        };
                        let a3958 = v11;
                        nat_eq(a3957, a3958)
                    } else {
                        false
                    } {
                        let a3960 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3959, _, _, _, _, _, _) => {
                                h3959
                            }
                        };
                        let a3961 = 1u64;
                        nat_eq(a3960, a3961)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f52(match v5.clone() {
                    Adt2::C0(_, h3962, _, _, _, _, _, _, _, _, _) => {
                        h3962
                    }
                }, v12, v10, v11) {
                    true
                } else {
                    let a3982 = {
                        let a3979 = {
                            let a3977 = {
                                let a3971 = {
                                    let a3965 = {
                                        let a3963 = v11;
                                        let a3964 = 1u64;
                                        nat_sub(a3963, a3964)
                                    };
                                    let a3966 = v18;
                                    nat_mul(a3965, a3966)?
                                };
                                let a3972 = {
                                    let a3969 = {
                                        let a3967 = v10;
                                        let a3968 = 1u64;
                                        nat_sub(a3967, a3968)
                                    };
                                    let a3970 = v17;
                                    nat_mul(a3969, a3970)?
                                };
                                nat_add(a3971, a3972)?
                            };
                            let a3978 = {
                                let a3975 = {
                                    let a3973 = v12;
                                    let a3974 = 1u64;
                                    nat_sub(a3973, a3974)
                                };
                                let a3976 = v16;
                                nat_mul(a3975, a3976)?
                            };
                            nat_add(a3977, a3978)?
                        };
                        let a3980 = v16;
                        nat_add(a3979, a3980)?
                    };
                    let a3983 = match v5.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h3981, _) => {
                            h3981
                        }
                    };
                    nat_le(a3982, a3983)
                }
            } else {
                false
            } {
                f35(v5.clone(), v4.clone())?
            } else {
                false
            } {
                let v19: Adt3 = if v6 {
                    f27(v4.clone(), 1u64)
                } else {
                    Adt3::C0(match v4.clone() {
                        Adt2::C0(_, h3984, _, _, _, _, _, _, _, _, _) => {
                            h3984
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3985, _, _, _, _, _, _, _, _) => {
                            h3985
                        }
                    }, 1u64, match v4.clone() {
                        Adt2::C0(_, _, _, h3986, _, _, _, _, _, _, _) => {
                            h3986
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, h3987, _, _, _, _, _) => {
                            h3987
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, h3988, _, _, _, _) => {
                            h3988
                        }
                    }, 0u64, match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h3989, _, _, _) => {
                            h3989
                        }
                    })
                };
                if v0 {
                    Ok(Adt10::C4({
                        let a3996 = f23(v5.clone(), 1u64)?;
                        let a3997 = {
                            let a3994 = {
                                let a3992 = match v7.clone() {
                                    Adt6::C0(h3990, _, _, _) => {
                                        h3990
                                    }
                                };
                                let a3993 = match v8.clone() {
                                    Adt6::C0(h3991, _, _, _) => {
                                        h3991
                                    }
                                };
                                nat_mul(a3992, a3993)?
                            };
                            let a3995 = 2u64;
                            nat_add(a3994, a3995)?
                        };
                        nat_mul(a3996, a3997)?
                    }, 4u64, v19, f42(v6, v5), Adt4::C42(v7, v8)))
                } else {
                    Ok(Adt10::C4(f23(v4, 1u64)?, {
                        let a3998 = v14;
                        let a3999 = 0u64;
                        if nat_eq(a3998, a3999) {
                            4u64
                        } else {
                            2u64
                        }
                    }, Adt3::C0(match v5 {
                        Adt2::C0(_, h4000, _, _, _, _, _, _, _, _, _) => {
                            h4000
                        }
                    }, v12, 1u64, v11, v15, v16, 0u64, 0u64), v19, Adt4::C27(Adt5::C0(v7, v8, v9, v17, v18, v13, 1u64, v14))))
                }
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f125(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4001 = v2;
    match m4001 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h4002, _, _, _, _, _, _, _, _, _) => {
                    h4002
                }
            }, match v0.clone() {
                Adt1::C0(h4003, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4003
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h4004, _, _, _, _, _, _, _, _) => {
                    h4004
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h4005, _, _, _, _, _) => {
                    h4005
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h4006, _, _, _, _, _, _, _, _) => {
                    h4006
                }
            }, match v0.clone() {
                Adt1::C0(_, h4007, _, _, _, _, _, _, _, _, _, _) => {
                    h4007
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h4008, _, _, _, _, _, _, _) => {
                    h4008
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h4009, _, _, _, _) => {
                    h4009
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h4010, _, _, _, _, _, _, _) => {
                    h4010
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h4011, _, _, _, _, _, _, _, _, _) => {
                    h4011
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h4012, _, _, _, _, _, _) => {
                    h4012
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h4013, _, _, _) => {
                    h4013
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h4014, _, _) => {
                    h4014
                }
            };
            let v9: u64 = f100(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h4015, _, _, _, _, _, _, _) => {
                    h4015
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h4016, _, _, _, _, _, _, _, _, _, _) => {
                    h4016
                }
            };
            let v11: u64 = {
                let a4024 = v8;
                let a4025 = {
                    let a4022 = match v5.clone() {
                        Adt6::C0(h4017, _, _, _) => {
                            h4017
                        }
                    };
                    let a4023 = {
                        let a4020 = match v6.clone() {
                            Adt6::C0(h4018, _, _, _) => {
                                h4018
                            }
                        };
                        let a4021 = match v7.clone() {
                            Adt6::C0(h4019, _, _, _) => {
                                h4019
                            }
                        };
                        nat_mul(a4020, a4021)?
                    };
                    nat_mul(a4022, a4023)?
                };
                nat_mul(a4024, a4025)?
            };
            let a4027 = match v4.clone() {
                Adt2::C0(h4026, _, _, _, _, _, _, _, _, _, _) => {
                    h4026
                }
            };
            let a4028 = 0u64;
            if if if if if if if if if if if if if nat_eq(a4027, a4028) {
                let a4029 = v10;
                let a4030 = 0u64;
                if nat_eq(a4029, a4030) {
                    true
                } else {
                    let a4031 = v10;
                    let a4032 = 1u64;
                    nat_eq(a4031, a4032)
                }
            } else {
                false
            } {
                let a4033 = 0u64;
                let a4034 = v8;
                nat_lt(a4033, a4034)
            } else {
                false
            } {
                let a4035 = v8;
                let a4036 = 2147483648u64;
                nat_lt(a4035, a4036)
            } else {
                false
            } {
                let a4038 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h4037, _, _, _, _, _) => {
                        h4037
                    }
                };
                let a4039 = 4u64;
                nat_eq(a4038, a4039)
            } else {
                false
            } {
                let a4040 = 0u64;
                let a4041 = v9;
                nat_lt(a4040, a4041)
            } else {
                false
            } {
                let a4043 = 0u64;
                let a4044 = match v3.clone() {
                    Adt2::C0(_, _, h4042, _, _, _, _, _, _, _, _) => {
                        h4042
                    }
                };
                nat_lt(a4043, a4044)
            } else {
                false
            } {
                let a4046 = 0u64;
                let a4047 = match v3.clone() {
                    Adt2::C0(_, _, _, h4045, _, _, _, _, _, _, _) => {
                        h4045
                    }
                };
                nat_lt(a4046, a4047)
            } else {
                false
            } {
                let a4049 = match v3.clone() {
                    Adt2::C0(_, h4048, _, _, _, _, _, _, _, _, _) => {
                        h4048
                    }
                };
                let a4050 = v11;
                nat_eq(a4049, a4050)
            } else {
                false
            } {
                let a4053 = match v3.clone() {
                    Adt2::C0(_, _, h4051, _, _, _, _, _, _, _, _) => {
                        h4051
                    }
                };
                let a4054 = f100(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h4052, _, _, _, _, _, _, _, _, _) => {
                        h4052
                    }
                })?;
                nat_eq(a4053, a4054)
            } else {
                false
            } {
                let a4057 = match v3.clone() {
                    Adt2::C0(_, _, _, h4055, _, _, _, _, _, _, _) => {
                        h4055
                    }
                };
                let a4058 = f100(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h4056, _, _, _, _, _, _, _, _) => {
                        h4056
                    }
                })?;
                nat_eq(a4057, a4058)
            } else {
                false
            } {
                let a4066 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h4059, _, _, _, _, _, _) => {
                        h4059
                    }
                };
                let a4067 = {
                    let a4064 = v9;
                    let a4065 = {
                        let a4061 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h4060, _, _, _, _, _, _) => {
                                h4060
                            }
                        };
                        let a4062 = v8;
                        let a4063 = 0u64;
                        nat_quot(a4061, a4062, a4063)
                    };
                    nat_mul(a4064, a4065)?
                };
                nat_eq(a4066, a4067)
            } else {
                false
            } {
                f35(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
                    let a4068 = v10;
                    let a4069 = 0u64;
                    if nat_eq(a4068, a4069) {
                        4u64
                    } else {
                        2u64
                    }
                }, f27(v4.clone(), 1u64), f27(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4070, _, _) => {
                        h4070
                    }
                }, {
                    let a4072 = v8;
                    let a4073 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4071, _, _) => {
                            h4071
                        }
                    };
                    nat_mul(a4072, a4073)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f126(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4074 = v1;
    match m4074 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f232(v0, v3, v2)
        }
    }
}

pub fn f127(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4076 = match v0.clone() {
        Adt2::C0(h4075, _, _, _, _, _, _, _, _, _, _) => {
            h4075
        }
    };
    let a4077 = 1u64;
    if if nat_eq(a4076, a4077) {
        true
    } else {
        let a4079 = match v0.clone() {
            Adt2::C0(h4078, _, _, _, _, _, _, _, _, _, _) => {
                h4078
            }
        };
        let a4080 = 30u64;
        nat_eq(a4079, a4080)
    } {
        let a4082 = match v1.clone() {
            Adt2::C0(h4081, _, _, _, _, _, _, _, _, _, _) => {
                h4081
            }
        };
        let a4083 = 1u64;
        if if if if if if if if if if if nat_eq(a4082, a4083) {
            let a4086 = match v0.clone() {
                Adt2::C0(_, h4084, _, _, _, _, _, _, _, _, _) => {
                    h4084
                }
            };
            let a4087 = match v1.clone() {
                Adt2::C0(_, h4085, _, _, _, _, _, _, _, _, _) => {
                    h4085
                }
            };
            nat_eq(a4086, a4087)
        } else {
            false
        } {
            let a4089 = 0u64;
            let a4090 = match v1.clone() {
                Adt2::C0(_, _, h4088, _, _, _, _, _, _, _, _) => {
                    h4088
                }
            };
            nat_lt(a4089, a4090)
        } else {
            false
        } {
            let a4093 = match v1.clone() {
                Adt2::C0(_, _, h4091, _, _, _, _, _, _, _, _) => {
                    h4091
                }
            };
            let a4094 = match v1.clone() {
                Adt2::C0(_, _, _, h4092, _, _, _, _, _, _, _) => {
                    h4092
                }
            };
            nat_eq(a4093, a4094)
        } else {
            false
        } {
            let a4096 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4095, _, _, _, _, _, _) => {
                    h4095
                }
            };
            let a4097 = 1u64;
            nat_eq(a4096, a4097)
        } else {
            false
        } {
            let a4105 = match v0.clone() {
                Adt2::C0(_, _, h4098, _, _, _, _, _, _, _, _) => {
                    h4098
                }
            };
            let a4106 = {
                let a4103 = {
                    let a4101 = match v1.clone() {
                        Adt2::C0(_, _, h4099, _, _, _, _, _, _, _, _) => {
                            h4099
                        }
                    };
                    let a4102 = match v1.clone() {
                        Adt2::C0(_, _, h4100, _, _, _, _, _, _, _, _) => {
                            h4100
                        }
                    };
                    nat_add(a4101, a4102)?
                };
                let a4104 = 1u64;
                nat_sub(a4103, a4104)
            };
            nat_eq(a4105, a4106)
        } else {
            false
        } {
            let a4108 = 0u64;
            let a4109 = match v0.clone() {
                Adt2::C0(_, _, _, h4107, _, _, _, _, _, _, _) => {
                    h4107
                }
            };
            nat_lt(a4108, a4109)
        } else {
            false
        } {
            let a4111 = 0u64;
            let a4112 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4110, _, _, _, _, _, _) => {
                    h4110
                }
            };
            nat_lt(a4111, a4112)
        } else {
            false
        } {
            let a4114 = 2u64;
            let a4115 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h4113, _, _, _, _, _) => {
                    h4113
                }
            };
            nat_le(a4114, a4115)
        } else {
            false
        } {
            let a4117 = 2u64;
            let a4118 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h4116, _, _, _, _, _) => {
                    h4116
                }
            };
            nat_le(a4117, a4118)
        } else {
            false
        } {
            f35(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f23(v1.clone(), 1u64)?, 2u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f128(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4119 = v1;
    match m4119 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f233(v0, v3, v2)
        }
    }
}

pub fn f129(v0: u64) -> bool {
    let a4120 = v0;
    let a4121 = 1u64;
    if if nat_eq(a4120, a4121) {
        true
    } else {
        let a4122 = v0;
        let a4123 = 33u64;
        nat_eq(a4122, a4123)
    } {
        true
    } else {
        let a4124 = v0;
        let a4125 = 34u64;
        nat_eq(a4124, a4125)
    }
}

pub fn f130(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f23(v0.clone(), 1u64)?;
    if if if if if f51(v1, v2.clone()) {
        let a4126 = v3;
        let a4127 = f23(v2.clone(), 1u64)?;
        nat_eq(a4126, a4127)
    } else {
        false
    } {
        f31(v0.clone(), v3)?
    } else {
        false
    } {
        f31(v2.clone(), v3)?
    } else {
        false
    } {
        let a4128 = v3;
        let a4129 = 0u64;
        if nat_eq(a4128, a4129) {
            true
        } else {
            let a4131 = 4u64;
            let a4132 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h4130, _, _, _, _, _) => {
                    h4130
                }
            };
            nat_le(a4131, a4132)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4133 = 2u64;
            let a4134 = v3;
            nat_mul(a4133, a4134)?
        }, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f131(v0: u64) -> bool {
    if f234(v0) {
        !f235(v0)
    } else {
        false
    }
}

pub fn f132(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4135 = v1;
    match m4135 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m4137 = f226(match v0.clone() {
                Adt2::C0(h4136, _, _, _, _, _, _, _, _, _, _) => {
                    h4136
                }
            });
            match m4137 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h4138, _) => {
                            h4138
                        }
                    };
                    if if if if if if f51(v3, v2.clone()) {
                        f35(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a4139 = f23(v0.clone(), 1u64)?;
                        let a4140 = f23(v2.clone(), 1u64)?;
                        nat_eq(a4139, a4140)
                    } else {
                        false
                    } {
                        let a4145 = {
                            let a4142 = match v0.clone() {
                                Adt2::C0(_, h4141, _, _, _, _, _, _, _, _, _) => {
                                    h4141
                                }
                            };
                            let a4143 = v5;
                            let a4144 = 1u64;
                            nat_rem(a4142, a4143, a4144)
                        };
                        let a4146 = 0u64;
                        if nat_eq(a4145, a4146) {
                            let a4149 = match v4 {
                                Adt19::C0(_, _, h4147) => {
                                    h4147
                                }
                            };
                            let a4150 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h4148, _, _, _, _, _) => {
                                    h4148
                                }
                            };
                            nat_le(a4149, a4150)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a4151 = v5;
                        let a4152 = 1u64;
                        if nat_eq(a4151, a4152) {
                            let a4154 = 4u64;
                            let a4155 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4153, _, _, _, _, _) => {
                                    h4153
                                }
                            };
                            nat_le(a4154, a4155)
                        } else {
                            let a4157 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h4156, _, _, _, _, _) => {
                                    h4156
                                }
                            };
                            let a4158 = 4u64;
                            nat_eq(a4157, a4158)
                        }
                    } else {
                        false
                    } {
                        let a4163 = {
                            let a4160 = match v2.clone() {
                                Adt2::C0(_, h4159, _, _, _, _, _, _, _, _, _) => {
                                    h4159
                                }
                            };
                            let a4161 = v5;
                            let a4162 = 1u64;
                            nat_rem(a4160, a4161, a4162)
                        };
                        let a4164 = 0u64;
                        if nat_eq(a4163, a4164) {
                            true
                        } else {
                            f22(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v0.clone(), v5)?, 4u64, f27(v0.clone(), v5), f27(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h4165, _, _, _, _, _, _, _, _, _, _) => {
                                h4165
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

pub fn f133(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a4168 = match v1.clone() {
        Adt2::C0(h4166, _, _, _, _, _, _, _, _, _, _) => {
            h4166
        }
    };
    let a4169 = match v3.clone() {
        Adt2::C0(h4167, _, _, _, _, _, _, _, _, _, _) => {
            h4167
        }
    };
    if nat_eq(a4168, a4169) {
        f236(v0, v1, v2, v3)
    } else {
        if if if if f235(match v1.clone() {
            Adt2::C0(h4170, _, _, _, _, _, _, _, _, _, _) => {
                h4170
            }
        }) {
            f235(match v3.clone() {
                Adt2::C0(h4171, _, _, _, _, _, _, _, _, _, _) => {
                    h4171
                }
            })
        } else {
            false
        } {
            let a4172 = v0;
            let a4173 = 33u64;
            if nat_eq(a4172, a4173) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a4174 = f23(v1.clone(), 1u64)?;
            let a4175 = f23(v3.clone(), 1u64)?;
            nat_eq(a4174, a4175)
        } else {
            false
        } {
            let m4177 = f226(match v3.clone() {
                Adt2::C0(h4176, _, _, _, _, _, _, _, _, _, _) => {
                    h4176
                }
            });
            match m4177 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h4178, _, _, _, _, _, _, _, _, _, _) => {
                            h4178
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h4179, _, _, _, _, _, _, _, _, _, _) => {
                            h4179
                        }
                    };
                    let v7: bool = {
                        let a4180 = v5;
                        let a4181 = 43u64;
                        if nat_eq(a4180, a4181) {
                            true
                        } else {
                            let a4182 = v6;
                            let a4183 = 43u64;
                            nat_eq(a4182, a4183)
                        }
                    };
                    Ok(Adt10::C4({
                        let a4184 = f23(v3.clone(), 1u64)?;
                        let a4185 = if v7 {
                            f174(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a4184, a4185)?
                    }, match v4 {
                        Adt19::C0(_, _, h4186) => {
                            h4186
                        }
                    }, f27(v1, 1u64), f27(v3, 1u64), if v7 {
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

pub fn f134(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4188 = match v0.clone() {
        Adt2::C0(h4187, _, _, _, _, _, _, _, _, _, _) => {
            h4187
        }
    };
    let a4189 = 0u64;
    if if if if nat_eq(a4188, a4189) {
        true
    } else {
        let a4191 = match v0.clone() {
            Adt2::C0(h4190, _, _, _, _, _, _, _, _, _, _) => {
                h4190
            }
        };
        let a4192 = 1u64;
        nat_eq(a4191, a4192)
    } {
        true
    } else {
        let a4194 = match v0.clone() {
            Adt2::C0(h4193, _, _, _, _, _, _, _, _, _, _) => {
                h4193
            }
        };
        let a4195 = 25u64;
        if nat_eq(a4194, a4195) {
            true
        } else {
            let a4197 = match v0.clone() {
                Adt2::C0(h4196, _, _, _, _, _, _, _, _, _, _) => {
                    h4196
                }
            };
            let a4198 = 26u64;
            nat_eq(a4197, a4198)
        }
    } {
        true
    } else {
        let a4200 = match v0.clone() {
            Adt2::C0(h4199, _, _, _, _, _, _, _, _, _, _) => {
                h4199
            }
        };
        let a4201 = 30u64;
        nat_eq(a4200, a4201)
    } {
        let a4204 = match v0.clone() {
            Adt2::C0(h4202, _, _, _, _, _, _, _, _, _, _) => {
                h4202
            }
        };
        let a4205 = match v1.clone() {
            Adt2::C0(h4203, _, _, _, _, _, _, _, _, _, _) => {
                h4203
            }
        };
        if if nat_eq(a4204, a4205) {
            f237(match v1.clone() {
                Adt2::C0(_, h4206, _, _, _, _, _, _, _, _, _) => {
                    h4206
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h4207, _, _, _, _, _, _, _, _) => {
                    h4207
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h4208, _, _, _, _, _, _, _) => {
                    h4208
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h4209, _, _, _, _, _, _) => {
                    h4209
                }
            }, match v0.clone() {
                Adt2::C0(_, h4210, _, _, _, _, _, _, _, _, _) => {
                    h4210
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h4211, _, _, _, _, _, _, _, _) => {
                    h4211
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h4212, _, _, _, _, _, _, _) => {
                    h4212
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h4213, _, _, _, _, _, _) => {
                    h4213
                }
            })
        } else {
            false
        } {
            let m4215 = f226(match v0.clone() {
                Adt2::C0(h4214, _, _, _, _, _, _, _, _, _, _) => {
                    h4214
                }
            });
            match m4215 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f23(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h4216) => {
                            h4216
                        }
                    }, f27(v0, 1u64), f27(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f135(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m4217 = v2;
    match m4217 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a4218 = v0;
            let a4219 = 4u64;
            if if if nat_lt(a4218, a4219) {
                let a4222 = match v1.clone() {
                    Adt2::C0(h4220, _, _, _, _, _, _, _, _, _, _) => {
                        h4220
                    }
                };
                let a4223 = match v4.clone() {
                    Adt2::C0(h4221, _, _, _, _, _, _, _, _, _, _) => {
                        h4221
                    }
                };
                if nat_eq(a4222, a4223) {
                    let a4226 = match v1.clone() {
                        Adt2::C0(h4224, _, _, _, _, _, _, _, _, _, _) => {
                            h4224
                        }
                    };
                    let a4227 = match v3.clone() {
                        Adt2::C0(h4225, _, _, _, _, _, _, _, _, _, _) => {
                            h4225
                        }
                    };
                    nat_eq(a4226, a4227)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f238(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h4228, _, _, _, _, _, _, _, _, _) => {
                        h4228
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h4229, _, _, _, _, _, _, _, _, _) => {
                        h4229
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h4230, _, _, _, _, _, _, _, _, _) => {
                        h4230
                    }
                })? {
                    f238(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h4231, _, _, _, _, _, _, _, _) => {
                            h4231
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h4232, _, _, _, _, _, _, _, _) => {
                            h4232
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h4233, _, _, _, _, _, _, _, _) => {
                            h4233
                        }
                    })?
                } else {
                    false
                } {
                    f238(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h4234, _, _, _, _, _, _, _) => {
                            h4234
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h4235, _, _, _, _, _, _, _) => {
                            h4235
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h4236, _, _, _, _, _, _, _) => {
                            h4236
                        }
                    })?
                } else {
                    false
                } {
                    f238(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h4237, _, _, _, _, _, _) => {
                            h4237
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h4238, _, _, _, _, _, _) => {
                            h4238
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h4239, _, _, _, _, _, _) => {
                            h4239
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m4241 = f226(match v1.clone() {
                    Adt2::C0(h4240, _, _, _, _, _, _, _, _, _, _) => {
                        h4240
                    }
                });
                match m4241 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h4242, _) => {
                                h4242
                            }
                        };
                        let a4247 = {
                            let a4244 = match v1.clone() {
                                Adt2::C0(_, h4243, _, _, _, _, _, _, _, _, _) => {
                                    h4243
                                }
                            };
                            let a4245 = v6;
                            let a4246 = 1u64;
                            nat_rem(a4244, a4245, a4246)
                        };
                        let a4248 = 0u64;
                        if if if nat_eq(a4247, a4248) {
                            let a4253 = {
                                let a4250 = match v4.clone() {
                                    Adt2::C0(_, h4249, _, _, _, _, _, _, _, _, _) => {
                                        h4249
                                    }
                                };
                                let a4251 = v6;
                                let a4252 = 1u64;
                                nat_rem(a4250, a4251, a4252)
                            };
                            let a4254 = 0u64;
                            nat_eq(a4253, a4254)
                        } else {
                            false
                        } {
                            let a4259 = {
                                let a4256 = match v3.clone() {
                                    Adt2::C0(_, h4255, _, _, _, _, _, _, _, _, _) => {
                                        h4255
                                    }
                                };
                                let a4257 = v6;
                                let a4258 = 1u64;
                                nat_rem(a4256, a4257, a4258)
                            };
                            let a4260 = 0u64;
                            nat_eq(a4259, a4260)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f23(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h4261) => {
                                    h4261
                                }
                            }, f27(v1, v6), f27(v3, v6), Adt4::C2(v0, f27(v4, v6))))
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

pub fn f136(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4263 = match v2.clone() {
        Adt2::C0(h4262, _, _, _, _, _, _, _, _, _, _) => {
            h4262
        }
    };
    let a4264 = 0u64;
    if if nat_eq(a4263, a4264) {
        let a4266 = match v3.clone() {
            Adt2::C0(h4265, _, _, _, _, _, _, _, _, _, _) => {
                h4265
            }
        };
        let a4267 = 0u64;
        nat_eq(a4266, a4267)
    } else {
        false
    } {
        let a4268 = v0;
        let a4269 = 61u64;
        if nat_eq(a4268, a4269) {
            let v4: bool = {
                let a4271 = 0u64;
                let a4272 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4270, _, _, _) => {
                        h4270
                    }
                };
                nat_lt(a4271, a4272)
            };
            if if if if if if f239(match v2.clone() {
                Adt2::C0(_, h4273, _, _, _, _, _, _, _, _, _) => {
                    h4273
                }
            }, match v1.clone() {
                Adt1::C0(h4274, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4274
                }
            }, match v1.clone() {
                Adt1::C0(_, h4275, _, _, _, _, _, _, _, _, _, _) => {
                    h4275
                }
            }, match v3.clone() {
                Adt2::C0(_, h4276, _, _, _, _, _, _, _, _, _) => {
                    h4276
                }
            })? {
                f239(match v2.clone() {
                    Adt2::C0(_, _, h4277, _, _, _, _, _, _, _, _) => {
                        h4277
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4278, _, _, _, _, _, _, _, _, _) => {
                        h4278
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h4279, _, _, _, _, _, _, _, _) => {
                        h4279
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h4280, _, _, _, _, _, _, _, _) => {
                        h4280
                    }
                })?
            } else {
                false
            } {
                f239(match v2.clone() {
                    Adt2::C0(_, _, _, h4281, _, _, _, _, _, _, _) => {
                        h4281
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4282, _, _, _, _, _, _, _) => {
                        h4282
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h4283, _, _, _, _, _, _) => {
                        h4283
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h4284, _, _, _, _, _, _, _) => {
                        h4284
                    }
                })?
            } else {
                false
            } {
                f239(match v2.clone() {
                    Adt2::C0(_, _, _, _, h4285, _, _, _, _, _, _) => {
                        h4285
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h4286, _, _, _, _, _) => {
                        h4286
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h4287, _, _, _, _) => {
                        h4287
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h4288, _, _, _, _, _, _) => {
                        h4288
                    }
                })?
            } else {
                false
            } {
                let a4290 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h4289, _, _, _) => {
                        h4289
                    }
                };
                let a4291 = 4294967295u64;
                nat_le(a4290, a4291)
            } else {
                false
            } {
                if v4 {
                    !f52(match v2.clone() {
                        Adt2::C0(_, h4292, _, _, _, _, _, _, _, _, _) => {
                            h4292
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h4293, _, _, _, _, _, _, _, _) => {
                            h4293
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h4294, _, _, _, _, _, _, _) => {
                            h4294
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h4295, _, _, _, _, _, _) => {
                            h4295
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h4296, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4296
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h4297, _, _, _, _, _, _, _, _, _) => {
                        h4297
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h4298, _, _, _, _, _, _, _) => {
                        h4298
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h4299, _, _, _, _, _) => {
                        h4299
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a4300 = v0;
            let a4301 = 62u64;
            if nat_eq(a4300, a4301) {
                let a4303 = match v1.clone() {
                    Adt1::C0(h4302, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4302
                    }
                };
                let a4304 = 2147483648u64;
                if if if if if nat_lt(a4303, a4304) {
                    let a4306 = match v1.clone() {
                        Adt1::C0(_, h4305, _, _, _, _, _, _, _, _, _, _) => {
                            h4305
                        }
                    };
                    let a4307 = 2147483648u64;
                    nat_lt(a4306, a4307)
                } else {
                    false
                } {
                    let a4310 = match v1.clone() {
                        Adt1::C0(h4308, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4308
                        }
                    };
                    let a4311 = match v2.clone() {
                        Adt2::C0(_, h4309, _, _, _, _, _, _, _, _, _) => {
                            h4309
                        }
                    };
                    nat_lt(a4310, a4311)
                } else {
                    false
                } {
                    let a4314 = match v1.clone() {
                        Adt1::C0(_, h4312, _, _, _, _, _, _, _, _, _, _) => {
                            h4312
                        }
                    };
                    let a4315 = match v2.clone() {
                        Adt2::C0(_, h4313, _, _, _, _, _, _, _, _, _) => {
                            h4313
                        }
                    };
                    nat_lt(a4314, a4315)
                } else {
                    false
                } {
                    let a4318 = match v2.clone() {
                        Adt2::C0(_, _, h4316, _, _, _, _, _, _, _, _) => {
                            h4316
                        }
                    };
                    let a4319 = match v3.clone() {
                        Adt2::C0(_, _, h4317, _, _, _, _, _, _, _, _) => {
                            h4317
                        }
                    };
                    if if if nat_eq(a4318, a4319) {
                        let a4322 = match v2.clone() {
                            Adt2::C0(_, _, _, h4320, _, _, _, _, _, _, _) => {
                                h4320
                            }
                        };
                        let a4323 = match v3.clone() {
                            Adt2::C0(_, _, _, h4321, _, _, _, _, _, _, _) => {
                                h4321
                            }
                        };
                        nat_eq(a4322, a4323)
                    } else {
                        false
                    } {
                        let a4326 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h4324, _, _, _, _, _, _) => {
                                h4324
                            }
                        };
                        let a4327 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h4325, _, _, _, _, _, _) => {
                                h4325
                            }
                        };
                        nat_eq(a4326, a4327)
                    } else {
                        false
                    } {
                        let a4336 = match v3.clone() {
                            Adt2::C0(_, h4328, _, _, _, _, _, _, _, _, _) => {
                                h4328
                            }
                        };
                        let a4337 = {
                            let a4334 = {
                                let a4331 = match v2.clone() {
                                    Adt2::C0(_, h4329, _, _, _, _, _, _, _, _, _) => {
                                        h4329
                                    }
                                };
                                let a4332 = match v1.clone() {
                                    Adt1::C0(h4330, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h4330
                                    }
                                };
                                nat_add(a4331, a4332)?
                            };
                            let a4335 = match v1.clone() {
                                Adt1::C0(_, h4333, _, _, _, _, _, _, _, _, _, _) => {
                                    h4333
                                }
                            };
                            nat_add(a4334, a4335)?
                        };
                        nat_eq(a4336, a4337)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h4338, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4338
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a4339 = v0;
                let a4340 = 63u64;
                if nat_eq(a4339, a4340) {
                    if if if if if f240(match v2.clone() {
                        Adt2::C0(_, h4341, _, _, _, _, _, _, _, _, _) => {
                            h4341
                        }
                    }, match v1.clone() {
                        Adt1::C0(h4342, _, _, _, _, _, _, _, _, _, _, _) => {
                            h4342
                        }
                    })? {
                        f240(match v2.clone() {
                            Adt2::C0(_, _, h4343, _, _, _, _, _, _, _, _) => {
                                h4343
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4344, _, _, _, _, _, _, _, _, _, _) => {
                                h4344
                            }
                        })?
                    } else {
                        false
                    } {
                        f240(match v2.clone() {
                            Adt2::C0(_, _, _, h4345, _, _, _, _, _, _, _) => {
                                h4345
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4346, _, _, _, _, _, _, _, _, _) => {
                                h4346
                            }
                        })?
                    } else {
                        false
                    } {
                        f240(match v2.clone() {
                            Adt2::C0(_, _, _, _, h4347, _, _, _, _, _, _) => {
                                h4347
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h4348, _, _, _, _, _, _, _, _) => {
                                h4348
                            }
                        })?
                    } else {
                        false
                    } {
                        let a4351 = match v2.clone() {
                            Adt2::C0(_, h4349, _, _, _, _, _, _, _, _, _) => {
                                h4349
                            }
                        };
                        let a4352 = match v3.clone() {
                            Adt2::C0(_, h4350, _, _, _, _, _, _, _, _, _) => {
                                h4350
                            }
                        };
                        if if if nat_eq(a4351, a4352) {
                            let a4355 = match v2.clone() {
                                Adt2::C0(_, _, h4353, _, _, _, _, _, _, _, _) => {
                                    h4353
                                }
                            };
                            let a4356 = match v3.clone() {
                                Adt2::C0(_, _, h4354, _, _, _, _, _, _, _, _) => {
                                    h4354
                                }
                            };
                            nat_eq(a4355, a4356)
                        } else {
                            false
                        } {
                            let a4359 = match v2.clone() {
                                Adt2::C0(_, _, _, h4357, _, _, _, _, _, _, _) => {
                                    h4357
                                }
                            };
                            let a4360 = match v3.clone() {
                                Adt2::C0(_, _, _, h4358, _, _, _, _, _, _, _) => {
                                    h4358
                                }
                            };
                            nat_eq(a4359, a4360)
                        } else {
                            false
                        } {
                            let a4363 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h4361, _, _, _, _, _, _) => {
                                    h4361
                                }
                            };
                            let a4364 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h4362, _, _, _, _, _, _) => {
                                    h4362
                                }
                            };
                            nat_eq(a4363, a4364)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h4365, _, _, _, _, _, _, _, _, _, _, _) => {
                                h4365
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h4366, _, _, _, _, _, _, _, _, _, _) => {
                                h4366
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h4367, _, _, _, _, _, _, _, _, _) => {
                                h4367
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h4368, _, _, _, _, _, _, _, _) => {
                                h4368
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

pub fn f137(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4369 = v3;
    match m4369 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a4370 = v0;
                let a4371 = 4u64;
                nat_eq(a4370, a4371)
            };
            let v7: bool = {
                let a4373 = 0u64;
                let a4374 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h4372, _, _, _, _, _, _, _) => {
                        h4372
                    }
                };
                nat_lt(a4373, a4374)
            };
            let v8: u64 = f23(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h4375, _, _, _, _, _, _, _, _, _) => {
                    h4375
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h4376, _, _, _, _, _, _, _, _) => {
                    h4376
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h4377, _, _, _, _, _, _, _) => {
                    h4377
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h4378, _, _, _, _, _, _) => {
                    h4378
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h4379, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4379
                }
            }, match v1.clone() {
                Adt1::C0(_, h4380, _, _, _, _, _, _, _, _, _, _) => {
                    h4380
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h4381, _, _, _, _, _, _, _, _, _) => {
                    h4381
                }
            });
            if if if if if if if if if if if if v6 {
                let a4383 = match v2.clone() {
                    Adt2::C0(h4382, _, _, _, _, _, _, _, _, _, _) => {
                        h4382
                    }
                };
                let a4384 = 0u64;
                if nat_eq(a4383, a4384) {
                    let a4386 = match v5.clone() {
                        Adt2::C0(h4385, _, _, _, _, _, _, _, _, _, _) => {
                            h4385
                        }
                    };
                    let a4387 = 0u64;
                    nat_eq(a4386, a4387)
                } else {
                    false
                }
            } else {
                let a4389 = match v2.clone() {
                    Adt2::C0(h4388, _, _, _, _, _, _, _, _, _, _) => {
                        h4388
                    }
                };
                let a4390 = 0u64;
                if if nat_eq(a4389, a4390) {
                    true
                } else {
                    let a4392 = match v2.clone() {
                        Adt2::C0(h4391, _, _, _, _, _, _, _, _, _, _) => {
                            h4391
                        }
                    };
                    let a4393 = 26u64;
                    nat_eq(a4392, a4393)
                } {
                    let a4395 = match v5.clone() {
                        Adt2::C0(h4394, _, _, _, _, _, _, _, _, _, _) => {
                            h4394
                        }
                    };
                    let a4396 = 0u64;
                    if nat_eq(a4395, a4396) {
                        true
                    } else {
                        let a4398 = match v5.clone() {
                            Adt2::C0(h4397, _, _, _, _, _, _, _, _, _, _) => {
                                h4397
                            }
                        };
                        let a4399 = 26u64;
                        nat_eq(a4398, a4399)
                    }
                } else {
                    false
                }
            } {
                let a4402 = match v2.clone() {
                    Adt2::C0(h4400, _, _, _, _, _, _, _, _, _, _) => {
                        h4400
                    }
                };
                let a4403 = match v4.clone() {
                    Adt2::C0(h4401, _, _, _, _, _, _, _, _, _, _) => {
                        h4401
                    }
                };
                nat_eq(a4402, a4403)
            } else {
                false
            } {
                f28(v2.clone(), v4.clone())
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
                let a4405 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h4404, _, _, _, _, _) => {
                        h4404
                    }
                };
                let a4406 = 4u64;
                nat_eq(a4405, a4406)
            } else {
                false
            } {
                let a4407 = v8;
                let a4408 = f23(v2.clone(), 1u64)?;
                nat_le(a4407, a4408)
            } else {
                false
            } {
                let a4410 = match v1.clone() {
                    Adt1::C0(_, _, _, h4409, _, _, _, _, _, _, _, _) => {
                        h4409
                    }
                };
                let a4411 = 1073741824u64;
                nat_lt(a4410, a4411)
            } else {
                false
            } {
                let a4413 = match v1.clone() {
                    Adt1::C0(h4412, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4412
                    }
                };
                let a4414 = 4294967295u64;
                if if if if nat_le(a4413, a4414) {
                    let a4416 = match v1.clone() {
                        Adt1::C0(_, h4415, _, _, _, _, _, _, _, _, _, _) => {
                            h4415
                        }
                    };
                    let a4417 = 4294967295u64;
                    nat_le(a4416, a4417)
                } else {
                    false
                } {
                    let a4419 = match v1.clone() {
                        Adt1::C0(_, _, h4418, _, _, _, _, _, _, _, _, _) => {
                            h4418
                        }
                    };
                    let a4420 = 4294967295u64;
                    nat_le(a4419, a4420)
                } else {
                    false
                } {
                    let a4422 = match v1.clone() {
                        Adt1::C0(_, _, _, h4421, _, _, _, _, _, _, _, _) => {
                            h4421
                        }
                    };
                    let a4423 = 4294967295u64;
                    nat_le(a4422, a4423)
                } else {
                    false
                } {
                    let a4425 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h4424, _, _, _, _, _, _, _) => {
                            h4424
                        }
                    };
                    let a4426 = 4294967295u64;
                    nat_le(a4425, a4426)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a4429 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4427) => {
                            h4427
                        }
                    };
                    let a4430 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h4428) => {
                            h4428
                        }
                    };
                    nat_eq(a4429, a4430)
                } else {
                    true
                }
            } else {
                false
            } {
                f241(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h4431, _, _, _, _, _, _, _, _) => {
                        h4431
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4432, _) => {
                        h4432
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
                    let a4433 = v10;
                    let a4434 = v8;
                    nat_add(a4433, a4434)?
                }, 4u64, f27(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h4435, _, _, _, _, _, _, _, _) => {
                        h4435
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f138(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4437 = match v0.clone() {
        Adt2::C0(h4436, _, _, _, _, _, _, _, _, _, _) => {
            h4436
        }
    };
    let a4438 = 0u64;
    if if if nat_eq(a4437, a4438) {
        let a4440 = match v1.clone() {
            Adt2::C0(h4439, _, _, _, _, _, _, _, _, _, _) => {
                h4439
            }
        };
        let a4441 = 0u64;
        nat_eq(a4440, a4441)
    } else {
        false
    } {
        f237(match v0.clone() {
            Adt2::C0(_, h4442, _, _, _, _, _, _, _, _, _) => {
                h4442
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h4443, _, _, _, _, _, _, _, _) => {
                h4443
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h4444, _, _, _, _, _, _, _) => {
                h4444
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h4445, _, _, _, _, _, _) => {
                h4445
            }
        }, match v1.clone() {
            Adt2::C0(_, h4446, _, _, _, _, _, _, _, _, _) => {
                h4446
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h4447, _, _, _, _, _, _, _, _) => {
                h4447
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h4448, _, _, _, _, _, _, _) => {
                h4448
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h4449, _, _, _, _, _, _) => {
                h4449
            }
        })
    } else {
        false
    } {
        let v2: u64 = f23(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a4450 = v2;
            let a4451 = f23(v0.clone(), 1u64)?;
            nat_add(a4450, a4451)?
        }, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f139(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a4453 = match v1.clone() {
        Adt2::C0(h4452, _, _, _, _, _, _, _, _, _, _) => {
            h4452
        }
    };
    let a4454 = 0u64;
    if if if nat_eq(a4453, a4454) {
        true
    } else {
        let a4456 = match v1.clone() {
            Adt2::C0(h4455, _, _, _, _, _, _, _, _, _, _) => {
                h4455
            }
        };
        let a4457 = 1u64;
        nat_eq(a4456, a4457)
    } {
        true
    } else {
        let a4459 = match v1.clone() {
            Adt2::C0(h4458, _, _, _, _, _, _, _, _, _, _) => {
                h4458
            }
        };
        let a4460 = 30u64;
        nat_eq(a4459, a4460)
    } {
        let a4463 = match v1.clone() {
            Adt2::C0(h4461, _, _, _, _, _, _, _, _, _, _) => {
                h4461
            }
        };
        let a4464 = match v2.clone() {
            Adt2::C0(h4462, _, _, _, _, _, _, _, _, _, _) => {
                h4462
            }
        };
        if if if if if nat_eq(a4463, a4464) {
            let a4467 = match v1.clone() {
                Adt2::C0(_, h4465, _, _, _, _, _, _, _, _, _) => {
                    h4465
                }
            };
            let a4468 = match v2.clone() {
                Adt2::C0(_, h4466, _, _, _, _, _, _, _, _, _) => {
                    h4466
                }
            };
            nat_eq(a4467, a4468)
        } else {
            false
        } {
            let a4471 = match v1.clone() {
                Adt2::C0(_, _, _, h4469, _, _, _, _, _, _, _) => {
                    h4469
                }
            };
            let a4472 = match v2.clone() {
                Adt2::C0(_, _, _, h4470, _, _, _, _, _, _, _) => {
                    h4470
                }
            };
            nat_eq(a4471, a4472)
        } else {
            false
        } {
            let a4475 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4473, _, _, _, _, _, _) => {
                    h4473
                }
            };
            let a4476 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4474, _, _, _, _, _, _) => {
                    h4474
                }
            };
            nat_eq(a4475, a4476)
        } else {
            false
        } {
            let a4477 = v0;
            let a4478 = 4u64;
            if nat_eq(a4477, a4478) {
                let a4480 = match v1.clone() {
                    Adt2::C0(_, _, h4479, _, _, _, _, _, _, _, _) => {
                        h4479
                    }
                };
                let a4481 = 1u64;
                if nat_eq(a4480, a4481) {
                    let a4484 = match v1.clone() {
                        Adt2::C0(_, h4482, _, _, _, _, _, _, _, _, _) => {
                            h4482
                        }
                    };
                    let a4485 = match v2.clone() {
                        Adt2::C0(_, _, h4483, _, _, _, _, _, _, _, _) => {
                            h4483
                        }
                    };
                    nat_eq(a4484, a4485)
                } else {
                    false
                }
            } else {
                let a4486 = v0;
                let a4487 = 4u64;
                if if nat_lt(a4486, a4487) {
                    let a4490 = match v1.clone() {
                        Adt2::C0(_, h4488, _, _, _, _, _, _, _, _, _) => {
                            h4488
                        }
                    };
                    let a4491 = match v1.clone() {
                        Adt2::C0(_, _, h4489, _, _, _, _, _, _, _, _) => {
                            h4489
                        }
                    };
                    nat_eq(a4490, a4491)
                } else {
                    false
                } {
                    let a4494 = match v1.clone() {
                        Adt2::C0(_, _, h4492, _, _, _, _, _, _, _, _) => {
                            h4492
                        }
                    };
                    let a4495 = match v2.clone() {
                        Adt2::C0(_, _, h4493, _, _, _, _, _, _, _, _) => {
                            h4493
                        }
                    };
                    nat_eq(a4494, a4495)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m4497 = f226(match v1.clone() {
                Adt2::C0(h4496, _, _, _, _, _, _, _, _, _, _) => {
                    h4496
                }
            });
            match m4497 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h4498) => {
                            h4498
                        }
                    }, f27(v1, 1u64), f27(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f140(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f242(match v1.clone() {
        Adt2::C0(h4499, _, _, _, _, _, _, _, _, _, _) => {
            h4499
        }
    }, v0) {
        let a4502 = match v1.clone() {
            Adt2::C0(h4500, _, _, _, _, _, _, _, _, _, _) => {
                h4500
            }
        };
        let a4503 = match v2.clone() {
            Adt2::C0(h4501, _, _, _, _, _, _, _, _, _, _) => {
                h4501
            }
        };
        nat_eq(a4502, a4503)
    } else {
        false
    } {
        let a4506 = match v1.clone() {
            Adt2::C0(_, h4504, _, _, _, _, _, _, _, _, _) => {
                h4504
            }
        };
        let a4507 = match v2.clone() {
            Adt2::C0(_, h4505, _, _, _, _, _, _, _, _, _) => {
                h4505
            }
        };
        if if if nat_eq(a4506, a4507) {
            let a4510 = match v1.clone() {
                Adt2::C0(_, _, h4508, _, _, _, _, _, _, _, _) => {
                    h4508
                }
            };
            let a4511 = match v2.clone() {
                Adt2::C0(_, _, h4509, _, _, _, _, _, _, _, _) => {
                    h4509
                }
            };
            nat_eq(a4510, a4511)
        } else {
            false
        } {
            let a4514 = match v1.clone() {
                Adt2::C0(_, _, _, h4512, _, _, _, _, _, _, _) => {
                    h4512
                }
            };
            let a4515 = match v2.clone() {
                Adt2::C0(_, _, _, h4513, _, _, _, _, _, _, _) => {
                    h4513
                }
            };
            nat_eq(a4514, a4515)
        } else {
            false
        } {
            let a4518 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4516, _, _, _, _, _, _) => {
                    h4516
                }
            };
            let a4519 = match v2.clone() {
                Adt2::C0(_, _, _, _, h4517, _, _, _, _, _, _) => {
                    h4517
                }
            };
            nat_eq(a4518, a4519)
        } else {
            false
        }
    } else {
        false
    } {
        let m4521 = f226(match v1.clone() {
            Adt2::C0(h4520, _, _, _, _, _, _, _, _, _, _) => {
                h4520
            }
        });
        match m4521 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h4522) => {
                        h4522
                    }
                }, f27(v1.clone(), 1u64), f27(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h4523, _, _, _, _, _, _, _, _, _, _) => {
                        h4523
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f141(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h4524, _, _, _, _, _, _, _, _, _, _, _) => {
                h4524
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h4525, _, _, _, _, _, _, _, _, _) => {
                h4525
            }
        }
    };
    let a4526 = 0u64;
    let a4527 = v4;
    if if nat_lt(a4526, a4527) {
        let a4528 = v4;
        let a4529 = 2147483648u64;
        nat_lt(a4528, a4529)
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
        let v7: u64 = f243(match v5.clone() {
            Adt2::C0(_, _, h4530, _, _, _, _, _, _, _, _) => {
                h4530
            }
        }, v4)?;
        let v8: u64 = f243(match v5.clone() {
            Adt2::C0(_, _, _, h4531, _, _, _, _, _, _, _) => {
                h4531
            }
        }, v4)?;
        let a4533 = match v5.clone() {
            Adt2::C0(h4532, _, _, _, _, _, _, _, _, _, _) => {
                h4532
            }
        };
        let a4534 = 0u64;
        if if if if if nat_eq(a4533, a4534) {
            let a4536 = match v6.clone() {
                Adt2::C0(h4535, _, _, _, _, _, _, _, _, _, _) => {
                    h4535
                }
            };
            let a4537 = 0u64;
            nat_eq(a4536, a4537)
        } else {
            false
        } {
            let a4539 = match v5.clone() {
                Adt2::C0(_, _, _, _, h4538, _, _, _, _, _, _) => {
                    h4538
                }
            };
            let a4540 = 1u64;
            nat_eq(a4539, a4540)
        } else {
            false
        } {
            let a4543 = match v5.clone() {
                Adt2::C0(_, h4541, _, _, _, _, _, _, _, _, _) => {
                    h4541
                }
            };
            let a4544 = match v6.clone() {
                Adt2::C0(_, h4542, _, _, _, _, _, _, _, _, _) => {
                    h4542
                }
            };
            nat_eq(a4543, a4544)
        } else {
            false
        } {
            if v0 {
                let a4546 = match v5.clone() {
                    Adt2::C0(_, _, h4545, _, _, _, _, _, _, _, _) => {
                        h4545
                    }
                };
                let a4547 = 2147483648u64;
                if if if nat_lt(a4546, a4547) {
                    let a4549 = match v5.clone() {
                        Adt2::C0(_, _, _, h4548, _, _, _, _, _, _, _) => {
                            h4548
                        }
                    };
                    let a4550 = 2147483648u64;
                    nat_lt(a4549, a4550)
                } else {
                    false
                } {
                    let a4553 = {
                        let a4551 = v7;
                        let a4552 = v8;
                        nat_mul(a4551, a4552)?
                    };
                    let a4554 = 2147483648u64;
                    nat_le(a4553, a4554)
                } else {
                    false
                } {
                    f244(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a4556 = match v1.clone() {
                    Adt1::C0(h4555, _, _, _, _, _, _, _, _, _, _, _) => {
                        h4555
                    }
                };
                let a4557 = v7;
                if if if if nat_eq(a4556, a4557) {
                    let a4559 = match v1 {
                        Adt1::C0(_, h4558, _, _, _, _, _, _, _, _, _, _) => {
                            h4558
                        }
                    };
                    let a4560 = v8;
                    nat_eq(a4559, a4560)
                } else {
                    false
                } {
                    f245(v7, v8, v4)?
                } else {
                    false
                } {
                    let a4562 = match v6.clone() {
                        Adt2::C0(_, _, h4561, _, _, _, _, _, _, _, _) => {
                            h4561
                        }
                    };
                    let a4563 = v4;
                    if nat_eq(a4562, a4563) {
                        let a4565 = match v6.clone() {
                            Adt2::C0(_, _, _, h4564, _, _, _, _, _, _, _) => {
                                h4564
                            }
                        };
                        let a4566 = v4;
                        nat_eq(a4565, a4566)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a4570 = match v6 {
                        Adt2::C0(_, _, _, _, h4567, _, _, _, _, _, _) => {
                            h4567
                        }
                    };
                    let a4571 = {
                        let a4568 = v7;
                        let a4569 = v8;
                        nat_mul(a4568, a4569)?
                    };
                    nat_eq(a4570, a4571)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f23(v3.clone(), 1u64)?, 4u64, f27(v2, 1u64), f27(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f142(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4573 = match v2.clone() {
        Adt2::C0(h4572, _, _, _, _, _, _, _, _, _, _) => {
            h4572
        }
    };
    let a4574 = 0u64;
    if if if if if nat_eq(a4573, a4574) {
        true
    } else {
        let a4576 = match v2.clone() {
            Adt2::C0(h4575, _, _, _, _, _, _, _, _, _, _) => {
                h4575
            }
        };
        let a4577 = 1u64;
        nat_eq(a4576, a4577)
    } {
        true
    } else {
        let a4579 = match v2.clone() {
            Adt2::C0(h4578, _, _, _, _, _, _, _, _, _, _) => {
                h4578
            }
        };
        let a4580 = 30u64;
        nat_eq(a4579, a4580)
    } {
        let a4583 = match v2.clone() {
            Adt2::C0(h4581, _, _, _, _, _, _, _, _, _, _) => {
                h4581
            }
        };
        let a4584 = match v3.clone() {
            Adt2::C0(h4582, _, _, _, _, _, _, _, _, _, _) => {
                h4582
            }
        };
        nat_eq(a4583, a4584)
    } else {
        false
    } {
        if if f28(v2.clone(), v3.clone()) {
            let a4586 = match v1.clone() {
                Adt1::C0(h4585, _, _, _, _, _, _, _, _, _, _, _) => {
                    h4585
                }
            };
            let a4587 = 4294967296u64;
            nat_lt(a4586, a4587)
        } else {
            false
        } {
            let a4589 = match v1.clone() {
                Adt1::C0(_, h4588, _, _, _, _, _, _, _, _, _, _) => {
                    h4588
                }
            };
            let a4590 = 4294967296u64;
            nat_lt(a4589, a4590)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v3.clone(), 1u64)?, {
            let a4592 = match v2.clone() {
                Adt2::C0(h4591, _, _, _, _, _, _, _, _, _, _) => {
                    h4591
                }
            };
            let a4593 = 0u64;
            if nat_eq(a4592, a4593) {
                4u64
            } else {
                2u64
            }
        }, f27(v2.clone(), 1u64), f27(v3, 1u64), Adt4::C13({
            let a4594 = v0;
            let a4595 = 31u64;
            if nat_eq(a4594, a4595) {
                0u64
            } else {
                let a4596 = v0;
                let a4597 = 47u64;
                if nat_eq(a4596, a4597) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h4598, _, _, _, _, _, _, _, _, _, _) => {
                h4598
            }
        }, match v1.clone() {
            Adt1::C0(h4599, _, _, _, _, _, _, _, _, _, _, _) => {
                h4599
            }
        }, match v1 {
            Adt1::C0(_, h4600, _, _, _, _, _, _, _, _, _, _) => {
                h4600
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f143(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4602 = match v0.clone() {
        Adt2::C0(h4601, _, _, _, _, _, _, _, _, _, _) => {
            h4601
        }
    };
    let a4603 = 0u64;
    if if if nat_eq(a4602, a4603) {
        let a4605 = match v1.clone() {
            Adt2::C0(h4604, _, _, _, _, _, _, _, _, _, _) => {
                h4604
            }
        };
        let a4606 = 0u64;
        nat_eq(a4605, a4606)
    } else {
        false
    } {
        f28(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f23(v1.clone(), 1u64)?, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f144(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a4608 = match v0.clone() {
        Adt2::C0(h4607, _, _, _, _, _, _, _, _, _, _) => {
            h4607
        }
    };
    let a4609 = 0u64;
    if if nat_eq(a4608, a4609) {
        let a4611 = match v1.clone() {
            Adt2::C0(h4610, _, _, _, _, _, _, _, _, _, _) => {
                h4610
            }
        };
        let a4612 = 26u64;
        nat_eq(a4611, a4612)
    } else {
        false
    } {
        let a4614 = match v0.clone() {
            Adt2::C0(_, _, _, h4613, _, _, _, _, _, _, _) => {
                h4613
            }
        };
        let a4615 = 1u64;
        if if if if if if if nat_eq(a4614, a4615) {
            let a4617 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4616, _, _, _, _, _, _) => {
                    h4616
                }
            };
            let a4618 = 1u64;
            nat_eq(a4617, a4618)
        } else {
            false
        } {
            let a4620 = match v0.clone() {
                Adt2::C0(_, h4619, _, _, _, _, _, _, _, _, _) => {
                    h4619
                }
            };
            let a4621 = 2147483647u64;
            nat_le(a4620, a4621)
        } else {
            false
        } {
            let a4624 = match v1.clone() {
                Adt2::C0(_, h4622, _, _, _, _, _, _, _, _, _) => {
                    h4622
                }
            };
            let a4625 = match v0.clone() {
                Adt2::C0(_, _, h4623, _, _, _, _, _, _, _, _) => {
                    h4623
                }
            };
            nat_eq(a4624, a4625)
        } else {
            false
        } {
            let a4627 = match v1.clone() {
                Adt2::C0(_, _, h4626, _, _, _, _, _, _, _, _) => {
                    h4626
                }
            };
            let a4628 = 1u64;
            nat_eq(a4627, a4628)
        } else {
            false
        } {
            let a4630 = match v1.clone() {
                Adt2::C0(_, _, _, h4629, _, _, _, _, _, _, _) => {
                    h4629
                }
            };
            let a4631 = 1u64;
            nat_eq(a4630, a4631)
        } else {
            false
        } {
            let a4633 = match v1.clone() {
                Adt2::C0(_, _, _, _, h4632, _, _, _, _, _, _) => {
                    h4632
                }
            };
            let a4634 = 1u64;
            nat_eq(a4633, a4634)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a4637 = match v0.clone() {
                    Adt2::C0(_, _, h4635, _, _, _, _, _, _, _, _) => {
                        h4635
                    }
                };
                let a4638 = f187(match v0.clone() {
                    Adt2::C0(_, h4636, _, _, _, _, _, _, _, _, _) => {
                        h4636
                    }
                })?;
                nat_mul(a4637, a4638)?
            }, 4u64, f27(v0, 1u64), f27(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f145(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m4639 = v1;
    match m4639 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f192(f27(v0.clone(), 1u64))?;
            let a4641 = match v0.clone() {
                Adt2::C0(h4640, _, _, _, _, _, _, _, _, _, _) => {
                    h4640
                }
            };
            let a4642 = 26u64;
            if if if nat_eq(a4641, a4642) {
                let a4644 = match v3.clone() {
                    Adt2::C0(h4643, _, _, _, _, _, _, _, _, _, _) => {
                        h4643
                    }
                };
                let a4645 = 26u64;
                nat_eq(a4644, a4645)
            } else {
                false
            } {
                let a4647 = match v2.clone() {
                    Adt2::C0(h4646, _, _, _, _, _, _, _, _, _, _) => {
                        h4646
                    }
                };
                let a4648 = 27u64;
                nat_eq(a4647, a4648)
            } else {
                false
            } {
                if if if if if if f28(v0.clone(), v3.clone()) {
                    let a4650 = match v2.clone() {
                        Adt2::C0(_, h4649, _, _, _, _, _, _, _, _, _) => {
                            h4649
                        }
                    };
                    let a4651 = 1u64;
                    nat_eq(a4650, a4651)
                } else {
                    false
                } {
                    let a4653 = match v2.clone() {
                        Adt2::C0(_, _, h4652, _, _, _, _, _, _, _, _) => {
                            h4652
                        }
                    };
                    let a4654 = 1u64;
                    nat_eq(a4653, a4654)
                } else {
                    false
                } {
                    let a4656 = match v2.clone() {
                        Adt2::C0(_, _, _, h4655, _, _, _, _, _, _, _) => {
                            h4655
                        }
                    };
                    let a4657 = 1u64;
                    nat_eq(a4656, a4657)
                } else {
                    false
                } {
                    let a4659 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4658, _, _, _, _, _, _) => {
                            h4658
                        }
                    };
                    let a4660 = 1u64;
                    nat_eq(a4659, a4660)
                } else {
                    false
                } {
                    let a4661 = v4;
                    let a4662 = 9223372036854775808u64;
                    nat_lt(a4661, a4662)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a4665 = 2u64;
                        let a4666 = {
                            let a4663 = v4;
                            let a4664 = 2u64;
                            nat_mul(a4663, a4664)?
                        };
                        nat_add(a4665, a4666)?
                    }, 8u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C18(v4, f27(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f146(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h4667, _, _, _, _, _, _, _, _, _, _) => {
            h4667
        }
    };
    let a4668 = v3;
    let a4669 = 0u64;
    if if nat_eq(a4668, a4669) {
        true
    } else {
        let a4670 = v0;
        let a4671 = 13u64;
        if nat_eq(a4670, a4671) {
            let a4672 = v3;
            let a4673 = 1u64;
            if nat_eq(a4672, a4673) {
                true
            } else {
                let a4674 = v3;
                let a4675 = 30u64;
                nat_eq(a4674, a4675)
            }
        } else {
            false
        }
    } {
        let a4677 = v3;
        let a4678 = match v2.clone() {
            Adt2::C0(h4676, _, _, _, _, _, _, _, _, _, _) => {
                h4676
            }
        };
        if if nat_eq(a4677, a4678) {
            let a4679 = v0;
            let a4680 = 13u64;
            if nat_eq(a4679, a4680) {
                let a4682 = match v2.clone() {
                    Adt2::C0(_, h4681, _, _, _, _, _, _, _, _, _) => {
                        h4681
                    }
                };
                let a4683 = 1u64;
                if if if nat_eq(a4682, a4683) {
                    let a4685 = match v2.clone() {
                        Adt2::C0(_, _, h4684, _, _, _, _, _, _, _, _) => {
                            h4684
                        }
                    };
                    let a4686 = 1u64;
                    nat_eq(a4685, a4686)
                } else {
                    false
                } {
                    let a4688 = match v2.clone() {
                        Adt2::C0(_, _, _, h4687, _, _, _, _, _, _, _) => {
                            h4687
                        }
                    };
                    let a4689 = 1u64;
                    nat_eq(a4688, a4689)
                } else {
                    false
                } {
                    let a4691 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4690, _, _, _, _, _, _) => {
                            h4690
                        }
                    };
                    let a4692 = 1u64;
                    nat_eq(a4691, a4692)
                } else {
                    false
                }
            } else {
                let a4694 = match v2.clone() {
                    Adt2::C0(_, h4693, _, _, _, _, _, _, _, _, _) => {
                        h4693
                    }
                };
                let a4695 = 1u64;
                if if if nat_eq(a4694, a4695) {
                    let a4698 = match v1.clone() {
                        Adt2::C0(_, _, h4696, _, _, _, _, _, _, _, _) => {
                            h4696
                        }
                    };
                    let a4699 = match v2.clone() {
                        Adt2::C0(_, _, h4697, _, _, _, _, _, _, _, _) => {
                            h4697
                        }
                    };
                    nat_eq(a4698, a4699)
                } else {
                    false
                } {
                    let a4702 = match v1.clone() {
                        Adt2::C0(_, _, _, h4700, _, _, _, _, _, _, _) => {
                            h4700
                        }
                    };
                    let a4703 = match v2.clone() {
                        Adt2::C0(_, _, _, h4701, _, _, _, _, _, _, _) => {
                            h4701
                        }
                    };
                    nat_eq(a4702, a4703)
                } else {
                    false
                } {
                    let a4706 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h4704, _, _, _, _, _, _) => {
                            h4704
                        }
                    };
                    let a4707 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h4705, _, _, _, _, _, _) => {
                            h4705
                        }
                    };
                    nat_eq(a4706, a4707)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f27(v1, 1u64);
            Ok(Adt10::C4(f246(v4.clone(), v0, v3)?, {
                let a4708 = v3;
                let a4709 = 0u64;
                if nat_eq(a4708, a4709) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f27(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f147(v0: u64, v1: u64) -> R<Option<u64>> {
    let a4710 = v0;
    let a4711 = 89u64;
    if if nat_eq(a4710, a4711) {
        let a4712 = v1;
        let a4713 = 5u64;
        nat_eq(a4712, a4713)
    } else {
        false
    } {
        Ok(Some(25u64))
    } else {
        let a4714 = v0;
        let a4715 = 89u64;
        if if nat_eq(a4714, a4715) {
            let a4716 = v1;
            let a4717 = 17u64;
            nat_eq(a4716, a4717)
        } else {
            false
        } {
            Ok(Some(26u64))
        } else {
            let a4718 = v0;
            let a4719 = 89u64;
            if if nat_eq(a4718, a4719) {
                let a4720 = v1;
                let a4721 = 16u64;
                nat_eq(a4720, a4721)
            } else {
                false
            } {
                Ok(Some(24u64))
            } else {
                let a4722 = v0;
                let a4723 = 89u64;
                if if nat_eq(a4722, a4723) {
                    let a4724 = v1;
                    let a4725 = 14u64;
                    nat_eq(a4724, a4725)
                } else {
                    false
                } {
                    Ok(Some(23u64))
                } else {
                    let a4726 = v0;
                    let a4727 = 89u64;
                    if if nat_eq(a4726, a4727) {
                        let a4728 = v1;
                        let a4729 = 15u64;
                        nat_eq(a4728, a4729)
                    } else {
                        false
                    } {
                        Ok(Some(22u64))
                    } else {
                        let a4730 = v0;
                        let a4731 = 89u64;
                        if if nat_eq(a4730, a4731) {
                            let a4732 = 8u64;
                            let a4733 = v1;
                            if nat_le(a4732, a4733) {
                                let a4734 = v1;
                                let a4735 = 9u64;
                                nat_le(a4734, a4735)
                            } else {
                                false
                            }
                        } else {
                            false
                        } {
                            Ok(Some({
                                let a4736 = v1;
                                let a4737 = 12u64;
                                nat_add(a4736, a4737)?
                            }))
                        } else {
                            let a4738 = v0;
                            let a4739 = 89u64;
                            if if nat_eq(a4738, a4739) {
                                let a4740 = v1;
                                let a4741 = 4u64;
                                nat_eq(a4740, a4741)
                            } else {
                                false
                            } {
                                Ok(Some(19u64))
                            } else {
                                let a4742 = 10u64;
                                let a4743 = v0;
                                if if nat_le(a4742, a4743) {
                                    let a4744 = v0;
                                    let a4745 = 12u64;
                                    nat_le(a4744, a4745)
                                } else {
                                    false
                                } {
                                    Ok(Some({
                                        let a4746 = v0;
                                        let a4747 = 3u64;
                                        nat_add(a4746, a4747)?
                                    }))
                                } else {
                                    let a4748 = v0;
                                    let a4749 = 89u64;
                                    if if nat_eq(a4748, a4749) {
                                        let a4750 = v1;
                                        let a4751 = 7u64;
                                        nat_eq(a4750, a4751)
                                    } else {
                                        false
                                    } {
                                        Ok(Some(17u64))
                                    } else {
                                        let a4752 = v0;
                                        let a4753 = 89u64;
                                        if if nat_eq(a4752, a4753) {
                                            let a4754 = v1;
                                            let a4755 = 10u64;
                                            nat_eq(a4754, a4755)
                                        } else {
                                            false
                                        } {
                                            Ok(Some(18u64))
                                        } else {
                                            let a4756 = v0;
                                            let a4757 = 89u64;
                                            if if nat_eq(a4756, a4757) {
                                                let a4758 = v1;
                                                let a4759 = 13u64;
                                                nat_eq(a4758, a4759)
                                            } else {
                                                false
                                            } {
                                                Ok(Some(16u64))
                                            } else {
                                                let a4760 = v0;
                                                let a4761 = 8u64;
                                                if nat_eq(a4760, a4761) {
                                                    Ok(Some(5u64))
                                                } else {
                                                    let a4762 = v0;
                                                    let a4763 = 9u64;
                                                    if nat_eq(a4762, a4763) {
                                                        Ok(Some(6u64))
                                                    } else {
                                                        let a4764 = v0;
                                                        let a4765 = 89u64;
                                                        if nat_eq(a4764, a4765) {
                                                            let a4766 = v1;
                                                            let a4767 = 0u64;
                                                            if nat_eq(a4766, a4767) {
                                                                Ok(Some(0u64))
                                                            } else {
                                                                let a4768 = v1;
                                                                let a4769 = 1u64;
                                                                if nat_eq(a4768, a4769) {
                                                                    Ok(Some(1u64))
                                                                } else {
                                                                    let a4770 = v1;
                                                                    let a4771 = 2u64;
                                                                    if nat_eq(a4770, a4771) {
                                                                        Ok(Some(2u64))
                                                                    } else {
                                                                        let a4772 = v1;
                                                                        let a4773 = 3u64;
                                                                        if nat_eq(a4772, a4773) {
                                                                            Ok(Some(3u64))
                                                                        } else {
                                                                            let a4774 = v1;
                                                                            let a4775 = 6u64;
                                                                            if nat_eq(a4774, a4775) {
                                                                                Ok(Some(4u64))
                                                                            } else {
                                                                                let a4776 = v1;
                                                                                let a4777 = 12u64;
                                                                                if nat_eq(a4776, a4777) {
                                                                                    Ok(Some(7u64))
                                                                                } else {
                                                                                    let a4778 = v1;
                                                                                    let a4779 = 11u64;
                                                                                    if nat_eq(a4778, a4779) {
                                                                                        Ok(Some(8u64))
                                                                                    } else {
                                                                                        let a4780 = v1;
                                                                                        let a4781 = 18u64;
                                                                                        if nat_eq(a4780, a4781) {
                                                                                            Ok(Some(9u64))
                                                                                        } else {
                                                                                            let a4782 = v1;
                                                                                            let a4783 = 19u64;
                                                                                            if nat_eq(a4782, a4783) {
                                                                                                Ok(Some(10u64))
                                                                                            } else {
                                                                                                let a4784 = v1;
                                                                                                let a4785 = 20u64;
                                                                                                if nat_eq(a4784, a4785) {
                                                                                                    Ok(Some(11u64))
                                                                                                } else {
                                                                                                    let a4786 = v1;
                                                                                                    let a4787 = 21u64;
                                                                                                    if nat_eq(a4786, a4787) {
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

pub fn f148(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m4788 = f247(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m4788 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a4789 = v0;
            let a4790 = 2u64;
            if if if if nat_eq(a4789, a4790) {
                true
            } else {
                let a4791 = v0;
                let a4792 = 5u64;
                nat_eq(a4791, a4792)
            } {
                true
            } else {
                let a4793 = v0;
                let a4794 = 6u64;
                nat_eq(a4793, a4794)
            } {
                true
            } else {
                let a4795 = v0;
                let a4796 = 7u64;
                nat_eq(a4795, a4796)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f27(v2.clone(), 1u64), f27(v4, 1u64), Adt4::C10(v7, v8, {
                    let m4797 = v3;
                    match m4797 {
                        None => {
                            f27(v2, 1u64)
                        }
                        Some(v14) => {
                            f27(v14, 1u64)
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

pub fn f149(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a4799 = match v2.clone() {
        Adt2::C0(h4798, _, _, _, _, _, _, _, _, _, _) => {
            h4798
        }
    };
    let a4800 = 0u64;
    if if if if if nat_eq(a4799, a4800) {
        true
    } else {
        let a4802 = match v2.clone() {
            Adt2::C0(h4801, _, _, _, _, _, _, _, _, _, _) => {
                h4801
            }
        };
        let a4803 = 1u64;
        nat_eq(a4802, a4803)
    } {
        true
    } else {
        let a4805 = match v2.clone() {
            Adt2::C0(h4804, _, _, _, _, _, _, _, _, _, _) => {
                h4804
            }
        };
        let a4806 = 30u64;
        nat_eq(a4805, a4806)
    } {
        let a4809 = match v2.clone() {
            Adt2::C0(h4807, _, _, _, _, _, _, _, _, _, _) => {
                h4807
            }
        };
        let a4810 = match v3.clone() {
            Adt2::C0(h4808, _, _, _, _, _, _, _, _, _, _) => {
                h4808
            }
        };
        if nat_eq(a4809, a4810) {
            true
        } else {
            let a4812 = match v3.clone() {
                Adt2::C0(h4811, _, _, _, _, _, _, _, _, _, _) => {
                    h4811
                }
            };
            let a4813 = 0u64;
            nat_eq(a4812, a4813)
        }
    } else {
        false
    } {
        f28(v2.clone(), v3.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a4820 = f23(v3.clone(), 1u64)?;
            let a4821 = {
                let a4814 = 25u64;
                let a4815 = v0;
                if nat_le(a4814, a4815) {
                    f183({
                        let a4816 = v0;
                        let a4817 = 26u64;
                        nat_eq(a4816, a4817)
                    }, match v3.clone() {
                        Adt2::C0(h4818, _, _, _, _, _, _, _, _, _, _) => {
                            h4818
                        }
                    })?
                } else {
                    f248(v0, match v3.clone() {
                        Adt2::C0(h4819, _, _, _, _, _, _, _, _, _, _) => {
                            h4819
                        }
                    })
                }
            };
            nat_mul(a4820, a4821)?
        }, {
            let a4823 = match v3.clone() {
                Adt2::C0(h4822, _, _, _, _, _, _, _, _, _, _) => {
                    h4822
                }
            };
            let a4824 = 0u64;
            if nat_eq(a4823, a4824) {
                4u64
            } else {
                2u64
            }
        }, f27(v2.clone(), 1u64), f27(v3.clone(), 1u64), {
            let a4825 = 25u64;
            let a4826 = v0;
            if nat_le(a4825, a4826) {
                Adt4::C45({
                    let a4827 = v0;
                    let a4828 = 26u64;
                    nat_eq(a4827, a4828)
                }, match v2 {
                    Adt2::C0(h4829, _, _, _, _, _, _, _, _, _, _) => {
                        h4829
                    }
                }, match v3 {
                    Adt2::C0(h4830, _, _, _, _, _, _, _, _, _, _) => {
                        h4830
                    }
                }, v1)
            } else {
                Adt4::C14(v0, match v2 {
                    Adt2::C0(h4831, _, _, _, _, _, _, _, _, _, _) => {
                        h4831
                    }
                }, match v3 {
                    Adt2::C0(h4832, _, _, _, _, _, _, _, _, _, _) => {
                        h4832
                    }
                })
            }
        }))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f150(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a4845 = {
        let a4841 = {
            let a4837 = {
                let a4833 = v0;
                let a4834 = v4;
                nat_mul(a4833, a4834)?
            };
            let a4838 = {
                let a4835 = v1;
                let a4836 = v5;
                nat_mul(a4835, a4836)?
            };
            nat_add(a4837, a4838)?
        };
        let a4842 = {
            let a4839 = v2;
            let a4840 = v6;
            nat_mul(a4839, a4840)?
        };
        nat_add(a4841, a4842)?
    };
    let a4846 = {
        let a4843 = v3;
        let a4844 = v7;
        nat_mul(a4843, a4844)?
    };
    nat_add(a4845, a4846)
}

pub fn f151(v0: u64) -> u64 {
    let a4847 = v0;
    let a4848 = 2147483648u64;
    if nat_lt(a4847, a4848) {
        0u64
    } else {
        let a4849 = 4294967296u64;
        let a4850 = v0;
        nat_sub(a4849, a4850)
    }
}

pub fn f152(v0: u64) -> u64 {
    let a4851 = v0;
    let a4852 = 2147483648u64;
    if nat_lt(a4851, a4852) {
        v0
    } else {
        0u64
    }
}

pub fn f153(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a4853 = v7;
        let a4854 = 6u64;
        if nat_eq(a4853, a4854) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f97(v3, v4), f97(v5, v6), f98(v7, v8), f184(v1), f184(v9))
}

pub fn f154(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m4855 = v0;
    match m4855 {
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
        Adt4::C13(v64, v65, v66, v67) => {
            Adt11::C4(v64, v65, v66, v67)
        }
        Adt4::C14(v68, _, _) => {
            let a4856 = 13u64;
            let a4857 = v68;
            if nat_le(a4856, a4857) {
                Adt11::C7(v68, v1, v2)
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
        Adt4::C27(v88) => {
            Adt11::C2(match v88.clone() {
                Adt5::C0(_, _, _, _, _, h4858, _, _) => {
                    h4858
                }
            }, match v88 {
                Adt5::C0(_, _, _, _, _, _, _, h4859) => {
                    h4859
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
        Adt4::C46(_, _) => {
            Adt11::C6
        }
    }
}

pub fn f155(v0: Adt4) -> u64 {
    let m4860 = v0;
    match m4860 {
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
        Adt4::C14(v66, _, _) => {
            v66
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
        Adt4::C46(_, _) => {
            0u64
        }
    }
}

pub fn f156(v0: Adt4, v1: u64) -> u64 {
    let m4861 = v0;
    match m4861 {
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
        Adt4::C16(_, v71) => {
            v71
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v74, _) => {
            let a4862 = v1;
            let a4863 = 6u64;
            if nat_eq(a4862, a4863) {
                v74
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
        Adt4::C26(v88, _, _, _) => {
            let a4872 = v1;
            let a4873 = 6u64;
            if if if nat_eq(a4872, a4873) {
                true
            } else {
                let a4874 = v1;
                let a4875 = 7u64;
                nat_eq(a4874, a4875)
            } {
                let a4876 = v88;
                let a4877 = 30u64;
                nat_eq(a4876, a4877)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v87) => {
            match v87 {
                Adt5::C0(_, _, _, _, _, _, _, h4871) => {
                    h4871
                }
            }
        }
        Adt4::C28(v83) => {
            let a4864 = v1;
            let a4865 = 6u64;
            if nat_eq(a4864, a4865) {
                match v83 {
                    Adt7::C0(_, _, _, _, h4866) => {
                        h4866
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v86) => {
            let a4867 = v1;
            let a4868 = 6u64;
            if if nat_eq(a4867, a4868) {
                true
            } else {
                let a4869 = v1;
                let a4870 = 7u64;
                nat_eq(a4869, a4870)
            } {
                v86
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
        Adt4::C46(_, _) => {
            0u64
        }
    }
}

pub fn f157(v0: Adt4, v1: u64) -> Adt11 {
    let a4878 = v1;
    let a4879 = 4u64;
    if nat_lt(a4878, a4879) {
        Adt11::C0(v1)
    } else {
        let a4880 = v1;
        let a4881 = 4u64;
        if nat_eq(a4880, a4881) {
            Adt11::C1(0u64)
        } else {
            let a4882 = v1;
            let a4883 = 5u64;
            if nat_eq(a4882, a4883) {
                Adt11::C1(3u64)
            } else {
                let a4884 = v1;
                let a4885 = 6u64;
                if nat_eq(a4884, a4885) {
                    Adt11::C2(f156(v0, v1), 0u64, false)
                } else {
                    let a4886 = v1;
                    let a4887 = 7u64;
                    if nat_eq(a4886, a4887) {
                        Adt11::C2(0u64, f156(v0, v1), false)
                    } else {
                        let a4888 = v1;
                        let a4889 = 16u64;
                        if nat_lt(a4888, a4889) {
                            Adt11::C1({
                                let a4890 = v1;
                                let a4891 = 8u64;
                                nat_sub(a4890, a4891)
                            })
                        } else {
                            let a4892 = v1;
                            let a4893 = 20u64;
                            if nat_lt(a4892, a4893) {
                                Adt11::C5({
                                    let a4894 = v1;
                                    let a4895 = 16u64;
                                    nat_sub(a4894, a4895)
                                })
                            } else {
                                let a4896 = v1;
                                let a4897 = 20u64;
                                if nat_eq(a4896, a4897) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a4898 = v1;
                                    let a4899 = 21u64;
                                    if nat_eq(a4898, a4899) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a4900 = v1;
                                        let a4901 = 22u64;
                                        if nat_eq(a4900, a4901) {
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

pub fn f158(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4919 = {
        let a4914 = f249(v2, {
            let a4905 = {
                let a4902 = v0;
                let a4903 = v1;
                let a4904 = 0u64;
                nat_quot(a4902, a4903, a4904)
            };
            let a4906 = v2;
            let a4907 = 0u64;
            nat_rem(a4905, a4906, a4907)
        }, {
            let a4911 = {
                let a4908 = v0;
                let a4909 = v1;
                let a4910 = 0u64;
                nat_quot(a4908, a4909, a4910)
            };
            let a4912 = v2;
            let a4913 = 0u64;
            nat_quot(a4911, a4912, a4913)
        })?;
        let a4915 = v1;
        nat_mul(a4914, a4915)?
    };
    let a4920 = {
        let a4916 = v0;
        let a4917 = v1;
        let a4918 = 0u64;
        nat_rem(a4916, a4917, a4918)
    };
    nat_add(a4919, a4920)
}

pub fn f159(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a4922 = v0;
        let a4923 = match v2.clone() {
            Adt3::C0(h4921, _, _, _, _, _, _, _) => {
                h4921
            }
        };
        let a4924 = 0u64;
        nat_quot(a4922, a4923, a4924)
    };
    let v4: u64 = {
        let a4926 = v3;
        let a4927 = match v2.clone() {
            Adt3::C0(_, h4925, _, _, _, _, _, _) => {
                h4925
            }
        };
        let a4928 = 0u64;
        nat_quot(a4926, a4927, a4928)
    };
    f250({
        let a4930 = v0;
        let a4931 = match v2.clone() {
            Adt3::C0(h4929, _, _, _, _, _, _, _) => {
                h4929
            }
        };
        let a4932 = 0u64;
        nat_rem(a4930, a4931, a4932)
    }, {
        let a4934 = v3;
        let a4935 = match v2.clone() {
            Adt3::C0(_, h4933, _, _, _, _, _, _) => {
                h4933
            }
        };
        let a4936 = 0u64;
        nat_rem(a4934, a4935, a4936)
    }, {
        let a4938 = v4;
        let a4939 = match v2.clone() {
            Adt3::C0(_, _, h4937, _, _, _, _, _) => {
                h4937
            }
        };
        let a4940 = 0u64;
        nat_rem(a4938, a4939, a4940)
    }, {
        let a4942 = v4;
        let a4943 = match v2 {
            Adt3::C0(_, _, h4941, _, _, _, _, _) => {
                h4941
            }
        };
        let a4944 = 0u64;
        nat_quot(a4942, a4943, a4944)
    }, match v1.clone() {
        Adt3::C0(h4945, _, _, _, _, _, _, _) => {
            h4945
        }
    }, match v1.clone() {
        Adt3::C0(_, h4946, _, _, _, _, _, _) => {
            h4946
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h4947, _, _, _, _, _) => {
            h4947
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h4948, _, _, _, _) => {
            h4948
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h4949, _, _, _) => {
            h4949
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h4950, _, _) => {
            h4950
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h4951, _) => {
            h4951
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h4952) => {
            h4952
        }
    })
}

pub fn f160(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4954 = v0;
        let a4955 = match v3.clone() {
            Adt3::C0(h4953, _, _, _, _, _, _, _) => {
                h4953
            }
        };
        let a4956 = 0u64;
        nat_quot(a4954, a4955, a4956)
    };
    let v6: u64 = {
        let a4958 = v5;
        let a4959 = match v3.clone() {
            Adt3::C0(_, h4957, _, _, _, _, _, _) => {
                h4957
            }
        };
        let a4960 = 0u64;
        nat_quot(a4958, a4959, a4960)
    };
    let v7: u64 = {
        let a4962 = v0;
        let a4963 = match v3.clone() {
            Adt3::C0(h4961, _, _, _, _, _, _, _) => {
                h4961
            }
        };
        let a4964 = 0u64;
        nat_rem(a4962, a4963, a4964)
    };
    let v8: u64 = {
        let a4966 = v5;
        let a4967 = match v3.clone() {
            Adt3::C0(_, h4965, _, _, _, _, _, _) => {
                h4965
            }
        };
        let a4968 = 0u64;
        nat_rem(a4966, a4967, a4968)
    };
    let v9: u64 = {
        let a4970 = v6;
        let a4971 = match v3.clone() {
            Adt3::C0(_, _, h4969, _, _, _, _, _) => {
                h4969
            }
        };
        let a4972 = 0u64;
        nat_rem(a4970, a4971, a4972)
    };
    let v10: u64 = {
        let a4974 = v6;
        let a4975 = match v3.clone() {
            Adt3::C0(_, _, h4973, _, _, _, _, _) => {
                h4973
            }
        };
        let a4976 = 0u64;
        nat_quot(a4974, a4975, a4976)
    };
    let v11: u64 = {
        let a4977 = v4;
        let a4978 = 0u64;
        if nat_eq(a4977, a4978) {
            match v1.clone() {
                Adt3::C0(h4979, _, _, _, _, _, _, _) => {
                    h4979
                }
            }
        } else {
            let a4980 = v4;
            let a4981 = 1u64;
            if nat_eq(a4980, a4981) {
                match v1.clone() {
                    Adt3::C0(_, h4982, _, _, _, _, _, _) => {
                        h4982
                    }
                }
            } else {
                let a4983 = v4;
                let a4984 = 2u64;
                if nat_eq(a4983, a4984) {
                    match v1.clone() {
                        Adt3::C0(_, _, h4985, _, _, _, _, _) => {
                            h4985
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h4986, _, _, _, _) => {
                            h4986
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a4993 = v11;
        let a4994 = {
            let a4987 = v4;
            let a4988 = 0u64;
            if nat_eq(a4987, a4988) {
                v7
            } else {
                let a4989 = v4;
                let a4990 = 1u64;
                if nat_eq(a4989, a4990) {
                    v8
                } else {
                    let a4991 = v4;
                    let a4992 = 2u64;
                    if nat_eq(a4991, a4992) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a4993, a4994)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f150(if if v12 {
        let a4995 = v4;
        let a4996 = 0u64;
        nat_eq(a4995, a4996)
    } else {
        false
    } {
        let a4997 = v7;
        let a4998 = v11;
        nat_sub(a4997, a4998)
    } else {
        v7
    }, if if v12 {
        let a4999 = v4;
        let a5000 = 1u64;
        nat_eq(a4999, a5000)
    } else {
        false
    } {
        let a5001 = v8;
        let a5002 = v11;
        nat_sub(a5001, a5002)
    } else {
        v8
    }, if if v12 {
        let a5003 = v4;
        let a5004 = 2u64;
        nat_eq(a5003, a5004)
    } else {
        false
    } {
        let a5005 = v9;
        let a5006 = v11;
        nat_sub(a5005, a5006)
    } else {
        v9
    }, if if v12 {
        let a5007 = v4;
        let a5008 = 3u64;
        nat_eq(a5007, a5008)
    } else {
        false
    } {
        let a5009 = v10;
        let a5010 = v11;
        nat_sub(a5009, a5010)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h5011, _, _, _) => {
            h5011
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h5012, _, _) => {
            h5012
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h5013, _) => {
            h5013
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h5014) => {
            h5014
        }
    })?, f99(v0, v3)?))
}

pub fn f161(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f62(v0, v2.clone(), 0u64);
    let v9: u64 = f62(v0, v2.clone(), 1u64);
    let v10: u64 = f62(v0, v2.clone(), 2u64);
    let v11: u64 = f62(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f61(v1.clone(), f251(v8, v3, match v1.clone() {
            Adt3::C0(h5015, _, _, _, _, _, _, _) => {
                h5015
            }
        })?, f251(v9, v4, match v1.clone() {
            Adt3::C0(_, h5016, _, _, _, _, _, _) => {
                h5016
            }
        })?, f251(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h5017, _, _, _, _, _) => {
                h5017
            }
        })?, f251(v11, v6, match v1 {
            Adt3::C0(_, _, _, h5018, _, _, _, _) => {
                h5018
            }
        })?)?, f99(v0, v2)?))
    } else {
        if if if if f252(v8, v3, match v1.clone() {
            Adt3::C0(h5019, _, _, _, _, _, _, _) => {
                h5019
            }
        })? {
            f252(v9, v4, match v1.clone() {
                Adt3::C0(_, h5020, _, _, _, _, _, _) => {
                    h5020
                }
            })?
        } else {
            false
        } {
            f252(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h5021, _, _, _, _, _) => {
                    h5021
                }
            })?
        } else {
            false
        } {
            f252(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h5022, _, _, _, _) => {
                    h5022
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f61(v1, f253(v8, v3)?, f253(v9, v4)?, f253(v10, v5)?, f253(v11, v6)?)?, f99(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f99(v0, v2)?))
        }
    }
}

pub fn f162(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f62(v0, v2.clone(), 0u64);
    let v5: u64 = f62(v0, v2.clone(), 1u64);
    let v6: u64 = f62(v0, v2.clone(), 2u64);
    let v7: u64 = f62(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f61(v1.clone(), f254(v4, v3, match v1 {
        Adt3::C0(h5023, _, _, _, _, _, _, _) => {
            h5023
        }
    })?, v5, v6, v7)?, f99(v0, v2)?))
}

pub fn f163(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a5024 = v0;
    let a5025 = v3;
    if nat_lt(a5024, a5025) {
        Ok(Adt9::C1(false, {
            let a5026 = v0;
            let a5027 = 4u64;
            nat_mul(a5026, a5027)?
        }, {
            let a5028 = v0;
            let a5029 = 4u64;
            nat_mul(a5028, a5029)?
        }))
    } else {
        let v6: u64 = {
            let a5030 = v0;
            let a5031 = v3;
            nat_sub(a5030, a5031)
        };
        let v7: u64 = f99(v6, v1)?;
        let v8: u64 = f255(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f164(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5032 = v0;
    let a5033 = v3;
    if nat_lt(a5032, a5033) {
        Ok(Adt9::C1(true, 0u64, f99(v0, v2)?))
    } else {
        let v4: u64 = {
            let a5034 = v0;
            let a5035 = v3;
            nat_sub(a5034, a5035)
        };
        let v5: u64 = f159(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f99(v4, v1)?, 3u64, v5))
    }
}

pub fn f165(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a5037 = v1;
        let a5038 = match v3.clone() {
            Adt3::C0(h5036, _, _, _, _, _, _, _) => {
                h5036
            }
        };
        let a5039 = 0u64;
        nat_rem(a5037, a5038, a5039)
    };
    let v5: u64 = {
        let a5045 = {
            let a5041 = v1;
            let a5042 = match v3.clone() {
                Adt3::C0(h5040, _, _, _, _, _, _, _) => {
                    h5040
                }
            };
            let a5043 = 0u64;
            nat_quot(a5041, a5042, a5043)
        };
        let a5046 = match v3.clone() {
            Adt3::C0(_, h5044, _, _, _, _, _, _) => {
                h5044
            }
        };
        let a5047 = 0u64;
        nat_rem(a5045, a5046, a5047)
    };
    let v6: bool = f256(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f159(v1, v2, v3.clone())?
    } else {
        0u64
    }, f99(v1, v3)?))
}

pub fn f166(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a5048 = v3;
        let a5049 = 8u64;
        nat_add(a5048, a5049)?
    }, 0u64, f99(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f159(v0, v5, v2.clone())?
    }, 3u64, f99(v0, v2)?))
}

pub fn f167(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5050 = v4;
    let a5051 = 0u64;
    if if nat_eq(a5050, a5051) {
        f131(v3)
    } else {
        false
    } {
        let m5052 = f226(v3);
        match m5052 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h5053, _) => {
                        h5053
                    }
                };
                Ok(Adt9::C5(v3, f99(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h5054) => {
                        h5054
                    }
                }, f99({
                    let a5055 = v0;
                    let a5056 = v6;
                    nat_mul(a5055, a5056)?
                }, v2.clone())?, {
                    let a5062 = {
                        let a5060 = {
                            let a5057 = v6;
                            let a5058 = 1u64;
                            nat_sub(a5057, a5058)
                        };
                        let a5061 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h5059, _, _, _) => {
                                h5059
                            }
                        };
                        nat_mul(a5060, a5061)?
                    };
                    let a5063 = 4u64;
                    nat_add(a5062, a5063)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h5064, _, _, _) => {
                        h5064
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f99(v0, v1)?, f99(v0, v2)?))
    }
}

pub fn f168(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a5066 = v0;
        let a5067 = match v2.clone() {
            Adt3::C0(h5065, _, _, _, _, _, _, _) => {
                h5065
            }
        };
        let a5068 = 0u64;
        nat_rem(a5066, a5067, a5068)
    };
    let v7: u64 = {
        let a5070 = v0;
        let a5071 = match v2.clone() {
            Adt3::C0(h5069, _, _, _, _, _, _, _) => {
                h5069
            }
        };
        let a5072 = 0u64;
        nat_quot(a5070, a5071, a5072)
    };
    let v8: u64 = {
        let a5074 = v7;
        let a5075 = match v2.clone() {
            Adt3::C0(_, h5073, _, _, _, _, _, _) => {
                h5073
            }
        };
        let a5076 = 0u64;
        nat_rem(a5074, a5075, a5076)
    };
    let v9: u64 = {
        let a5078 = v7;
        let a5079 = match v2.clone() {
            Adt3::C0(_, h5077, _, _, _, _, _, _) => {
                h5077
            }
        };
        let a5080 = 0u64;
        nat_quot(a5078, a5079, a5080)
    };
    let v10: u64 = {
        let a5082 = v9;
        let a5083 = match v2.clone() {
            Adt3::C0(_, _, h5081, _, _, _, _, _) => {
                h5081
            }
        };
        let a5084 = 0u64;
        nat_rem(a5082, a5083, a5084)
    };
    let v11: u64 = {
        let a5086 = v9;
        let a5087 = match v2.clone() {
            Adt3::C0(_, _, h5085, _, _, _, _, _) => {
                h5085
            }
        };
        let a5088 = 0u64;
        nat_quot(a5086, a5087, a5088)
    };
    let v12: u64 = f99(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a5097 = {
                let a5092 = {
                    let a5089 = v10;
                    let a5090 = v4;
                    let a5091 = 0u64;
                    nat_quot(a5089, a5090, a5091)
                };
                let a5093 = v5;
                nat_mul(a5092, a5093)?
            };
            let a5098 = {
                let a5094 = v8;
                let a5095 = v4;
                let a5096 = 0u64;
                nat_quot(a5094, a5095, a5096)
            };
            nat_add(a5097, a5098)?
        };
        let v14: u64 = {
            let a5118 = v6;
            let a5119 = {
                let a5116 = match v1.clone() {
                    Adt3::C0(h5099, _, _, _, _, _, _, _) => {
                        h5099
                    }
                };
                let a5117 = {
                    let a5114 = {
                        let a5100 = v8;
                        let a5101 = v4;
                        let a5102 = 0u64;
                        nat_rem(a5100, a5101, a5102)
                    };
                    let a5115 = {
                        let a5112 = match v1.clone() {
                            Adt3::C0(_, h5103, _, _, _, _, _, _) => {
                                h5103
                            }
                        };
                        let a5113 = {
                            let a5110 = {
                                let a5104 = v10;
                                let a5105 = v4;
                                let a5106 = 0u64;
                                nat_rem(a5104, a5105, a5106)
                            };
                            let a5111 = {
                                let a5108 = match v1.clone() {
                                    Adt3::C0(_, _, h5107, _, _, _, _, _) => {
                                        h5107
                                    }
                                };
                                let a5109 = v13;
                                nat_mul(a5108, a5109)?
                            };
                            nat_add(a5110, a5111)?
                        };
                        nat_mul(a5112, a5113)?
                    };
                    nat_add(a5114, a5115)?
                };
                nat_mul(a5116, a5117)?
            };
            nat_add(a5118, a5119)?
        };
        Ok(Adt9::C1(false, f99(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a5125 = {
                let a5123 = {
                    let a5120 = v11;
                    let a5121 = v5;
                    let a5122 = 0u64;
                    nat_rem(a5120, a5121, a5122)
                };
                let a5124 = v4;
                nat_mul(a5123, a5124)?
            };
            let a5126 = v8;
            nat_add(a5125, a5126)?
        };
        let v16: u64 = {
            let a5132 = {
                let a5130 = {
                    let a5127 = v11;
                    let a5128 = v5;
                    let a5129 = 0u64;
                    nat_quot(a5127, a5128, a5129)
                };
                let a5131 = v4;
                nat_mul(a5130, a5131)?
            };
            let a5133 = v10;
            nat_add(a5132, a5133)?
        };
        let a5135 = v15;
        let a5136 = match v1.clone() {
            Adt3::C0(_, h5134, _, _, _, _, _, _) => {
                h5134
            }
        };
        if if nat_lt(a5135, a5136) {
            let a5138 = v16;
            let a5139 = match v1.clone() {
                Adt3::C0(_, _, h5137, _, _, _, _, _) => {
                    h5137
                }
            };
            nat_lt(a5138, a5139)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f99({
                let a5148 = v6;
                let a5149 = {
                    let a5146 = match v1.clone() {
                        Adt3::C0(h5140, _, _, _, _, _, _, _) => {
                            h5140
                        }
                    };
                    let a5147 = {
                        let a5144 = v15;
                        let a5145 = {
                            let a5142 = match v1.clone() {
                                Adt3::C0(_, h5141, _, _, _, _, _, _) => {
                                    h5141
                                }
                            };
                            let a5143 = v16;
                            nat_mul(a5142, a5143)?
                        };
                        nat_add(a5144, a5145)?
                    };
                    nat_mul(a5146, a5147)?
                };
                nat_add(a5148, a5149)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f169(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f248(23u64, v4);
    let v6: u64 = {
        let a5150 = v0;
        let a5151 = v5;
        let a5152 = 0u64;
        nat_rem(a5150, a5151, a5152)
    };
    let v7: u64 = {
        let a5153 = v0;
        let a5154 = v5;
        let a5155 = 0u64;
        nat_quot(a5153, a5154, a5155)
    };
    let v8: Adt13 = Adt13::C1(f99(v7, v2)?);
    let a5156 = v6;
    let a5157 = 0u64;
    if nat_eq(a5156, a5157) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f99(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f184(v3), 4u64))
    } else {
        let a5158 = v6;
        let a5159 = 1u64;
        if nat_eq(a5158, a5159) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a5160 = v4;
                let a5161 = 0u64;
                if nat_eq(a5160, a5161) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f184(v4)))
        }
    }
}

pub fn f170(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f99(v0, v2.clone())?;
    let v4: u64 = f99(v0, v1.clone())?;
    let a5166 = {
        let a5163 = v0;
        let a5164 = match v1 {
            Adt3::C0(h5162, _, _, _, _, _, _, _) => {
                h5162
            }
        };
        let a5165 = 0u64;
        nat_rem(a5163, a5164, a5165)
    };
    let a5167 = 0u64;
    if nat_eq(a5166, a5167) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f99({
            let a5168 = v0;
            let a5169 = 1u64;
            nat_sub(a5168, a5169)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f171(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a5170 = v3;
    let a5171 = 13u64;
    if nat_eq(a5170, a5171) {
        f257(v0, v1, v4)
    } else {
        let v5: u64 = f258(match v1.clone() {
            Adt3::C0(h5172, _, _, _, _, _, _, _) => {
                h5172
            }
        }, {
            let a5173 = v3;
            let a5174 = 16u64;
            nat_eq(a5173, a5174)
        })?;
        let v6: u64 = {
            let a5175 = v0;
            let a5176 = v5;
            let a5177 = 0u64;
            nat_quot(a5175, a5176, a5177)
        };
        let v7: u64 = {
            let a5178 = v0;
            let a5179 = v5;
            let a5180 = 0u64;
            nat_rem(a5178, a5179, a5180)
        };
        let v8: u64 = {
            let a5184 = 1u64;
            let a5185 = {
                let a5182 = match v1.clone() {
                    Adt3::C0(h5181, _, _, _, _, _, _, _) => {
                        h5181
                    }
                };
                let a5183 = 2u64;
                nat_mul(a5182, a5183)?
            };
            nat_add(a5184, a5185)?
        };
        let a5186 = v7;
        let a5187 = v8;
        if nat_lt(a5186, a5187) {
            f259(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f99(v6, v2)?;
            let a5188 = v3;
            let a5189 = 14u64;
            if nat_eq(a5188, a5189) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a5190 = v7;
                let a5191 = v8;
                if nat_eq(a5190, a5191) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5194 = v7;
                    let a5195 = {
                        let a5192 = v8;
                        let a5193 = 1u64;
                        nat_add(a5192, a5193)?
                    };
                    if nat_eq(a5194, a5195) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h5196, _, _, _, _, _, _, _) => {
                                h5196
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

pub fn f172(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5197 = v0;
    let a5198 = 0u64;
    if nat_eq(a5197, a5198) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a5205 = {
            let a5199 = v0;
            let a5200 = 1u64;
            nat_add(a5199, a5200)?
        };
        let a5206 = {
            let a5203 = 2u64;
            let a5204 = {
                let a5201 = v3;
                let a5202 = 2u64;
                nat_mul(a5201, a5202)?
            };
            nat_add(a5203, a5204)?
        };
        if nat_eq(a5205, a5206) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a5207 = v0;
                let a5208 = 1u64;
                nat_sub(a5207, a5208)
            };
            let v5: u64 = {
                let a5209 = v4;
                let a5210 = 2u64;
                let a5211 = 0u64;
                nat_quot(a5209, a5210, a5211)
            };
            let a5215 = {
                let a5212 = v4;
                let a5213 = 2u64;
                let a5214 = 0u64;
                nat_rem(a5212, a5213, a5214)
            };
            let a5216 = 0u64;
            if nat_eq(a5215, a5216) {
                Ok(Adt9::C4(17u64, 0u64, f99(v5, v1)?, 4u64, f99(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f173(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a5217 = v0;
    let a5218 = f45(v1, v2, v3)?;
    if nat_lt(a5217, a5218) {
        let a5219 = v0;
        let a5220 = v1;
        if nat_lt(a5219, a5220) {
            Ok(Adt9::C1(false, {
                let a5221 = 4u64;
                let a5222 = v0;
                nat_mul(a5221, a5222)?
            }, {
                let a5223 = 4u64;
                let a5224 = v0;
                nat_mul(a5223, a5224)?
            }))
        } else {
            let v4: u64 = {
                let a5225 = v0;
                let a5226 = v1;
                nat_sub(a5225, a5226)
            };
            let v5: u64 = f260(v4, v2)?;
            let a5227 = v5;
            let a5228 = 2u64;
            if nat_lt(a5227, a5228) {
                Ok(Adt9::C4(6u64, {
                    let a5229 = v5;
                    let a5230 = 0u64;
                    if nat_eq(a5229, a5230) {
                        4u64
                    } else {
                        5u64
                    }
                }, f261(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f262(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f263(v4, v2)? {
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

pub fn f174(v0: u64, v1: u64) -> u64 {
    let a5231 = v0;
    let a5232 = 0u64;
    if if nat_eq(a5231, a5232) {
        true
    } else {
        let a5233 = v1;
        let a5234 = 0u64;
        nat_eq(a5233, a5234)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f175(v0: u64, v1: u64) -> Adt11 {
    let a5235 = v0;
    let a5236 = 43u64;
    if nat_eq(a5235, a5236) {
        Adt11::C8(3u64)
    } else {
        let a5237 = v1;
        let a5238 = 43u64;
        if nat_eq(a5237, a5238) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f176(v0: u64) -> R<u64> {
    let a5241 = 2u64;
    let a5242 = {
        let a5239 = 6u64;
        let a5240 = v0;
        nat_mul(a5239, a5240)?
    };
    nat_add(a5241, a5242)
}

pub fn f177(v0: u64, v1: bool) -> R<u64> {
    let a5245 = {
        let a5243 = v0;
        let a5244 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a5243, a5244)?
    };
    let a5246 = 1u64;
    nat_add(a5245, a5246)
}

pub fn f178(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a5247 = v0;
    let a5248 = 16u64;
    if nat_lt(a5247, a5248) {
        let v9: u64 = {
            let a5249 = v0;
            let a5250 = 4u64;
            let a5251 = 0u64;
            nat_quot(a5249, a5250, a5251)
        };
        let v10: u64 = {
            let a5256 = {
                let a5254 = 8u64;
                let a5255 = {
                    let a5252 = 4u64;
                    let a5253 = v1;
                    nat_mul(a5252, a5253)?
                };
                nat_add(a5254, a5255)?
            };
            let a5257 = v9;
            nat_add(a5256, a5257)?
        };
        let v11: u64 = f61(v6, v9, v1, v2, 0u64)?;
        let a5261 = {
            let a5258 = v0;
            let a5259 = 4u64;
            let a5260 = 0u64;
            nat_rem(a5258, a5259, a5260)
        };
        let a5262 = 0u64;
        if nat_eq(a5261, a5262) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f61(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f61(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a5266 = {
                let a5263 = v0;
                let a5264 = 4u64;
                let a5265 = 0u64;
                nat_rem(a5263, a5264, a5265)
            };
            let a5267 = 1u64;
            if nat_eq(a5266, a5267) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f61(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a5271 = {
                    let a5268 = v0;
                    let a5269 = 4u64;
                    let a5270 = 0u64;
                    nat_rem(a5268, a5269, a5270)
                };
                let a5272 = 2u64;
                if nat_eq(a5271, a5272) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a5273 = v9;
                        let a5274 = 0u64;
                        if nat_eq(a5273, a5274) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a5275 = v8;
                        let a5276 = 2u64;
                        if nat_eq(a5275, a5276) {
                            let a5277 = v9;
                            let a5278 = 0u64;
                            if nat_eq(a5277, a5278) {
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
        let a5279 = v0;
        let a5280 = 28u64;
        if nat_lt(a5279, a5280) {
            let v12: u64 = {
                let a5281 = v0;
                let a5282 = 16u64;
                nat_sub(a5281, a5282)
            };
            let v13: u64 = {
                let a5283 = v12;
                let a5284 = 3u64;
                let a5285 = 0u64;
                nat_quot(a5283, a5284, a5285)
            };
            let v14: u64 = f61(v6, v13, v1, v2, 0u64)?;
            let a5289 = {
                let a5286 = v12;
                let a5287 = 3u64;
                let a5288 = 0u64;
                nat_rem(a5286, a5287, a5288)
            };
            let a5290 = 0u64;
            if nat_eq(a5289, a5290) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a5294 = {
                    let a5291 = v12;
                    let a5292 = 3u64;
                    let a5293 = 0u64;
                    nat_rem(a5291, a5292, a5293)
                };
                let a5295 = 1u64;
                if nat_eq(a5294, a5295) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a5296 = v13;
                        let a5297 = 0u64;
                        if nat_eq(a5296, a5297) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a5298 = v0;
            let a5299 = 28u64;
            if nat_eq(a5298, a5299) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a5300 = v0;
                    let a5301 = 29u64;
                    nat_sub(a5300, a5301)
                };
                let v16: u64 = {
                    let a5302 = v15;
                    let a5303 = 2u64;
                    let a5304 = 0u64;
                    nat_quot(a5302, a5303, a5304)
                };
                let v17: u64 = f61(v6, v16, v1, v2, 0u64)?;
                let a5308 = {
                    let a5305 = v15;
                    let a5306 = 2u64;
                    let a5307 = 0u64;
                    nat_rem(a5305, a5306, a5307)
                };
                let a5309 = 0u64;
                if nat_eq(a5308, a5309) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f179(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5310 = v0;
        let a5311 = 9u64;
        let a5312 = 0u64;
        nat_rem(a5310, a5311, a5312)
    };
    let v5: u64 = {
        let a5316 = {
            let a5313 = v0;
            let a5314 = 9u64;
            let a5315 = 0u64;
            nat_quot(a5313, a5314, a5315)
        };
        let a5317 = 4u64;
        let a5318 = 0u64;
        nat_rem(a5316, a5317, a5318)
    };
    let v6: bool = {
        let a5325 = {
            let a5322 = {
                let a5319 = v0;
                let a5320 = 36u64;
                let a5321 = 0u64;
                nat_quot(a5319, a5320, a5321)
            };
            let a5323 = 2u64;
            let a5324 = 0u64;
            nat_rem(a5322, a5323, a5324)
        };
        let a5326 = 0u64;
        nat_eq(a5325, a5326)
    };
    let v7: u64 = {
        let a5327 = v4;
        let a5328 = 4u64;
        if nat_lt(a5327, a5328) {
            v4
        } else {
            let a5329 = v4;
            let a5330 = 5u64;
            nat_sub(a5329, a5330)
        }
    };
    let v8: u64 = f61(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a5331 = v4;
    let a5332 = 4u64;
    if nat_lt(a5331, a5332) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a5333 = v4;
            let a5334 = 0u64;
            if nat_eq(a5333, a5334) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a5335 = v4;
        let a5336 = 4u64;
        if nat_eq(a5335, a5336) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f180(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a5339 = {
            let a5337 = 3u64;
            let a5338 = v1;
            nat_mul(a5337, a5338)?
        };
        let a5340 = 4u64;
        nat_add(a5339, a5340)
    } else {
        let a5341 = v1;
        let a5342 = 3u64;
        nat_add(a5341, a5342)
    }
}

pub fn f181(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m5351 = f64(match v0.clone() {
        Adt7::C0(h5343, _, _, _, _) => {
            h5343
        }
    }, match v1.clone() {
        Adt3::C0(h5344, _, _, _, _, _, _, _) => {
            h5344
        }
    }, f62(v3, v2.clone(), 0u64), {
        let a5348 = v4;
        let a5349 = {
            let m5347 = match v0.clone() {
                Adt7::C0(h5346, _, _, _, _) => {
                    h5346
                }
            };
            match m5347 {
                Adt6::C0(h5345, _, _, _) => {
                    h5345
                }
            }
        };
        let a5350 = 0u64;
        nat_rem(a5348, a5349, a5350)
    })?;
    match m5351 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m5360 = f64(match v0.clone() {
                Adt7::C0(_, h5352, _, _, _) => {
                    h5352
                }
            }, match v1.clone() {
                Adt3::C0(_, h5353, _, _, _, _, _, _) => {
                    h5353
                }
            }, f62(v3, v2.clone(), 1u64), {
                let a5357 = v4;
                let a5358 = {
                    let m5356 = match v0 {
                        Adt7::C0(h5355, _, _, _, _) => {
                            h5355
                        }
                    };
                    match m5356 {
                        Adt6::C0(h5354, _, _, _) => {
                            h5354
                        }
                    }
                };
                let a5359 = 0u64;
                nat_quot(a5357, a5358, a5359)
            })?;
            match m5360 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f61(v1, v5, v6, f62(v3, v2.clone(), 2u64), f62(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f182(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a5362 = v2;
        let a5363 = f152(match v0.clone() {
            Adt6::C0(_, _, h5361, _) => {
                h5361
            }
        });
        nat_add(a5362, a5363)?
    };
    let v5: u64 = {
        let a5368 = {
            let a5365 = v3;
            let a5366 = match v0.clone() {
                Adt6::C0(_, _, _, h5364) => {
                    h5364
                }
            };
            nat_mul(a5365, a5366)?
        };
        let a5369 = f151(match v0.clone() {
            Adt6::C0(_, _, h5367, _) => {
                h5367
            }
        });
        nat_add(a5368, a5369)?
    };
    let v6: u64 = {
        let a5370 = v4;
        let a5371 = v5;
        nat_sub(a5370, a5371)
    };
    let v7: u64 = match v0 {
        Adt6::C0(_, h5372, _, _) => {
            h5372
        }
    };
    let v8: u64 = {
        let a5373 = v6;
        let a5374 = v7;
        let a5375 = 0u64;
        nat_quot(a5373, a5374, a5375)
    };
    let a5376 = v5;
    let a5377 = v4;
    if if if nat_le(a5376, a5377) {
        let a5381 = {
            let a5378 = v6;
            let a5379 = v7;
            let a5380 = 0u64;
            nat_rem(a5378, a5379, a5380)
        };
        let a5382 = 0u64;
        nat_eq(a5381, a5382)
    } else {
        false
    } {
        let a5383 = v8;
        let a5384 = v1;
        nat_lt(a5383, a5384)
    } else {
        false
    } {
        Ok(Some(v8))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f183(v0: bool, v1: u64) -> R<u64> {
    let a5387 = if v0 {
        8u64
    } else {
        3u64
    };
    let a5388 = {
        let a5385 = v1;
        let a5386 = 0u64;
        if nat_eq(a5385, a5386) {
            0u64
        } else {
            1u64
        }
    };
    nat_add(a5387, a5388)
}

pub fn f184(v0: u64) -> u64 {
    let m5389 = f226(v0);
    match m5389 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a5391 = match v1.clone() {
                Adt19::C0(_, h5390, _) => {
                    h5390
                }
            };
            let a5392 = 1u64;
            if if nat_eq(a5391, a5392) {
                let a5394 = match v1.clone() {
                    Adt19::C0(_, _, h5393) => {
                        h5393
                    }
                };
                let a5395 = 1u64;
                if if if nat_eq(a5394, a5395) {
                    true
                } else {
                    let a5397 = match v1.clone() {
                        Adt19::C0(_, _, h5396) => {
                            h5396
                        }
                    };
                    let a5398 = 2u64;
                    nat_eq(a5397, a5398)
                } {
                    true
                } else {
                    let a5400 = match v1.clone() {
                        Adt19::C0(_, _, h5399) => {
                            h5399
                        }
                    };
                    let a5401 = 4u64;
                    nat_eq(a5400, a5401)
                } {
                    true
                } else {
                    let a5403 = match v1.clone() {
                        Adt19::C0(_, _, h5402) => {
                            h5402
                        }
                    };
                    let a5404 = 8u64;
                    nat_eq(a5403, a5404)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h5405) => {
                        h5405
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f185(v0: u64, v1: u64, v2: Adt1, v3: Adt13) -> Adt9 {
    let a5406 = v0;
    let a5407 = 2u64;
    if nat_eq(a5406, a5407) {
        let a5408 = v1;
        let a5409 = 2u64;
        if nat_eq(a5408, a5409) {
            Adt9::C7(Adt11::C5(3u64), Adt12::C2(5u64), Adt12::C2(0u64), Adt13::C0(2u64), 4u64, 4u64)
        } else {
            Adt9::C7(Adt11::C5(0u64), Adt12::C1(0u64), Adt12::C2(match v2 {
                Adt1::C0(_, _, _, _, h5410, _, _, _, _, _, _, _) => {
                    h5410
                }
            }), Adt13::C0(2u64), 4u64, 4u64)
        }
    } else {
        let a5411 = v0;
        let a5412 = 3u64;
        if nat_eq(a5411, a5412) {
            let a5413 = v1;
            let a5414 = 5u64;
            if nat_eq(a5413, a5414) {
                Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                    Adt1::C0(_, _, h5415, _, _, _, _, _, _, _, _, _) => {
                        h5415
                    }
                }), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                Adt9::C7(Adt11::C3(0u64, 0u64, false), {
                    let a5416 = v1;
                    let a5417 = 0u64;
                    if if if nat_eq(a5416, a5417) {
                        true
                    } else {
                        let a5418 = v1;
                        let a5419 = 3u64;
                        nat_eq(a5418, a5419)
                    } {
                        true
                    } else {
                        let a5420 = v1;
                        let a5421 = 1u64;
                        if nat_eq(a5420, a5421) {
                            let a5423 = match v2.clone() {
                                Adt1::C0(_, _, _, _, h5422, _, _, _, _, _, _, _) => {
                                    h5422
                                }
                            };
                            let a5424 = 2147483648u64;
                            nat_lt(a5423, a5424)
                        } else {
                            false
                        }
                    } {
                        Adt12::C1(0u64)
                    } else {
                        Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, _, h5425, _, _, _, _, _, _, _) => {
                                h5425
                            }
                        })
                    }
                }, Adt12::C2(13u64), Adt13::C0(1u64), 4u64, 4u64)
            }
        } else {
            let a5426 = v0;
            let a5427 = 4u64;
            if nat_eq(a5426, a5427) {
                Adt9::C7({
                    let a5428 = v1;
                    let a5429 = 5u64;
                    if nat_eq(a5428, a5429) {
                        Adt11::C1(2u64)
                    } else {
                        Adt11::C1(1u64)
                    }
                }, Adt12::C1(1u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64)
            } else {
                let a5430 = v0;
                let a5431 = 5u64;
                if nat_eq(a5430, a5431) {
                    let a5432 = v1;
                    let a5433 = 5u64;
                    if nat_eq(a5432, a5433) {
                        Adt9::C7(Adt11::C5(3u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64)
                    } else {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C2(match v2 {
                            Adt1::C0(_, h5434, _, _, _, _, _, _, _, _, _, _) => {
                                h5434
                            }
                        }), Adt13::C0(1u64), 4u64, 4u64)
                    }
                } else {
                    let a5435 = v0;
                    let a5436 = 6u64;
                    if nat_eq(a5435, a5436) {
                        Adt9::C7(Adt11::C1(2u64), Adt12::C2(match v2 {
                            Adt1::C0(_, _, _, h5437, _, _, _, _, _, _, _, _) => {
                                h5437
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

pub fn f186(v0: u64) -> R<u64> {
    let a5440 = v0;
    let a5441 = {
        let a5438 = v0;
        let a5439 = 2u64;
        nat_add(a5438, a5439)?
    };
    nat_mul(a5440, a5441)
}

pub fn f187(v0: u64) -> R<u64> {
    let a5444 = 3u64;
    let a5445 = {
        let a5442 = v0;
        let a5443 = 3u64;
        nat_mul(a5442, a5443)?
    };
    nat_add(a5444, a5445)
}

pub fn f188(v0: u64, v1: u64) -> R<u64> {
    if f189(v0) {
        let a5446 = 2u64;
        let a5447 = v1;
        nat_mul(a5446, a5447)
    } else {
        let a5448 = v1;
        let a5449 = f264(v0);
        let a5450 = 0u64;
        Ok(nat_quot(a5448, a5449, a5450))
    }
}

pub fn f189(v0: u64) -> bool {
    let a5451 = v0;
    let a5452 = 0u64;
    if if nat_eq(a5451, a5452) {
        true
    } else {
        let a5453 = v0;
        let a5454 = 1u64;
        nat_eq(a5453, a5454)
    } {
        true
    } else {
        let a5455 = v0;
        let a5456 = 30u64;
        nat_eq(a5455, a5456)
    }
}

pub fn f190(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m5457 = f226(v0);
    match m5457 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f150(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h5458, _, _, _) => {
                    h5458
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h5459, _, _) => {
                    h5459
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h5460, _) => {
                    h5460
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h5461) => {
                    h5461
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h5462) => {
                    h5462
                }
            }, f150({
                let a5464 = v2;
                let a5465 = match v6.clone() {
                    Adt19::C0(_, h5463, _) => {
                        h5463
                    }
                };
                nat_mul(a5464, a5465)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h5466, _, _, _) => {
                    h5466
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h5467, _, _) => {
                    h5467
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h5468, _) => {
                    h5468
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h5469) => {
                    h5469
                }
            })?, match v6 {
                Adt19::C0(_, h5470, _) => {
                    h5470
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h5471, _, _, _) => {
                    h5471
                }
            }))
        }
    }
}

pub fn f191(v0: Adt3, v1: Adt3) -> R<u64> {
    let a5479 = {
        let a5472 = 2u64;
        let a5473 = f192(v1.clone())?;
        nat_mul(a5472, a5473)?
    };
    let a5480 = {
        let a5477 = f192(v1)?;
        let a5478 = {
            let a5475 = 1u64;
            let a5476 = match v0 {
                Adt3::C0(h5474, _, _, _, _, _, _, _) => {
                    h5474
                }
            };
            nat_add(a5475, a5476)?
        };
        nat_mul(a5477, a5478)?
    };
    nat_add(a5479, a5480)
}

pub fn f192(v0: Adt3) -> R<u64> {
    if f52(match v0.clone() {
        Adt3::C0(h5481, _, _, _, _, _, _, _) => {
            h5481
        }
    }, match v0.clone() {
        Adt3::C0(_, h5482, _, _, _, _, _, _) => {
            h5482
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h5483, _, _, _, _, _) => {
            h5483
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h5484, _, _, _, _) => {
            h5484
        }
    }) {
        Ok(0u64)
    } else {
        let a5493 = match v0.clone() {
            Adt3::C0(h5485, _, _, _, _, _, _, _) => {
                h5485
            }
        };
        let a5494 = {
            let a5491 = match v0.clone() {
                Adt3::C0(_, h5486, _, _, _, _, _, _) => {
                    h5486
                }
            };
            let a5492 = {
                let a5489 = match v0.clone() {
                    Adt3::C0(_, _, h5487, _, _, _, _, _) => {
                        h5487
                    }
                };
                let a5490 = match v0 {
                    Adt3::C0(_, _, _, h5488, _, _, _, _) => {
                        h5488
                    }
                };
                nat_mul(a5489, a5490)?
            };
            nat_mul(a5491, a5492)?
        };
        nat_mul(a5493, a5494)
    }
}

pub fn f193(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f99(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f194(v0: u64, v1: u64) -> bool {
    let a5495 = v1;
    let a5496 = 2147483647u64;
    if nat_le(a5495, a5496) {
        let a5497 = v1;
        let a5498 = v0;
        nat_lt(a5497, a5498)
    } else {
        false
    }
}

pub fn f195(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m5499 = f226(v0);
    match m5499 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a5501 = v2;
                let a5502 = match v7.clone() {
                    Adt19::C0(_, h5500, _) => {
                        h5500
                    }
                };
                nat_mul(a5501, a5502)?
            };
            let m5515 = f265(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h5503, _, _, _, _, _, _, _) => {
                    h5503
                }
            }, match v5.clone() {
                Adt3::C0(_, h5504, _, _, _, _, _, _) => {
                    h5504
                }
            }, match v5 {
                Adt3::C0(_, _, h5505, _, _, _, _, _) => {
                    h5505
                }
            }, {
                let a5508 = match v3.clone() {
                    Adt3::C0(h5506, _, _, _, _, _, _, _) => {
                        h5506
                    }
                };
                let a5509 = match v7.clone() {
                    Adt19::C0(_, h5507, _) => {
                        h5507
                    }
                };
                nat_mul(a5508, a5509)?
            }, match v3.clone() {
                Adt3::C0(_, h5510, _, _, _, _, _, _) => {
                    h5510
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h5511, _, _, _) => {
                    h5511
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h5512, _, _) => {
                    h5512
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h5513, _) => {
                    h5513
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h5514) => {
                    h5514
                }
            })?;
            match m5515 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a5516 = v0;
                    let a5517 = 0u64;
                    if if nat_eq(a5516, a5517) {
                        true
                    } else {
                        let a5518 = v0;
                        let a5519 = 26u64;
                        nat_eq(a5518, a5519)
                    } {
                        Ok(Adt9::C1(false, v9, f99({
                            let a5523 = {
                                let a5521 = v1;
                                let a5522 = match v4.clone() {
                                    Adt3::C0(h5520, _, _, _, _, _, _, _) => {
                                        h5520
                                    }
                                };
                                nat_mul(a5521, a5522)?
                            };
                            let a5524 = v8;
                            nat_add(a5523, a5524)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h5525) => {
                                h5525
                            }
                        }, f99({
                            let a5529 = {
                                let a5527 = v1;
                                let a5528 = match v4.clone() {
                                    Adt3::C0(h5526, _, _, _, _, _, _, _) => {
                                        h5526
                                    }
                                };
                                nat_mul(a5527, a5528)?
                            };
                            let a5530 = v8;
                            nat_add(a5529, a5530)?
                        }, v4.clone())?, {
                            let a5537 = {
                                let a5535 = {
                                    let a5532 = match v7 {
                                        Adt19::C0(_, h5531, _) => {
                                            h5531
                                        }
                                    };
                                    let a5533 = 1u64;
                                    nat_sub(a5532, a5533)
                                };
                                let a5536 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h5534, _, _, _) => {
                                        h5534
                                    }
                                };
                                nat_mul(a5535, a5536)?
                            };
                            let a5538 = 4u64;
                            nat_add(a5537, a5538)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h5539, _, _, _) => {
                                h5539
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f196(v0: u64, v1: u64, v2: u64) -> u64 {
    let a5540 = v1;
    let a5541 = 0u64;
    if if nat_eq(a5540, a5541) {
        true
    } else {
        let a5542 = v2;
        let a5543 = 0u64;
        nat_eq(a5542, a5543)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f197(v0: u64, v1: u64) -> bool {
    let a5544 = v0;
    let a5545 = 4294967295u64;
    if nat_eq(a5544, a5545) {
        true
    } else {
        let a5546 = v0;
        let a5547 = 2147483648u64;
        if nat_lt(a5546, a5547) {
            let a5548 = v0;
            let a5549 = v1;
            nat_lt(a5548, a5549)
        } else {
            false
        }
    }
}

pub fn f198(v0: Adt7) -> R<u64> {
    let a5561 = if match v0.clone() {
        Adt7::C0(_, _, _, h5550, _) => {
            h5550
        }
    } {
        2u64
    } else {
        3u64
    };
    let a5562 = {
        let a5559 = {
            let a5557 = {
                let m5553 = match v0.clone() {
                    Adt7::C0(h5552, _, _, _, _) => {
                        h5552
                    }
                };
                match m5553 {
                    Adt6::C0(h5551, _, _, _) => {
                        h5551
                    }
                }
            };
            let a5558 = {
                let m5556 = match v0.clone() {
                    Adt7::C0(_, h5555, _, _, _) => {
                        h5555
                    }
                };
                match m5556 {
                    Adt6::C0(h5554, _, _, _) => {
                        h5554
                    }
                }
            };
            nat_mul(a5557, a5558)?
        };
        let a5560 = f199(v0)?;
        nat_mul(a5559, a5560)?
    };
    nat_add(a5561, a5562)
}

pub fn f199(v0: Adt7) -> R<u64> {
    let a5567 = if match v0.clone() {
        Adt7::C0(_, _, _, h5563, _) => {
            h5563
        }
    } {
        2u64
    } else {
        1u64
    };
    let a5568 = {
        let a5565 = match v0 {
            Adt7::C0(_, _, _, _, h5564) => {
                h5564
            }
        };
        let a5566 = 1u64;
        if nat_eq(a5565, a5566) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a5567, a5568)
}

pub fn f200(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a5573 = {
            let a5570 = v2;
            let a5571 = match v0.clone() {
                Adt6::C0(_, h5569, _, _) => {
                    h5569
                }
            };
            nat_mul(a5570, a5571)?
        };
        let a5574 = f151(match v0.clone() {
            Adt6::C0(_, _, h5572, _) => {
                h5572
            }
        });
        nat_add(a5573, a5574)?
    };
    let v4: u64 = f152(match v0.clone() {
        Adt6::C0(_, _, h5575, _) => {
            h5575
        }
    });
    let v5: u64 = {
        let a5579 = {
            let a5577 = v3;
            let a5578 = match v0 {
                Adt6::C0(h5576, _, _, _) => {
                    h5576
                }
            };
            nat_add(a5577, a5578)?
        };
        let a5580 = v4;
        nat_sub(a5579, a5580)
    };
    let a5585 = {
        let a5581 = v5;
        let a5582 = v1;
        if nat_lt(a5581, a5582) {
            v5
        } else {
            v1
        }
    };
    let a5586 = {
        let a5583 = v3;
        let a5584 = v4;
        nat_sub(a5583, a5584)
    };
    Ok(nat_sub(a5585, a5586))
}

pub fn f201(v0: u64) -> R<u64> {
    let a5589 = 2u64;
    let a5590 = {
        let a5587 = v0;
        let a5588 = 2u64;
        nat_mul(a5587, a5588)?
    };
    nat_add(a5589, a5590)
}

pub fn f202(v0: Adt6) -> R<u64> {
    let a5603 = 2u64;
    let a5604 = {
        let a5601 = 2u64;
        let a5602 = {
            let a5598 = {
                let a5595 = match v0.clone() {
                    Adt6::C0(h5591, _, _, _) => {
                        h5591
                    }
                };
                let a5596 = {
                    let a5593 = match v0.clone() {
                        Adt6::C0(_, h5592, _, _) => {
                            h5592
                        }
                    };
                    let a5594 = 1u64;
                    nat_sub(a5593, a5594)
                };
                nat_add(a5595, a5596)?
            };
            let a5599 = match v0 {
                Adt6::C0(_, h5597, _, _) => {
                    h5597
                }
            };
            let a5600 = 0u64;
            nat_quot(a5598, a5599, a5600)
        };
        nat_mul(a5601, a5602)?
    };
    nat_add(a5603, a5604)
}

pub fn f203(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m5605 = v0;
    match m5605 {
        Adt16::C0(v19) => {
            f266(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a5606 = v20;
            let a5607 = 66u64;
            if nat_eq(a5606, a5607) {
                f267(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f268(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f204(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a5614 = v3;
        let a5615 = {
            let a5611 = f99({
                let a5608 = v0;
                let a5609 = 2u64;
                let a5610 = 0u64;
                nat_quot(a5608, a5609, a5610)
            }, v1)?;
            let a5612 = v2;
            let a5613 = 0u64;
            nat_quot(a5611, a5612, a5613)
        };
        nat_add(a5614, a5615)?
    };
    let a5619 = {
        let a5616 = v0;
        let a5617 = 2u64;
        let a5618 = 0u64;
        nat_rem(a5616, a5617, a5618)
    };
    let a5620 = 0u64;
    if nat_eq(a5619, a5620) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a5621 = v4;
        let a5622 = 0u64;
        if nat_eq(a5621, a5622) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f205(v0: u64, v1: u64) -> R<u64> {
    let a5625 = 8u64;
    let a5626 = {
        let a5623 = f269(v0);
        let a5624 = v1;
        nat_mul(a5623, a5624)?
    };
    nat_add(a5625, a5626)
}

pub fn f206(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h5627, _, _, _, _, _, _, _) => {
            h5627
        }
    };
    let v7: u64 = f269(v3);
    let v8: u64 = {
        let a5630 = 1u64;
        let a5631 = {
            let a5628 = v6;
            let a5629 = v7;
            nat_mul(a5628, a5629)?
        };
        nat_add(a5630, a5631)?
    };
    let a5632 = v0;
    let a5633 = 0u64;
    if nat_eq(a5632, a5633) {
        Ok(f270(0u64))
    } else {
        let a5634 = v0;
        let a5635 = v8;
        if nat_lt(a5634, a5635) {
            let v9: u64 = {
                let a5636 = v0;
                let a5637 = 1u64;
                nat_sub(a5636, a5637)
            };
            let v10: u64 = {
                let a5638 = v9;
                let a5639 = v7;
                let a5640 = 0u64;
                nat_rem(a5638, a5639, a5640)
            };
            let v11: u64 = f99({
                let a5646 = {
                    let a5641 = v1;
                    let a5642 = v6;
                    nat_mul(a5641, a5642)?
                };
                let a5647 = {
                    let a5643 = v9;
                    let a5644 = v7;
                    let a5645 = 0u64;
                    nat_quot(a5643, a5644, a5645)
                };
                nat_add(a5646, a5647)?
            }, v2)?;
            let a5648 = v3;
            let a5649 = 30u64;
            if nat_eq(a5648, a5649) {
                let a5650 = v10;
                let a5651 = 0u64;
                if nat_eq(a5650, a5651) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a5652 = v10;
                    let a5653 = 1u64;
                    if nat_eq(a5652, a5653) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a5654 = v10;
                        let a5655 = 2u64;
                        if nat_eq(a5654, a5655) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a5656 = v10;
                let a5657 = 0u64;
                if nat_eq(a5656, a5657) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a5658 = v10;
                    let a5659 = 1u64;
                    if nat_eq(a5658, a5659) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a5660 = v0;
                let a5661 = v8;
                nat_sub(a5660, a5661)
            };
            let a5662 = v12;
            let a5663 = 0u64;
            if nat_eq(a5662, a5663) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a5664 = v12;
                let a5665 = 1u64;
                if nat_eq(a5664, a5665) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a5666 = v12;
                    let a5667 = 2u64;
                    if nat_eq(a5666, a5667) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a5668 = v12;
                        let a5669 = 3u64;
                        if nat_eq(a5668, a5669) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a5670 = v12;
                            let a5671 = 4u64;
                            if nat_eq(a5670, a5671) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a5672 = v12;
                                let a5673 = 5u64;
                                if nat_eq(a5672, a5673) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a5674 = v12;
                                    let a5675 = 6u64;
                                    if nat_eq(a5674, a5675) {
                                        if f271(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a5676 = 3u64;
                                                let a5677 = v1;
                                                nat_add(a5676, a5677)?
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

pub fn f207(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a5678 = v0;
        let a5679 = f110(v3);
        let a5680 = 0u64;
        nat_quot(a5678, a5679, a5680)
    };
    let v5: u64 = {
        let a5685 = 3u64;
        let a5686 = {
            let a5682 = v4;
            let a5683 = match v1.clone() {
                Adt3::C0(h5681, _, _, _, _, _, _, _) => {
                    h5681
                }
            };
            let a5684 = 0u64;
            nat_quot(a5682, a5683, a5684)
        };
        nat_add(a5685, a5686)?
    };
    let v6: u64 = f99(v4, v1)?;
    let v7: u64 = f99(v4, v2)?;
    let a5687 = v3;
    let a5688 = 30u64;
    if nat_eq(a5687, a5688) {
        let a5692 = {
            let a5689 = v0;
            let a5690 = 3u64;
            let a5691 = 0u64;
            nat_rem(a5689, a5690, a5691)
        };
        let a5693 = 0u64;
        if nat_eq(a5692, a5693) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a5697 = {
                let a5694 = v0;
                let a5695 = 3u64;
                let a5696 = 0u64;
                nat_rem(a5694, a5695, a5696)
            };
            let a5698 = 1u64;
            if nat_eq(a5697, a5698) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f208(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a5699 = v0;
        let a5700 = 8388608u64;
        let a5701 = 0u64;
        nat_quot(a5699, a5700, a5701)
    };
    let a5702 = v0;
    let a5703 = 2147483648u64;
    if if if nat_lt(a5702, a5703) {
        let a5704 = 127u64;
        let a5705 = v1;
        nat_le(a5704, a5705)
    } else {
        false
    } {
        let a5706 = v1;
        let a5707 = 190u64;
        nat_lt(a5706, a5707)
    } else {
        false
    } {
        let v2: u64 = {
            let a5711 = 8388608u64;
            let a5712 = {
                let a5708 = v0;
                let a5709 = 8388608u64;
                let a5710 = 0u64;
                nat_rem(a5708, a5709, a5710)
            };
            nat_add(a5711, a5712)?
        };
        let a5713 = v1;
        let a5714 = 150u64;
        if nat_lt(a5713, a5714) {
            let v3: u64 = f272({
                let a5715 = 150u64;
                let a5716 = v1;
                nat_sub(a5715, a5716)
            });
            let a5720 = {
                let a5717 = v2;
                let a5718 = v3;
                let a5719 = 0u64;
                nat_rem(a5717, a5718, a5719)
            };
            let a5721 = 0u64;
            if nat_eq(a5720, a5721) {
                let a5722 = v2;
                let a5723 = v3;
                let a5724 = 0u64;
                Ok(nat_quot(a5722, a5723, a5724))
            } else {
                Ok(0u64)
            }
        } else {
            let a5727 = f272({
                let a5725 = v1;
                let a5726 = 150u64;
                nat_sub(a5725, a5726)
            });
            let a5728 = v2;
            nat_mul(a5727, a5728)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f209(v0: Adt14, v1: Adt14) -> bool {
    let a5731 = match v0.clone() {
        Adt14::C0(_, h5729, _, _, _, _, _, _, _, _, _) => {
            h5729
        }
    };
    let a5732 = match v1.clone() {
        Adt14::C0(_, h5730, _, _, _, _, _, _, _, _, _) => {
            h5730
        }
    };
    if if if nat_eq(a5731, a5732) {
        let a5735 = match v0.clone() {
            Adt14::C0(_, _, h5733, _, _, _, _, _, _, _, _) => {
                h5733
            }
        };
        let a5736 = match v1.clone() {
            Adt14::C0(_, _, h5734, _, _, _, _, _, _, _, _) => {
                h5734
            }
        };
        nat_eq(a5735, a5736)
    } else {
        false
    } {
        let a5739 = match v0.clone() {
            Adt14::C0(_, _, _, h5737, _, _, _, _, _, _, _) => {
                h5737
            }
        };
        let a5740 = match v1.clone() {
            Adt14::C0(_, _, _, h5738, _, _, _, _, _, _, _) => {
                h5738
            }
        };
        nat_eq(a5739, a5740)
    } else {
        false
    } {
        let a5743 = match v0 {
            Adt14::C0(_, _, _, _, h5741, _, _, _, _, _, _) => {
                h5741
            }
        };
        let a5744 = match v1 {
            Adt14::C0(_, _, _, _, h5742, _, _, _, _, _, _) => {
                h5742
            }
        };
        nat_eq(a5743, a5744)
    } else {
        false
    }
}

pub fn f210(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5745 = v0;
    let a5746 = 0u64;
    if if if nat_eq(a5745, a5746) {
        true
    } else {
        let a5747 = v1;
        let a5748 = 0u64;
        nat_eq(a5747, a5748)
    } {
        true
    } else {
        let a5749 = v2;
        let a5750 = 0u64;
        nat_eq(a5749, a5750)
    } {
        true
    } else {
        let a5751 = v3;
        let a5752 = 0u64;
        nat_eq(a5751, a5752)
    }
}

pub fn f211(v0: Adt14, v1: Adt14) -> bool {
    if f217(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5753, _, _, _, _, _) => {
            h5753
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h5754, _, _, _, _, _) => {
            h5754
        }
    }) {
        let a5756 = match v0 {
            Adt14::C0(_, h5755, _, _, _, _, _, _, _, _, _) => {
                h5755
            }
        };
        let a5757 = 2147483647u64;
        nat_le(a5756, a5757)
    } else {
        true
    }
}

pub fn f212(v0: Adt14) -> bool {
    let a5762 = {
        let a5759 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5758) => {
                h5758
            }
        };
        let a5760 = 4u64;
        let a5761 = 0u64;
        nat_rem(a5759, a5760, a5761)
    };
    let a5763 = 0u64;
    if if nat_eq(a5762, a5763) {
        f273(match v0.clone() {
            Adt14::C0(_, h5764, _, _, _, _, _, _, _, _, _) => {
                h5764
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5765, _, _, _, _, _) => {
                h5765
            }
        })
    } else {
        false
    } {
        f106(match v0.clone() {
            Adt14::C0(_, _, h5766, _, _, _, _, _, _, _, _) => {
                h5766
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5767, _, _, _, _, _, _, _) => {
                h5767
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5768, _, _, _, _, _, _) => {
                h5768
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5769, _, _, _, _) => {
                h5769
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5770, _, _, _) => {
                h5770
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5771, _, _) => {
                h5771
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f213(v0: Adt14, v1: Adt14) -> R<bool> {
    let a5773 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5772, _) => {
            h5772
        }
    };
    let a5774 = 0u64;
    if nat_eq(a5773, a5774) {
        Ok(true)
    } else {
        let a5776 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h5775, _) => {
                h5775
            }
        };
        let a5777 = 0u64;
        if nat_eq(a5776, a5777) {
            Ok(true)
        } else {
            let a5779 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5778) => {
                    h5778
                }
            };
            let a5780 = 0u64;
            if nat_eq(a5779, a5780) {
                Ok(true)
            } else {
                let a5782 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5781) => {
                        h5781
                    }
                };
                let a5783 = 0u64;
                if nat_eq(a5782, a5783) {
                    Ok(true)
                } else {
                    let a5789 = {
                        let a5786 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5784) => {
                                h5784
                            }
                        };
                        let a5787 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h5785, _) => {
                                h5785
                            }
                        };
                        nat_add(a5786, a5787)?
                    };
                    let a5790 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5788) => {
                            h5788
                        }
                    };
                    if nat_le(a5789, a5790) {
                        Ok(true)
                    } else {
                        let a5796 = {
                            let a5793 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5791) => {
                                    h5791
                                }
                            };
                            let a5794 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h5792, _) => {
                                    h5792
                                }
                            };
                            nat_add(a5793, a5794)?
                        };
                        let a5797 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5795) => {
                                h5795
                            }
                        };
                        Ok(nat_le(a5796, a5797))
                    }
                }
            }
        }
    }
}

pub fn f214(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f209(v0.clone(), v1.clone()) {
        let a5800 = match v0.clone() {
            Adt14::C0(h5798, _, _, _, _, _, _, _, _, _, _) => {
                h5798
            }
        };
        let a5801 = match v1.clone() {
            Adt14::C0(h5799, _, _, _, _, _, _, _, _, _, _) => {
                h5799
            }
        };
        nat_eq(a5800, a5801)
    } else {
        false
    } {
        let a5804 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5802) => {
                h5802
            }
        };
        let a5805 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5803) => {
                h5803
            }
        };
        nat_eq(a5804, a5805)
    } else {
        false
    } {
        let a5808 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5806, _, _, _, _, _) => {
                h5806
            }
        };
        let a5809 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h5807, _, _, _, _, _) => {
                h5807
            }
        };
        nat_eq(a5808, a5809)
    } else {
        false
    } {
        let a5812 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5810, _, _, _, _) => {
                h5810
            }
        };
        let a5813 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h5811, _, _, _, _) => {
                h5811
            }
        };
        nat_eq(a5812, a5813)
    } else {
        false
    } {
        let a5816 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5814, _, _, _) => {
                h5814
            }
        };
        let a5817 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5815, _, _, _) => {
                h5815
            }
        };
        nat_eq(a5816, a5817)
    } else {
        false
    } {
        let a5820 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5818, _, _) => {
                h5818
            }
        };
        let a5821 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5819, _, _) => {
                h5819
            }
        };
        nat_eq(a5820, a5821)
    } else {
        false
    }
}

pub fn f215(v0: Adt14) -> R<bool> {
    if f274(match v0.clone() {
        Adt14::C0(_, h5822, _, _, _, _, _, _, _, _, _) => {
            h5822
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h5823, _, _, _, _, _) => {
            h5823
        }
    }) {
        f107(f275(match v0.clone() {
            Adt14::C0(_, h5824, _, _, _, _, _, _, _, _, _) => {
                h5824
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h5825, _, _, _, _, _) => {
                h5825
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h5826, _, _, _, _, _, _, _, _) => {
                h5826
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h5827, _, _, _, _, _, _, _) => {
                h5827
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h5828, _, _, _, _, _, _) => {
                h5828
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h5829, _, _, _, _) => {
                h5829
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h5830, _, _, _) => {
                h5830
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5831, _, _) => {
                h5831
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f216(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5832 = v1;
    let a5833 = 0u64;
    if if nat_eq(a5832, a5833) {
        true
    } else {
        let a5834 = v2;
        let a5835 = 0u64;
        nat_eq(a5834, a5835)
    } {
        Ok(0u64)
    } else {
        let a5838 = f108(v1, v2, v3)?;
        let a5839 = {
            let a5836 = v2;
            let a5837 = f276(v0, v1)?;
            nat_mul(a5836, a5837)?
        };
        nat_add(a5838, a5839)
    }
}

pub fn f217(v0: u64, v1: u64) -> bool {
    let a5840 = v0;
    let a5841 = 4u64;
    if nat_eq(a5840, a5841) {
        let a5842 = v1;
        let a5843 = 4u64;
        nat_eq(a5842, a5843)
    } else {
        false
    }
}

pub fn f218(v0: u64) -> R<u64> {
    let a5844 = 6u64;
    let a5845 = v0;
    nat_add(a5844, a5845)
}

pub fn f219(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h5846, _, _, _, _, _, _, _, _, _, _) => {
            h5846
        }
    }, match v0.clone() {
        Adt14::C0(_, h5847, _, _, _, _, _, _, _, _, _) => {
            h5847
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h5848, _, _, _, _, _, _, _, _) => {
            h5848
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h5849, _, _, _, _, _, _, _) => {
            h5849
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h5850, _, _, _, _, _, _) => {
            h5850
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h5851, _, _, _, _) => {
            h5851
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h5852, _, _, _) => {
            h5852
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h5853, _, _) => {
            h5853
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h5854, _) => {
            h5854
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h5855) => {
            h5855
        }
    })
}

pub fn f220(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a5856 = v0;
    let a5857 = 2147483647u64;
    if if if nat_le(a5856, a5857) {
        let a5858 = 0u64;
        let a5859 = v3;
        if nat_lt(a5858, a5859) {
            let a5860 = v3;
            let a5861 = 2147483647u64;
            nat_le(a5860, a5861)
        } else {
            false
        }
    } else {
        false
    } {
        let a5866 = {
            let a5864 = {
                let a5862 = v2;
                let a5863 = v3;
                nat_add(a5862, a5863)?
            };
            let a5865 = 1u64;
            nat_sub(a5864, a5865)
        };
        let a5867 = 2147483647u64;
        nat_le(a5866, a5867)
    } else {
        false
    } {
        let a5872 = {
            let a5870 = {
                let a5868 = v0;
                let a5869 = v1;
                nat_mul(a5868, a5869)?
            };
            let a5871 = f277(v2, v3)?;
            nat_mul(a5870, a5871)?
        };
        let a5873 = 9223372036854775807u64;
        Ok(nat_le(a5872, a5873))
    } else {
        Ok(false)
    }
}

pub fn f221(v0: Adt14) -> R<u64> {
    let a5904 = {
        let a5896 = {
            let a5888 = {
                let a5880 = 4u64;
                let a5881 = {
                    let a5878 = {
                        let a5875 = match v0.clone() {
                            Adt14::C0(_, h5874, _, _, _, _, _, _, _, _, _) => {
                                h5874
                            }
                        };
                        let a5876 = 1u64;
                        nat_sub(a5875, a5876)
                    };
                    let a5879 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h5877, _, _, _, _, _) => {
                            h5877
                        }
                    };
                    nat_mul(a5878, a5879)?
                };
                nat_add(a5880, a5881)?
            };
            let a5889 = {
                let a5886 = {
                    let a5883 = match v0.clone() {
                        Adt14::C0(_, _, h5882, _, _, _, _, _, _, _, _) => {
                            h5882
                        }
                    };
                    let a5884 = 1u64;
                    nat_sub(a5883, a5884)
                };
                let a5887 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h5885, _, _, _, _) => {
                        h5885
                    }
                };
                nat_mul(a5886, a5887)?
            };
            nat_add(a5888, a5889)?
        };
        let a5897 = {
            let a5894 = {
                let a5891 = match v0.clone() {
                    Adt14::C0(_, _, _, h5890, _, _, _, _, _, _, _) => {
                        h5890
                    }
                };
                let a5892 = 1u64;
                nat_sub(a5891, a5892)
            };
            let a5895 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h5893, _, _, _) => {
                    h5893
                }
            };
            nat_mul(a5894, a5895)?
        };
        nat_add(a5896, a5897)?
    };
    let a5905 = {
        let a5902 = {
            let a5899 = match v0.clone() {
                Adt14::C0(_, _, _, _, h5898, _, _, _, _, _, _) => {
                    h5898
                }
            };
            let a5900 = 1u64;
            nat_sub(a5899, a5900)
        };
        let a5903 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h5901, _, _) => {
                h5901
            }
        };
        nat_mul(a5902, a5903)?
    };
    nat_add(a5904, a5905)
}

pub fn f222(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a5906 = v0;
    let a5907 = 0u64;
    if if if if nat_eq(a5906, a5907) {
        true
    } else {
        let a5908 = v1;
        let a5909 = 0u64;
        nat_eq(a5908, a5909)
    } {
        true
    } else {
        let a5910 = v2;
        let a5911 = 0u64;
        nat_eq(a5910, a5911)
    } {
        true
    } else {
        let a5912 = v3;
        let a5913 = 0u64;
        nat_eq(a5912, a5913)
    } {
        Ok(0u64)
    } else {
        let a5914 = f278(v0, v1, v2, v3, v5)?;
        let a5915 = f279(v0, v1, v2, v3, v4)?;
        nat_add(a5914, a5915)
    }
}

pub fn f223(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5916 = v0;
    let a5917 = 0u64;
    if if nat_eq(a5916, a5917) {
        true
    } else {
        let a5918 = v1;
        let a5919 = 0u64;
        nat_eq(a5918, a5919)
    } {
        Ok(0u64)
    } else {
        let a5922 = f108(v0, v1, v2)?;
        let a5923 = {
            let a5920 = v1;
            let a5921 = f280(v0)?;
            nat_mul(a5920, a5921)?
        };
        nat_add(a5922, a5923)
    }
}

pub fn f224(v0: u64, v1: u64, v2: u64) -> bool {
    let a5924 = v0;
    let a5925 = 1u64;
    if nat_le(a5924, a5925) {
        true
    } else {
        let a5929 = {
            let a5926 = v1;
            let a5927 = v2;
            let a5928 = 1u64;
            nat_rem(a5926, a5927, a5928)
        };
        let a5930 = 0u64;
        nat_eq(a5929, a5930)
    }
}

pub fn f225(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a5931 = v1;
    let a5932 = 1u64;
    if if if nat_le(a5931, a5932) {
        true
    } else {
        let a5933 = v0;
        let a5934 = v2;
        nat_le(a5933, a5934)
    } {
        let a5935 = v3;
        let a5936 = 1u64;
        if nat_le(a5935, a5936) {
            true
        } else {
            let a5943 = {
                let a5941 = v0;
                let a5942 = {
                    let a5939 = {
                        let a5937 = v1;
                        let a5938 = 1u64;
                        nat_sub(a5937, a5938)
                    };
                    let a5940 = v2;
                    nat_mul(a5939, a5940)?
                };
                nat_add(a5941, a5942)?
            };
            let a5944 = v4;
            nat_le(a5943, a5944)
        }
    } else {
        false
    } {
        let a5945 = v5;
        let a5946 = 1u64;
        if nat_le(a5945, a5946) {
            Ok(true)
        } else {
            let a5959 = {
                let a5957 = {
                    let a5951 = v0;
                    let a5952 = {
                        let a5949 = {
                            let a5947 = v1;
                            let a5948 = 1u64;
                            nat_sub(a5947, a5948)
                        };
                        let a5950 = v2;
                        nat_mul(a5949, a5950)?
                    };
                    nat_add(a5951, a5952)?
                };
                let a5958 = {
                    let a5955 = {
                        let a5953 = v3;
                        let a5954 = 1u64;
                        nat_sub(a5953, a5954)
                    };
                    let a5956 = v4;
                    nat_mul(a5955, a5956)?
                };
                nat_add(a5957, a5958)?
            };
            let a5960 = v6;
            Ok(nat_le(a5959, a5960))
        }
    } else {
        Ok(false)
    }
}

pub fn f226(v0: u64) -> Adt18 {
    let a5961 = v0;
    let a5962 = 0u64;
    if nat_eq(a5961, a5962) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a5963 = v0;
        let a5964 = 1u64;
        if nat_eq(a5963, a5964) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a5965 = v0;
            let a5966 = 2u64;
            if nat_eq(a5965, a5966) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a5967 = v0;
                let a5968 = 3u64;
                if nat_eq(a5967, a5968) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a5969 = v0;
                    let a5970 = 6u64;
                    if nat_eq(a5969, a5970) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a5971 = v0;
                        let a5972 = 7u64;
                        if nat_eq(a5971, a5972) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a5973 = v0;
                            let a5974 = 8u64;
                            if nat_eq(a5973, a5974) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a5975 = v0;
                                let a5976 = 9u64;
                                if nat_eq(a5975, a5976) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a5977 = v0;
                                    let a5978 = 10u64;
                                    if nat_eq(a5977, a5978) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a5979 = v0;
                                        let a5980 = 11u64;
                                        if nat_eq(a5979, a5980) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a5981 = v0;
                                            let a5982 = 12u64;
                                            if nat_eq(a5981, a5982) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a5983 = v0;
                                                let a5984 = 13u64;
                                                if nat_eq(a5983, a5984) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a5985 = v0;
                                                    let a5986 = 14u64;
                                                    if nat_eq(a5985, a5986) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a5987 = v0;
                                                        let a5988 = 15u64;
                                                        if nat_eq(a5987, a5988) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a5989 = v0;
                                                            let a5990 = 16u64;
                                                            if nat_eq(a5989, a5990) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a5991 = v0;
                                                                let a5992 = 17u64;
                                                                if nat_eq(a5991, a5992) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a5993 = v0;
                                                                    let a5994 = 18u64;
                                                                    if nat_eq(a5993, a5994) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a5995 = v0;
                                                                        let a5996 = 19u64;
                                                                        if nat_eq(a5995, a5996) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a5997 = v0;
                                                                            let a5998 = 20u64;
                                                                            if nat_eq(a5997, a5998) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a5999 = v0;
                                                                                let a6000 = 21u64;
                                                                                if nat_eq(a5999, a6000) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a6001 = v0;
                                                                                    let a6002 = 22u64;
                                                                                    if nat_eq(a6001, a6002) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a6003 = v0;
                                                                                        let a6004 = 23u64;
                                                                                        if nat_eq(a6003, a6004) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a6005 = v0;
                                                                                            let a6006 = 24u64;
                                                                                            if nat_eq(a6005, a6006) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a6007 = v0;
                                                                                                let a6008 = 25u64;
                                                                                                if nat_eq(a6007, a6008) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a6009 = v0;
                                                                                                    let a6010 = 26u64;
                                                                                                    if nat_eq(a6009, a6010) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a6011 = v0;
                                                                                                        let a6012 = 27u64;
                                                                                                        if nat_eq(a6011, a6012) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a6013 = v0;
                                                                                                            let a6014 = 28u64;
                                                                                                            if nat_eq(a6013, a6014) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a6015 = v0;
                                                                                                                let a6016 = 29u64;
                                                                                                                if nat_eq(a6015, a6016) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a6017 = v0;
                                                                                                                    let a6018 = 30u64;
                                                                                                                    if nat_eq(a6017, a6018) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a6019 = v0;
                                                                                                                        let a6020 = 34u64;
                                                                                                                        if nat_eq(a6019, a6020) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a6021 = v0;
                                                                                                                            let a6022 = 35u64;
                                                                                                                            if nat_eq(a6021, a6022) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a6023 = v0;
                                                                                                                                let a6024 = 39u64;
                                                                                                                                if nat_eq(a6023, a6024) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a6025 = v0;
                                                                                                                                    let a6026 = 40u64;
                                                                                                                                    if nat_eq(a6025, a6026) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a6027 = v0;
                                                                                                                                        let a6028 = 41u64;
                                                                                                                                        if nat_eq(a6027, a6028) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a6029 = v0;
                                                                                                                                            let a6030 = 42u64;
                                                                                                                                            if nat_eq(a6029, a6030) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a6031 = v0;
                                                                                                                                                let a6032 = 43u64;
                                                                                                                                                if nat_eq(a6031, a6032) {
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

pub fn f227(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = {
        let a6033 = v0;
        let a6034 = 2u64;
        let a6035 = 0u64;
        nat_quot(a6033, a6034, a6035)
    };
    let a6051 = {
        let a6036 = v2;
        let a6037 = 0u64;
        if nat_eq(a6036, a6037) {
            0u64
        } else {
            let a6044 = 7u64;
            let a6045 = {
                let a6042 = v2;
                let a6043 = {
                    let a6040 = 5u64;
                    let a6041 = {
                        let a6038 = 3u64;
                        let a6039 = v1;
                        nat_mul(a6038, a6039)?
                    };
                    nat_add(a6040, a6041)?
                };
                nat_mul(a6042, a6043)?
            };
            nat_add(a6044, a6045)?
        }
    };
    let a6052 = {
        let a6049 = {
            let a6046 = v0;
            let a6047 = 2u64;
            let a6048 = 0u64;
            nat_rem(a6046, a6047, a6048)
        };
        let a6050 = 1u64;
        if nat_eq(a6049, a6050) {
            v1
        } else {
            0u64
        }
    };
    nat_add(a6051, a6052)
}

pub fn f228(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a6054 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6053, _, _, _, _, _, _) => {
                    h6053
                }
            };
            let a6055 = 1u64;
            if if if if if nat_le(a6054, a6055) {
                true
            } else {
                let a6057 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6056, _, _) => {
                        h6056
                    }
                };
                let a6058 = v1;
                nat_eq(a6057, a6058)
            } {
                let a6060 = match v0.clone() {
                    Adt2::C0(_, h6059, _, _, _, _, _, _, _, _, _) => {
                        h6059
                    }
                };
                let a6061 = 1u64;
                if nat_le(a6060, a6061) {
                    true
                } else {
                    let a6066 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6062, _, _, _, _, _) => {
                            h6062
                        }
                    };
                    let a6067 = {
                        let a6064 = v1;
                        let a6065 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h6063, _, _, _, _, _, _) => {
                                h6063
                            }
                        };
                        nat_mul(a6064, a6065)?
                    };
                    nat_eq(a6066, a6067)
                }
            } else {
                false
            } {
                let a6069 = match v0.clone() {
                    Adt2::C0(_, _, h6068, _, _, _, _, _, _, _, _) => {
                        h6068
                    }
                };
                let a6070 = 1u64;
                if nat_le(a6069, a6070) {
                    true
                } else {
                    let a6078 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6071, _, _, _, _) => {
                            h6071
                        }
                    };
                    let a6079 = {
                        let a6076 = {
                            let a6073 = v1;
                            let a6074 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h6072, _, _, _, _, _, _) => {
                                    h6072
                                }
                            };
                            nat_mul(a6073, a6074)?
                        };
                        let a6077 = match v0.clone() {
                            Adt2::C0(_, h6075, _, _, _, _, _, _, _, _, _) => {
                                h6075
                            }
                        };
                        nat_mul(a6076, a6077)?
                    };
                    nat_eq(a6078, a6079)
                }
            } else {
                false
            } {
                let a6081 = match v0.clone() {
                    Adt2::C0(_, _, _, h6080, _, _, _, _, _, _, _) => {
                        h6080
                    }
                };
                let a6082 = 1u64;
                if nat_le(a6081, a6082) {
                    true
                } else {
                    let a6093 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6083, _, _, _) => {
                            h6083
                        }
                    };
                    let a6094 = {
                        let a6091 = {
                            let a6088 = {
                                let a6085 = v1;
                                let a6086 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6084, _, _, _, _, _, _) => {
                                        h6084
                                    }
                                };
                                nat_mul(a6085, a6086)?
                            };
                            let a6089 = match v0.clone() {
                                Adt2::C0(_, h6087, _, _, _, _, _, _, _, _, _) => {
                                    h6087
                                }
                            };
                            nat_mul(a6088, a6089)?
                        };
                        let a6092 = match v0.clone() {
                            Adt2::C0(_, _, h6090, _, _, _, _, _, _, _, _) => {
                                h6090
                            }
                        };
                        nat_mul(a6091, a6092)?
                    };
                    nat_eq(a6093, a6094)
                }
            } else {
                false
            } {
                let a6108 = {
                    let a6105 = {
                        let a6102 = {
                            let a6099 = {
                                let a6096 = v1;
                                let a6097 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h6095, _, _, _, _, _, _) => {
                                        h6095
                                    }
                                };
                                nat_mul(a6096, a6097)?
                            };
                            let a6100 = match v0.clone() {
                                Adt2::C0(_, h6098, _, _, _, _, _, _, _, _, _) => {
                                    h6098
                                }
                            };
                            nat_mul(a6099, a6100)?
                        };
                        let a6103 = match v0.clone() {
                            Adt2::C0(_, _, h6101, _, _, _, _, _, _, _, _) => {
                                h6101
                            }
                        };
                        nat_mul(a6102, a6103)?
                    };
                    let a6106 = match v0.clone() {
                        Adt2::C0(_, _, _, h6104, _, _, _, _, _, _, _) => {
                            h6104
                        }
                    };
                    nat_mul(a6105, a6106)?
                };
                let a6109 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6107, _) => {
                        h6107
                    }
                };
                Ok(nat_le(a6108, a6109))
            } else {
                Ok(false)
            }
        } else {
            let a6111 = match v0.clone() {
                Adt2::C0(_, _, _, h6110, _, _, _, _, _, _, _) => {
                    h6110
                }
            };
            let a6112 = 1u64;
            if if if if if nat_le(a6111, a6112) {
                true
            } else {
                let a6114 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6113, _, _, _) => {
                        h6113
                    }
                };
                let a6115 = v1;
                nat_eq(a6114, a6115)
            } {
                let a6117 = match v0.clone() {
                    Adt2::C0(_, h6116, _, _, _, _, _, _, _, _, _) => {
                        h6116
                    }
                };
                let a6118 = 1u64;
                if nat_le(a6117, a6118) {
                    true
                } else {
                    let a6123 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6119, _, _, _, _, _) => {
                            h6119
                        }
                    };
                    let a6124 = {
                        let a6121 = v1;
                        let a6122 = match v0.clone() {
                            Adt2::C0(_, _, _, h6120, _, _, _, _, _, _, _) => {
                                h6120
                            }
                        };
                        nat_mul(a6121, a6122)?
                    };
                    nat_eq(a6123, a6124)
                }
            } else {
                false
            } {
                let a6126 = match v0.clone() {
                    Adt2::C0(_, _, h6125, _, _, _, _, _, _, _, _) => {
                        h6125
                    }
                };
                let a6127 = 1u64;
                if nat_le(a6126, a6127) {
                    true
                } else {
                    let a6135 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6128, _, _, _, _) => {
                            h6128
                        }
                    };
                    let a6136 = {
                        let a6133 = {
                            let a6130 = v1;
                            let a6131 = match v0.clone() {
                                Adt2::C0(_, _, _, h6129, _, _, _, _, _, _, _) => {
                                    h6129
                                }
                            };
                            nat_mul(a6130, a6131)?
                        };
                        let a6134 = match v0.clone() {
                            Adt2::C0(_, h6132, _, _, _, _, _, _, _, _, _) => {
                                h6132
                            }
                        };
                        nat_mul(a6133, a6134)?
                    };
                    nat_eq(a6135, a6136)
                }
            } else {
                false
            } {
                let a6138 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6137, _, _, _, _, _, _) => {
                        h6137
                    }
                };
                let a6139 = 1u64;
                if nat_le(a6138, a6139) {
                    true
                } else {
                    let a6150 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6140, _, _) => {
                            h6140
                        }
                    };
                    let a6151 = {
                        let a6148 = {
                            let a6145 = {
                                let a6142 = v1;
                                let a6143 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6141, _, _, _, _, _, _, _) => {
                                        h6141
                                    }
                                };
                                nat_mul(a6142, a6143)?
                            };
                            let a6146 = match v0.clone() {
                                Adt2::C0(_, h6144, _, _, _, _, _, _, _, _, _) => {
                                    h6144
                                }
                            };
                            nat_mul(a6145, a6146)?
                        };
                        let a6149 = match v0.clone() {
                            Adt2::C0(_, _, h6147, _, _, _, _, _, _, _, _) => {
                                h6147
                            }
                        };
                        nat_mul(a6148, a6149)?
                    };
                    nat_eq(a6150, a6151)
                }
            } else {
                false
            } {
                let a6165 = {
                    let a6162 = {
                        let a6159 = {
                            let a6156 = {
                                let a6153 = v1;
                                let a6154 = match v0.clone() {
                                    Adt2::C0(_, _, _, h6152, _, _, _, _, _, _, _) => {
                                        h6152
                                    }
                                };
                                nat_mul(a6153, a6154)?
                            };
                            let a6157 = match v0.clone() {
                                Adt2::C0(_, h6155, _, _, _, _, _, _, _, _, _) => {
                                    h6155
                                }
                            };
                            nat_mul(a6156, a6157)?
                        };
                        let a6160 = match v0.clone() {
                            Adt2::C0(_, _, h6158, _, _, _, _, _, _, _, _) => {
                                h6158
                            }
                        };
                        nat_mul(a6159, a6160)?
                    };
                    let a6163 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h6161, _, _, _, _, _, _) => {
                            h6161
                        }
                    };
                    nat_mul(a6162, a6163)?
                };
                let a6166 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h6164, _) => {
                        h6164
                    }
                };
                Ok(nat_le(a6165, a6166))
            } else {
                Ok(false)
            }
        }
    } else {
        let a6168 = match v0.clone() {
            Adt2::C0(_, h6167, _, _, _, _, _, _, _, _, _) => {
                h6167
            }
        };
        let a6169 = 1u64;
        if if if if if nat_le(a6168, a6169) {
            true
        } else {
            let a6171 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h6170, _, _, _, _, _) => {
                    h6170
                }
            };
            let a6172 = v1;
            nat_eq(a6171, a6172)
        } {
            let a6174 = match v0.clone() {
                Adt2::C0(_, _, h6173, _, _, _, _, _, _, _, _) => {
                    h6173
                }
            };
            let a6175 = 1u64;
            if nat_le(a6174, a6175) {
                true
            } else {
                let a6180 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h6176, _, _, _, _) => {
                        h6176
                    }
                };
                let a6181 = {
                    let a6178 = v1;
                    let a6179 = match v0.clone() {
                        Adt2::C0(_, h6177, _, _, _, _, _, _, _, _, _) => {
                            h6177
                        }
                    };
                    nat_mul(a6178, a6179)?
                };
                nat_eq(a6180, a6181)
            }
        } else {
            false
        } {
            let a6183 = match v0.clone() {
                Adt2::C0(_, _, _, h6182, _, _, _, _, _, _, _) => {
                    h6182
                }
            };
            let a6184 = 1u64;
            if nat_le(a6183, a6184) {
                true
            } else {
                let a6192 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h6185, _, _, _) => {
                        h6185
                    }
                };
                let a6193 = {
                    let a6190 = {
                        let a6187 = v1;
                        let a6188 = match v0.clone() {
                            Adt2::C0(_, h6186, _, _, _, _, _, _, _, _, _) => {
                                h6186
                            }
                        };
                        nat_mul(a6187, a6188)?
                    };
                    let a6191 = match v0.clone() {
                        Adt2::C0(_, _, h6189, _, _, _, _, _, _, _, _) => {
                            h6189
                        }
                    };
                    nat_mul(a6190, a6191)?
                };
                nat_eq(a6192, a6193)
            }
        } else {
            false
        } {
            let a6195 = match v0.clone() {
                Adt2::C0(_, _, _, _, h6194, _, _, _, _, _, _) => {
                    h6194
                }
            };
            let a6196 = 1u64;
            if nat_le(a6195, a6196) {
                true
            } else {
                let a6207 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h6197, _, _) => {
                        h6197
                    }
                };
                let a6208 = {
                    let a6205 = {
                        let a6202 = {
                            let a6199 = v1;
                            let a6200 = match v0.clone() {
                                Adt2::C0(_, h6198, _, _, _, _, _, _, _, _, _) => {
                                    h6198
                                }
                            };
                            nat_mul(a6199, a6200)?
                        };
                        let a6203 = match v0.clone() {
                            Adt2::C0(_, _, h6201, _, _, _, _, _, _, _, _) => {
                                h6201
                            }
                        };
                        nat_mul(a6202, a6203)?
                    };
                    let a6206 = match v0.clone() {
                        Adt2::C0(_, _, _, h6204, _, _, _, _, _, _, _) => {
                            h6204
                        }
                    };
                    nat_mul(a6205, a6206)?
                };
                nat_eq(a6207, a6208)
            }
        } else {
            false
        } {
            let a6222 = {
                let a6219 = {
                    let a6216 = {
                        let a6213 = {
                            let a6210 = v1;
                            let a6211 = match v0.clone() {
                                Adt2::C0(_, h6209, _, _, _, _, _, _, _, _, _) => {
                                    h6209
                                }
                            };
                            nat_mul(a6210, a6211)?
                        };
                        let a6214 = match v0.clone() {
                            Adt2::C0(_, _, h6212, _, _, _, _, _, _, _, _) => {
                                h6212
                            }
                        };
                        nat_mul(a6213, a6214)?
                    };
                    let a6217 = match v0.clone() {
                        Adt2::C0(_, _, _, h6215, _, _, _, _, _, _, _) => {
                            h6215
                        }
                    };
                    nat_mul(a6216, a6217)?
                };
                let a6220 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h6218, _, _, _, _, _, _) => {
                        h6218
                    }
                };
                nat_mul(a6219, a6220)?
            };
            let a6223 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h6221, _) => {
                    h6221
                }
            };
            Ok(nat_le(a6222, a6223))
        } else {
            Ok(false)
        }
    }
}

pub fn f229(v0: Adt2, v1: u64) -> R<bool> {
    let a6225 = match v0.clone() {
        Adt2::C0(h6224, _, _, _, _, _, _, _, _, _, _) => {
            h6224
        }
    };
    let a6226 = 0u64;
    if if nat_eq(a6225, a6226) {
        f22(v0.clone())?
    } else {
        false
    } {
        let a6227 = v1;
        let a6228 = 1u64;
        if nat_le(a6227, a6228) {
            Ok(true)
        } else {
            let a6233 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6229, _, _, _, _) => {
                    h6229
                }
            };
            let a6234 = {
                let a6231 = 4u64;
                let a6232 = match v0 {
                    Adt2::C0(_, h6230, _, _, _, _, _, _, _, _, _) => {
                        h6230
                    }
                };
                nat_mul(a6231, a6232)?
            };
            Ok(nat_eq(a6233, a6234))
        }
    } else {
        Ok(false)
    }
}

pub fn f230(v0: Adt2, v1: u64) -> R<bool> {
    let a6236 = match v0.clone() {
        Adt2::C0(_, _, _, h6235, _, _, _, _, _, _, _) => {
            h6235
        }
    };
    let a6237 = 1u64;
    if if if nat_eq(a6236, a6237) {
        let a6239 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6238, _, _, _, _, _, _) => {
                h6238
            }
        };
        let a6240 = 1u64;
        nat_eq(a6239, a6240)
    } else {
        false
    } {
        let a6242 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h6241, _, _, _, _, _) => {
                h6241
            }
        };
        let a6243 = v1;
        nat_eq(a6242, a6243)
    } else {
        false
    } {
        let a6245 = match v0.clone() {
            Adt2::C0(_, _, h6244, _, _, _, _, _, _, _, _) => {
                h6244
            }
        };
        let a6246 = 1u64;
        if nat_eq(a6245, a6246) {
            Ok(true)
        } else {
            let a6251 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h6247, _, _, _, _) => {
                    h6247
                }
            };
            let a6252 = {
                let a6249 = match v0 {
                    Adt2::C0(_, h6248, _, _, _, _, _, _, _, _, _) => {
                        h6248
                    }
                };
                let a6250 = v1;
                nat_mul(a6249, a6250)?
            };
            Ok(nat_eq(a6251, a6252))
        }
    } else {
        Ok(false)
    }
}

pub fn f231(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a6256 = {
            let a6253 = v2;
            let a6254 = 1u64;
            nat_sub(a6253, a6254)
        };
        let a6257 = match v0.clone() {
            Adt6::C0(_, h6255, _, _) => {
                h6255
            }
        };
        nat_mul(a6256, a6257)?
    };
    let a6259 = 0u64;
    let a6260 = match v0.clone() {
        Adt6::C0(h6258, _, _, _) => {
            h6258
        }
    };
    if if if if if if if if nat_lt(a6259, a6260) {
        let a6262 = match v0.clone() {
            Adt6::C0(h6261, _, _, _) => {
                h6261
            }
        };
        let a6263 = 2147483648u64;
        nat_lt(a6262, a6263)
    } else {
        false
    } {
        let a6265 = 0u64;
        let a6266 = match v0.clone() {
            Adt6::C0(_, h6264, _, _) => {
                h6264
            }
        };
        nat_lt(a6265, a6266)
    } else {
        false
    } {
        let a6268 = match v0.clone() {
            Adt6::C0(_, h6267, _, _) => {
                h6267
            }
        };
        let a6269 = 2147483648u64;
        nat_lt(a6268, a6269)
    } else {
        false
    } {
        let a6270 = v1;
        let a6271 = 2147483648u64;
        nat_lt(a6270, a6271)
    } else {
        false
    } {
        let a6272 = 0u64;
        let a6273 = v2;
        nat_lt(a6272, a6273)
    } else {
        false
    } {
        let a6274 = v3;
        let a6275 = 2147483648u64;
        nat_lt(a6274, a6275)
    } else {
        false
    } {
        let a6287 = {
            let a6282 = {
                let a6279 = v3;
                let a6280 = {
                    let a6277 = match v0.clone() {
                        Adt6::C0(h6276, _, _, _) => {
                            h6276
                        }
                    };
                    let a6278 = 1u64;
                    nat_sub(a6277, a6278)
                };
                nat_add(a6279, a6280)?
            };
            let a6283 = f151(match v0.clone() {
                Adt6::C0(_, _, h6281, _) => {
                    h6281
                }
            });
            nat_add(a6282, a6283)?
        };
        let a6288 = {
            let a6285 = 2147483647u64;
            let a6286 = f152(match v0.clone() {
                Adt6::C0(_, _, h6284, _) => {
                    h6284
                }
            });
            nat_add(a6285, a6286)?
        };
        nat_le(a6287, a6288)
    } else {
        false
    } {
        let a6290 = match v0 {
            Adt6::C0(_, _, h6289, _) => {
                h6289
            }
        };
        let a6291 = 2147483648u64;
        Ok(!nat_eq(a6290, a6291))
    } else {
        Ok(false)
    }
}

pub fn f232(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f281(match v0.clone() {
        Adt2::C0(h6292, _, _, _, _, _, _, _, _, _, _) => {
            h6292
        }
    }, match v0.clone() {
        Adt2::C0(_, h6293, _, _, _, _, _, _, _, _, _) => {
            h6293
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6294, _, _, _, _, _, _, _, _) => {
            h6294
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6295, _, _, _, _, _, _, _) => {
            h6295
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6296, _, _, _, _, _, _) => {
            h6296
        }
    }, match v1.clone() {
        Adt2::C0(h6297, _, _, _, _, _, _, _, _, _, _) => {
            h6297
        }
    }, match v1.clone() {
        Adt2::C0(_, h6298, _, _, _, _, _, _, _, _, _) => {
            h6298
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h6299, _, _, _, _, _, _, _, _) => {
            h6299
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h6300, _, _, _, _, _, _, _) => {
            h6300
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h6301, _, _, _, _, _, _) => {
            h6301
        }
    }, match v2.clone() {
        Adt2::C0(h6302, _, _, _, _, _, _, _, _, _, _) => {
            h6302
        }
    }, match v2.clone() {
        Adt2::C0(_, h6303, _, _, _, _, _, _, _, _, _) => {
            h6303
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h6304, _, _, _, _, _, _, _, _) => {
            h6304
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h6305, _, _, _, _, _, _, _) => {
            h6305
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h6306, _, _, _, _, _, _) => {
            h6306
        }
    }) {
        let m6308 = f226(match v0.clone() {
            Adt2::C0(h6307, _, _, _, _, _, _, _, _, _, _) => {
                h6307
            }
        });
        match m6308 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a6314 = {
                    let a6311 = match v0.clone() {
                        Adt2::C0(_, h6309, _, _, _, _, _, _, _, _, _) => {
                            h6309
                        }
                    };
                    let a6312 = match v3.clone() {
                        Adt19::C0(_, h6310, _) => {
                            h6310
                        }
                    };
                    let a6313 = 0u64;
                    nat_rem(a6311, a6312, a6313)
                };
                let a6315 = 0u64;
                if if nat_eq(a6314, a6315) {
                    if f282(match v0.clone() {
                        Adt2::C0(_, h6316, _, _, _, _, _, _, _, _, _) => {
                            h6316
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6317, _, _, _, _, _, _, _, _, _) => {
                            h6317
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6318, _, _, _, _, _, _, _, _) => {
                            h6318
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6319, _, _, _, _, _, _, _) => {
                            h6319
                        }
                    }) {
                        true
                    } else {
                        let a6321 = 4u64;
                        let a6322 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6320, _, _, _, _, _) => {
                                h6320
                            }
                        };
                        if if nat_le(a6321, a6322) {
                            let a6324 = match v3.clone() {
                                Adt19::C0(_, h6323, _) => {
                                    h6323
                                }
                            };
                            let a6325 = 1u64;
                            if nat_eq(a6324, a6325) {
                                true
                            } else {
                                let a6328 = match v3.clone() {
                                    Adt19::C0(_, _, h6326) => {
                                        h6326
                                    }
                                };
                                let a6329 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h6327, _, _, _, _, _) => {
                                        h6327
                                    }
                                };
                                nat_le(a6328, a6329)
                            }
                        } else {
                            false
                        } {
                            if f35(v0.clone(), v2.clone())? {
                                f35(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f27(v0.clone(), match v3 {
                        Adt19::C0(_, h6330, _) => {
                            h6330
                        }
                    });
                    Ok(Adt10::C4(if f282(match v0.clone() {
                        Adt2::C0(_, h6331, _, _, _, _, _, _, _, _, _) => {
                            h6331
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6332, _, _, _, _, _, _, _, _, _) => {
                            h6332
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h6333, _, _, _, _, _, _, _, _) => {
                            h6333
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h6334, _, _, _, _, _, _, _) => {
                            h6334
                        }
                    }) {
                        0u64
                    } else {
                        f191(v4.clone(), f27(v1.clone(), 1u64))?
                    }, 4u64, v4, f27(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h6335, _, _, _, _, _, _, _, _, _, _) => {
                            h6335
                        }
                    }, f27(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f233(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f234(match v0.clone() {
        Adt2::C0(h6336, _, _, _, _, _, _, _, _, _, _) => {
            h6336
        }
    }) {
        let a6342 = {
            let a6339 = match v0.clone() {
                Adt2::C0(_, h6337, _, _, _, _, _, _, _, _, _) => {
                    h6337
                }
            };
            let a6340 = f264(match v0.clone() {
                Adt2::C0(h6338, _, _, _, _, _, _, _, _, _, _) => {
                    h6338
                }
            });
            let a6341 = 1u64;
            nat_rem(a6339, a6340, a6341)
        };
        let a6343 = 0u64;
        nat_eq(a6342, a6343)
    } else {
        false
    } {
        let a6345 = match v1.clone() {
            Adt2::C0(h6344, _, _, _, _, _, _, _, _, _, _) => {
                h6344
            }
        };
        let a6346 = 26u64;
        nat_eq(a6345, a6346)
    } else {
        false
    } {
        let a6348 = match v2.clone() {
            Adt2::C0(h6347, _, _, _, _, _, _, _, _, _, _) => {
                h6347
            }
        };
        let a6349 = 0u64;
        nat_eq(a6348, a6349)
    } else {
        false
    } {
        let a6352 = match v0.clone() {
            Adt2::C0(_, h6350, _, _, _, _, _, _, _, _, _) => {
                h6350
            }
        };
        let a6353 = match v2.clone() {
            Adt2::C0(_, h6351, _, _, _, _, _, _, _, _, _) => {
                h6351
            }
        };
        nat_eq(a6352, a6353)
    } else {
        false
    } {
        let a6356 = match v1.clone() {
            Adt2::C0(_, h6354, _, _, _, _, _, _, _, _, _) => {
                h6354
            }
        };
        let a6357 = match v0.clone() {
            Adt2::C0(_, _, h6355, _, _, _, _, _, _, _, _) => {
                h6355
            }
        };
        nat_le(a6356, a6357)
    } else {
        false
    } {
        let a6359 = match v0.clone() {
            Adt2::C0(_, _, _, h6358, _, _, _, _, _, _, _) => {
                h6358
            }
        };
        let a6360 = 1u64;
        nat_eq(a6359, a6360)
    } else {
        false
    } {
        let a6362 = match v0.clone() {
            Adt2::C0(_, _, _, _, h6361, _, _, _, _, _, _) => {
                h6361
            }
        };
        let a6363 = 1u64;
        nat_eq(a6362, a6363)
    } else {
        false
    } {
        let a6365 = match v2.clone() {
            Adt2::C0(_, _, _, h6364, _, _, _, _, _, _, _) => {
                h6364
            }
        };
        let a6366 = 1u64;
        nat_eq(a6365, a6366)
    } else {
        false
    } {
        let a6368 = match v2.clone() {
            Adt2::C0(_, _, _, _, h6367, _, _, _, _, _, _) => {
                h6367
            }
        };
        let a6369 = 1u64;
        nat_eq(a6368, a6369)
    } else {
        false
    } {
        let a6371 = match v1.clone() {
            Adt2::C0(_, _, h6370, _, _, _, _, _, _, _, _) => {
                h6370
            }
        };
        let a6372 = 1u64;
        nat_eq(a6371, a6372)
    } else {
        false
    } {
        let a6374 = match v1.clone() {
            Adt2::C0(_, _, _, h6373, _, _, _, _, _, _, _) => {
                h6373
            }
        };
        let a6375 = 1u64;
        nat_eq(a6374, a6375)
    } else {
        false
    } {
        let a6377 = match v1.clone() {
            Adt2::C0(_, _, _, _, h6376, _, _, _, _, _, _) => {
                h6376
            }
        };
        let a6378 = 1u64;
        nat_eq(a6377, a6378)
    } else {
        false
    } {
        if f52(match v2.clone() {
            Adt2::C0(_, h6379, _, _, _, _, _, _, _, _, _) => {
                h6379
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h6380, _, _, _, _, _, _, _, _) => {
                h6380
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h6381, _, _, _, _, _, _, _) => {
                h6381
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h6382, _, _, _, _, _, _) => {
                h6382
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a6384 = match v1.clone() {
                Adt2::C0(_, h6383, _, _, _, _, _, _, _, _, _) => {
                    h6383
                }
            };
            let a6385 = 0u64;
            if nat_eq(a6384, a6385) {
                Ok(Adt10::C4(f23(v2.clone(), 1u64)?, 4u64, f27(v0, 1u64), f27(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a6387 = match v0.clone() {
                    Adt2::C0(_, h6386, _, _, _, _, _, _, _, _, _) => {
                        h6386
                    }
                };
                let a6388 = 2147483647u64;
                if if if if if if nat_le(a6387, a6388) {
                    let a6390 = match v1.clone() {
                        Adt2::C0(_, h6389, _, _, _, _, _, _, _, _, _) => {
                            h6389
                        }
                    };
                    let a6391 = 2147483647u64;
                    nat_le(a6390, a6391)
                } else {
                    false
                } {
                    let a6393 = 4u64;
                    let a6394 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h6392, _, _, _, _, _) => {
                            h6392
                        }
                    };
                    nat_le(a6393, a6394)
                } else {
                    false
                } {
                    let a6400 = {
                        let a6397 = match v2.clone() {
                            Adt2::C0(_, h6395, _, _, _, _, _, _, _, _, _) => {
                                h6395
                            }
                        };
                        let a6398 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h6396, _, _, _, _, _) => {
                                h6396
                            }
                        };
                        nat_mul(a6397, a6398)?
                    };
                    let a6401 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6399, _, _, _, _) => {
                            h6399
                        }
                    };
                    nat_le(a6400, a6401)
                } else {
                    false
                } {
                    f35(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f35(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a6414 = {
                            let a6405 = {
                                let a6403 = 2u64;
                                let a6404 = match v1.clone() {
                                    Adt2::C0(_, h6402, _, _, _, _, _, _, _, _, _) => {
                                        h6402
                                    }
                                };
                                nat_mul(a6403, a6404)?
                            };
                            let a6406 = f23(v2.clone(), 1u64)?;
                            nat_add(a6405, a6406)?
                        };
                        let a6415 = {
                            let a6412 = match v1.clone() {
                                Adt2::C0(_, h6407, _, _, _, _, _, _, _, _, _) => {
                                    h6407
                                }
                            };
                            let a6413 = {
                                let a6410 = 1u64;
                                let a6411 = f188(match v0.clone() {
                                    Adt2::C0(h6408, _, _, _, _, _, _, _, _, _, _) => {
                                        h6408
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h6409, _, _, _, _, _, _, _, _, _) => {
                                        h6409
                                    }
                                })?;
                                nat_add(a6410, a6411)?
                            };
                            nat_mul(a6412, a6413)?
                        };
                        nat_add(a6414, a6415)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h6416, _, _, _, _, _, _, _, _, _) => {
                            h6416
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h6417, _, _, _, _, _, _, _, _, _) => {
                            h6417
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h6418, _, _, _, _, _) => {
                            h6418
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h6419, _, _, _, _) => {
                            h6419
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h6420, _, _, _) => {
                            h6420
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h6421, _, _) => {
                            h6421
                        }
                    }), f27(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h6422, _, _, _, _, _, _, _, _, _, _) => {
                            h6422
                        }
                    }, f27(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f234(v0: u64) -> bool {
    let a6423 = v0;
    let a6424 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a6423, a6424) {
        true
    } else {
        let a6425 = v0;
        let a6426 = 1u64;
        nat_eq(a6425, a6426)
    } {
        true
    } else {
        let a6427 = v0;
        let a6428 = 2u64;
        nat_eq(a6427, a6428)
    } {
        true
    } else {
        let a6429 = v0;
        let a6430 = 3u64;
        nat_eq(a6429, a6430)
    } {
        true
    } else {
        let a6431 = v0;
        let a6432 = 6u64;
        nat_eq(a6431, a6432)
    } {
        true
    } else {
        let a6433 = v0;
        let a6434 = 7u64;
        nat_eq(a6433, a6434)
    } {
        true
    } else {
        let a6435 = v0;
        let a6436 = 8u64;
        nat_eq(a6435, a6436)
    } {
        true
    } else {
        let a6437 = v0;
        let a6438 = 10u64;
        nat_eq(a6437, a6438)
    } {
        true
    } else {
        let a6439 = v0;
        let a6440 = 11u64;
        nat_eq(a6439, a6440)
    } {
        true
    } else {
        let a6441 = v0;
        let a6442 = 12u64;
        nat_eq(a6441, a6442)
    } {
        true
    } else {
        let a6443 = v0;
        let a6444 = 13u64;
        nat_eq(a6443, a6444)
    } {
        true
    } else {
        let a6445 = v0;
        let a6446 = 14u64;
        nat_eq(a6445, a6446)
    } {
        true
    } else {
        let a6447 = v0;
        let a6448 = 16u64;
        nat_eq(a6447, a6448)
    } {
        true
    } else {
        let a6449 = v0;
        let a6450 = 17u64;
        nat_eq(a6449, a6450)
    } {
        true
    } else {
        let a6451 = v0;
        let a6452 = 18u64;
        nat_eq(a6451, a6452)
    } {
        true
    } else {
        let a6453 = v0;
        let a6454 = 19u64;
        nat_eq(a6453, a6454)
    } {
        true
    } else {
        let a6455 = v0;
        let a6456 = 20u64;
        nat_eq(a6455, a6456)
    } {
        true
    } else {
        let a6457 = v0;
        let a6458 = 21u64;
        nat_eq(a6457, a6458)
    } {
        true
    } else {
        let a6459 = v0;
        let a6460 = 22u64;
        nat_eq(a6459, a6460)
    } {
        true
    } else {
        let a6461 = v0;
        let a6462 = 23u64;
        nat_eq(a6461, a6462)
    } {
        true
    } else {
        let a6463 = v0;
        let a6464 = 26u64;
        nat_eq(a6463, a6464)
    } {
        true
    } else {
        let a6465 = v0;
        let a6466 = 29u64;
        nat_eq(a6465, a6466)
    } {
        true
    } else {
        let a6467 = v0;
        let a6468 = 30u64;
        nat_eq(a6467, a6468)
    } {
        true
    } else {
        let a6469 = v0;
        let a6470 = 34u64;
        nat_eq(a6469, a6470)
    } {
        true
    } else {
        let a6471 = v0;
        let a6472 = 35u64;
        nat_eq(a6471, a6472)
    } {
        true
    } else {
        let a6473 = v0;
        let a6474 = 39u64;
        nat_eq(a6473, a6474)
    } {
        true
    } else {
        let a6475 = v0;
        let a6476 = 40u64;
        nat_eq(a6475, a6476)
    } {
        true
    } else {
        let a6477 = v0;
        let a6478 = 41u64;
        nat_eq(a6477, a6478)
    } {
        true
    } else {
        let a6479 = v0;
        let a6480 = 42u64;
        nat_eq(a6479, a6480)
    } {
        true
    } else {
        let a6481 = v0;
        let a6482 = 43u64;
        nat_eq(a6481, a6482)
    }
}

pub fn f235(v0: u64) -> bool {
    if f189(v0) {
        true
    } else {
        let a6483 = v0;
        let a6484 = 43u64;
        nat_eq(a6483, a6484)
    }
}

pub fn f236(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a6487 = match v1.clone() {
        Adt2::C0(h6485, _, _, _, _, _, _, _, _, _, _) => {
            h6485
        }
    };
    let a6488 = match v3.clone() {
        Adt2::C0(h6486, _, _, _, _, _, _, _, _, _, _) => {
            h6486
        }
    };
    if if nat_eq(a6487, a6488) {
        let a6489 = v0;
        let a6490 = 33u64;
        if nat_eq(a6489, a6490) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m6492 = f226(match v1.clone() {
            Adt2::C0(h6491, _, _, _, _, _, _, _, _, _, _) => {
                h6491
            }
        });
        match m6492 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h6493, _) => {
                        h6493
                    }
                };
                let v6: u64 = f23(v1.clone(), v5)?;
                let a6498 = {
                    let a6495 = match v1.clone() {
                        Adt2::C0(_, h6494, _, _, _, _, _, _, _, _, _) => {
                            h6494
                        }
                    };
                    let a6496 = v5;
                    let a6497 = 1u64;
                    nat_rem(a6495, a6496, a6497)
                };
                let a6499 = 0u64;
                if if if nat_eq(a6498, a6499) {
                    let a6504 = {
                        let a6501 = match v3.clone() {
                            Adt2::C0(_, h6500, _, _, _, _, _, _, _, _, _) => {
                                h6500
                            }
                        };
                        let a6502 = v5;
                        let a6503 = 1u64;
                        nat_rem(a6501, a6502, a6503)
                    };
                    let a6505 = 0u64;
                    nat_eq(a6504, a6505)
                } else {
                    false
                } {
                    let a6506 = v6;
                    let a6507 = f23(v3.clone(), v5)?;
                    nat_eq(a6506, a6507)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h6508) => {
                            h6508
                        }
                    }, f27(v1, v5), f27(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f237(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f52(v4, v5, v6, v7) {
        f52(v0, v1, v2, v3)
    } else {
        let a6512 = {
            let a6509 = v0;
            let a6510 = v4;
            let a6511 = 1u64;
            nat_rem(a6509, a6510, a6511)
        };
        let a6513 = 0u64;
        if if if nat_eq(a6512, a6513) {
            let a6517 = {
                let a6514 = v1;
                let a6515 = v5;
                let a6516 = 1u64;
                nat_rem(a6514, a6515, a6516)
            };
            let a6518 = 0u64;
            nat_eq(a6517, a6518)
        } else {
            false
        } {
            let a6522 = {
                let a6519 = v2;
                let a6520 = v6;
                let a6521 = 1u64;
                nat_rem(a6519, a6520, a6521)
            };
            let a6523 = 0u64;
            nat_eq(a6522, a6523)
        } else {
            false
        } {
            let a6527 = {
                let a6524 = v3;
                let a6525 = v7;
                let a6526 = 1u64;
                nat_rem(a6524, a6525, a6526)
            };
            let a6528 = 0u64;
            nat_eq(a6527, a6528)
        } else {
            false
        }
    }
}

pub fn f238(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a6529 = v0;
    let a6530 = v1;
    if nat_eq(a6529, a6530) {
        let a6533 = v4;
        let a6534 = {
            let a6531 = v2;
            let a6532 = v3;
            nat_add(a6531, a6532)?
        };
        Ok(nat_eq(a6533, a6534))
    } else {
        let a6535 = v2;
        let a6536 = v3;
        if nat_eq(a6535, a6536) {
            let a6537 = v4;
            let a6538 = v2;
            Ok(nat_eq(a6537, a6538))
        } else {
            Ok(false)
        }
    }
}

pub fn f239(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a6541 = {
            let a6539 = v0;
            let a6540 = f152(v1);
            nat_add(a6539, a6540)?
        };
        let a6542 = f152(v2);
        nat_add(a6541, a6542)?
    };
    let v5: u64 = {
        let a6543 = f151(v1);
        let a6544 = f151(v2);
        nat_add(a6543, a6544)?
    };
    let a6545 = v1;
    let a6546 = 4294967295u64;
    if if nat_le(a6545, a6546) {
        let a6547 = v2;
        let a6548 = 4294967295u64;
        nat_le(a6547, a6548)
    } else {
        false
    } {
        let a6549 = v5;
        let a6550 = v4;
        if nat_le(a6549, a6550) {
            let a6553 = v3;
            let a6554 = {
                let a6551 = v4;
                let a6552 = v5;
                nat_sub(a6551, a6552)
            };
            Ok(nat_eq(a6553, a6554))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f240(v0: u64, v1: u64) -> R<bool> {
    let a6555 = v1;
    let a6556 = 4294967295u64;
    if nat_le(a6555, a6556) {
        let a6559 = {
            let a6557 = f152(v1);
            let a6558 = f151(v1);
            nat_add(a6557, a6558)?
        };
        let a6560 = v0;
        Ok(nat_lt(a6559, a6560))
    } else {
        Ok(false)
    }
}

pub fn f241(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f283(v0.clone(), false)?;
    let v6: u64 = f283(v0, true)?;
    let a6561 = v6;
    let a6562 = v1;
    if if nat_le(a6561, a6562) {
        let a6569 = {
            let a6567 = {
                let a6563 = v1;
                let a6564 = v5;
                nat_add(a6563, a6564)?
            };
            let a6568 = {
                let a6565 = v3;
                let a6566 = 0u64;
                if nat_eq(a6565, a6566) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a6567, a6568)?
        };
        let a6570 = v2;
        nat_le(a6569, a6570)
    } else {
        false
    } {
        if v4 {
            let a6575 = {
                let a6573 = {
                    let a6571 = v1;
                    let a6572 = v5;
                    nat_add(a6571, a6572)?
                };
                let a6574 = v6;
                nat_sub(a6573, a6574)
            };
            let a6576 = v2;
            Ok(nat_lt(a6575, a6576))
        } else {
            let a6581 = {
                let a6579 = {
                    let a6577 = v1;
                    let a6578 = v5;
                    nat_add(a6577, a6578)?
                };
                let a6580 = v6;
                nat_sub(a6579, a6580)
            };
            let a6582 = v2;
            Ok(nat_le(a6581, a6582))
        }
    } else {
        Ok(false)
    }
}

pub fn f242(v0: u64, v1: u64) -> bool {
    let a6583 = v0;
    let a6584 = 0u64;
    if if nat_eq(a6583, a6584) {
        true
    } else {
        let a6585 = v0;
        let a6586 = 1u64;
        nat_eq(a6585, a6586)
    } {
        let a6587 = v1;
        let a6588 = 4294967296u64;
        nat_lt(a6587, a6588)
    } else {
        false
    }
}

pub fn f243(v0: u64, v1: u64) -> R<u64> {
    let a6593 = {
        let a6591 = v0;
        let a6592 = {
            let a6589 = v1;
            let a6590 = 1u64;
            nat_sub(a6589, a6590)
        };
        nat_add(a6591, a6592)?
    };
    let a6594 = v1;
    let a6595 = 0u64;
    Ok(nat_quot(a6593, a6594, a6595))
}

pub fn f244(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f52(match v0.clone() {
        Adt2::C0(_, h6596, _, _, _, _, _, _, _, _, _) => {
            h6596
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h6597, _, _, _, _, _, _, _, _) => {
            h6597
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h6598, _, _, _, _, _, _, _) => {
            h6598
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h6599, _, _, _, _, _, _) => {
            h6599
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a6602 = match v1.clone() {
                Adt2::C0(_, h6600, _, _, _, _, _, _, _, _, _) => {
                    h6600
                }
            };
            let a6603 = match v1.clone() {
                Adt2::C0(_, _, h6601, _, _, _, _, _, _, _, _) => {
                    h6601
                }
            };
            nat_mul(a6602, a6603)?
        };
        let v5: u64 = {
            let a6605 = v4;
            let a6606 = match v1.clone() {
                Adt2::C0(_, _, _, h6604, _, _, _, _, _, _, _) => {
                    h6604
                }
            };
            nat_mul(a6605, a6606)?
        };
        let a6619 = {
            let a6617 = {
                let a6608 = match v0.clone() {
                    Adt2::C0(_, h6607, _, _, _, _, _, _, _, _, _) => {
                        h6607
                    }
                };
                let a6609 = 1u64;
                nat_sub(a6608, a6609)
            };
            let a6618 = {
                let a6615 = f284(match v0.clone() {
                    Adt2::C0(_, _, h6610, _, _, _, _, _, _, _, _) => {
                        h6610
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h6611, _, _, _, _, _, _, _, _, _) => {
                        h6611
                    }
                })?;
                let a6616 = f284(match v0 {
                    Adt2::C0(_, _, _, h6612, _, _, _, _, _, _, _) => {
                        h6612
                    }
                }, v2, {
                    let a6613 = v3;
                    let a6614 = v5;
                    nat_mul(a6613, a6614)?
                }, v4)?;
                nat_add(a6615, a6616)?
            };
            nat_add(a6617, a6618)?
        };
        let a6620 = f23(v1, 1u64)?;
        Ok(nat_lt(a6619, a6620))
    }
}

pub fn f245(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6621 = v0;
    let a6622 = 2147483648u64;
    if if if if nat_lt(a6621, a6622) {
        let a6623 = v1;
        let a6624 = 2147483648u64;
        nat_lt(a6623, a6624)
    } else {
        false
    } {
        let a6627 = {
            let a6625 = v0;
            let a6626 = v1;
            nat_mul(a6625, a6626)?
        };
        let a6628 = 2147483648u64;
        nat_lt(a6627, a6628)
    } else {
        false
    } {
        let a6633 = {
            let a6631 = {
                let a6629 = v0;
                let a6630 = 1u64;
                nat_sub(a6629, a6630)
            };
            let a6632 = v2;
            nat_mul(a6631, a6632)?
        };
        let a6634 = 2147483648u64;
        nat_lt(a6633, a6634)
    } else {
        false
    } {
        let a6639 = {
            let a6637 = {
                let a6635 = v1;
                let a6636 = 1u64;
                nat_sub(a6635, a6636)
            };
            let a6638 = v2;
            nat_mul(a6637, a6638)?
        };
        let a6640 = 2147483648u64;
        Ok(nat_lt(a6639, a6640))
    } else {
        Ok(false)
    }
}

pub fn f246(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a6641 = v1;
    let a6642 = 13u64;
    if if nat_eq(a6641, a6642) {
        f52(match v0.clone() {
            Adt3::C0(h6643, _, _, _, _, _, _, _) => {
                h6643
            }
        }, match v0.clone() {
            Adt3::C0(_, h6644, _, _, _, _, _, _) => {
                h6644
            }
        }, match v0.clone() {
            Adt3::C0(_, _, h6645, _, _, _, _, _) => {
                h6645
            }
        }, match v0.clone() {
            Adt3::C0(_, _, _, h6646, _, _, _, _) => {
                h6646
            }
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a6652 = match v0.clone() {
                Adt3::C0(_, h6647, _, _, _, _, _, _) => {
                    h6647
                }
            };
            let a6653 = {
                let a6650 = match v0.clone() {
                    Adt3::C0(_, _, h6648, _, _, _, _, _) => {
                        h6648
                    }
                };
                let a6651 = match v0.clone() {
                    Adt3::C0(_, _, _, h6649, _, _, _, _) => {
                        h6649
                    }
                };
                nat_mul(a6650, a6651)?
            };
            nat_mul(a6652, a6653)?
        };
        let a6654 = v1;
        let a6655 = 13u64;
        if nat_eq(a6654, a6655) {
            let a6664 = 2u64;
            let a6665 = if f285(v0.clone(), v2) {
                let a6657 = v3;
                let a6658 = f258(match v0 {
                    Adt3::C0(h6656, _, _, _, _, _, _, _) => {
                        h6656
                    }
                }, false)?;
                nat_mul(a6657, a6658)?
            } else {
                let a6662 = v3;
                let a6663 = {
                    let a6660 = match v0 {
                        Adt3::C0(h6659, _, _, _, _, _, _, _) => {
                            h6659
                        }
                    };
                    let a6661 = 2u64;
                    nat_mul(a6660, a6661)?
                };
                nat_mul(a6662, a6663)?
            };
            nat_add(a6664, a6665)
        } else {
            let a6669 = v3;
            let a6670 = f258(match v0 {
                Adt3::C0(h6666, _, _, _, _, _, _, _) => {
                    h6666
                }
            }, {
                let a6667 = v1;
                let a6668 = 16u64;
                nat_eq(a6667, a6668)
            })?;
            nat_mul(a6669, a6670)
        }
    }
}

pub fn f247(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a6671 = v0;
    let a6672 = 30u64;
    if if nat_eq(a6671, a6672) {
        let a6675 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h6673, _, _, _, _) => {
                h6673
            }
        };
        let a6676 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h6674, _, _, _, _, _) => {
                h6674
            }
        };
        nat_lt(a6675, a6676)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m6677 = v3;
        match m6677 {
            None => {
                f286(v0, v1, match v2.clone() {
                    Adt2::C0(h6678, _, _, _, _, _, _, _, _, _, _) => {
                        h6678
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h6679, _, _, _, _, _, _, _, _, _, _) => {
                        h6679
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h6680, _, _, _, _, _, _, _, _, _) => {
                        h6680
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6681, _, _, _, _, _, _, _, _) => {
                        h6681
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6682, _, _, _, _, _, _, _) => {
                        h6682
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6683, _, _, _, _, _, _) => {
                        h6683
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h6684, _, _, _, _, _, _, _, _, _) => {
                        h6684
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6685, _, _, _, _, _, _, _, _) => {
                        h6685
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6686, _, _, _, _, _, _, _) => {
                        h6686
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6687, _, _, _, _, _, _) => {
                        h6687
                    }
                })
            }
            Some(v5) => {
                f286(v0, v1, match v2.clone() {
                    Adt2::C0(h6688, _, _, _, _, _, _, _, _, _, _) => {
                        h6688
                    }
                }, match v5.clone() {
                    Adt2::C0(h6689, _, _, _, _, _, _, _, _, _, _) => {
                        h6689
                    }
                }, match v4.clone() {
                    Adt2::C0(h6690, _, _, _, _, _, _, _, _, _, _) => {
                        h6690
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h6691, _, _, _, _, _, _, _, _, _) => {
                        h6691
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h6692, _, _, _, _, _, _, _, _) => {
                        h6692
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h6693, _, _, _, _, _, _, _) => {
                        h6693
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h6694, _, _, _, _, _, _) => {
                        h6694
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h6695, _, _, _, _, _, _, _, _, _) => {
                        h6695
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h6696, _, _, _, _, _, _, _, _) => {
                        h6696
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h6697, _, _, _, _, _, _, _) => {
                        h6697
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h6698, _, _, _, _, _, _) => {
                        h6698
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h6699, _, _, _, _, _, _, _, _, _) => {
                        h6699
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h6700, _, _, _, _, _, _, _, _) => {
                        h6700
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h6701, _, _, _, _, _, _, _) => {
                        h6701
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h6702, _, _, _, _, _, _) => {
                        h6702
                    }
                })
            }
        }
    }
}

pub fn f248(v0: u64, v1: u64) -> u64 {
    let a6703 = v0;
    let a6704 = 23u64;
    if nat_eq(a6703, a6704) {
        let a6705 = v1;
        let a6706 = 0u64;
        if nat_eq(a6705, a6706) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f249(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6711 = {
        let a6709 = {
            let a6707 = v0;
            let a6708 = v1;
            nat_sub(a6707, a6708)
        };
        let a6710 = 1u64;
        nat_sub(a6709, a6710)
    };
    let a6712 = v2;
    nat_add(a6711, a6712)
}

pub fn f250(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f150({
        let a6713 = v0;
        let a6714 = v4;
        let a6715 = 0u64;
        nat_rem(a6713, a6714, a6715)
    }, {
        let a6716 = v1;
        let a6717 = v5;
        let a6718 = 0u64;
        nat_rem(a6716, a6717, a6718)
    }, {
        let a6719 = v2;
        let a6720 = v6;
        let a6721 = 0u64;
        nat_rem(a6719, a6720, a6721)
    }, {
        let a6722 = v3;
        let a6723 = v7;
        let a6724 = 0u64;
        nat_rem(a6722, a6723, a6724)
    }, v8, v9, v10, v11)
}

pub fn f251(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6725 = v1;
    let a6726 = 2147483648u64;
    if nat_lt(a6725, a6726) {
        let a6737 = {
            let a6735 = {
                let a6730 = {
                    let a6727 = v0;
                    let a6728 = v2;
                    let a6729 = 0u64;
                    nat_rem(a6727, a6728, a6729)
                };
                let a6731 = v2;
                nat_add(a6730, a6731)?
            };
            let a6736 = {
                let a6732 = v1;
                let a6733 = v2;
                let a6734 = 0u64;
                nat_rem(a6732, a6733, a6734)
            };
            nat_sub(a6735, a6736)
        };
        let a6738 = v2;
        let a6739 = 0u64;
        Ok(nat_rem(a6737, a6738, a6739))
    } else {
        let a6742 = {
            let a6740 = v0;
            let a6741 = f151(v1);
            nat_add(a6740, a6741)?
        };
        let a6743 = v2;
        let a6744 = 0u64;
        Ok(nat_rem(a6742, a6743, a6744))
    }
}

pub fn f252(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a6747 = f152(v1);
    let a6748 = {
        let a6745 = v0;
        let a6746 = f151(v1);
        nat_add(a6745, a6746)?
    };
    if nat_le(a6747, a6748) {
        let a6749 = f253(v0, v1)?;
        let a6750 = v2;
        Ok(nat_lt(a6749, a6750))
    } else {
        Ok(false)
    }
}

pub fn f253(v0: u64, v1: u64) -> R<u64> {
    let a6753 = {
        let a6751 = v0;
        let a6752 = f151(v1);
        nat_add(a6751, a6752)?
    };
    let a6754 = f152(v1);
    Ok(nat_sub(a6753, a6754))
}

pub fn f254(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6755 = v0;
    let a6756 = v1;
    if nat_lt(a6755, a6756) {
        let a6757 = v1;
        let a6758 = v0;
        Ok(nat_sub(a6757, a6758))
    } else {
        let a6761 = v0;
        let a6762 = {
            let a6759 = v1;
            let a6760 = v2;
            nat_add(a6759, a6760)?
        };
        if nat_lt(a6761, a6762) {
            let a6763 = v0;
            let a6764 = v1;
            Ok(nat_sub(a6763, a6764))
        } else {
            let a6771 = {
                let a6767 = 2u64;
                let a6768 = {
                    let a6765 = v2;
                    let a6766 = 1u64;
                    nat_sub(a6765, a6766)
                };
                nat_mul(a6767, a6768)?
            };
            let a6772 = {
                let a6769 = v0;
                let a6770 = v1;
                nat_sub(a6769, a6770)
            };
            Ok(nat_sub(a6771, a6772))
        }
    }
}

pub fn f255(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a6775 = {
        let a6773 = v2;
        let a6774 = f287(v1.clone(), f62(v0, v1.clone(), 0u64), f62(v0, v1.clone(), 1u64), f62(v0, v1.clone(), 2u64), f62(v0, v1.clone(), 3u64), false)?;
        nat_add(a6773, a6774)?
    };
    let a6776 = f287(v1.clone(), f62(v0, v1.clone(), 0u64), f62(v0, v1.clone(), 1u64), f62(v0, v1.clone(), 2u64), f62(v0, v1, 3u64), true)?;
    Ok(nat_sub(a6775, a6776))
}

pub fn f256(v0: u64, v1: u64, v2: u64) -> bool {
    let a6777 = v0;
    let a6778 = 0u64;
    if nat_eq(a6777, a6778) {
        let a6779 = v2;
        let a6780 = v1;
        nat_le(a6779, a6780)
    } else {
        let a6781 = v0;
        let a6782 = 1u64;
        if nat_eq(a6781, a6782) {
            let a6783 = v2;
            let a6784 = v1;
            nat_lt(a6783, a6784)
        } else {
            let a6785 = v0;
            let a6786 = 2u64;
            if nat_eq(a6785, a6786) {
                let a6787 = v1;
                let a6788 = v2;
                nat_le(a6787, a6788)
            } else {
                let a6789 = v0;
                let a6790 = 3u64;
                if nat_eq(a6789, a6790) {
                    let a6791 = v1;
                    let a6792 = v2;
                    nat_lt(a6791, a6792)
                } else {
                    let a6793 = v1;
                    let a6794 = v2;
                    nat_eq(a6793, a6794)
                }
            }
        }
    }
}

pub fn f257(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a6795 = v0;
    let a6796 = 0u64;
    if nat_eq(a6795, a6796) {
        Ok(f270(0u64))
    } else {
        let a6799 = {
            let a6797 = v0;
            let a6798 = 1u64;
            nat_add(a6797, a6798)?
        };
        let a6800 = f246(v1.clone(), 13u64, v2)?;
        if nat_eq(a6799, a6800) {
            Ok(Adt9::C4({
                let a6801 = v2;
                let a6802 = 0u64;
                if nat_eq(a6801, a6802) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a6803 = v0;
                let a6804 = 1u64;
                nat_sub(a6803, a6804)
            };
            if f285(v1.clone(), v2) {
                let v4: u64 = f258(match v1.clone() {
                    Adt3::C0(h6805, _, _, _, _, _, _, _) => {
                        h6805
                    }
                }, false)?;
                let v5: u64 = {
                    let a6806 = v3;
                    let a6807 = v4;
                    let a6808 = 0u64;
                    nat_quot(a6806, a6807, a6808)
                };
                let v6: u64 = {
                    let a6809 = v3;
                    let a6810 = v4;
                    let a6811 = 0u64;
                    nat_rem(a6809, a6810, a6811)
                };
                let a6814 = {
                    let a6812 = v6;
                    let a6813 = 1u64;
                    nat_add(a6812, a6813)?
                };
                let a6815 = v4;
                if nat_eq(a6814, a6815) {
                    Ok(Adt9::C4({
                        let a6816 = v2;
                        let a6817 = 0u64;
                        if nat_eq(a6816, a6817) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f259(v6, v5, v1, v2)
                }
            } else {
                f288({
                    let a6818 = v3;
                    let a6819 = 2u64;
                    let a6820 = 0u64;
                    nat_quot(a6818, a6819, a6820)
                }, {
                    let a6821 = v3;
                    let a6822 = 2u64;
                    let a6823 = 0u64;
                    nat_rem(a6821, a6822, a6823)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f258(v0: u64, v1: bool) -> R<u64> {
    let a6826 = {
        let a6824 = v0;
        let a6825 = 2u64;
        nat_mul(a6824, a6825)?
    };
    let a6827 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a6826, a6827)
}

pub fn f259(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a6828 = v0;
    let a6829 = 0u64;
    if nat_eq(a6828, a6829) {
        Ok(f270(1u64))
    } else {
        f288({
            let a6838 = {
                let a6831 = v1;
                let a6832 = match v2.clone() {
                    Adt3::C0(h6830, _, _, _, _, _, _, _) => {
                        h6830
                    }
                };
                nat_mul(a6831, a6832)?
            };
            let a6839 = {
                let a6835 = {
                    let a6833 = v0;
                    let a6834 = 1u64;
                    nat_sub(a6833, a6834)
                };
                let a6836 = 2u64;
                let a6837 = 0u64;
                nat_quot(a6835, a6836, a6837)
            };
            nat_add(a6838, a6839)?
        }, {
            let a6842 = {
                let a6840 = v0;
                let a6841 = 1u64;
                nat_sub(a6840, a6841)
            };
            let a6843 = 2u64;
            let a6844 = 0u64;
            nat_rem(a6842, a6843, a6844)
        }, v2, v3, 1u64)
    }
}

pub fn f260(v0: u64, v1: u64) -> R<u64> {
    let a6845 = v0;
    let a6846 = f102(v1)?;
    let a6847 = 0u64;
    Ok(nat_rem(a6845, a6846, a6847))
}

pub fn f261(v0: u64, v1: u64) -> R<u64> {
    let a6848 = 4u64;
    let a6849 = f289(v0, v1)?;
    nat_mul(a6848, a6849)
}

pub fn f262(v0: u64, v1: u64) -> R<u64> {
    let a6850 = 4u64;
    let a6851 = f290(v0, v1)?;
    nat_mul(a6850, a6851)
}

pub fn f263(v0: u64, v1: u64) -> R<bool> {
    let a6857 = {
        let a6854 = {
            let a6852 = f260(v0, v1)?;
            let a6853 = 2u64;
            nat_sub(a6852, a6853)
        };
        let a6855 = 2u64;
        let a6856 = 0u64;
        nat_rem(a6854, a6855, a6856)
    };
    let a6858 = 0u64;
    Ok(nat_eq(a6857, a6858))
}

pub fn f264(v0: u64) -> u64 {
    let m6859 = f226(v0);
    match m6859 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h6860, _) => {
                    h6860
                }
            }
        }
    }
}

pub fn f265(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m6861 = f226(v0);
    match m6861 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f234(v0) {
                let a6862 = v1;
                let a6863 = v7;
                nat_lt(a6862, a6863)
            } else {
                false
            } {
                f194(v8, v2)
            } else {
                false
            } {
                let a6864 = v3;
                let a6865 = f291(v4, v5, v6)?;
                nat_lt(a6864, a6865)
            } else {
                false
            } {
                let a6870 = {
                    let a6867 = v7;
                    let a6868 = match v13.clone() {
                        Adt19::C0(_, h6866, _) => {
                            h6866
                        }
                    };
                    let a6869 = 0u64;
                    nat_rem(a6867, a6868, a6869)
                };
                let a6871 = 0u64;
                nat_eq(a6870, a6871)
            } else {
                false
            } {
                Ok(Some({
                    let a6900 = {
                        let a6890 = {
                            let a6880 = {
                                let a6876 = {
                                    let a6873 = v1;
                                    let a6874 = match v13 {
                                        Adt19::C0(_, h6872, _) => {
                                            h6872
                                        }
                                    };
                                    let a6875 = 0u64;
                                    nat_quot(a6873, a6874, a6875)
                                };
                                let a6877 = v9;
                                nat_mul(a6876, a6877)?
                            };
                            let a6881 = {
                                let a6878 = v2;
                                let a6879 = v10;
                                nat_mul(a6878, a6879)?
                            };
                            nat_add(a6880, a6881)?
                        };
                        let a6891 = {
                            let a6888 = {
                                let a6885 = {
                                    let a6882 = v3;
                                    let a6883 = v4;
                                    let a6884 = 0u64;
                                    nat_quot(a6882, a6883, a6884)
                                };
                                let a6886 = v5;
                                let a6887 = 0u64;
                                nat_rem(a6885, a6886, a6887)
                            };
                            let a6889 = v11;
                            nat_mul(a6888, a6889)?
                        };
                        nat_add(a6890, a6891)?
                    };
                    let a6901 = {
                        let a6898 = {
                            let a6895 = {
                                let a6892 = v3;
                                let a6893 = v4;
                                let a6894 = 0u64;
                                nat_quot(a6892, a6893, a6894)
                            };
                            let a6896 = v5;
                            let a6897 = 0u64;
                            nat_quot(a6895, a6896, a6897)
                        };
                        let a6899 = v12;
                        nat_mul(a6898, a6899)?
                    };
                    nat_add(a6900, a6901)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f266(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f292(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f267(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f293(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f268(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a6902 = v0;
    let a6903 = 64u64;
    if nat_eq(a6902, a6903) {
        f294(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a6904 = v0;
        let a6905 = 65u64;
        if nat_eq(a6904, a6905) {
            f295(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f269(v0: u64) -> u64 {
    let a6906 = v0;
    let a6907 = 30u64;
    if nat_eq(a6906, a6907) {
        4u64
    } else {
        3u64
    }
}

pub fn f270(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f271(v0: u64) -> bool {
    let a6908 = 0u64;
    let a6909 = v0;
    if nat_lt(a6908, a6909) {
        let a6910 = v0;
        let a6911 = 2139095040u64;
        nat_le(a6910, a6911)
    } else {
        false
    }
}

pub fn f272(v0: u64) -> u64 {
    let a6912 = v0;
    let a6913 = 40u64;
    if nat_lt(a6912, a6913) {
        let a6914 = v0;
        let a6915 = 20u64;
        if nat_lt(a6914, a6915) {
            let a6916 = v0;
            let a6917 = 10u64;
            if nat_lt(a6916, a6917) {
                let a6918 = v0;
                let a6919 = 5u64;
                if nat_lt(a6918, a6919) {
                    let a6920 = v0;
                    let a6921 = 2u64;
                    if nat_lt(a6920, a6921) {
                        let a6922 = v0;
                        let a6923 = 1u64;
                        if nat_lt(a6922, a6923) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a6924 = v0;
                        let a6925 = 3u64;
                        if nat_lt(a6924, a6925) {
                            4u64
                        } else {
                            let a6926 = v0;
                            let a6927 = 4u64;
                            if nat_lt(a6926, a6927) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a6928 = v0;
                    let a6929 = 7u64;
                    if nat_lt(a6928, a6929) {
                        let a6930 = v0;
                        let a6931 = 6u64;
                        if nat_lt(a6930, a6931) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a6932 = v0;
                        let a6933 = 8u64;
                        if nat_lt(a6932, a6933) {
                            128u64
                        } else {
                            let a6934 = v0;
                            let a6935 = 9u64;
                            if nat_lt(a6934, a6935) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a6936 = v0;
                let a6937 = 15u64;
                if nat_lt(a6936, a6937) {
                    let a6938 = v0;
                    let a6939 = 12u64;
                    if nat_lt(a6938, a6939) {
                        let a6940 = v0;
                        let a6941 = 11u64;
                        if nat_lt(a6940, a6941) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a6942 = v0;
                        let a6943 = 13u64;
                        if nat_lt(a6942, a6943) {
                            4096u64
                        } else {
                            let a6944 = v0;
                            let a6945 = 14u64;
                            if nat_lt(a6944, a6945) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a6946 = v0;
                    let a6947 = 17u64;
                    if nat_lt(a6946, a6947) {
                        let a6948 = v0;
                        let a6949 = 16u64;
                        if nat_lt(a6948, a6949) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a6950 = v0;
                        let a6951 = 18u64;
                        if nat_lt(a6950, a6951) {
                            131072u64
                        } else {
                            let a6952 = v0;
                            let a6953 = 19u64;
                            if nat_lt(a6952, a6953) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a6954 = v0;
            let a6955 = 30u64;
            if nat_lt(a6954, a6955) {
                let a6956 = v0;
                let a6957 = 25u64;
                if nat_lt(a6956, a6957) {
                    let a6958 = v0;
                    let a6959 = 22u64;
                    if nat_lt(a6958, a6959) {
                        let a6960 = v0;
                        let a6961 = 21u64;
                        if nat_lt(a6960, a6961) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a6962 = v0;
                        let a6963 = 23u64;
                        if nat_lt(a6962, a6963) {
                            4194304u64
                        } else {
                            let a6964 = v0;
                            let a6965 = 24u64;
                            if nat_lt(a6964, a6965) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a6966 = v0;
                    let a6967 = 27u64;
                    if nat_lt(a6966, a6967) {
                        let a6968 = v0;
                        let a6969 = 26u64;
                        if nat_lt(a6968, a6969) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a6970 = v0;
                        let a6971 = 28u64;
                        if nat_lt(a6970, a6971) {
                            134217728u64
                        } else {
                            let a6972 = v0;
                            let a6973 = 29u64;
                            if nat_lt(a6972, a6973) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a6974 = v0;
                let a6975 = 35u64;
                if nat_lt(a6974, a6975) {
                    let a6976 = v0;
                    let a6977 = 32u64;
                    if nat_lt(a6976, a6977) {
                        let a6978 = v0;
                        let a6979 = 31u64;
                        if nat_lt(a6978, a6979) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a6980 = v0;
                        let a6981 = 33u64;
                        if nat_lt(a6980, a6981) {
                            4294967296u64
                        } else {
                            let a6982 = v0;
                            let a6983 = 34u64;
                            if nat_lt(a6982, a6983) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a6984 = v0;
                    let a6985 = 37u64;
                    if nat_lt(a6984, a6985) {
                        let a6986 = v0;
                        let a6987 = 36u64;
                        if nat_lt(a6986, a6987) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a6988 = v0;
                        let a6989 = 38u64;
                        if nat_lt(a6988, a6989) {
                            137438953472u64
                        } else {
                            let a6990 = v0;
                            let a6991 = 39u64;
                            if nat_lt(a6990, a6991) {
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

pub fn f273(v0: u64, v1: u64) -> bool {
    let a6992 = v0;
    let a6993 = 1u64;
    if nat_le(a6992, a6993) {
        true
    } else {
        let a6997 = {
            let a6994 = v1;
            let a6995 = 4u64;
            let a6996 = 0u64;
            nat_rem(a6994, a6995, a6996)
        };
        let a6998 = 0u64;
        nat_eq(a6997, a6998)
    }
}

pub fn f274(v0: u64, v1: u64) -> bool {
    let a6999 = v0;
    let a7000 = 1u64;
    if nat_le(a6999, a7000) {
        true
    } else {
        let a7001 = 4u64;
        let a7002 = v1;
        nat_le(a7001, a7002)
    }
}

pub fn f275(v0: u64, v1: u64) -> R<u64> {
    let a7003 = v0;
    let a7004 = 0u64;
    if nat_eq(a7003, a7004) {
        Ok(0u64)
    } else {
        let a7009 = 4u64;
        let a7010 = {
            let a7007 = {
                let a7005 = v0;
                let a7006 = 1u64;
                nat_sub(a7005, a7006)
            };
            let a7008 = v1;
            nat_mul(a7007, a7008)?
        };
        nat_add(a7009, a7010)
    }
}

pub fn f276(v0: bool, v1: u64) -> R<u64> {
    let a7015 = {
        let a7013 = {
            let a7011 = f296(v0, v1)?;
            let a7012 = f297(v0, v1)?;
            nat_add(a7011, a7012)?
        };
        let a7014 = f298(v0);
        nat_add(a7013, a7014)?
    };
    let a7016 = v1;
    nat_add(a7015, a7016)
}

pub fn f277(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f299(v0, v1)?;
    let v3: u64 = {
        let a7021 = {
            let a7019 = {
                let a7017 = v1;
                let a7018 = 1u64;
                nat_sub(a7017, a7018)
            };
            let a7020 = v2;
            nat_mul(a7019, a7020)?
        };
        let a7022 = v0;
        nat_sub(a7021, a7022)
    };
    let a7023 = v2;
    let a7024 = v3;
    if nat_lt(a7023, a7024) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f278(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7025 = v4;
    let a7026 = 0u64;
    if nat_eq(a7025, a7026) {
        Ok(0u64)
    } else {
        let a7033 = 2u64;
        let a7034 = {
            let a7031 = {
                let a7027 = v0;
                let a7028 = v1;
                nat_mul(a7027, a7028)?
            };
            let a7032 = {
                let a7029 = v2;
                let a7030 = v3;
                nat_mul(a7029, a7030)?
            };
            nat_mul(a7031, a7032)?
        };
        nat_mul(a7033, a7034)
    }
}

pub fn f279(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a7039 = v3;
    let a7040 = {
        let a7037 = {
            let a7035 = f300(v2, v4)?;
            let a7036 = f301(v0, v1, f299(v2, v4)?)?;
            nat_mul(a7035, a7036)?
        };
        let a7038 = f301(v0, v1, f302(v2, v4)?)?;
        nat_add(a7037, a7038)?
    };
    nat_mul(a7039, a7040)
}

pub fn f280(v0: u64) -> R<u64> {
    let a7043 = 13u64;
    let a7044 = {
        let a7041 = 9u64;
        let a7042 = v0;
        nat_mul(a7041, a7042)?
    };
    nat_add(a7043, a7044)
}

pub fn f281(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f303(v0, v1, v6, v7, v8) {
        let a7045 = v5;
        let a7046 = 26u64;
        nat_eq(a7045, a7046)
    } else {
        false
    } {
        let a7047 = v3;
        let a7048 = v7;
        nat_eq(a7047, a7048)
    } else {
        false
    } {
        let a7049 = v4;
        let a7050 = v8;
        nat_eq(a7049, a7050)
    } else {
        false
    } {
        let a7051 = v9;
        let a7052 = 1u64;
        nat_eq(a7051, a7052)
    } else {
        false
    } {
        let a7053 = v10;
        let a7054 = f304(v0);
        nat_eq(a7053, a7054)
    } else {
        false
    } {
        let a7055 = v11;
        let a7056 = v1;
        nat_eq(a7055, a7056)
    } else {
        false
    } {
        let a7057 = v12;
        let a7058 = v6;
        nat_eq(a7057, a7058)
    } else {
        false
    } {
        let a7059 = v13;
        let a7060 = v7;
        nat_eq(a7059, a7060)
    } else {
        false
    } {
        let a7061 = v14;
        let a7062 = v8;
        nat_eq(a7061, a7062)
    } else {
        false
    }
}

pub fn f282(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a7063 = v0;
    let a7064 = 0u64;
    if if if nat_eq(a7063, a7064) {
        true
    } else {
        let a7065 = v1;
        let a7066 = 0u64;
        nat_eq(a7065, a7066)
    } {
        true
    } else {
        let a7067 = v2;
        let a7068 = 0u64;
        nat_eq(a7067, a7068)
    } {
        true
    } else {
        let a7069 = v3;
        let a7070 = 0u64;
        nat_eq(a7069, a7070)
    }
}

pub fn f283(v0: Adt3, v1: bool) -> R<u64> {
    f287(v0.clone(), {
        let a7072 = match v0.clone() {
            Adt3::C0(h7071, _, _, _, _, _, _, _) => {
                h7071
            }
        };
        let a7073 = 1u64;
        nat_sub(a7072, a7073)
    }, {
        let a7075 = match v0.clone() {
            Adt3::C0(_, h7074, _, _, _, _, _, _) => {
                h7074
            }
        };
        let a7076 = 1u64;
        nat_sub(a7075, a7076)
    }, {
        let a7078 = match v0.clone() {
            Adt3::C0(_, _, h7077, _, _, _, _, _) => {
                h7077
            }
        };
        let a7079 = 1u64;
        nat_sub(a7078, a7079)
    }, {
        let a7081 = match v0 {
            Adt3::C0(_, _, _, h7080, _, _, _, _) => {
                h7080
            }
        };
        let a7082 = 1u64;
        nat_sub(a7081, a7082)
    }, v1)
}

pub fn f284(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a7083 = v0;
    let a7084 = 0u64;
    if nat_eq(a7083, a7084) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a7085 = v0;
            let a7086 = 1u64;
            nat_sub(a7085, a7086)
        };
        let v5: u64 = {
            let a7087 = v4;
            let a7088 = v1;
            let a7089 = 0u64;
            nat_quot(a7087, a7088, a7089)
        };
        let v6: u64 = {
            let a7097 = {
                let a7090 = v5;
                let a7091 = v2;
                nat_mul(a7090, a7091)?
            };
            let a7098 = {
                let a7095 = {
                    let a7092 = v4;
                    let a7093 = v1;
                    let a7094 = 0u64;
                    nat_rem(a7092, a7093, a7094)
                };
                let a7096 = v3;
                nat_mul(a7095, a7096)?
            };
            nat_add(a7097, a7098)?
        };
        let a7099 = v5;
        let a7100 = 0u64;
        if nat_eq(a7099, a7100) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a7109 = {
                    let a7103 = {
                        let a7101 = v5;
                        let a7102 = 1u64;
                        nat_sub(a7101, a7102)
                    };
                    let a7104 = v2;
                    nat_mul(a7103, a7104)?
                };
                let a7110 = {
                    let a7107 = {
                        let a7105 = v1;
                        let a7106 = 1u64;
                        nat_sub(a7105, a7106)
                    };
                    let a7108 = v3;
                    nat_mul(a7107, a7108)?
                };
                nat_add(a7109, a7110)?
            };
            let a7111 = v6;
            let a7112 = v7;
            if nat_lt(a7111, a7112) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f285(v0: Adt3, v1: u64) -> bool {
    let a7113 = v1;
    let a7114 = 0u64;
    if !nat_eq(a7113, a7114) {
        true
    } else {
        let a7116 = match v0 {
            Adt3::C0(_, _, _, _, h7115, _, _, _) => {
                h7115
            }
        };
        let a7117 = 4u64;
        nat_eq(a7116, a7117)
    }
}

pub fn f286(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m7118 = f305(v0, v1);
    match m7118 {
        None => {
            if v5 {
                let m7119 = f306(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m7119 {
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
                let a7120 = 4u64;
                let a7121 = v28;
                nat_le(a7120, a7121)
            };
            let a7122 = v2;
            let a7123 = 0u64;
            if if if if if nat_eq(a7122, a7123) {
                let a7124 = v4;
                let a7125 = 0u64;
                nat_eq(a7124, a7125)
            } else {
                false
            } {
                let a7126 = v6;
                let a7127 = v14;
                if nat_eq(a7126, a7127) {
                    let a7128 = v7;
                    let a7129 = v15;
                    nat_eq(a7128, a7129)
                } else {
                    false
                }
            } else {
                false
            } {
                let a7130 = v8;
                let a7131 = v16;
                if nat_eq(a7130, a7131) {
                    let a7132 = v9;
                    let a7133 = v17;
                    nat_eq(a7132, a7133)
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
                        let a7134 = v3;
                        let a7135 = 0u64;
                        nat_eq(a7134, a7135)
                    } else {
                        false
                    } {
                        f237(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a7140 = {
                        let a7136 = v6;
                        let a7137 = v7;
                        nat_mul(a7136, a7137)?
                    };
                    let a7141 = {
                        let a7138 = v8;
                        let a7139 = v9;
                        nat_mul(a7138, a7139)?
                    };
                    nat_mul(a7140, a7141)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f287(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a7162 = {
        let a7156 = {
            let a7150 = {
                let a7144 = v1;
                let a7145 = if v5 {
                    f151(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7142, _, _, _) => {
                            h7142
                        }
                    })
                } else {
                    f152(match v0.clone() {
                        Adt3::C0(_, _, _, _, h7143, _, _, _) => {
                            h7143
                        }
                    })
                };
                nat_mul(a7144, a7145)?
            };
            let a7151 = {
                let a7148 = v2;
                let a7149 = if v5 {
                    f151(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7146, _, _) => {
                            h7146
                        }
                    })
                } else {
                    f152(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h7147, _, _) => {
                            h7147
                        }
                    })
                };
                nat_mul(a7148, a7149)?
            };
            nat_add(a7150, a7151)?
        };
        let a7157 = {
            let a7154 = v3;
            let a7155 = if v5 {
                f151(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7152, _) => {
                        h7152
                    }
                })
            } else {
                f152(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h7153, _) => {
                        h7153
                    }
                })
            };
            nat_mul(a7154, a7155)?
        };
        nat_add(a7156, a7157)?
    };
    let a7163 = {
        let a7160 = v4;
        let a7161 = if v5 {
            f151(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7158) => {
                    h7158
                }
            })
        } else {
            f152(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h7159) => {
                    h7159
                }
            })
        };
        nat_mul(a7160, a7161)?
    };
    nat_add(a7162, a7163)
}

pub fn f288(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a7164 = v1;
    let a7165 = 0u64;
    if nat_eq(a7164, a7165) {
        Ok(Adt9::C4({
            let a7166 = v3;
            let a7167 = 0u64;
            if nat_eq(a7166, a7167) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f99(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a7168 = v3;
            let a7169 = 0u64;
            if nat_eq(a7168, a7169) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f289(v0: u64, v1: u64) -> R<u64> {
    let a7170 = v0;
    let a7171 = f102(v1)?;
    let a7172 = 0u64;
    Ok(nat_quot(a7170, a7171, a7172))
}

pub fn f290(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f289(v0, v1)?;
    let v3: u64 = {
        let a7175 = {
            let a7173 = f260(v0, v1)?;
            let a7174 = 2u64;
            nat_sub(a7173, a7174)
        };
        let a7176 = 2u64;
        let a7177 = 0u64;
        nat_quot(a7175, a7176, a7177)
    };
    let v4: u64 = {
        let a7178 = v2;
        let a7179 = v1;
        nat_mul(a7178, a7179)?
    };
    if f263(v0, v1)? {
        let a7180 = v4;
        let a7181 = v3;
        nat_add(a7180, a7181)
    } else {
        let a7193 = {
            let a7189 = v4;
            let a7190 = {
                let a7187 = {
                    let a7182 = v1;
                    let a7183 = 1u64;
                    nat_sub(a7182, a7183)
                };
                let a7188 = {
                    let a7184 = v2;
                    let a7185 = v1;
                    let a7186 = 0u64;
                    nat_rem(a7184, a7185, a7186)
                };
                nat_mul(a7187, a7188)?
            };
            nat_sub(a7189, a7190)
        };
        let a7194 = {
            let a7191 = v3;
            let a7192 = v1;
            nat_mul(a7191, a7192)?
        };
        nat_add(a7193, a7194)
    }
}

pub fn f291(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7195 = v0;
    let a7196 = 0u64;
    if if if nat_eq(a7195, a7196) {
        true
    } else {
        let a7197 = v1;
        let a7198 = 0u64;
        nat_eq(a7197, a7198)
    } {
        true
    } else {
        let a7199 = v2;
        let a7200 = 0u64;
        nat_eq(a7199, a7200)
    } {
        Ok(0u64)
    } else {
        let a7203 = {
            let a7201 = v0;
            let a7202 = v1;
            nat_mul(a7201, a7202)?
        };
        let a7204 = v2;
        nat_mul(a7203, a7204)
    }
}

pub fn f292(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f111(match v1.clone() {
        Adt20::C0(h7205, _, _, _, _, _, _, _) => {
            h7205
        }
    })?;
    let v6: u64 = {
        let a7211 = match v1.clone() {
            Adt20::C0(_, h7206, _, _, _, _, _, _) => {
                h7206
            }
        };
        let a7212 = {
            let a7209 = match v1.clone() {
                Adt20::C0(_, _, h7207, _, _, _, _, _) => {
                    h7207
                }
            };
            let a7210 = match v1.clone() {
                Adt20::C0(_, _, _, h7208, _, _, _, _) => {
                    h7208
                }
            };
            nat_mul(a7209, a7210)?
        };
        nat_mul(a7211, a7212)?
    };
    let a7214 = v0;
    let a7215 = f57(match v1.clone() {
        Adt20::C0(h7213, _, _, _, _, _, _, _) => {
            h7213
        }
    }, v6)?;
    if nat_lt(a7214, a7215) {
        f307({
            let a7216 = v0;
            let a7217 = v5;
            let a7218 = 0u64;
            nat_rem(a7216, a7217, a7218)
        }, {
            let a7219 = v0;
            let a7220 = v5;
            let a7221 = 0u64;
            nat_quot(a7219, a7220, a7221)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f293(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a7222 = v6;
        let a7223 = 6u64;
        nat_sub(a7222, a7223)
    };
    let v9: Adt20 = f308(v3);
    let a7225 = v0;
    let a7226 = f223(match v1.clone() {
        Adt20::C0(h7224, _, _, _, _, _, _, _) => {
            h7224
        }
    }, v5, v8)?;
    if nat_lt(a7225, a7226) {
        let v10: u64 = f108(match v1.clone() {
            Adt20::C0(h7227, _, _, _, _, _, _, _) => {
                h7227
            }
        }, v5, v8)?;
        let a7228 = v0;
        let a7229 = v10;
        if nat_lt(a7228, a7229) {
            f309(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a7230 = v0;
                let a7231 = v10;
                nat_sub(a7230, a7231)
            };
            let v12: u64 = f280(match v1.clone() {
                Adt20::C0(h7232, _, _, _, _, _, _, _) => {
                    h7232
                }
            })?;
            f310({
                let a7233 = v11;
                let a7234 = v12;
                let a7235 = 0u64;
                nat_rem(a7233, a7234, a7235)
            }, {
                let a7236 = v11;
                let a7237 = v12;
                let a7238 = 0u64;
                nat_quot(a7236, a7237, a7238)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f294(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7239 = v5;
        let a7240 = 6u64;
        nat_sub(a7239, a7240)
    };
    let v8: bool = f217(match v1.clone() {
        Adt20::C0(_, _, _, _, h7241, _, _, _) => {
            h7241
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h7242, _, _, _) => {
            h7242
        }
    });
    let a7244 = v0;
    let a7245 = f216(v8, match v1.clone() {
        Adt20::C0(h7243, _, _, _, _, _, _, _) => {
            h7243
        }
    }, v4, v7)?;
    if nat_lt(a7244, a7245) {
        let v9: u64 = f108(match v1.clone() {
            Adt20::C0(h7246, _, _, _, _, _, _, _) => {
                h7246
            }
        }, v4, v7)?;
        let a7247 = v0;
        let a7248 = v9;
        if nat_lt(a7247, a7248) {
            f309(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a7249 = v0;
                let a7250 = v9;
                nat_sub(a7249, a7250)
            };
            let v11: u64 = f276(v8, match v1.clone() {
                Adt20::C0(h7251, _, _, _, _, _, _, _) => {
                    h7251
                }
            })?;
            f311({
                let a7252 = v10;
                let a7253 = v11;
                let a7254 = 0u64;
                nat_rem(a7252, a7253, a7254)
            }, {
                let a7255 = v10;
                let a7256 = v11;
                let a7257 = 0u64;
                nat_quot(a7255, a7256, a7257)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f295(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a7258 = v5;
        let a7259 = 6u64;
        nat_sub(a7258, a7259)
    };
    let v8: Adt20 = f308(v2);
    let v9: u64 = f278(match v1.clone() {
        Adt20::C0(h7260, _, _, _, _, _, _, _) => {
            h7260
        }
    }, match v1.clone() {
        Adt20::C0(_, h7261, _, _, _, _, _, _) => {
            h7261
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7262, _, _, _, _, _) => {
            h7262
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7263, _, _, _, _) => {
            h7263
        }
    }, v7)?;
    let a7268 = v0;
    let a7269 = f222(match v1.clone() {
        Adt20::C0(h7264, _, _, _, _, _, _, _) => {
            h7264
        }
    }, match v1.clone() {
        Adt20::C0(_, h7265, _, _, _, _, _, _) => {
            h7265
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h7266, _, _, _, _, _) => {
            h7266
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h7267, _, _, _, _) => {
            h7267
        }
    }, v4, v7)?;
    if nat_lt(a7268, a7269) {
        let a7270 = v0;
        let a7271 = v9;
        if nat_lt(a7270, a7271) {
            f309(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f312({
                let a7272 = v0;
                let a7273 = v9;
                nat_sub(a7272, a7273)
            }, match v1.clone() {
                Adt20::C0(h7274, _, _, _, _, _, _, _) => {
                    h7274
                }
            }, match v1.clone() {
                Adt20::C0(_, h7275, _, _, _, _, _, _) => {
                    h7275
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h7276, _, _, _, _, _) => {
                    h7276
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h7277, _, _, _, _) => {
                    h7277
                }
            }, v4)?;
            f313(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f296(v0: bool, v1: u64) -> R<u64> {
    let a7280 = if v0 {
        5u64
    } else {
        4u64
    };
    let a7281 = {
        let a7278 = f314(v0);
        let a7279 = v1;
        nat_mul(a7278, a7279)?
    };
    nat_add(a7280, a7281)
}

pub fn f297(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a7289 = {
            let a7282 = 13u64;
            let a7283 = f315(v1);
            nat_mul(a7282, a7283)?
        };
        let a7290 = {
            let a7287 = 4u64;
            let a7288 = {
                let a7284 = v1;
                let a7285 = 4u64;
                let a7286 = 0u64;
                nat_rem(a7284, a7285, a7286)
            };
            nat_mul(a7287, a7288)?
        };
        nat_add(a7289, a7290)
    } else {
        let a7291 = 3u64;
        let a7292 = v1;
        nat_mul(a7291, a7292)
    }
}

pub fn f298(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f299(v0: u64, v1: u64) -> R<u64> {
    let a7297 = {
        let a7295 = {
            let a7293 = v0;
            let a7294 = v1;
            nat_add(a7293, a7294)?
        };
        let a7296 = 1u64;
        nat_sub(a7295, a7296)
    };
    let a7298 = v1;
    let a7299 = 0u64;
    Ok(nat_quot(a7297, a7298, a7299))
}

pub fn f300(v0: u64, v1: u64) -> R<u64> {
    let a7300 = v0;
    let a7301 = f299(v0, v1)?;
    let a7302 = 0u64;
    Ok(nat_quot(a7300, a7301, a7302))
}

pub fn f301(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a7303 = v2;
    let a7304 = 0u64;
    if nat_eq(a7303, a7304) {
        Ok(0u64)
    } else {
        let a7309 = 11u64;
        let a7310 = {
            let a7307 = {
                let a7305 = v1;
                let a7306 = v2;
                nat_mul(a7305, a7306)?
            };
            let a7308 = f316(v0)?;
            nat_mul(a7307, a7308)?
        };
        nat_add(a7309, a7310)
    }
}

pub fn f302(v0: u64, v1: u64) -> R<u64> {
    let a7311 = v0;
    let a7312 = f299(v0, v1)?;
    let a7313 = 0u64;
    Ok(nat_rem(a7311, a7312, a7313))
}

pub fn f303(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f282(v1, v2, v3, v4) {
        f317(v0)
    } else {
        f234(v0)
    }
}

pub fn f304(v0: u64) -> u64 {
    let a7314 = v0;
    let a7315 = 26u64;
    if nat_eq(a7314, a7315) {
        26u64
    } else {
        0u64
    }
}

pub fn f305(v0: u64, v1: u64) -> Option<u64> {
    let a7316 = v0;
    let a7317 = 2u64;
    if nat_eq(a7316, a7317) {
        Some(0u64)
    } else {
        let a7318 = v0;
        let a7319 = 5u64;
        if nat_eq(a7318, a7319) {
            Some(1u64)
        } else {
            let a7320 = v0;
            let a7321 = 6u64;
            if nat_eq(a7320, a7321) {
                Some(2u64)
            } else {
                let a7322 = v0;
                let a7323 = 7u64;
                if nat_eq(a7322, a7323) {
                    Some(3u64)
                } else {
                    let a7324 = v0;
                    let a7325 = 9u64;
                    if nat_eq(a7324, a7325) {
                        Some(4u64)
                    } else {
                        let a7326 = v0;
                        let a7327 = 8u64;
                        if nat_eq(a7326, a7327) {
                            Some(5u64)
                        } else {
                            let a7328 = v0;
                            let a7329 = 89u64;
                            if nat_eq(a7328, a7329) {
                                let a7330 = v1;
                                let a7331 = 2u64;
                                if nat_eq(a7330, a7331) {
                                    Some(6u64)
                                } else {
                                    let a7332 = v1;
                                    let a7333 = 0u64;
                                    if nat_eq(a7332, a7333) {
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

pub fn f306(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7334 = v0;
    let a7335 = 30u64;
    if nat_eq(a7334, a7335) {
        let a7336 = v2;
        let a7337 = 0u64;
        if if if nat_eq(a7336, a7337) {
            true
        } else {
            let a7338 = v2;
            let a7339 = 1u64;
            nat_eq(a7338, a7339)
        } {
            let a7340 = v1;
            let a7341 = 0u64;
            if nat_eq(a7340, a7341) {
                true
            } else {
                f318(v1, v4)
            }
        } else {
            false
        } {
            f319(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f320(v1) {
            f320(v2)
        } else {
            false
        } {
            let a7342 = v1;
            let a7343 = 0u64;
            if if nat_eq(a7342, a7343) {
                true
            } else {
                let a7344 = v2;
                let a7345 = 0u64;
                nat_eq(a7344, a7345)
            } {
                true
            } else {
                let a7346 = v1;
                let a7347 = v2;
                nat_eq(a7346, a7347)
            }
        } else {
            false
        } {
            true
        } else {
            let a7348 = v1;
            let a7349 = 43u64;
            if nat_eq(a7348, a7349) {
                f320(v2)
            } else {
                false
            }
        } {
            f319(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f307(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h7350, _, _, _, _, _, _, _) => {
            h7350
        }
    };
    let v7: u64 = {
        let a7353 = 1u64;
        let a7354 = {
            let a7351 = 3u64;
            let a7352 = v6;
            nat_mul(a7351, a7352)?
        };
        nat_add(a7353, a7354)?
    };
    let a7355 = v0;
    let a7356 = 0u64;
    if nat_eq(a7355, a7356) {
        Ok(f321(0u64))
    } else {
        let a7357 = v0;
        let a7358 = v7;
        if nat_lt(a7357, a7358) {
            let v8: u64 = {
                let a7359 = v0;
                let a7360 = 1u64;
                nat_sub(a7359, a7360)
            };
            let v9: u64 = {
                let a7366 = {
                    let a7361 = v1;
                    let a7362 = v6;
                    nat_mul(a7361, a7362)?
                };
                let a7367 = {
                    let a7363 = v8;
                    let a7364 = 3u64;
                    let a7365 = 0u64;
                    nat_quot(a7363, a7364, a7365)
                };
                nat_add(a7366, a7367)?
            };
            let v10: u64 = f322(v9, v2)?;
            let v11: u64 = {
                let a7368 = v8;
                let a7369 = 3u64;
                let a7370 = 0u64;
                nat_rem(a7368, a7369, a7370)
            };
            let a7371 = v11;
            let a7372 = 0u64;
            if nat_eq(a7371, a7372) {
                Ok(f323(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a7373 = v11;
                let a7374 = 1u64;
                if nat_eq(a7373, a7374) {
                    Ok(f323(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f323(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a7375 = v0;
            let a7376 = v7;
            if nat_eq(a7375, a7376) {
                Ok(f323(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a7379 = v0;
                let a7380 = {
                    let a7377 = v7;
                    let a7378 = 1u64;
                    nat_add(a7377, a7378)?
                };
                if nat_eq(a7379, a7380) {
                    Ok(f323(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7383 = v0;
                    let a7384 = {
                        let a7381 = v7;
                        let a7382 = 2u64;
                        nat_add(a7381, a7382)?
                    };
                    if nat_eq(a7383, a7384) {
                        Ok(f323(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a7387 = v0;
                        let a7388 = {
                            let a7385 = v7;
                            let a7386 = 3u64;
                            nat_add(a7385, a7386)?
                        };
                        if nat_eq(a7387, a7388) {
                            let a7389 = v5;
                            let a7390 = 1u64;
                            if if nat_eq(a7389, a7390) {
                                true
                            } else {
                                let a7391 = v5;
                                let a7392 = 2u64;
                                nat_eq(a7391, a7392)
                            } {
                                Ok(f323(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f323(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a7399 = {
                                    let a7393 = v1;
                                    let a7394 = v6;
                                    nat_mul(a7393, a7394)?
                                };
                                let a7400 = {
                                    let a7397 = v0;
                                    let a7398 = {
                                        let a7395 = v7;
                                        let a7396 = 4u64;
                                        nat_add(a7395, a7396)?
                                    };
                                    nat_sub(a7397, a7398)
                                };
                                nat_add(a7399, a7400)?
                            };
                            Ok(f323(10u64, 0u64, f322(v12, v2)?, 1u64, 2u64, 3u64, f322(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f308(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h7401, _, _, _, _, _, _, _) => {
            h7401
        }
    }, match v0.clone() {
        Adt20::C0(_, h7402, _, _, _, _, _, _) => {
            h7402
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h7403, _, _, _, _, _) => {
            h7403
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h7404, _, _, _, _) => {
            h7404
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h7405, _, _) => {
            h7405
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h7406, _) => {
            h7406
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h7407) => {
            h7407
        }
    })
}

pub fn f309(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a7414 = v3;
        let a7415 = {
            let a7411 = f322({
                let a7408 = v0;
                let a7409 = 2u64;
                let a7410 = 0u64;
                nat_quot(a7408, a7409, a7410)
            }, v1)?;
            let a7412 = v2;
            let a7413 = 0u64;
            nat_quot(a7411, a7412, a7413)
        };
        nat_add(a7414, a7415)?
    };
    let a7419 = {
        let a7416 = v0;
        let a7417 = 2u64;
        let a7418 = 0u64;
        nat_rem(a7416, a7417, a7418)
    };
    let a7420 = 0u64;
    if nat_eq(a7419, a7420) {
        Ok(f323(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a7421 = v4;
        let a7422 = 0u64;
        if nat_eq(a7421, a7422) {
            Ok(f323(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f310(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f324(match v2.clone() {
        Adt20::C0(h7423, _, _, _, _, _, _, _) => {
            h7423
        }
    })?;
    let v7: u64 = f325(match v2.clone() {
        Adt20::C0(h7424, _, _, _, _, _, _, _) => {
            h7424
        }
    })?;
    let a7425 = v0;
    let a7426 = 2u64;
    if nat_lt(a7425, a7426) {
        Ok(f321(v0))
    } else {
        let a7427 = v0;
        let a7428 = v6;
        if nat_lt(a7427, a7428) {
            f326({
                let a7429 = v0;
                let a7430 = 2u64;
                nat_sub(a7429, a7430)
            }, v1, v2, v3)
        } else {
            let a7431 = v0;
            let a7432 = v7;
            if nat_lt(a7431, a7432) {
                Ok(f327({
                    let a7433 = v0;
                    let a7434 = v6;
                    nat_sub(a7433, a7434)
                }, match v2 {
                    Adt20::C0(h7435, _, _, _, _, _, _, _) => {
                        h7435
                    }
                }, v5))
            } else {
                let a7437 = v0;
                let a7438 = f280(match v2.clone() {
                    Adt20::C0(h7436, _, _, _, _, _, _, _) => {
                        h7436
                    }
                })?;
                if nat_lt(a7437, a7438) {
                    f328({
                        let a7439 = v0;
                        let a7440 = v7;
                        nat_sub(a7439, a7440)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f311(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f296(v5, match v2.clone() {
        Adt20::C0(h7441, _, _, _, _, _, _, _) => {
            h7441
        }
    })?;
    let v7: u64 = {
        let a7443 = v6;
        let a7444 = f297(v5, match v2.clone() {
            Adt20::C0(h7442, _, _, _, _, _, _, _) => {
                h7442
            }
        })?;
        nat_add(a7443, a7444)?
    };
    let v8: u64 = {
        let a7445 = v7;
        let a7446 = f298(v5);
        nat_add(a7445, a7446)?
    };
    let a7447 = v0;
    let a7448 = v6;
    if nat_lt(a7447, a7448) {
        f329(v0, v1, v2, v5)
    } else {
        let a7449 = v0;
        let a7450 = v7;
        if nat_lt(a7449, a7450) {
            f330({
                let a7451 = v0;
                let a7452 = v6;
                nat_sub(a7451, a7452)
            }, v1, v2, v3, v5)
        } else {
            let a7453 = v0;
            let a7454 = v8;
            if nat_lt(a7453, a7454) {
                Ok(f331({
                    let a7455 = v0;
                    let a7456 = v7;
                    nat_sub(a7455, a7456)
                }, match v2 {
                    Adt20::C0(h7457, _, _, _, _, _, _, _) => {
                        h7457
                    }
                }, v4, v5))
            } else {
                let a7461 = v0;
                let a7462 = {
                    let a7459 = v8;
                    let a7460 = match v2.clone() {
                        Adt20::C0(h7458, _, _, _, _, _, _, _) => {
                            h7458
                        }
                    };
                    nat_add(a7459, a7460)?
                };
                if nat_lt(a7461, a7462) {
                    let v9: u64 = f322({
                        let a7468 = {
                            let a7464 = v1;
                            let a7465 = match v2 {
                                Adt20::C0(h7463, _, _, _, _, _, _, _) => {
                                    h7463
                                }
                            };
                            nat_mul(a7464, a7465)?
                        };
                        let a7469 = {
                            let a7466 = v0;
                            let a7467 = v8;
                            nat_sub(a7466, a7467)
                        };
                        nat_add(a7468, a7469)?
                    }, v3)?;
                    Ok(f323(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f312(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f299(v3, v5)?;
    let v7: u64 = f301(v1, v2, v6)?;
    let v8: u64 = {
        let a7472 = {
            let a7470 = f300(v3, v5)?;
            let a7471 = v4;
            nat_mul(a7470, a7471)?
        };
        let a7473 = v7;
        nat_mul(a7472, a7473)?
    };
    let a7474 = v0;
    let a7475 = v8;
    if nat_lt(a7474, a7475) {
        let v9: u64 = {
            let a7476 = v0;
            let a7477 = v7;
            let a7478 = 0u64;
            nat_quot(a7476, a7477, a7478)
        };
        Ok(Adt21::C0({
            let a7482 = {
                let a7479 = v9;
                let a7480 = v4;
                let a7481 = 0u64;
                nat_quot(a7479, a7480, a7481)
            };
            let a7483 = v6;
            nat_mul(a7482, a7483)?
        }, v6, {
            let a7484 = v9;
            let a7485 = v4;
            let a7486 = 0u64;
            nat_rem(a7484, a7485, a7486)
        }, {
            let a7487 = v0;
            let a7488 = v7;
            let a7489 = 0u64;
            nat_rem(a7487, a7488, a7489)
        }))
    } else {
        let v10: u64 = {
            let a7490 = v0;
            let a7491 = v8;
            nat_sub(a7490, a7491)
        };
        let v11: u64 = f301(v1, v2, f302(v3, v5)?)?;
        Ok(Adt21::C0({
            let a7492 = f300(v3, v5)?;
            let a7493 = v6;
            nat_mul(a7492, a7493)?
        }, f302(v3, v5)?, {
            let a7494 = v10;
            let a7495 = v11;
            let a7496 = 0u64;
            nat_quot(a7494, a7495, a7496)
        }, {
            let a7497 = v10;
            let a7498 = v11;
            let a7499 = 0u64;
            nat_rem(a7497, a7498, a7499)
        }))
    }
}

pub fn f313(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h7500) => {
            h7500
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h7501, _, _, _, _, _, _, _) => {
            h7501
        }
    };
    let v6: u64 = {
        let a7504 = match v0.clone() {
            Adt21::C0(_, h7502, _, _) => {
                h7502
            }
        };
        let a7505 = match v1.clone() {
            Adt20::C0(_, h7503, _, _, _, _, _, _) => {
                h7503
            }
        };
        nat_mul(a7504, a7505)?
    };
    let v7: u64 = {
        let a7506 = v6;
        let a7507 = v5;
        nat_mul(a7506, a7507)?
    };
    let v8: u64 = {
        let a7510 = {
            let a7508 = 2u64;
            let a7509 = v5;
            nat_mul(a7508, a7509)?
        };
        let a7511 = 2u64;
        nat_add(a7510, a7511)?
    };
    let v9: u64 = {
        let a7514 = {
            let a7512 = 4u64;
            let a7513 = v5;
            nat_mul(a7512, a7513)?
        };
        let a7515 = 2u64;
        nat_add(a7514, a7515)?
    };
    let v10: u64 = {
        let a7518 = 1u64;
        let a7519 = {
            let a7516 = v6;
            let a7517 = v8;
            nat_mul(a7516, a7517)?
        };
        nat_add(a7518, a7519)?
    };
    let v11: u64 = {
        let a7520 = v10;
        let a7521 = 4u64;
        nat_add(a7520, a7521)?
    };
    let v12: u64 = {
        let a7524 = v11;
        let a7525 = {
            let a7522 = v6;
            let a7523 = v9;
            nat_mul(a7522, a7523)?
        };
        nat_add(a7524, a7525)?
    };
    let v13: u64 = {
        let a7526 = v12;
        let a7527 = 6u64;
        nat_add(a7526, a7527)?
    };
    let a7528 = v4;
    let a7529 = 0u64;
    if nat_eq(a7528, a7529) {
        Ok(f321(0u64))
    } else {
        let a7530 = v4;
        let a7531 = v10;
        if nat_lt(a7530, a7531) {
            let v14: u64 = {
                let a7532 = v4;
                let a7533 = 1u64;
                nat_sub(a7532, a7533)
            };
            f332(false, {
                let a7534 = v14;
                let a7535 = v8;
                let a7536 = 0u64;
                nat_rem(a7534, a7535, a7536)
            }, {
                let a7537 = v14;
                let a7538 = v8;
                let a7539 = 0u64;
                nat_quot(a7537, a7538, a7539)
            }, v0, v1, v2)
        } else {
            let a7540 = v4;
            let a7541 = v11;
            if nat_lt(a7540, a7541) {
                Ok(f333({
                    let a7542 = v4;
                    let a7543 = v10;
                    nat_sub(a7542, a7543)
                }, v7))
            } else {
                let a7544 = v4;
                let a7545 = v12;
                if nat_lt(a7544, a7545) {
                    let v15: u64 = {
                        let a7546 = v4;
                        let a7547 = v11;
                        nat_sub(a7546, a7547)
                    };
                    f332(true, {
                        let a7548 = v15;
                        let a7549 = v9;
                        let a7550 = 0u64;
                        nat_rem(a7548, a7549, a7550)
                    }, {
                        let a7551 = v15;
                        let a7552 = v9;
                        let a7553 = 0u64;
                        nat_quot(a7551, a7552, a7553)
                    }, v0, v1, v2)
                } else {
                    let a7554 = v4;
                    let a7555 = v13;
                    if nat_lt(a7554, a7555) {
                        Ok(f334({
                            let a7556 = v4;
                            let a7557 = v12;
                            nat_sub(a7556, a7557)
                        }, v7, v3))
                    } else {
                        let a7560 = v4;
                        let a7561 = {
                            let a7558 = v13;
                            let a7559 = v7;
                            nat_add(a7558, a7559)?
                        };
                        if nat_lt(a7560, a7561) {
                            let v16: u64 = {
                                let a7562 = v4;
                                let a7563 = v13;
                                nat_sub(a7562, a7563)
                            };
                            let v17: u64 = {
                                let a7564 = v16;
                                let a7565 = v5;
                                let a7566 = 0u64;
                                nat_quot(a7564, a7565, a7566)
                            };
                            let v18: u64 = {
                                let a7572 = {
                                    let a7567 = f335(v17, v0, v1)?;
                                    let a7568 = v5;
                                    nat_mul(a7567, a7568)?
                                };
                                let a7573 = {
                                    let a7569 = v16;
                                    let a7570 = v5;
                                    let a7571 = 0u64;
                                    nat_rem(a7569, a7570, a7571)
                                };
                                nat_add(a7572, a7573)?
                            };
                            let v19: u64 = f322(v18, v2)?;
                            Ok(f323(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f314(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f315(v0: u64) -> u64 {
    let a7574 = v0;
    let a7575 = 4u64;
    let a7576 = 0u64;
    nat_quot(a7574, a7575, a7576)
}

pub fn f316(v0: u64) -> R<u64> {
    let a7579 = {
        let a7577 = 7u64;
        let a7578 = v0;
        nat_mul(a7577, a7578)?
    };
    let a7580 = 4u64;
    nat_add(a7579, a7580)
}

pub fn f317(v0: u64) -> bool {
    let m7581 = f226(v0);
    match m7581 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f318(v0: u64, v1: u64) -> bool {
    let a7582 = v0;
    let a7583 = 1u64;
    if nat_eq(a7582, a7583) {
        true
    } else {
        let v2: u64 = f336(v0);
        let a7584 = 0u64;
        let a7585 = v2;
        if nat_lt(a7584, a7585) {
            let a7589 = {
                let a7586 = v1;
                let a7587 = v2;
                let a7588 = 1u64;
                nat_rem(a7586, a7587, a7588)
            };
            let a7590 = 0u64;
            nat_eq(a7589, a7590)
        } else {
            false
        }
    }
}

pub fn f319(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a7591 = v0;
    let a7592 = 28u64;
    if if if if nat_eq(a7591, a7592) {
        let a7593 = v1;
        let a7594 = 0u64;
        nat_eq(a7593, a7594)
    } else {
        false
    } {
        let a7595 = v2;
        let a7596 = 0u64;
        nat_eq(a7595, a7596)
    } else {
        false
    } {
        let a7597 = v3;
        let a7598 = 0u64;
        nat_eq(a7597, a7598)
    } else {
        false
    } {
        let a7599 = v4;
        let a7600 = v8;
        if if if if if if if if nat_eq(a7599, a7600) {
            let a7601 = v5;
            let a7602 = v12;
            nat_eq(a7601, a7602)
        } else {
            false
        } {
            let a7603 = v9;
            let a7604 = v13;
            nat_eq(a7603, a7604)
        } else {
            false
        } {
            let a7605 = v10;
            let a7606 = v14;
            nat_eq(a7605, a7606)
        } else {
            false
        } {
            let a7607 = v11;
            let a7608 = v15;
            nat_eq(a7607, a7608)
        } else {
            false
        } {
            let a7609 = 0u64;
            let a7610 = v6;
            nat_lt(a7609, a7610)
        } else {
            false
        } {
            let a7611 = 0u64;
            let a7612 = v7;
            nat_lt(a7611, a7612)
        } else {
            false
        } {
            let a7616 = {
                let a7613 = v10;
                let a7614 = v6;
                let a7615 = 1u64;
                nat_rem(a7613, a7614, a7615)
            };
            let a7617 = 0u64;
            if nat_eq(a7616, a7617) {
                let a7621 = {
                    let a7618 = v11;
                    let a7619 = v7;
                    let a7620 = 1u64;
                    nat_rem(a7618, a7619, a7620)
                };
                let a7622 = 0u64;
                nat_eq(a7621, a7622)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a7623 = v10;
                let a7624 = v6;
                let a7625 = 0u64;
                nat_quot(a7623, a7624, a7625)
            }, {
                let a7626 = v11;
                let a7627 = v7;
                let a7628 = 0u64;
                nat_quot(a7626, a7627, a7628)
            }, {
                let a7629 = v5;
                let a7630 = v4;
                nat_mul(a7629, a7630)?
            }, {
                let a7631 = v4;
                let a7632 = v9;
                nat_mul(a7631, a7632)?
            }, {
                let a7633 = v5;
                let a7634 = v9;
                nat_mul(a7633, a7634)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f320(v0: u64) -> bool {
    let a7635 = v0;
    let a7636 = 0u64;
    if if nat_eq(a7635, a7636) {
        true
    } else {
        let a7637 = v0;
        let a7638 = 1u64;
        nat_eq(a7637, a7638)
    } {
        true
    } else {
        let a7639 = v0;
        let a7640 = 30u64;
        nat_eq(a7639, a7640)
    }
}

pub fn f321(v0: u64) -> Adt17 {
    f323(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f322(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a7642 = v0;
        let a7643 = match v1.clone() {
            Adt20::C0(h7641, _, _, _, _, _, _, _) => {
                h7641
            }
        };
        let a7644 = 0u64;
        nat_quot(a7642, a7643, a7644)
    };
    let v3: u64 = {
        let a7646 = v2;
        let a7647 = match v1.clone() {
            Adt20::C0(_, h7645, _, _, _, _, _, _) => {
                h7645
            }
        };
        let a7648 = 0u64;
        nat_quot(a7646, a7647, a7648)
    };
    f150({
        let a7650 = v0;
        let a7651 = match v1.clone() {
            Adt20::C0(h7649, _, _, _, _, _, _, _) => {
                h7649
            }
        };
        let a7652 = 0u64;
        nat_rem(a7650, a7651, a7652)
    }, {
        let a7654 = v2;
        let a7655 = match v1.clone() {
            Adt20::C0(_, h7653, _, _, _, _, _, _) => {
                h7653
            }
        };
        let a7656 = 0u64;
        nat_rem(a7654, a7655, a7656)
    }, {
        let a7658 = v3;
        let a7659 = match v1.clone() {
            Adt20::C0(_, _, h7657, _, _, _, _, _) => {
                h7657
            }
        };
        let a7660 = 0u64;
        nat_rem(a7658, a7659, a7660)
    }, {
        let a7662 = v3;
        let a7663 = match v1.clone() {
            Adt20::C0(_, _, h7661, _, _, _, _, _) => {
                h7661
            }
        };
        let a7664 = 0u64;
        nat_quot(a7662, a7663, a7664)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h7665, _, _, _) => {
            h7665
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h7666, _, _) => {
            h7666
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h7667, _) => {
            h7667
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h7668) => {
            h7668
        }
    })
}

pub fn f323(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a7669 = v0;
    let a7670 = 4u64;
    if if if nat_le(a7669, a7670) {
        true
    } else {
        let a7671 = 9u64;
        let a7672 = v0;
        if nat_le(a7671, a7672) {
            let a7673 = v0;
            let a7674 = 12u64;
            nat_le(a7673, a7674)
        } else {
            false
        }
    } {
        true
    } else {
        let a7675 = v0;
        let a7676 = 16u64;
        if if nat_eq(a7675, a7676) {
            true
        } else {
            let a7677 = v0;
            let a7678 = 19u64;
            nat_eq(a7677, a7678)
        } {
            true
        } else {
            let a7679 = v0;
            let a7680 = 21u64;
            if nat_eq(a7679, a7680) {
                true
            } else {
                let a7681 = v0;
                let a7682 = 22u64;
                nat_eq(a7681, a7682)
            }
        }
    } {
        Adt17::C1({
            let a7683 = v0;
            let a7684 = 4u64;
            if if nat_lt(a7683, a7684) {
                true
            } else {
                let a7685 = 21u64;
                let a7686 = v0;
                nat_le(a7685, a7686)
            } {
                0u64
            } else {
                let a7687 = v0;
                let a7688 = 16u64;
                if if nat_eq(a7687, a7688) {
                    true
                } else {
                    let a7689 = v0;
                    let a7690 = 19u64;
                    nat_eq(a7689, a7690)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a7691 = v0;
            let a7692 = 4u64;
            if nat_lt(a7691, a7692) {
                v0
            } else {
                let a7693 = v0;
                let a7694 = 4u64;
                if if nat_eq(a7693, a7694) {
                    true
                } else {
                    let a7695 = v0;
                    let a7696 = 16u64;
                    nat_eq(a7695, a7696)
                } {
                    0u64
                } else {
                    let a7697 = v0;
                    let a7698 = 19u64;
                    if nat_eq(a7697, a7698) {
                        3u64
                    } else {
                        let a7699 = 21u64;
                        let a7700 = v0;
                        if nat_le(a7699, a7700) {
                            let a7701 = v0;
                            let a7702 = 17u64;
                            nat_sub(a7701, a7702)
                        } else {
                            let a7703 = v0;
                            let a7704 = 8u64;
                            nat_sub(a7703, a7704)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f324(v0: u64) -> R<u64> {
    let a7707 = 2u64;
    let a7708 = {
        let a7705 = 6u64;
        let a7706 = v0;
        nat_mul(a7705, a7706)?
    };
    nat_add(a7707, a7708)
}

pub fn f325(v0: u64) -> R<u64> {
    let a7709 = f324(v0)?;
    let a7710 = 11u64;
    nat_add(a7709, a7710)
}

pub fn f326(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a7711 = v0;
        let a7712 = 6u64;
        let a7713 = 0u64;
        nat_rem(a7711, a7712, a7713)
    };
    let v5: u64 = {
        let a7720 = {
            let a7715 = v1;
            let a7716 = match v2.clone() {
                Adt20::C0(h7714, _, _, _, _, _, _, _) => {
                    h7714
                }
            };
            nat_mul(a7715, a7716)?
        };
        let a7721 = {
            let a7717 = v0;
            let a7718 = 6u64;
            let a7719 = 0u64;
            nat_quot(a7717, a7718, a7719)
        };
        nat_add(a7720, a7721)?
    };
    let v6: u64 = f322(v5, v3)?;
    let v7: u64 = f322(v5, v2)?;
    let a7722 = v4;
    let a7723 = 0u64;
    if nat_eq(a7722, a7723) {
        Ok(f323(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a7724 = v4;
        let a7725 = 1u64;
        if nat_eq(a7724, a7725) {
            Ok(f323(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a7726 = v4;
            let a7727 = 2u64;
            if nat_eq(a7726, a7727) {
                Ok(f323(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a7728 = v4;
                let a7729 = 3u64;
                if nat_eq(a7728, a7729) {
                    Ok(f323(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a7730 = v4;
                    let a7731 = 4u64;
                    if nat_eq(a7730, a7731) {
                        Ok(f323(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f323(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f327(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7732 = v0;
    let a7733 = 0u64;
    if nat_eq(a7732, a7733) {
        f323(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a7734 = v0;
        let a7735 = 1u64;
        if nat_eq(a7734, a7735) {
            f323(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a7736 = v0;
            let a7737 = 2u64;
            if nat_eq(a7736, a7737) {
                f323(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a7738 = v0;
                let a7739 = 3u64;
                if nat_eq(a7738, a7739) {
                    f323(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a7740 = v0;
                    let a7741 = 4u64;
                    if nat_eq(a7740, a7741) {
                        f323(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a7742 = v0;
                        let a7743 = 5u64;
                        if nat_eq(a7742, a7743) {
                            f323(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a7744 = v0;
                            let a7745 = 6u64;
                            if nat_eq(a7744, a7745) {
                                f323(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a7746 = v0;
                                let a7747 = 7u64;
                                if nat_eq(a7746, a7747) {
                                    f323(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a7748 = v0;
                                    let a7749 = 8u64;
                                    if nat_eq(a7748, a7749) {
                                        f323(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a7750 = v0;
                                        let a7751 = 9u64;
                                        if nat_eq(a7750, a7751) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a7752 = v0;
                                            let a7753 = 10u64;
                                            if nat_eq(a7752, a7753) {
                                                f323(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f328(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7754 = v0;
        let a7755 = 3u64;
        let a7756 = 0u64;
        nat_rem(a7754, a7755, a7756)
    };
    let v6: u64 = {
        let a7763 = {
            let a7758 = v1;
            let a7759 = match v2.clone() {
                Adt20::C0(h7757, _, _, _, _, _, _, _) => {
                    h7757
                }
            };
            nat_mul(a7758, a7759)?
        };
        let a7764 = {
            let a7760 = v0;
            let a7761 = 3u64;
            let a7762 = 0u64;
            nat_quot(a7760, a7761, a7762)
        };
        nat_add(a7763, a7764)?
    };
    let v7: u64 = f322(v6, v3)?;
    let v8: u64 = f322(v6, v2)?;
    let v9: u64 = f322(v6, v4)?;
    let a7765 = v5;
    let a7766 = 0u64;
    if nat_eq(a7765, a7766) {
        Ok(f323(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a7767 = v5;
        let a7768 = 1u64;
        if nat_eq(a7767, a7768) {
            Ok(f323(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f323(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f329(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h7769, _, _, _, _, _, _, _) => {
            h7769
        }
    };
    let v5: u64 = {
        let a7772 = 1u64;
        let a7773 = {
            let a7770 = v4;
            let a7771 = f314(v3);
            nat_mul(a7770, a7771)?
        };
        nat_add(a7772, a7773)?
    };
    let a7774 = v0;
    let a7775 = 0u64;
    if nat_eq(a7774, a7775) {
        Ok(f321(0u64))
    } else {
        let a7776 = v0;
        let a7777 = v5;
        if nat_lt(a7776, a7777) {
            let v6: u64 = {
                let a7778 = v0;
                let a7779 = 1u64;
                nat_sub(a7778, a7779)
            };
            let v7: u64 = f322({
                let a7785 = {
                    let a7780 = v1;
                    let a7781 = v4;
                    nat_mul(a7780, a7781)?
                };
                let a7786 = {
                    let a7782 = v6;
                    let a7783 = f314(v3);
                    let a7784 = 0u64;
                    nat_quot(a7782, a7783, a7784)
                };
                nat_add(a7785, a7786)?
            }, v2)?;
            if v3 {
                let a7790 = {
                    let a7787 = v6;
                    let a7788 = 2u64;
                    let a7789 = 0u64;
                    nat_rem(a7787, a7788, a7789)
                };
                let a7791 = 0u64;
                if nat_eq(a7790, a7791) {
                    Ok(f323(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f323(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f323(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a7792 = v0;
                let a7793 = v5;
                nat_sub(a7792, a7793)
            };
            let a7794 = v8;
            let a7795 = 0u64;
            if nat_eq(a7794, a7795) {
                Ok(f323(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a7796 = v8;
                    let a7797 = 1u64;
                    nat_eq(a7796, a7797)
                } else {
                    false
                } {
                    Ok(f323(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a7798 = v8;
                    let a7799 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a7798, a7799) {
                        Ok(f323(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a7800 = v8;
                        let a7801 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a7800, a7801) {
                            Ok(f321(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f330(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a7803 = 13u64;
        let a7804 = f315(match v2.clone() {
            Adt20::C0(h7802, _, _, _, _, _, _, _) => {
                h7802
            }
        });
        nat_mul(a7803, a7804)?
    } else {
        0u64
    };
    let a7805 = v0;
    let a7806 = v5;
    if nat_lt(a7805, a7806) {
        f337({
            let a7807 = v0;
            let a7808 = 13u64;
            let a7809 = 0u64;
            nat_rem(a7807, a7808, a7809)
        }, v1, {
            let a7810 = v0;
            let a7811 = 13u64;
            let a7812 = 0u64;
            nat_quot(a7810, a7811, a7812)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a7813 = v0;
            let a7814 = v5;
            nat_sub(a7813, a7814)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f338({
            let a7815 = v6;
            let a7816 = v7;
            let a7817 = 0u64;
            nat_rem(a7815, a7816, a7817)
        }, {
            let a7827 = {
                let a7822 = {
                    let a7819 = v1;
                    let a7820 = match v2.clone() {
                        Adt20::C0(h7818, _, _, _, _, _, _, _) => {
                            h7818
                        }
                    };
                    nat_mul(a7819, a7820)?
                };
                let a7823 = if v4 {
                    f339(match v2.clone() {
                        Adt20::C0(h7821, _, _, _, _, _, _, _) => {
                            h7821
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a7822, a7823)?
            };
            let a7828 = {
                let a7824 = v6;
                let a7825 = v7;
                let a7826 = 0u64;
                nat_quot(a7824, a7825, a7826)
            };
            nat_add(a7827, a7828)?
        }, v2, v3, v4)
    }
}

pub fn f331(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a7829 = v0;
    let a7830 = 0u64;
    if nat_eq(a7829, a7830) {
        f323(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a7831 = v0;
        let a7832 = 1u64;
        if nat_eq(a7831, a7832) {
            f323(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a7833 = v0;
                let a7834 = 2u64;
                nat_eq(a7833, a7834)
            } else {
                false
            } {
                f323(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a7835 = v0;
                let a7836 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a7835, a7836) {
                    f323(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7837 = v0;
                    let a7838 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a7837, a7838) {
                        f323(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7839 = v0;
                        let a7840 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a7839, a7840) {
                            f323(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f332(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a7844 = 1u64;
        let a7845 = {
            let a7842 = match v4.clone() {
                Adt20::C0(h7841, _, _, _, _, _, _, _) => {
                    h7841
                }
            };
            let a7843 = v6;
            nat_mul(a7842, a7843)?
        };
        nat_add(a7844, a7845)?
    };
    let a7846 = v1;
    let a7847 = 0u64;
    if nat_eq(a7846, a7847) {
        Ok(f321(1u64))
    } else {
        let a7848 = v1;
        let a7849 = v7;
        if nat_lt(a7848, a7849) {
            let v8: u64 = {
                let a7850 = v1;
                let a7851 = 1u64;
                nat_sub(a7850, a7851)
            };
            let v9: u64 = {
                let a7858 = {
                    let a7853 = f335(v2, v3, v4.clone())?;
                    let a7854 = match v4.clone() {
                        Adt20::C0(h7852, _, _, _, _, _, _, _) => {
                            h7852
                        }
                    };
                    nat_mul(a7853, a7854)?
                };
                let a7859 = {
                    let a7855 = v8;
                    let a7856 = v6;
                    let a7857 = 0u64;
                    nat_quot(a7855, a7856, a7857)
                };
                nat_add(a7858, a7859)?
            };
            let v10: u64 = f322(v9, v4)?;
            let v11: u64 = f322(v9, v5)?;
            let v12: u64 = {
                let a7860 = v8;
                let a7861 = v6;
                let a7862 = 0u64;
                nat_rem(a7860, a7861, a7862)
            };
            if v0 {
                let a7863 = v12;
                let a7864 = 0u64;
                if nat_eq(a7863, a7864) {
                    Ok(f323(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a7865 = v12;
                    let a7866 = 1u64;
                    if nat_eq(a7865, a7866) {
                        Ok(f323(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a7867 = v12;
                        let a7868 = 2u64;
                        if nat_eq(a7867, a7868) {
                            Ok(f323(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f323(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a7869 = v12;
                let a7870 = 0u64;
                if nat_eq(a7869, a7870) {
                    Ok(f323(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f323(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a7871 = v1;
            let a7872 = v7;
            if nat_eq(a7871, a7872) {
                Ok(f323(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f333(v0: u64, v1: u64) -> Adt17 {
    let a7873 = v0;
    let a7874 = 0u64;
    if nat_eq(a7873, a7874) {
        f323(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7875 = v0;
        let a7876 = 1u64;
        if nat_eq(a7875, a7876) {
            f323(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a7877 = v0;
            let a7878 = 2u64;
            if nat_eq(a7877, a7878) {
                f323(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a7879 = v0;
                let a7880 = 3u64;
                if nat_eq(a7879, a7880) {
                    f321(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f334(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a7881 = v0;
    let a7882 = 0u64;
    if nat_eq(a7881, a7882) {
        f323(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a7883 = v0;
        let a7884 = 1u64;
        if nat_eq(a7883, a7884) {
            f323(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a7885 = v0;
            let a7886 = 2u64;
            if nat_eq(a7885, a7886) {
                f323(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a7887 = v0;
                let a7888 = 3u64;
                if nat_eq(a7887, a7888) {
                    f323(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a7889 = v0;
                    let a7890 = 4u64;
                    if nat_eq(a7889, a7890) {
                        f323(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a7891 = v0;
                        let a7892 = 5u64;
                        if nat_eq(a7891, a7892) {
                            f323(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f335(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a7903 = {
        let a7901 = {
            let a7898 = {
                let a7895 = match v1.clone() {
                    Adt21::C0(_, _, h7893, _) => {
                        h7893
                    }
                };
                let a7896 = match v2.clone() {
                    Adt20::C0(_, _, h7894, _, _, _, _, _) => {
                        h7894
                    }
                };
                nat_mul(a7895, a7896)?
            };
            let a7899 = match v1 {
                Adt21::C0(h7897, _, _, _) => {
                    h7897
                }
            };
            nat_add(a7898, a7899)?
        };
        let a7902 = match v2 {
            Adt20::C0(_, h7900, _, _, _, _, _, _) => {
                h7900
            }
        };
        nat_mul(a7901, a7902)?
    };
    let a7904 = v0;
    nat_add(a7903, a7904)
}

pub fn f336(v0: u64) -> u64 {
    if f234(v0) {
        let m7905 = f226(v0);
        match m7905 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a7907 = 1u64;
                let a7908 = match v1.clone() {
                    Adt19::C0(_, h7906, _) => {
                        h7906
                    }
                };
                if if nat_lt(a7907, a7908) {
                    true
                } else {
                    let a7909 = v0;
                    let a7910 = 43u64;
                    nat_eq(a7909, a7910)
                } {
                    match v1 {
                        Adt19::C0(_, h7911, _) => {
                            h7911
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

pub fn f337(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a7919 = {
            let a7917 = {
                let a7913 = v1;
                let a7914 = match v3.clone() {
                    Adt20::C0(h7912, _, _, _, _, _, _, _) => {
                        h7912
                    }
                };
                nat_mul(a7913, a7914)?
            };
            let a7918 = {
                let a7915 = v2;
                let a7916 = 4u64;
                nat_mul(a7915, a7916)?
            };
            nat_add(a7917, a7918)?
        };
        let a7920 = f340(v0);
        nat_add(a7919, a7920)?
    };
    let v6: u64 = f322(v5, v3)?;
    let v7: u64 = f322(v5, v4)?;
    let a7921 = v0;
    let a7922 = 0u64;
    if if if if nat_eq(a7921, a7922) {
        true
    } else {
        let a7923 = v0;
        let a7924 = 2u64;
        nat_eq(a7923, a7924)
    } {
        true
    } else {
        let a7925 = v0;
        let a7926 = 5u64;
        nat_eq(a7925, a7926)
    } {
        true
    } else {
        let a7927 = v0;
        let a7928 = 7u64;
        nat_eq(a7927, a7928)
    } {
        Ok(f323(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a7929 = v0;
        let a7930 = 1u64;
        if if if if nat_eq(a7929, a7930) {
            true
        } else {
            let a7931 = v0;
            let a7932 = 3u64;
            nat_eq(a7931, a7932)
        } {
            true
        } else {
            let a7933 = v0;
            let a7934 = 6u64;
            nat_eq(a7933, a7934)
        } {
            true
        } else {
            let a7935 = v0;
            let a7936 = 8u64;
            nat_eq(a7935, a7936)
        } {
            Ok(f323(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a7937 = v0;
                let a7938 = 1u64;
                if nat_eq(a7937, a7938) {
                    4u64
                } else {
                    let a7939 = v0;
                    let a7940 = 6u64;
                    if nat_eq(a7939, a7940) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a7941 = v0;
            let a7942 = 4u64;
            if nat_eq(a7941, a7942) {
                Ok(f323(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a7943 = v0;
                let a7944 = 9u64;
                if nat_eq(a7943, a7944) {
                    Ok(f323(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a7945 = v0;
                    let a7946 = 10u64;
                    if nat_eq(a7945, a7946) {
                        Ok(f323(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a7947 = v0;
                        let a7948 = 11u64;
                        if nat_eq(a7947, a7948) {
                            Ok(f323(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a7949 = v0;
                            let a7950 = 12u64;
                            if nat_eq(a7949, a7950) {
                                Ok(f323(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f338(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f322(v1, v2)?;
    let v6: u64 = f322(v1, v3)?;
    let a7951 = v0;
    let a7952 = 0u64;
    if nat_eq(a7951, a7952) {
        Ok(f323(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a7953 = v0;
        let a7954 = 1u64;
        if nat_eq(a7953, a7954) {
            Ok(f323(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a7955 = v0;
                let a7956 = 2u64;
                if nat_eq(a7955, a7956) {
                    Ok(f323(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a7957 = v0;
                    let a7958 = 3u64;
                    if nat_eq(a7957, a7958) {
                        Ok(f323(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a7959 = v0;
                let a7960 = 2u64;
                if nat_eq(a7959, a7960) {
                    Ok(f323(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f339(v0: u64) -> R<u64> {
    let a7961 = 4u64;
    let a7962 = f315(v0);
    nat_mul(a7961, a7962)
}

pub fn f340(v0: u64) -> u64 {
    let a7963 = v0;
    let a7964 = 5u64;
    if nat_lt(a7963, a7964) {
        let a7965 = v0;
        let a7966 = 2u64;
        if nat_lt(a7965, a7966) {
            0u64
        } else {
            1u64
        }
    } else {
        let a7967 = v0;
        let a7968 = 7u64;
        if nat_lt(a7967, a7968) {
            2u64
        } else {
            3u64
        }
    }
}
