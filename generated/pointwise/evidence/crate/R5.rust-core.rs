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
    C0(u64, Adt1, Option<Adt2>, Option<Adt2>, Option<Adt2>, Option<Adt2>, bool, Adt2),
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
        Adt0::C0(v1, v2, v3, v4, v5, v6, v7, v8) => {
            let a2 = v1;
            let a3 = 85u64;
            if nat_eq(a2, a3) {
                Ok(Adt9::C0(f1(v2, v3, v4, v5, {
                    let m4 = v6;
                    match m4 {
                        None => {
                            v7
                        }
                        Some(_) => {
                            true
                        }
                    }
                }, v8)?))
            } else {
                let a5 = v1;
                let a6 = 87u64;
                if nat_eq(a5, a6) {
                    Ok(Adt9::C0(f2(v3, v4, v5, v6, v7, v8)?))
                } else {
                    let a7 = v1;
                    let a8 = 64u64;
                    if nat_eq(a7, a8) {
                        Ok(Adt9::C0(f3(v2, v8)?))
                    } else {
                        let m9 = v3;
                        match m9 {
                            None => {
                                Ok(Adt9::C0(Adt10::C1))
                            }
                            Some(v10) => {
                                let a10 = v1;
                                let a11 = 48u64;
                                if if nat_eq(a10, a11) {
                                    true
                                } else {
                                    let a12 = v1;
                                    let a13 = 56u64;
                                    nat_eq(a12, a13)
                                } {
                                    Ok(Adt9::C0(f4({
                                        let a14 = v1;
                                        let a15 = 56u64;
                                        nat_eq(a14, a15)
                                    }, v2, v10, v4, v8)?))
                                } else {
                                    let a16 = v1;
                                    let a17 = 53u64;
                                    if if nat_eq(a16, a17) {
                                        true
                                    } else {
                                        let a18 = v1;
                                        let a19 = 54u64;
                                        nat_eq(a18, a19)
                                    } {
                                        Ok(Adt9::C0(f5({
                                            let a20 = v1;
                                            let a21 = 54u64;
                                            nat_eq(a20, a21)
                                        }, v2, v10, v4, v8)?))
                                    } else {
                                        let a22 = v1;
                                        let a23 = 78u64;
                                        if nat_eq(a22, a23) {
                                            Ok(Adt9::C0(f6(v2, v10, v4, v5, v8)?))
                                        } else {
                                            let a24 = v1;
                                            let a25 = 23u64;
                                            if if if nat_eq(a24, a25) {
                                                true
                                            } else {
                                                let a26 = v1;
                                                let a27 = 26u64;
                                                nat_eq(a26, a27)
                                            } {
                                                true
                                            } else {
                                                let a28 = v1;
                                                let a29 = 25u64;
                                                nat_eq(a28, a29)
                                            } {
                                                Ok(Adt9::C0(f7(v1, v2, v10, v4, v8)?))
                                            } else {
                                                let a30 = v1;
                                                let a31 = 24u64;
                                                if nat_eq(a30, a31) {
                                                    Ok(Adt9::C0(f8(v2, v10, v8)?))
                                                } else {
                                                    let a32 = v1;
                                                    let a33 = 27u64;
                                                    if nat_eq(a32, a33) {
                                                        Ok(Adt9::C0(f9(v2, v10, v8)?))
                                                    } else {
                                                        let a34 = v1;
                                                        let a35 = 3u64;
                                                        if nat_eq(a34, a35) {
                                                            Ok(Adt9::C0(f10(v10, v4, v5, v8)?))
                                                        } else {
                                                            f11(v1, v2, v10, v4, v8)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Adt0::C1(v16, v17, v18, v19, v20) => {
            f14(v16, v17, v18, v19, v20)
        }
        Adt0::C2(v31) => {
            f16(v31)
        }
        Adt0::C3(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44) => {
            Ok(f17(v32, v33, v34, v35, v36, v37, v38, v39, v40, v41, v42, v43, v44))
        }
        Adt0::C4(v11, v12, v13, v14) => {
            let a37 = 0u64;
            let a38 = match v11.clone() {
                Adt8::C0(_, _, _, _, _, _, _, h36, _, _, _, _, _, _, _) => {
                    h36
                }
            };
            if nat_lt(a37, a38) {
                f12(v11, v12, v13, v14)
            } else {
                f13(v11, v12, v13, v14)
            }
        }
        Adt0::C5(v15) => {
            Ok(Adt9::C9({
                let a39 = 3240099840u64;
                let a40 = v15;
                if if nat_le(a39, a40) {
                    let a41 = v15;
                    let a42 = 4286578688u64;
                    nat_le(a41, a42)
                } else {
                    false
                } {
                    Some(0u64)
                } else {
                    let a43 = 1092616192u64;
                    let a44 = v15;
                    if if nat_le(a43, a44) {
                        let a45 = v15;
                        let a46 = 2139095040u64;
                        nat_le(a45, a46)
                    } else {
                        false
                    } {
                        Some(v15)
                    } else {
                        None::<u64>
                    }
                }
            }))
        }
        Adt0::C6(v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) => {
            f15(v21, v22, v23, v24, v25, true, v26, v27, v28, v29, v30)
        }
    }
}

pub fn f1(v0: Adt1, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m47 = v1.clone();
        match m47 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m48 = v2.clone();
        match m48 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m49 = v3.clone();
        match m49 {
            None => {
                v5.clone()
            }
            Some(v10) => {
                v10
            }
        }
    };
    let v12: u64 = match v0.clone() {
        Adt1::C0(_, h50, _, _, _, _, _, _, _, _, _, _) => {
            h50
        }
    };
    let m51 = v1;
    if if if if if if if if if if if if if if if if if if if if if if if if if match m51 {
        None => {
            false
        }
        Some(_) => {
            true
        }
    } {
        let m52 = v2;
        match m52 {
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
        let m53 = v3;
        match m53 {
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
        let a55 = match v7.clone() {
            Adt2::C0(h54, _, _, _, _, _, _, _, _, _, _) => {
                h54
            }
        };
        let a56 = 0u64;
        if if nat_eq(a55, a56) {
            f18(v7.clone(), f19(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f20(v7.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a58 = match v9.clone() {
            Adt2::C0(h57, _, _, _, _, _, _, _, _, _, _) => {
                h57
            }
        };
        let a59 = 0u64;
        if if nat_eq(a58, a59) {
            f18(v9.clone(), f19(v9.clone(), 1u64)?)?
        } else {
            false
        } {
            f20(v9.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a61 = match v11.clone() {
            Adt2::C0(h60, _, _, _, _, _, _, _, _, _, _) => {
                h60
            }
        };
        let a62 = 0u64;
        if if nat_eq(a61, a62) {
            f18(v11.clone(), f19(v11.clone(), 1u64)?)?
        } else {
            false
        } {
            f20(v11.clone(), v5.clone())?
        } else {
            false
        }
    } else {
        false
    } {
        let a64 = match v7.clone() {
            Adt2::C0(_, h63, _, _, _, _, _, _, _, _, _) => {
                h63
            }
        };
        let a65 = 24u64;
        nat_eq(a64, a65)
    } else {
        false
    } {
        let a67 = match v7.clone() {
            Adt2::C0(_, _, _, h66, _, _, _, _, _, _, _) => {
                h66
            }
        };
        let a68 = 1u64;
        nat_eq(a67, a68)
    } else {
        false
    } {
        let a70 = match v7.clone() {
            Adt2::C0(_, _, _, _, h69, _, _, _, _, _, _) => {
                h69
            }
        };
        let a71 = 1u64;
        nat_eq(a70, a71)
    } else {
        false
    } {
        let a73 = match v11.clone() {
            Adt2::C0(_, h72, _, _, _, _, _, _, _, _, _) => {
                h72
            }
        };
        let a74 = 24u64;
        nat_eq(a73, a74)
    } else {
        false
    } {
        let a76 = match v11.clone() {
            Adt2::C0(_, _, h75, _, _, _, _, _, _, _, _) => {
                h75
            }
        };
        let a77 = 1u64;
        nat_eq(a76, a77)
    } else {
        false
    } {
        let a79 = match v11.clone() {
            Adt2::C0(_, _, _, h78, _, _, _, _, _, _, _) => {
                h78
            }
        };
        let a80 = 1u64;
        nat_eq(a79, a80)
    } else {
        false
    } {
        let a82 = match v11.clone() {
            Adt2::C0(_, _, _, _, h81, _, _, _, _, _, _) => {
                h81
            }
        };
        let a83 = 1u64;
        nat_eq(a82, a83)
    } else {
        false
    } {
        let a85 = match v9.clone() {
            Adt2::C0(_, _, h84, _, _, _, _, _, _, _, _) => {
                h84
            }
        };
        let a86 = 1u64;
        nat_eq(a85, a86)
    } else {
        false
    } {
        let a88 = match v9.clone() {
            Adt2::C0(_, _, _, h87, _, _, _, _, _, _, _) => {
                h87
            }
        };
        let a89 = 1u64;
        nat_eq(a88, a89)
    } else {
        false
    } {
        let a91 = match v9.clone() {
            Adt2::C0(_, _, _, _, h90, _, _, _, _, _, _) => {
                h90
            }
        };
        let a92 = 1u64;
        nat_eq(a91, a92)
    } else {
        false
    } {
        let a94 = 3u64;
        let a95 = match v9.clone() {
            Adt2::C0(_, h93, _, _, _, _, _, _, _, _, _) => {
                h93
            }
        };
        nat_le(a94, a95)
    } else {
        false
    } {
        let a97 = match v5.clone() {
            Adt2::C0(_, h96, _, _, _, _, _, _, _, _, _) => {
                h96
            }
        };
        let a98 = 4u64;
        nat_eq(a97, a98)
    } else {
        false
    } {
        let a100 = match v5.clone() {
            Adt2::C0(_, _, h99, _, _, _, _, _, _, _, _) => {
                h99
            }
        };
        let a101 = 4u64;
        nat_eq(a100, a101)
    } else {
        false
    } {
        let a104 = match v5.clone() {
            Adt2::C0(_, _, _, h102, _, _, _, _, _, _, _) => {
                h102
            }
        };
        let a105 = match v7.clone() {
            Adt2::C0(_, _, h103, _, _, _, _, _, _, _, _) => {
                h103
            }
        };
        nat_eq(a104, a105)
    } else {
        false
    } {
        let a107 = match v5.clone() {
            Adt2::C0(_, _, _, _, h106, _, _, _, _, _, _) => {
                h106
            }
        };
        let a108 = 1u64;
        nat_eq(a107, a108)
    } else {
        false
    } {
        f21(v5.clone())?
    } else {
        false
    } {
        let a109 = 0u64;
        let a110 = v12;
        nat_lt(a109, a110)
    } else {
        false
    } {
        let a111 = v12;
        let a112 = 2147483648u64;
        nat_lt(a111, a112)
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a114 = match v7.clone() {
                Adt2::C0(_, _, h113, _, _, _, _, _, _, _, _) => {
                    h113
                }
            };
            let a115 = f22(v12)?;
            nat_mul(a114, a115)?
        }, 4u64, f23(v7, 1u64), f23(v5, 1u64), Adt4::C38(f23(v9, 1u64), f23(v11, 1u64), match v0 {
            Adt1::C0(h116, _, _, _, _, _, _, _, _, _, _, _) => {
                h116
            }
        }, v12)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f2(v0: Option<Adt2>, v1: Option<Adt2>, v2: Option<Adt2>, v3: Option<Adt2>, v4: bool, v5: Adt2) -> R<Adt10> {
    let v7: Adt2 = {
        let m117 = v0.clone();
        match m117 {
            None => {
                v5.clone()
            }
            Some(v6) => {
                v6
            }
        }
    };
    let v9: Adt2 = {
        let m118 = v1.clone();
        match m118 {
            None => {
                v5.clone()
            }
            Some(v8) => {
                v8
            }
        }
    };
    let v11: Adt2 = {
        let m119 = v2.clone();
        match m119 {
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
        let m120 = v0;
        match m120 {
            None => {
                false
            }
            Some(_) => {
                let m121 = v1;
                match m121 {
                    None => {
                        false
                    }
                    Some(_) => {
                        let m122 = v2;
                        match m122 {
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
        let a124 = match v7.clone() {
            Adt2::C0(h123, _, _, _, _, _, _, _, _, _, _) => {
                h123
            }
        };
        let a125 = 0u64;
        if if if if if if if if if if if if if if nat_eq(a124, a125) {
            let a127 = match v7.clone() {
                Adt2::C0(_, _, _, _, h126, _, _, _, _, _, _) => {
                    h126
                }
            };
            let a128 = 1u64;
            nat_eq(a127, a128)
        } else {
            false
        } {
            f18(v7.clone(), f19(v7.clone(), 1u64)?)?
        } else {
            false
        } {
            f20(v7.clone(), v5.clone())?
        } else {
            false
        } {
            let a130 = match v9.clone() {
                Adt2::C0(h129, _, _, _, _, _, _, _, _, _, _) => {
                    h129
                }
            };
            let a131 = 0u64;
            if if if nat_eq(a130, a131) {
                let a133 = match v9.clone() {
                    Adt2::C0(_, _, _, _, h132, _, _, _, _, _, _) => {
                        h132
                    }
                };
                let a134 = 1u64;
                nat_eq(a133, a134)
            } else {
                false
            } {
                f18(v9.clone(), f19(v9.clone(), 1u64)?)?
            } else {
                false
            } {
                f20(v9.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a136 = match v11.clone() {
                Adt2::C0(h135, _, _, _, _, _, _, _, _, _, _) => {
                    h135
                }
            };
            let a137 = 0u64;
            if if if nat_eq(a136, a137) {
                let a139 = match v11.clone() {
                    Adt2::C0(_, _, _, _, h138, _, _, _, _, _, _) => {
                        h138
                    }
                };
                let a140 = 1u64;
                nat_eq(a139, a140)
            } else {
                false
            } {
                f18(v11.clone(), f19(v11.clone(), 1u64)?)?
            } else {
                false
            } {
                f20(v11.clone(), v5.clone())?
            } else {
                false
            }
        } else {
            false
        } {
            let a142 = match v7.clone() {
                Adt2::C0(_, _, _, h141, _, _, _, _, _, _, _) => {
                    h141
                }
            };
            let a143 = 1u64;
            nat_eq(a142, a143)
        } else {
            false
        } {
            let a145 = match v11.clone() {
                Adt2::C0(_, _, _, h144, _, _, _, _, _, _, _) => {
                    h144
                }
            };
            let a146 = 1u64;
            nat_eq(a145, a146)
        } else {
            false
        } {
            let a149 = match v9.clone() {
                Adt2::C0(_, h147, _, _, _, _, _, _, _, _, _) => {
                    h147
                }
            };
            let a150 = match v7.clone() {
                Adt2::C0(_, h148, _, _, _, _, _, _, _, _, _) => {
                    h148
                }
            };
            nat_eq(a149, a150)
        } else {
            false
        } {
            let a153 = match v9.clone() {
                Adt2::C0(_, _, _, h151, _, _, _, _, _, _, _) => {
                    h151
                }
            };
            let a154 = match v7.clone() {
                Adt2::C0(_, _, h152, _, _, _, _, _, _, _, _) => {
                    h152
                }
            };
            nat_eq(a153, a154)
        } else {
            false
        } {
            let a157 = match v11.clone() {
                Adt2::C0(_, h155, _, _, _, _, _, _, _, _, _) => {
                    h155
                }
            };
            let a158 = match v9.clone() {
                Adt2::C0(_, _, h156, _, _, _, _, _, _, _, _) => {
                    h156
                }
            };
            nat_eq(a157, a158)
        } else {
            false
        } {
            let a161 = match v11.clone() {
                Adt2::C0(_, _, h159, _, _, _, _, _, _, _, _) => {
                    h159
                }
            };
            let a162 = match v7.clone() {
                Adt2::C0(_, _, h160, _, _, _, _, _, _, _, _) => {
                    h160
                }
            };
            nat_eq(a161, a162)
        } else {
            false
        } {
            let a164 = 0u64;
            let a165 = match v9.clone() {
                Adt2::C0(_, _, h163, _, _, _, _, _, _, _, _) => {
                    h163
                }
            };
            if nat_lt(a164, a165) {
                f24(v9.clone(), v5.clone())
            } else {
                false
            }
        } else {
            false
        } {
            f21(v5.clone())?
        } else {
            false
        } {
            let m166 = v3.clone();
            match m166 {
                None => {
                    true
                }
                Some(v15) => {
                    let a168 = match v15.clone() {
                        Adt2::C0(h167, _, _, _, _, _, _, _, _, _, _) => {
                            h167
                        }
                    };
                    let a169 = 0u64;
                    if if if if if if nat_eq(a168, a169) {
                        let a171 = match v15.clone() {
                            Adt2::C0(_, _, _, _, h170, _, _, _, _, _, _) => {
                                h170
                            }
                        };
                        let a172 = 1u64;
                        nat_eq(a171, a172)
                    } else {
                        false
                    } {
                        f18(v15.clone(), f19(v15.clone(), 1u64)?)?
                    } else {
                        false
                    } {
                        f20(v15.clone(), v5.clone())?
                    } else {
                        false
                    } {
                        let a175 = match v15.clone() {
                            Adt2::C0(_, h173, _, _, _, _, _, _, _, _, _) => {
                                h173
                            }
                        };
                        let a176 = match v9.clone() {
                            Adt2::C0(_, _, h174, _, _, _, _, _, _, _, _) => {
                                h174
                            }
                        };
                        nat_eq(a175, a176)
                    } else {
                        false
                    } {
                        let a179 = match v15.clone() {
                            Adt2::C0(_, _, h177, _, _, _, _, _, _, _, _) => {
                                h177
                            }
                        };
                        let a180 = match v9.clone() {
                            Adt2::C0(_, _, h178, _, _, _, _, _, _, _, _) => {
                                h178
                            }
                        };
                        nat_eq(a179, a180)
                    } else {
                        false
                    } {
                        let a183 = match v15 {
                            Adt2::C0(_, _, _, h181, _, _, _, _, _, _, _) => {
                                h181
                            }
                        };
                        let a184 = match v7.clone() {
                            Adt2::C0(_, _, h182, _, _, _, _, _, _, _, _) => {
                                h182
                            }
                        };
                        nat_eq(a183, a184)
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
            let m185 = v3;
            match m185 {
                None => {
                    None::<Adt3>
                }
                Some(v16) => {
                    Some(f23(v16, 1u64))
                }
            }
        };
        Ok(Adt10::C4({
            let a187 = f19(v5.clone(), 1u64)?;
            let a188 = f25(match v9.clone() {
                Adt2::C0(_, _, h186, _, _, _, _, _, _, _, _) => {
                    h186
                }
            }, v17.clone())?;
            nat_mul(a187, a188)?
        }, 4u64, f23(v7, 1u64), f23(v5, 1u64), Adt4::C37(f23(v9, 1u64), f23(v11, 1u64), v17)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f3(v0: Adt1, v1: Adt2) -> R<Adt10> {
    let v2: u64 = f19(v1.clone(), 1u64)?;
    let a190 = match v1.clone() {
        Adt2::C0(h189, _, _, _, _, _, _, _, _, _, _) => {
            h189
        }
    };
    let a191 = 0u64;
    if if if if if if if if nat_eq(a190, a191) {
        let a193 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h192, _, _, _, _, _) => {
                h192
            }
        };
        let a194 = 4u64;
        nat_eq(a193, a194)
    } else {
        false
    } {
        f26(match v0.clone() {
            Adt1::C0(h195, _, _, _, _, _, _, _, _, _, _, _) => {
                h195
            }
        })
    } else {
        false
    } {
        f26(match v0.clone() {
            Adt1::C0(_, h196, _, _, _, _, _, _, _, _, _, _) => {
                h196
            }
        })
    } else {
        false
    } {
        let a198 = 0u64;
        let a199 = match v0.clone() {
            Adt1::C0(_, _, h197, _, _, _, _, _, _, _, _, _) => {
                h197
            }
        };
        nat_lt(a198, a199)
    } else {
        false
    } {
        let a201 = match v0.clone() {
            Adt1::C0(_, _, h200, _, _, _, _, _, _, _, _, _) => {
                h200
            }
        };
        let a202 = 2139095040u64;
        nat_lt(a201, a202)
    } else {
        false
    } {
        let a203 = v2;
        let a204 = 9223372036854775808u64;
        nat_lt(a203, a204)
    } else {
        false
    } {
        let a206 = f27(v2)?;
        let a207 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h205, _) => {
                h205
            }
        };
        nat_le(a206, a207)
    } else {
        false
    } {
        Ok(Adt10::C4(f28(v2)?, 4u64, f23(v1.clone(), 1u64), f23(v1, 1u64), Adt4::C23(match v0.clone() {
            Adt1::C0(h208, _, _, _, _, _, _, _, _, _, _, _) => {
                h208
            }
        }, match v0.clone() {
            Adt1::C0(_, h209, _, _, _, _, _, _, _, _, _, _) => {
                h209
            }
        }, match v0 {
            Adt1::C0(_, _, h210, _, _, _, _, _, _, _, _, _) => {
                h210
            }
        }, v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f4(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m211 = v3;
    match m211 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt3 = f29(v0, v2.clone());
            let v7: Adt3 = f29(v0, v5.clone());
            let v8: Adt3 = f29(v0, v4.clone());
            let v9: u64 = match v7.clone() {
                Adt3::C0(h212, _, _, _, _, _, _, _) => {
                    h212
                }
            };
            let v10: u64 = match v7.clone() {
                Adt3::C0(_, h213, _, _, _, _, _, _) => {
                    h213
                }
            };
            let v11: u64 = match v7.clone() {
                Adt3::C0(_, _, h214, _, _, _, _, _) => {
                    h214
                }
            };
            let v12: u64 = match v7.clone() {
                Adt3::C0(_, _, _, h215, _, _, _, _) => {
                    h215
                }
            };
            let v13: u64 = match v6.clone() {
                Adt3::C0(h216, _, _, _, _, _, _, _) => {
                    h216
                }
            };
            let v14: u64 = match v6.clone() {
                Adt3::C0(_, h217, _, _, _, _, _, _) => {
                    h217
                }
            };
            let v15: u64 = match v6.clone() {
                Adt3::C0(_, _, h218, _, _, _, _, _) => {
                    h218
                }
            };
            let v16: u64 = match v1.clone() {
                Adt1::C0(h219, _, _, _, _, _, _, _, _, _, _, _) => {
                    h219
                }
            };
            let v17: u64 = {
                let a224 = v13;
                let a225 = {
                    let a222 = {
                        let a220 = v9;
                        let a221 = 1u64;
                        nat_sub(a220, a221)
                    };
                    let a223 = v16;
                    nat_mul(a222, a223)?
                };
                nat_add(a224, a225)?
            };
            let v18: u64 = {
                let a230 = v14;
                let a231 = {
                    let a228 = {
                        let a226 = v10;
                        let a227 = 1u64;
                        nat_sub(a226, a227)
                    };
                    let a229 = v16;
                    nat_mul(a228, a229)?
                };
                nat_add(a230, a231)?
            };
            let v19: u64 = {
                let a234 = {
                    let a232 = v9;
                    let a233 = v10;
                    nat_mul(a232, a233)?
                };
                let a235 = v12;
                nat_mul(a234, a235)?
            };
            let v20: u64 = {
                let a238 = {
                    let a236 = v13;
                    let a237 = v14;
                    nat_mul(a236, a237)?
                };
                let a239 = v15;
                nat_mul(a238, a239)?
            };
            let v21: u64 = {
                let a240 = v19;
                let a241 = 32u64;
                if nat_lt(a240, a241) {
                    v19
                } else {
                    32u64
                }
            };
            let a243 = match v2.clone() {
                Adt2::C0(h242, _, _, _, _, _, _, _, _, _, _) => {
                    h242
                }
            };
            let a244 = 0u64;
            if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a243, a244) {
                true
            } else {
                let a246 = match v2.clone() {
                    Adt2::C0(h245, _, _, _, _, _, _, _, _, _, _) => {
                        h245
                    }
                };
                let a247 = 1u64;
                nat_eq(a246, a247)
            } {
                let a249 = match v5.clone() {
                    Adt2::C0(h248, _, _, _, _, _, _, _, _, _, _) => {
                        h248
                    }
                };
                let a250 = 0u64;
                nat_eq(a249, a250)
            } else {
                false
            } {
                let a252 = match v4.clone() {
                    Adt2::C0(h251, _, _, _, _, _, _, _, _, _, _) => {
                        h251
                    }
                };
                let a253 = 0u64;
                nat_eq(a252, a253)
            } else {
                false
            } {
                let a254 = v16;
                let a255 = 2147483648u64;
                nat_lt(a254, a255)
            } else {
                false
            } {
                let a256 = 0u64;
                let a257 = v9;
                nat_lt(a256, a257)
            } else {
                false
            } {
                let a258 = 0u64;
                let a259 = v10;
                nat_lt(a258, a259)
            } else {
                false
            } {
                let a260 = 0u64;
                let a261 = v11;
                nat_lt(a260, a261)
            } else {
                false
            } {
                let a262 = 0u64;
                let a263 = v12;
                nat_lt(a262, a263)
            } else {
                false
            } {
                let a264 = 0u64;
                let a265 = v13;
                nat_lt(a264, a265)
            } else {
                false
            } {
                let a266 = 0u64;
                let a267 = v14;
                nat_lt(a266, a267)
            } else {
                false
            } {
                let a268 = 0u64;
                let a269 = v15;
                nat_lt(a268, a269)
            } else {
                false
            } {
                let a270 = 0u64;
                let a271 = v16;
                nat_lt(a270, a271)
            } else {
                false
            } {
                let a273 = match v6.clone() {
                    Adt3::C0(_, _, _, h272, _, _, _, _) => {
                        h272
                    }
                };
                let a274 = v11;
                nat_eq(a273, a274)
            } else {
                false
            } {
                let a276 = match v8.clone() {
                    Adt3::C0(h275, _, _, _, _, _, _, _) => {
                        h275
                    }
                };
                let a277 = v17;
                nat_eq(a276, a277)
            } else {
                false
            } {
                let a279 = match v8.clone() {
                    Adt3::C0(_, h278, _, _, _, _, _, _) => {
                        h278
                    }
                };
                let a280 = v18;
                nat_eq(a279, a280)
            } else {
                false
            } {
                let a282 = match v8.clone() {
                    Adt3::C0(_, _, h281, _, _, _, _, _) => {
                        h281
                    }
                };
                let a283 = v15;
                nat_eq(a282, a283)
            } else {
                false
            } {
                let a285 = match v8.clone() {
                    Adt3::C0(_, _, _, h284, _, _, _, _) => {
                        h284
                    }
                };
                let a286 = v12;
                nat_eq(a285, a286)
            } else {
                false
            } {
                f30(v4.clone())?
            } else {
                false
            } {
                let a291 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, h287, _, _, _, _, _) => {
                        h287
                    }
                };
                let a292 = {
                    let a289 = match v2.clone() {
                        Adt2::C0(h288, _, _, _, _, _, _, _, _, _, _) => {
                            h288
                        }
                    };
                    let a290 = 1u64;
                    if nat_eq(a289, a290) {
                        2u64
                    } else {
                        4u64
                    }
                };
                nat_eq(a291, a292)
            } else {
                false
            } {
                let a294 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h293, _, _, _, _, _) => {
                        h293
                    }
                };
                let a295 = 4u64;
                nat_eq(a294, a295)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f20(v5.clone(), v4.clone())?
            } else {
                false
            } {
                if v0 {
                    let a300 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h296, _, _, _, _) => {
                            h296
                        }
                    };
                    let a301 = {
                        let a298 = v13;
                        let a299 = match v2 {
                            Adt2::C0(_, _, _, _, _, h297, _, _, _, _, _) => {
                                h297
                            }
                        };
                        nat_mul(a298, a299)?
                    };
                    nat_eq(a300, a301)
                } else {
                    let a303 = match v1.clone() {
                        Adt1::C0(_, h302, _, _, _, _, _, _, _, _, _, _) => {
                            h302
                        }
                    };
                    let a304 = 0u64;
                    if if if if if if nat_eq(a303, a304) {
                        let a306 = match v1 {
                            Adt1::C0(_, _, h305, _, _, _, _, _, _, _, _, _) => {
                                h305
                            }
                        };
                        let a307 = 1u64;
                        nat_eq(a306, a307)
                    } else {
                        false
                    } {
                        let a309 = match v2 {
                            Adt2::C0(_, _, _, _, h308, _, _, _, _, _, _) => {
                                h308
                            }
                        };
                        let a310 = 1u64;
                        nat_eq(a309, a310)
                    } else {
                        false
                    } {
                        let a312 = match v5.clone() {
                            Adt2::C0(_, _, _, h311, _, _, _, _, _, _, _) => {
                                h311
                            }
                        };
                        let a313 = 1u64;
                        nat_eq(a312, a313)
                    } else {
                        false
                    } {
                        let a315 = match v5 {
                            Adt2::C0(_, _, _, _, h314, _, _, _, _, _, _) => {
                                h314
                            }
                        };
                        let a316 = 1u64;
                        nat_eq(a315, a316)
                    } else {
                        false
                    } {
                        let a318 = match v4.clone() {
                            Adt2::C0(_, _, _, h317, _, _, _, _, _, _, _) => {
                                h317
                            }
                        };
                        let a319 = 1u64;
                        nat_eq(a318, a319)
                    } else {
                        false
                    } {
                        let a321 = match v4 {
                            Adt2::C0(_, _, _, _, h320, _, _, _, _, _, _) => {
                                h320
                            }
                        };
                        let a322 = 1u64;
                        nat_eq(a321, a322)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v19, v11, v20, {
                    let a323 = v11;
                    let a324 = v20;
                    nat_mul(a323, a324)?
                }, v21, {
                    let a325 = v21;
                    let a326 = v11;
                    nat_mul(a325, a326)?
                }, {
                    let a327 = v21;
                    let a328 = v20;
                    nat_mul(a327, a328)?
                }, {
                    let a333 = {
                        let a329 = v17;
                        let a330 = v18;
                        nat_mul(a329, a330)?
                    };
                    let a334 = {
                        let a331 = v15;
                        let a332 = v12;
                        nat_mul(a331, a332)?
                    };
                    nat_mul(a333, a334)?
                }, v6, v7, v8, Adt6::C0(v13, v16, 0u64, 1u64), Adt6::C0(v14, v16, 0u64, 1u64), Adt6::C0(1u64, 1u64, 0u64, 1u64), 0u64)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f5(v0: bool, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m335 = v3;
    match m335 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, h336, _, _, _, _, _, _, _, _, _) => {
                    h336
                }
            }, match v1.clone() {
                Adt1::C0(h337, _, _, _, _, _, _, _, _, _, _, _) => {
                    h337
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, h338, _, _, _, _, _, _, _, _) => {
                        h338
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, h339, _, _, _, _, _, _, _, _, _) => {
                        h339
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h340, _, _, _, _, _) => {
                        h340
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h341, _, _, _, _, _, _, _) => {
                        h341
                    }
                }
            });
            let v7: Adt6 = Adt6::C0(match v2.clone() {
                Adt2::C0(_, _, h342, _, _, _, _, _, _, _, _) => {
                    h342
                }
            }, match v1.clone() {
                Adt1::C0(_, h343, _, _, _, _, _, _, _, _, _, _) => {
                    h343
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, h344, _, _, _, _, _, _, _) => {
                        h344
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, h345, _, _, _, _, _, _, _, _) => {
                        h345
                    }
                }
            }, if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h346, _, _, _, _) => {
                        h346
                    }
                }
            } else {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h347, _, _, _, _, _, _) => {
                        h347
                    }
                }
            });
            let v8: Adt6 = if v0 {
                Adt6::C0(match v2.clone() {
                    Adt2::C0(_, _, _, h348, _, _, _, _, _, _, _) => {
                        h348
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h349, _, _, _, _, _, _, _, _, _) => {
                        h349
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h350, _, _, _, _, _, _) => {
                        h350
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h351, _, _, _) => {
                        h351
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v9: u64 = f31(v6.clone(), match v5.clone() {
                Adt2::C0(_, h352, _, _, _, _, _, _, _, _, _) => {
                    h352
                }
            })?;
            let v10: u64 = f31(v7.clone(), match v5.clone() {
                Adt2::C0(_, _, h353, _, _, _, _, _, _, _, _) => {
                    h353
                }
            })?;
            let v11: u64 = if v0 {
                f31(v8.clone(), match v5.clone() {
                    Adt2::C0(_, _, _, h354, _, _, _, _, _, _, _) => {
                        h354
                    }
                })?
            } else {
                1u64
            };
            let v12: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, h355, _, _) => {
                        h355
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, h356, _, _, _, _, _, _, _) => {
                        h356
                    }
                }
            };
            let v13: u64 = if v0 {
                match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, h357, _) => {
                        h357
                    }
                }
            } else {
                match v5.clone() {
                    Adt2::C0(_, _, _, _, h358, _, _, _, _, _, _) => {
                        h358
                    }
                }
            };
            let v14: u64 = if v0 {
                match v1 {
                    Adt1::C0(_, _, _, _, _, _, _, _, _, _, _, h359) => {
                        h359
                    }
                }
            } else {
                match v2.clone() {
                    Adt2::C0(_, _, _, _, h360, _, _, _, _, _, _) => {
                        h360
                    }
                }
            };
            let v15: u64 = {
                let a365 = {
                    let a361 = v9;
                    let a362 = v10;
                    nat_mul(a361, a362)?
                };
                let a366 = {
                    let a363 = v11;
                    let a364 = v13;
                    nat_mul(a363, a364)?
                };
                nat_mul(a365, a366)?
            };
            let v16: u64 = {
                let a374 = {
                    let a369 = match v6.clone() {
                        Adt6::C0(h367, _, _, _) => {
                            h367
                        }
                    };
                    let a370 = match v7.clone() {
                        Adt6::C0(h368, _, _, _) => {
                            h368
                        }
                    };
                    nat_mul(a369, a370)?
                };
                let a375 = {
                    let a372 = match v8.clone() {
                        Adt6::C0(h371, _, _, _) => {
                            h371
                        }
                    };
                    let a373 = v12;
                    nat_mul(a372, a373)?
                };
                nat_mul(a374, a375)?
            };
            let v17: u64 = {
                let a376 = v15;
                let a377 = 32u64;
                if nat_lt(a376, a377) {
                    v15
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
            if if if if if if if if if if if nat_eq(a379, a380) {
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
                let a391 = match v4.clone() {
                    Adt2::C0(_, h390, _, _, _, _, _, _, _, _, _) => {
                        h390
                    }
                };
                let a392 = v9;
                nat_eq(a391, a392)
            } else {
                false
            } {
                let a394 = match v4.clone() {
                    Adt2::C0(_, _, h393, _, _, _, _, _, _, _, _) => {
                        h393
                    }
                };
                let a395 = v10;
                nat_eq(a394, a395)
            } else {
                false
            } {
                if v0 {
                    let a396 = 0u64;
                    let a397 = v12;
                    if if if if if if if nat_lt(a396, a397) {
                        let a398 = v12;
                        let a399 = 2147483648u64;
                        nat_lt(a398, a399)
                    } else {
                        false
                    } {
                        let a400 = v14;
                        let a401 = 2147483648u64;
                        nat_lt(a400, a401)
                    } else {
                        false
                    } {
                        let a402 = v13;
                        let a403 = 2147483648u64;
                        nat_lt(a402, a403)
                    } else {
                        false
                    } {
                        let a407 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h404, _, _, _, _, _, _) => {
                                h404
                            }
                        };
                        let a408 = {
                            let a405 = v12;
                            let a406 = v14;
                            nat_mul(a405, a406)?
                        };
                        nat_eq(a407, a408)
                    } else {
                        false
                    } {
                        let a412 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h409, _, _, _, _, _, _) => {
                                h409
                            }
                        };
                        let a413 = {
                            let a410 = v12;
                            let a411 = v13;
                            nat_mul(a410, a411)?
                        };
                        nat_eq(a412, a413)
                    } else {
                        false
                    } {
                        let a415 = match v4.clone() {
                            Adt2::C0(_, _, _, h414, _, _, _, _, _, _, _) => {
                                h414
                            }
                        };
                        let a416 = v11;
                        nat_eq(a415, a416)
                    } else {
                        false
                    } {
                        let a420 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h417, _, _, _, _, _, _) => {
                                h417
                            }
                        };
                        let a421 = {
                            let a418 = v14;
                            let a419 = v13;
                            nat_mul(a418, a419)?
                        };
                        nat_eq(a420, a421)
                    } else {
                        false
                    }
                } else {
                    let a423 = match v2.clone() {
                        Adt2::C0(_, _, _, h422, _, _, _, _, _, _, _) => {
                            h422
                        }
                    };
                    let a424 = v12;
                    if if if nat_eq(a423, a424) {
                        let a426 = match v4.clone() {
                            Adt2::C0(_, _, _, h425, _, _, _, _, _, _, _) => {
                                h425
                            }
                        };
                        let a427 = v14;
                        nat_eq(a426, a427)
                    } else {
                        false
                    } {
                        let a429 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h428, _, _, _, _, _, _) => {
                                h428
                            }
                        };
                        let a430 = v13;
                        nat_eq(a429, a430)
                    } else {
                        false
                    } {
                        let a431 = 0u64;
                        let a432 = v16;
                        nat_lt(a431, a432)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a433 = 0u64;
                let a434 = v14;
                nat_lt(a433, a434)
            } else {
                false
            } {
                let a435 = 0u64;
                let a436 = v15;
                nat_lt(a435, a436)
            } else {
                false
            } {
                f20(v2.clone(), v4.clone())?
            } else {
                false
            } {
                f20(v5.clone(), v4.clone())?
            } else {
                false
            } {
                Ok(Adt10::C6(Adt8::C0(v15, v16, v14, {
                    let a437 = v16;
                    let a438 = v14;
                    nat_mul(a437, a438)?
                }, v17, {
                    let a439 = v17;
                    let a440 = v16;
                    nat_mul(a439, a440)?
                }, {
                    let a441 = v17;
                    let a442 = v14;
                    nat_mul(a441, a442)?
                }, 0u64, f23(v2, 1u64), f23(v5, 1u64), f23(v4, 1u64), v6, v7, v8, if v0 {
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

pub fn f6(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m443 = v2;
    match m443 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let m444 = v3;
            match m444 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v6) => {
                    let a446 = match v1.clone() {
                        Adt2::C0(h445, _, _, _, _, _, _, _, _, _, _) => {
                            h445
                        }
                    };
                    let a447 = 0u64;
                    if if if if if if if if if if if if if if if nat_eq(a446, a447) {
                        let a449 = match v5.clone() {
                            Adt2::C0(h448, _, _, _, _, _, _, _, _, _, _) => {
                                h448
                            }
                        };
                        let a450 = 0u64;
                        nat_eq(a449, a450)
                    } else {
                        false
                    } {
                        let a452 = match v6.clone() {
                            Adt2::C0(h451, _, _, _, _, _, _, _, _, _, _) => {
                                h451
                            }
                        };
                        let a453 = 0u64;
                        nat_eq(a452, a453)
                    } else {
                        false
                    } {
                        let a455 = match v4.clone() {
                            Adt2::C0(h454, _, _, _, _, _, _, _, _, _, _) => {
                                h454
                            }
                        };
                        let a456 = 0u64;
                        nat_eq(a455, a456)
                    } else {
                        false
                    } {
                        f24(v5.clone(), v6.clone())
                    } else {
                        false
                    } {
                        f24(v1.clone(), v4.clone())
                    } else {
                        false
                    } {
                        f30(v1.clone())?
                    } else {
                        false
                    } {
                        f30(v5.clone())?
                    } else {
                        false
                    } {
                        f30(v6)?
                    } else {
                        false
                    } {
                        f30(v4.clone())?
                    } else {
                        false
                    } {
                        let a462 = match v1.clone() {
                            Adt2::C0(_, h457, _, _, _, _, _, _, _, _, _) => {
                                h457
                            }
                        };
                        let a463 = {
                            let a460 = match v5.clone() {
                                Adt2::C0(_, h458, _, _, _, _, _, _, _, _, _) => {
                                    h458
                                }
                            };
                            let a461 = match v5.clone() {
                                Adt2::C0(_, h459, _, _, _, _, _, _, _, _, _) => {
                                    h459
                                }
                            };
                            nat_mul(a460, a461)?
                        };
                        nat_eq(a462, a463)
                    } else {
                        false
                    } {
                        let a469 = match v1.clone() {
                            Adt2::C0(_, _, h464, _, _, _, _, _, _, _, _) => {
                                h464
                            }
                        };
                        let a470 = {
                            let a467 = match v5.clone() {
                                Adt2::C0(_, _, h465, _, _, _, _, _, _, _, _) => {
                                    h465
                                }
                            };
                            let a468 = match v5.clone() {
                                Adt2::C0(_, _, _, h466, _, _, _, _, _, _, _) => {
                                    h466
                                }
                            };
                            nat_mul(a467, a468)?
                        };
                        nat_eq(a469, a470)
                    } else {
                        false
                    } {
                        let a473 = match v1.clone() {
                            Adt2::C0(_, _, _, h471, _, _, _, _, _, _, _) => {
                                h471
                            }
                        };
                        let a474 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h472, _, _, _, _, _, _) => {
                                h472
                            }
                        };
                        nat_eq(a473, a474)
                    } else {
                        false
                    } {
                        let a476 = match v0.clone() {
                            Adt1::C0(h475, _, _, _, _, _, _, _, _, _, _, _) => {
                                h475
                            }
                        };
                        let a477 = 4294967296u64;
                        nat_lt(a476, a477)
                    } else {
                        false
                    } {
                        let a479 = match v0.clone() {
                            Adt1::C0(h478, _, _, _, _, _, _, _, _, _, _, _) => {
                                h478
                            }
                        };
                        let a480 = 0u64;
                        if nat_eq(a479, a480) {
                            if f20(v1.clone(), v4.clone())? {
                                true
                            } else {
                                let a483 = match v1.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h481) => {
                                        h481
                                    }
                                };
                                let a484 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h482) => {
                                        h482
                                    }
                                };
                                nat_eq(a483, a484)
                            }
                        } else {
                            let a487 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h485) => {
                                    h485
                                }
                            };
                            let a488 = match v4.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h486) => {
                                    h486
                                }
                            };
                            nat_eq(a487, a488)
                        }
                    } else {
                        false
                    } {
                        let v7: bool = {
                            let a490 = match v4.clone() {
                                Adt2::C0(_, h489, _, _, _, _, _, _, _, _, _) => {
                                    h489
                                }
                            };
                            let a491 = 0u64;
                            if if if nat_eq(a490, a491) {
                                true
                            } else {
                                let a493 = match v4.clone() {
                                    Adt2::C0(_, _, h492, _, _, _, _, _, _, _, _) => {
                                        h492
                                    }
                                };
                                let a494 = 0u64;
                                nat_eq(a493, a494)
                            } {
                                true
                            } else {
                                let a496 = match v4.clone() {
                                    Adt2::C0(_, _, _, h495, _, _, _, _, _, _, _) => {
                                        h495
                                    }
                                };
                                let a497 = 0u64;
                                nat_eq(a496, a497)
                            } {
                                true
                            } else {
                                let a499 = match v4.clone() {
                                    Adt2::C0(_, _, _, _, h498, _, _, _, _, _, _) => {
                                        h498
                                    }
                                };
                                let a500 = 0u64;
                                nat_eq(a499, a500)
                            }
                        };
                        let v8: u64 = match v5.clone() {
                            Adt2::C0(_, h501, _, _, _, _, _, _, _, _, _) => {
                                h501
                            }
                        };
                        let v9: u64 = if v7 {
                            0u64
                        } else {
                            let a503 = match v0 {
                                Adt1::C0(h502, _, _, _, _, _, _, _, _, _, _, _) => {
                                    h502
                                }
                            };
                            let a504 = 0u64;
                            if nat_eq(a503, a504) {
                                f19(v1.clone(), 1u64)?
                            } else {
                                0u64
                            }
                        };
                        let v10: u64 = if v7 {
                            0u64
                        } else {
                            f32(v8, match v5.clone() {
                                Adt2::C0(_, _, h505, _, _, _, _, _, _, _, _) => {
                                    h505
                                }
                            }, match v5.clone() {
                                Adt2::C0(_, _, _, h506, _, _, _, _, _, _, _) => {
                                    h506
                                }
                            }, match v5 {
                                Adt2::C0(_, _, _, _, h507, _, _, _, _, _, _) => {
                                    h507
                                }
                            })?
                        };
                        Ok(Adt10::C4(f33(v9, v8, v10)?, 4u64, f23(v1, 1u64), f23(v4, 1u64), Adt4::C25(v9, v8, v10)))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f7(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let v5: Adt14 = f34(v2.clone());
    let v6: Adt14 = f34(v4.clone());
    let v8: Adt15 = {
        let m508 = v3;
        match m508 {
            None => {
                f35(v0, match v1.clone() {
                    Adt1::C0(h509, _, _, _, _, _, _, _, _, _, _, _) => {
                        h509
                    }
                }, match v1 {
                    Adt1::C0(_, h510, _, _, _, _, _, _, _, _, _, _) => {
                        h510
                    }
                }, false, v5.clone(), v5, v6)?
            }
            Some(v7) => {
                f35(v0, match v1.clone() {
                    Adt1::C0(h511, _, _, _, _, _, _, _, _, _, _, _) => {
                        h511
                    }
                }, match v1 {
                    Adt1::C0(_, h512, _, _, _, _, _, _, _, _, _, _) => {
                        h512
                    }
                }, true, v5, f34(v7), v6)?
            }
        }
    };
    let m513 = v8;
    match m513 {
        Adt15::C0 => {
            Ok(Adt10::C1)
        }
        Adt15::C1(v9, v10, v11, v12, v13, v14) => {
            Ok(Adt10::C4(v9, v10, f23(v2, 1u64), f23(v4, 1u64), Adt4::C26(v11, v14, v12, v13)))
        }
    }
}

pub fn f8(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h514, _, _, _, _, _, _, _, _, _, _) => {
            h514
        }
    };
    let v4: u64 = f36(v3);
    let a516 = match v1.clone() {
        Adt2::C0(h515, _, _, _, _, _, _, _, _, _, _) => {
            h515
        }
    };
    let a517 = 0u64;
    if if if if if if if if nat_eq(a516, a517) {
        true
    } else {
        let a519 = match v1.clone() {
            Adt2::C0(h518, _, _, _, _, _, _, _, _, _, _) => {
                h518
            }
        };
        let a520 = 30u64;
        nat_eq(a519, a520)
    } {
        let a523 = match v1.clone() {
            Adt2::C0(h521, _, _, _, _, _, _, _, _, _, _) => {
                h521
            }
        };
        let a524 = match v2.clone() {
            Adt2::C0(h522, _, _, _, _, _, _, _, _, _, _) => {
                h522
            }
        };
        nat_eq(a523, a524)
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        f37(match v0.clone() {
            Adt1::C0(h525, _, _, _, _, _, _, _, _, _, _, _) => {
                h525
            }
        })
    } else {
        false
    } {
        f38(v1.clone(), v4)
    } else {
        false
    } {
        f38(v2.clone(), v4)
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f39(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v5: u64 = if f40(match v1.clone() {
            Adt2::C0(_, h526, _, _, _, _, _, _, _, _, _) => {
                h526
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h527, _, _, _, _, _, _, _, _) => {
                h527
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h528, _, _, _, _, _, _, _) => {
                h528
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h529, _, _, _, _, _, _) => {
                h529
            }
        }) {
            0u64
        } else {
            let a535 = match v1.clone() {
                Adt2::C0(_, _, h530, _, _, _, _, _, _, _, _) => {
                    h530
                }
            };
            let a536 = {
                let a533 = match v1.clone() {
                    Adt2::C0(_, _, _, h531, _, _, _, _, _, _, _) => {
                        h531
                    }
                };
                let a534 = match v1.clone() {
                    Adt2::C0(_, _, _, _, h532, _, _, _, _, _, _) => {
                        h532
                    }
                };
                nat_mul(a533, a534)?
            };
            nat_mul(a535, a536)?
        };
        let v6: u64 = {
            let a537 = v5;
            let a538 = 0u64;
            if if nat_eq(a537, a538) {
                true
            } else {
                f41(v2.clone(), v4)?
            } {
                0u64
            } else {
                let a540 = match v2.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h539, _) => {
                        h539
                    }
                };
                let a541 = v4;
                let a542 = 0u64;
                nat_quot(a540, a541, a542)
            }
        };
        Ok(Adt10::C4(f42(v3, match v1.clone() {
            Adt2::C0(_, h543, _, _, _, _, _, _, _, _, _) => {
                h543
            }
        }, v5, v6)?, v4, f23(v1.clone(), 1u64), f23(v2, 1u64), Adt4::C26(v3, match v0 {
            Adt1::C0(h544, _, _, _, _, _, _, _, _, _, _, _) => {
                h544
            }
        }, v5, f43(match v1 {
            Adt2::C0(_, h545, _, _, _, _, _, _, _, _, _) => {
                h545
            }
        }, v5, v6)?)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f9(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a547 = match v1.clone() {
        Adt2::C0(h546, _, _, _, _, _, _, _, _, _, _) => {
            h546
        }
    };
    let a548 = 0u64;
    if if if if if nat_eq(a547, a548) {
        let a550 = match v2.clone() {
            Adt2::C0(h549, _, _, _, _, _, _, _, _, _, _) => {
                h549
            }
        };
        let a551 = 0u64;
        nat_eq(a550, a551)
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        f44(match v0.clone() {
            Adt1::C0(h552, _, _, _, _, _, _, _, _, _, _, _) => {
                h552
            }
        })
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f39(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        let v3: Adt3 = f23(v1, 1u64);
        let v4: u64 = {
            let a558 = match v3.clone() {
                Adt3::C0(_, h553, _, _, _, _, _, _) => {
                    h553
                }
            };
            let a559 = {
                let a556 = match v3.clone() {
                    Adt3::C0(_, _, h554, _, _, _, _, _) => {
                        h554
                    }
                };
                let a557 = match v3.clone() {
                    Adt3::C0(_, _, _, h555, _, _, _, _) => {
                        h555
                    }
                };
                nat_mul(a556, a557)?
            };
            nat_mul(a558, a559)?
        };
        Ok(Adt10::C4(f45(match v3.clone() {
            Adt3::C0(h560, _, _, _, _, _, _, _) => {
                h560
            }
        }, v4)?, 4u64, v3, f23(v2, 1u64), Adt4::C24(match v0 {
            Adt1::C0(h561, _, _, _, _, _, _, _, _, _, _, _) => {
                h561
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f10(v0: Adt2, v1: Option<Adt2>, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m562 = v1;
    match m562 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let m563 = v2;
            match m563 {
                None => {
                    Ok(Adt10::C1)
                }
                Some(v5) => {
                    let a565 = match v0.clone() {
                        Adt2::C0(h564, _, _, _, _, _, _, _, _, _, _) => {
                            h564
                        }
                    };
                    let a566 = 0u64;
                    if if if if if if if if if if if if if if if if if nat_eq(a565, a566) {
                        let a568 = match v4.clone() {
                            Adt2::C0(h567, _, _, _, _, _, _, _, _, _, _) => {
                                h567
                            }
                        };
                        let a569 = 0u64;
                        nat_eq(a568, a569)
                    } else {
                        false
                    } {
                        let a571 = match v5.clone() {
                            Adt2::C0(h570, _, _, _, _, _, _, _, _, _, _) => {
                                h570
                            }
                        };
                        let a572 = 26u64;
                        nat_eq(a571, a572)
                    } else {
                        false
                    } {
                        let a574 = match v3.clone() {
                            Adt2::C0(h573, _, _, _, _, _, _, _, _, _, _) => {
                                h573
                            }
                        };
                        let a575 = 0u64;
                        nat_eq(a574, a575)
                    } else {
                        false
                    } {
                        f24(v0.clone(), v3.clone())
                    } else {
                        false
                    } {
                        let a578 = match v0.clone() {
                            Adt2::C0(_, h576, _, _, _, _, _, _, _, _, _) => {
                                h576
                            }
                        };
                        let a579 = match v4.clone() {
                            Adt2::C0(_, h577, _, _, _, _, _, _, _, _, _) => {
                                h577
                            }
                        };
                        nat_eq(a578, a579)
                    } else {
                        false
                    } {
                        let a582 = match v0.clone() {
                            Adt2::C0(_, _, h580, _, _, _, _, _, _, _, _) => {
                                h580
                            }
                        };
                        let a583 = match v5.clone() {
                            Adt2::C0(_, h581, _, _, _, _, _, _, _, _, _) => {
                                h581
                            }
                        };
                        nat_eq(a582, a583)
                    } else {
                        false
                    } {
                        let a586 = match v0.clone() {
                            Adt2::C0(_, _, _, h584, _, _, _, _, _, _, _) => {
                                h584
                            }
                        };
                        let a587 = match v5.clone() {
                            Adt2::C0(_, _, h585, _, _, _, _, _, _, _, _) => {
                                h585
                            }
                        };
                        nat_eq(a586, a587)
                    } else {
                        false
                    } {
                        let a589 = 0u64;
                        let a590 = match v5.clone() {
                            Adt2::C0(_, _, _, h588, _, _, _, _, _, _, _) => {
                                h588
                            }
                        };
                        nat_lt(a589, a590)
                    } else {
                        false
                    } {
                        let a592 = 0u64;
                        let a593 = match v5.clone() {
                            Adt2::C0(_, _, _, _, h591, _, _, _, _, _, _) => {
                                h591
                            }
                        };
                        nat_lt(a592, a593)
                    } else {
                        false
                    } {
                        let a595 = 4u64;
                        let a596 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, h594, _, _, _, _, _) => {
                                h594
                            }
                        };
                        nat_le(a595, a596)
                    } else {
                        false
                    } {
                        let a598 = 4u64;
                        let a599 = match v4.clone() {
                            Adt2::C0(_, _, _, _, _, h597, _, _, _, _, _) => {
                                h597
                            }
                        };
                        nat_le(a598, a599)
                    } else {
                        false
                    } {
                        let a601 = 4u64;
                        let a602 = match v5.clone() {
                            Adt2::C0(_, _, _, _, _, h600, _, _, _, _, _) => {
                                h600
                            }
                        };
                        nat_le(a601, a602)
                    } else {
                        false
                    } {
                        let a604 = 4u64;
                        let a605 = match v3.clone() {
                            Adt2::C0(_, _, _, _, _, h603, _, _, _, _, _) => {
                                h603
                            }
                        };
                        nat_le(a604, a605)
                    } else {
                        false
                    } {
                        f20(v0.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f20(v4.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        f20(v5.clone(), v3.clone())?
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f46(match v0.clone() {
                            Adt2::C0(_, h606, _, _, _, _, _, _, _, _, _) => {
                                h606
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, h607, _, _, _, _, _, _, _, _) => {
                                h607
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, h608, _, _, _, _, _, _, _) => {
                                h608
                            }
                        }, match v0.clone() {
                            Adt2::C0(_, _, _, _, h609, _, _, _, _, _, _) => {
                                h609
                            }
                        })?, 4u64, f23(v0, 1u64), f23(v3, 1u64), Adt4::C22(f23(v4, 1u64), f23(v5, 1u64))))
                    } else {
                        Ok(Adt10::C1)
                    }
                }
            }
        }
    }
}

pub fn f11(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let a610 = v0;
    let a611 = 28u64;
    if if if nat_eq(a610, a611) {
        true
    } else {
        let a612 = v0;
        let a613 = 29u64;
        nat_eq(a612, a613)
    } {
        let a615 = 0u64;
        let a616 = match v1.clone() {
            Adt1::C0(h614, _, _, _, _, _, _, _, _, _, _, _) => {
                h614
            }
        };
        if nat_lt(a615, a616) {
            true
        } else {
            let a618 = 0u64;
            let a619 = match v1.clone() {
                Adt1::C0(_, _, _, h617, _, _, _, _, _, _, _, _) => {
                    h617
                }
            };
            nat_lt(a618, a619)
        }
    } else {
        false
    } {
        let m623 = f47(match v2.clone() {
            Adt2::C0(h620, _, _, _, _, _, _, _, _, _, _) => {
                h620
            }
        }, match v1.clone() {
            Adt1::C0(h621, _, _, _, _, _, _, _, _, _, _, _) => {
                h621
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, h622, _, _, _, _, _, _, _, _) => {
                h622
            }
        });
        match m623 {
            None => {
                Ok(Adt9::C0(Adt10::C8))
            }
            Some(v5) => {
                if v5 {
                    Ok(Adt9::C0(Adt10::C7))
                } else {
                    f48(v0, v1, v2, v3, v4)
                }
            }
        }
    } else {
        f48(v0, v1, v2, v3, v4)
    }
}

pub fn f12(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let v4: u64 = match v0.clone() {
        Adt8::C0(_, h624, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h624
        }
    };
    let v5: u64 = match v0.clone() {
        Adt8::C0(_, _, h625, _, _, _, _, _, _, _, _, _, _, _, _) => {
            h625
        }
    };
    let v6: u64 = {
        let a626 = v3;
        let a627 = v5;
        let a628 = 0u64;
        nat_rem(a626, a627, a628)
    };
    let a629 = v1;
    let a630 = 0u64;
    if nat_eq(a629, a630) {
        let a632 = v3;
        let a633 = match v0.clone() {
            Adt8::C0(_, _, _, h631, _, _, _, _, _, _, _, _, _, _, _) => {
                h631
            }
        };
        if nat_lt(a632, a633) {
            Ok(Adt9::C8(Some(f49(match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h634, _, _, _, _, _, _) => {
                    h634
                }
            }, {
                let a638 = v6;
                let a639 = {
                    let m637 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h636, _, _, _, _, _, _) => {
                            h636
                        }
                    };
                    match m637 {
                        Adt3::C0(h635, _, _, _, _, _, _, _) => {
                            h635
                        }
                    }
                };
                let a640 = 0u64;
                nat_rem(a638, a639, a640)
            }, {
                let a650 = {
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
                    nat_quot(a644, a645, a646)
                };
                let a651 = {
                    let m649 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, h648, _, _, _, _, _, _) => {
                            h648
                        }
                    };
                    match m649 {
                        Adt3::C0(_, h647, _, _, _, _, _, _) => {
                            h647
                        }
                    }
                };
                let a652 = 0u64;
                nat_rem(a650, a651, a652)
            }, {
                let a662 = {
                    let a656 = v6;
                    let a657 = {
                        let m655 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, h654, _, _, _, _, _, _) => {
                                h654
                            }
                        };
                        match m655 {
                            Adt3::C0(h653, _, _, _, _, _, _, _) => {
                                h653
                            }
                        }
                    };
                    let a658 = 0u64;
                    nat_quot(a656, a657, a658)
                };
                let a663 = {
                    let m661 = match v0 {
                        Adt8::C0(_, _, _, _, _, _, _, _, h660, _, _, _, _, _, _) => {
                            h660
                        }
                    };
                    match m661 {
                        Adt3::C0(_, h659, _, _, _, _, _, _) => {
                            h659
                        }
                    }
                };
                let a664 = 0u64;
                nat_quot(a662, a663, a664)
            }, {
                let a665 = v3;
                let a666 = v5;
                let a667 = 0u64;
                nat_quot(a665, a666, a667)
            })?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v7: u64 = {
            let a673 = v2;
            let a674 = {
                let a670 = v3;
                let a671 = {
                    let a668 = v1;
                    let a669 = 1u64;
                    if nat_eq(a668, a669) {
                        v4
                    } else {
                        v5
                    }
                };
                let a672 = 0u64;
                nat_quot(a670, a671, a672)
            };
            nat_add(a673, a674)?
        };
        let a676 = v7;
        let a677 = match v0.clone() {
            Adt8::C0(h675, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h675
            }
        };
        if nat_lt(a676, a677) {
            let v8: u64 = {
                let a681 = v7;
                let a682 = {
                    let m680 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h679, _, _, _, _, _) => {
                            h679
                        }
                    };
                    match m680 {
                        Adt3::C0(h678, _, _, _, _, _, _, _) => {
                            h678
                        }
                    }
                };
                let a683 = 0u64;
                nat_rem(a681, a682, a683)
            };
            let v9: u64 = {
                let a687 = v7;
                let a688 = {
                    let m686 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h685, _, _, _, _, _) => {
                            h685
                        }
                    };
                    match m686 {
                        Adt3::C0(h684, _, _, _, _, _, _, _) => {
                            h684
                        }
                    }
                };
                let a689 = 0u64;
                nat_quot(a687, a688, a689)
            };
            let v10: u64 = {
                let a693 = v9;
                let a694 = {
                    let m692 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h691, _, _, _, _, _) => {
                            h691
                        }
                    };
                    match m692 {
                        Adt3::C0(_, h690, _, _, _, _, _, _) => {
                            h690
                        }
                    }
                };
                let a695 = 0u64;
                nat_rem(a693, a694, a695)
            };
            let v11: u64 = {
                let a699 = v9;
                let a700 = {
                    let m698 = match v0.clone() {
                        Adt8::C0(_, _, _, _, _, _, _, _, _, h697, _, _, _, _, _) => {
                            h697
                        }
                    };
                    match m698 {
                        Adt3::C0(_, h696, _, _, _, _, _, _) => {
                            h696
                        }
                    }
                };
                let a701 = 0u64;
                nat_quot(a699, a700, a701)
            };
            let a702 = v1;
            let a703 = 1u64;
            if nat_eq(a702, a703) {
                Ok(Adt9::C8(Some(f49(match v0 {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h704, _, _, _, _, _) => {
                        h704
                    }
                }, v8, v10, {
                    let a705 = v3;
                    let a706 = v4;
                    let a707 = 0u64;
                    nat_rem(a705, a706, a707)
                }, v11)?)))
            } else {
                let a708 = v1;
                let a709 = 2u64;
                if nat_eq(a708, a709) {
                    Ok(Adt9::C8(Some({
                        let a755 = f49(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h710, _, _, _, _) => {
                                h710
                            }
                        }, {
                            let a722 = {
                                let a714 = v8;
                                let a715 = {
                                    let m713 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h712, _, _, _) => {
                                            h712
                                        }
                                    };
                                    match m713 {
                                        Adt6::C0(_, h711, _, _) => {
                                            h711
                                        }
                                    }
                                };
                                nat_mul(a714, a715)?
                            };
                            let a723 = {
                                let a719 = v6;
                                let a720 = {
                                    let m718 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h717, _, _, _, _, _, _) => {
                                            h717
                                        }
                                    };
                                    match m718 {
                                        Adt3::C0(h716, _, _, _, _, _, _, _) => {
                                            h716
                                        }
                                    }
                                };
                                let a721 = 0u64;
                                nat_rem(a719, a720, a721)
                            };
                            nat_add(a722, a723)?
                        }, {
                            let a741 = {
                                let a727 = v10;
                                let a728 = {
                                    let m726 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h725, _, _, _) => {
                                            h725
                                        }
                                    };
                                    match m726 {
                                        Adt6::C0(_, h724, _, _) => {
                                            h724
                                        }
                                    }
                                };
                                nat_mul(a727, a728)?
                            };
                            let a742 = {
                                let a738 = {
                                    let a732 = v6;
                                    let a733 = {
                                        let m731 = match v0.clone() {
                                            Adt8::C0(_, _, _, _, _, _, _, _, h730, _, _, _, _, _, _) => {
                                                h730
                                            }
                                        };
                                        match m731 {
                                            Adt3::C0(h729, _, _, _, _, _, _, _) => {
                                                h729
                                            }
                                        }
                                    };
                                    let a734 = 0u64;
                                    nat_quot(a732, a733, a734)
                                };
                                let a739 = {
                                    let m737 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h736, _, _, _, _, _, _) => {
                                            h736
                                        }
                                    };
                                    match m737 {
                                        Adt3::C0(_, h735, _, _, _, _, _, _) => {
                                            h735
                                        }
                                    }
                                };
                                let a740 = 0u64;
                                nat_rem(a738, a739, a740)
                            };
                            nat_add(a741, a742)?
                        }, {
                            let a752 = {
                                let a746 = v6;
                                let a747 = {
                                    let m745 = match v0.clone() {
                                        Adt8::C0(_, _, _, _, _, _, _, _, h744, _, _, _, _, _, _) => {
                                            h744
                                        }
                                    };
                                    match m745 {
                                        Adt3::C0(h743, _, _, _, _, _, _, _) => {
                                            h743
                                        }
                                    }
                                };
                                let a748 = 0u64;
                                nat_quot(a746, a747, a748)
                            };
                            let a753 = {
                                let m751 = match v0 {
                                    Adt8::C0(_, _, _, _, _, _, _, _, h750, _, _, _, _, _, _) => {
                                        h750
                                    }
                                };
                                match m751 {
                                    Adt3::C0(_, h749, _, _, _, _, _, _) => {
                                        h749
                                    }
                                }
                            };
                            let a754 = 0u64;
                            nat_quot(a752, a753, a754)
                        }, v11)?;
                        let a756 = 4u64;
                        let a757 = 0u64;
                        nat_quot(a755, a756, a757)
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

pub fn f13(v0: Adt8, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a758 = v1;
    let a759 = 0u64;
    if nat_eq(a758, a759) {
        let a761 = v3;
        let a762 = match v0.clone() {
            Adt8::C0(_, _, _, h760, _, _, _, _, _, _, _, _, _, _, _) => {
                h760
            }
        };
        if nat_lt(a761, a762) {
            let v4: Adt3 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, h763, _, _, _, _, _, _) => {
                    h763
                }
            };
            let v5: u64 = {
                let a765 = v3;
                let a766 = match v0.clone() {
                    Adt8::C0(_, _, h764, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h764
                    }
                };
                let a767 = 0u64;
                nat_quot(a765, a766, a767)
            };
            Ok(Adt9::C8(Some(f49(v4.clone(), f50(v5, v4.clone(), 0u64), f50(v5, v4.clone(), 1u64), f50(v5, v4, 2u64), f51(v0.clone(), v5, {
                let a769 = v3;
                let a770 = match v0 {
                    Adt8::C0(_, _, h768, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h768
                    }
                };
                let a771 = 0u64;
                nat_rem(a769, a770, a771)
            })?)?)))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    } else {
        let v6: Adt3 = match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, h772, _, _, _, _) => {
                h772
            }
        };
        let v7: u64 = {
            let a773 = v1;
            let a774 = 1u64;
            if nat_eq(a773, a774) {
                match v0.clone() {
                    Adt8::C0(_, h775, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h775
                    }
                }
            } else {
                match v0.clone() {
                    Adt8::C0(_, _, h776, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        h776
                    }
                }
            }
        };
        let v8: u64 = {
            let a780 = v2;
            let a781 = {
                let a777 = v3;
                let a778 = v7;
                let a779 = 0u64;
                nat_quot(a777, a778, a779)
            };
            nat_add(a780, a781)?
        };
        let v9: u64 = {
            let a782 = v3;
            let a783 = v7;
            let a784 = 0u64;
            nat_rem(a782, a783, a784)
        };
        let v10: u64 = {
            let a786 = v8;
            let a787 = match v6.clone() {
                Adt3::C0(h785, _, _, _, _, _, _, _) => {
                    h785
                }
            };
            let a788 = 0u64;
            nat_rem(a786, a787, a788)
        };
        let v11: u64 = {
            let a790 = v8;
            let a791 = match v6.clone() {
                Adt3::C0(h789, _, _, _, _, _, _, _) => {
                    h789
                }
            };
            let a792 = 0u64;
            nat_quot(a790, a791, a792)
        };
        let v12: u64 = {
            let a794 = v11;
            let a795 = match v6.clone() {
                Adt3::C0(_, h793, _, _, _, _, _, _) => {
                    h793
                }
            };
            let a796 = 0u64;
            nat_rem(a794, a795, a796)
        };
        let v13: u64 = {
            let a798 = v11;
            let a799 = match v6.clone() {
                Adt3::C0(_, h797, _, _, _, _, _, _) => {
                    h797
                }
            };
            let a800 = 0u64;
            nat_quot(a798, a799, a800)
        };
        let a802 = v8;
        let a803 = match v0.clone() {
            Adt8::C0(h801, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                h801
            }
        };
        if nat_lt(a802, a803) {
            let a804 = v1;
            let a805 = 1u64;
            if nat_eq(a804, a805) {
                let v14: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, h806, _, _, _, _, _, _) => {
                        h806
                    }
                };
                let v15: Adt3 = match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, h807, _, _, _, _, _) => {
                        h807
                    }
                };
                let m810 = f52(match v0.clone() {
                    Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, h808, _, _, _) => {
                        h808
                    }
                }, match v15.clone() {
                    Adt3::C0(h809, _, _, _, _, _, _, _) => {
                        h809
                    }
                }, v10, f50(v9, v14.clone(), 0u64))?;
                match m810 {
                    None => {
                        Ok(Adt9::C8(None::<u64>))
                    }
                    Some(v16) => {
                        let m813 = f52(match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, h811, _, _) => {
                                h811
                            }
                        }, match v15 {
                            Adt3::C0(_, h812, _, _, _, _, _, _) => {
                                h812
                            }
                        }, v12, f50(v9, v14, 1u64))?;
                        match m813 {
                            None => {
                                Ok(Adt9::C8(None::<u64>))
                            }
                            Some(v17) => {
                                f53(v0, v9, v13, v16, v17)
                            }
                        }
                    }
                }
            } else {
                let a814 = v1;
                let a815 = 2u64;
                if nat_eq(a814, a815) {
                    Ok(Adt9::C8(Some({
                        let a817 = match v0.clone() {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h816) => {
                                h816
                            }
                        };
                        let a818 = 0u64;
                        if nat_eq(a817, a818) {
                            f49(v6, v10, v12, v9, v13)?
                        } else {
                            f49(v6.clone(), v10, v12, {
                                let a820 = v13;
                                let a821 = match v6.clone() {
                                    Adt3::C0(_, _, h819, _, _, _, _, _) => {
                                        h819
                                    }
                                };
                                let a822 = 0u64;
                                nat_rem(a820, a821, a822)
                            }, {
                                let a830 = {
                                    let a828 = {
                                        let a824 = v13;
                                        let a825 = match v6 {
                                            Adt3::C0(_, _, h823, _, _, _, _, _) => {
                                                h823
                                            }
                                        };
                                        let a826 = 0u64;
                                        nat_quot(a824, a825, a826)
                                    };
                                    let a829 = match v0 {
                                        Adt8::C0(_, _, h827, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            h827
                                        }
                                    };
                                    nat_mul(a828, a829)?
                                };
                                let a831 = v9;
                                nat_add(a830, a831)?
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

pub fn f14(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64) -> R<Adt9> {
    f15(v0, v1, v2, v3, v4, false, 0u64, 0u64, 0u64, 0u64, 0u64)
}

pub fn f15(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64) -> R<Adt9> {
    Ok(f54(v3.clone(), {
        let m832 = v3.clone();
        match m832 {
            Adt4::C0 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C1 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C2(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C3(_, _, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C4(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C5(_, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C6(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C7(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C8(_) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C9(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C10(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C11(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C12(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C13(_, _, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C14(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C15 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C16(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C17 => {
                f64(v0, v1, v2, v4)?
            }
            Adt4::C18(_, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C19(v68, v69) => {
                f65(v68, v0, v1, v2, v69, v4)?
            }
            Adt4::C20(v70, v71) => {
                f66(v70, v0, v1, v2, v71, v4)?
            }
            Adt4::C21 => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C22(v72, v73) => {
                f67(v0, v1, v2, v72, v73, v4)?
            }
            Adt4::C23(v87, v88, v89, v90) => {
                f74(v0, v90, v87, v88, v89, v4)?
            }
            Adt4::C24(v86) => {
                if v5 {
                    f72(Adt16::C0(v86), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                } else {
                    Adt9::C0(Adt10::C5(65u64))
                }
            }
            Adt4::C25(_, _, _) => {
                f55(v0, v1, v2, v3)?
            }
            Adt4::C26(v82, v83, v84, v85) => {
                let a834 = v82;
                let a835 = 64u64;
                if if if nat_eq(a834, a835) {
                    true
                } else {
                    let a836 = v82;
                    let a837 = 65u64;
                    nat_eq(a836, a837)
                } {
                    true
                } else {
                    let a838 = v82;
                    let a839 = 66u64;
                    nat_eq(a838, a839)
                } {
                    if v5 {
                        f72(Adt16::C1(v82, v83, v84, v85), v0, v1, v2, v4, v6, v7, v8, v9, v10)?
                    } else {
                        Adt9::C0(Adt10::C5(64u64))
                    }
                } else {
                    let a840 = v82;
                    let a841 = 0u64;
                    if if nat_eq(a840, a841) {
                        true
                    } else {
                        let a842 = v82;
                        let a843 = 30u64;
                        nat_eq(a842, a843)
                    } {
                        f73(v0, v1, v2, v82, v83, v84, v85, v4)?
                    } else {
                        Adt9::C0(Adt10::C1)
                    }
                }
            }
            Adt4::C27(v81) => {
                f71(v0, v1, v2, v81)?
            }
            Adt4::C28(v77) => {
                f68(v0, v1, v2, v77, v4)?
            }
            Adt4::C29(v78) => {
                f69(v0, v1, v2, v78)?
            }
            Adt4::C30(v79, _) => {
                f70(v0, v1, v2, v79)?
            }
            Adt4::C31(v33, v34) => {
                f56(v0, v1, v2, v33, v34)?
            }
            Adt4::C32(v35) => {
                f57(v0, match v1 {
                    Adt3::C0(h833, _, _, _, _, _, _, _) => {
                        h833
                    }
                }, v35)?
            }
            Adt4::C33(v36) => {
                f58(v0, v1, v36, v2)?
            }
            Adt4::C34(v48, v49, v50, v51) => {
                f63(v0, v1, v48, v2, v49, v50, v51)?
            }
            Adt4::C35(v47) => {
                f62(v0, v47, v1, v2)?
            }
            Adt4::C36(v37, v38, v39) => {
                f59(v0, v1, v37, v2, v38, v39)?
            }
            Adt4::C37(v40, v41, v42) => {
                f60(v0, v1, v40, v41, v42, v2)?
            }
            Adt4::C38(v43, v44, v45, v46) => {
                f61(v0, v1, v43, v44, v2, v45, v46, v4)?
            }
        }
    }))
}

pub fn f16(v0: Adt4) -> R<Adt9> {
    let m844 = v0;
    match m844 {
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
        Adt4::C25(v65, _, v67) => {
            Ok(Adt9::C6({
                let a850 = {
                    let a846 = 0u64;
                    let a847 = v65;
                    if nat_lt(a846, a847) {
                        1u64
                    } else {
                        0u64
                    }
                };
                let a851 = {
                    let a848 = 0u64;
                    let a849 = v67;
                    if nat_lt(a848, a849) {
                        6u64
                    } else {
                        0u64
                    }
                };
                nat_add(a850, a851)?
            }, 0u64, 0u64, false))
        }
        Adt4::C26(v73, _, _, _) => {
            Ok(Adt9::C6({
                let a852 = v73;
                let a853 = 66u64;
                if nat_eq(a852, a853) {
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
        Adt4::C35(_) => {
            Ok(Adt9::C6(1u64, 0u64, 0u64, false))
        }
        Adt4::C36(_, _, _) => {
            Ok(Adt9::C6(3u64, 0u64, 0u64, false))
        }
        Adt4::C37(_, _, v32) => {
            Ok(Adt9::C6({
                let m845 = v32;
                match m845 {
                    None => {
                        7u64
                    }
                    Some(_) => {
                        15u64
                    }
                }
            }, 0u64, 0u64, false))
        }
        Adt4::C38(_, _, _, _) => {
            Ok(Adt9::C6(7u64, 0u64, 0u64, false))
        }
    }
}

pub fn f17(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> Adt9 {
    let a854 = v1;
    let a855 = 5u64;
    if if if if if nat_lt(a854, a855) {
        true
    } else {
        f75(v0, v1, v2, v3, v4)
    } {
        let a856 = v5;
        let a857 = 5u64;
        nat_lt(a856, a857)
    } else {
        false
    } {
        let a858 = v7;
        let a859 = 5u64;
        nat_lt(a858, a859)
    } else {
        false
    } {
        let a860 = v9;
        let a861 = 2u64;
        if nat_lt(a860, a861) {
            true
        } else {
            let a862 = v9;
            let a863 = 3u64;
            nat_eq(a862, a863)
        }
    } else {
        false
    } {
        Adt9::C7(f76(v0, v1, v2, v3, v4), f77(v5, v6), f77(v7, v8), f78({
            let a864 = v9;
            let a865 = 0u64;
            if nat_eq(a864, a865) {
                1u64
            } else {
                v9
            }
        }, v10), v11, v12)
    } else {
        Adt9::C0(Adt10::C1)
    }
}

pub fn f18(v0: Adt2, v1: u64) -> R<bool> {
    let a866 = v1;
    let a867 = 0u64;
    if nat_eq(a866, a867) {
        Ok(true)
    } else {
        let a873 = {
            let a870 = f79({
                let a868 = v1;
                let a869 = 1u64;
                nat_sub(a868, a869)
            }, f23(v0.clone(), 1u64))?;
            let a871 = 4u64;
            nat_add(a870, a871)?
        };
        let a874 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h872, _) => {
                h872
            }
        };
        Ok(nat_le(a873, a874))
    }
}

pub fn f19(v0: Adt2, v1: u64) -> R<u64> {
    let a886 = {
        let a880 = {
            let a876 = match v0.clone() {
                Adt2::C0(_, h875, _, _, _, _, _, _, _, _, _) => {
                    h875
                }
            };
            let a877 = v1;
            let a878 = 0u64;
            nat_quot(a876, a877, a878)
        };
        let a881 = match v0.clone() {
            Adt2::C0(_, _, h879, _, _, _, _, _, _, _, _) => {
                h879
            }
        };
        nat_mul(a880, a881)?
    };
    let a887 = {
        let a884 = match v0.clone() {
            Adt2::C0(_, _, _, h882, _, _, _, _, _, _, _) => {
                h882
            }
        };
        let a885 = match v0 {
            Adt2::C0(_, _, _, _, h883, _, _, _, _, _, _) => {
                h883
            }
        };
        nat_mul(a884, a885)?
    };
    nat_mul(a886, a887)
}

pub fn f20(v0: Adt2, v1: Adt2) -> R<bool> {
    let a889 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h888, _) => {
            h888
        }
    };
    let a890 = 0u64;
    if nat_eq(a889, a890) {
        Ok(true)
    } else {
        let a892 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, h891, _) => {
                h891
            }
        };
        let a893 = 0u64;
        if nat_eq(a892, a893) {
            Ok(true)
        } else {
            let a895 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h894) => {
                    h894
                }
            };
            let a896 = 0u64;
            if nat_eq(a895, a896) {
                Ok(true)
            } else {
                let a898 = match v1.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, _, h897) => {
                        h897
                    }
                };
                let a899 = 0u64;
                if nat_eq(a898, a899) {
                    Ok(true)
                } else {
                    let a905 = {
                        let a902 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h900) => {
                                h900
                            }
                        };
                        let a903 = match v0.clone() {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, h901, _) => {
                                h901
                            }
                        };
                        nat_add(a902, a903)?
                    };
                    let a906 = match v1.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h904) => {
                            h904
                        }
                    };
                    if nat_le(a905, a906) {
                        Ok(true)
                    } else {
                        let a912 = {
                            let a909 = match v1.clone() {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h907) => {
                                    h907
                                }
                            };
                            let a910 = match v1 {
                                Adt2::C0(_, _, _, _, _, _, _, _, _, h908, _) => {
                                    h908
                                }
                            };
                            nat_add(a909, a910)?
                        };
                        let a913 = match v0 {
                            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h911) => {
                                h911
                            }
                        };
                        Ok(nat_le(a912, a913))
                    }
                }
            }
        }
    }
}

pub fn f21(v0: Adt2) -> R<bool> {
    let v1: u64 = {
        let a919 = match v0.clone() {
            Adt2::C0(_, _, h914, _, _, _, _, _, _, _, _) => {
                h914
            }
        };
        let a920 = {
            let a917 = match v0.clone() {
                Adt2::C0(_, _, _, h915, _, _, _, _, _, _, _) => {
                    h915
                }
            };
            let a918 = match v0.clone() {
                Adt2::C0(_, _, _, _, h916, _, _, _, _, _, _) => {
                    h916
                }
            };
            nat_mul(a917, a918)?
        };
        nat_mul(a919, a920)?
    };
    let v2: u64 = f19(v0.clone(), 1u64)?;
    let a922 = match v0.clone() {
        Adt2::C0(h921, _, _, _, _, _, _, _, _, _, _) => {
            h921
        }
    };
    let a923 = 0u64;
    if if if if if if if nat_eq(a922, a923) {
        let a925 = match v0.clone() {
            Adt2::C0(_, h924, _, _, _, _, _, _, _, _, _) => {
                h924
            }
        };
        let a926 = 2147483647u64;
        nat_le(a925, a926)
    } else {
        false
    } {
        let a927 = v1;
        let a928 = 2147483647u64;
        nat_le(a927, a928)
    } else {
        false
    } {
        let a930 = match v0.clone() {
            Adt2::C0(_, h929, _, _, _, _, _, _, _, _, _) => {
                h929
            }
        };
        let a931 = 1u64;
        if nat_eq(a930, a931) {
            true
        } else {
            let a933 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h932, _, _, _, _, _) => {
                    h932
                }
            };
            let a934 = 4u64;
            nat_eq(a933, a934)
        }
    } else {
        false
    } {
        let a935 = v1;
        let a936 = 1u64;
        if nat_le(a935, a936) {
            true
        } else {
            let a941 = {
                let a938 = match v0.clone() {
                    Adt2::C0(_, h937, _, _, _, _, _, _, _, _, _) => {
                        h937
                    }
                };
                let a939 = 4u64;
                nat_mul(a938, a939)?
            };
            let a942 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h940, _, _, _, _) => {
                    h940
                }
            };
            nat_le(a941, a942)
        }
    } else {
        false
    } {
        let a944 = match v0.clone() {
            Adt2::C0(_, _, _, h943, _, _, _, _, _, _, _) => {
                h943
            }
        };
        let a945 = 1u64;
        if nat_eq(a944, a945) {
            true
        } else {
            let a951 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h946, _, _, _) => {
                    h946
                }
            };
            let a952 = {
                let a949 = match v0.clone() {
                    Adt2::C0(_, _, h947, _, _, _, _, _, _, _, _) => {
                        h947
                    }
                };
                let a950 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h948, _, _, _, _) => {
                        h948
                    }
                };
                nat_mul(a949, a950)?
            };
            nat_eq(a951, a952)
        }
    } else {
        false
    } {
        let a954 = match v0.clone() {
            Adt2::C0(_, _, _, _, h953, _, _, _, _, _, _) => {
                h953
            }
        };
        let a955 = 1u64;
        if nat_eq(a954, a955) {
            true
        } else {
            let a964 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h956, _, _) => {
                    h956
                }
            };
            let a965 = {
                let a962 = match v0.clone() {
                    Adt2::C0(_, _, _, h957, _, _, _, _, _, _, _) => {
                        h957
                    }
                };
                let a963 = {
                    let a960 = match v0.clone() {
                        Adt2::C0(_, _, h958, _, _, _, _, _, _, _, _) => {
                            h958
                        }
                    };
                    let a961 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h959, _, _, _, _) => {
                            h959
                        }
                    };
                    nat_mul(a960, a961)?
                };
                nat_mul(a962, a963)?
            };
            nat_eq(a964, a965)
        }
    } else {
        false
    } {
        let a966 = v2;
        let a967 = 0u64;
        if nat_eq(a966, a967) {
            Ok(true)
        } else {
            let a973 = {
                let a970 = f79({
                    let a968 = v2;
                    let a969 = 1u64;
                    nat_sub(a968, a969)
                }, f23(v0.clone(), 1u64))?;
                let a971 = 4u64;
                nat_add(a970, a971)?
            };
            let a974 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h972, _) => {
                    h972
                }
            };
            Ok(nat_le(a973, a974))
        }
    } else {
        Ok(false)
    }
}

pub fn f22(v0: u64) -> R<u64> {
    let a981 = 148u64;
    let a982 = {
        let a979 = 36u64;
        let a980 = {
            let a977 = {
                let a975 = 2u64;
                let a976 = v0;
                nat_mul(a975, a976)?
            };
            let a978 = 1u64;
            nat_sub(a977, a978)
        };
        nat_mul(a979, a980)?
    };
    nat_add(a981, a982)
}

pub fn f23(v0: Adt2, v1: u64) -> Adt3 {
    Adt3::C0({
        let a984 = match v0.clone() {
            Adt2::C0(_, h983, _, _, _, _, _, _, _, _, _) => {
                h983
            }
        };
        let a985 = v1;
        let a986 = 0u64;
        nat_quot(a984, a985, a986)
    }, match v0.clone() {
        Adt2::C0(_, _, h987, _, _, _, _, _, _, _, _) => {
            h987
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h988, _, _, _, _, _, _, _) => {
            h988
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h989, _, _, _, _, _, _) => {
            h989
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h990, _, _, _, _, _) => {
            h990
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h991, _, _, _, _) => {
            h991
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h992, _, _, _) => {
            h992
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h993, _, _) => {
            h993
        }
    })
}

pub fn f24(v0: Adt2, v1: Adt2) -> bool {
    let a996 = match v0.clone() {
        Adt2::C0(_, h994, _, _, _, _, _, _, _, _, _) => {
            h994
        }
    };
    let a997 = match v1.clone() {
        Adt2::C0(_, h995, _, _, _, _, _, _, _, _, _) => {
            h995
        }
    };
    if if if nat_eq(a996, a997) {
        let a1000 = match v0.clone() {
            Adt2::C0(_, _, h998, _, _, _, _, _, _, _, _) => {
                h998
            }
        };
        let a1001 = match v1.clone() {
            Adt2::C0(_, _, h999, _, _, _, _, _, _, _, _) => {
                h999
            }
        };
        nat_eq(a1000, a1001)
    } else {
        false
    } {
        let a1004 = match v0.clone() {
            Adt2::C0(_, _, _, h1002, _, _, _, _, _, _, _) => {
                h1002
            }
        };
        let a1005 = match v1.clone() {
            Adt2::C0(_, _, _, h1003, _, _, _, _, _, _, _) => {
                h1003
            }
        };
        nat_eq(a1004, a1005)
    } else {
        false
    } {
        let a1008 = match v0 {
            Adt2::C0(_, _, _, _, h1006, _, _, _, _, _, _) => {
                h1006
            }
        };
        let a1009 = match v1 {
            Adt2::C0(_, _, _, _, h1007, _, _, _, _, _, _) => {
                h1007
            }
        };
        nat_eq(a1008, a1009)
    } else {
        false
    }
}

pub fn f25(v0: u64, v1: Option<Adt3>) -> R<u64> {
    let m1010 = v1;
    match m1010 {
        None => {
            Ok(2u64)
        }
        Some(_) => {
            let a1013 = 1u64;
            let a1014 = {
                let a1011 = 2u64;
                let a1012 = v0;
                nat_mul(a1011, a1012)?
            };
            nat_add(a1013, a1014)
        }
    }
}

pub fn f26(v0: u64) -> bool {
    let a1015 = v0;
    let a1016 = 4294967296u64;
    if nat_lt(a1015, a1016) {
        let a1020 = {
            let a1017 = v0;
            let a1018 = 2147483648u64;
            let a1019 = 0u64;
            nat_rem(a1017, a1018, a1019)
        };
        let a1021 = 2139095040u64;
        nat_lt(a1020, a1021)
    } else {
        false
    }
}

pub fn f27(v0: u64) -> R<u64> {
    let a1022 = 4u64;
    let a1023 = v0;
    nat_mul(a1022, a1023)
}

pub fn f28(v0: u64) -> R<u64> {
    let a1024 = v0;
    let a1025 = 0u64;
    if nat_eq(a1024, a1025) {
        Ok(0u64)
    } else {
        let a1028 = 4u64;
        let a1029 = {
            let a1026 = 3u64;
            let a1027 = v0;
            nat_mul(a1026, a1027)?
        };
        nat_add(a1028, a1029)
    }
}

pub fn f29(v0: bool, v1: Adt2) -> Adt3 {
    if v0 {
        f23(v1, 1u64)
    } else {
        Adt3::C0(match v1.clone() {
            Adt2::C0(_, h1030, _, _, _, _, _, _, _, _, _) => {
                h1030
            }
        }, 1u64, match v1.clone() {
            Adt2::C0(_, _, h1031, _, _, _, _, _, _, _, _) => {
                h1031
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h1032, _, _, _, _, _, _, _) => {
                h1032
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1033, _, _, _, _, _) => {
                h1033
            }
        }, 0u64, match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1034, _, _, _, _) => {
                h1034
            }
        }, match v1 {
            Adt2::C0(_, _, _, _, _, _, _, h1035, _, _, _) => {
                h1035
            }
        })
    }
}

pub fn f30(v0: Adt2) -> R<bool> {
    let a1037 = match v0.clone() {
        Adt2::C0(_, h1036, _, _, _, _, _, _, _, _, _) => {
            h1036
        }
    };
    let a1038 = 1u64;
    if if if if nat_eq(a1037, a1038) {
        true
    } else {
        let a1040 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1039, _, _, _, _, _) => {
                h1039
            }
        };
        let a1041 = 4u64;
        nat_eq(a1040, a1041)
    } {
        let a1043 = match v0.clone() {
            Adt2::C0(_, _, h1042, _, _, _, _, _, _, _, _) => {
                h1042
            }
        };
        let a1044 = 1u64;
        if nat_eq(a1043, a1044) {
            true
        } else {
            let a1049 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h1045, _, _, _, _) => {
                    h1045
                }
            };
            let a1050 = {
                let a1047 = match v0.clone() {
                    Adt2::C0(_, h1046, _, _, _, _, _, _, _, _, _) => {
                        h1046
                    }
                };
                let a1048 = 4u64;
                nat_mul(a1047, a1048)?
            };
            nat_eq(a1049, a1050)
        }
    } else {
        false
    } {
        let a1052 = match v0.clone() {
            Adt2::C0(_, _, _, h1051, _, _, _, _, _, _, _) => {
                h1051
            }
        };
        let a1053 = 1u64;
        if nat_eq(a1052, a1053) {
            true
        } else {
            let a1061 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, h1054, _, _, _) => {
                    h1054
                }
            };
            let a1062 = {
                let a1059 = {
                    let a1057 = match v0.clone() {
                        Adt2::C0(_, h1055, _, _, _, _, _, _, _, _, _) => {
                            h1055
                        }
                    };
                    let a1058 = match v0.clone() {
                        Adt2::C0(_, _, h1056, _, _, _, _, _, _, _, _) => {
                            h1056
                        }
                    };
                    nat_mul(a1057, a1058)?
                };
                let a1060 = 4u64;
                nat_mul(a1059, a1060)?
            };
            nat_eq(a1061, a1062)
        }
    } else {
        false
    } {
        let a1064 = match v0.clone() {
            Adt2::C0(_, _, _, _, h1063, _, _, _, _, _, _) => {
                h1063
            }
        };
        let a1065 = 1u64;
        if nat_eq(a1064, a1065) {
            Ok(true)
        } else {
            let a1076 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, h1066, _, _) => {
                    h1066
                }
            };
            let a1077 = {
                let a1074 = {
                    let a1072 = {
                        let a1069 = match v0.clone() {
                            Adt2::C0(_, h1067, _, _, _, _, _, _, _, _, _) => {
                                h1067
                            }
                        };
                        let a1070 = match v0.clone() {
                            Adt2::C0(_, _, h1068, _, _, _, _, _, _, _, _) => {
                                h1068
                            }
                        };
                        nat_mul(a1069, a1070)?
                    };
                    let a1073 = match v0 {
                        Adt2::C0(_, _, _, h1071, _, _, _, _, _, _, _) => {
                            h1071
                        }
                    };
                    nat_mul(a1072, a1073)?
                };
                let a1075 = 4u64;
                nat_mul(a1074, a1075)?
            };
            Ok(nat_eq(a1076, a1077))
        }
    } else {
        Ok(false)
    }
}

pub fn f31(v0: Adt6, v1: u64) -> R<u64> {
    let a1079 = match v0.clone() {
        Adt6::C0(h1078, _, _, _) => {
            h1078
        }
    };
    let a1080 = 0u64;
    if nat_eq(a1079, a1080) {
        f80(Adt6::C0(1u64, match v0.clone() {
            Adt6::C0(_, h1081, _, _) => {
                h1081
            }
        }, match v0.clone() {
            Adt6::C0(_, _, h1082, _) => {
                h1082
            }
        }, match v0.clone() {
            Adt6::C0(_, _, _, h1083) => {
                h1083
            }
        }), {
            let a1085 = v1;
            let a1086 = match v0 {
                Adt6::C0(_, _, _, h1084) => {
                    h1084
                }
            };
            nat_add(a1085, a1086)?
        })
    } else {
        f80(v0, v1)
    }
}

pub fn f32(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1091 = {
        let a1087 = v0;
        let a1088 = v1;
        nat_mul(a1087, a1088)?
    };
    let a1092 = {
        let a1089 = v2;
        let a1090 = f81(v3);
        nat_mul(a1089, a1090)?
    };
    nat_mul(a1091, a1092)
}

pub fn f33(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1095 = v0;
    let a1096 = {
        let a1093 = f82(v1)?;
        let a1094 = v2;
        nat_mul(a1093, a1094)?
    };
    nat_add(a1095, a1096)
}

pub fn f34(v0: Adt2) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt2::C0(h1097, _, _, _, _, _, _, _, _, _, _) => {
            h1097
        }
    }, match v0.clone() {
        Adt2::C0(_, h1098, _, _, _, _, _, _, _, _, _) => {
            h1098
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h1099, _, _, _, _, _, _, _, _) => {
            h1099
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1100, _, _, _, _, _, _, _) => {
            h1100
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1101, _, _, _, _, _, _) => {
            h1101
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1102, _, _, _, _, _) => {
            h1102
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1103, _, _, _, _) => {
            h1103
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1104, _, _, _) => {
            h1104
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, h1105, _, _) => {
            h1105
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, _, _, h1106, _) => {
            h1106
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1107) => {
            h1107
        }
    })
}

pub fn f35(v0: u64, v1: u64, v2: u64, v3: bool, v4: Adt14, v5: Adt14, v6: Adt14) -> R<Adt15> {
    let a1108 = v0;
    let a1109 = 23u64;
    if nat_eq(a1108, a1109) {
        f83(v1, v4, v6)
    } else {
        let a1110 = v0;
        let a1111 = 26u64;
        if nat_eq(a1110, a1111) {
            f84(v1, v2, v4, v6)
        } else {
            let a1112 = v0;
            let a1113 = 25u64;
            if nat_eq(a1112, a1113) {
                f85(v1, v3, v4, v5, v6)
            } else {
                Ok(Adt15::C0)
            }
        }
    }
}

pub fn f36(v0: u64) -> u64 {
    let a1114 = v0;
    let a1115 = 30u64;
    if nat_eq(a1114, a1115) {
        2u64
    } else {
        4u64
    }
}

pub fn f37(v0: u64) -> bool {
    let a1116 = v0;
    let a1117 = 2139095040u64;
    if nat_le(a1116, a1117) {
        true
    } else {
        let a1118 = v0;
        let a1119 = 2147483648u64;
        nat_eq(a1118, a1119)
    }
}

pub fn f38(v0: Adt2, v1: u64) -> bool {
    let a1121 = match v0.clone() {
        Adt2::C0(_, _, _, _, _, h1120, _, _, _, _, _) => {
            h1120
        }
    };
    let a1122 = v1;
    if if nat_eq(a1121, a1122) {
        let a1127 = {
            let a1124 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1123) => {
                    h1123
                }
            };
            let a1125 = v1;
            let a1126 = 0u64;
            nat_rem(a1124, a1125, a1126)
        };
        let a1128 = 0u64;
        nat_eq(a1127, a1128)
    } else {
        false
    } {
        f86(match v0.clone() {
            Adt2::C0(_, _, h1129, _, _, _, _, _, _, _, _) => {
                h1129
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h1130, _, _, _, _, _, _, _) => {
                h1130
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h1131, _, _, _, _, _, _) => {
                h1131
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1132, _, _, _, _) => {
                h1132
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1133, _, _, _) => {
                h1133
            }
        }, match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1134, _, _) => {
                h1134
            }
        }, v1)
    } else {
        false
    }
}

pub fn f39(v0: Adt2, v1: Adt2) -> bool {
    if if if if if if f24(v0.clone(), v1.clone()) {
        let a1137 = match v0.clone() {
            Adt2::C0(h1135, _, _, _, _, _, _, _, _, _, _) => {
                h1135
            }
        };
        let a1138 = match v1.clone() {
            Adt2::C0(h1136, _, _, _, _, _, _, _, _, _, _) => {
                h1136
            }
        };
        nat_eq(a1137, a1138)
    } else {
        false
    } {
        let a1141 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1139) => {
                h1139
            }
        };
        let a1142 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, _, _, _, h1140) => {
                h1140
            }
        };
        nat_eq(a1141, a1142)
    } else {
        false
    } {
        let a1145 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h1143, _, _, _, _, _) => {
                h1143
            }
        };
        let a1146 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, h1144, _, _, _, _, _) => {
                h1144
            }
        };
        nat_eq(a1145, a1146)
    } else {
        false
    } {
        let a1149 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, h1147, _, _, _, _) => {
                h1147
            }
        };
        let a1150 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, h1148, _, _, _, _) => {
                h1148
            }
        };
        nat_eq(a1149, a1150)
    } else {
        false
    } {
        let a1153 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1151, _, _, _) => {
                h1151
            }
        };
        let a1154 = match v1.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h1152, _, _, _) => {
                h1152
            }
        };
        nat_eq(a1153, a1154)
    } else {
        false
    } {
        let a1157 = match v0 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1155, _, _) => {
                h1155
            }
        };
        let a1158 = match v1 {
            Adt2::C0(_, _, _, _, _, _, _, _, h1156, _, _) => {
                h1156
            }
        };
        nat_eq(a1157, a1158)
    } else {
        false
    }
}

pub fn f40(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a1159 = v0;
    let a1160 = 0u64;
    if if if nat_eq(a1159, a1160) {
        true
    } else {
        let a1161 = v1;
        let a1162 = 0u64;
        nat_eq(a1161, a1162)
    } {
        true
    } else {
        let a1163 = v2;
        let a1164 = 0u64;
        nat_eq(a1163, a1164)
    } {
        true
    } else {
        let a1165 = v3;
        let a1166 = 0u64;
        nat_eq(a1165, a1166)
    }
}

pub fn f41(v0: Adt2, v1: u64) -> R<bool> {
    f87({
        let a1168 = match v0.clone() {
            Adt2::C0(_, h1167, _, _, _, _, _, _, _, _, _) => {
                h1167
            }
        };
        let a1169 = v1;
        nat_mul(a1168, a1169)?
    }, match v0.clone() {
        Adt2::C0(_, _, h1170, _, _, _, _, _, _, _, _) => {
            h1170
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h1171, _, _, _, _, _, _, _) => {
            h1171
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h1172, _, _, _, _, _, _) => {
            h1172
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, h1173, _, _, _, _) => {
            h1173
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, _, _, _, h1174, _, _, _) => {
            h1174
        }
    }, match v0 {
        Adt2::C0(_, _, _, _, _, _, _, _, h1175, _, _) => {
            h1175
        }
    })
}

pub fn f42(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1176 = v1;
    let a1177 = 0u64;
    if if nat_eq(a1176, a1177) {
        true
    } else {
        let a1178 = v2;
        let a1179 = 0u64;
        nat_eq(a1178, a1179)
    } {
        Ok(0u64)
    } else {
        let a1186 = {
            let a1180 = f88(v1, v2, v3)?;
            let a1181 = f89(v0, v1, v2)?;
            nat_add(a1180, a1181)?
        };
        let a1187 = {
            let a1184 = f90(v0);
            let a1185 = {
                let a1182 = v1;
                let a1183 = v2;
                nat_mul(a1182, a1183)?
            };
            nat_mul(a1184, a1185)?
        };
        nat_add(a1186, a1187)
    }
}

pub fn f43(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a1188 = v0;
    let a1189 = 0u64;
    if if nat_eq(a1188, a1189) {
        true
    } else {
        let a1190 = v1;
        let a1191 = 0u64;
        nat_eq(a1190, a1191)
    } {
        Ok(3u64)
    } else {
        let a1194 = {
            let a1192 = 3u64;
            let a1193 = v1;
            nat_add(a1192, a1193)?
        };
        let a1195 = v2;
        nat_add(a1194, a1195)
    }
}

pub fn f44(v0: u64) -> bool {
    let a1196 = v0;
    let a1197 = 2139095040u64;
    if nat_le(a1196, a1197) {
        true
    } else {
        let a1198 = v0;
        let a1199 = 2147483648u64;
        nat_eq(a1198, a1199)
    }
}

pub fn f45(v0: u64, v1: u64) -> R<u64> {
    let a1200 = v0;
    let a1201 = 0u64;
    if nat_eq(a1200, a1201) {
        Ok(0u64)
    } else {
        let a1202 = v1;
        let a1203 = f91(v0)?;
        nat_mul(a1202, a1203)
    }
}

pub fn f46(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a1204 = v0;
    let a1205 = 0u64;
    if if if if nat_eq(a1204, a1205) {
        true
    } else {
        let a1206 = v1;
        let a1207 = 0u64;
        nat_eq(a1206, a1207)
    } {
        true
    } else {
        let a1208 = v2;
        let a1209 = 0u64;
        nat_eq(a1208, a1209)
    } {
        true
    } else {
        let a1210 = v3;
        let a1211 = 0u64;
        nat_eq(a1210, a1211)
    } {
        Ok(0u64)
    } else {
        let a1224 = {
            let a1214 = 2u64;
            let a1215 = {
                let a1212 = v1;
                let a1213 = v2;
                nat_mul(a1212, a1213)?
            };
            nat_mul(a1214, a1215)?
        };
        let a1225 = {
            let a1222 = {
                let a1218 = {
                    let a1216 = v1;
                    let a1217 = v2;
                    nat_mul(a1216, a1217)?
                };
                let a1219 = v3;
                nat_mul(a1218, a1219)?
            };
            let a1223 = {
                let a1220 = 1u64;
                let a1221 = v0;
                nat_add(a1220, a1221)?
            };
            nat_mul(a1222, a1223)?
        };
        nat_add(a1224, a1225)
    }
}

pub fn f47(v0: u64, v1: u64, v2: u64) -> Option<bool> {
    let v3: bool = {
        let a1226 = v1;
        let a1227 = 0u64;
        if if if nat_eq(a1226, a1227) {
            true
        } else {
            let a1228 = v1;
            let a1229 = 10u64;
            nat_eq(a1228, a1229)
        } {
            true
        } else {
            let a1230 = v1;
            let a1231 = 15u64;
            nat_eq(a1230, a1231)
        } {
            true
        } else {
            let a1232 = v1;
            let a1233 = 20u64;
            nat_eq(a1232, a1233)
        }
    };
    let v4: bool = {
        let a1234 = v2;
        let a1235 = 0u64;
        if if if if if nat_eq(a1234, a1235) {
            true
        } else {
            let a1236 = v2;
            let a1237 = 10u64;
            nat_eq(a1236, a1237)
        } {
            true
        } else {
            let a1238 = v2;
            let a1239 = 15u64;
            nat_eq(a1238, a1239)
        } {
            true
        } else {
            let a1240 = v2;
            let a1241 = 20u64;
            nat_eq(a1240, a1241)
        } {
            true
        } else {
            let a1242 = v2;
            let a1243 = 30u64;
            nat_eq(a1242, a1243)
        } {
            true
        } else {
            let a1244 = v2;
            let a1245 = 40u64;
            nat_eq(a1244, a1245)
        }
    };
    if if v3 {
        v4
    } else {
        false
    } {
        Some({
            let a1246 = v0;
            let a1247 = 30u64;
            if if nat_eq(a1246, a1247) {
                let a1248 = v2;
                let a1249 = 10u64;
                nat_eq(a1248, a1249)
            } else {
                false
            } {
                true
            } else {
                if f92(v0) {
                    let a1250 = v2;
                    let a1251 = 10u64;
                    if if nat_eq(a1250, a1251) {
                        true
                    } else {
                        let a1252 = v2;
                        let a1253 = 15u64;
                        nat_eq(a1252, a1253)
                    } {
                        true
                    } else {
                        let a1254 = v2;
                        let a1255 = 20u64;
                        nat_eq(a1254, a1255)
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

pub fn f48(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt9> {
    let v5: u64 = match v1.clone() {
        Adt1::C0(h1256, _, _, _, _, _, _, _, _, _, _, _) => {
            h1256
        }
    };
    Ok(Adt9::C0({
        let a1257 = v0;
        let a1258 = 86u64;
        if nat_eq(a1257, a1258) {
            let m1259 = v3;
            match m1259 {
                None => {
                    Adt10::C1
                }
                Some(v6) => {
                    f93(v1, v2, v6, v4)?
                }
            }
        } else {
            let a1260 = v0;
            let a1261 = 55u64;
            if nat_eq(a1260, a1261) {
                let m1262 = v3;
                match m1262 {
                    None => {
                        Adt10::C1
                    }
                    Some(v7) => {
                        f94(v1, v2, v7, v4)?
                    }
                }
            } else {
                let a1263 = v0;
                let a1264 = 22u64;
                if nat_eq(a1263, a1264) {
                    let m1265 = v3;
                    match m1265 {
                        None => {
                            Adt10::C1
                        }
                        Some(v8) => {
                            f95(v2, v8, v4)?
                        }
                    }
                } else {
                    let a1266 = v0;
                    let a1267 = 44u64;
                    if nat_eq(a1266, a1267) {
                        f96(v1, v2, v3, v4)?
                    } else {
                        let a1268 = v0;
                        let a1269 = 52u64;
                        if nat_eq(a1268, a1269) {
                            f97(v1, v2, v4)?
                        } else {
                            let a1270 = v0;
                            let a1271 = 73u64;
                            if nat_eq(a1270, a1271) {
                                f98(v2, v3, v4)?
                            } else {
                                let a1272 = v0;
                                let a1273 = 57u64;
                                if if nat_eq(a1272, a1273) {
                                    true
                                } else {
                                    let a1274 = v0;
                                    let a1275 = 58u64;
                                    nat_eq(a1274, a1275)
                                } {
                                    let m1276 = v3;
                                    match m1276 {
                                        None => {
                                            f99({
                                                let a1277 = v0;
                                                let a1278 = 58u64;
                                                nat_eq(a1277, a1278)
                                            }, v1, v2, v4)?
                                        }
                                        Some(_) => {
                                            Adt10::C1
                                        }
                                    }
                                } else {
                                    let a1279 = v0;
                                    let a1280 = 49u64;
                                    if nat_eq(a1279, a1280) {
                                        f100(v1, v2, v3, v4)?
                                    } else {
                                        let a1281 = v0;
                                        let a1282 = 51u64;
                                        if nat_eq(a1281, a1282) {
                                            f101(v1, v2, v3, v4)?
                                        } else {
                                            let a1283 = v0;
                                            let a1284 = 39u64;
                                            if nat_eq(a1283, a1284) {
                                                f102(v2, v3, v4)?
                                            } else {
                                                let a1285 = v0;
                                                let a1286 = 77u64;
                                                if nat_eq(a1285, a1286) {
                                                    f103(v2, v4)?
                                                } else {
                                                    let a1287 = v0;
                                                    let a1288 = 40u64;
                                                    if nat_eq(a1287, a1288) {
                                                        f104(v2, v3, v4)?
                                                    } else {
                                                        if f105(v0) {
                                                            let a1289 = v0;
                                                            let a1290 = 33u64;
                                                            if if if nat_eq(a1289, a1290) {
                                                                let a1292 = match v2.clone() {
                                                                    Adt2::C0(h1291, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1291
                                                                    }
                                                                };
                                                                let a1293 = 0u64;
                                                                nat_eq(a1292, a1293)
                                                            } else {
                                                                false
                                                            } {
                                                                let a1295 = match v4.clone() {
                                                                    Adt2::C0(h1294, _, _, _, _, _, _, _, _, _, _) => {
                                                                        h1294
                                                                    }
                                                                };
                                                                let a1296 = 26u64;
                                                                nat_eq(a1295, a1296)
                                                            } else {
                                                                false
                                                            } {
                                                                let m1297 = v3;
                                                                match m1297 {
                                                                    None => {
                                                                        Adt10::C1
                                                                    }
                                                                    Some(v10) => {
                                                                        f106(v2, v10, v4)?
                                                                    }
                                                                }
                                                            } else {
                                                                let a1298 = v0;
                                                                let a1299 = 33u64;
                                                                if if if nat_eq(a1298, a1299) {
                                                                    let a1301 = match v4.clone() {
                                                                        Adt2::C0(h1300, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1300
                                                                        }
                                                                    };
                                                                    let a1302 = 0u64;
                                                                    nat_eq(a1301, a1302)
                                                                } else {
                                                                    false
                                                                } {
                                                                    f107(match v2.clone() {
                                                                        Adt2::C0(h1303, _, _, _, _, _, _, _, _, _, _) => {
                                                                            h1303
                                                                        }
                                                                    })
                                                                } else {
                                                                    false
                                                                } {
                                                                    f108(v2, v3, v4)?
                                                                } else {
                                                                    f109(v0, v2, {
                                                                        let m1304 = v3;
                                                                        match m1304 {
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
                                                            let a1305 = v0;
                                                            let a1306 = 19u64;
                                                            if nat_eq(a1305, a1306) {
                                                                f110(v2, v4)?
                                                            } else {
                                                                let a1307 = v0;
                                                                let a1308 = 21u64;
                                                                if nat_eq(a1307, a1308) {
                                                                    f111(v5, v2, v3, v4)?
                                                                } else {
                                                                    let a1309 = v0;
                                                                    let a1310 = 61u64;
                                                                    if if if nat_eq(a1309, a1310) {
                                                                        true
                                                                    } else {
                                                                        let a1311 = v0;
                                                                        let a1312 = 62u64;
                                                                        nat_eq(a1311, a1312)
                                                                    } {
                                                                        true
                                                                    } else {
                                                                        let a1313 = v0;
                                                                        let a1314 = 63u64;
                                                                        nat_eq(a1313, a1314)
                                                                    } {
                                                                        f112(v0, v1, v2, v4)?
                                                                    } else {
                                                                        let a1315 = v0;
                                                                        let a1316 = 4u64;
                                                                        if if nat_eq(a1315, a1316) {
                                                                            true
                                                                        } else {
                                                                            let a1317 = v0;
                                                                            let a1318 = 32u64;
                                                                            nat_eq(a1317, a1318)
                                                                        } {
                                                                            f113(v0, v1, v2, v3, v4)?
                                                                        } else {
                                                                            let a1319 = v0;
                                                                            let a1320 = 20u64;
                                                                            if nat_eq(a1319, a1320) {
                                                                                f114(v2, v4)?
                                                                            } else {
                                                                                let a1321 = v0;
                                                                                let a1322 = 42u64;
                                                                                if nat_eq(a1321, a1322) {
                                                                                    f115(4u64, v2, v4)?
                                                                                } else {
                                                                                    let a1323 = v0;
                                                                                    let a1324 = 69u64;
                                                                                    if nat_eq(a1323, a1324) {
                                                                                        let a1325 = v5;
                                                                                        let a1326 = 4u64;
                                                                                        if nat_lt(a1325, a1326) {
                                                                                            f115(v5, v2, v4)?
                                                                                        } else {
                                                                                            Adt10::C1
                                                                                        }
                                                                                    } else {
                                                                                        let a1327 = v0;
                                                                                        let a1328 = 70u64;
                                                                                        if nat_eq(a1327, a1328) {
                                                                                            f116(v5, v2, v4)?
                                                                                        } else {
                                                                                            let a1329 = v0;
                                                                                            let a1330 = 75u64;
                                                                                            if if nat_eq(a1329, a1330) {
                                                                                                true
                                                                                            } else {
                                                                                                let a1331 = v0;
                                                                                                let a1332 = 76u64;
                                                                                                nat_eq(a1331, a1332)
                                                                                            } {
                                                                                                f117({
                                                                                                    let a1333 = v0;
                                                                                                    let a1334 = 76u64;
                                                                                                    nat_eq(a1333, a1334)
                                                                                                }, v1, v2, v4)?
                                                                                            } else {
                                                                                                let a1335 = v0;
                                                                                                let a1336 = 31u64;
                                                                                                if if if nat_eq(a1335, a1336) {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1337 = v0;
                                                                                                    let a1338 = 47u64;
                                                                                                    nat_eq(a1337, a1338)
                                                                                                } {
                                                                                                    true
                                                                                                } else {
                                                                                                    let a1339 = v0;
                                                                                                    let a1340 = 68u64;
                                                                                                    nat_eq(a1339, a1340)
                                                                                                } {
                                                                                                    f118(v0, v1, v2, v4)?
                                                                                                } else {
                                                                                                    let a1341 = v0;
                                                                                                    let a1342 = 15u64;
                                                                                                    if nat_eq(a1341, a1342) {
                                                                                                        f119(v2, v4)?
                                                                                                    } else {
                                                                                                        let a1343 = v0;
                                                                                                        let a1344 = 17u64;
                                                                                                        if nat_eq(a1343, a1344) {
                                                                                                            f120(v2, v4)?
                                                                                                        } else {
                                                                                                            let a1345 = v0;
                                                                                                            let a1346 = 18u64;
                                                                                                            if nat_eq(a1345, a1346) {
                                                                                                                f121(v2, v3, v4)?
                                                                                                            } else {
                                                                                                                let a1347 = v0;
                                                                                                                let a1348 = 13u64;
                                                                                                                if if if nat_eq(a1347, a1348) {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1349 = v0;
                                                                                                                    let a1350 = 14u64;
                                                                                                                    nat_eq(a1349, a1350)
                                                                                                                } {
                                                                                                                    true
                                                                                                                } else {
                                                                                                                    let a1351 = v0;
                                                                                                                    let a1352 = 16u64;
                                                                                                                    nat_eq(a1351, a1352)
                                                                                                                } {
                                                                                                                    f122(v0, v2, v4)?
                                                                                                                } else {
                                                                                                                    let m1353 = f123(v0, v5)?;
                                                                                                                    match m1353 {
                                                                                                                        None => {
                                                                                                                            f124(v0, v5, v2, v3, v4)?
                                                                                                                        }
                                                                                                                        Some(v12) => {
                                                                                                                            f125(v12, v2, v4)?
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
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

pub fn f49(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    f126(v1, v2, v3, v4, match v0.clone() {
        Adt3::C0(_, _, _, _, h1354, _, _, _) => {
            h1354
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, h1355, _, _) => {
            h1355
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, _, _, _, h1356, _) => {
            h1356
        }
    }, match v0 {
        Adt3::C0(_, _, _, _, _, _, _, h1357) => {
            h1357
        }
    })
}

pub fn f50(v0: u64, v1: Adt3, v2: u64) -> u64 {
    let v3: u64 = {
        let a1359 = v0;
        let a1360 = match v1.clone() {
            Adt3::C0(h1358, _, _, _, _, _, _, _) => {
                h1358
            }
        };
        let a1361 = 0u64;
        nat_quot(a1359, a1360, a1361)
    };
    let v4: u64 = {
        let a1363 = v3;
        let a1364 = match v1.clone() {
            Adt3::C0(_, h1362, _, _, _, _, _, _) => {
                h1362
            }
        };
        let a1365 = 0u64;
        nat_quot(a1363, a1364, a1365)
    };
    let a1366 = v2;
    let a1367 = 0u64;
    if nat_eq(a1366, a1367) {
        let a1369 = v0;
        let a1370 = match v1 {
            Adt3::C0(h1368, _, _, _, _, _, _, _) => {
                h1368
            }
        };
        let a1371 = 0u64;
        nat_rem(a1369, a1370, a1371)
    } else {
        let a1372 = v2;
        let a1373 = 1u64;
        if nat_eq(a1372, a1373) {
            let a1375 = v3;
            let a1376 = match v1 {
                Adt3::C0(_, h1374, _, _, _, _, _, _) => {
                    h1374
                }
            };
            let a1377 = 0u64;
            nat_rem(a1375, a1376, a1377)
        } else {
            let a1378 = v2;
            let a1379 = 2u64;
            if nat_eq(a1378, a1379) {
                let a1381 = v4;
                let a1382 = match v1 {
                    Adt3::C0(_, _, h1380, _, _, _, _, _) => {
                        h1380
                    }
                };
                let a1383 = 0u64;
                nat_rem(a1381, a1382, a1383)
            } else {
                let a1385 = v4;
                let a1386 = match v1 {
                    Adt3::C0(_, _, h1384, _, _, _, _, _) => {
                        h1384
                    }
                };
                let a1387 = 0u64;
                nat_quot(a1385, a1386, a1387)
            }
        }
    }
}

pub fn f51(v0: Adt8, v1: u64, v2: u64) -> R<u64> {
    let a1389 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1388) => {
            h1388
        }
    };
    let a1390 = 0u64;
    if nat_eq(a1389, a1390) {
        Ok(v2)
    } else {
        let a1395 = {
            let a1392 = v2;
            let a1393 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1391) => {
                    h1391
                }
            };
            nat_mul(a1392, a1393)?
        };
        let a1396 = f50(v1, match v0 {
            Adt8::C0(_, _, _, _, _, _, _, _, h1394, _, _, _, _, _, _) => {
                h1394
            }
        }, 3u64);
        nat_add(a1395, a1396)
    }
}

pub fn f52(v0: Adt6, v1: u64, v2: u64, v3: u64) -> R<Option<u64>> {
    let v4: u64 = {
        let a1406 = {
            let a1403 = {
                let a1398 = v2;
                let a1399 = match v0.clone() {
                    Adt6::C0(_, h1397, _, _) => {
                        h1397
                    }
                };
                nat_mul(a1398, a1399)?
            };
            let a1404 = {
                let a1401 = v3;
                let a1402 = match v0.clone() {
                    Adt6::C0(_, _, _, h1400) => {
                        h1400
                    }
                };
                nat_mul(a1401, a1402)?
            };
            nat_add(a1403, a1404)?
        };
        let a1407 = f127(match v0.clone() {
            Adt6::C0(_, _, h1405, _) => {
                h1405
            }
        });
        nat_add(a1406, a1407)?
    };
    let v5: u64 = f128(match v0 {
        Adt6::C0(_, _, h1408, _) => {
            h1408
        }
    });
    let a1409 = v5;
    let a1410 = v4;
    if if nat_le(a1409, a1410) {
        let a1413 = {
            let a1411 = v4;
            let a1412 = v5;
            nat_sub(a1411, a1412)
        };
        let a1414 = v1;
        nat_lt(a1413, a1414)
    } else {
        false
    } {
        Ok(Some({
            let a1415 = v4;
            let a1416 = v5;
            nat_sub(a1415, a1416)
        }))
    } else {
        Ok(None::<u64>)
    }
}

pub fn f53(v0: Adt8, v1: u64, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, h1417, _, _, _, _, _) => {
            h1417
        }
    };
    let v6: Adt3 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, h1418, _, _, _, _, _, _) => {
            h1418
        }
    };
    let v7: u64 = f50(v1, v6.clone(), 2u64);
    let a1420 = match v0.clone() {
        Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1419) => {
            h1419
        }
    };
    let a1421 = 0u64;
    if nat_eq(a1420, a1421) {
        Ok(Adt9::C8(Some(f49(v5, v3, v4, v7, v2)?)))
    } else {
        let v8: u64 = {
            let m1424 = match v0.clone() {
                Adt8::C0(_, _, _, _, _, _, _, _, _, _, h1423, _, _, _, _) => {
                    h1423
                }
            };
            match m1424 {
                Adt3::C0(_, _, h1422, _, _, _, _, _) => {
                    h1422
                }
            }
        };
        let m1430 = f52(match v0.clone() {
            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, h1425, _) => {
                h1425
            }
        }, match v5.clone() {
            Adt3::C0(_, _, h1426, _, _, _, _, _) => {
                h1426
            }
        }, {
            let a1427 = v2;
            let a1428 = v8;
            let a1429 = 0u64;
            nat_rem(a1427, a1428, a1429)
        }, v7)?;
        match m1430 {
            None => {
                Ok(Adt9::C8(None::<u64>))
            }
            Some(v9) => {
                let v10: u64 = {
                    let a1437 = {
                        let a1435 = {
                            let a1431 = v2;
                            let a1432 = v8;
                            let a1433 = 0u64;
                            nat_quot(a1431, a1432, a1433)
                        };
                        let a1436 = match v0 {
                            Adt8::C0(_, _, _, _, _, _, _, _, _, _, _, _, _, _, h1434) => {
                                h1434
                            }
                        };
                        nat_mul(a1435, a1436)?
                    };
                    let a1438 = f50(v1, v6, 3u64);
                    nat_add(a1437, a1438)?
                };
                Ok(Adt9::C8(Some(f49(v5, v3, v4, v9, v10)?)))
            }
        }
    }
}

pub fn f54(v0: Adt4, v1: Adt9) -> Adt9 {
    let m1439 = v1;
    match m1439 {
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
            f129(f130(v0.clone(), v20, v21), v20, v21, 0u64, v22, 2u64, f131(v0), 3u64, v23)
        }
        Adt9::C4(v24, v25, v26, v27, v28, v29, v30) => {
            let v31: u64 = f132(v0.clone(), v24);
            f129(f133(v0, v24), v31, v31, v25, v26, v27, v28, v29, v30)
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

pub fn f55(v0: u64, v1: Adt3, v2: Adt3, v3: Adt4) -> R<Adt9> {
    let m1440 = v3;
    match m1440 {
        Adt4::C0 => {
            Ok(Adt9::C1(false, f79(v0, v1)?, f79(v0, v2)?))
        }
        Adt4::C1 => {
            Ok(Adt9::C1(false, f135(v0, v1, v2.clone())?, f79(v0, v2)?))
        }
        Adt4::C2(v6, v7) => {
            f136(v0, v1, v7, v2, v6)
        }
        Adt4::C3(v8, v9, v10, v11, v12) => {
            f137(v0, v1, v2, v8, v9, v10, v11, v12)
        }
        Adt4::C4(v13) => {
            f138(v0, v1, v2, v13)
        }
        Adt4::C5(v14, v15, v16, v17) => {
            f137(v0, v1, v2, v14, v15, v16, v17, true)
        }
        Adt4::C6(v18, v19, v20) => {
            f139(v0, v1, v2, v18, v19, v20)
        }
        Adt4::C7(v21) => {
            f140(v0, v1, v2, v21)
        }
        Adt4::C8(v22) => {
            f141(v22, v0, v1, v2)
        }
        Adt4::C9(_, _) => {
            Ok(Adt9::C1(true, 0u64, f79(v0, v2)?))
        }
        Adt4::C10(v25, v26, v27) => {
            f142(v0, v1, v2, v25, v26, v27)
        }
        Adt4::C11(v47, v48) => {
            f143(v0, v1, v2, v47, v48)
        }
        Adt4::C12(v49, v50, v51) => {
            f144(v0, v1, v2, v49, v50, v51)
        }
        Adt4::C13(_, v53, _, _) => {
            Ok(Adt9::C3(v53, v53, f79(v0, v1)?, f79(v0, v2)?))
        }
        Adt4::C14(v56, v57, v58) => {
            let a1443 = v56;
            let a1444 = 23u64;
            if nat_eq(a1443, a1444) {
                f145(v0, v1, v2, v57, v58)
            } else {
                Ok(Adt9::C3(v57, v58, f79(v0, v1)?, f79(v0, v2)?))
            }
        }
        Adt4::C15 => {
            f146(v0, v1, v2)
        }
        Adt4::C16(v59, v60) => {
            f147(v0, v1, v2, v59, v60)
        }
        Adt4::C17 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C18(v61, v62) => {
            f148(v0, v1, v62, v61)
        }
        Adt4::C19(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C20(_, _) => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt4::C21 => {
            Ok(Adt9::C1(false, f79(f134(v0, match v2.clone() {
                Adt3::C0(h1441, _, _, _, _, _, _, _) => {
                    h1441
                }
            }, match v2.clone() {
                Adt3::C0(_, h1442, _, _, _, _, _, _) => {
                    h1442
                }
            })?, v1)?, f79(v0, v2)?))
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
        Adt4::C25(v67, v68, v69) => {
            f149(v0, v67, v68, v69)
        }
        Adt4::C26(v75, _, _, _) => {
            let a1445 = v75;
            let a1446 = 64u64;
            if if if nat_eq(a1445, a1446) {
                true
            } else {
                let a1447 = v75;
                let a1448 = 65u64;
                nat_eq(a1447, a1448)
            } {
                true
            } else {
                let a1449 = v75;
                let a1450 = 66u64;
                nat_eq(a1449, a1450)
            } {
                Ok(Adt9::C0(Adt10::C5(64u64)))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
        Adt4::C27(v74) => {
            f71(v0, v1, v2, v74)
        }
        Adt4::C28(v70) => {
            f68(v0, v1, v2, v70, 0u64)
        }
        Adt4::C29(v71) => {
            f69(v0, v1, v2, v71)
        }
        Adt4::C30(v72, _) => {
            f70(v0, v1, v2, v72)
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
    }
}

pub fn f56(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f150(v3, v4);
    let v6: u64 = {
        let a1451 = v0;
        let a1452 = v5;
        let a1453 = 0u64;
        nat_quot(a1451, a1452, a1453)
    };
    let a1454 = v5;
    let a1455 = 1u64;
    if nat_eq(a1454, a1455) {
        Ok(f129(f151(v3, v4), v3, v4, 0u64, f79(v6, v1)?, 2u64, 0u64, 3u64, f79(v6, v2)?))
    } else {
        let a1459 = {
            let a1456 = v0;
            let a1457 = 2u64;
            let a1458 = 0u64;
            nat_rem(a1456, a1457, a1458)
        };
        let a1460 = 0u64;
        if nat_eq(a1459, a1460) {
            Ok(f129(f151(v3, 0u64), v3, 0u64, 0u64, f79(v6, v1)?, 2u64, 0u64, 1u64, 0u64))
        } else {
            Ok(f129(f151(0u64, v4), 0u64, v4, 1u64, 0u64, 2u64, 0u64, 3u64, f79(v6, v2)?))
        }
    }
}

pub fn f57(v0: u64, v1: u64, v2: u64) -> R<Adt9> {
    let v3: u64 = f152(v1)?;
    let v4: u64 = {
        let a1461 = v0;
        let a1462 = v3;
        let a1463 = 0u64;
        nat_rem(a1461, a1462, a1463)
    };
    let v5: u64 = {
        let a1466 = {
            let a1464 = v1;
            let a1465 = 3u64;
            nat_mul(a1464, a1465)?
        };
        let a1467 = 1u64;
        nat_add(a1466, a1467)?
    };
    let a1468 = v4;
    let a1469 = 0u64;
    if nat_eq(a1468, a1469) {
        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
    } else {
        let a1470 = v4;
        let a1471 = v5;
        if nat_eq(a1470, a1471) {
            Ok(Adt9::C7(Adt11::C0(2u64), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 8u64, 8u64))
        } else {
            let v6: u64 = {
                let a1476 = v4;
                let a1477 = {
                    let a1472 = v4;
                    let a1473 = v5;
                    if nat_lt(a1472, a1473) {
                        1u64
                    } else {
                        let a1474 = v5;
                        let a1475 = 1u64;
                        nat_add(a1474, a1475)?
                    }
                };
                nat_sub(a1476, a1477)
            };
            let v7: u64 = {
                let a1478 = v6;
                let a1479 = 3u64;
                let a1480 = 0u64;
                nat_rem(a1478, a1479, a1480)
            };
            let v8: u64 = {
                let a1491 = 4u64;
                let a1492 = {
                    let a1489 = {
                        let a1484 = {
                            let a1481 = v0;
                            let a1482 = v3;
                            let a1483 = 0u64;
                            nat_quot(a1481, a1482, a1483)
                        };
                        let a1485 = v1;
                        nat_mul(a1484, a1485)?
                    };
                    let a1490 = {
                        let a1486 = v6;
                        let a1487 = 3u64;
                        let a1488 = 0u64;
                        nat_quot(a1486, a1487, a1488)
                    };
                    nat_add(a1489, a1490)?
                };
                nat_mul(a1491, a1492)?
            };
            let a1493 = v4;
            let a1494 = v5;
            if nat_lt(a1493, a1494) {
                let a1495 = v7;
                let a1496 = 0u64;
                if nat_eq(a1495, a1496) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, v8), Adt12::C0(0u64, v8), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1497 = v7;
                    let a1498 = 1u64;
                    if nat_eq(a1497, a1498) {
                        Ok(Adt9::C7(Adt11::C0(1u64), Adt12::C1(1u64), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C0(0u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C0(0u64), 8u64, 8u64))
                    }
                }
            } else {
                let a1499 = v7;
                let a1500 = 0u64;
                if nat_eq(a1499, a1500) {
                    Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C0(0u64, v8), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1501 = v7;
                    let a1502 = 1u64;
                    if nat_eq(a1501, a1502) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v2), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, v8), Adt13::C1(v8), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f58(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a1503 = v0;
        let a1504 = 6u64;
        let a1505 = 0u64;
        nat_quot(a1503, a1504, a1505)
    };
    let v5: u64 = {
        let a1506 = v0;
        let a1507 = 6u64;
        let a1508 = 0u64;
        nat_rem(a1506, a1507, a1508)
    };
    let a1509 = v5;
    let a1510 = 0u64;
    if nat_eq(a1509, a1510) {
        Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, f79(v4, v2)?), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1511 = v5;
        let a1512 = 1u64;
        if nat_eq(a1511, a1512) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C2(1065353216u64), Adt12::C1(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1513 = v5;
            let a1514 = 2u64;
            if nat_eq(a1513, a1514) {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, f79(v4, v2)?), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let a1515 = v5;
                let a1516 = 3u64;
                if nat_eq(a1515, a1516) {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    let a1517 = v5;
                    let a1518 = 4u64;
                    if nat_eq(a1517, a1518) {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f79(v4, v1)?), Adt12::C1(0u64), Adt13::C0(0u64), 4u64, 4u64))
                    } else {
                        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(0u64), Adt12::C1(1u64), Adt13::C1(f79(v4, v3)?), 4u64, 4u64))
                    }
                }
            }
        }
    }
}

pub fn f59(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: bool, v5: u64) -> R<Adt9> {
    let v6: u64 = if v4 {
        3u64
    } else {
        2u64
    };
    let v7: u64 = f153(match v1.clone() {
        Adt3::C0(_, h1519, _, _, _, _, _, _) => {
            h1519
        }
    }, v4)?;
    let v8: u64 = {
        let a1520 = v0;
        let a1521 = v7;
        let a1522 = 0u64;
        nat_rem(a1520, a1521, a1522)
    };
    let v9: u64 = {
        let a1523 = v0;
        let a1524 = v7;
        let a1525 = 0u64;
        nat_quot(a1523, a1524, a1525)
    };
    let a1528 = {
        let a1526 = v8;
        let a1527 = 1u64;
        nat_add(a1526, a1527)?
    };
    let a1529 = v7;
    if nat_eq(a1528, a1529) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C2(v5), Adt12::C1(0u64), Adt13::C1(f79(v9, v3)?), 4u64, 4u64))
    } else {
        let v10: u64 = {
            let a1530 = v8;
            let a1531 = v6;
            let a1532 = 0u64;
            nat_quot(a1530, a1531, a1532)
        };
        let v11: u64 = {
            let a1533 = v8;
            let a1534 = v6;
            let a1535 = 0u64;
            nat_rem(a1533, a1534, a1535)
        };
        let v12: u64 = {
            let a1537 = v9;
            let a1538 = match v1.clone() {
                Adt3::C0(h1536, _, _, _, _, _, _, _) => {
                    h1536
                }
            };
            let a1539 = 0u64;
            nat_rem(a1537, a1538, a1539)
        };
        let v13: u64 = {
            let a1541 = v9;
            let a1542 = match v1.clone() {
                Adt3::C0(h1540, _, _, _, _, _, _, _) => {
                    h1540
                }
            };
            let a1543 = 0u64;
            nat_quot(a1541, a1542, a1543)
        };
        let v14: u64 = if v4 {
            let a1555 = {
                let a1550 = {
                    let a1545 = match v2.clone() {
                        Adt3::C0(_, _, _, _, h1544, _, _, _) => {
                            h1544
                        }
                    };
                    let a1546 = v12;
                    nat_mul(a1545, a1546)?
                };
                let a1551 = {
                    let a1548 = match v2.clone() {
                        Adt3::C0(_, _, _, _, _, h1547, _, _) => {
                            h1547
                        }
                    };
                    let a1549 = v10;
                    nat_mul(a1548, a1549)?
                };
                nat_add(a1550, a1551)?
            };
            let a1556 = {
                let a1553 = match v2 {
                    Adt3::C0(_, _, _, _, _, _, h1552, _) => {
                        h1552
                    }
                };
                let a1554 = v13;
                nat_mul(a1553, a1554)?
            };
            nat_add(a1555, a1556)?
        } else {
            let a1563 = {
                let a1558 = match v2.clone() {
                    Adt3::C0(_, _, _, _, h1557, _, _, _) => {
                        h1557
                    }
                };
                let a1559 = v10;
                nat_mul(a1558, a1559)?
            };
            let a1564 = {
                let a1561 = match v2 {
                    Adt3::C0(_, _, _, _, _, h1560, _, _) => {
                        h1560
                    }
                };
                let a1562 = v13;
                nat_mul(a1561, a1562)?
            };
            nat_add(a1563, a1564)?
        };
        if if v4 {
            let a1565 = v11;
            let a1566 = 0u64;
            nat_eq(a1565, a1566)
        } else {
            false
        } {
            Ok(Adt9::C7(Adt11::C7(17u64, 0u64, 0u64), Adt12::C0(1u64, v14), Adt12::C2(0u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a1569 = {
                let a1567 = v11;
                let a1568 = 1u64;
                nat_add(a1567, a1568)?
            };
            let a1570 = v6;
            if nat_eq(a1569, a1570) {
                Ok(Adt9::C7(Adt11::C1(0u64), {
                    let a1571 = v10;
                    let a1572 = 0u64;
                    if nat_eq(a1571, a1572) {
                        Adt12::C2(0u64)
                    } else {
                        Adt12::C1(0u64)
                    }
                }, Adt12::C1(1u64), Adt13::C0(0u64), 4u64, 4u64))
            } else {
                Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
                    let a1584 = {
                        let a1579 = {
                            let a1574 = match v1.clone() {
                                Adt3::C0(_, _, _, _, h1573, _, _, _) => {
                                    h1573
                                }
                            };
                            let a1575 = v12;
                            nat_mul(a1574, a1575)?
                        };
                        let a1580 = {
                            let a1577 = match v1.clone() {
                                Adt3::C0(_, _, _, _, _, h1576, _, _) => {
                                    h1576
                                }
                            };
                            let a1578 = v10;
                            nat_mul(a1577, a1578)?
                        };
                        nat_add(a1579, a1580)?
                    };
                    let a1585 = {
                        let a1582 = match v1 {
                            Adt3::C0(_, _, _, _, _, _, h1581, _) => {
                                h1581
                            }
                        };
                        let a1583 = v13;
                        nat_mul(a1582, a1583)?
                    };
                    nat_add(a1584, a1585)?
                }), if v4 {
                    Adt12::C1(1u64)
                } else {
                    Adt12::C0(1u64, v14)
                }, Adt13::C0(1u64), 4u64, 4u64))
            }
        }
    }
}

pub fn f60(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Option<Adt3>, v5: Adt3) -> R<Adt9> {
    let v6: u64 = f25(match v2.clone() {
        Adt3::C0(_, h1586, _, _, _, _, _, _) => {
            h1586
        }
    }, v4.clone())?;
    let v7: u64 = {
        let a1587 = v0;
        let a1588 = v6;
        let a1589 = 0u64;
        nat_rem(a1587, a1588, a1589)
    };
    let v8: u64 = {
        let a1590 = v0;
        let a1591 = v6;
        let a1592 = 0u64;
        nat_quot(a1590, a1591, a1592)
    };
    let v9: u64 = {
        let a1594 = v8;
        let a1595 = match v5.clone() {
            Adt3::C0(h1593, _, _, _, _, _, _, _) => {
                h1593
            }
        };
        let a1596 = 0u64;
        nat_rem(a1594, a1595, a1596)
    };
    let v10: u64 = {
        let a1598 = v8;
        let a1599 = match v5.clone() {
            Adt3::C0(h1597, _, _, _, _, _, _, _) => {
                h1597
            }
        };
        let a1600 = 0u64;
        nat_quot(a1598, a1599, a1600)
    };
    let v11: u64 = {
        let a1602 = v10;
        let a1603 = match v2.clone() {
            Adt3::C0(_, h1601, _, _, _, _, _, _) => {
                h1601
            }
        };
        let a1604 = 0u64;
        nat_rem(a1602, a1603, a1604)
    };
    let v12: u64 = {
        let a1606 = v10;
        let a1607 = match v2.clone() {
            Adt3::C0(_, h1605, _, _, _, _, _, _) => {
                h1605
            }
        };
        let a1608 = 0u64;
        nat_quot(a1606, a1607, a1608)
    };
    let a1609 = v7;
    let a1610 = 0u64;
    if nat_eq(a1609, a1610) {
        Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, {
            let a1617 = {
                let a1612 = match v1.clone() {
                    Adt3::C0(_, _, _, _, h1611, _, _, _) => {
                        h1611
                    }
                };
                let a1613 = v9;
                nat_mul(a1612, a1613)?
            };
            let a1618 = {
                let a1615 = match v1 {
                    Adt3::C0(_, _, _, _, _, h1614, _, _) => {
                        h1614
                    }
                };
                let a1616 = v12;
                nat_mul(a1615, a1616)?
            };
            nat_add(a1617, a1618)?
        }), Adt12::C0(2u64, {
            let a1625 = {
                let a1620 = match v3.clone() {
                    Adt3::C0(_, _, _, _, h1619, _, _, _) => {
                        h1619
                    }
                };
                let a1621 = v11;
                nat_mul(a1620, a1621)?
            };
            let a1626 = {
                let a1623 = match v3 {
                    Adt3::C0(_, _, _, _, _, h1622, _, _) => {
                        h1622
                    }
                };
                let a1624 = v12;
                nat_mul(a1623, a1624)?
            };
            nat_add(a1625, a1626)?
        }), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let m1627 = v4;
        match m1627 {
            None => {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C0(1u64, {
                    let a1639 = {
                        let a1634 = {
                            let a1629 = match v2.clone() {
                                Adt3::C0(_, _, _, _, h1628, _, _, _) => {
                                    h1628
                                }
                            };
                            let a1630 = v9;
                            nat_mul(a1629, a1630)?
                        };
                        let a1635 = {
                            let a1632 = match v2.clone() {
                                Adt3::C0(_, _, _, _, _, h1631, _, _) => {
                                    h1631
                                }
                            };
                            let a1633 = v11;
                            nat_mul(a1632, a1633)?
                        };
                        nat_add(a1634, a1635)?
                    };
                    let a1640 = {
                        let a1637 = match v2 {
                            Adt3::C0(_, _, _, _, _, _, h1636, _) => {
                                h1636
                            }
                        };
                        let a1638 = v12;
                        nat_mul(a1637, a1638)?
                    };
                    nat_add(a1639, a1640)?
                }), Adt13::C1(f79(v8, v5)?), 4u64, 4u64))
            }
            Some(v13) => {
                let v14: u64 = {
                    let a1641 = v7;
                    let a1642 = 1u64;
                    nat_sub(a1641, a1642)
                };
                let v15: u64 = {
                    let a1643 = v14;
                    let a1644 = 2u64;
                    let a1645 = 0u64;
                    nat_quot(a1643, a1644, a1645)
                };
                let a1649 = {
                    let a1646 = v14;
                    let a1647 = 2u64;
                    let a1648 = 0u64;
                    nat_rem(a1646, a1647, a1648)
                };
                let a1650 = 0u64;
                if nat_eq(a1649, a1650) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(1u64, {
                        let a1662 = {
                            let a1657 = {
                                let a1652 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, h1651, _, _, _) => {
                                        h1651
                                    }
                                };
                                let a1653 = v9;
                                nat_mul(a1652, a1653)?
                            };
                            let a1658 = {
                                let a1655 = match v2.clone() {
                                    Adt3::C0(_, _, _, _, _, h1654, _, _) => {
                                        h1654
                                    }
                                };
                                let a1656 = v15;
                                nat_mul(a1655, a1656)?
                            };
                            nat_add(a1657, a1658)?
                        };
                        let a1663 = {
                            let a1660 = match v2 {
                                Adt3::C0(_, _, _, _, _, _, h1659, _) => {
                                    h1659
                                }
                            };
                            let a1661 = v12;
                            nat_mul(a1660, a1661)?
                        };
                        nat_add(a1662, a1663)?
                    }), Adt12::C0(3u64, {
                        let a1675 = {
                            let a1670 = {
                                let a1665 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, h1664, _, _, _) => {
                                        h1664
                                    }
                                };
                                let a1666 = v11;
                                nat_mul(a1665, a1666)?
                            };
                            let a1671 = {
                                let a1668 = match v13.clone() {
                                    Adt3::C0(_, _, _, _, _, h1667, _, _) => {
                                        h1667
                                    }
                                };
                                let a1669 = v15;
                                nat_mul(a1668, a1669)?
                            };
                            nat_add(a1670, a1671)?
                        };
                        let a1676 = {
                            let a1673 = match v13 {
                                Adt3::C0(_, _, _, _, _, _, h1672, _) => {
                                    h1672
                                }
                            };
                            let a1674 = v12;
                            nat_mul(a1673, a1674)?
                        };
                        nat_add(a1675, a1676)?
                    }), Adt13::C0(1u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(0u64), Adt12::C1(1u64), {
                        let a1679 = {
                            let a1677 = v7;
                            let a1678 = 1u64;
                            nat_add(a1677, a1678)?
                        };
                        let a1680 = v6;
                        if nat_eq(a1679, a1680) {
                            Adt13::C1(f79(v8, v5)?)
                        } else {
                            Adt13::C0(0u64)
                        }
                    }, 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f61(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = f22(v6)?;
    let v9: u64 = {
        let a1681 = v0;
        let a1682 = v8;
        let a1683 = 0u64;
        nat_rem(a1681, a1682, a1683)
    };
    let v10: u64 = {
        let a1684 = v0;
        let a1685 = v8;
        let a1686 = 0u64;
        nat_quot(a1684, a1685, a1686)
    };
    let a1687 = v9;
    let a1688 = 148u64;
    if nat_lt(a1687, a1688) {
        f154({
            let a1689 = v9;
            let a1690 = 37u64;
            let a1691 = 0u64;
            nat_rem(a1689, a1690, a1691)
        }, {
            let a1692 = v9;
            let a1693 = 37u64;
            let a1694 = 0u64;
            nat_quot(a1692, a1693, a1694)
        }, v10, v1, v2, v3, v4, v5, v7)
    } else {
        f155({
            let a1695 = v9;
            let a1696 = 148u64;
            nat_sub(a1695, a1696)
        }, v10, v4, v5)
    }
}

pub fn f62(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let a1697 = v0;
    let a1698 = v1;
    if nat_lt(a1697, a1698) {
        Ok(Adt9::C7(Adt11::C9, Adt12::C0(0u64, f79(v0, v2)?), Adt12::C2(0u64), Adt13::C0(v0), 4u64, 4u64))
    } else {
        let v4: u64 = {
            let a1699 = v0;
            let a1700 = v1;
            nat_sub(a1699, a1700)
        };
        Ok(Adt9::C7(Adt11::C5(3u64), Adt12::C1(v4), Adt12::C2(0u64), Adt13::C1(f79(v4, v3)?), 4u64, 4u64))
    }
}

pub fn f63(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt6, v5: Adt6, v6: u64) -> R<Adt9> {
    let v7: u64 = {
        let a1707 = {
            let a1705 = 3u64;
            let a1706 = {
                let a1703 = match v4.clone() {
                    Adt6::C0(h1701, _, _, _) => {
                        h1701
                    }
                };
                let a1704 = match v5.clone() {
                    Adt6::C0(h1702, _, _, _) => {
                        h1702
                    }
                };
                nat_mul(a1703, a1704)?
            };
            nat_mul(a1705, a1706)?
        };
        let a1708 = 2u64;
        nat_add(a1707, a1708)?
    };
    let v8: u64 = {
        let a1709 = v0;
        let a1710 = v7;
        let a1711 = 0u64;
        nat_quot(a1709, a1710, a1711)
    };
    let v9: u64 = {
        let a1712 = v0;
        let a1713 = v7;
        let a1714 = 0u64;
        nat_rem(a1712, a1713, a1714)
    };
    let a1715 = v9;
    let a1716 = 0u64;
    if nat_eq(a1715, a1716) {
        Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C2(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
    } else {
        let a1719 = {
            let a1717 = v9;
            let a1718 = 1u64;
            nat_add(a1717, a1718)?
        };
        let a1720 = v7;
        if nat_eq(a1719, a1720) {
            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C1(f79(v8, v3)?), 4u64, 4u64))
        } else {
            let v10: u64 = {
                let a1721 = v9;
                let a1722 = 1u64;
                nat_sub(a1721, a1722)
            };
            let v11: u64 = {
                let a1723 = v10;
                let a1724 = 3u64;
                let a1725 = 0u64;
                nat_quot(a1723, a1724, a1725)
            };
            let v12: u64 = {
                let a1727 = v11;
                let a1728 = match v4.clone() {
                    Adt6::C0(h1726, _, _, _) => {
                        h1726
                    }
                };
                let a1729 = 0u64;
                nat_rem(a1727, a1728, a1729)
            };
            let v13: u64 = {
                let a1731 = v11;
                let a1732 = match v4.clone() {
                    Adt6::C0(h1730, _, _, _) => {
                        h1730
                    }
                };
                let a1733 = 0u64;
                nat_quot(a1731, a1732, a1733)
            };
            let v14: u64 = {
                let a1734 = v10;
                let a1735 = 3u64;
                let a1736 = 0u64;
                nat_rem(a1734, a1735, a1736)
            };
            let v15: u64 = f50(v8, v3.clone(), 2u64);
            let m1738 = f52(v4, match v2.clone() {
                Adt3::C0(h1737, _, _, _, _, _, _, _) => {
                    h1737
                }
            }, f50(v8, v3.clone(), 0u64), v12)?;
            match m1738 {
                None => {
                    Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                }
                Some(v16) => {
                    let m1740 = f52(v5, match v2.clone() {
                        Adt3::C0(_, h1739, _, _, _, _, _, _) => {
                            h1739
                        }
                    }, f50(v8, v3.clone(), 1u64), v13)?;
                    match m1740 {
                        None => {
                            Ok(Adt9::C7(Adt11::C2(0u64, 0u64, false), Adt12::C1(0u64), Adt12::C2(0u64), Adt13::C0(0u64), 4u64, 4u64))
                        }
                        Some(v17) => {
                            let a1741 = v14;
                            let a1742 = 0u64;
                            if nat_eq(a1741, a1742) {
                                Ok(Adt9::C7(Adt11::C2(v6, 0u64, false), Adt12::C0(0u64, f49(v1, v12, v13, 0u64, v15)?), Adt12::C2(0u64), Adt13::C0(1u64), f156(v6), 4u64))
                            } else {
                                let a1743 = v14;
                                let a1744 = 1u64;
                                if nat_eq(a1743, a1744) {
                                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C1(1u64), Adt12::C0(1u64, f49(v2, v16, v17, v15, f50(v8, v3, 3u64))?), Adt13::C0(1u64), 4u64, 4u64))
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

pub fn f64(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f157(match v1.clone() {
        Adt3::C0(h1745, _, _, _, _, _, _, _) => {
            h1745
        }
    })?;
    let v5: u64 = {
        let a1746 = v0;
        let a1747 = v4;
        let a1748 = 0u64;
        nat_quot(a1746, a1747, a1748)
    };
    let v6: u64 = {
        let a1749 = v0;
        let a1750 = v4;
        let a1751 = 0u64;
        nat_rem(a1749, a1750, a1751)
    };
    let a1752 = v6;
    let a1753 = 0u64;
    if nat_eq(a1752, a1753) {
        Ok(Adt9::C4(19u64, 2u64, 4286578688u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1754 = v6;
        let a1755 = 1u64;
        if nat_eq(a1754, a1755) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a1758 = {
                let a1756 = v6;
                let a1757 = 1u64;
                nat_add(a1756, a1757)?
            };
            let a1759 = v4;
            if nat_eq(a1758, a1759) {
                Ok(Adt9::C4(19u64, 1u64, 1u64, 2u64, 0u64, 3u64, f79(v5, v2)?))
            } else {
                let v7: u64 = {
                    let a1760 = v6;
                    let a1761 = 2u64;
                    nat_sub(a1760, a1761)
                };
                let v8: u64 = {
                    let a1762 = v7;
                    let a1763 = 3u64;
                    let a1764 = 0u64;
                    nat_quot(a1762, a1763, a1764)
                };
                let v9: u64 = {
                    let a1765 = v7;
                    let a1766 = 3u64;
                    let a1767 = 0u64;
                    nat_rem(a1765, a1766, a1767)
                };
                let v10: u64 = f79({
                    let a1771 = {
                        let a1769 = v5;
                        let a1770 = match v1.clone() {
                            Adt3::C0(h1768, _, _, _, _, _, _, _) => {
                                h1768
                            }
                        };
                        nat_mul(a1769, a1770)?
                    };
                    let a1772 = v8;
                    nat_add(a1771, a1772)?
                }, v1)?;
                let a1773 = v9;
                let a1774 = 0u64;
                if nat_eq(a1773, a1774) {
                    Ok(Adt9::C4(16u64, 1u64, 0u64, 0u64, v10, 1u64, 2u64))
                } else {
                    let a1775 = v9;
                    let a1776 = 1u64;
                    if nat_eq(a1775, a1776) {
                        let a1777 = v3;
                        let a1778 = 2u64;
                        if nat_eq(a1777, a1778) {
                            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                        } else {
                            Ok(Adt9::C4(19u64, 0u64, v10, 2u64, 0u64, 1u64, 0u64))
                        }
                    } else {
                        let a1779 = v3;
                        let a1780 = 2u64;
                        if if nat_eq(a1779, a1780) {
                            true
                        } else {
                            let a1781 = v3;
                            let a1782 = 3u64;
                            nat_eq(a1781, a1782)
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

pub fn f65(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1784 = 2u64;
        let a1785 = match v2.clone() {
            Adt3::C0(_, h1783, _, _, _, _, _, _) => {
                h1783
            }
        };
        nat_mul(a1784, a1785)?
    };
    let a1786 = v1;
    let a1787 = v6;
    if nat_lt(a1786, a1787) {
        let a1791 = {
            let a1788 = v1;
            let a1789 = 2u64;
            let a1790 = 0u64;
            nat_rem(a1788, a1789, a1790)
        };
        let a1792 = 0u64;
        if nat_eq(a1791, a1792) {
            Ok(Adt9::C4(19u64, 4u64, f79({
                let a1793 = v1;
                let a1794 = 2u64;
                let a1795 = 0u64;
                nat_quot(a1793, a1794, a1795)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a1796 = v5;
            let a1797 = 2147483648u64;
            if if nat_lt(a1796, a1797) {
                let a1799 = v5;
                let a1800 = match v3 {
                    Adt3::C0(_, h1798, _, _, _, _, _, _) => {
                        h1798
                    }
                };
                nat_lt(a1799, a1800)
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
            let a1803 = match v3.clone() {
                Adt3::C0(h1801, _, _, _, _, _, _, _) => {
                    h1801
                }
            };
            let a1804 = match v3.clone() {
                Adt3::C0(_, h1802, _, _, _, _, _, _) => {
                    h1802
                }
            };
            nat_mul(a1803, a1804)?
        };
        let a1807 = {
            let a1805 = v1;
            let a1806 = v6;
            nat_sub(a1805, a1806)
        };
        let a1808 = v7;
        if nat_lt(a1807, a1808) {
            Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 3u64, f79({
                let a1809 = v1;
                let a1810 = v6;
                nat_sub(a1809, a1810)
            }, v3)?))
        } else {
            let v8: u64 = {
                let a1813 = {
                    let a1811 = v1;
                    let a1812 = v6;
                    nat_sub(a1811, a1812)
                };
                let a1814 = v7;
                nat_sub(a1813, a1814)
            };
            let v9: u64 = {
                let a1816 = 1u64;
                let a1817 = f158(v0, match v2.clone() {
                    Adt3::C0(h1815, _, _, _, _, _, _, _) => {
                        h1815
                    }
                })?;
                nat_add(a1816, a1817)?
            };
            let v10: u64 = {
                let a1818 = v8;
                let a1819 = v9;
                let a1820 = 0u64;
                nat_quot(a1818, a1819, a1820)
            };
            let v11: u64 = {
                let a1821 = v8;
                let a1822 = v9;
                let a1823 = 0u64;
                nat_rem(a1821, a1822, a1823)
            };
            let a1824 = v11;
            let a1825 = 0u64;
            if nat_eq(a1824, a1825) {
                Ok(Adt9::C4(19u64, 4u64, f79(v10, v4)?, 2u64, 0u64, 1u64, 2u64))
            } else {
                if f159(v0) {
                    let a1829 = {
                        let a1826 = v11;
                        let a1827 = 2u64;
                        let a1828 = 0u64;
                        nat_rem(a1826, a1827, a1828)
                    };
                    let a1830 = 1u64;
                    if nat_eq(a1829, a1830) {
                        Ok(Adt9::C4(6u64, 0u64, f79({
                            let a1839 = {
                                let a1832 = v10;
                                let a1833 = match v2.clone() {
                                    Adt3::C0(h1831, _, _, _, _, _, _, _) => {
                                        h1831
                                    }
                                };
                                nat_mul(a1832, a1833)?
                            };
                            let a1840 = {
                                let a1836 = {
                                    let a1834 = v11;
                                    let a1835 = 1u64;
                                    nat_sub(a1834, a1835)
                                };
                                let a1837 = 2u64;
                                let a1838 = 0u64;
                                nat_quot(a1836, a1837, a1838)
                            };
                            nat_add(a1839, a1840)?
                        }, v2)?, 2u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(4u64, 3u64, f79({
                            let a1849 = {
                                let a1842 = v5;
                                let a1843 = match v2.clone() {
                                    Adt3::C0(h1841, _, _, _, _, _, _, _) => {
                                        h1841
                                    }
                                };
                                nat_mul(a1842, a1843)?
                            };
                            let a1850 = {
                                let a1846 = {
                                    let a1844 = v11;
                                    let a1845 = 1u64;
                                    nat_sub(a1844, a1845)
                                };
                                let a1847 = 2u64;
                                let a1848 = 0u64;
                                nat_quot(a1846, a1847, a1848)
                            };
                            nat_add(a1849, a1850)?
                        }, v3.clone())?, 1u64, 0u64, 3u64, f79({
                            let a1859 = {
                                let a1852 = v5;
                                let a1853 = match v2 {
                                    Adt3::C0(h1851, _, _, _, _, _, _, _) => {
                                        h1851
                                    }
                                };
                                nat_mul(a1852, a1853)?
                            };
                            let a1860 = {
                                let a1856 = {
                                    let a1854 = v11;
                                    let a1855 = 1u64;
                                    nat_sub(a1854, a1855)
                                };
                                let a1857 = 2u64;
                                let a1858 = 0u64;
                                nat_quot(a1856, a1857, a1858)
                            };
                            nat_add(a1859, a1860)?
                        }, v3)?))
                    }
                } else {
                    f160(v0, v10, {
                        let a1861 = v11;
                        let a1862 = 1u64;
                        nat_sub(a1861, a1862)
                    }, v5, v2, v3)
                }
            }
        }
    }
}

pub fn f66(v0: u64, v1: u64, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let a1863 = v1;
    let a1864 = f161(v2.clone(), v4.clone())?;
    if nat_lt(a1863, a1864) {
        let a1867 = v1;
        let a1868 = {
            let a1865 = 2u64;
            let a1866 = f162(v4.clone())?;
            nat_mul(a1865, a1866)?
        };
        if nat_lt(a1867, a1868) {
            let a1872 = {
                let a1869 = v1;
                let a1870 = 2u64;
                let a1871 = 0u64;
                nat_rem(a1869, a1870, a1871)
            };
            let a1873 = 0u64;
            if nat_eq(a1872, a1873) {
                f163({
                    let a1874 = v1;
                    let a1875 = 2u64;
                    let a1876 = 0u64;
                    nat_quot(a1874, a1875, a1876)
                }, v4)
            } else {
                if f164(match v2 {
                    Adt3::C0(_, h1877, _, _, _, _, _, _) => {
                        h1877
                    }
                }, v5) {
                    Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(Adt9::C0(Adt10::C1))
                }
            }
        } else {
            let v6: u64 = {
                let a1880 = v1;
                let a1881 = {
                    let a1878 = 2u64;
                    let a1879 = f162(v4.clone())?;
                    nat_mul(a1878, a1879)?
                };
                nat_sub(a1880, a1881)
            };
            let v7: u64 = {
                let a1885 = v6;
                let a1886 = {
                    let a1883 = 1u64;
                    let a1884 = match v2.clone() {
                        Adt3::C0(h1882, _, _, _, _, _, _, _) => {
                            h1882
                        }
                    };
                    nat_add(a1883, a1884)?
                };
                let a1887 = 0u64;
                nat_quot(a1885, a1886, a1887)
            };
            let v8: u64 = {
                let a1891 = v6;
                let a1892 = {
                    let a1889 = 1u64;
                    let a1890 = match v2.clone() {
                        Adt3::C0(h1888, _, _, _, _, _, _, _) => {
                            h1888
                        }
                    };
                    nat_add(a1889, a1890)?
                };
                let a1893 = 0u64;
                nat_rem(a1891, a1892, a1893)
            };
            let a1894 = v8;
            let a1895 = 0u64;
            if nat_eq(a1894, a1895) {
                f163(v7, v4)
            } else {
                f165(v0, v7, {
                    let a1896 = v8;
                    let a1897 = 1u64;
                    nat_sub(a1896, a1897)
                }, v2, v3, v4, v5)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f67(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: Adt3, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a1900 = match v4.clone() {
            Adt3::C0(h1898, _, _, _, _, _, _, _) => {
                h1898
            }
        };
        let a1901 = match v4.clone() {
            Adt3::C0(_, h1899, _, _, _, _, _, _) => {
                h1899
            }
        };
        nat_mul(a1900, a1901)?
    };
    let v7: u64 = {
        let a1902 = 2u64;
        let a1903 = v6;
        nat_mul(a1902, a1903)?
    };
    let v8: u64 = f166(match v3.clone() {
        Adt3::C0(_, h1904, _, _, _, _, _, _) => {
            h1904
        }
    }, match v3.clone() {
        Adt3::C0(_, _, h1905, _, _, _, _, _) => {
            h1905
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, h1906, _, _, _, _) => {
            h1906
        }
    });
    let a1907 = v0;
    let a1908 = v7;
    if nat_lt(a1907, a1908) {
        let a1912 = {
            let a1909 = v0;
            let a1910 = 2u64;
            let a1911 = 0u64;
            nat_rem(a1909, a1910, a1911)
        };
        let a1913 = 0u64;
        if nat_eq(a1912, a1913) {
            Ok(Adt9::C4(19u64, 5u64, f79({
                let a1914 = v0;
                let a1915 = 2u64;
                let a1916 = 0u64;
                nat_quot(a1914, a1915, a1916)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f167(v5, v8) {
                Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    } else {
        let v9: u64 = {
            let a1917 = v0;
            let a1918 = v7;
            nat_sub(a1917, a1918)
        };
        let v10: u64 = {
            let a1922 = v9;
            let a1923 = {
                let a1920 = 1u64;
                let a1921 = match v1.clone() {
                    Adt3::C0(h1919, _, _, _, _, _, _, _) => {
                        h1919
                    }
                };
                nat_add(a1920, a1921)?
            };
            let a1924 = 0u64;
            nat_quot(a1922, a1923, a1924)
        };
        let v11: u64 = {
            let a1928 = v9;
            let a1929 = {
                let a1926 = 1u64;
                let a1927 = match v1.clone() {
                    Adt3::C0(h1925, _, _, _, _, _, _, _) => {
                        h1925
                    }
                };
                nat_add(a1926, a1927)?
            };
            let a1930 = 0u64;
            nat_rem(a1928, a1929, a1930)
        };
        let a1931 = v11;
        let a1932 = 0u64;
        if nat_eq(a1931, a1932) {
            Ok(Adt9::C4(19u64, 5u64, f79({
                let a1933 = v10;
                let a1934 = v6;
                let a1935 = 0u64;
                nat_rem(a1933, a1934, a1935)
            }, v4)?, 2u64, 0u64, 1u64, 2u64))
        } else {
            if f167(v5, v8) {
                let v12: u64 = {
                    let a1941 = {
                        let a1937 = v10;
                        let a1938 = match v1.clone() {
                            Adt3::C0(h1936, _, _, _, _, _, _, _) => {
                                h1936
                            }
                        };
                        nat_mul(a1937, a1938)?
                    };
                    let a1942 = {
                        let a1939 = v11;
                        let a1940 = 1u64;
                        nat_sub(a1939, a1940)
                    };
                    nat_add(a1941, a1942)?
                };
                let a1943 = v5;
                let a1944 = 4294967295u64;
                if nat_eq(a1943, a1944) {
                    Ok(Adt9::C1(false, f79(v12, v1)?, f79(v12, v2)?))
                } else {
                    Ok(Adt9::C4(8u64, 0u64, f79(v12, v1)?, 4u64, f79({
                        let a1950 = {
                            let a1946 = v5;
                            let a1947 = match v3.clone() {
                                Adt3::C0(h1945, _, _, _, _, _, _, _) => {
                                    h1945
                                }
                            };
                            nat_mul(a1946, a1947)?
                        };
                        let a1951 = {
                            let a1948 = v11;
                            let a1949 = 1u64;
                            nat_sub(a1948, a1949)
                        };
                        nat_add(a1950, a1951)?
                    }, v3)?, 3u64, f79(v12, v2)?))
                }
            } else {
                Ok(Adt9::C0(Adt10::C1))
            }
        }
    }
}

pub fn f68(v0: u64, v1: Adt3, v2: Adt3, v3: Adt7, v4: u64) -> R<Adt9> {
    let v5: u64 = f168(v3.clone())?;
    let v6: u64 = {
        let a1952 = v0;
        let a1953 = v5;
        let a1954 = 0u64;
        nat_quot(a1952, a1953, a1954)
    };
    let v7: u64 = {
        let a1955 = v0;
        let a1956 = v5;
        let a1957 = 0u64;
        nat_rem(a1955, a1956, a1957)
    };
    let v8: u64 = f169(v3.clone())?;
    let a1958 = v7;
    let a1959 = 0u64;
    if nat_eq(a1958, a1959) {
        Ok(Adt9::C4(19u64, 2u64, if match v3 {
            Adt7::C0(_, _, _, h1960, _) => {
                h1960
            }
        } {
            4286578687u64
        } else {
            0u64
        }, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a1964 = v5;
        let a1965 = {
            let a1962 = v7;
            let a1963 = if match v3.clone() {
                Adt7::C0(_, _, _, h1961, _) => {
                    h1961
                }
            } {
                1u64
            } else {
                2u64
            };
            nat_add(a1962, a1963)?
        };
        if nat_le(a1964, a1965) {
            if match v3.clone() {
                Adt7::C0(_, _, _, h1966, _) => {
                    h1966
                }
            } {
                Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f79(v6, v2)?))
            } else {
                let a1969 = {
                    let a1967 = v7;
                    let a1968 = 2u64;
                    nat_add(a1967, a1968)?
                };
                let a1970 = v5;
                if nat_eq(a1969, a1970) {
                    let v9: u64 = if match v3.clone() {
                        Adt7::C0(_, _, h1971, _, _) => {
                            h1971
                        }
                    } {
                        let a1978 = {
                            let m1974 = match v3.clone() {
                                Adt7::C0(h1973, _, _, _, _) => {
                                    h1973
                                }
                            };
                            match m1974 {
                                Adt6::C0(h1972, _, _, _) => {
                                    h1972
                                }
                            }
                        };
                        let a1979 = {
                            let m1977 = match v3 {
                                Adt7::C0(_, h1976, _, _, _) => {
                                    h1976
                                }
                            };
                            match m1977 {
                                Adt6::C0(h1975, _, _, _) => {
                                    h1975
                                }
                            }
                        };
                        nat_mul(a1978, a1979)?
                    } else {
                        f170(match v3 {
                            Adt7::C0(h1980, _, _, _, _) => {
                                h1980
                            }
                        }, match v1 {
                            Adt3::C0(h1981, _, _, _, _, _, _, _) => {
                                h1981
                            }
                        }, f50(v6, v2, 0u64))?
                    };
                    Ok(Adt9::C4(3u64, 2u64, {
                        let a1982 = v9;
                        let a1983 = 0u64;
                        if nat_eq(a1982, a1983) {
                            1u64
                        } else {
                            v9
                        }
                    }, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(5u64, 1u64, 0u64, 1u64, 1u64, 3u64, f79(v6, v2)?))
                }
            }
        } else {
            let v10: u64 = {
                let a1986 = {
                    let a1984 = v7;
                    let a1985 = 1u64;
                    nat_sub(a1984, a1985)
                };
                let a1987 = v8;
                let a1988 = 0u64;
                nat_rem(a1986, a1987, a1988)
            };
            let m1994 = f171(v3.clone(), v1, v2, v6, {
                let a1991 = {
                    let a1989 = v7;
                    let a1990 = 1u64;
                    nat_sub(a1989, a1990)
                };
                let a1992 = v8;
                let a1993 = 0u64;
                nat_quot(a1991, a1992, a1993)
            })?;
            match m1994 {
                None => {
                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                }
                Some(v11) => {
                    let a1996 = match v3.clone() {
                        Adt7::C0(_, _, _, _, h1995) => {
                            h1995
                        }
                    };
                    let a1997 = 1u64;
                    if if nat_eq(a1996, a1997) {
                        let a1998 = v10;
                        let a1999 = 0u64;
                        nat_eq(a1998, a1999)
                    } else {
                        false
                    } {
                        Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        if match v3.clone() {
                            Adt7::C0(_, _, _, h2000, _) => {
                                h2000
                            }
                        } {
                            let a2003 = {
                                let a2001 = v10;
                                let a2002 = 1u64;
                                nat_add(a2001, a2002)?
                            };
                            let a2004 = v8;
                            if nat_eq(a2003, a2004) {
                                let a2005 = v4;
                                let a2006 = 2u64;
                                if nat_eq(a2005, a2006) {
                                    Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                                } else {
                                    Ok(Adt9::C4(19u64, {
                                        let a2008 = match v3.clone() {
                                            Adt7::C0(_, _, _, _, h2007) => {
                                                h2007
                                            }
                                        };
                                        let a2009 = 1u64;
                                        if nat_eq(a2008, a2009) {
                                            1u64
                                        } else {
                                            0u64
                                        }
                                    }, {
                                        let a2011 = match v3 {
                                            Adt7::C0(_, _, _, _, h2010) => {
                                                h2010
                                            }
                                        };
                                        let a2012 = 1u64;
                                        if nat_eq(a2011, a2012) {
                                            1u64
                                        } else {
                                            v11
                                        }
                                    }, 2u64, 0u64, 1u64, 0u64))
                                }
                            } else {
                                Ok(Adt9::C4(16u64, 1u64, 0u64, {
                                    let a2014 = match v3.clone() {
                                        Adt7::C0(_, _, _, _, h2013) => {
                                            h2013
                                        }
                                    };
                                    let a2015 = 1u64;
                                    if nat_eq(a2014, a2015) {
                                        1u64
                                    } else {
                                        0u64
                                    }
                                }, {
                                    let a2017 = match v3 {
                                        Adt7::C0(_, _, _, _, h2016) => {
                                            h2016
                                        }
                                    };
                                    let a2018 = 1u64;
                                    if nat_eq(a2017, a2018) {
                                        1u64
                                    } else {
                                        v11
                                    }
                                }, 1u64, 2u64))
                            }
                        } else {
                            Ok(Adt9::C4(4u64, 1u64, 0u64, {
                                let a2020 = match v3.clone() {
                                    Adt7::C0(_, _, _, _, h2019) => {
                                        h2019
                                    }
                                };
                                let a2021 = 1u64;
                                if nat_eq(a2020, a2021) {
                                    1u64
                                } else {
                                    0u64
                                }
                            }, {
                                let a2023 = match v3 {
                                    Adt7::C0(_, _, _, _, h2022) => {
                                        h2022
                                    }
                                };
                                let a2024 = 1u64;
                                if nat_eq(a2023, a2024) {
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

pub fn f69(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = f172(match v3.clone() {
        Adt3::C0(h2025, _, _, _, _, _, _, _) => {
            h2025
        }
    })?;
    let v5: u64 = {
        let a2026 = v0;
        let a2027 = v4;
        let a2028 = 0u64;
        nat_rem(a2026, a2027, a2028)
    };
    let v6: u64 = {
        let a2029 = v0;
        let a2030 = v4;
        let a2031 = 0u64;
        nat_quot(a2029, a2030, a2031)
    };
    let a2032 = v5;
    let a2033 = 0u64;
    if nat_eq(a2032, a2033) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2036 = {
            let a2034 = v5;
            let a2035 = 1u64;
            nat_add(a2034, a2035)?
        };
        let a2037 = v4;
        if nat_eq(a2036, a2037) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 3u64, f79(v6, v2)?))
        } else {
            let a2041 = {
                let a2038 = v5;
                let a2039 = 2u64;
                let a2040 = 0u64;
                nat_rem(a2038, a2039, a2040)
            };
            let a2042 = 0u64;
            if nat_eq(a2041, a2042) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2045 = {
                        let a2043 = v5;
                        let a2044 = 1u64;
                        nat_sub(a2043, a2044)
                    };
                    let a2046 = 2u64;
                    let a2047 = 0u64;
                    nat_quot(a2045, a2046, a2047)
                };
                let v8: u64 = f50(v6, v2.clone(), 0u64);
                let v9: u64 = f50(v6, v2.clone(), 1u64);
                let v10: u64 = f50(v6, v2, 2u64);
                Ok(Adt9::C4(10u64, 0u64, f49(v1, {
                    let a2048 = v9;
                    let a2049 = v7;
                    nat_add(a2048, a2049)?
                }, v8, v10, 0u64)?, 4u64, f49(v3, v7, v8, 0u64, 0u64)?, 1u64, 1u64))
            }
        }
    }
}

pub fn f70(v0: u64, v1: Adt3, v2: Adt3, v3: Adt6) -> R<Adt9> {
    let v4: u64 = f173(v3.clone())?;
    let v5: u64 = {
        let a2050 = v0;
        let a2051 = v4;
        let a2052 = 0u64;
        nat_rem(a2050, a2051, a2052)
    };
    let v6: u64 = {
        let a2053 = v0;
        let a2054 = v4;
        let a2055 = 0u64;
        nat_quot(a2053, a2054, a2055)
    };
    let a2056 = v5;
    let a2057 = 0u64;
    if nat_eq(a2056, a2057) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a2060 = {
            let a2058 = v5;
            let a2059 = 1u64;
            nat_add(a2058, a2059)?
        };
        let a2061 = v4;
        if nat_eq(a2060, a2061) {
            Ok(Adt9::C4(7u64, 1u64, 0u64, 2u64, 0u64, 3u64, f79(v6, v2)?))
        } else {
            let a2065 = {
                let a2062 = v5;
                let a2063 = 2u64;
                let a2064 = 0u64;
                nat_rem(a2062, a2063, a2064)
            };
            let a2066 = 0u64;
            if nat_eq(a2065, a2066) {
                Ok(Adt9::C4(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                let v7: u64 = {
                    let a2068 = f50(v6, v2.clone(), 0u64);
                    let a2069 = match v3.clone() {
                        Adt6::C0(_, _, h2067, _) => {
                            h2067
                        }
                    };
                    nat_add(a2068, a2069)?
                };
                let v8: u64 = {
                    let a2077 = {
                        let a2074 = {
                            let a2071 = v7;
                            let a2072 = match v3.clone() {
                                Adt6::C0(_, h2070, _, _) => {
                                    h2070
                                }
                            };
                            nat_add(a2071, a2072)?
                        };
                        let a2075 = match v3.clone() {
                            Adt6::C0(h2073, _, _, _) => {
                                h2073
                            }
                        };
                        nat_sub(a2074, a2075)
                    };
                    let a2078 = match v3.clone() {
                        Adt6::C0(_, h2076, _, _) => {
                            h2076
                        }
                    };
                    let a2079 = 0u64;
                    nat_quot(a2077, a2078, a2079)
                };
                let v9: u64 = {
                    let a2085 = v8;
                    let a2086 = {
                        let a2082 = {
                            let a2080 = v5;
                            let a2081 = 1u64;
                            nat_sub(a2080, a2081)
                        };
                        let a2083 = 2u64;
                        let a2084 = 0u64;
                        nat_quot(a2082, a2083, a2084)
                    };
                    nat_add(a2085, a2086)?
                };
                let v10: u64 = {
                    let a2088 = v9;
                    let a2089 = match v3.clone() {
                        Adt6::C0(_, h2087, _, _) => {
                            h2087
                        }
                    };
                    nat_mul(a2088, a2089)?
                };
                let v11: u64 = {
                    let a2090 = v7;
                    let a2091 = v10;
                    nat_sub(a2090, a2091)
                };
                let a2093 = v9;
                let a2094 = match v1.clone() {
                    Adt3::C0(_, h2092, _, _, _, _, _, _) => {
                        h2092
                    }
                };
                if if if nat_lt(a2093, a2094) {
                    let a2095 = v10;
                    let a2096 = v7;
                    nat_le(a2095, a2096)
                } else {
                    false
                } {
                    let a2098 = v11;
                    let a2099 = match v3.clone() {
                        Adt6::C0(h2097, _, _, _) => {
                            h2097
                        }
                    };
                    nat_lt(a2098, a2099)
                } else {
                    false
                } {
                    Ok(Adt9::C4(6u64, 0u64, f49(v1, {
                        let a2103 = {
                            let a2101 = f50(v6, v2, 1u64);
                            let a2102 = match v3 {
                                Adt6::C0(h2100, _, _, _) => {
                                    h2100
                                }
                            };
                            nat_mul(a2101, a2102)?
                        };
                        let a2104 = v11;
                        nat_add(a2103, a2104)?
                    }, v9, 0u64, 0u64)?, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 1u64))
                }
            }
        }
    }
}

pub fn f71(v0: u64, v1: Adt3, v2: Adt3, v3: Adt5) -> R<Adt9> {
    let v4: u64 = f50(v0, v2.clone(), 0u64);
    let v5: u64 = f50(v0, v2.clone(), 3u64);
    let v6: u64 = {
        let a2106 = v5;
        let a2107 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2105, _) => {
                h2105
            }
        };
        let a2108 = 0u64;
        nat_rem(a2106, a2107, a2108)
    };
    let v7: u64 = {
        let a2110 = v5;
        let a2111 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, _, h2109, _) => {
                h2109
            }
        };
        let a2112 = 0u64;
        nat_quot(a2110, a2111, a2112)
    };
    let v8: Option<u64> = f52(match v3.clone() {
        Adt5::C0(h2113, _, _, _, _, _, _, _) => {
            h2113
        }
    }, match v1.clone() {
        Adt3::C0(h2114, _, _, _, _, _, _, _) => {
            h2114
        }
    }, f50(v0, v2.clone(), 1u64), {
        let a2118 = v4;
        let a2119 = {
            let m2117 = match v3.clone() {
                Adt5::C0(h2116, _, _, _, _, _, _, _) => {
                    h2116
                }
            };
            match m2117 {
                Adt6::C0(h2115, _, _, _) => {
                    h2115
                }
            }
        };
        let a2120 = 0u64;
        nat_rem(a2118, a2119, a2120)
    })?;
    let v9: u64 = {
        let a2124 = v4;
        let a2125 = {
            let m2123 = match v3.clone() {
                Adt5::C0(h2122, _, _, _, _, _, _, _) => {
                    h2122
                }
            };
            match m2123 {
                Adt6::C0(h2121, _, _, _) => {
                    h2121
                }
            }
        };
        let a2126 = 0u64;
        nat_quot(a2124, a2125, a2126)
    };
    let v10: Option<u64> = f52(match v3.clone() {
        Adt5::C0(_, h2127, _, _, _, _, _, _) => {
            h2127
        }
    }, match v1.clone() {
        Adt3::C0(_, h2128, _, _, _, _, _, _) => {
            h2128
        }
    }, f50(v0, v2.clone(), 2u64), {
        let a2132 = v9;
        let a2133 = {
            let m2131 = match v3.clone() {
                Adt5::C0(_, h2130, _, _, _, _, _, _) => {
                    h2130
                }
            };
            match m2131 {
                Adt6::C0(h2129, _, _, _) => {
                    h2129
                }
            }
        };
        let a2134 = 0u64;
        nat_rem(a2132, a2133, a2134)
    })?;
    let v11: u64 = {
        let a2138 = v9;
        let a2139 = {
            let m2137 = match v3.clone() {
                Adt5::C0(_, h2136, _, _, _, _, _, _) => {
                    h2136
                }
            };
            match m2137 {
                Adt6::C0(h2135, _, _, _) => {
                    h2135
                }
            }
        };
        let a2140 = 0u64;
        nat_quot(a2138, a2139, a2140)
    };
    let v12: Option<u64> = f52(match v3.clone() {
        Adt5::C0(_, _, h2141, _, _, _, _, _) => {
            h2141
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h2142, _, _, _, _, _) => {
            h2142
        }
    }, v6, {
        let a2146 = v11;
        let a2147 = {
            let m2145 = match v3.clone() {
                Adt5::C0(_, _, h2144, _, _, _, _, _) => {
                    h2144
                }
            };
            match m2145 {
                Adt6::C0(h2143, _, _, _) => {
                    h2143
                }
            }
        };
        let a2148 = 0u64;
        nat_rem(a2146, a2147, a2148)
    })?;
    let v13: u64 = {
        let a2152 = v11;
        let a2153 = {
            let m2151 = match v3.clone() {
                Adt5::C0(_, _, h2150, _, _, _, _, _) => {
                    h2150
                }
            };
            match m2151 {
                Adt6::C0(h2149, _, _, _) => {
                    h2149
                }
            }
        };
        let a2154 = 0u64;
        nat_quot(a2152, a2153, a2154)
    };
    let v17: Adt12 = {
        let m2155 = v8;
        match m2155 {
            None => {
                Adt12::C2(0u64)
            }
            Some(v14) => {
                let m2156 = v10;
                match m2156 {
                    None => {
                        Adt12::C2(0u64)
                    }
                    Some(v15) => {
                        let m2157 = v12;
                        match m2157 {
                            None => {
                                Adt12::C2(0u64)
                            }
                            Some(v16) => {
                                Adt12::C0(1u64, {
                                    let a2166 = {
                                        let a2161 = f49(v1, v14, v15, v16, 0u64)?;
                                        let a2162 = {
                                            let a2159 = v7;
                                            let a2160 = match v3.clone() {
                                                Adt5::C0(_, _, _, _, h2158, _, _, _) => {
                                                    h2158
                                                }
                                            };
                                            nat_mul(a2159, a2160)?
                                        };
                                        nat_add(a2161, a2162)?
                                    };
                                    let a2167 = {
                                        let a2164 = v13;
                                        let a2165 = match v3.clone() {
                                            Adt5::C0(_, _, _, h2163, _, _, _, _) => {
                                                h2163
                                            }
                                        };
                                        nat_mul(a2164, a2165)?
                                    };
                                    nat_add(a2166, a2167)?
                                })
                            }
                        }
                    }
                }
            }
        }
    };
    Ok(Adt9::C7(Adt11::C2(match v3.clone() {
        Adt5::C0(_, _, _, _, _, h2168, _, _) => {
            h2168
        }
    }, match v3.clone() {
        Adt5::C0(_, _, _, _, _, _, _, h2169) => {
            h2169
        }
    }, true), v17, Adt12::C2(0u64), Adt13::C1(f79(v0, v2)?), {
        let a2171 = match v3.clone() {
            Adt5::C0(_, _, _, _, _, h2170, _, _) => {
                h2170
            }
        };
        let a2172 = 0u64;
        if nat_eq(a2171, a2172) {
            4u64
        } else {
            2u64
        }
    }, {
        let a2174 = match v3 {
            Adt5::C0(_, _, _, _, _, _, _, h2173) => {
                h2173
            }
        };
        let a2175 = 0u64;
        if nat_eq(a2174, a2175) {
            4u64
        } else {
            2u64
        }
    }))
}

pub fn f72(v0: Adt16, v1: u64, v2: Adt3, v3: Adt3, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64) -> R<Adt9> {
    let m2188 = f174(v0, v1, match v2.clone() {
        Adt3::C0(h2176, _, _, _, _, _, _, _) => {
            h2176
        }
    }, match v2.clone() {
        Adt3::C0(_, h2177, _, _, _, _, _, _) => {
            h2177
        }
    }, match v2.clone() {
        Adt3::C0(_, _, h2178, _, _, _, _, _) => {
            h2178
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, h2179, _, _, _, _) => {
            h2179
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, h2180, _, _, _) => {
            h2180
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, h2181, _, _) => {
            h2181
        }
    }, match v2.clone() {
        Adt3::C0(_, _, _, _, _, _, h2182, _) => {
            h2182
        }
    }, match v2 {
        Adt3::C0(_, _, _, _, _, _, _, h2183) => {
            h2183
        }
    }, v6, v7, v8, v9, match v3.clone() {
        Adt3::C0(_, _, _, _, h2184, _, _, _) => {
            h2184
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, h2185, _, _) => {
            h2185
        }
    }, match v3.clone() {
        Adt3::C0(_, _, _, _, _, _, h2186, _) => {
            h2186
        }
    }, match v3 {
        Adt3::C0(_, _, _, _, _, _, _, h2187) => {
            h2187
        }
    }, v4)?;
    match m2188 {
        Adt17::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt17::C1(v10, v11, v12, v13, v14, v15, v16, v17) => {
            Ok(f17(false, v10, v11, 0u64, 0u64, v12, v13, v14, v15, v16, v17, v5, v5))
        }
    }
}

pub fn f73(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt9> {
    let v8: u64 = {
        let a2191 = v6;
        let a2192 = {
            let a2189 = 3u64;
            let a2190 = v5;
            nat_add(a2189, a2190)?
        };
        nat_sub(a2191, a2192)
    };
    let a2194 = v0;
    let a2195 = f42(v3, match v1.clone() {
        Adt3::C0(h2193, _, _, _, _, _, _, _) => {
            h2193
        }
    }, v5, v8)?;
    if nat_lt(a2194, a2195) {
        let v9: u64 = f88(match v1.clone() {
            Adt3::C0(h2196, _, _, _, _, _, _, _) => {
                h2196
            }
        }, v5, v8)?;
        let v10: u64 = f89(v3, match v1.clone() {
            Adt3::C0(h2197, _, _, _, _, _, _, _) => {
                h2197
            }
        }, v5)?;
        let a2198 = v0;
        let a2199 = v9;
        if nat_lt(a2198, a2199) {
            f175(v0, v2, f36(v3), {
                let a2200 = 3u64;
                let a2201 = v5;
                nat_add(a2200, a2201)?
            }, v7)
        } else {
            let a2204 = v0;
            let a2205 = {
                let a2202 = v9;
                let a2203 = v10;
                nat_add(a2202, a2203)?
            };
            if nat_lt(a2204, a2205) {
                let v11: u64 = {
                    let a2206 = v0;
                    let a2207 = v9;
                    nat_sub(a2206, a2207)
                };
                let v12: u64 = f176(v3, match v1.clone() {
                    Adt3::C0(h2208, _, _, _, _, _, _, _) => {
                        h2208
                    }
                })?;
                f177({
                    let a2209 = v11;
                    let a2210 = v12;
                    let a2211 = 0u64;
                    nat_rem(a2209, a2210, a2211)
                }, {
                    let a2212 = v11;
                    let a2213 = v12;
                    let a2214 = 0u64;
                    nat_quot(a2212, a2213, a2214)
                }, v1, v3, v4, v7)
            } else {
                f178({
                    let a2217 = v0;
                    let a2218 = {
                        let a2215 = v9;
                        let a2216 = v10;
                        nat_add(a2215, a2216)?
                    };
                    nat_sub(a2217, a2218)
                }, v1, v2, v3)
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f74(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let a2219 = v0;
    let a2220 = f28(v1)?;
    if nat_lt(a2219, a2220) {
        let a2221 = v0;
        let a2222 = 4u64;
        if nat_lt(a2221, a2222) {
            let a2223 = v0;
            let a2224 = 0u64;
            if nat_eq(a2223, a2224) {
                Ok(Adt9::C4(16u64, 2u64, v2, 2u64, v3, 1u64, 2u64))
            } else {
                let a2225 = v0;
                let a2226 = 1u64;
                if nat_eq(a2225, a2226) {
                    let a2227 = v5;
                    let a2228 = 0u64;
                    if nat_eq(a2227, a2228) {
                        Ok(Adt9::C4(9u64, 2u64, v3, 2u64, v2, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C0(Adt10::C1))
                    }
                } else {
                    let a2229 = v0;
                    let a2230 = 2u64;
                    if nat_eq(a2229, a2230) {
                        Ok(Adt9::C4(11u64, 1u64, 2u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        Ok(Adt9::C4(20u64, 1u64, 2u64, 2u64, 10u64, 1u64, 2u64))
                    }
                }
            }
        } else {
            let a2231 = v0;
            let a2232 = 4u64;
            if if nat_eq(a2231, a2232) {
                let a2233 = f179(v5)?;
                let a2234 = v1;
                !nat_eq(a2233, a2234)
            } else {
                false
            } {
                Ok(Adt9::C0(Adt10::C1))
            } else {
                let v6: u64 = {
                    let a2237 = {
                        let a2235 = v0;
                        let a2236 = 4u64;
                        nat_sub(a2235, a2236)
                    };
                    let a2238 = 3u64;
                    let a2239 = 0u64;
                    nat_quot(a2237, a2238, a2239)
                };
                let v7: u64 = {
                    let a2242 = {
                        let a2240 = v0;
                        let a2241 = 4u64;
                        nat_sub(a2240, a2241)
                    };
                    let a2243 = 3u64;
                    let a2244 = 0u64;
                    nat_rem(a2242, a2243, a2244)
                };
                let a2245 = v7;
                let a2246 = 0u64;
                if nat_eq(a2245, a2246) {
                    Ok(Adt9::C4(3u64, 2u64, v6, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a2247 = v7;
                    let a2248 = 1u64;
                    if nat_eq(a2247, a2248) {
                        Ok(Adt9::C4(10u64, 2u64, v4, 1u64, 0u64, 1u64, 0u64))
                    } else {
                        Ok(Adt9::C4(8u64, 2u64, v2, 1u64, 0u64, 3u64, f27(v6)?))
                    }
                }
            }
        }
    } else {
        Ok(Adt9::C0(Adt10::C1))
    }
}

pub fn f75(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if if if if v0 {
        let a2249 = v1;
        let a2250 = 5u64;
        nat_eq(a2249, a2250)
    } else {
        false
    } {
        let a2251 = v2;
        let a2252 = 16u64;
        nat_eq(a2251, a2252)
    } else {
        false
    } {
        let a2253 = v3;
        let a2254 = 0u64;
        nat_eq(a2253, a2254)
    } else {
        false
    } {
        let a2255 = v4;
        let a2256 = 0u64;
        nat_eq(a2255, a2256)
    } else {
        false
    }
}

pub fn f76(v0: bool, v1: u64, v2: u64, v3: u64, v4: u64) -> Adt11 {
    let a2257 = v1;
    let a2258 = 0u64;
    if nat_eq(a2257, a2258) {
        if v0 {
            let a2259 = v3;
            let a2260 = 0u64;
            if if if nat_eq(a2259, a2260) {
                let a2261 = v4;
                let a2262 = 43u64;
                nat_eq(a2261, a2262)
            } else {
                false
            } {
                true
            } else {
                let a2263 = v3;
                let a2264 = 43u64;
                if nat_eq(a2263, a2264) {
                    let a2265 = v4;
                    let a2266 = 0u64;
                    nat_eq(a2265, a2266)
                } else {
                    false
                }
            } {
                f151(v3, v4)
            } else {
                Adt11::C2(v3, v4, false)
            }
        } else {
            Adt11::C0(v2)
        }
    } else {
        let a2267 = v1;
        let a2268 = 1u64;
        if nat_eq(a2267, a2268) {
            Adt11::C1(v2)
        } else {
            let a2269 = v1;
            let a2270 = 2u64;
            if nat_eq(a2269, a2270) {
                Adt11::C5(v2)
            } else {
                let a2271 = v1;
                let a2272 = 3u64;
                if nat_eq(a2271, a2272) {
                    Adt11::C3(v2, v2, false)
                } else {
                    let a2273 = v1;
                    let a2274 = 4u64;
                    if nat_eq(a2273, a2274) {
                        Adt11::C2(v3, v4, false)
                    } else {
                        if f75(v0, v1, v2, v3, v4) {
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

pub fn f77(v0: u64, v1: u64) -> Adt12 {
    let a2275 = v0;
    let a2276 = 0u64;
    if nat_eq(a2275, a2276) {
        Adt12::C0(0u64, v1)
    } else {
        let a2277 = v0;
        let a2278 = 1u64;
        if nat_eq(a2277, a2278) {
            Adt12::C1(v1)
        } else {
            let a2279 = v0;
            let a2280 = 2u64;
            if nat_eq(a2279, a2280) {
                Adt12::C2(v1)
            } else {
                let a2281 = v0;
                let a2282 = 3u64;
                if nat_eq(a2281, a2282) {
                    Adt12::C3(v1)
                } else {
                    let a2283 = v0;
                    let a2284 = 13u64;
                    if nat_lt(a2283, a2284) {
                        Adt12::C0({
                            let a2285 = v0;
                            let a2286 = 3u64;
                            nat_sub(a2285, a2286)
                        }, v1)
                    } else {
                        Adt12::C4
                    }
                }
            }
        }
    }
}

pub fn f78(v0: u64, v1: u64) -> Adt13 {
    let a2287 = v0;
    let a2288 = 1u64;
    if nat_eq(a2287, a2288) {
        Adt13::C0(v1)
    } else {
        let a2289 = v0;
        let a2290 = 3u64;
        if if nat_eq(a2289, a2290) {
            true
        } else {
            let a2291 = v0;
            let a2292 = 6u64;
            nat_eq(a2291, a2292)
        } {
            Adt13::C1(v1)
        } else {
            Adt13::C2
        }
    }
}

pub fn f79(v0: u64, v1: Adt3) -> R<u64> {
    let v2: u64 = {
        let a2294 = v0;
        let a2295 = match v1.clone() {
            Adt3::C0(h2293, _, _, _, _, _, _, _) => {
                h2293
            }
        };
        let a2296 = 0u64;
        nat_quot(a2294, a2295, a2296)
    };
    let v3: u64 = {
        let a2298 = v2;
        let a2299 = match v1.clone() {
            Adt3::C0(_, h2297, _, _, _, _, _, _) => {
                h2297
            }
        };
        let a2300 = 0u64;
        nat_quot(a2298, a2299, a2300)
    };
    f126({
        let a2302 = v0;
        let a2303 = match v1.clone() {
            Adt3::C0(h2301, _, _, _, _, _, _, _) => {
                h2301
            }
        };
        let a2304 = 0u64;
        nat_rem(a2302, a2303, a2304)
    }, {
        let a2306 = v2;
        let a2307 = match v1.clone() {
            Adt3::C0(_, h2305, _, _, _, _, _, _) => {
                h2305
            }
        };
        let a2308 = 0u64;
        nat_rem(a2306, a2307, a2308)
    }, {
        let a2310 = v3;
        let a2311 = match v1.clone() {
            Adt3::C0(_, _, h2309, _, _, _, _, _) => {
                h2309
            }
        };
        let a2312 = 0u64;
        nat_rem(a2310, a2311, a2312)
    }, {
        let a2314 = v3;
        let a2315 = match v1.clone() {
            Adt3::C0(_, _, h2313, _, _, _, _, _) => {
                h2313
            }
        };
        let a2316 = 0u64;
        nat_quot(a2314, a2315, a2316)
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h2317, _, _, _) => {
            h2317
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h2318, _, _) => {
            h2318
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h2319, _) => {
            h2319
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h2320) => {
            h2320
        }
    })
}

pub fn f80(v0: Adt6, v1: u64) -> R<u64> {
    let a2322 = 0u64;
    let a2323 = match v0.clone() {
        Adt6::C0(h2321, _, _, _) => {
            h2321
        }
    };
    if if if if if nat_lt(a2322, a2323) {
        let a2325 = 0u64;
        let a2326 = match v0.clone() {
            Adt6::C0(_, h2324, _, _) => {
                h2324
            }
        };
        nat_lt(a2325, a2326)
    } else {
        false
    } {
        let a2328 = 0u64;
        let a2329 = match v0.clone() {
            Adt6::C0(_, _, _, h2327) => {
                h2327
            }
        };
        nat_lt(a2328, a2329)
    } else {
        false
    } {
        let a2331 = match v0.clone() {
            Adt6::C0(_, h2330, _, _) => {
                h2330
            }
        };
        let a2332 = 2147483648u64;
        nat_lt(a2331, a2332)
    } else {
        false
    } {
        let a2334 = match v0.clone() {
            Adt6::C0(_, _, _, h2333) => {
                h2333
            }
        };
        let a2335 = 2147483648u64;
        nat_lt(a2334, a2335)
    } else {
        false
    } {
        let v2: u64 = {
            let a2339 = v1;
            let a2340 = {
                let a2337 = 2u64;
                let a2338 = f128(match v0.clone() {
                    Adt6::C0(_, _, h2336, _) => {
                        h2336
                    }
                });
                nat_mul(a2337, a2338)?
            };
            nat_add(a2339, a2340)?
        };
        let v3: u64 = {
            let a2352 = {
                let a2347 = 1u64;
                let a2348 = {
                    let a2345 = match v0.clone() {
                        Adt6::C0(_, _, _, h2341) => {
                            h2341
                        }
                    };
                    let a2346 = {
                        let a2343 = match v0.clone() {
                            Adt6::C0(h2342, _, _, _) => {
                                h2342
                            }
                        };
                        let a2344 = 1u64;
                        nat_sub(a2343, a2344)
                    };
                    nat_mul(a2345, a2346)?
                };
                nat_add(a2347, a2348)?
            };
            let a2353 = {
                let a2350 = 2u64;
                let a2351 = f127(match v0.clone() {
                    Adt6::C0(_, _, h2349, _) => {
                        h2349
                    }
                });
                nat_mul(a2350, a2351)?
            };
            nat_add(a2352, a2353)?
        };
        let a2354 = v3;
        let a2355 = v2;
        if nat_le(a2354, a2355) {
            let a2362 = 1u64;
            let a2363 = {
                let a2359 = {
                    let a2356 = v2;
                    let a2357 = v3;
                    nat_sub(a2356, a2357)
                };
                let a2360 = match v0 {
                    Adt6::C0(_, h2358, _, _) => {
                        h2358
                    }
                };
                let a2361 = 0u64;
                nat_quot(a2359, a2360, a2361)
            };
            nat_add(a2362, a2363)
        } else {
            let a2367 = {
                let a2364 = v3;
                let a2365 = v2;
                nat_sub(a2364, a2365)
            };
            let a2368 = match v0 {
                Adt6::C0(_, h2366, _, _) => {
                    h2366
                }
            };
            if nat_lt(a2367, a2368) {
                Ok(1u64)
            } else {
                Ok(0u64)
            }
        }
    } else {
        Ok(0u64)
    }
}

pub fn f81(v0: u64) -> u64 {
    let v1: u64 = {
        let a2369 = v0;
        let a2370 = 4294967296u64;
        let a2371 = 0u64;
        nat_rem(a2369, a2370, a2371)
    };
    let a2372 = v1;
    let a2373 = 2147483648u64;
    if nat_lt(a2372, a2373) {
        v1
    } else {
        0u64
    }
}

pub fn f82(v0: u64) -> R<u64> {
    let a2376 = 2u64;
    let a2377 = {
        let a2374 = 2u64;
        let a2375 = v0;
        nat_mul(a2374, a2375)?
    };
    nat_add(a2376, a2377)
}

pub fn f83(v0: u64, v1: Adt14, v2: Adt14) -> R<Adt15> {
    let a2379 = match v1.clone() {
        Adt14::C0(h2378, _, _, _, _, _, _, _, _, _, _) => {
            h2378
        }
    };
    let a2380 = 0u64;
    if if if if nat_eq(a2379, a2380) {
        let a2382 = match v2.clone() {
            Adt14::C0(h2381, _, _, _, _, _, _, _, _, _, _) => {
                h2381
            }
        };
        let a2383 = 0u64;
        nat_eq(a2382, a2383)
    } else {
        false
    } {
        f180(v1.clone(), v2.clone())
    } else {
        false
    } {
        if f181(match v1.clone() {
            Adt14::C0(_, h2384, _, _, _, _, _, _, _, _, _) => {
                h2384
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2385, _, _, _, _, _, _, _, _) => {
                h2385
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2386, _, _, _, _, _, _, _) => {
                h2386
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2387, _, _, _, _, _, _) => {
                h2387
            }
        }) {
            true
        } else {
            if if if if f37(v0) {
                f182(v1.clone(), v2.clone())
            } else {
                false
            } {
                f183(v1.clone())
            } else {
                false
            } {
                f183(v2.clone())
            } else {
                false
            } {
                if f184(v1.clone(), v2.clone())? {
                    true
                } else {
                    f185(v1.clone(), v2.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v3: u64 = if f181(match v1.clone() {
            Adt14::C0(_, h2388, _, _, _, _, _, _, _, _, _) => {
                h2388
            }
        }, match v1.clone() {
            Adt14::C0(_, _, h2389, _, _, _, _, _, _, _, _) => {
                h2389
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, h2390, _, _, _, _, _, _, _) => {
                h2390
            }
        }, match v1.clone() {
            Adt14::C0(_, _, _, _, h2391, _, _, _, _, _, _) => {
                h2391
            }
        }) {
            0u64
        } else {
            let a2397 = match v1.clone() {
                Adt14::C0(_, _, h2392, _, _, _, _, _, _, _, _) => {
                    h2392
                }
            };
            let a2398 = {
                let a2395 = match v1.clone() {
                    Adt14::C0(_, _, _, h2393, _, _, _, _, _, _, _) => {
                        h2393
                    }
                };
                let a2396 = match v1.clone() {
                    Adt14::C0(_, _, _, _, h2394, _, _, _, _, _, _) => {
                        h2394
                    }
                };
                nat_mul(a2395, a2396)?
            };
            nat_mul(a2397, a2398)?
        };
        let v4: u64 = {
            let a2399 = v3;
            let a2400 = 0u64;
            if if nat_eq(a2399, a2400) {
                true
            } else {
                f186(v2.clone())?
            } {
                0u64
            } else {
                let a2402 = match v2.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2401, _) => {
                        h2401
                    }
                };
                let a2403 = 4u64;
                let a2404 = 0u64;
                nat_quot(a2402, a2403, a2404)
            }
        };
        Ok(Adt15::C1(f187(f188(match v1.clone() {
            Adt14::C0(_, _, _, _, _, h2405, _, _, _, _, _) => {
                h2405
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, _, h2406, _, _, _, _, _) => {
                h2406
            }
        }), match v1 {
            Adt14::C0(_, h2407, _, _, _, _, _, _, _, _, _) => {
                h2407
            }
        }, v3, v4)?, 4u64, 64u64, v3, f189(v4)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f84(v0: u64, v1: u64, v2: Adt14, v3: Adt14) -> R<Adt15> {
    let v4: Adt14 = f190(v3.clone());
    let a2409 = match v2.clone() {
        Adt14::C0(h2408, _, _, _, _, _, _, _, _, _, _) => {
            h2408
        }
    };
    let a2410 = 0u64;
    if if if if nat_eq(a2409, a2410) {
        let a2412 = match v3.clone() {
            Adt14::C0(h2411, _, _, _, _, _, _, _, _, _, _) => {
                h2411
            }
        };
        let a2413 = 0u64;
        nat_eq(a2412, a2413)
    } else {
        false
    } {
        f180(v2.clone(), v3)
    } else {
        false
    } {
        if f181(match v2.clone() {
            Adt14::C0(_, h2414, _, _, _, _, _, _, _, _, _) => {
                h2414
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2415, _, _, _, _, _, _, _, _) => {
                h2415
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2416, _, _, _, _, _, _, _) => {
                h2416
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2417, _, _, _, _, _, _) => {
                h2417
            }
        }) {
            true
        } else {
            let a2419 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2418, _, _, _, _, _) => {
                    h2418
                }
            };
            let a2420 = 4u64;
            if if if if if nat_eq(a2419, a2420) {
                f191(match v2.clone() {
                    Adt14::C0(_, h2421, _, _, _, _, _, _, _, _, _) => {
                        h2421
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, h2422, _, _, _, _, _, _, _, _) => {
                        h2422
                    }
                }, match v2.clone() {
                    Adt14::C0(_, _, _, h2423, _, _, _, _, _, _, _) => {
                        h2423
                    }
                }, v0)?
            } else {
                false
            } {
                f183(v2.clone())
            } else {
                false
            } {
                f183(v4.clone())
            } else {
                false
            } {
                let a2425 = f192(v4.clone())?;
                let a2426 = match v4.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2424, _) => {
                        h2424
                    }
                };
                nat_le(a2425, a2426)
            } else {
                false
            } {
                if f184(v2.clone(), v4.clone())? {
                    true
                } else {
                    f185(v2.clone(), v4.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v5: u64 = if if f181(match v2.clone() {
            Adt14::C0(_, h2427, _, _, _, _, _, _, _, _, _) => {
                h2427
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2428, _, _, _, _, _, _, _, _) => {
                h2428
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2429, _, _, _, _, _, _, _) => {
                h2429
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2430, _, _, _, _, _, _) => {
                h2430
            }
        }) {
            true
        } else {
            f186(v4.clone())?
        } {
            0u64
        } else {
            let a2432 = match v4 {
                Adt14::C0(_, _, _, _, _, _, _, _, _, h2431, _) => {
                    h2431
                }
            };
            let a2433 = 4u64;
            let a2434 = 0u64;
            nat_quot(a2432, a2433, a2434)
        };
        Ok(Adt15::C1(f193(match v2.clone() {
            Adt14::C0(_, h2435, _, _, _, _, _, _, _, _, _) => {
                h2435
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2436, _, _, _, _, _, _, _, _) => {
                h2436
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2437, _, _, _, _, _, _, _) => {
                h2437
            }
        }, match v2 {
            Adt14::C0(_, _, _, _, h2438, _, _, _, _, _, _) => {
                h2438
            }
        }, v0, v5)?, 4u64, 65u64, v0, f189(v5)?, v1))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f85(v0: u64, v1: bool, v2: Adt14, v3: Adt14, v4: Adt14) -> R<Adt15> {
    let v5: Adt14 = f190(v4.clone());
    if if if if if if if v1 {
        let a2440 = match v2.clone() {
            Adt14::C0(h2439, _, _, _, _, _, _, _, _, _, _) => {
                h2439
            }
        };
        let a2441 = 0u64;
        nat_eq(a2440, a2441)
    } else {
        false
    } {
        let a2443 = match v3.clone() {
            Adt14::C0(h2442, _, _, _, _, _, _, _, _, _, _) => {
                h2442
            }
        };
        let a2444 = 0u64;
        nat_eq(a2443, a2444)
    } else {
        false
    } {
        let a2446 = match v4.clone() {
            Adt14::C0(h2445, _, _, _, _, _, _, _, _, _, _) => {
                h2445
            }
        };
        let a2447 = 0u64;
        nat_eq(a2446, a2447)
    } else {
        false
    } {
        f180(v2.clone(), v3.clone())
    } else {
        false
    } {
        f180(v2.clone(), v4)
    } else {
        false
    } {
        if f181(match v2.clone() {
            Adt14::C0(_, h2448, _, _, _, _, _, _, _, _, _) => {
                h2448
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2449, _, _, _, _, _, _, _, _) => {
                h2449
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2450, _, _, _, _, _, _, _) => {
                h2450
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2451, _, _, _, _, _, _) => {
                h2451
            }
        }) {
            true
        } else {
            let a2453 = match v2.clone() {
                Adt14::C0(_, _, _, _, _, h2452, _, _, _, _, _) => {
                    h2452
                }
            };
            let a2454 = 4u64;
            if if if if if if if nat_eq(a2453, a2454) {
                let a2456 = match v3.clone() {
                    Adt14::C0(_, _, _, _, _, h2455, _, _, _, _, _) => {
                        h2455
                    }
                };
                let a2457 = 4u64;
                nat_eq(a2456, a2457)
            } else {
                false
            } {
                f183(v2.clone())
            } else {
                false
            } {
                f183(v3.clone())
            } else {
                false
            } {
                f183(v5.clone())
            } else {
                false
            } {
                let a2459 = f192(v5.clone())?;
                let a2460 = match v5.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2458, _) => {
                        h2458
                    }
                };
                nat_le(a2459, a2460)
            } else {
                false
            } {
                if f184(v2.clone(), v5.clone())? {
                    true
                } else {
                    f185(v2.clone(), v5.clone())
                }
            } else {
                false
            } {
                if f184(v3.clone(), v5.clone())? {
                    true
                } else {
                    f185(v3, v5.clone())
                }
            } else {
                false
            }
        }
    } else {
        false
    } {
        let v6: u64 = if f181(match v2.clone() {
            Adt14::C0(_, h2461, _, _, _, _, _, _, _, _, _) => {
                h2461
            }
        }, match v2.clone() {
            Adt14::C0(_, _, h2462, _, _, _, _, _, _, _, _) => {
                h2462
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, h2463, _, _, _, _, _, _, _) => {
                h2463
            }
        }, match v2.clone() {
            Adt14::C0(_, _, _, _, h2464, _, _, _, _, _, _) => {
                h2464
            }
        }) {
            0u64
        } else {
            let a2470 = match v2.clone() {
                Adt14::C0(_, _, h2465, _, _, _, _, _, _, _, _) => {
                    h2465
                }
            };
            let a2471 = {
                let a2468 = match v2.clone() {
                    Adt14::C0(_, _, _, h2466, _, _, _, _, _, _, _) => {
                        h2466
                    }
                };
                let a2469 = match v2.clone() {
                    Adt14::C0(_, _, _, _, h2467, _, _, _, _, _, _) => {
                        h2467
                    }
                };
                nat_mul(a2468, a2469)?
            };
            nat_mul(a2470, a2471)?
        };
        let v7: u64 = {
            let a2472 = v6;
            let a2473 = 0u64;
            if if nat_eq(a2472, a2473) {
                true
            } else {
                f186(v5.clone())?
            } {
                0u64
            } else {
                let a2475 = match v5 {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, h2474, _) => {
                        h2474
                    }
                };
                let a2476 = 4u64;
                let a2477 = 0u64;
                nat_quot(a2475, a2476, a2477)
            }
        };
        Ok(Adt15::C1(f194(match v2 {
            Adt14::C0(_, h2478, _, _, _, _, _, _, _, _, _) => {
                h2478
            }
        }, v6, v7)?, 4u64, 66u64, v6, f189(v7)?, v0))
    } else {
        Ok(Adt15::C0)
    }
}

pub fn f86(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> bool {
    if if f195(v0, v3, v6) {
        f195(v1, v4, v6)
    } else {
        false
    } {
        f195(v2, v5, v6)
    } else {
        false
    }
}

pub fn f87(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    if if if if if f196(v0, v1, v4, v2, v5, v3, v6)? {
        true
    } else {
        f196(v0, v1, v4, v3, v6, v2, v5)?
    } {
        true
    } else {
        f196(v0, v2, v5, v1, v4, v3, v6)?
    } {
        true
    } else {
        f196(v0, v2, v5, v3, v6, v1, v4)?
    } {
        true
    } else {
        f196(v0, v3, v6, v1, v4, v2, v5)?
    } {
        Ok(true)
    } else {
        f196(v0, v3, v6, v2, v5, v1, v4)
    }
}

pub fn f88(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2479 = v2;
    let a2480 = 0u64;
    if nat_eq(a2479, a2480) {
        Ok(0u64)
    } else {
        let a2483 = 2u64;
        let a2484 = {
            let a2481 = v0;
            let a2482 = v1;
            nat_mul(a2481, a2482)?
        };
        nat_mul(a2483, a2484)
    }
}

pub fn f89(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a2485 = v2;
    let a2486 = f176(v0, v1)?;
    nat_mul(a2485, a2486)
}

pub fn f90(v0: u64) -> u64 {
    let a2487 = v0;
    let a2488 = 30u64;
    if nat_eq(a2487, a2488) {
        3u64
    } else {
        1u64
    }
}

pub fn f91(v0: u64) -> R<u64> {
    let a2491 = 5u64;
    let a2492 = {
        let a2489 = 4u64;
        let a2490 = v0;
        nat_mul(a2489, a2490)?
    };
    nat_add(a2491, a2492)
}

pub fn f92(v0: u64) -> bool {
    let m2493 = f197(v0);
    match m2493 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(v1) => {
            let a2495 = 1u64;
            let a2496 = match v1 {
                Adt19::C0(_, h2494, _) => {
                    h2494
                }
            };
            nat_lt(a2495, a2496)
        }
    }
}

pub fn f93(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: bool = {
        let a2498 = 0u64;
        let a2499 = match v0.clone() {
            Adt1::C0(_, h2497, _, _, _, _, _, _, _, _, _, _) => {
                h2497
            }
        };
        nat_lt(a2498, a2499)
    };
    let a2501 = match v1.clone() {
        Adt2::C0(h2500, _, _, _, _, _, _, _, _, _, _) => {
            h2500
        }
    };
    let a2502 = 0u64;
    if if if if if if if if if if if if if if if if nat_eq(a2501, a2502) {
        let a2504 = match v2.clone() {
            Adt2::C0(h2503, _, _, _, _, _, _, _, _, _, _) => {
                h2503
            }
        };
        let a2505 = 0u64;
        nat_eq(a2504, a2505)
    } else {
        false
    } {
        let a2507 = match v3.clone() {
            Adt2::C0(h2506, _, _, _, _, _, _, _, _, _, _) => {
                h2506
            }
        };
        let a2508 = 0u64;
        nat_eq(a2507, a2508)
    } else {
        false
    } {
        let a2510 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2509, _, _, _, _, _, _) => {
                h2509
            }
        };
        let a2511 = 1u64;
        nat_eq(a2510, a2511)
    } else {
        false
    } {
        let a2513 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2512, _, _, _, _, _, _) => {
                h2512
            }
        };
        let a2514 = 1u64;
        nat_eq(a2513, a2514)
    } else {
        false
    } {
        let a2516 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2515, _, _, _, _, _, _) => {
                h2515
            }
        };
        let a2517 = 1u64;
        nat_eq(a2516, a2517)
    } else {
        false
    } {
        let a2519 = 0u64;
        let a2520 = match v1.clone() {
            Adt2::C0(_, _, h2518, _, _, _, _, _, _, _, _) => {
                h2518
            }
        };
        nat_lt(a2519, a2520)
    } else {
        false
    } {
        let a2522 = match v3.clone() {
            Adt2::C0(_, _, _, h2521, _, _, _, _, _, _, _) => {
                h2521
            }
        };
        let a2523 = 1u64;
        nat_eq(a2522, a2523)
    } else {
        false
    } {
        let a2526 = match v3.clone() {
            Adt2::C0(_, h2524, _, _, _, _, _, _, _, _, _) => {
                h2524
            }
        };
        let a2527 = match v1.clone() {
            Adt2::C0(_, h2525, _, _, _, _, _, _, _, _, _) => {
                h2525
            }
        };
        nat_eq(a2526, a2527)
    } else {
        false
    } {
        let a2530 = match v3.clone() {
            Adt2::C0(_, _, h2528, _, _, _, _, _, _, _, _) => {
                h2528
            }
        };
        let a2531 = match v1.clone() {
            Adt2::C0(_, _, _, h2529, _, _, _, _, _, _, _) => {
                h2529
            }
        };
        nat_eq(a2530, a2531)
    } else {
        false
    } {
        if v4 {
            f24(v1.clone(), v2.clone())
        } else {
            let a2534 = match v2.clone() {
                Adt2::C0(_, h2532, _, _, _, _, _, _, _, _, _) => {
                    h2532
                }
            };
            let a2535 = match v1.clone() {
                Adt2::C0(_, _, h2533, _, _, _, _, _, _, _, _) => {
                    h2533
                }
            };
            if if nat_eq(a2534, a2535) {
                let a2538 = match v2.clone() {
                    Adt2::C0(_, _, h2536, _, _, _, _, _, _, _, _) => {
                        h2536
                    }
                };
                let a2539 = match v1.clone() {
                    Adt2::C0(_, _, _, h2537, _, _, _, _, _, _, _) => {
                        h2537
                    }
                };
                nat_eq(a2538, a2539)
            } else {
                false
            } {
                let a2541 = match v2.clone() {
                    Adt2::C0(_, _, _, h2540, _, _, _, _, _, _, _) => {
                        h2540
                    }
                };
                let a2542 = 1u64;
                nat_eq(a2541, a2542)
            } else {
                false
            }
        }
    } else {
        false
    } {
        f18(v1.clone(), f19(v1.clone(), 1u64)?)?
    } else {
        false
    } {
        f18(v2.clone(), f19(v2.clone(), 1u64)?)?
    } else {
        false
    } {
        f21(v3.clone())?
    } else {
        false
    } {
        f20(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f20(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2544 = f19(v3.clone(), 1u64)?;
            let a2545 = f153(match v1.clone() {
                Adt2::C0(_, _, h2543, _, _, _, _, _, _, _, _) => {
                    h2543
                }
            }, v4)?;
            nat_mul(a2544, a2545)?
        }, 4u64, f23(v1, 1u64), f23(v3, 1u64), Adt4::C36(f23(v2, 1u64), v4, match v0 {
            Adt1::C0(h2546, _, _, _, _, _, _, _, _, _, _, _) => {
                h2546
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f94(v0: Adt1, v1: Adt2, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, h2547, _, _, _, _, _, _, _, _, _) => {
            h2547
        }
    }, match v0.clone() {
        Adt1::C0(h2548, _, _, _, _, _, _, _, _, _, _, _) => {
            h2548
        }
    }, match v0.clone() {
        Adt1::C0(_, _, h2549, _, _, _, _, _, _, _, _, _) => {
            h2549
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, _, h2550, _, _, _, _, _, _, _) => {
            h2550
        }
    });
    let v5: Adt6 = Adt6::C0(match v1.clone() {
        Adt2::C0(_, _, h2551, _, _, _, _, _, _, _, _) => {
            h2551
        }
    }, match v0.clone() {
        Adt1::C0(_, h2552, _, _, _, _, _, _, _, _, _, _) => {
            h2552
        }
    }, match v0.clone() {
        Adt1::C0(_, _, _, h2553, _, _, _, _, _, _, _, _) => {
            h2553
        }
    }, match v0 {
        Adt1::C0(_, _, _, _, _, h2554, _, _, _, _, _, _) => {
            h2554
        }
    });
    let v6: u64 = match v1.clone() {
        Adt2::C0(h2555, _, _, _, _, _, _, _, _, _, _) => {
            h2555
        }
    };
    let v7: bool = {
        let a2558 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, _, h2556, _, _, _) => {
                h2556
            }
        };
        let a2559 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2557, _, _, _, _, _) => {
                h2557
            }
        };
        nat_lt(a2558, a2559)
    };
    let a2560 = v6;
    let a2561 = 0u64;
    if if if if if if if if if if if if if if if nat_eq(a2560, a2561) {
        true
    } else {
        let a2562 = v6;
        let a2563 = 1u64;
        nat_eq(a2562, a2563)
    } {
        let a2565 = match v2.clone() {
            Adt2::C0(h2564, _, _, _, _, _, _, _, _, _, _) => {
                h2564
            }
        };
        let a2566 = 0u64;
        nat_eq(a2565, a2566)
    } else {
        false
    } {
        let a2568 = match v3.clone() {
            Adt2::C0(h2567, _, _, _, _, _, _, _, _, _, _) => {
                h2567
            }
        };
        let a2569 = 0u64;
        nat_eq(a2568, a2569)
    } else {
        false
    } {
        let a2571 = match v1.clone() {
            Adt2::C0(_, _, _, h2570, _, _, _, _, _, _, _) => {
                h2570
            }
        };
        let a2572 = 1u64;
        nat_eq(a2571, a2572)
    } else {
        false
    } {
        let a2575 = match v1.clone() {
            Adt2::C0(_, _, _, _, h2573, _, _, _, _, _, _) => {
                h2573
            }
        };
        let a2576 = match v2.clone() {
            Adt2::C0(_, _, _, h2574, _, _, _, _, _, _, _) => {
                h2574
            }
        };
        nat_eq(a2575, a2576)
    } else {
        false
    } {
        let a2579 = match v3.clone() {
            Adt2::C0(_, _, _, h2577, _, _, _, _, _, _, _) => {
                h2577
            }
        };
        let a2580 = match v2.clone() {
            Adt2::C0(_, _, _, h2578, _, _, _, _, _, _, _) => {
                h2578
            }
        };
        nat_eq(a2579, a2580)
    } else {
        false
    } {
        let a2583 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2581, _, _, _, _, _, _) => {
                h2581
            }
        };
        let a2584 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2582, _, _, _, _, _, _) => {
                h2582
            }
        };
        nat_eq(a2583, a2584)
    } else {
        false
    } {
        let a2586 = 0u64;
        let a2587 = match v4.clone() {
            Adt6::C0(h2585, _, _, _) => {
                h2585
            }
        };
        if if if if if nat_lt(a2586, a2587) {
            let a2589 = 0u64;
            let a2590 = match v4.clone() {
                Adt6::C0(_, h2588, _, _) => {
                    h2588
                }
            };
            nat_lt(a2589, a2590)
        } else {
            false
        } {
            let a2592 = match v4.clone() {
                Adt6::C0(_, h2591, _, _) => {
                    h2591
                }
            };
            let a2593 = 2147483648u64;
            nat_lt(a2592, a2593)
        } else {
            false
        } {
            let a2595 = 0u64;
            let a2596 = match v4.clone() {
                Adt6::C0(_, _, _, h2594) => {
                    h2594
                }
            };
            nat_lt(a2595, a2596)
        } else {
            false
        } {
            let a2598 = match v4.clone() {
                Adt6::C0(_, _, _, h2597) => {
                    h2597
                }
            };
            let a2599 = 2147483648u64;
            nat_lt(a2598, a2599)
        } else {
            false
        } {
            let a2602 = match v3.clone() {
                Adt2::C0(_, h2600, _, _, _, _, _, _, _, _, _) => {
                    h2600
                }
            };
            let a2603 = f80(v4.clone(), match v2.clone() {
                Adt2::C0(_, h2601, _, _, _, _, _, _, _, _, _) => {
                    h2601
                }
            })?;
            nat_eq(a2602, a2603)
        } else {
            false
        }
    } else {
        false
    } {
        let a2605 = 0u64;
        let a2606 = match v5.clone() {
            Adt6::C0(h2604, _, _, _) => {
                h2604
            }
        };
        if if if if if nat_lt(a2605, a2606) {
            let a2608 = 0u64;
            let a2609 = match v5.clone() {
                Adt6::C0(_, h2607, _, _) => {
                    h2607
                }
            };
            nat_lt(a2608, a2609)
        } else {
            false
        } {
            let a2611 = match v5.clone() {
                Adt6::C0(_, h2610, _, _) => {
                    h2610
                }
            };
            let a2612 = 2147483648u64;
            nat_lt(a2611, a2612)
        } else {
            false
        } {
            let a2614 = 0u64;
            let a2615 = match v5.clone() {
                Adt6::C0(_, _, _, h2613) => {
                    h2613
                }
            };
            nat_lt(a2614, a2615)
        } else {
            false
        } {
            let a2617 = match v5.clone() {
                Adt6::C0(_, _, _, h2616) => {
                    h2616
                }
            };
            let a2618 = 2147483648u64;
            nat_lt(a2617, a2618)
        } else {
            false
        } {
            let a2621 = match v3.clone() {
                Adt2::C0(_, _, h2619, _, _, _, _, _, _, _, _) => {
                    h2619
                }
            };
            let a2622 = f80(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2620, _, _, _, _, _, _, _, _) => {
                    h2620
                }
            })?;
            nat_eq(a2621, a2622)
        } else {
            false
        }
    } else {
        false
    } {
        f198(v2.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f198(v3.clone(), 4u64, v7, false)?
    } else {
        false
    } {
        f198(v1.clone(), {
            let a2623 = v6;
            let a2624 = 0u64;
            if nat_eq(a2623, a2624) {
                4u64
            } else {
                2u64
            }
        }, v7, true)?
    } else {
        false
    } {
        f20(v1.clone(), v3.clone())?
    } else {
        false
    } {
        f20(v2.clone(), v3.clone())?
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2633 = {
                let a2631 = {
                    let a2629 = 3u64;
                    let a2630 = {
                        let a2627 = match v4.clone() {
                            Adt6::C0(h2625, _, _, _) => {
                                h2625
                            }
                        };
                        let a2628 = match v5.clone() {
                            Adt6::C0(h2626, _, _, _) => {
                                h2626
                            }
                        };
                        nat_mul(a2627, a2628)?
                    };
                    nat_mul(a2629, a2630)?
                };
                let a2632 = 2u64;
                nat_add(a2631, a2632)?
            };
            let a2634 = f19(v3.clone(), 1u64)?;
            nat_mul(a2633, a2634)?
        }, 4u64, f23(v1, 1u64), f23(v3, 1u64), Adt4::C34(f23(v2, 1u64), v4, v5, v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f95(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if f24(v0.clone(), v1.clone()) {
        f24(v0.clone(), v2.clone())
    } else {
        false
    } {
        f21(v0.clone())?
    } else {
        false
    } {
        f21(v1.clone())?
    } else {
        false
    } {
        f21(v2.clone())?
    } else {
        false
    } {
        if f20(v0.clone(), v2.clone())? {
            true
        } else {
            f39(v0.clone(), v2.clone())
        }
    } else {
        false
    } {
        if f20(v1.clone(), v2.clone())? {
            true
        } else {
            f39(v1.clone(), v2.clone())
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a2635 = 6u64;
            let a2636 = f19(v0.clone(), 1u64)?;
            nat_mul(a2635, a2636)?
        }, 2u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C33(f23(v1, 1u64))))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f96(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2637 = v2;
    match m2637 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: u64 = match v1.clone() {
                Adt2::C0(_, h2638, _, _, _, _, _, _, _, _, _) => {
                    h2638
                }
            };
            let v6: u64 = {
                let a2644 = match v1.clone() {
                    Adt2::C0(_, _, h2639, _, _, _, _, _, _, _, _) => {
                        h2639
                    }
                };
                let a2645 = {
                    let a2642 = match v1.clone() {
                        Adt2::C0(_, _, _, h2640, _, _, _, _, _, _, _) => {
                            h2640
                        }
                    };
                    let a2643 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h2641, _, _, _, _, _, _) => {
                            h2641
                        }
                    };
                    nat_mul(a2642, a2643)?
                };
                nat_mul(a2644, a2645)?
            };
            if if if if if if if if if if f24(v1.clone(), v4.clone()) {
                f24(v1.clone(), v3.clone())
            } else {
                false
            } {
                f199(v1.clone(), v6)?
            } else {
                false
            } {
                f199(v4.clone(), v6)?
            } else {
                false
            } {
                f199(v3.clone(), v6)?
            } else {
                false
            } {
                if f20(v1.clone(), v3.clone())? {
                    true
                } else {
                    f39(v1.clone(), v3.clone())
                }
            } else {
                false
            } {
                if f20(v4.clone(), v3.clone())? {
                    true
                } else {
                    f39(v4, v3.clone())
                }
            } else {
                false
            } {
                let a2646 = v5;
                let a2647 = 2147483647u64;
                nat_le(a2646, a2647)
            } else {
                false
            } {
                let a2648 = v6;
                let a2649 = 2147483647u64;
                nat_le(a2648, a2649)
            } else {
                false
            } {
                let a2651 = match v0.clone() {
                    Adt1::C0(h2650, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2650
                    }
                };
                let a2652 = 4294967295u64;
                if nat_le(a2651, a2652) {
                    let a2654 = match v0.clone() {
                        Adt1::C0(_, h2653, _, _, _, _, _, _, _, _, _, _) => {
                            h2653
                        }
                    };
                    let a2655 = 4294967295u64;
                    nat_le(a2654, a2655)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2656 = v5;
                    let a2657 = 0u64;
                    if nat_eq(a2656, a2657) {
                        0u64
                    } else {
                        let a2658 = v6;
                        let a2659 = f152(v5)?;
                        nat_mul(a2658, a2659)?
                    }
                }, 4u64, f23(v1, 1u64), f23(v3, 1u64), Adt4::C32(match v0 {
                    Adt1::C0(h2660, _, _, _, _, _, _, _, _, _, _, _) => {
                        h2660
                    }
                })))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f97(v0: Adt1, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v0.clone() {
        Adt1::C0(h2661, _, _, _, _, _, _, _, _, _, _, _) => {
            h2661
        }
    };
    let v4: u64 = match v0.clone() {
        Adt1::C0(_, h2662, _, _, _, _, _, _, _, _, _, _) => {
            h2662
        }
    };
    let v5: u64 = match v0 {
        Adt1::C0(_, _, h2663, _, _, _, _, _, _, _, _, _) => {
            h2663
        }
    };
    let v6: u64 = {
        let a2665 = match v1.clone() {
            Adt2::C0(_, h2664, _, _, _, _, _, _, _, _, _) => {
                h2664
            }
        };
        let a2666 = v4;
        let a2667 = 0u64;
        nat_quot(a2665, a2666, a2667)
    };
    let v7: u64 = {
        let a2669 = match v1.clone() {
            Adt2::C0(h2668, _, _, _, _, _, _, _, _, _, _) => {
                h2668
            }
        };
        let a2670 = 0u64;
        if nat_eq(a2669, a2670) {
            4u64
        } else {
            2u64
        }
    };
    if if if if if if if if if if if if if if if f159(match v1.clone() {
        Adt2::C0(h2671, _, _, _, _, _, _, _, _, _, _) => {
            h2671
        }
    }) {
        let a2674 = match v1.clone() {
            Adt2::C0(h2672, _, _, _, _, _, _, _, _, _, _) => {
                h2672
            }
        };
        let a2675 = match v2.clone() {
            Adt2::C0(h2673, _, _, _, _, _, _, _, _, _, _) => {
                h2673
            }
        };
        nat_eq(a2674, a2675)
    } else {
        false
    } {
        let a2676 = 0u64;
        let a2677 = v3;
        nat_lt(a2676, a2677)
    } else {
        false
    } {
        let a2678 = v3;
        let a2679 = 2147483648u64;
        nat_lt(a2678, a2679)
    } else {
        false
    } {
        let a2680 = 0u64;
        let a2681 = v4;
        nat_lt(a2680, a2681)
    } else {
        false
    } {
        let a2682 = v4;
        let a2683 = 2147483648u64;
        nat_lt(a2682, a2683)
    } else {
        false
    } {
        let a2684 = v5;
        let a2685 = 2147483648u64;
        nat_lt(a2684, a2685)
    } else {
        false
    } {
        let a2686 = 0u64;
        let a2687 = v6;
        nat_lt(a2686, a2687)
    } else {
        false
    } {
        let a2689 = 0u64;
        let a2690 = match v2.clone() {
            Adt2::C0(_, h2688, _, _, _, _, _, _, _, _, _) => {
                h2688
            }
        };
        nat_lt(a2689, a2690)
    } else {
        false
    } {
        let a2694 = match v1.clone() {
            Adt2::C0(_, h2691, _, _, _, _, _, _, _, _, _) => {
                h2691
            }
        };
        let a2695 = {
            let a2692 = v6;
            let a2693 = v4;
            nat_mul(a2692, a2693)?
        };
        nat_eq(a2694, a2695)
    } else {
        false
    } {
        let a2697 = match v2.clone() {
            Adt2::C0(_, _, h2696, _, _, _, _, _, _, _, _) => {
                h2696
            }
        };
        let a2698 = v4;
        nat_eq(a2697, a2698)
    } else {
        false
    } {
        let a2711 = {
            let a2704 = {
                let a2702 = match v2.clone() {
                    Adt2::C0(_, h2699, _, _, _, _, _, _, _, _, _) => {
                        h2699
                    }
                };
                let a2703 = {
                    let a2700 = v5;
                    let a2701 = 2u64;
                    nat_mul(a2700, a2701)?
                };
                nat_add(a2702, a2703)?
            };
            let a2705 = v3;
            nat_add(a2704, a2705)?
        };
        let a2712 = {
            let a2709 = {
                let a2707 = match v1.clone() {
                    Adt2::C0(_, _, h2706, _, _, _, _, _, _, _, _) => {
                        h2706
                    }
                };
                let a2708 = v3;
                nat_mul(a2707, a2708)?
            };
            let a2710 = v6;
            nat_add(a2709, a2710)?
        };
        nat_eq(a2711, a2712)
    } else {
        false
    } {
        f200(v1.clone(), v7)?
    } else {
        false
    } {
        f200(v2.clone(), v7)?
    } else {
        false
    } {
        f20(v1.clone(), v2.clone())?
    } else {
        false
    } {
        let v8: Adt6 = Adt6::C0(v6, v3, v5, 1u64);
        Ok(Adt10::C4({
            let a2713 = f19(v2.clone(), 1u64)?;
            let a2714 = f173(v8.clone())?;
            nat_mul(a2713, a2714)?
        }, v7, f23(v1.clone(), 1u64), f23(v2, 1u64), Adt4::C30(v8, match v1 {
            Adt2::C0(h2715, _, _, _, _, _, _, _, _, _, _) => {
                h2715
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f98(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m2716 = v1;
    match m2716 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let a2718 = match v0.clone() {
                Adt2::C0(h2717, _, _, _, _, _, _, _, _, _, _) => {
                    h2717
                }
            };
            let a2719 = 0u64;
            if if if if if if if if if if if if if if if if if if if nat_eq(a2718, a2719) {
                let a2721 = match v3.clone() {
                    Adt2::C0(h2720, _, _, _, _, _, _, _, _, _, _) => {
                        h2720
                    }
                };
                let a2722 = 0u64;
                nat_eq(a2721, a2722)
            } else {
                false
            } {
                let a2724 = match v2.clone() {
                    Adt2::C0(h2723, _, _, _, _, _, _, _, _, _, _) => {
                        h2723
                    }
                };
                let a2725 = 0u64;
                nat_eq(a2724, a2725)
            } else {
                false
            } {
                let a2727 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h2726, _, _, _, _, _, _) => {
                        h2726
                    }
                };
                let a2728 = 1u64;
                nat_eq(a2727, a2728)
            } else {
                false
            } {
                let a2730 = match v3.clone() {
                    Adt2::C0(_, _, _, h2729, _, _, _, _, _, _, _) => {
                        h2729
                    }
                };
                let a2731 = 1u64;
                nat_eq(a2730, a2731)
            } else {
                false
            } {
                let a2733 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h2732, _, _, _, _, _, _) => {
                        h2732
                    }
                };
                let a2734 = 1u64;
                nat_eq(a2733, a2734)
            } else {
                false
            } {
                let a2737 = match v0.clone() {
                    Adt2::C0(_, _, h2735, _, _, _, _, _, _, _, _) => {
                        h2735
                    }
                };
                let a2738 = match v3.clone() {
                    Adt2::C0(_, _, h2736, _, _, _, _, _, _, _, _) => {
                        h2736
                    }
                };
                nat_eq(a2737, a2738)
            } else {
                false
            } {
                let a2743 = match v3.clone() {
                    Adt2::C0(_, h2739, _, _, _, _, _, _, _, _, _) => {
                        h2739
                    }
                };
                let a2744 = {
                    let a2741 = match v0.clone() {
                        Adt2::C0(_, h2740, _, _, _, _, _, _, _, _, _) => {
                            h2740
                        }
                    };
                    let a2742 = 1u64;
                    nat_add(a2741, a2742)?
                };
                nat_le(a2743, a2744)
            } else {
                false
            } {
                let a2747 = match v2.clone() {
                    Adt2::C0(_, h2745, _, _, _, _, _, _, _, _, _) => {
                        h2745
                    }
                };
                let a2748 = match v0.clone() {
                    Adt2::C0(_, _, h2746, _, _, _, _, _, _, _, _) => {
                        h2746
                    }
                };
                nat_eq(a2747, a2748)
            } else {
                false
            } {
                let a2756 = match v2.clone() {
                    Adt2::C0(_, _, h2749, _, _, _, _, _, _, _, _) => {
                        h2749
                    }
                };
                let a2757 = {
                    let a2754 = {
                        let a2751 = match v0.clone() {
                            Adt2::C0(_, h2750, _, _, _, _, _, _, _, _, _) => {
                                h2750
                            }
                        };
                        let a2752 = 1u64;
                        nat_add(a2751, a2752)?
                    };
                    let a2755 = match v3.clone() {
                        Adt2::C0(_, h2753, _, _, _, _, _, _, _, _, _) => {
                            h2753
                        }
                    };
                    nat_sub(a2754, a2755)
                };
                nat_eq(a2756, a2757)
            } else {
                false
            } {
                let a2760 = match v2.clone() {
                    Adt2::C0(_, _, _, h2758, _, _, _, _, _, _, _) => {
                        h2758
                    }
                };
                let a2761 = match v0.clone() {
                    Adt2::C0(_, _, _, h2759, _, _, _, _, _, _, _) => {
                        h2759
                    }
                };
                nat_eq(a2760, a2761)
            } else {
                false
            } {
                let a2763 = match v2.clone() {
                    Adt2::C0(_, _, _, _, h2762, _, _, _, _, _, _) => {
                        h2762
                    }
                };
                let a2764 = 1u64;
                nat_eq(a2763, a2764)
            } else {
                false
            } {
                f30(v2.clone())?
            } else {
                false
            } {
                let a2766 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, h2765, _, _, _, _, _) => {
                        h2765
                    }
                };
                let a2767 = 4u64;
                nat_eq(a2766, a2767)
            } else {
                false
            } {
                let a2769 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, h2768, _, _, _, _, _) => {
                        h2768
                    }
                };
                let a2770 = 4u64;
                nat_eq(a2769, a2770)
            } else {
                false
            } {
                let a2775 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2771, _, _, _, _) => {
                        h2771
                    }
                };
                let a2776 = {
                    let a2773 = match v0.clone() {
                        Adt2::C0(_, h2772, _, _, _, _, _, _, _, _, _) => {
                            h2772
                        }
                    };
                    let a2774 = 4u64;
                    nat_mul(a2773, a2774)?
                };
                nat_eq(a2775, a2776)
            } else {
                false
            } {
                let a2781 = match v3.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2777, _, _, _, _) => {
                        h2777
                    }
                };
                let a2782 = {
                    let a2779 = match v3.clone() {
                        Adt2::C0(_, h2778, _, _, _, _, _, _, _, _, _) => {
                            h2778
                        }
                    };
                    let a2780 = 4u64;
                    nat_mul(a2779, a2780)?
                };
                nat_eq(a2781, a2782)
            } else {
                false
            } {
                f20(v0.clone(), v2.clone())?
            } else {
                false
            } {
                f20(v3.clone(), v2.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4({
                    let a2784 = f19(v2.clone(), 1u64)?;
                    let a2785 = f172(match v3.clone() {
                        Adt2::C0(_, h2783, _, _, _, _, _, _, _, _, _) => {
                            h2783
                        }
                    })?;
                    nat_mul(a2784, a2785)?
                }, 4u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C29(f23(v3, 1u64))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f99(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: Adt6 = Adt6::C0(match v1.clone() {
        Adt1::C0(_, h2786, _, _, _, _, _, _, _, _, _, _) => {
            h2786
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, h2787, _, _, _, _, _, _, _, _) => {
                h2787
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h2788, _, _, _, _, _, _, _, _, _) => {
                h2788
            }
        }
    }, if v0 {
        match v1.clone() {
            Adt1::C0(_, _, _, _, _, h2789, _, _, _, _, _, _) => {
                h2789
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, _, h2790, _, _, _, _, _, _, _, _) => {
                h2790
            }
        }
    }, 1u64);
    let v5: Adt6 = if v0 {
        Adt6::C0(match v1.clone() {
            Adt1::C0(_, _, h2791, _, _, _, _, _, _, _, _, _) => {
                h2791
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, h2792, _, _, _, _, _, _, _) => {
                h2792
            }
        }, match v1.clone() {
            Adt1::C0(_, _, _, _, _, _, h2793, _, _, _, _, _) => {
                h2793
            }
        }, 1u64)
    } else {
        Adt6::C0(1u64, 1u64, 0u64, 1u64)
    };
    let a2795 = match v1.clone() {
        Adt1::C0(h2794, _, _, _, _, _, _, _, _, _, _, _) => {
            h2794
        }
    };
    let a2796 = 1u64;
    if if if if if if if if if if if nat_le(a2795, a2796) {
        let a2798 = match v2.clone() {
            Adt2::C0(h2797, _, _, _, _, _, _, _, _, _, _) => {
                h2797
            }
        };
        let a2799 = 1u64;
        nat_le(a2798, a2799)
    } else {
        false
    } {
        let a2801 = match v3.clone() {
            Adt2::C0(h2800, _, _, _, _, _, _, _, _, _, _) => {
                h2800
            }
        };
        let a2802 = 0u64;
        nat_eq(a2801, a2802)
    } else {
        false
    } {
        let a2807 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h2803, _, _, _, _, _) => {
                h2803
            }
        };
        let a2808 = {
            let a2805 = match v2.clone() {
                Adt2::C0(h2804, _, _, _, _, _, _, _, _, _, _) => {
                    h2804
                }
            };
            let a2806 = 0u64;
            if nat_eq(a2805, a2806) {
                4u64
            } else {
                2u64
            }
        };
        nat_eq(a2807, a2808)
    } else {
        false
    } {
        f30(v3.clone())?
    } else {
        false
    } {
        f20(v2.clone(), v3.clone())?
    } else {
        false
    } {
        f201(v4.clone(), match v2.clone() {
            Adt2::C0(_, h2809, _, _, _, _, _, _, _, _, _) => {
                h2809
            }
        }, match v3.clone() {
            Adt2::C0(_, h2810, _, _, _, _, _, _, _, _, _) => {
                h2810
            }
        })?
    } else {
        false
    } {
        if v0 {
            f201(v5.clone(), match v2.clone() {
                Adt2::C0(_, _, h2811, _, _, _, _, _, _, _, _) => {
                    h2811
                }
            }, match v3.clone() {
                Adt2::C0(_, _, h2812, _, _, _, _, _, _, _, _) => {
                    h2812
                }
            })?
        } else {
            let a2815 = match v2.clone() {
                Adt2::C0(_, _, h2813, _, _, _, _, _, _, _, _) => {
                    h2813
                }
            };
            let a2816 = match v3.clone() {
                Adt2::C0(_, _, h2814, _, _, _, _, _, _, _, _) => {
                    h2814
                }
            };
            nat_eq(a2815, a2816)
        }
    } else {
        false
    } {
        let a2819 = match v2.clone() {
            Adt2::C0(_, _, _, h2817, _, _, _, _, _, _, _) => {
                h2817
            }
        };
        let a2820 = match v3.clone() {
            Adt2::C0(_, _, _, h2818, _, _, _, _, _, _, _) => {
                h2818
            }
        };
        nat_eq(a2819, a2820)
    } else {
        false
    } {
        let a2823 = match v2.clone() {
            Adt2::C0(_, _, _, _, h2821, _, _, _, _, _, _) => {
                h2821
            }
        };
        let a2824 = match v3.clone() {
            Adt2::C0(_, _, _, _, h2822, _, _, _, _, _, _) => {
                h2822
            }
        };
        nat_eq(a2823, a2824)
    } else {
        false
    } {
        let a2829 = {
            let a2827 = match v4.clone() {
                Adt6::C0(h2825, _, _, _) => {
                    h2825
                }
            };
            let a2828 = match v5.clone() {
                Adt6::C0(h2826, _, _, _) => {
                    h2826
                }
            };
            nat_mul(a2827, a2828)?
        };
        let a2830 = 2147483648u64;
        nat_lt(a2829, a2830)
    } else {
        false
    } {
        let v6: Adt7 = Adt7::C0(v4, v5, v0, {
            let a2832 = match v1 {
                Adt1::C0(h2831, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2831
                }
            };
            let a2833 = 0u64;
            nat_eq(a2832, a2833)
        }, match v2.clone() {
            Adt2::C0(h2834, _, _, _, _, _, _, _, _, _, _) => {
                h2834
            }
        });
        Ok(Adt10::C4({
            let a2835 = f19(v3.clone(), 1u64)?;
            let a2836 = f168(v6.clone())?;
            nat_mul(a2835, a2836)?
        }, 4u64, f23(v2, 1u64), f23(v3, 1u64), Adt4::C28(v6)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f100(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2837 = v2;
    match m2837 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: bool = {
                let a2839 = match v0.clone() {
                    Adt1::C0(_, _, _, _, _, _, h2838, _, _, _, _, _) => {
                        h2838
                    }
                };
                let a2840 = 1u64;
                nat_eq(a2839, a2840)
            };
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2841, _, _, _, _, _, _, _, _, _) => {
                    h2841
                }
            }, match v0.clone() {
                Adt1::C0(h2842, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2842
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2843, _, _, _, _, _, _, _, _, _) => {
                    h2843
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2844, _, _, _, _, _, _, _) => {
                    h2844
                }
            });
            let v7: Adt6 = if v5 {
                Adt6::C0(match v1.clone() {
                    Adt2::C0(_, _, h2845, _, _, _, _, _, _, _, _) => {
                        h2845
                    }
                }, match v0.clone() {
                    Adt1::C0(_, h2846, _, _, _, _, _, _, _, _, _, _) => {
                        h2846
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, h2847, _, _, _, _, _, _, _, _) => {
                        h2847
                    }
                }, match v0.clone() {
                    Adt1::C0(_, _, _, _, _, h2848, _, _, _, _, _, _) => {
                        h2848
                    }
                })
            } else {
                Adt6::C0(1u64, 1u64, 0u64, 1u64)
            };
            let v8: Adt6 = Adt6::C0(1u64, 1u64, 0u64, 1u64);
            let v9: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2849, _, _, _, _, _, _, _) => {
                        h2849
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, h2850, _, _, _, _, _, _, _, _) => {
                        h2850
                    }
                }
            };
            let v10: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, h2851, _, _, _, _, _, _) => {
                        h2851
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, h2852, _, _, _, _, _, _, _) => {
                        h2852
                    }
                }
            };
            let v11: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, h2853, _, _, _, _, _, _, _, _) => {
                        h2853
                    }
                }
            } else {
                1u64
            };
            let v12: u64 = match v4.clone() {
                Adt2::C0(h2854, _, _, _, _, _, _, _, _, _, _) => {
                    h2854
                }
            };
            let v13: u64 = match v3.clone() {
                Adt2::C0(h2855, _, _, _, _, _, _, _, _, _, _) => {
                    h2855
                }
            };
            let v14: u64 = {
                let a2856 = v12;
                let a2857 = 0u64;
                if nat_eq(a2856, a2857) {
                    4u64
                } else {
                    2u64
                }
            };
            let v15: u64 = {
                let a2859 = match v4.clone() {
                    Adt2::C0(_, h2858, _, _, _, _, _, _, _, _, _) => {
                        h2858
                    }
                };
                let a2860 = v14;
                nat_mul(a2859, a2860)?
            };
            let v16: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2861, _, _, _) => {
                        h2861
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, h2862, _, _, _, _) => {
                        h2862
                    }
                }
            };
            let v17: u64 = if v5 {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h2863, _, _) => {
                        h2863
                    }
                }
            } else {
                match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h2864, _, _, _) => {
                        h2864
                    }
                }
            };
            let a2866 = match v0 {
                Adt1::C0(_, _, _, _, _, _, h2865, _, _, _, _, _) => {
                    h2865
                }
            };
            let a2867 = 1u64;
            if if if if if if if if if if nat_le(a2866, a2867) {
                let a2868 = v12;
                let a2869 = 0u64;
                if nat_eq(a2868, a2869) {
                    true
                } else {
                    let a2870 = v12;
                    let a2871 = 1u64;
                    if nat_eq(a2870, a2871) {
                        let a2872 = v13;
                        let a2873 = 1u64;
                        nat_eq(a2872, a2873)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                let a2874 = v13;
                let a2875 = 0u64;
                if nat_eq(a2874, a2875) {
                    true
                } else {
                    let a2876 = v13;
                    let a2877 = 1u64;
                    nat_eq(a2876, a2877)
                }
            } else {
                false
            } {
                let a2879 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2878, _, _, _, _, _) => {
                        h2878
                    }
                };
                let a2880 = v14;
                nat_eq(a2879, a2880)
            } else {
                false
            } {
                let a2888 = match v3.clone() {
                    Adt2::C0(_, h2881, _, _, _, _, _, _, _, _, _) => {
                        h2881
                    }
                };
                let a2889 = {
                    let a2886 = v9;
                    let a2887 = {
                        let a2884 = match v6.clone() {
                            Adt6::C0(h2882, _, _, _) => {
                                h2882
                            }
                        };
                        let a2885 = match v7.clone() {
                            Adt6::C0(h2883, _, _, _) => {
                                h2883
                            }
                        };
                        nat_mul(a2884, a2885)?
                    };
                    nat_mul(a2886, a2887)?
                };
                nat_eq(a2888, a2889)
            } else {
                false
            } {
                let a2892 = match v3.clone() {
                    Adt2::C0(_, _, h2890, _, _, _, _, _, _, _, _) => {
                        h2890
                    }
                };
                let a2893 = f80(v6.clone(), match v4.clone() {
                    Adt2::C0(_, h2891, _, _, _, _, _, _, _, _, _) => {
                        h2891
                    }
                })?;
                nat_eq(a2892, a2893)
            } else {
                false
            } {
                let a2895 = 0u64;
                let a2896 = match v3.clone() {
                    Adt2::C0(_, _, h2894, _, _, _, _, _, _, _, _) => {
                        h2894
                    }
                };
                nat_lt(a2895, a2896)
            } else {
                false
            } {
                if v5 {
                    let a2898 = match v1 {
                        Adt2::C0(_, _, _, h2897, _, _, _, _, _, _, _) => {
                            h2897
                        }
                    };
                    let a2899 = v9;
                    if if if nat_eq(a2898, a2899) {
                        let a2901 = match v3.clone() {
                            Adt2::C0(_, _, _, h2900, _, _, _, _, _, _, _) => {
                                h2900
                            }
                        };
                        let a2902 = f80(v7.clone(), v11)?;
                        nat_eq(a2901, a2902)
                    } else {
                        false
                    } {
                        let a2904 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2903, _, _, _, _, _, _) => {
                                h2903
                            }
                        };
                        let a2905 = v10;
                        nat_eq(a2904, a2905)
                    } else {
                        false
                    } {
                        let a2907 = 0u64;
                        let a2908 = match v3.clone() {
                            Adt2::C0(_, _, _, h2906, _, _, _, _, _, _, _) => {
                                h2906
                            }
                        };
                        nat_lt(a2907, a2908)
                    } else {
                        false
                    }
                } else {
                    let a2910 = match v1 {
                        Adt2::C0(_, _, h2909, _, _, _, _, _, _, _, _) => {
                            h2909
                        }
                    };
                    let a2911 = v9;
                    if if if nat_eq(a2910, a2911) {
                        let a2913 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h2912, _, _, _, _, _, _) => {
                                h2912
                            }
                        };
                        let a2914 = 1u64;
                        nat_eq(a2913, a2914)
                    } else {
                        false
                    } {
                        let a2916 = match v3.clone() {
                            Adt2::C0(_, _, _, h2915, _, _, _, _, _, _, _) => {
                                h2915
                            }
                        };
                        let a2917 = v10;
                        nat_eq(a2916, a2917)
                    } else {
                        false
                    } {
                        let a2919 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h2918, _, _, _, _, _, _) => {
                                h2918
                            }
                        };
                        let a2920 = 1u64;
                        nat_eq(a2919, a2920)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                if f40(match v4.clone() {
                    Adt2::C0(_, h2921, _, _, _, _, _, _, _, _, _) => {
                        h2921
                    }
                }, v11, v9, v10) {
                    true
                } else {
                    let a2941 = {
                        let a2938 = {
                            let a2936 = {
                                let a2930 = {
                                    let a2924 = {
                                        let a2922 = v10;
                                        let a2923 = 1u64;
                                        nat_sub(a2922, a2923)
                                    };
                                    let a2925 = v17;
                                    nat_mul(a2924, a2925)?
                                };
                                let a2931 = {
                                    let a2928 = {
                                        let a2926 = v9;
                                        let a2927 = 1u64;
                                        nat_sub(a2926, a2927)
                                    };
                                    let a2929 = v16;
                                    nat_mul(a2928, a2929)?
                                };
                                nat_add(a2930, a2931)?
                            };
                            let a2937 = {
                                let a2934 = {
                                    let a2932 = v11;
                                    let a2933 = 1u64;
                                    nat_sub(a2932, a2933)
                                };
                                let a2935 = v15;
                                nat_mul(a2934, a2935)?
                            };
                            nat_add(a2936, a2937)?
                        };
                        let a2939 = v15;
                        nat_add(a2938, a2939)?
                    };
                    let a2942 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, h2940, _) => {
                            h2940
                        }
                    };
                    nat_le(a2941, a2942)
                }
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f19(v3.clone(), 1u64)?, {
                    let a2943 = v13;
                    let a2944 = 0u64;
                    if nat_eq(a2943, a2944) {
                        4u64
                    } else {
                        2u64
                    }
                }, Adt3::C0(match v4 {
                    Adt2::C0(_, h2945, _, _, _, _, _, _, _, _, _) => {
                        h2945
                    }
                }, v11, 1u64, v10, v14, v15, 0u64, 0u64), if v5 {
                    f23(v3, 1u64)
                } else {
                    Adt3::C0(match v3.clone() {
                        Adt2::C0(_, h2946, _, _, _, _, _, _, _, _, _) => {
                            h2946
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h2947, _, _, _, _, _, _, _, _) => {
                            h2947
                        }
                    }, 1u64, match v3.clone() {
                        Adt2::C0(_, _, _, h2948, _, _, _, _, _, _, _) => {
                            h2948
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, h2949, _, _, _, _, _) => {
                            h2949
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, _, _, h2950, _, _, _, _) => {
                            h2950
                        }
                    }, 0u64, match v3 {
                        Adt2::C0(_, _, _, _, _, _, _, h2951, _, _, _) => {
                            h2951
                        }
                    })
                }, Adt4::C27(Adt5::C0(v6, v7, v8, v16, v17, v12, 1u64, v13))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f101(v0: Adt1, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m2952 = v2;
    match m2952 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let v5: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, h2953, _, _, _, _, _, _, _, _, _) => {
                    h2953
                }
            }, match v0.clone() {
                Adt1::C0(h2954, _, _, _, _, _, _, _, _, _, _, _) => {
                    h2954
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, h2955, _, _, _, _, _, _, _, _) => {
                    h2955
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, h2956, _, _, _, _, _) => {
                    h2956
                }
            });
            let v6: Adt6 = Adt6::C0(match v1.clone() {
                Adt2::C0(_, _, h2957, _, _, _, _, _, _, _, _) => {
                    h2957
                }
            }, match v0.clone() {
                Adt1::C0(_, h2958, _, _, _, _, _, _, _, _, _, _) => {
                    h2958
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, h2959, _, _, _, _, _, _, _) => {
                    h2959
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, h2960, _, _, _, _) => {
                    h2960
                }
            });
            let v7: Adt6 = Adt6::C0(match v1 {
                Adt2::C0(_, _, _, h2961, _, _, _, _, _, _, _) => {
                    h2961
                }
            }, match v0.clone() {
                Adt1::C0(_, _, h2962, _, _, _, _, _, _, _, _, _) => {
                    h2962
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, h2963, _, _, _, _, _, _) => {
                    h2963
                }
            }, match v0.clone() {
                Adt1::C0(_, _, _, _, _, _, _, _, h2964, _, _, _) => {
                    h2964
                }
            });
            let v8: u64 = match v0 {
                Adt1::C0(_, _, _, _, _, _, _, _, _, h2965, _, _) => {
                    h2965
                }
            };
            let v9: u64 = f80(v7.clone(), match v4.clone() {
                Adt2::C0(_, _, _, h2966, _, _, _, _, _, _, _) => {
                    h2966
                }
            })?;
            let v10: u64 = match v3.clone() {
                Adt2::C0(h2967, _, _, _, _, _, _, _, _, _, _) => {
                    h2967
                }
            };
            let v11: u64 = {
                let a2975 = v8;
                let a2976 = {
                    let a2973 = match v5.clone() {
                        Adt6::C0(h2968, _, _, _) => {
                            h2968
                        }
                    };
                    let a2974 = {
                        let a2971 = match v6.clone() {
                            Adt6::C0(h2969, _, _, _) => {
                                h2969
                            }
                        };
                        let a2972 = match v7.clone() {
                            Adt6::C0(h2970, _, _, _) => {
                                h2970
                            }
                        };
                        nat_mul(a2971, a2972)?
                    };
                    nat_mul(a2973, a2974)?
                };
                nat_mul(a2975, a2976)?
            };
            let a2978 = match v4.clone() {
                Adt2::C0(h2977, _, _, _, _, _, _, _, _, _, _) => {
                    h2977
                }
            };
            let a2979 = 0u64;
            if if if if if if if if if if if if if nat_eq(a2978, a2979) {
                let a2980 = v10;
                let a2981 = 0u64;
                if nat_eq(a2980, a2981) {
                    true
                } else {
                    let a2982 = v10;
                    let a2983 = 1u64;
                    nat_eq(a2982, a2983)
                }
            } else {
                false
            } {
                let a2984 = 0u64;
                let a2985 = v8;
                nat_lt(a2984, a2985)
            } else {
                false
            } {
                let a2986 = v8;
                let a2987 = 2147483648u64;
                nat_lt(a2986, a2987)
            } else {
                false
            } {
                let a2989 = match v4.clone() {
                    Adt2::C0(_, _, _, _, _, h2988, _, _, _, _, _) => {
                        h2988
                    }
                };
                let a2990 = 4u64;
                nat_eq(a2989, a2990)
            } else {
                false
            } {
                let a2991 = 0u64;
                let a2992 = v9;
                nat_lt(a2991, a2992)
            } else {
                false
            } {
                let a2994 = 0u64;
                let a2995 = match v3.clone() {
                    Adt2::C0(_, _, h2993, _, _, _, _, _, _, _, _) => {
                        h2993
                    }
                };
                nat_lt(a2994, a2995)
            } else {
                false
            } {
                let a2997 = 0u64;
                let a2998 = match v3.clone() {
                    Adt2::C0(_, _, _, h2996, _, _, _, _, _, _, _) => {
                        h2996
                    }
                };
                nat_lt(a2997, a2998)
            } else {
                false
            } {
                let a3000 = match v3.clone() {
                    Adt2::C0(_, h2999, _, _, _, _, _, _, _, _, _) => {
                        h2999
                    }
                };
                let a3001 = v11;
                nat_eq(a3000, a3001)
            } else {
                false
            } {
                let a3004 = match v3.clone() {
                    Adt2::C0(_, _, h3002, _, _, _, _, _, _, _, _) => {
                        h3002
                    }
                };
                let a3005 = f80(v5.clone(), match v4.clone() {
                    Adt2::C0(_, h3003, _, _, _, _, _, _, _, _, _) => {
                        h3003
                    }
                })?;
                nat_eq(a3004, a3005)
            } else {
                false
            } {
                let a3008 = match v3.clone() {
                    Adt2::C0(_, _, _, h3006, _, _, _, _, _, _, _) => {
                        h3006
                    }
                };
                let a3009 = f80(v6.clone(), match v4.clone() {
                    Adt2::C0(_, _, h3007, _, _, _, _, _, _, _, _) => {
                        h3007
                    }
                })?;
                nat_eq(a3008, a3009)
            } else {
                false
            } {
                let a3017 = match v3.clone() {
                    Adt2::C0(_, _, _, _, h3010, _, _, _, _, _, _) => {
                        h3010
                    }
                };
                let a3018 = {
                    let a3015 = v9;
                    let a3016 = {
                        let a3012 = match v4.clone() {
                            Adt2::C0(_, _, _, _, h3011, _, _, _, _, _, _) => {
                                h3011
                            }
                        };
                        let a3013 = v8;
                        let a3014 = 0u64;
                        nat_quot(a3012, a3013, a3014)
                    };
                    nat_mul(a3015, a3016)?
                };
                nat_eq(a3017, a3018)
            } else {
                false
            } {
                f20(v4.clone(), v3.clone())?
            } else {
                false
            } {
                Ok(Adt10::C4(f19(v3.clone(), 1u64)?, {
                    let a3019 = v10;
                    let a3020 = 0u64;
                    if nat_eq(a3019, a3020) {
                        4u64
                    } else {
                        2u64
                    }
                }, f23(v4.clone(), 1u64), f23(v3, 1u64), Adt4::C27(Adt5::C0(v5, v6, v7, match v4.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h3021, _, _) => {
                        h3021
                    }
                }, {
                    let a3023 = v8;
                    let a3024 = match v4 {
                        Adt2::C0(_, _, _, _, _, _, _, _, h3022, _, _) => {
                            h3022
                        }
                    };
                    nat_mul(a3023, a3024)?
                }, 0u64, v9, v10))))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f102(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3025 = v1;
    match m3025 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f202(v0, v3, v2)
        }
    }
}

pub fn f103(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3027 = match v0.clone() {
        Adt2::C0(h3026, _, _, _, _, _, _, _, _, _, _) => {
            h3026
        }
    };
    let a3028 = 1u64;
    if if nat_eq(a3027, a3028) {
        true
    } else {
        let a3030 = match v0.clone() {
            Adt2::C0(h3029, _, _, _, _, _, _, _, _, _, _) => {
                h3029
            }
        };
        let a3031 = 30u64;
        nat_eq(a3030, a3031)
    } {
        let a3033 = match v1.clone() {
            Adt2::C0(h3032, _, _, _, _, _, _, _, _, _, _) => {
                h3032
            }
        };
        let a3034 = 1u64;
        if if if if if if if if if if if nat_eq(a3033, a3034) {
            let a3037 = match v0.clone() {
                Adt2::C0(_, h3035, _, _, _, _, _, _, _, _, _) => {
                    h3035
                }
            };
            let a3038 = match v1.clone() {
                Adt2::C0(_, h3036, _, _, _, _, _, _, _, _, _) => {
                    h3036
                }
            };
            nat_eq(a3037, a3038)
        } else {
            false
        } {
            let a3040 = 0u64;
            let a3041 = match v1.clone() {
                Adt2::C0(_, _, h3039, _, _, _, _, _, _, _, _) => {
                    h3039
                }
            };
            nat_lt(a3040, a3041)
        } else {
            false
        } {
            let a3044 = match v1.clone() {
                Adt2::C0(_, _, h3042, _, _, _, _, _, _, _, _) => {
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
            let a3047 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3046, _, _, _, _, _, _) => {
                    h3046
                }
            };
            let a3048 = 1u64;
            nat_eq(a3047, a3048)
        } else {
            false
        } {
            let a3056 = match v0.clone() {
                Adt2::C0(_, _, h3049, _, _, _, _, _, _, _, _) => {
                    h3049
                }
            };
            let a3057 = {
                let a3054 = {
                    let a3052 = match v1.clone() {
                        Adt2::C0(_, _, h3050, _, _, _, _, _, _, _, _) => {
                            h3050
                        }
                    };
                    let a3053 = match v1.clone() {
                        Adt2::C0(_, _, h3051, _, _, _, _, _, _, _, _) => {
                            h3051
                        }
                    };
                    nat_add(a3052, a3053)?
                };
                let a3055 = 1u64;
                nat_sub(a3054, a3055)
            };
            nat_eq(a3056, a3057)
        } else {
            false
        } {
            let a3059 = 0u64;
            let a3060 = match v0.clone() {
                Adt2::C0(_, _, _, h3058, _, _, _, _, _, _, _) => {
                    h3058
                }
            };
            nat_lt(a3059, a3060)
        } else {
            false
        } {
            let a3062 = 0u64;
            let a3063 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3061, _, _, _, _, _, _) => {
                    h3061
                }
            };
            nat_lt(a3062, a3063)
        } else {
            false
        } {
            let a3065 = 2u64;
            let a3066 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h3064, _, _, _, _, _) => {
                    h3064
                }
            };
            nat_le(a3065, a3066)
        } else {
            false
        } {
            let a3068 = 2u64;
            let a3069 = match v1.clone() {
                Adt2::C0(_, _, _, _, _, h3067, _, _, _, _, _) => {
                    h3067
                }
            };
            nat_le(a3068, a3069)
        } else {
            false
        } {
            f20(v0.clone(), v1.clone())?
        } else {
            false
        } {
            Ok(Adt10::C4(f19(v1.clone(), 1u64)?, 2u64, f23(v0, 1u64), f23(v1, 1u64), Adt4::C21))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f104(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3070 = v1;
    match m3070 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            f203(v0, v3, v2)
        }
    }
}

pub fn f105(v0: u64) -> bool {
    let a3071 = v0;
    let a3072 = 1u64;
    if if nat_eq(a3071, a3072) {
        true
    } else {
        let a3073 = v0;
        let a3074 = 33u64;
        nat_eq(a3073, a3074)
    } {
        true
    } else {
        let a3075 = v0;
        let a3076 = 34u64;
        nat_eq(a3075, a3076)
    }
}

pub fn f106(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = f19(v0.clone(), 1u64)?;
    if if if if if f39(v1, v2.clone()) {
        let a3077 = v3;
        let a3078 = f19(v2.clone(), 1u64)?;
        nat_eq(a3077, a3078)
    } else {
        false
    } {
        f18(v0.clone(), v3)?
    } else {
        false
    } {
        f18(v2.clone(), v3)?
    } else {
        false
    } {
        let a3079 = v3;
        let a3080 = 0u64;
        if nat_eq(a3079, a3080) {
            true
        } else {
            let a3082 = 4u64;
            let a3083 = match v2.clone() {
                Adt2::C0(_, _, _, _, _, h3081, _, _, _, _, _) => {
                    h3081
                }
            };
            nat_le(a3082, a3083)
        }
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3084 = 2u64;
            let a3085 = v3;
            nat_mul(a3084, a3085)?
        }, 4u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C35(v3)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f107(v0: u64) -> bool {
    if f204(v0) {
        !f205(v0)
    } else {
        false
    }
}

pub fn f108(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3086 = v1;
    match m3086 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let m3088 = f197(match v0.clone() {
                Adt2::C0(h3087, _, _, _, _, _, _, _, _, _, _) => {
                    h3087
                }
            });
            match m3088 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v4.clone() {
                        Adt19::C0(_, h3089, _) => {
                            h3089
                        }
                    };
                    if if if if if if f39(v3, v2.clone()) {
                        f20(v0.clone(), v2.clone())?
                    } else {
                        false
                    } {
                        let a3090 = f19(v0.clone(), 1u64)?;
                        let a3091 = f19(v2.clone(), 1u64)?;
                        nat_eq(a3090, a3091)
                    } else {
                        false
                    } {
                        let a3096 = {
                            let a3093 = match v0.clone() {
                                Adt2::C0(_, h3092, _, _, _, _, _, _, _, _, _) => {
                                    h3092
                                }
                            };
                            let a3094 = v5;
                            let a3095 = 1u64;
                            nat_rem(a3093, a3094, a3095)
                        };
                        let a3097 = 0u64;
                        if nat_eq(a3096, a3097) {
                            let a3100 = match v4 {
                                Adt19::C0(_, _, h3098) => {
                                    h3098
                                }
                            };
                            let a3101 = match v0.clone() {
                                Adt2::C0(_, _, _, _, _, h3099, _, _, _, _, _) => {
                                    h3099
                                }
                            };
                            nat_le(a3100, a3101)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        let a3102 = v5;
                        let a3103 = 1u64;
                        if nat_eq(a3102, a3103) {
                            let a3105 = 4u64;
                            let a3106 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3104, _, _, _, _, _) => {
                                    h3104
                                }
                            };
                            nat_le(a3105, a3106)
                        } else {
                            let a3108 = match v2.clone() {
                                Adt2::C0(_, _, _, _, _, h3107, _, _, _, _, _) => {
                                    h3107
                                }
                            };
                            let a3109 = 4u64;
                            nat_eq(a3108, a3109)
                        }
                    } else {
                        false
                    } {
                        let a3114 = {
                            let a3111 = match v2.clone() {
                                Adt2::C0(_, h3110, _, _, _, _, _, _, _, _, _) => {
                                    h3110
                                }
                            };
                            let a3112 = v5;
                            let a3113 = 1u64;
                            nat_rem(a3111, a3112, a3113)
                        };
                        let a3115 = 0u64;
                        if nat_eq(a3114, a3115) {
                            true
                        } else {
                            f30(v2.clone())?
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f19(v0.clone(), v5)?, 4u64, f23(v0.clone(), v5), f23(v2, 1u64), Adt4::C11(match v0 {
                            Adt2::C0(h3116, _, _, _, _, _, _, _, _, _, _) => {
                                h3116
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

pub fn f109(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a3119 = match v1.clone() {
        Adt2::C0(h3117, _, _, _, _, _, _, _, _, _, _) => {
            h3117
        }
    };
    let a3120 = match v3.clone() {
        Adt2::C0(h3118, _, _, _, _, _, _, _, _, _, _) => {
            h3118
        }
    };
    if nat_eq(a3119, a3120) {
        f206(v0, v1, v2, v3)
    } else {
        if if if if f205(match v1.clone() {
            Adt2::C0(h3121, _, _, _, _, _, _, _, _, _, _) => {
                h3121
            }
        }) {
            f205(match v3.clone() {
                Adt2::C0(h3122, _, _, _, _, _, _, _, _, _, _) => {
                    h3122
                }
            })
        } else {
            false
        } {
            let a3123 = v0;
            let a3124 = 33u64;
            if nat_eq(a3123, a3124) {
                v2
            } else {
                true
            }
        } else {
            false
        } {
            let a3125 = f19(v1.clone(), 1u64)?;
            let a3126 = f19(v3.clone(), 1u64)?;
            nat_eq(a3125, a3126)
        } else {
            false
        } {
            let m3128 = f197(match v3.clone() {
                Adt2::C0(h3127, _, _, _, _, _, _, _, _, _, _) => {
                    h3127
                }
            });
            match m3128 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v4) => {
                    let v5: u64 = match v1.clone() {
                        Adt2::C0(h3129, _, _, _, _, _, _, _, _, _, _) => {
                            h3129
                        }
                    };
                    let v6: u64 = match v3.clone() {
                        Adt2::C0(h3130, _, _, _, _, _, _, _, _, _, _) => {
                            h3130
                        }
                    };
                    let v7: bool = {
                        let a3131 = v5;
                        let a3132 = 43u64;
                        if nat_eq(a3131, a3132) {
                            true
                        } else {
                            let a3133 = v6;
                            let a3134 = 43u64;
                            nat_eq(a3133, a3134)
                        }
                    };
                    Ok(Adt10::C4({
                        let a3135 = f19(v3.clone(), 1u64)?;
                        let a3136 = if v7 {
                            f150(v5, v6)
                        } else {
                            1u64
                        };
                        nat_mul(a3135, a3136)?
                    }, match v4 {
                        Adt19::C0(_, _, h3137) => {
                            h3137
                        }
                    }, f23(v1, 1u64), f23(v3, 1u64), if v7 {
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

pub fn f110(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3139 = match v0.clone() {
        Adt2::C0(h3138, _, _, _, _, _, _, _, _, _, _) => {
            h3138
        }
    };
    let a3140 = 0u64;
    if if if if nat_eq(a3139, a3140) {
        true
    } else {
        let a3142 = match v0.clone() {
            Adt2::C0(h3141, _, _, _, _, _, _, _, _, _, _) => {
                h3141
            }
        };
        let a3143 = 1u64;
        nat_eq(a3142, a3143)
    } {
        true
    } else {
        let a3145 = match v0.clone() {
            Adt2::C0(h3144, _, _, _, _, _, _, _, _, _, _) => {
                h3144
            }
        };
        let a3146 = 25u64;
        if nat_eq(a3145, a3146) {
            true
        } else {
            let a3148 = match v0.clone() {
                Adt2::C0(h3147, _, _, _, _, _, _, _, _, _, _) => {
                    h3147
                }
            };
            let a3149 = 26u64;
            nat_eq(a3148, a3149)
        }
    } {
        true
    } else {
        let a3151 = match v0.clone() {
            Adt2::C0(h3150, _, _, _, _, _, _, _, _, _, _) => {
                h3150
            }
        };
        let a3152 = 30u64;
        nat_eq(a3151, a3152)
    } {
        let a3155 = match v0.clone() {
            Adt2::C0(h3153, _, _, _, _, _, _, _, _, _, _) => {
                h3153
            }
        };
        let a3156 = match v1.clone() {
            Adt2::C0(h3154, _, _, _, _, _, _, _, _, _, _) => {
                h3154
            }
        };
        if if nat_eq(a3155, a3156) {
            f207(match v1.clone() {
                Adt2::C0(_, h3157, _, _, _, _, _, _, _, _, _) => {
                    h3157
                }
            }, match v1.clone() {
                Adt2::C0(_, _, h3158, _, _, _, _, _, _, _, _) => {
                    h3158
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, h3159, _, _, _, _, _, _, _) => {
                    h3159
                }
            }, match v1.clone() {
                Adt2::C0(_, _, _, _, h3160, _, _, _, _, _, _) => {
                    h3160
                }
            }, match v0.clone() {
                Adt2::C0(_, h3161, _, _, _, _, _, _, _, _, _) => {
                    h3161
                }
            }, match v0.clone() {
                Adt2::C0(_, _, h3162, _, _, _, _, _, _, _, _) => {
                    h3162
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, h3163, _, _, _, _, _, _, _) => {
                    h3163
                }
            }, match v0.clone() {
                Adt2::C0(_, _, _, _, h3164, _, _, _, _, _, _) => {
                    h3164
                }
            })
        } else {
            false
        } {
            let m3166 = f197(match v0.clone() {
                Adt2::C0(h3165, _, _, _, _, _, _, _, _, _, _) => {
                    h3165
                }
            });
            match m3166 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v2) => {
                    Ok(Adt10::C4(f19(v1.clone(), 1u64)?, match v2 {
                        Adt19::C0(_, _, h3167) => {
                            h3167
                        }
                    }, f23(v0, 1u64), f23(v1, 1u64), Adt4::C1))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f111(v0: u64, v1: Adt2, v2: Option<Adt2>, v3: Adt2) -> R<Adt10> {
    let m3168 = v2;
    match m3168 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v4) => {
            let a3169 = v0;
            let a3170 = 4u64;
            if if if nat_lt(a3169, a3170) {
                let a3173 = match v1.clone() {
                    Adt2::C0(h3171, _, _, _, _, _, _, _, _, _, _) => {
                        h3171
                    }
                };
                let a3174 = match v4.clone() {
                    Adt2::C0(h3172, _, _, _, _, _, _, _, _, _, _) => {
                        h3172
                    }
                };
                if nat_eq(a3173, a3174) {
                    let a3177 = match v1.clone() {
                        Adt2::C0(h3175, _, _, _, _, _, _, _, _, _, _) => {
                            h3175
                        }
                    };
                    let a3178 = match v3.clone() {
                        Adt2::C0(h3176, _, _, _, _, _, _, _, _, _, _) => {
                            h3176
                        }
                    };
                    nat_eq(a3177, a3178)
                } else {
                    false
                }
            } else {
                false
            } {
                if if if f208(v0, 0u64, match v1.clone() {
                    Adt2::C0(_, h3179, _, _, _, _, _, _, _, _, _) => {
                        h3179
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h3180, _, _, _, _, _, _, _, _, _) => {
                        h3180
                    }
                }, match v3.clone() {
                    Adt2::C0(_, h3181, _, _, _, _, _, _, _, _, _) => {
                        h3181
                    }
                })? {
                    f208(v0, 1u64, match v1.clone() {
                        Adt2::C0(_, _, h3182, _, _, _, _, _, _, _, _) => {
                            h3182
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, h3183, _, _, _, _, _, _, _, _) => {
                            h3183
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, h3184, _, _, _, _, _, _, _, _) => {
                            h3184
                        }
                    })?
                } else {
                    false
                } {
                    f208(v0, 2u64, match v1.clone() {
                        Adt2::C0(_, _, _, h3185, _, _, _, _, _, _, _) => {
                            h3185
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, h3186, _, _, _, _, _, _, _) => {
                            h3186
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, h3187, _, _, _, _, _, _, _) => {
                            h3187
                        }
                    })?
                } else {
                    false
                } {
                    f208(v0, 3u64, match v1.clone() {
                        Adt2::C0(_, _, _, _, h3188, _, _, _, _, _, _) => {
                            h3188
                        }
                    }, match v4.clone() {
                        Adt2::C0(_, _, _, _, h3189, _, _, _, _, _, _) => {
                            h3189
                        }
                    }, match v3.clone() {
                        Adt2::C0(_, _, _, _, h3190, _, _, _, _, _, _) => {
                            h3190
                        }
                    })?
                } else {
                    false
                }
            } else {
                false
            } {
                let m3192 = f197(match v1.clone() {
                    Adt2::C0(h3191, _, _, _, _, _, _, _, _, _, _) => {
                        h3191
                    }
                });
                match m3192 {
                    Adt18::C0 => {
                        Ok(Adt10::C0)
                    }
                    Adt18::C1(v5) => {
                        let v6: u64 = match v5.clone() {
                            Adt19::C0(_, h3193, _) => {
                                h3193
                            }
                        };
                        let a3198 = {
                            let a3195 = match v1.clone() {
                                Adt2::C0(_, h3194, _, _, _, _, _, _, _, _, _) => {
                                    h3194
                                }
                            };
                            let a3196 = v6;
                            let a3197 = 1u64;
                            nat_rem(a3195, a3196, a3197)
                        };
                        let a3199 = 0u64;
                        if if if nat_eq(a3198, a3199) {
                            let a3204 = {
                                let a3201 = match v4.clone() {
                                    Adt2::C0(_, h3200, _, _, _, _, _, _, _, _, _) => {
                                        h3200
                                    }
                                };
                                let a3202 = v6;
                                let a3203 = 1u64;
                                nat_rem(a3201, a3202, a3203)
                            };
                            let a3205 = 0u64;
                            nat_eq(a3204, a3205)
                        } else {
                            false
                        } {
                            let a3210 = {
                                let a3207 = match v3.clone() {
                                    Adt2::C0(_, h3206, _, _, _, _, _, _, _, _, _) => {
                                        h3206
                                    }
                                };
                                let a3208 = v6;
                                let a3209 = 1u64;
                                nat_rem(a3207, a3208, a3209)
                            };
                            let a3211 = 0u64;
                            nat_eq(a3210, a3211)
                        } else {
                            false
                        } {
                            Ok(Adt10::C4(f19(v3.clone(), v6)?, match v5 {
                                Adt19::C0(_, _, h3212) => {
                                    h3212
                                }
                            }, f23(v1, v6), f23(v3, v6), Adt4::C2(v0, f23(v4, v6))))
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

pub fn f112(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3214 = match v2.clone() {
        Adt2::C0(h3213, _, _, _, _, _, _, _, _, _, _) => {
            h3213
        }
    };
    let a3215 = 0u64;
    if if nat_eq(a3214, a3215) {
        let a3217 = match v3.clone() {
            Adt2::C0(h3216, _, _, _, _, _, _, _, _, _, _) => {
                h3216
            }
        };
        let a3218 = 0u64;
        nat_eq(a3217, a3218)
    } else {
        false
    } {
        let a3219 = v0;
        let a3220 = 61u64;
        if nat_eq(a3219, a3220) {
            let v4: bool = {
                let a3222 = 0u64;
                let a3223 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3221, _, _, _) => {
                        h3221
                    }
                };
                nat_lt(a3222, a3223)
            };
            if if if if if if f209(match v2.clone() {
                Adt2::C0(_, h3224, _, _, _, _, _, _, _, _, _) => {
                    h3224
                }
            }, match v1.clone() {
                Adt1::C0(h3225, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3225
                }
            }, match v1.clone() {
                Adt1::C0(_, h3226, _, _, _, _, _, _, _, _, _, _) => {
                    h3226
                }
            }, match v3.clone() {
                Adt2::C0(_, h3227, _, _, _, _, _, _, _, _, _) => {
                    h3227
                }
            })? {
                f209(match v2.clone() {
                    Adt2::C0(_, _, h3228, _, _, _, _, _, _, _, _) => {
                        h3228
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3229, _, _, _, _, _, _, _, _, _) => {
                        h3229
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, h3230, _, _, _, _, _, _, _, _) => {
                        h3230
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, h3231, _, _, _, _, _, _, _, _) => {
                        h3231
                    }
                })?
            } else {
                false
            } {
                f209(match v2.clone() {
                    Adt2::C0(_, _, _, h3232, _, _, _, _, _, _, _) => {
                        h3232
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3233, _, _, _, _, _, _, _) => {
                        h3233
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, h3234, _, _, _, _, _, _) => {
                        h3234
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, h3235, _, _, _, _, _, _, _) => {
                        h3235
                    }
                })?
            } else {
                false
            } {
                f209(match v2.clone() {
                    Adt2::C0(_, _, _, _, h3236, _, _, _, _, _, _) => {
                        h3236
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, h3237, _, _, _, _, _) => {
                        h3237
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, h3238, _, _, _, _) => {
                        h3238
                    }
                }, match v3.clone() {
                    Adt2::C0(_, _, _, _, h3239, _, _, _, _, _, _) => {
                        h3239
                    }
                })?
            } else {
                false
            } {
                let a3241 = match v1.clone() {
                    Adt1::C0(_, _, _, _, _, _, _, _, h3240, _, _, _) => {
                        h3240
                    }
                };
                let a3242 = 4294967295u64;
                nat_le(a3241, a3242)
            } else {
                false
            } {
                if v4 {
                    !f40(match v2.clone() {
                        Adt2::C0(_, h3243, _, _, _, _, _, _, _, _, _) => {
                            h3243
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, h3244, _, _, _, _, _, _, _, _) => {
                            h3244
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, h3245, _, _, _, _, _, _, _) => {
                            h3245
                        }
                    }, match v2.clone() {
                        Adt2::C0(_, _, _, _, h3246, _, _, _, _, _, _) => {
                            h3246
                        }
                    })
                } else {
                    true
                }
            } else {
                false
            } {
                Ok(Adt10::C4(f19(v3.clone(), 1u64)?, 4u64, f23(v2, 1u64), f23(v3, 1u64), Adt4::C3(match v1.clone() {
                    Adt1::C0(h3247, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3247
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, h3248, _, _, _, _, _, _, _, _, _) => {
                        h3248
                    }
                }, match v1.clone() {
                    Adt1::C0(_, _, _, _, h3249, _, _, _, _, _, _, _) => {
                        h3249
                    }
                }, match v1 {
                    Adt1::C0(_, _, _, _, _, _, h3250, _, _, _, _, _) => {
                        h3250
                    }
                }, v4)))
            } else {
                Ok(Adt10::C1)
            }
        } else {
            let a3251 = v0;
            let a3252 = 62u64;
            if nat_eq(a3251, a3252) {
                let a3254 = match v1.clone() {
                    Adt1::C0(h3253, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3253
                    }
                };
                let a3255 = 2147483648u64;
                if if if if if nat_lt(a3254, a3255) {
                    let a3257 = match v1.clone() {
                        Adt1::C0(_, h3256, _, _, _, _, _, _, _, _, _, _) => {
                            h3256
                        }
                    };
                    let a3258 = 2147483648u64;
                    nat_lt(a3257, a3258)
                } else {
                    false
                } {
                    let a3261 = match v1.clone() {
                        Adt1::C0(h3259, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3259
                        }
                    };
                    let a3262 = match v2.clone() {
                        Adt2::C0(_, h3260, _, _, _, _, _, _, _, _, _) => {
                            h3260
                        }
                    };
                    nat_lt(a3261, a3262)
                } else {
                    false
                } {
                    let a3265 = match v1.clone() {
                        Adt1::C0(_, h3263, _, _, _, _, _, _, _, _, _, _) => {
                            h3263
                        }
                    };
                    let a3266 = match v2.clone() {
                        Adt2::C0(_, h3264, _, _, _, _, _, _, _, _, _) => {
                            h3264
                        }
                    };
                    nat_lt(a3265, a3266)
                } else {
                    false
                } {
                    let a3269 = match v2.clone() {
                        Adt2::C0(_, _, h3267, _, _, _, _, _, _, _, _) => {
                            h3267
                        }
                    };
                    let a3270 = match v3.clone() {
                        Adt2::C0(_, _, h3268, _, _, _, _, _, _, _, _) => {
                            h3268
                        }
                    };
                    if if if nat_eq(a3269, a3270) {
                        let a3273 = match v2.clone() {
                            Adt2::C0(_, _, _, h3271, _, _, _, _, _, _, _) => {
                                h3271
                            }
                        };
                        let a3274 = match v3.clone() {
                            Adt2::C0(_, _, _, h3272, _, _, _, _, _, _, _) => {
                                h3272
                            }
                        };
                        nat_eq(a3273, a3274)
                    } else {
                        false
                    } {
                        let a3277 = match v2.clone() {
                            Adt2::C0(_, _, _, _, h3275, _, _, _, _, _, _) => {
                                h3275
                            }
                        };
                        let a3278 = match v3.clone() {
                            Adt2::C0(_, _, _, _, h3276, _, _, _, _, _, _) => {
                                h3276
                            }
                        };
                        nat_eq(a3277, a3278)
                    } else {
                        false
                    } {
                        let a3287 = match v3.clone() {
                            Adt2::C0(_, h3279, _, _, _, _, _, _, _, _, _) => {
                                h3279
                            }
                        };
                        let a3288 = {
                            let a3285 = {
                                let a3282 = match v2.clone() {
                                    Adt2::C0(_, h3280, _, _, _, _, _, _, _, _, _) => {
                                        h3280
                                    }
                                };
                                let a3283 = match v1.clone() {
                                    Adt1::C0(h3281, _, _, _, _, _, _, _, _, _, _, _) => {
                                        h3281
                                    }
                                };
                                nat_add(a3282, a3283)?
                            };
                            let a3286 = match v1.clone() {
                                Adt1::C0(_, h3284, _, _, _, _, _, _, _, _, _, _) => {
                                    h3284
                                }
                            };
                            nat_add(a3285, a3286)?
                        };
                        nat_eq(a3287, a3288)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    Ok(Adt10::C4(f19(v3.clone(), 1u64)?, 4u64, f23(v2, 1u64), f23(v3, 1u64), Adt4::C4(match v1 {
                        Adt1::C0(h3289, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3289
                        }
                    })))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                let a3290 = v0;
                let a3291 = 63u64;
                if nat_eq(a3290, a3291) {
                    if if if if if f210(match v2.clone() {
                        Adt2::C0(_, h3292, _, _, _, _, _, _, _, _, _) => {
                            h3292
                        }
                    }, match v1.clone() {
                        Adt1::C0(h3293, _, _, _, _, _, _, _, _, _, _, _) => {
                            h3293
                        }
                    })? {
                        f210(match v2.clone() {
                            Adt2::C0(_, _, h3294, _, _, _, _, _, _, _, _) => {
                                h3294
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3295, _, _, _, _, _, _, _, _, _, _) => {
                                h3295
                            }
                        })?
                    } else {
                        false
                    } {
                        f210(match v2.clone() {
                            Adt2::C0(_, _, _, h3296, _, _, _, _, _, _, _) => {
                                h3296
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3297, _, _, _, _, _, _, _, _, _) => {
                                h3297
                            }
                        })?
                    } else {
                        false
                    } {
                        f210(match v2.clone() {
                            Adt2::C0(_, _, _, _, h3298, _, _, _, _, _, _) => {
                                h3298
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, _, h3299, _, _, _, _, _, _, _, _) => {
                                h3299
                            }
                        })?
                    } else {
                        false
                    } {
                        let a3302 = match v2.clone() {
                            Adt2::C0(_, h3300, _, _, _, _, _, _, _, _, _) => {
                                h3300
                            }
                        };
                        let a3303 = match v3.clone() {
                            Adt2::C0(_, h3301, _, _, _, _, _, _, _, _, _) => {
                                h3301
                            }
                        };
                        if if if nat_eq(a3302, a3303) {
                            let a3306 = match v2.clone() {
                                Adt2::C0(_, _, h3304, _, _, _, _, _, _, _, _) => {
                                    h3304
                                }
                            };
                            let a3307 = match v3.clone() {
                                Adt2::C0(_, _, h3305, _, _, _, _, _, _, _, _) => {
                                    h3305
                                }
                            };
                            nat_eq(a3306, a3307)
                        } else {
                            false
                        } {
                            let a3310 = match v2.clone() {
                                Adt2::C0(_, _, _, h3308, _, _, _, _, _, _, _) => {
                                    h3308
                                }
                            };
                            let a3311 = match v3.clone() {
                                Adt2::C0(_, _, _, h3309, _, _, _, _, _, _, _) => {
                                    h3309
                                }
                            };
                            nat_eq(a3310, a3311)
                        } else {
                            false
                        } {
                            let a3314 = match v2.clone() {
                                Adt2::C0(_, _, _, _, h3312, _, _, _, _, _, _) => {
                                    h3312
                                }
                            };
                            let a3315 = match v3.clone() {
                                Adt2::C0(_, _, _, _, h3313, _, _, _, _, _, _) => {
                                    h3313
                                }
                            };
                            nat_eq(a3314, a3315)
                        } else {
                            false
                        }
                    } else {
                        false
                    } {
                        Ok(Adt10::C4(f19(v3.clone(), 1u64)?, 4u64, f23(v2, 1u64), f23(v3, 1u64), Adt4::C5(match v1.clone() {
                            Adt1::C0(h3316, _, _, _, _, _, _, _, _, _, _, _) => {
                                h3316
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, h3317, _, _, _, _, _, _, _, _, _, _) => {
                                h3317
                            }
                        }, match v1.clone() {
                            Adt1::C0(_, _, h3318, _, _, _, _, _, _, _, _, _) => {
                                h3318
                            }
                        }, match v1 {
                            Adt1::C0(_, _, _, h3319, _, _, _, _, _, _, _, _) => {
                                h3319
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

pub fn f113(v0: u64, v1: Adt1, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3320 = v3;
    match m3320 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v5) => {
            let v6: bool = {
                let a3321 = v0;
                let a3322 = 4u64;
                nat_eq(a3321, a3322)
            };
            let v7: bool = {
                let a3324 = 0u64;
                let a3325 = match v1.clone() {
                    Adt1::C0(_, _, _, _, h3323, _, _, _, _, _, _, _) => {
                        h3323
                    }
                };
                nat_lt(a3324, a3325)
            };
            let v8: u64 = f19(v5.clone(), 1u64)?;
            let v9: Adt3 = Adt3::C0(match v5.clone() {
                Adt2::C0(_, h3326, _, _, _, _, _, _, _, _, _) => {
                    h3326
                }
            }, match v5.clone() {
                Adt2::C0(_, _, h3327, _, _, _, _, _, _, _, _) => {
                    h3327
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, h3328, _, _, _, _, _, _, _) => {
                    h3328
                }
            }, match v5.clone() {
                Adt2::C0(_, _, _, _, h3329, _, _, _, _, _, _) => {
                    h3329
                }
            }, 4u64, match v1.clone() {
                Adt1::C0(h3330, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3330
                }
            }, match v1.clone() {
                Adt1::C0(_, h3331, _, _, _, _, _, _, _, _, _, _) => {
                    h3331
                }
            }, match v1.clone() {
                Adt1::C0(_, _, h3332, _, _, _, _, _, _, _, _, _) => {
                    h3332
                }
            });
            if if if if if if if if if if if if v6 {
                let a3334 = match v2.clone() {
                    Adt2::C0(h3333, _, _, _, _, _, _, _, _, _, _) => {
                        h3333
                    }
                };
                let a3335 = 0u64;
                if nat_eq(a3334, a3335) {
                    let a3337 = match v5.clone() {
                        Adt2::C0(h3336, _, _, _, _, _, _, _, _, _, _) => {
                            h3336
                        }
                    };
                    let a3338 = 0u64;
                    nat_eq(a3337, a3338)
                } else {
                    false
                }
            } else {
                let a3340 = match v2.clone() {
                    Adt2::C0(h3339, _, _, _, _, _, _, _, _, _, _) => {
                        h3339
                    }
                };
                let a3341 = 0u64;
                if if nat_eq(a3340, a3341) {
                    true
                } else {
                    let a3343 = match v2.clone() {
                        Adt2::C0(h3342, _, _, _, _, _, _, _, _, _, _) => {
                            h3342
                        }
                    };
                    let a3344 = 26u64;
                    nat_eq(a3343, a3344)
                } {
                    let a3346 = match v5.clone() {
                        Adt2::C0(h3345, _, _, _, _, _, _, _, _, _, _) => {
                            h3345
                        }
                    };
                    let a3347 = 0u64;
                    if nat_eq(a3346, a3347) {
                        true
                    } else {
                        let a3349 = match v5.clone() {
                            Adt2::C0(h3348, _, _, _, _, _, _, _, _, _, _) => {
                                h3348
                            }
                        };
                        let a3350 = 26u64;
                        nat_eq(a3349, a3350)
                    }
                } else {
                    false
                }
            } {
                let a3353 = match v2.clone() {
                    Adt2::C0(h3351, _, _, _, _, _, _, _, _, _, _) => {
                        h3351
                    }
                };
                let a3354 = match v4.clone() {
                    Adt2::C0(h3352, _, _, _, _, _, _, _, _, _, _) => {
                        h3352
                    }
                };
                nat_eq(a3353, a3354)
            } else {
                false
            } {
                f24(v2.clone(), v4.clone())
            } else {
                false
            } {
                f30(v2.clone())?
            } else {
                false
            } {
                f30(v4.clone())?
            } else {
                false
            } {
                let a3356 = match v5.clone() {
                    Adt2::C0(_, _, _, _, _, h3355, _, _, _, _, _) => {
                        h3355
                    }
                };
                let a3357 = 4u64;
                nat_eq(a3356, a3357)
            } else {
                false
            } {
                let a3358 = v8;
                let a3359 = f19(v2.clone(), 1u64)?;
                nat_le(a3358, a3359)
            } else {
                false
            } {
                let a3361 = match v1.clone() {
                    Adt1::C0(_, _, _, h3360, _, _, _, _, _, _, _, _) => {
                        h3360
                    }
                };
                let a3362 = 1073741824u64;
                nat_lt(a3361, a3362)
            } else {
                false
            } {
                let a3364 = match v1.clone() {
                    Adt1::C0(h3363, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3363
                    }
                };
                let a3365 = 4294967295u64;
                if if if if nat_le(a3364, a3365) {
                    let a3367 = match v1.clone() {
                        Adt1::C0(_, h3366, _, _, _, _, _, _, _, _, _, _) => {
                            h3366
                        }
                    };
                    let a3368 = 4294967295u64;
                    nat_le(a3367, a3368)
                } else {
                    false
                } {
                    let a3370 = match v1.clone() {
                        Adt1::C0(_, _, h3369, _, _, _, _, _, _, _, _, _) => {
                            h3369
                        }
                    };
                    let a3371 = 4294967295u64;
                    nat_le(a3370, a3371)
                } else {
                    false
                } {
                    let a3373 = match v1.clone() {
                        Adt1::C0(_, _, _, h3372, _, _, _, _, _, _, _, _) => {
                            h3372
                        }
                    };
                    let a3374 = 4294967295u64;
                    nat_le(a3373, a3374)
                } else {
                    false
                } {
                    let a3376 = match v1.clone() {
                        Adt1::C0(_, _, _, _, h3375, _, _, _, _, _, _, _) => {
                            h3375
                        }
                    };
                    let a3377 = 4294967295u64;
                    nat_le(a3376, a3377)
                } else {
                    false
                }
            } else {
                false
            } {
                if v7 {
                    let a3380 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3378) => {
                            h3378
                        }
                    };
                    let a3381 = match v4.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, _, _, h3379) => {
                            h3379
                        }
                    };
                    nat_eq(a3380, a3381)
                } else {
                    true
                }
            } else {
                false
            } {
                f211(v9.clone(), match v1.clone() {
                    Adt1::C0(_, _, _, h3382, _, _, _, _, _, _, _, _) => {
                        h3382
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h3383, _) => {
                        h3383
                    }
                }, v8, v6)?
            } else {
                false
            } {
                let v10: u64 = if v7 {
                    0u64
                } else {
                    f19(v2, 1u64)?
                };
                Ok(Adt10::C4({
                    let a3384 = v10;
                    let a3385 = v8;
                    nat_add(a3384, a3385)?
                }, 4u64, f23(v5, 1u64), v9, Adt4::C6(v10, match v1 {
                    Adt1::C0(_, _, _, h3386, _, _, _, _, _, _, _, _) => {
                        h3386
                    }
                }, v6)))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f114(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3388 = match v0.clone() {
        Adt2::C0(h3387, _, _, _, _, _, _, _, _, _, _) => {
            h3387
        }
    };
    let a3389 = 0u64;
    if if if nat_eq(a3388, a3389) {
        let a3391 = match v1.clone() {
            Adt2::C0(h3390, _, _, _, _, _, _, _, _, _, _) => {
                h3390
            }
        };
        let a3392 = 0u64;
        nat_eq(a3391, a3392)
    } else {
        false
    } {
        f207(match v0.clone() {
            Adt2::C0(_, h3393, _, _, _, _, _, _, _, _, _) => {
                h3393
            }
        }, match v0.clone() {
            Adt2::C0(_, _, h3394, _, _, _, _, _, _, _, _) => {
                h3394
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, h3395, _, _, _, _, _, _, _) => {
                h3395
            }
        }, match v0.clone() {
            Adt2::C0(_, _, _, _, h3396, _, _, _, _, _, _) => {
                h3396
            }
        }, match v1.clone() {
            Adt2::C0(_, h3397, _, _, _, _, _, _, _, _, _) => {
                h3397
            }
        }, match v1.clone() {
            Adt2::C0(_, _, h3398, _, _, _, _, _, _, _, _) => {
                h3398
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, h3399, _, _, _, _, _, _, _) => {
                h3399
            }
        }, match v1.clone() {
            Adt2::C0(_, _, _, _, h3400, _, _, _, _, _, _) => {
                h3400
            }
        })
    } else {
        false
    } {
        let v2: u64 = f19(v1.clone(), 1u64)?;
        Ok(Adt10::C4({
            let a3401 = v2;
            let a3402 = f19(v0.clone(), 1u64)?;
            nat_add(a3401, a3402)?
        }, 4u64, f23(v0, 1u64), f23(v1, 1u64), Adt4::C7(v2)))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f115(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3404 = match v1.clone() {
        Adt2::C0(h3403, _, _, _, _, _, _, _, _, _, _) => {
            h3403
        }
    };
    let a3405 = 0u64;
    if if if nat_eq(a3404, a3405) {
        true
    } else {
        let a3407 = match v1.clone() {
            Adt2::C0(h3406, _, _, _, _, _, _, _, _, _, _) => {
                h3406
            }
        };
        let a3408 = 1u64;
        nat_eq(a3407, a3408)
    } {
        true
    } else {
        let a3410 = match v1.clone() {
            Adt2::C0(h3409, _, _, _, _, _, _, _, _, _, _) => {
                h3409
            }
        };
        let a3411 = 30u64;
        nat_eq(a3410, a3411)
    } {
        let a3414 = match v1.clone() {
            Adt2::C0(h3412, _, _, _, _, _, _, _, _, _, _) => {
                h3412
            }
        };
        let a3415 = match v2.clone() {
            Adt2::C0(h3413, _, _, _, _, _, _, _, _, _, _) => {
                h3413
            }
        };
        if if if if if nat_eq(a3414, a3415) {
            let a3418 = match v1.clone() {
                Adt2::C0(_, h3416, _, _, _, _, _, _, _, _, _) => {
                    h3416
                }
            };
            let a3419 = match v2.clone() {
                Adt2::C0(_, h3417, _, _, _, _, _, _, _, _, _) => {
                    h3417
                }
            };
            nat_eq(a3418, a3419)
        } else {
            false
        } {
            let a3422 = match v1.clone() {
                Adt2::C0(_, _, _, h3420, _, _, _, _, _, _, _) => {
                    h3420
                }
            };
            let a3423 = match v2.clone() {
                Adt2::C0(_, _, _, h3421, _, _, _, _, _, _, _) => {
                    h3421
                }
            };
            nat_eq(a3422, a3423)
        } else {
            false
        } {
            let a3426 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3424, _, _, _, _, _, _) => {
                    h3424
                }
            };
            let a3427 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3425, _, _, _, _, _, _) => {
                    h3425
                }
            };
            nat_eq(a3426, a3427)
        } else {
            false
        } {
            let a3428 = v0;
            let a3429 = 4u64;
            if nat_eq(a3428, a3429) {
                let a3431 = match v1.clone() {
                    Adt2::C0(_, _, h3430, _, _, _, _, _, _, _, _) => {
                        h3430
                    }
                };
                let a3432 = 1u64;
                if nat_eq(a3431, a3432) {
                    let a3435 = match v1.clone() {
                        Adt2::C0(_, h3433, _, _, _, _, _, _, _, _, _) => {
                            h3433
                        }
                    };
                    let a3436 = match v2.clone() {
                        Adt2::C0(_, _, h3434, _, _, _, _, _, _, _, _) => {
                            h3434
                        }
                    };
                    nat_eq(a3435, a3436)
                } else {
                    false
                }
            } else {
                let a3437 = v0;
                let a3438 = 4u64;
                if if nat_lt(a3437, a3438) {
                    let a3441 = match v1.clone() {
                        Adt2::C0(_, h3439, _, _, _, _, _, _, _, _, _) => {
                            h3439
                        }
                    };
                    let a3442 = match v1.clone() {
                        Adt2::C0(_, _, h3440, _, _, _, _, _, _, _, _) => {
                            h3440
                        }
                    };
                    nat_eq(a3441, a3442)
                } else {
                    false
                } {
                    let a3445 = match v1.clone() {
                        Adt2::C0(_, _, h3443, _, _, _, _, _, _, _, _) => {
                            h3443
                        }
                    };
                    let a3446 = match v2.clone() {
                        Adt2::C0(_, _, h3444, _, _, _, _, _, _, _, _) => {
                            h3444
                        }
                    };
                    nat_eq(a3445, a3446)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let m3448 = f197(match v1.clone() {
                Adt2::C0(h3447, _, _, _, _, _, _, _, _, _, _) => {
                    h3447
                }
            });
            match m3448 {
                Adt18::C0 => {
                    Ok(Adt10::C0)
                }
                Adt18::C1(v3) => {
                    Ok(Adt10::C4(f19(v2.clone(), 1u64)?, match v3 {
                        Adt19::C0(_, _, h3449) => {
                            h3449
                        }
                    }, f23(v1, 1u64), f23(v2, 1u64), Adt4::C8(v0)))
                }
            }
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f116(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if f212(match v1.clone() {
        Adt2::C0(h3450, _, _, _, _, _, _, _, _, _, _) => {
            h3450
        }
    }, v0) {
        let a3453 = match v1.clone() {
            Adt2::C0(h3451, _, _, _, _, _, _, _, _, _, _) => {
                h3451
            }
        };
        let a3454 = match v2.clone() {
            Adt2::C0(h3452, _, _, _, _, _, _, _, _, _, _) => {
                h3452
            }
        };
        nat_eq(a3453, a3454)
    } else {
        false
    } {
        let a3457 = match v1.clone() {
            Adt2::C0(_, h3455, _, _, _, _, _, _, _, _, _) => {
                h3455
            }
        };
        let a3458 = match v2.clone() {
            Adt2::C0(_, h3456, _, _, _, _, _, _, _, _, _) => {
                h3456
            }
        };
        if if if nat_eq(a3457, a3458) {
            let a3461 = match v1.clone() {
                Adt2::C0(_, _, h3459, _, _, _, _, _, _, _, _) => {
                    h3459
                }
            };
            let a3462 = match v2.clone() {
                Adt2::C0(_, _, h3460, _, _, _, _, _, _, _, _) => {
                    h3460
                }
            };
            nat_eq(a3461, a3462)
        } else {
            false
        } {
            let a3465 = match v1.clone() {
                Adt2::C0(_, _, _, h3463, _, _, _, _, _, _, _) => {
                    h3463
                }
            };
            let a3466 = match v2.clone() {
                Adt2::C0(_, _, _, h3464, _, _, _, _, _, _, _) => {
                    h3464
                }
            };
            nat_eq(a3465, a3466)
        } else {
            false
        } {
            let a3469 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3467, _, _, _, _, _, _) => {
                    h3467
                }
            };
            let a3470 = match v2.clone() {
                Adt2::C0(_, _, _, _, h3468, _, _, _, _, _, _) => {
                    h3468
                }
            };
            nat_eq(a3469, a3470)
        } else {
            false
        }
    } else {
        false
    } {
        let m3472 = f197(match v1.clone() {
            Adt2::C0(h3471, _, _, _, _, _, _, _, _, _, _) => {
                h3471
            }
        });
        match m3472 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                Ok(Adt10::C4(f19(v2.clone(), 1u64)?, match v3 {
                    Adt19::C0(_, _, h3473) => {
                        h3473
                    }
                }, f23(v1.clone(), 1u64), f23(v2, 1u64), Adt4::C9(match v1 {
                    Adt2::C0(h3474, _, _, _, _, _, _, _, _, _, _) => {
                        h3474
                    }
                }, v0)))
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f117(v0: bool, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let v4: u64 = if v0 {
        match v1.clone() {
            Adt1::C0(h3475, _, _, _, _, _, _, _, _, _, _, _) => {
                h3475
            }
        }
    } else {
        match v1.clone() {
            Adt1::C0(_, _, h3476, _, _, _, _, _, _, _, _, _) => {
                h3476
            }
        }
    };
    let a3477 = 0u64;
    let a3478 = v4;
    if if nat_lt(a3477, a3478) {
        let a3479 = v4;
        let a3480 = 2147483648u64;
        nat_lt(a3479, a3480)
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
        let v7: u64 = f213(match v5.clone() {
            Adt2::C0(_, _, h3481, _, _, _, _, _, _, _, _) => {
                h3481
            }
        }, v4)?;
        let v8: u64 = f213(match v5.clone() {
            Adt2::C0(_, _, _, h3482, _, _, _, _, _, _, _) => {
                h3482
            }
        }, v4)?;
        let a3484 = match v5.clone() {
            Adt2::C0(h3483, _, _, _, _, _, _, _, _, _, _) => {
                h3483
            }
        };
        let a3485 = 0u64;
        if if if if if nat_eq(a3484, a3485) {
            let a3487 = match v6.clone() {
                Adt2::C0(h3486, _, _, _, _, _, _, _, _, _, _) => {
                    h3486
                }
            };
            let a3488 = 0u64;
            nat_eq(a3487, a3488)
        } else {
            false
        } {
            let a3490 = match v5.clone() {
                Adt2::C0(_, _, _, _, h3489, _, _, _, _, _, _) => {
                    h3489
                }
            };
            let a3491 = 1u64;
            nat_eq(a3490, a3491)
        } else {
            false
        } {
            let a3494 = match v5.clone() {
                Adt2::C0(_, h3492, _, _, _, _, _, _, _, _, _) => {
                    h3492
                }
            };
            let a3495 = match v6.clone() {
                Adt2::C0(_, h3493, _, _, _, _, _, _, _, _, _) => {
                    h3493
                }
            };
            nat_eq(a3494, a3495)
        } else {
            false
        } {
            if v0 {
                let a3497 = match v5.clone() {
                    Adt2::C0(_, _, h3496, _, _, _, _, _, _, _, _) => {
                        h3496
                    }
                };
                let a3498 = 2147483648u64;
                if if if nat_lt(a3497, a3498) {
                    let a3500 = match v5.clone() {
                        Adt2::C0(_, _, _, h3499, _, _, _, _, _, _, _) => {
                            h3499
                        }
                    };
                    let a3501 = 2147483648u64;
                    nat_lt(a3500, a3501)
                } else {
                    false
                } {
                    let a3504 = {
                        let a3502 = v7;
                        let a3503 = v8;
                        nat_mul(a3502, a3503)?
                    };
                    let a3505 = 2147483648u64;
                    nat_le(a3504, a3505)
                } else {
                    false
                } {
                    f214(v5, v6, v4, v7)?
                } else {
                    false
                }
            } else {
                let a3507 = match v1.clone() {
                    Adt1::C0(h3506, _, _, _, _, _, _, _, _, _, _, _) => {
                        h3506
                    }
                };
                let a3508 = v7;
                if if if if nat_eq(a3507, a3508) {
                    let a3510 = match v1 {
                        Adt1::C0(_, h3509, _, _, _, _, _, _, _, _, _, _) => {
                            h3509
                        }
                    };
                    let a3511 = v8;
                    nat_eq(a3510, a3511)
                } else {
                    false
                } {
                    f215(v7, v8, v4)?
                } else {
                    false
                } {
                    let a3513 = match v6.clone() {
                        Adt2::C0(_, _, h3512, _, _, _, _, _, _, _, _) => {
                            h3512
                        }
                    };
                    let a3514 = v4;
                    if nat_eq(a3513, a3514) {
                        let a3516 = match v6.clone() {
                            Adt2::C0(_, _, _, h3515, _, _, _, _, _, _, _) => {
                                h3515
                            }
                        };
                        let a3517 = v4;
                        nat_eq(a3516, a3517)
                    } else {
                        false
                    }
                } else {
                    false
                } {
                    let a3521 = match v6 {
                        Adt2::C0(_, _, _, _, h3518, _, _, _, _, _, _) => {
                            h3518
                        }
                    };
                    let a3522 = {
                        let a3519 = v7;
                        let a3520 = v8;
                        nat_mul(a3519, a3520)?
                    };
                    nat_eq(a3521, a3522)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            Ok(Adt10::C4(f19(v3.clone(), 1u64)?, 4u64, f23(v2, 1u64), f23(v3, 1u64), Adt4::C12(v0, v4, v7)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f118(v0: u64, v1: Adt1, v2: Adt2, v3: Adt2) -> R<Adt10> {
    let a3524 = match v2.clone() {
        Adt2::C0(h3523, _, _, _, _, _, _, _, _, _, _) => {
            h3523
        }
    };
    let a3525 = 0u64;
    if if if if if nat_eq(a3524, a3525) {
        true
    } else {
        let a3527 = match v2.clone() {
            Adt2::C0(h3526, _, _, _, _, _, _, _, _, _, _) => {
                h3526
            }
        };
        let a3528 = 1u64;
        nat_eq(a3527, a3528)
    } {
        true
    } else {
        let a3530 = match v2.clone() {
            Adt2::C0(h3529, _, _, _, _, _, _, _, _, _, _) => {
                h3529
            }
        };
        let a3531 = 30u64;
        nat_eq(a3530, a3531)
    } {
        let a3534 = match v2.clone() {
            Adt2::C0(h3532, _, _, _, _, _, _, _, _, _, _) => {
                h3532
            }
        };
        let a3535 = match v3.clone() {
            Adt2::C0(h3533, _, _, _, _, _, _, _, _, _, _) => {
                h3533
            }
        };
        nat_eq(a3534, a3535)
    } else {
        false
    } {
        if if f24(v2.clone(), v3.clone()) {
            let a3537 = match v1.clone() {
                Adt1::C0(h3536, _, _, _, _, _, _, _, _, _, _, _) => {
                    h3536
                }
            };
            let a3538 = 4294967296u64;
            nat_lt(a3537, a3538)
        } else {
            false
        } {
            let a3540 = match v1.clone() {
                Adt1::C0(_, h3539, _, _, _, _, _, _, _, _, _, _) => {
                    h3539
                }
            };
            let a3541 = 4294967296u64;
            nat_lt(a3540, a3541)
        } else {
            false
        }
    } else {
        false
    } {
        Ok(Adt10::C4(f19(v3.clone(), 1u64)?, {
            let a3543 = match v2.clone() {
                Adt2::C0(h3542, _, _, _, _, _, _, _, _, _, _) => {
                    h3542
                }
            };
            let a3544 = 0u64;
            if nat_eq(a3543, a3544) {
                4u64
            } else {
                2u64
            }
        }, f23(v2.clone(), 1u64), f23(v3, 1u64), Adt4::C13({
            let a3545 = v0;
            let a3546 = 31u64;
            if nat_eq(a3545, a3546) {
                0u64
            } else {
                let a3547 = v0;
                let a3548 = 47u64;
                if nat_eq(a3547, a3548) {
                    1u64
                } else {
                    2u64
                }
            }
        }, match v2 {
            Adt2::C0(h3549, _, _, _, _, _, _, _, _, _, _) => {
                h3549
            }
        }, match v1.clone() {
            Adt1::C0(h3550, _, _, _, _, _, _, _, _, _, _, _) => {
                h3550
            }
        }, match v1 {
            Adt1::C0(_, h3551, _, _, _, _, _, _, _, _, _, _) => {
                h3551
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f119(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3553 = match v0.clone() {
        Adt2::C0(h3552, _, _, _, _, _, _, _, _, _, _) => {
            h3552
        }
    };
    let a3554 = 0u64;
    if if if nat_eq(a3553, a3554) {
        let a3556 = match v1.clone() {
            Adt2::C0(h3555, _, _, _, _, _, _, _, _, _, _) => {
                h3555
            }
        };
        let a3557 = 0u64;
        nat_eq(a3556, a3557)
    } else {
        false
    } {
        f24(v0.clone(), v1.clone())
    } else {
        false
    } {
        Ok(Adt10::C4(f19(v1.clone(), 1u64)?, 4u64, f23(v0, 1u64), f23(v1, 1u64), Adt4::C15))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f120(v0: Adt2, v1: Adt2) -> R<Adt10> {
    let a3559 = match v0.clone() {
        Adt2::C0(h3558, _, _, _, _, _, _, _, _, _, _) => {
            h3558
        }
    };
    let a3560 = 0u64;
    if if nat_eq(a3559, a3560) {
        let a3562 = match v1.clone() {
            Adt2::C0(h3561, _, _, _, _, _, _, _, _, _, _) => {
                h3561
            }
        };
        let a3563 = 26u64;
        nat_eq(a3562, a3563)
    } else {
        false
    } {
        let a3565 = match v0.clone() {
            Adt2::C0(_, _, _, h3564, _, _, _, _, _, _, _) => {
                h3564
            }
        };
        let a3566 = 1u64;
        if if if if if if if nat_eq(a3565, a3566) {
            let a3568 = match v0.clone() {
                Adt2::C0(_, _, _, _, h3567, _, _, _, _, _, _) => {
                    h3567
                }
            };
            let a3569 = 1u64;
            nat_eq(a3568, a3569)
        } else {
            false
        } {
            let a3571 = match v0.clone() {
                Adt2::C0(_, h3570, _, _, _, _, _, _, _, _, _) => {
                    h3570
                }
            };
            let a3572 = 2147483647u64;
            nat_le(a3571, a3572)
        } else {
            false
        } {
            let a3575 = match v1.clone() {
                Adt2::C0(_, h3573, _, _, _, _, _, _, _, _, _) => {
                    h3573
                }
            };
            let a3576 = match v0.clone() {
                Adt2::C0(_, _, h3574, _, _, _, _, _, _, _, _) => {
                    h3574
                }
            };
            nat_eq(a3575, a3576)
        } else {
            false
        } {
            let a3578 = match v1.clone() {
                Adt2::C0(_, _, h3577, _, _, _, _, _, _, _, _) => {
                    h3577
                }
            };
            let a3579 = 1u64;
            nat_eq(a3578, a3579)
        } else {
            false
        } {
            let a3581 = match v1.clone() {
                Adt2::C0(_, _, _, h3580, _, _, _, _, _, _, _) => {
                    h3580
                }
            };
            let a3582 = 1u64;
            nat_eq(a3581, a3582)
        } else {
            false
        } {
            let a3584 = match v1.clone() {
                Adt2::C0(_, _, _, _, h3583, _, _, _, _, _, _) => {
                    h3583
                }
            };
            let a3585 = 1u64;
            nat_eq(a3584, a3585)
        } else {
            false
        } {
            Ok(Adt10::C4({
                let a3588 = match v0.clone() {
                    Adt2::C0(_, _, h3586, _, _, _, _, _, _, _, _) => {
                        h3586
                    }
                };
                let a3589 = f157(match v0.clone() {
                    Adt2::C0(_, h3587, _, _, _, _, _, _, _, _, _) => {
                        h3587
                    }
                })?;
                nat_mul(a3588, a3589)?
            }, 4u64, f23(v0, 1u64), f23(v1, 1u64), Adt4::C17))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f121(v0: Adt2, v1: Option<Adt2>, v2: Adt2) -> R<Adt10> {
    let m3590 = v1;
    match m3590 {
        None => {
            Ok(Adt10::C1)
        }
        Some(v3) => {
            let v4: u64 = f162(f23(v0.clone(), 1u64))?;
            let a3592 = match v0.clone() {
                Adt2::C0(h3591, _, _, _, _, _, _, _, _, _, _) => {
                    h3591
                }
            };
            let a3593 = 26u64;
            if if if nat_eq(a3592, a3593) {
                let a3595 = match v3.clone() {
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
                let a3599 = 27u64;
                nat_eq(a3598, a3599)
            } else {
                false
            } {
                if if if if if if f24(v0.clone(), v3.clone()) {
                    let a3601 = match v2.clone() {
                        Adt2::C0(_, h3600, _, _, _, _, _, _, _, _, _) => {
                            h3600
                        }
                    };
                    let a3602 = 1u64;
                    nat_eq(a3601, a3602)
                } else {
                    false
                } {
                    let a3604 = match v2.clone() {
                        Adt2::C0(_, _, h3603, _, _, _, _, _, _, _, _) => {
                            h3603
                        }
                    };
                    let a3605 = 1u64;
                    nat_eq(a3604, a3605)
                } else {
                    false
                } {
                    let a3607 = match v2.clone() {
                        Adt2::C0(_, _, _, h3606, _, _, _, _, _, _, _) => {
                            h3606
                        }
                    };
                    let a3608 = 1u64;
                    nat_eq(a3607, a3608)
                } else {
                    false
                } {
                    let a3610 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3609, _, _, _, _, _, _) => {
                            h3609
                        }
                    };
                    let a3611 = 1u64;
                    nat_eq(a3610, a3611)
                } else {
                    false
                } {
                    let a3612 = v4;
                    let a3613 = 9223372036854775808u64;
                    nat_lt(a3612, a3613)
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a3616 = 2u64;
                        let a3617 = {
                            let a3614 = v4;
                            let a3615 = 2u64;
                            nat_mul(a3614, a3615)?
                        };
                        nat_add(a3616, a3617)?
                    }, 8u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C18(v4, f23(v3, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            } else {
                Ok(Adt10::C0)
            }
        }
    }
}

pub fn f122(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let v3: u64 = match v1.clone() {
        Adt2::C0(h3618, _, _, _, _, _, _, _, _, _, _) => {
            h3618
        }
    };
    let a3619 = v3;
    let a3620 = 0u64;
    if if nat_eq(a3619, a3620) {
        true
    } else {
        let a3621 = v0;
        let a3622 = 13u64;
        if nat_eq(a3621, a3622) {
            let a3623 = v3;
            let a3624 = 1u64;
            if nat_eq(a3623, a3624) {
                true
            } else {
                let a3625 = v3;
                let a3626 = 30u64;
                nat_eq(a3625, a3626)
            }
        } else {
            false
        }
    } {
        let a3628 = v3;
        let a3629 = match v2.clone() {
            Adt2::C0(h3627, _, _, _, _, _, _, _, _, _, _) => {
                h3627
            }
        };
        if if nat_eq(a3628, a3629) {
            let a3630 = v0;
            let a3631 = 13u64;
            if nat_eq(a3630, a3631) {
                let a3633 = match v2.clone() {
                    Adt2::C0(_, h3632, _, _, _, _, _, _, _, _, _) => {
                        h3632
                    }
                };
                let a3634 = 1u64;
                if if if nat_eq(a3633, a3634) {
                    let a3636 = match v2.clone() {
                        Adt2::C0(_, _, h3635, _, _, _, _, _, _, _, _) => {
                            h3635
                        }
                    };
                    let a3637 = 1u64;
                    nat_eq(a3636, a3637)
                } else {
                    false
                } {
                    let a3639 = match v2.clone() {
                        Adt2::C0(_, _, _, h3638, _, _, _, _, _, _, _) => {
                            h3638
                        }
                    };
                    let a3640 = 1u64;
                    nat_eq(a3639, a3640)
                } else {
                    false
                } {
                    let a3642 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3641, _, _, _, _, _, _) => {
                            h3641
                        }
                    };
                    let a3643 = 1u64;
                    nat_eq(a3642, a3643)
                } else {
                    false
                }
            } else {
                let a3645 = match v2.clone() {
                    Adt2::C0(_, h3644, _, _, _, _, _, _, _, _, _) => {
                        h3644
                    }
                };
                let a3646 = 1u64;
                if if if nat_eq(a3645, a3646) {
                    let a3649 = match v1.clone() {
                        Adt2::C0(_, _, h3647, _, _, _, _, _, _, _, _) => {
                            h3647
                        }
                    };
                    let a3650 = match v2.clone() {
                        Adt2::C0(_, _, h3648, _, _, _, _, _, _, _, _) => {
                            h3648
                        }
                    };
                    nat_eq(a3649, a3650)
                } else {
                    false
                } {
                    let a3653 = match v1.clone() {
                        Adt2::C0(_, _, _, h3651, _, _, _, _, _, _, _) => {
                            h3651
                        }
                    };
                    let a3654 = match v2.clone() {
                        Adt2::C0(_, _, _, h3652, _, _, _, _, _, _, _) => {
                            h3652
                        }
                    };
                    nat_eq(a3653, a3654)
                } else {
                    false
                } {
                    let a3657 = match v1.clone() {
                        Adt2::C0(_, _, _, _, h3655, _, _, _, _, _, _) => {
                            h3655
                        }
                    };
                    let a3658 = match v2.clone() {
                        Adt2::C0(_, _, _, _, h3656, _, _, _, _, _, _) => {
                            h3656
                        }
                    };
                    nat_eq(a3657, a3658)
                } else {
                    false
                }
            }
        } else {
            false
        } {
            let v4: Adt3 = f23(v1, 1u64);
            Ok(Adt10::C4(f216(v4.clone(), v0, v3)?, {
                let a3659 = v3;
                let a3660 = 0u64;
                if nat_eq(a3659, a3660) {
                    4u64
                } else {
                    2u64
                }
            }, v4, f23(v2, 1u64), Adt4::C16(v0, v3)))
        } else {
            Ok(Adt10::C1)
        }
    } else {
        Ok(Adt10::C0)
    }
}

pub fn f123(v0: u64, v1: u64) -> R<Option<u64>> {
    let a3661 = v0;
    let a3662 = 89u64;
    if if nat_eq(a3661, a3662) {
        let a3663 = v1;
        let a3664 = 14u64;
        nat_eq(a3663, a3664)
    } else {
        false
    } {
        Ok(Some(23u64))
    } else {
        let a3665 = v0;
        let a3666 = 89u64;
        if if nat_eq(a3665, a3666) {
            let a3667 = v1;
            let a3668 = 15u64;
            nat_eq(a3667, a3668)
        } else {
            false
        } {
            Ok(Some(22u64))
        } else {
            let a3669 = v0;
            let a3670 = 89u64;
            if if nat_eq(a3669, a3670) {
                let a3671 = 8u64;
                let a3672 = v1;
                if nat_le(a3671, a3672) {
                    let a3673 = v1;
                    let a3674 = 9u64;
                    nat_le(a3673, a3674)
                } else {
                    false
                }
            } else {
                false
            } {
                Ok(Some({
                    let a3675 = v1;
                    let a3676 = 12u64;
                    nat_add(a3675, a3676)?
                }))
            } else {
                let a3677 = v0;
                let a3678 = 89u64;
                if if nat_eq(a3677, a3678) {
                    let a3679 = v1;
                    let a3680 = 4u64;
                    nat_eq(a3679, a3680)
                } else {
                    false
                } {
                    Ok(Some(19u64))
                } else {
                    let a3681 = 10u64;
                    let a3682 = v0;
                    if if nat_le(a3681, a3682) {
                        let a3683 = v0;
                        let a3684 = 12u64;
                        nat_le(a3683, a3684)
                    } else {
                        false
                    } {
                        Ok(Some({
                            let a3685 = v0;
                            let a3686 = 3u64;
                            nat_add(a3685, a3686)?
                        }))
                    } else {
                        let a3687 = v0;
                        let a3688 = 89u64;
                        if if nat_eq(a3687, a3688) {
                            let a3689 = v1;
                            let a3690 = 7u64;
                            nat_eq(a3689, a3690)
                        } else {
                            false
                        } {
                            Ok(Some(17u64))
                        } else {
                            let a3691 = v0;
                            let a3692 = 89u64;
                            if if nat_eq(a3691, a3692) {
                                let a3693 = v1;
                                let a3694 = 10u64;
                                nat_eq(a3693, a3694)
                            } else {
                                false
                            } {
                                Ok(Some(18u64))
                            } else {
                                let a3695 = v0;
                                let a3696 = 89u64;
                                if if nat_eq(a3695, a3696) {
                                    let a3697 = v1;
                                    let a3698 = 13u64;
                                    nat_eq(a3697, a3698)
                                } else {
                                    false
                                } {
                                    Ok(Some(16u64))
                                } else {
                                    let a3699 = v0;
                                    let a3700 = 8u64;
                                    if nat_eq(a3699, a3700) {
                                        Ok(Some(5u64))
                                    } else {
                                        let a3701 = v0;
                                        let a3702 = 9u64;
                                        if nat_eq(a3701, a3702) {
                                            Ok(Some(6u64))
                                        } else {
                                            let a3703 = v0;
                                            let a3704 = 89u64;
                                            if nat_eq(a3703, a3704) {
                                                let a3705 = v1;
                                                let a3706 = 0u64;
                                                if nat_eq(a3705, a3706) {
                                                    Ok(Some(0u64))
                                                } else {
                                                    let a3707 = v1;
                                                    let a3708 = 1u64;
                                                    if nat_eq(a3707, a3708) {
                                                        Ok(Some(1u64))
                                                    } else {
                                                        let a3709 = v1;
                                                        let a3710 = 2u64;
                                                        if nat_eq(a3709, a3710) {
                                                            Ok(Some(2u64))
                                                        } else {
                                                            let a3711 = v1;
                                                            let a3712 = 3u64;
                                                            if nat_eq(a3711, a3712) {
                                                                Ok(Some(3u64))
                                                            } else {
                                                                let a3713 = v1;
                                                                let a3714 = 6u64;
                                                                if nat_eq(a3713, a3714) {
                                                                    Ok(Some(4u64))
                                                                } else {
                                                                    let a3715 = v1;
                                                                    let a3716 = 12u64;
                                                                    if nat_eq(a3715, a3716) {
                                                                        Ok(Some(7u64))
                                                                    } else {
                                                                        let a3717 = v1;
                                                                        let a3718 = 11u64;
                                                                        if nat_eq(a3717, a3718) {
                                                                            Ok(Some(8u64))
                                                                        } else {
                                                                            let a3719 = v1;
                                                                            let a3720 = 18u64;
                                                                            if nat_eq(a3719, a3720) {
                                                                                Ok(Some(9u64))
                                                                            } else {
                                                                                let a3721 = v1;
                                                                                let a3722 = 19u64;
                                                                                if nat_eq(a3721, a3722) {
                                                                                    Ok(Some(10u64))
                                                                                } else {
                                                                                    let a3723 = v1;
                                                                                    let a3724 = 20u64;
                                                                                    if nat_eq(a3723, a3724) {
                                                                                        Ok(Some(11u64))
                                                                                    } else {
                                                                                        let a3725 = v1;
                                                                                        let a3726 = 21u64;
                                                                                        if nat_eq(a3725, a3726) {
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

pub fn f124(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let m3727 = f217(v0, v1, v2.clone(), v3.clone(), v4.clone())?;
    match m3727 {
        Adt10::C0 => {
            Ok(Adt10::C0)
        }
        Adt10::C1 => {
            let a3728 = v0;
            let a3729 = 2u64;
            if if if if nat_eq(a3728, a3729) {
                true
            } else {
                let a3730 = v0;
                let a3731 = 5u64;
                nat_eq(a3730, a3731)
            } {
                true
            } else {
                let a3732 = v0;
                let a3733 = 6u64;
                nat_eq(a3732, a3733)
            } {
                true
            } else {
                let a3734 = v0;
                let a3735 = 7u64;
                nat_eq(a3734, a3735)
            } {
                Ok(Adt10::C5(67u64))
            } else {
                Ok(Adt10::C1)
            }
        }
        Adt10::C2(v7, v8, _, _, _, _, v13) => {
            if v8 {
                Ok(Adt10::C4(v13, 4u64, f23(v2.clone(), 1u64), f23(v4, 1u64), Adt4::C10(v7, v8, {
                    let m3736 = v3;
                    match m3736 {
                        None => {
                            f23(v2, 1u64)
                        }
                        Some(v14) => {
                            f23(v14, 1u64)
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

pub fn f125(v0: u64, v1: Adt2, v2: Adt2) -> R<Adt10> {
    let a3738 = match v1.clone() {
        Adt2::C0(h3737, _, _, _, _, _, _, _, _, _, _) => {
            h3737
        }
    };
    let a3739 = 0u64;
    if if if if if nat_eq(a3738, a3739) {
        true
    } else {
        let a3741 = match v1.clone() {
            Adt2::C0(h3740, _, _, _, _, _, _, _, _, _, _) => {
                h3740
            }
        };
        let a3742 = 1u64;
        nat_eq(a3741, a3742)
    } {
        true
    } else {
        let a3744 = match v1.clone() {
            Adt2::C0(h3743, _, _, _, _, _, _, _, _, _, _) => {
                h3743
            }
        };
        let a3745 = 30u64;
        nat_eq(a3744, a3745)
    } {
        let a3748 = match v1.clone() {
            Adt2::C0(h3746, _, _, _, _, _, _, _, _, _, _) => {
                h3746
            }
        };
        let a3749 = match v2.clone() {
            Adt2::C0(h3747, _, _, _, _, _, _, _, _, _, _) => {
                h3747
            }
        };
        if nat_eq(a3748, a3749) {
            true
        } else {
            let a3751 = match v2.clone() {
                Adt2::C0(h3750, _, _, _, _, _, _, _, _, _, _) => {
                    h3750
                }
            };
            let a3752 = 0u64;
            nat_eq(a3751, a3752)
        }
    } else {
        false
    } {
        f24(v1.clone(), v2.clone())
    } else {
        false
    } {
        Ok(Adt10::C4({
            let a3754 = f19(v2.clone(), 1u64)?;
            let a3755 = f218(v0, match v2.clone() {
                Adt2::C0(h3753, _, _, _, _, _, _, _, _, _, _) => {
                    h3753
                }
            });
            nat_mul(a3754, a3755)?
        }, {
            let a3757 = match v2.clone() {
                Adt2::C0(h3756, _, _, _, _, _, _, _, _, _, _) => {
                    h3756
                }
            };
            let a3758 = 0u64;
            if nat_eq(a3757, a3758) {
                4u64
            } else {
                2u64
            }
        }, f23(v1.clone(), 1u64), f23(v2.clone(), 1u64), Adt4::C14(v0, match v1 {
            Adt2::C0(h3759, _, _, _, _, _, _, _, _, _, _) => {
                h3759
            }
        }, match v2 {
            Adt2::C0(h3760, _, _, _, _, _, _, _, _, _, _) => {
                h3760
            }
        })))
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f126(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> R<u64> {
    let a3773 = {
        let a3769 = {
            let a3765 = {
                let a3761 = v0;
                let a3762 = v4;
                nat_mul(a3761, a3762)?
            };
            let a3766 = {
                let a3763 = v1;
                let a3764 = v5;
                nat_mul(a3763, a3764)?
            };
            nat_add(a3765, a3766)?
        };
        let a3770 = {
            let a3767 = v2;
            let a3768 = v6;
            nat_mul(a3767, a3768)?
        };
        nat_add(a3769, a3770)?
    };
    let a3774 = {
        let a3771 = v3;
        let a3772 = v7;
        nat_mul(a3771, a3772)?
    };
    nat_add(a3773, a3774)
}

pub fn f127(v0: u64) -> u64 {
    let a3775 = v0;
    let a3776 = 2147483648u64;
    if nat_lt(a3775, a3776) {
        0u64
    } else {
        let a3777 = 4294967296u64;
        let a3778 = v0;
        nat_sub(a3777, a3778)
    }
}

pub fn f128(v0: u64) -> u64 {
    let a3779 = v0;
    let a3780 = 2147483648u64;
    if nat_lt(a3779, a3780) {
        v0
    } else {
        0u64
    }
}

pub fn f129(v0: Adt11, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64) -> Adt9 {
    let v9: u64 = {
        let a3781 = v7;
        let a3782 = 6u64;
        if nat_eq(a3781, a3782) {
            27u64
        } else {
            v2
        }
    };
    Adt9::C7(v0, f77(v3, v4), f77(v5, v6), f78(v7, v8), f156(v1), f156(v9))
}

pub fn f130(v0: Adt4, v1: u64, v2: u64) -> Adt11 {
    let m3783 = v0;
    match m3783 {
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
        Adt4::C13(v49, v50, v51, v52) => {
            Adt11::C4(v49, v50, v51, v52)
        }
        Adt4::C14(v53, _, _) => {
            let a3784 = 13u64;
            let a3785 = v53;
            if nat_le(a3784, a3785) {
                Adt11::C7(v53, v1, v2)
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
        Adt4::C27(v73) => {
            Adt11::C2(match v73.clone() {
                Adt5::C0(_, _, _, _, _, h3786, _, _) => {
                    h3786
                }
            }, match v73 {
                Adt5::C0(_, _, _, _, _, _, _, h3787) => {
                    h3787
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
    }
}

pub fn f131(v0: Adt4) -> u64 {
    let m3788 = v0;
    match m3788 {
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
        Adt4::C14(v51, _, _) => {
            v51
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
    }
}

pub fn f132(v0: Adt4, v1: u64) -> u64 {
    let m3789 = v0;
    match m3789 {
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
        Adt4::C16(_, v56) => {
            v56
        }
        Adt4::C17 => {
            0u64
        }
        Adt4::C18(_, _) => {
            0u64
        }
        Adt4::C19(v59, _) => {
            let a3790 = v1;
            let a3791 = 6u64;
            if nat_eq(a3790, a3791) {
                v59
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
        Adt4::C26(v73, _, _, _) => {
            let a3800 = v1;
            let a3801 = 6u64;
            if if if nat_eq(a3800, a3801) {
                true
            } else {
                let a3802 = v1;
                let a3803 = 7u64;
                nat_eq(a3802, a3803)
            } {
                let a3804 = v73;
                let a3805 = 30u64;
                nat_eq(a3804, a3805)
            } else {
                false
            } {
                30u64
            } else {
                0u64
            }
        }
        Adt4::C27(v72) => {
            match v72 {
                Adt5::C0(_, _, _, _, _, _, _, h3799) => {
                    h3799
                }
            }
        }
        Adt4::C28(v68) => {
            let a3792 = v1;
            let a3793 = 6u64;
            if nat_eq(a3792, a3793) {
                match v68 {
                    Adt7::C0(_, _, _, _, h3794) => {
                        h3794
                    }
                }
            } else {
                0u64
            }
        }
        Adt4::C29(_) => {
            0u64
        }
        Adt4::C30(_, v71) => {
            let a3795 = v1;
            let a3796 = 6u64;
            if if nat_eq(a3795, a3796) {
                true
            } else {
                let a3797 = v1;
                let a3798 = 7u64;
                nat_eq(a3797, a3798)
            } {
                v71
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
    }
}

pub fn f133(v0: Adt4, v1: u64) -> Adt11 {
    let a3806 = v1;
    let a3807 = 4u64;
    if nat_lt(a3806, a3807) {
        Adt11::C0(v1)
    } else {
        let a3808 = v1;
        let a3809 = 4u64;
        if nat_eq(a3808, a3809) {
            Adt11::C1(0u64)
        } else {
            let a3810 = v1;
            let a3811 = 5u64;
            if nat_eq(a3810, a3811) {
                Adt11::C1(3u64)
            } else {
                let a3812 = v1;
                let a3813 = 6u64;
                if nat_eq(a3812, a3813) {
                    Adt11::C2(f132(v0, v1), 0u64, false)
                } else {
                    let a3814 = v1;
                    let a3815 = 7u64;
                    if nat_eq(a3814, a3815) {
                        Adt11::C2(0u64, f132(v0, v1), false)
                    } else {
                        let a3816 = v1;
                        let a3817 = 16u64;
                        if nat_lt(a3816, a3817) {
                            Adt11::C1({
                                let a3818 = v1;
                                let a3819 = 8u64;
                                nat_sub(a3818, a3819)
                            })
                        } else {
                            let a3820 = v1;
                            let a3821 = 20u64;
                            if nat_lt(a3820, a3821) {
                                Adt11::C5({
                                    let a3822 = v1;
                                    let a3823 = 16u64;
                                    nat_sub(a3822, a3823)
                                })
                            } else {
                                let a3824 = v1;
                                let a3825 = 20u64;
                                if nat_eq(a3824, a3825) {
                                    Adt11::C3(0u64, 0u64, false)
                                } else {
                                    let a3826 = v1;
                                    let a3827 = 21u64;
                                    if nat_eq(a3826, a3827) {
                                        Adt11::C0(4u64)
                                    } else {
                                        let a3828 = v1;
                                        let a3829 = 22u64;
                                        if nat_eq(a3828, a3829) {
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

pub fn f134(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a3847 = {
        let a3842 = f219(v2, {
            let a3833 = {
                let a3830 = v0;
                let a3831 = v1;
                let a3832 = 0u64;
                nat_quot(a3830, a3831, a3832)
            };
            let a3834 = v2;
            let a3835 = 0u64;
            nat_rem(a3833, a3834, a3835)
        }, {
            let a3839 = {
                let a3836 = v0;
                let a3837 = v1;
                let a3838 = 0u64;
                nat_quot(a3836, a3837, a3838)
            };
            let a3840 = v2;
            let a3841 = 0u64;
            nat_quot(a3839, a3840, a3841)
        })?;
        let a3843 = v1;
        nat_mul(a3842, a3843)?
    };
    let a3848 = {
        let a3844 = v0;
        let a3845 = v1;
        let a3846 = 0u64;
        nat_rem(a3844, a3845, a3846)
    };
    nat_add(a3847, a3848)
}

pub fn f135(v0: u64, v1: Adt3, v2: Adt3) -> R<u64> {
    let v3: u64 = {
        let a3850 = v0;
        let a3851 = match v2.clone() {
            Adt3::C0(h3849, _, _, _, _, _, _, _) => {
                h3849
            }
        };
        let a3852 = 0u64;
        nat_quot(a3850, a3851, a3852)
    };
    let v4: u64 = {
        let a3854 = v3;
        let a3855 = match v2.clone() {
            Adt3::C0(_, h3853, _, _, _, _, _, _) => {
                h3853
            }
        };
        let a3856 = 0u64;
        nat_quot(a3854, a3855, a3856)
    };
    f220({
        let a3858 = v0;
        let a3859 = match v2.clone() {
            Adt3::C0(h3857, _, _, _, _, _, _, _) => {
                h3857
            }
        };
        let a3860 = 0u64;
        nat_rem(a3858, a3859, a3860)
    }, {
        let a3862 = v3;
        let a3863 = match v2.clone() {
            Adt3::C0(_, h3861, _, _, _, _, _, _) => {
                h3861
            }
        };
        let a3864 = 0u64;
        nat_rem(a3862, a3863, a3864)
    }, {
        let a3866 = v4;
        let a3867 = match v2.clone() {
            Adt3::C0(_, _, h3865, _, _, _, _, _) => {
                h3865
            }
        };
        let a3868 = 0u64;
        nat_rem(a3866, a3867, a3868)
    }, {
        let a3870 = v4;
        let a3871 = match v2 {
            Adt3::C0(_, _, h3869, _, _, _, _, _) => {
                h3869
            }
        };
        let a3872 = 0u64;
        nat_quot(a3870, a3871, a3872)
    }, match v1.clone() {
        Adt3::C0(h3873, _, _, _, _, _, _, _) => {
            h3873
        }
    }, match v1.clone() {
        Adt3::C0(_, h3874, _, _, _, _, _, _) => {
            h3874
        }
    }, match v1.clone() {
        Adt3::C0(_, _, h3875, _, _, _, _, _) => {
            h3875
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, h3876, _, _, _, _) => {
            h3876
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, h3877, _, _, _) => {
            h3877
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, h3878, _, _) => {
            h3878
        }
    }, match v1.clone() {
        Adt3::C0(_, _, _, _, _, _, h3879, _) => {
            h3879
        }
    }, match v1 {
        Adt3::C0(_, _, _, _, _, _, _, h3880) => {
            h3880
        }
    })
}

pub fn f136(v0: u64, v1: Adt3, v2: Adt3, v3: Adt3, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a3882 = v0;
        let a3883 = match v3.clone() {
            Adt3::C0(h3881, _, _, _, _, _, _, _) => {
                h3881
            }
        };
        let a3884 = 0u64;
        nat_quot(a3882, a3883, a3884)
    };
    let v6: u64 = {
        let a3886 = v5;
        let a3887 = match v3.clone() {
            Adt3::C0(_, h3885, _, _, _, _, _, _) => {
                h3885
            }
        };
        let a3888 = 0u64;
        nat_quot(a3886, a3887, a3888)
    };
    let v7: u64 = {
        let a3890 = v0;
        let a3891 = match v3.clone() {
            Adt3::C0(h3889, _, _, _, _, _, _, _) => {
                h3889
            }
        };
        let a3892 = 0u64;
        nat_rem(a3890, a3891, a3892)
    };
    let v8: u64 = {
        let a3894 = v5;
        let a3895 = match v3.clone() {
            Adt3::C0(_, h3893, _, _, _, _, _, _) => {
                h3893
            }
        };
        let a3896 = 0u64;
        nat_rem(a3894, a3895, a3896)
    };
    let v9: u64 = {
        let a3898 = v6;
        let a3899 = match v3.clone() {
            Adt3::C0(_, _, h3897, _, _, _, _, _) => {
                h3897
            }
        };
        let a3900 = 0u64;
        nat_rem(a3898, a3899, a3900)
    };
    let v10: u64 = {
        let a3902 = v6;
        let a3903 = match v3.clone() {
            Adt3::C0(_, _, h3901, _, _, _, _, _) => {
                h3901
            }
        };
        let a3904 = 0u64;
        nat_quot(a3902, a3903, a3904)
    };
    let v11: u64 = {
        let a3905 = v4;
        let a3906 = 0u64;
        if nat_eq(a3905, a3906) {
            match v1.clone() {
                Adt3::C0(h3907, _, _, _, _, _, _, _) => {
                    h3907
                }
            }
        } else {
            let a3908 = v4;
            let a3909 = 1u64;
            if nat_eq(a3908, a3909) {
                match v1.clone() {
                    Adt3::C0(_, h3910, _, _, _, _, _, _) => {
                        h3910
                    }
                }
            } else {
                let a3911 = v4;
                let a3912 = 2u64;
                if nat_eq(a3911, a3912) {
                    match v1.clone() {
                        Adt3::C0(_, _, h3913, _, _, _, _, _) => {
                            h3913
                        }
                    }
                } else {
                    match v1.clone() {
                        Adt3::C0(_, _, _, h3914, _, _, _, _) => {
                            h3914
                        }
                    }
                }
            }
        }
    };
    let v12: bool = {
        let a3921 = v11;
        let a3922 = {
            let a3915 = v4;
            let a3916 = 0u64;
            if nat_eq(a3915, a3916) {
                v7
            } else {
                let a3917 = v4;
                let a3918 = 1u64;
                if nat_eq(a3917, a3918) {
                    v8
                } else {
                    let a3919 = v4;
                    let a3920 = 2u64;
                    if nat_eq(a3919, a3920) {
                        v9
                    } else {
                        v10
                    }
                }
            }
        };
        nat_le(a3921, a3922)
    };
    let v13: Adt3 = if v12 {
        v2
    } else {
        v1
    };
    Ok(Adt9::C1(v12, f126(if if v12 {
        let a3923 = v4;
        let a3924 = 0u64;
        nat_eq(a3923, a3924)
    } else {
        false
    } {
        let a3925 = v7;
        let a3926 = v11;
        nat_sub(a3925, a3926)
    } else {
        v7
    }, if if v12 {
        let a3927 = v4;
        let a3928 = 1u64;
        nat_eq(a3927, a3928)
    } else {
        false
    } {
        let a3929 = v8;
        let a3930 = v11;
        nat_sub(a3929, a3930)
    } else {
        v8
    }, if if v12 {
        let a3931 = v4;
        let a3932 = 2u64;
        nat_eq(a3931, a3932)
    } else {
        false
    } {
        let a3933 = v9;
        let a3934 = v11;
        nat_sub(a3933, a3934)
    } else {
        v9
    }, if if v12 {
        let a3935 = v4;
        let a3936 = 3u64;
        nat_eq(a3935, a3936)
    } else {
        false
    } {
        let a3937 = v10;
        let a3938 = v11;
        nat_sub(a3937, a3938)
    } else {
        v10
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, h3939, _, _, _) => {
            h3939
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, h3940, _, _) => {
            h3940
        }
    }, match v13.clone() {
        Adt3::C0(_, _, _, _, _, _, h3941, _) => {
            h3941
        }
    }, match v13 {
        Adt3::C0(_, _, _, _, _, _, _, h3942) => {
            h3942
        }
    })?, f79(v0, v3)?))
}

pub fn f137(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: u64, v6: u64, v7: bool) -> R<Adt9> {
    let v8: u64 = f50(v0, v2.clone(), 0u64);
    let v9: u64 = f50(v0, v2.clone(), 1u64);
    let v10: u64 = f50(v0, v2.clone(), 2u64);
    let v11: u64 = f50(v0, v2.clone(), 3u64);
    if v7 {
        Ok(Adt9::C1(false, f49(v1.clone(), f221(v8, v3, match v1.clone() {
            Adt3::C0(h3943, _, _, _, _, _, _, _) => {
                h3943
            }
        })?, f221(v9, v4, match v1.clone() {
            Adt3::C0(_, h3944, _, _, _, _, _, _) => {
                h3944
            }
        })?, f221(v10, v5, match v1.clone() {
            Adt3::C0(_, _, h3945, _, _, _, _, _) => {
                h3945
            }
        })?, f221(v11, v6, match v1 {
            Adt3::C0(_, _, _, h3946, _, _, _, _) => {
                h3946
            }
        })?)?, f79(v0, v2)?))
    } else {
        if if if if f222(v8, v3, match v1.clone() {
            Adt3::C0(h3947, _, _, _, _, _, _, _) => {
                h3947
            }
        })? {
            f222(v9, v4, match v1.clone() {
                Adt3::C0(_, h3948, _, _, _, _, _, _) => {
                    h3948
                }
            })?
        } else {
            false
        } {
            f222(v10, v5, match v1.clone() {
                Adt3::C0(_, _, h3949, _, _, _, _, _) => {
                    h3949
                }
            })?
        } else {
            false
        } {
            f222(v11, v6, match v1.clone() {
                Adt3::C0(_, _, _, h3950, _, _, _, _) => {
                    h3950
                }
            })?
        } else {
            false
        } {
            Ok(Adt9::C1(false, f49(v1, f223(v8, v3)?, f223(v9, v4)?, f223(v10, v5)?, f223(v11, v6)?)?, f79(v0, v2)?))
        } else {
            Ok(Adt9::C1(true, 0u64, f79(v0, v2)?))
        }
    }
}

pub fn f138(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = f50(v0, v2.clone(), 0u64);
    let v5: u64 = f50(v0, v2.clone(), 1u64);
    let v6: u64 = f50(v0, v2.clone(), 2u64);
    let v7: u64 = f50(v0, v2.clone(), 3u64);
    Ok(Adt9::C1(false, f49(v1.clone(), f224(v4, v3, match v1 {
        Adt3::C0(h3951, _, _, _, _, _, _, _) => {
            h3951
        }
    })?, v5, v6, v7)?, f79(v0, v2)?))
}

pub fn f139(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64, v5: bool) -> R<Adt9> {
    let a3952 = v0;
    let a3953 = v3;
    if nat_lt(a3952, a3953) {
        Ok(Adt9::C1(false, {
            let a3954 = v0;
            let a3955 = 4u64;
            nat_mul(a3954, a3955)?
        }, {
            let a3956 = v0;
            let a3957 = 4u64;
            nat_mul(a3956, a3957)?
        }))
    } else {
        let v6: u64 = {
            let a3958 = v0;
            let a3959 = v3;
            nat_sub(a3958, a3959)
        };
        let v7: u64 = f79(v6, v1)?;
        let v8: u64 = f225(v6, v2, v4)?;
        if v5 {
            Ok(Adt9::C4(8u64, 0u64, v8, 4u64, v7, 3u64, v8))
        } else {
            Ok(Adt9::C1(true, v7, v8))
        }
    }
}

pub fn f140(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a3960 = v0;
    let a3961 = v3;
    if nat_lt(a3960, a3961) {
        Ok(Adt9::C1(true, 0u64, f79(v0, v2)?))
    } else {
        let v4: u64 = {
            let a3962 = v0;
            let a3963 = v3;
            nat_sub(a3962, a3963)
        };
        let v5: u64 = f135(v4, v2, v1.clone())?;
        Ok(Adt9::C4(8u64, 3u64, v5, 0u64, f79(v4, v1)?, 3u64, v5))
    }
}

pub fn f141(v0: u64, v1: u64, v2: Adt3, v3: Adt3) -> R<Adt9> {
    let v4: u64 = {
        let a3965 = v1;
        let a3966 = match v3.clone() {
            Adt3::C0(h3964, _, _, _, _, _, _, _) => {
                h3964
            }
        };
        let a3967 = 0u64;
        nat_rem(a3965, a3966, a3967)
    };
    let v5: u64 = {
        let a3973 = {
            let a3969 = v1;
            let a3970 = match v3.clone() {
                Adt3::C0(h3968, _, _, _, _, _, _, _) => {
                    h3968
                }
            };
            let a3971 = 0u64;
            nat_quot(a3969, a3970, a3971)
        };
        let a3974 = match v3.clone() {
            Adt3::C0(_, h3972, _, _, _, _, _, _) => {
                h3972
            }
        };
        let a3975 = 0u64;
        nat_rem(a3973, a3974, a3975)
    };
    let v6: bool = f226(v0, v4, v5);
    Ok(Adt9::C1(!v6, if v6 {
        f135(v1, v2, v3.clone())?
    } else {
        0u64
    }, f79(v1, v3)?))
}

pub fn f142(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: bool, v5: Adt3) -> R<Adt9> {
    Ok(Adt9::C4({
        let a3976 = v3;
        let a3977 = 8u64;
        nat_add(a3976, a3977)?
    }, 0u64, f79(v0, v1)?, if v4 {
        5u64
    } else {
        4u64
    }, if v4 {
        0u64
    } else {
        f135(v0, v5, v2.clone())?
    }, 3u64, f79(v0, v2)?))
}

pub fn f143(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a3978 = v4;
    let a3979 = 0u64;
    if if nat_eq(a3978, a3979) {
        f107(v3)
    } else {
        false
    } {
        let m3980 = f197(v3);
        match m3980 {
            Adt18::C0 => {
                Ok(Adt9::C0(Adt10::C1))
            }
            Adt18::C1(v5) => {
                let v6: u64 = match v5.clone() {
                    Adt19::C0(_, h3981, _) => {
                        h3981
                    }
                };
                Ok(Adt9::C5(v3, f79(v0, v1)?, match v5 {
                    Adt19::C0(_, _, h3982) => {
                        h3982
                    }
                }, f79({
                    let a3983 = v0;
                    let a3984 = v6;
                    nat_mul(a3983, a3984)?
                }, v2.clone())?, {
                    let a3990 = {
                        let a3988 = {
                            let a3985 = v6;
                            let a3986 = 1u64;
                            nat_sub(a3985, a3986)
                        };
                        let a3989 = match v2.clone() {
                            Adt3::C0(_, _, _, _, h3987, _, _, _) => {
                                h3987
                            }
                        };
                        nat_mul(a3988, a3989)?
                    };
                    let a3991 = 4u64;
                    nat_add(a3990, a3991)?
                }, match v2 {
                    Adt3::C0(_, _, _, _, h3992, _, _, _) => {
                        h3992
                    }
                }))
            }
        }
    } else {
        Ok(Adt9::C3(v3, v4, f79(v0, v1)?, f79(v0, v2)?))
    }
}

pub fn f144(v0: u64, v1: Adt3, v2: Adt3, v3: bool, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = {
        let a3994 = v0;
        let a3995 = match v2.clone() {
            Adt3::C0(h3993, _, _, _, _, _, _, _) => {
                h3993
            }
        };
        let a3996 = 0u64;
        nat_rem(a3994, a3995, a3996)
    };
    let v7: u64 = {
        let a3998 = v0;
        let a3999 = match v2.clone() {
            Adt3::C0(h3997, _, _, _, _, _, _, _) => {
                h3997
            }
        };
        let a4000 = 0u64;
        nat_quot(a3998, a3999, a4000)
    };
    let v8: u64 = {
        let a4002 = v7;
        let a4003 = match v2.clone() {
            Adt3::C0(_, h4001, _, _, _, _, _, _) => {
                h4001
            }
        };
        let a4004 = 0u64;
        nat_rem(a4002, a4003, a4004)
    };
    let v9: u64 = {
        let a4006 = v7;
        let a4007 = match v2.clone() {
            Adt3::C0(_, h4005, _, _, _, _, _, _) => {
                h4005
            }
        };
        let a4008 = 0u64;
        nat_quot(a4006, a4007, a4008)
    };
    let v10: u64 = {
        let a4010 = v9;
        let a4011 = match v2.clone() {
            Adt3::C0(_, _, h4009, _, _, _, _, _) => {
                h4009
            }
        };
        let a4012 = 0u64;
        nat_rem(a4010, a4011, a4012)
    };
    let v11: u64 = {
        let a4014 = v9;
        let a4015 = match v2.clone() {
            Adt3::C0(_, _, h4013, _, _, _, _, _) => {
                h4013
            }
        };
        let a4016 = 0u64;
        nat_quot(a4014, a4015, a4016)
    };
    let v12: u64 = f79(v0, v2)?;
    if v3 {
        let v13: u64 = {
            let a4025 = {
                let a4020 = {
                    let a4017 = v10;
                    let a4018 = v4;
                    let a4019 = 0u64;
                    nat_quot(a4017, a4018, a4019)
                };
                let a4021 = v5;
                nat_mul(a4020, a4021)?
            };
            let a4026 = {
                let a4022 = v8;
                let a4023 = v4;
                let a4024 = 0u64;
                nat_quot(a4022, a4023, a4024)
            };
            nat_add(a4025, a4026)?
        };
        let v14: u64 = {
            let a4046 = v6;
            let a4047 = {
                let a4044 = match v1.clone() {
                    Adt3::C0(h4027, _, _, _, _, _, _, _) => {
                        h4027
                    }
                };
                let a4045 = {
                    let a4042 = {
                        let a4028 = v8;
                        let a4029 = v4;
                        let a4030 = 0u64;
                        nat_rem(a4028, a4029, a4030)
                    };
                    let a4043 = {
                        let a4040 = match v1.clone() {
                            Adt3::C0(_, h4031, _, _, _, _, _, _) => {
                                h4031
                            }
                        };
                        let a4041 = {
                            let a4038 = {
                                let a4032 = v10;
                                let a4033 = v4;
                                let a4034 = 0u64;
                                nat_rem(a4032, a4033, a4034)
                            };
                            let a4039 = {
                                let a4036 = match v1.clone() {
                                    Adt3::C0(_, _, h4035, _, _, _, _, _) => {
                                        h4035
                                    }
                                };
                                let a4037 = v13;
                                nat_mul(a4036, a4037)?
                            };
                            nat_add(a4038, a4039)?
                        };
                        nat_mul(a4040, a4041)?
                    };
                    nat_add(a4042, a4043)?
                };
                nat_mul(a4044, a4045)?
            };
            nat_add(a4046, a4047)?
        };
        Ok(Adt9::C1(false, f79(v14, v1)?, v12))
    } else {
        let v15: u64 = {
            let a4053 = {
                let a4051 = {
                    let a4048 = v11;
                    let a4049 = v5;
                    let a4050 = 0u64;
                    nat_rem(a4048, a4049, a4050)
                };
                let a4052 = v4;
                nat_mul(a4051, a4052)?
            };
            let a4054 = v8;
            nat_add(a4053, a4054)?
        };
        let v16: u64 = {
            let a4060 = {
                let a4058 = {
                    let a4055 = v11;
                    let a4056 = v5;
                    let a4057 = 0u64;
                    nat_quot(a4055, a4056, a4057)
                };
                let a4059 = v4;
                nat_mul(a4058, a4059)?
            };
            let a4061 = v10;
            nat_add(a4060, a4061)?
        };
        let a4063 = v15;
        let a4064 = match v1.clone() {
            Adt3::C0(_, h4062, _, _, _, _, _, _) => {
                h4062
            }
        };
        if if nat_lt(a4063, a4064) {
            let a4066 = v16;
            let a4067 = match v1.clone() {
                Adt3::C0(_, _, h4065, _, _, _, _, _) => {
                    h4065
                }
            };
            nat_lt(a4066, a4067)
        } else {
            false
        } {
            Ok(Adt9::C1(false, f79({
                let a4076 = v6;
                let a4077 = {
                    let a4074 = match v1.clone() {
                        Adt3::C0(h4068, _, _, _, _, _, _, _) => {
                            h4068
                        }
                    };
                    let a4075 = {
                        let a4072 = v15;
                        let a4073 = {
                            let a4070 = match v1.clone() {
                                Adt3::C0(_, h4069, _, _, _, _, _, _) => {
                                    h4069
                                }
                            };
                            let a4071 = v16;
                            nat_mul(a4070, a4071)?
                        };
                        nat_add(a4072, a4073)?
                    };
                    nat_mul(a4074, a4075)?
                };
                nat_add(a4076, a4077)?
            }, v1)?, v12))
        } else {
            Ok(Adt9::C1(true, 0u64, v12))
        }
    }
}

pub fn f145(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = f218(23u64, v4);
    let v6: u64 = {
        let a4078 = v0;
        let a4079 = v5;
        let a4080 = 0u64;
        nat_rem(a4078, a4079, a4080)
    };
    let v7: u64 = {
        let a4081 = v0;
        let a4082 = v5;
        let a4083 = 0u64;
        nat_quot(a4081, a4082, a4083)
    };
    let v8: Adt13 = Adt13::C1(f79(v7, v2)?);
    let a4084 = v6;
    let a4085 = 0u64;
    if nat_eq(a4084, a4085) {
        Ok(Adt9::C7(Adt11::C7(16u64, v3, 0u64), Adt12::C0(0u64, f79(v7, v1)?), Adt12::C2(0u64), Adt13::C0(0u64), f156(v3), 4u64))
    } else {
        let a4086 = v6;
        let a4087 = 1u64;
        if nat_eq(a4086, a4087) {
            Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C1(0u64), Adt12::C2(1065353216u64), {
                let a4088 = v4;
                let a4089 = 0u64;
                if nat_eq(a4088, a4089) {
                    v8
                } else {
                    Adt13::C0(0u64)
                }
            }, 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C2(0u64, v4, true), Adt12::C1(0u64), Adt12::C2(0u64), v8, 4u64, f156(v4)))
        }
    }
}

pub fn f146(v0: u64, v1: Adt3, v2: Adt3) -> R<Adt9> {
    let v3: u64 = f79(v0, v2.clone())?;
    let v4: u64 = f79(v0, v1.clone())?;
    let a4094 = {
        let a4091 = v0;
        let a4092 = match v1 {
            Adt3::C0(h4090, _, _, _, _, _, _, _) => {
                h4090
            }
        };
        let a4093 = 0u64;
        nat_rem(a4091, a4092, a4093)
    };
    let a4095 = 0u64;
    if nat_eq(a4094, a4095) {
        Ok(Adt9::C1(false, v4, v3))
    } else {
        Ok(Adt9::C4(8u64, 3u64, f79({
            let a4096 = v0;
            let a4097 = 1u64;
            nat_sub(a4096, a4097)
        }, v2)?, 0u64, v4, 3u64, v3))
    }
}

pub fn f147(v0: u64, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a4098 = v3;
    let a4099 = 13u64;
    if nat_eq(a4098, a4099) {
        f227(v0, v1, v4)
    } else {
        let v5: u64 = f228(match v1.clone() {
            Adt3::C0(h4100, _, _, _, _, _, _, _) => {
                h4100
            }
        }, {
            let a4101 = v3;
            let a4102 = 16u64;
            nat_eq(a4101, a4102)
        })?;
        let v6: u64 = {
            let a4103 = v0;
            let a4104 = v5;
            let a4105 = 0u64;
            nat_quot(a4103, a4104, a4105)
        };
        let v7: u64 = {
            let a4106 = v0;
            let a4107 = v5;
            let a4108 = 0u64;
            nat_rem(a4106, a4107, a4108)
        };
        let v8: u64 = {
            let a4112 = 1u64;
            let a4113 = {
                let a4110 = match v1.clone() {
                    Adt3::C0(h4109, _, _, _, _, _, _, _) => {
                        h4109
                    }
                };
                let a4111 = 2u64;
                nat_mul(a4110, a4111)?
            };
            nat_add(a4112, a4113)?
        };
        let a4114 = v7;
        let a4115 = v8;
        if nat_lt(a4114, a4115) {
            f229(v7, v6, v1, 0u64)
        } else {
            let v9: u64 = f79(v6, v2)?;
            let a4116 = v3;
            let a4117 = 14u64;
            if nat_eq(a4116, a4117) {
                Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 3u64, v9))
            } else {
                let a4118 = v7;
                let a4119 = v8;
                if nat_eq(a4118, a4119) {
                    Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4122 = v7;
                    let a4123 = {
                        let a4120 = v8;
                        let a4121 = 1u64;
                        nat_add(a4120, a4121)?
                    };
                    if nat_eq(a4122, a4123) {
                        Ok(Adt9::C4(3u64, 2u64, match v1 {
                            Adt3::C0(h4124, _, _, _, _, _, _, _) => {
                                h4124
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

pub fn f148(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let a4125 = v0;
    let a4126 = 0u64;
    if nat_eq(a4125, a4126) {
        Ok(Adt9::C4(19u64, 2u64, 0u64, 2u64, 0u64, 1u64, 0u64))
    } else {
        let a4133 = {
            let a4127 = v0;
            let a4128 = 1u64;
            nat_add(a4127, a4128)?
        };
        let a4134 = {
            let a4131 = 2u64;
            let a4132 = {
                let a4129 = v3;
                let a4130 = 2u64;
                nat_mul(a4129, a4130)?
            };
            nat_add(a4131, a4132)?
        };
        if nat_eq(a4133, a4134) {
            Ok(Adt9::C4(19u64, 1u64, 0u64, 2u64, 0u64, 6u64, 0u64))
        } else {
            let v4: u64 = {
                let a4135 = v0;
                let a4136 = 1u64;
                nat_sub(a4135, a4136)
            };
            let v5: u64 = {
                let a4137 = v4;
                let a4138 = 2u64;
                let a4139 = 0u64;
                nat_quot(a4137, a4138, a4139)
            };
            let a4143 = {
                let a4140 = v4;
                let a4141 = 2u64;
                let a4142 = 0u64;
                nat_rem(a4140, a4141, a4142)
            };
            let a4144 = 0u64;
            if nat_eq(a4143, a4144) {
                Ok(Adt9::C4(17u64, 0u64, f79(v5, v1)?, 4u64, f79(v5, v2)?, 1u64, 2u64))
            } else {
                Ok(Adt9::C4(18u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            }
        }
    }
}

pub fn f149(v0: u64, v1: u64, v2: u64, v3: u64) -> R<Adt9> {
    let a4145 = v0;
    let a4146 = f33(v1, v2, v3)?;
    if nat_lt(a4145, a4146) {
        let a4147 = v0;
        let a4148 = v1;
        if nat_lt(a4147, a4148) {
            Ok(Adt9::C1(false, {
                let a4149 = 4u64;
                let a4150 = v0;
                nat_mul(a4149, a4150)?
            }, {
                let a4151 = 4u64;
                let a4152 = v0;
                nat_mul(a4151, a4152)?
            }))
        } else {
            let v4: u64 = {
                let a4153 = v0;
                let a4154 = v1;
                nat_sub(a4153, a4154)
            };
            let v5: u64 = f230(v4, v2)?;
            let a4155 = v5;
            let a4156 = 2u64;
            if nat_lt(a4155, a4156) {
                Ok(Adt9::C4(6u64, {
                    let a4157 = v5;
                    let a4158 = 0u64;
                    if nat_eq(a4157, a4158) {
                        4u64
                    } else {
                        5u64
                    }
                }, f231(v4, v2)?, 2u64, 0u64, 1u64, v5))
            } else {
                let v6: u64 = f232(v4, v2)?;
                Ok(Adt9::C4(8u64, 3u64, v6, 1u64, if f233(v4, v2)? {
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

pub fn f150(v0: u64, v1: u64) -> u64 {
    let a4159 = v0;
    let a4160 = 0u64;
    if if nat_eq(a4159, a4160) {
        true
    } else {
        let a4161 = v1;
        let a4162 = 0u64;
        nat_eq(a4161, a4162)
    } {
        1u64
    } else {
        2u64
    }
}

pub fn f151(v0: u64, v1: u64) -> Adt11 {
    let a4163 = v0;
    let a4164 = 43u64;
    if nat_eq(a4163, a4164) {
        Adt11::C8(3u64)
    } else {
        let a4165 = v1;
        let a4166 = 43u64;
        if nat_eq(a4165, a4166) {
            Adt11::C8(4u64)
        } else {
            Adt11::C2(v0, v1, true)
        }
    }
}

pub fn f152(v0: u64) -> R<u64> {
    let a4169 = 2u64;
    let a4170 = {
        let a4167 = 6u64;
        let a4168 = v0;
        nat_mul(a4167, a4168)?
    };
    nat_add(a4169, a4170)
}

pub fn f153(v0: u64, v1: bool) -> R<u64> {
    let a4173 = {
        let a4171 = v0;
        let a4172 = if v1 {
            3u64
        } else {
            2u64
        };
        nat_mul(a4171, a4172)?
    };
    let a4174 = 1u64;
    nat_add(a4173, a4174)
}

pub fn f154(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: Adt3, v7: u64, v8: u64) -> R<Adt9> {
    let a4175 = v0;
    let a4176 = 16u64;
    if nat_lt(a4175, a4176) {
        let v9: u64 = {
            let a4177 = v0;
            let a4178 = 4u64;
            let a4179 = 0u64;
            nat_quot(a4177, a4178, a4179)
        };
        let v10: u64 = {
            let a4184 = {
                let a4182 = 8u64;
                let a4183 = {
                    let a4180 = 4u64;
                    let a4181 = v1;
                    nat_mul(a4180, a4181)?
                };
                nat_add(a4182, a4183)?
            };
            let a4185 = v9;
            nat_add(a4184, a4185)?
        };
        let v11: u64 = f49(v6, v9, v1, v2, 0u64)?;
        let a4189 = {
            let a4186 = v0;
            let a4187 = 4u64;
            let a4188 = 0u64;
            nat_rem(a4186, a4187, a4188)
        };
        let a4190 = 0u64;
        if nat_eq(a4189, a4190) {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C0(0u64, f49(v3, v10, v2, 0u64, 0u64)?), Adt12::C0(1u64, f49(v4, 2u64, 0u64, 0u64, 0u64)?), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            let a4194 = {
                let a4191 = v0;
                let a4192 = 4u64;
                let a4193 = 0u64;
                nat_rem(a4191, a4192, a4193)
            };
            let a4195 = 1u64;
            if nat_eq(a4194, a4195) {
                Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(1u64), Adt12::C0(2u64, f49(v5, v10, 0u64, 0u64, 0u64)?), Adt13::C1(v11), 4u64, 4u64))
            } else {
                let a4199 = {
                    let a4196 = v0;
                    let a4197 = 4u64;
                    let a4198 = 0u64;
                    nat_rem(a4196, a4197, a4198)
                };
                let a4200 = 2u64;
                if nat_eq(a4199, a4200) {
                    Ok(Adt9::C7(Adt11::C5(0u64), {
                        let a4201 = v9;
                        let a4202 = 0u64;
                        if nat_eq(a4201, a4202) {
                            Adt12::C2(4286578688u64)
                        } else {
                            Adt12::C1(0u64)
                        }
                    }, Adt12::C3(v11), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C5(3u64), {
                        let a4203 = v8;
                        let a4204 = 2u64;
                        if nat_eq(a4203, a4204) {
                            let a4205 = v9;
                            let a4206 = 0u64;
                            if nat_eq(a4205, a4206) {
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
        let a4207 = v0;
        let a4208 = 28u64;
        if nat_lt(a4207, a4208) {
            let v12: u64 = {
                let a4209 = v0;
                let a4210 = 16u64;
                nat_sub(a4209, a4210)
            };
            let v13: u64 = {
                let a4211 = v12;
                let a4212 = 3u64;
                let a4213 = 0u64;
                nat_quot(a4211, a4212, a4213)
            };
            let v14: u64 = f49(v6, v13, v1, v2, 0u64)?;
            let a4217 = {
                let a4214 = v12;
                let a4215 = 3u64;
                let a4216 = 0u64;
                nat_rem(a4214, a4215, a4216)
            };
            let a4218 = 0u64;
            if nat_eq(a4217, a4218) {
                Ok(Adt9::C7(Adt11::C1(1u64), Adt12::C3(v14), Adt12::C1(0u64), Adt13::C0(2u64), 4u64, 4u64))
            } else {
                let a4222 = {
                    let a4219 = v12;
                    let a4220 = 3u64;
                    let a4221 = 0u64;
                    nat_rem(a4219, a4220, a4221)
                };
                let a4223 = 1u64;
                if nat_eq(a4222, a4223) {
                    Ok(Adt9::C7(Adt11::C7(16u64, 0u64, 0u64), Adt12::C1(2u64), Adt12::C2(0u64), Adt13::C1(v14), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), {
                        let a4224 = v13;
                        let a4225 = 0u64;
                        if nat_eq(a4224, a4225) {
                            Adt12::C2(0u64)
                        } else {
                            Adt12::C1(1u64)
                        }
                    }, Adt12::C3(v14), Adt13::C0(1u64), 4u64, 4u64))
                }
            }
        } else {
            let a4226 = v0;
            let a4227 = 28u64;
            if nat_eq(a4226, a4227) {
                Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
            } else {
                let v15: u64 = {
                    let a4228 = v0;
                    let a4229 = 29u64;
                    nat_sub(a4228, a4229)
                };
                let v16: u64 = {
                    let a4230 = v15;
                    let a4231 = 2u64;
                    let a4232 = 0u64;
                    nat_quot(a4230, a4231, a4232)
                };
                let v17: u64 = f49(v6, v16, v1, v2, 0u64)?;
                let a4236 = {
                    let a4233 = v15;
                    let a4234 = 2u64;
                    let a4235 = 0u64;
                    nat_rem(a4233, a4234, a4235)
                };
                let a4237 = 0u64;
                if nat_eq(a4236, a4237) {
                    Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v17), Adt12::C1(1u64), Adt13::C0(2u64), 4u64, 4u64))
                } else {
                    Ok(Adt9::C7(Adt11::C1(0u64), Adt12::C1(2u64), Adt12::C2(v7), Adt13::C1(v17), 4u64, 4u64))
                }
            }
        }
    }
}

pub fn f155(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4238 = v0;
        let a4239 = 9u64;
        let a4240 = 0u64;
        nat_rem(a4238, a4239, a4240)
    };
    let v5: u64 = {
        let a4244 = {
            let a4241 = v0;
            let a4242 = 9u64;
            let a4243 = 0u64;
            nat_quot(a4241, a4242, a4243)
        };
        let a4245 = 4u64;
        let a4246 = 0u64;
        nat_rem(a4244, a4245, a4246)
    };
    let v6: bool = {
        let a4253 = {
            let a4250 = {
                let a4247 = v0;
                let a4248 = 36u64;
                let a4249 = 0u64;
                nat_quot(a4247, a4248, a4249)
            };
            let a4251 = 2u64;
            let a4252 = 0u64;
            nat_rem(a4250, a4251, a4252)
        };
        let a4254 = 0u64;
        nat_eq(a4253, a4254)
    };
    let v7: u64 = {
        let a4255 = v4;
        let a4256 = 4u64;
        if nat_lt(a4255, a4256) {
            v4
        } else {
            let a4257 = v4;
            let a4258 = 5u64;
            nat_sub(a4257, a4258)
        }
    };
    let v8: u64 = f49(v2, if v6 {
        v5
    } else {
        v7
    }, if v6 {
        v7
    } else {
        v5
    }, v1, 0u64)?;
    let a4259 = v4;
    let a4260 = 4u64;
    if nat_lt(a4259, a4260) {
        Ok(Adt9::C7(Adt11::C1(0u64), {
            let a4261 = v4;
            let a4262 = 0u64;
            if nat_eq(a4261, a4262) {
                Adt12::C2(v3)
            } else {
                Adt12::C1(1u64)
            }
        }, Adt12::C3(v8), Adt13::C0(1u64), 4u64, 4u64))
    } else {
        let a4263 = v4;
        let a4264 = 4u64;
        if nat_eq(a4263, a4264) {
            Ok(Adt9::C7(Adt11::C1(3u64), Adt12::C2(1065353216u64), Adt12::C1(1u64), Adt13::C0(1u64), 4u64, 4u64))
        } else {
            Ok(Adt9::C7(Adt11::C1(2u64), Adt12::C3(v8), Adt12::C1(1u64), Adt13::C1(v8), 4u64, 4u64))
        }
    }
}

pub fn f156(v0: u64) -> u64 {
    let m4265 = f197(v0);
    match m4265 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            let a4267 = match v1.clone() {
                Adt19::C0(_, h4266, _) => {
                    h4266
                }
            };
            let a4268 = 1u64;
            if if nat_eq(a4267, a4268) {
                let a4270 = match v1.clone() {
                    Adt19::C0(_, _, h4269) => {
                        h4269
                    }
                };
                let a4271 = 1u64;
                if if if nat_eq(a4270, a4271) {
                    true
                } else {
                    let a4273 = match v1.clone() {
                        Adt19::C0(_, _, h4272) => {
                            h4272
                        }
                    };
                    let a4274 = 2u64;
                    nat_eq(a4273, a4274)
                } {
                    true
                } else {
                    let a4276 = match v1.clone() {
                        Adt19::C0(_, _, h4275) => {
                            h4275
                        }
                    };
                    let a4277 = 4u64;
                    nat_eq(a4276, a4277)
                } {
                    true
                } else {
                    let a4279 = match v1.clone() {
                        Adt19::C0(_, _, h4278) => {
                            h4278
                        }
                    };
                    let a4280 = 8u64;
                    nat_eq(a4279, a4280)
                }
            } else {
                false
            } {
                match v1 {
                    Adt19::C0(_, _, h4281) => {
                        h4281
                    }
                }
            } else {
                0u64
            }
        }
    }
}

pub fn f157(v0: u64) -> R<u64> {
    let a4284 = 3u64;
    let a4285 = {
        let a4282 = v0;
        let a4283 = 3u64;
        nat_mul(a4282, a4283)?
    };
    nat_add(a4284, a4285)
}

pub fn f158(v0: u64, v1: u64) -> R<u64> {
    if f159(v0) {
        let a4286 = 2u64;
        let a4287 = v1;
        nat_mul(a4286, a4287)
    } else {
        let a4288 = v1;
        let a4289 = f234(v0);
        let a4290 = 0u64;
        Ok(nat_quot(a4288, a4289, a4290))
    }
}

pub fn f159(v0: u64) -> bool {
    let a4291 = v0;
    let a4292 = 0u64;
    if if nat_eq(a4291, a4292) {
        true
    } else {
        let a4293 = v0;
        let a4294 = 1u64;
        nat_eq(a4293, a4294)
    } {
        true
    } else {
        let a4295 = v0;
        let a4296 = 30u64;
        nat_eq(a4295, a4296)
    }
}

pub fn f160(v0: u64, v1: u64, v2: u64, v3: u64, v4: Adt3, v5: Adt3) -> R<Adt9> {
    let m4297 = f197(v0);
    match m4297 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v6) => {
            Ok(Adt9::C10(v0, f126(v2, v1, 0u64, 0u64, match v4.clone() {
                Adt3::C0(_, _, _, _, h4298, _, _, _) => {
                    h4298
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, h4299, _, _) => {
                    h4299
                }
            }, match v4.clone() {
                Adt3::C0(_, _, _, _, _, _, h4300, _) => {
                    h4300
                }
            }, match v4 {
                Adt3::C0(_, _, _, _, _, _, _, h4301) => {
                    h4301
                }
            })?, match v6.clone() {
                Adt19::C0(_, _, h4302) => {
                    h4302
                }
            }, f126({
                let a4304 = v2;
                let a4305 = match v6.clone() {
                    Adt19::C0(_, h4303, _) => {
                        h4303
                    }
                };
                nat_mul(a4304, a4305)?
            }, v3, 0u64, 0u64, match v5.clone() {
                Adt3::C0(_, _, _, _, h4306, _, _, _) => {
                    h4306
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, h4307, _, _) => {
                    h4307
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, h4308, _) => {
                    h4308
                }
            }, match v5.clone() {
                Adt3::C0(_, _, _, _, _, _, _, h4309) => {
                    h4309
                }
            })?, match v6 {
                Adt19::C0(_, h4310, _) => {
                    h4310
                }
            }, match v5 {
                Adt3::C0(_, _, _, _, h4311, _, _, _) => {
                    h4311
                }
            }))
        }
    }
}

pub fn f161(v0: Adt3, v1: Adt3) -> R<u64> {
    let a4319 = {
        let a4312 = 2u64;
        let a4313 = f162(v1.clone())?;
        nat_mul(a4312, a4313)?
    };
    let a4320 = {
        let a4317 = f162(v1)?;
        let a4318 = {
            let a4315 = 1u64;
            let a4316 = match v0 {
                Adt3::C0(h4314, _, _, _, _, _, _, _) => {
                    h4314
                }
            };
            nat_add(a4315, a4316)?
        };
        nat_mul(a4317, a4318)?
    };
    nat_add(a4319, a4320)
}

pub fn f162(v0: Adt3) -> R<u64> {
    if f40(match v0.clone() {
        Adt3::C0(h4321, _, _, _, _, _, _, _) => {
            h4321
        }
    }, match v0.clone() {
        Adt3::C0(_, h4322, _, _, _, _, _, _) => {
            h4322
        }
    }, match v0.clone() {
        Adt3::C0(_, _, h4323, _, _, _, _, _) => {
            h4323
        }
    }, match v0.clone() {
        Adt3::C0(_, _, _, h4324, _, _, _, _) => {
            h4324
        }
    }) {
        Ok(0u64)
    } else {
        let a4333 = match v0.clone() {
            Adt3::C0(h4325, _, _, _, _, _, _, _) => {
                h4325
            }
        };
        let a4334 = {
            let a4331 = match v0.clone() {
                Adt3::C0(_, h4326, _, _, _, _, _, _) => {
                    h4326
                }
            };
            let a4332 = {
                let a4329 = match v0.clone() {
                    Adt3::C0(_, _, h4327, _, _, _, _, _) => {
                        h4327
                    }
                };
                let a4330 = match v0 {
                    Adt3::C0(_, _, _, h4328, _, _, _, _) => {
                        h4328
                    }
                };
                nat_mul(a4329, a4330)?
            };
            nat_mul(a4331, a4332)?
        };
        nat_mul(a4333, a4334)
    }
}

pub fn f163(v0: u64, v1: Adt3) -> R<Adt9> {
    Ok(Adt9::C4(19u64, 4u64, f79(v0, v1)?, 2u64, 0u64, 1u64, 2u64))
}

pub fn f164(v0: u64, v1: u64) -> bool {
    let a4335 = v1;
    let a4336 = 2147483647u64;
    if nat_le(a4335, a4336) {
        let a4337 = v1;
        let a4338 = v0;
        nat_lt(a4337, a4338)
    } else {
        false
    }
}

pub fn f165(v0: u64, v1: u64, v2: u64, v3: Adt3, v4: Adt3, v5: Adt3, v6: u64) -> R<Adt9> {
    let m4339 = f197(v0);
    match m4339 {
        Adt18::C0 => {
            Ok(Adt9::C0(Adt10::C1))
        }
        Adt18::C1(v7) => {
            let v8: u64 = {
                let a4341 = v2;
                let a4342 = match v7.clone() {
                    Adt19::C0(_, h4340, _) => {
                        h4340
                    }
                };
                nat_mul(a4341, a4342)?
            };
            let m4355 = f235(v0, v8, v6, v1, match v5.clone() {
                Adt3::C0(h4343, _, _, _, _, _, _, _) => {
                    h4343
                }
            }, match v5.clone() {
                Adt3::C0(_, h4344, _, _, _, _, _, _) => {
                    h4344
                }
            }, match v5 {
                Adt3::C0(_, _, h4345, _, _, _, _, _) => {
                    h4345
                }
            }, {
                let a4348 = match v3.clone() {
                    Adt3::C0(h4346, _, _, _, _, _, _, _) => {
                        h4346
                    }
                };
                let a4349 = match v7.clone() {
                    Adt19::C0(_, h4347, _) => {
                        h4347
                    }
                };
                nat_mul(a4348, a4349)?
            }, match v3.clone() {
                Adt3::C0(_, h4350, _, _, _, _, _, _) => {
                    h4350
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, h4351, _, _, _) => {
                    h4351
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, h4352, _, _) => {
                    h4352
                }
            }, match v3.clone() {
                Adt3::C0(_, _, _, _, _, _, h4353, _) => {
                    h4353
                }
            }, match v3 {
                Adt3::C0(_, _, _, _, _, _, _, h4354) => {
                    h4354
                }
            })?;
            match m4355 {
                None => {
                    Ok(Adt9::C0(Adt10::C1))
                }
                Some(v9) => {
                    let a4356 = v0;
                    let a4357 = 0u64;
                    if if nat_eq(a4356, a4357) {
                        true
                    } else {
                        let a4358 = v0;
                        let a4359 = 26u64;
                        nat_eq(a4358, a4359)
                    } {
                        Ok(Adt9::C1(false, v9, f79({
                            let a4363 = {
                                let a4361 = v1;
                                let a4362 = match v4.clone() {
                                    Adt3::C0(h4360, _, _, _, _, _, _, _) => {
                                        h4360
                                    }
                                };
                                nat_mul(a4361, a4362)?
                            };
                            let a4364 = v8;
                            nat_add(a4363, a4364)?
                        }, v4)?))
                    } else {
                        Ok(Adt9::C5(v0, v9, match v7.clone() {
                            Adt19::C0(_, _, h4365) => {
                                h4365
                            }
                        }, f79({
                            let a4369 = {
                                let a4367 = v1;
                                let a4368 = match v4.clone() {
                                    Adt3::C0(h4366, _, _, _, _, _, _, _) => {
                                        h4366
                                    }
                                };
                                nat_mul(a4367, a4368)?
                            };
                            let a4370 = v8;
                            nat_add(a4369, a4370)?
                        }, v4.clone())?, {
                            let a4377 = {
                                let a4375 = {
                                    let a4372 = match v7 {
                                        Adt19::C0(_, h4371, _) => {
                                            h4371
                                        }
                                    };
                                    let a4373 = 1u64;
                                    nat_sub(a4372, a4373)
                                };
                                let a4376 = match v4.clone() {
                                    Adt3::C0(_, _, _, _, h4374, _, _, _) => {
                                        h4374
                                    }
                                };
                                nat_mul(a4375, a4376)?
                            };
                            let a4378 = 4u64;
                            nat_add(a4377, a4378)?
                        }, match v4 {
                            Adt3::C0(_, _, _, _, h4379, _, _, _) => {
                                h4379
                            }
                        }))
                    }
                }
            }
        }
    }
}

pub fn f166(v0: u64, v1: u64, v2: u64) -> u64 {
    let a4380 = v1;
    let a4381 = 0u64;
    if if nat_eq(a4380, a4381) {
        true
    } else {
        let a4382 = v2;
        let a4383 = 0u64;
        nat_eq(a4382, a4383)
    } {
        0u64
    } else {
        v0
    }
}

pub fn f167(v0: u64, v1: u64) -> bool {
    let a4384 = v0;
    let a4385 = 4294967295u64;
    if nat_eq(a4384, a4385) {
        true
    } else {
        let a4386 = v0;
        let a4387 = 2147483648u64;
        if nat_lt(a4386, a4387) {
            let a4388 = v0;
            let a4389 = v1;
            nat_lt(a4388, a4389)
        } else {
            false
        }
    }
}

pub fn f168(v0: Adt7) -> R<u64> {
    let a4401 = if match v0.clone() {
        Adt7::C0(_, _, _, h4390, _) => {
            h4390
        }
    } {
        2u64
    } else {
        3u64
    };
    let a4402 = {
        let a4399 = {
            let a4397 = {
                let m4393 = match v0.clone() {
                    Adt7::C0(h4392, _, _, _, _) => {
                        h4392
                    }
                };
                match m4393 {
                    Adt6::C0(h4391, _, _, _) => {
                        h4391
                    }
                }
            };
            let a4398 = {
                let m4396 = match v0.clone() {
                    Adt7::C0(_, h4395, _, _, _) => {
                        h4395
                    }
                };
                match m4396 {
                    Adt6::C0(h4394, _, _, _) => {
                        h4394
                    }
                }
            };
            nat_mul(a4397, a4398)?
        };
        let a4400 = f169(v0)?;
        nat_mul(a4399, a4400)?
    };
    nat_add(a4401, a4402)
}

pub fn f169(v0: Adt7) -> R<u64> {
    let a4407 = if match v0.clone() {
        Adt7::C0(_, _, _, h4403, _) => {
            h4403
        }
    } {
        2u64
    } else {
        1u64
    };
    let a4408 = {
        let a4405 = match v0 {
            Adt7::C0(_, _, _, _, h4404) => {
                h4404
            }
        };
        let a4406 = 1u64;
        if nat_eq(a4405, a4406) {
            1u64
        } else {
            0u64
        }
    };
    nat_add(a4407, a4408)
}

pub fn f170(v0: Adt6, v1: u64, v2: u64) -> R<u64> {
    let v3: u64 = {
        let a4413 = {
            let a4410 = v2;
            let a4411 = match v0.clone() {
                Adt6::C0(_, h4409, _, _) => {
                    h4409
                }
            };
            nat_mul(a4410, a4411)?
        };
        let a4414 = f127(match v0.clone() {
            Adt6::C0(_, _, h4412, _) => {
                h4412
            }
        });
        nat_add(a4413, a4414)?
    };
    let v4: u64 = f128(match v0.clone() {
        Adt6::C0(_, _, h4415, _) => {
            h4415
        }
    });
    let v5: u64 = {
        let a4419 = {
            let a4417 = v3;
            let a4418 = match v0 {
                Adt6::C0(h4416, _, _, _) => {
                    h4416
                }
            };
            nat_add(a4417, a4418)?
        };
        let a4420 = v4;
        nat_sub(a4419, a4420)
    };
    let a4425 = {
        let a4421 = v5;
        let a4422 = v1;
        if nat_lt(a4421, a4422) {
            v5
        } else {
            v1
        }
    };
    let a4426 = {
        let a4423 = v3;
        let a4424 = v4;
        nat_sub(a4423, a4424)
    };
    Ok(nat_sub(a4425, a4426))
}

pub fn f171(v0: Adt7, v1: Adt3, v2: Adt3, v3: u64, v4: u64) -> R<Option<u64>> {
    let m4435 = f52(match v0.clone() {
        Adt7::C0(h4427, _, _, _, _) => {
            h4427
        }
    }, match v1.clone() {
        Adt3::C0(h4428, _, _, _, _, _, _, _) => {
            h4428
        }
    }, f50(v3, v2.clone(), 0u64), {
        let a4432 = v4;
        let a4433 = {
            let m4431 = match v0.clone() {
                Adt7::C0(h4430, _, _, _, _) => {
                    h4430
                }
            };
            match m4431 {
                Adt6::C0(h4429, _, _, _) => {
                    h4429
                }
            }
        };
        let a4434 = 0u64;
        nat_rem(a4432, a4433, a4434)
    })?;
    match m4435 {
        None => {
            Ok(None::<u64>)
        }
        Some(v5) => {
            let m4444 = f52(match v0.clone() {
                Adt7::C0(_, h4436, _, _, _) => {
                    h4436
                }
            }, match v1.clone() {
                Adt3::C0(_, h4437, _, _, _, _, _, _) => {
                    h4437
                }
            }, f50(v3, v2.clone(), 1u64), {
                let a4441 = v4;
                let a4442 = {
                    let m4440 = match v0 {
                        Adt7::C0(h4439, _, _, _, _) => {
                            h4439
                        }
                    };
                    match m4440 {
                        Adt6::C0(h4438, _, _, _) => {
                            h4438
                        }
                    }
                };
                let a4443 = 0u64;
                nat_quot(a4441, a4442, a4443)
            })?;
            match m4444 {
                None => {
                    Ok(None::<u64>)
                }
                Some(v6) => {
                    Ok(Some(f49(v1, v5, v6, f50(v3, v2.clone(), 2u64), f50(v3, v2, 3u64))?))
                }
            }
        }
    }
}

pub fn f172(v0: u64) -> R<u64> {
    let a4447 = 2u64;
    let a4448 = {
        let a4445 = v0;
        let a4446 = 2u64;
        nat_mul(a4445, a4446)?
    };
    nat_add(a4447, a4448)
}

pub fn f173(v0: Adt6) -> R<u64> {
    let a4461 = 2u64;
    let a4462 = {
        let a4459 = 2u64;
        let a4460 = {
            let a4456 = {
                let a4453 = match v0.clone() {
                    Adt6::C0(h4449, _, _, _) => {
                        h4449
                    }
                };
                let a4454 = {
                    let a4451 = match v0.clone() {
                        Adt6::C0(_, h4450, _, _) => {
                            h4450
                        }
                    };
                    let a4452 = 1u64;
                    nat_sub(a4451, a4452)
                };
                nat_add(a4453, a4454)?
            };
            let a4457 = match v0 {
                Adt6::C0(_, h4455, _, _) => {
                    h4455
                }
            };
            let a4458 = 0u64;
            nat_quot(a4456, a4457, a4458)
        };
        nat_mul(a4459, a4460)?
    };
    nat_add(a4461, a4462)
}

pub fn f174(v0: Adt16, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64) -> R<Adt17> {
    let m4463 = v0;
    match m4463 {
        Adt16::C0(v19) => {
            f236(v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v19, v18)
        }
        Adt16::C1(v20, v21, v22, v23) => {
            let a4464 = v20;
            let a4465 = 66u64;
            if nat_eq(a4464, a4465) {
                f237(v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v21, v22, v23, v18)
            } else {
                f238(v20, v1, v2, v3, v4, v5, v6, v7, v8, v9, v14, v15, v16, v17, v21, v22, v23, v18)
            }
        }
    }
}

pub fn f175(v0: u64, v1: Adt3, v2: u64, v3: u64, v4: u64) -> R<Adt9> {
    let v5: u64 = {
        let a4472 = v3;
        let a4473 = {
            let a4469 = f79({
                let a4466 = v0;
                let a4467 = 2u64;
                let a4468 = 0u64;
                nat_quot(a4466, a4467, a4468)
            }, v1)?;
            let a4470 = v2;
            let a4471 = 0u64;
            nat_quot(a4469, a4470, a4471)
        };
        nat_add(a4472, a4473)?
    };
    let a4477 = {
        let a4474 = v0;
        let a4475 = 2u64;
        let a4476 = 0u64;
        nat_rem(a4474, a4475, a4476)
    };
    let a4478 = 0u64;
    if nat_eq(a4477, a4478) {
        Ok(Adt9::C4(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a4479 = v4;
        let a4480 = 0u64;
        if nat_eq(a4479, a4480) {
            Ok(Adt9::C4(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt9::C0(Adt10::C1))
        }
    }
}

pub fn f176(v0: u64, v1: u64) -> R<u64> {
    let a4483 = 8u64;
    let a4484 = {
        let a4481 = f239(v0);
        let a4482 = v1;
        nat_mul(a4481, a4482)?
    };
    nat_add(a4483, a4484)
}

pub fn f177(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64, v5: u64) -> R<Adt9> {
    let v6: u64 = match v2.clone() {
        Adt3::C0(h4485, _, _, _, _, _, _, _) => {
            h4485
        }
    };
    let v7: u64 = f239(v3);
    let v8: u64 = {
        let a4488 = 1u64;
        let a4489 = {
            let a4486 = v6;
            let a4487 = v7;
            nat_mul(a4486, a4487)?
        };
        nat_add(a4488, a4489)?
    };
    let a4490 = v0;
    let a4491 = 0u64;
    if nat_eq(a4490, a4491) {
        Ok(f240(0u64))
    } else {
        let a4492 = v0;
        let a4493 = v8;
        if nat_lt(a4492, a4493) {
            let v9: u64 = {
                let a4494 = v0;
                let a4495 = 1u64;
                nat_sub(a4494, a4495)
            };
            let v10: u64 = {
                let a4496 = v9;
                let a4497 = v7;
                let a4498 = 0u64;
                nat_rem(a4496, a4497, a4498)
            };
            let v11: u64 = f79({
                let a4504 = {
                    let a4499 = v1;
                    let a4500 = v6;
                    nat_mul(a4499, a4500)?
                };
                let a4505 = {
                    let a4501 = v9;
                    let a4502 = v7;
                    let a4503 = 0u64;
                    nat_quot(a4501, a4502, a4503)
                };
                nat_add(a4504, a4505)?
            }, v2)?;
            let a4506 = v3;
            let a4507 = 30u64;
            if nat_eq(a4506, a4507) {
                let a4508 = v10;
                let a4509 = 0u64;
                if nat_eq(a4508, a4509) {
                    Ok(Adt9::C4(6u64, 0u64, v11, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a4510 = v10;
                    let a4511 = 1u64;
                    if nat_eq(a4510, a4511) {
                        Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, 1u64, 1u64, 1u64))
                    } else {
                        let a4512 = v10;
                        let a4513 = 2u64;
                        if nat_eq(a4512, a4513) {
                            Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                        }
                    }
                }
            } else {
                let a4514 = v10;
                let a4515 = 0u64;
                if nat_eq(a4514, a4515) {
                    Ok(Adt9::C4(10u64, 0u64, v11, 0u64, v11, 1u64, 1u64))
                } else {
                    let a4516 = v10;
                    let a4517 = 1u64;
                    if nat_eq(a4516, a4517) {
                        Ok(Adt9::C4(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        Ok(Adt9::C4(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    }
                }
            }
        } else {
            let v12: u64 = {
                let a4518 = v0;
                let a4519 = v8;
                nat_sub(a4518, a4519)
            };
            let a4520 = v12;
            let a4521 = 0u64;
            if nat_eq(a4520, a4521) {
                Ok(Adt9::C4(22u64, 2u64, v6, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a4522 = v12;
                let a4523 = 1u64;
                if nat_eq(a4522, a4523) {
                    Ok(Adt9::C4(21u64, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64))
                } else {
                    let a4524 = v12;
                    let a4525 = 2u64;
                    if nat_eq(a4524, a4525) {
                        Ok(Adt9::C4(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                    } else {
                        let a4526 = v12;
                        let a4527 = 3u64;
                        if nat_eq(a4526, a4527) {
                            Ok(Adt9::C4(4u64, 1u64, 1u64, 2u64, v4, 1u64, 1u64))
                        } else {
                            let a4528 = v12;
                            let a4529 = 4u64;
                            if nat_eq(a4528, a4529) {
                                Ok(Adt9::C4(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                            } else {
                                let a4530 = v12;
                                let a4531 = 5u64;
                                if nat_eq(a4530, a4531) {
                                    Ok(Adt9::C4(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                                } else {
                                    let a4532 = v12;
                                    let a4533 = 6u64;
                                    if nat_eq(a4532, a4533) {
                                        if f241(v5) {
                                            Ok(Adt9::C4(19u64, 1u64, 2u64, 2u64, 0u64, 1u64, {
                                                let a4534 = 3u64;
                                                let a4535 = v1;
                                                nat_add(a4534, a4535)?
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

pub fn f178(v0: u64, v1: Adt3, v2: Adt3, v3: u64) -> R<Adt9> {
    let v4: u64 = {
        let a4536 = v0;
        let a4537 = f90(v3);
        let a4538 = 0u64;
        nat_quot(a4536, a4537, a4538)
    };
    let v5: u64 = {
        let a4543 = 3u64;
        let a4544 = {
            let a4540 = v4;
            let a4541 = match v1.clone() {
                Adt3::C0(h4539, _, _, _, _, _, _, _) => {
                    h4539
                }
            };
            let a4542 = 0u64;
            nat_quot(a4540, a4541, a4542)
        };
        nat_add(a4543, a4544)?
    };
    let v6: u64 = f79(v4, v1)?;
    let v7: u64 = f79(v4, v2)?;
    let a4545 = v3;
    let a4546 = 30u64;
    if nat_eq(a4545, a4546) {
        let a4550 = {
            let a4547 = v0;
            let a4548 = 3u64;
            let a4549 = 0u64;
            nat_rem(a4547, a4548, a4549)
        };
        let a4551 = 0u64;
        if nat_eq(a4550, a4551) {
            Ok(Adt9::C4(6u64, 0u64, v6, 2u64, 0u64, 1u64, 1u64))
        } else {
            let a4555 = {
                let a4552 = v0;
                let a4553 = 3u64;
                let a4554 = 0u64;
                nat_rem(a4552, a4553, a4554)
            };
            let a4556 = 1u64;
            if nat_eq(a4555, a4556) {
                Ok(Adt9::C4(10u64, 1u64, 1u64, 1u64, v5, 1u64, 1u64))
            } else {
                Ok(Adt9::C4(7u64, 1u64, 1u64, 2u64, 0u64, 3u64, v7))
            }
        }
    } else {
        Ok(Adt9::C4(10u64, 0u64, v6, 1u64, v5, 3u64, v7))
    }
}

pub fn f179(v0: u64) -> R<u64> {
    let v1: u64 = {
        let a4557 = v0;
        let a4558 = 8388608u64;
        let a4559 = 0u64;
        nat_quot(a4557, a4558, a4559)
    };
    let a4560 = v0;
    let a4561 = 2147483648u64;
    if if if nat_lt(a4560, a4561) {
        let a4562 = 127u64;
        let a4563 = v1;
        nat_le(a4562, a4563)
    } else {
        false
    } {
        let a4564 = v1;
        let a4565 = 190u64;
        nat_lt(a4564, a4565)
    } else {
        false
    } {
        let v2: u64 = {
            let a4569 = 8388608u64;
            let a4570 = {
                let a4566 = v0;
                let a4567 = 8388608u64;
                let a4568 = 0u64;
                nat_rem(a4566, a4567, a4568)
            };
            nat_add(a4569, a4570)?
        };
        let a4571 = v1;
        let a4572 = 150u64;
        if nat_lt(a4571, a4572) {
            let v3: u64 = f242({
                let a4573 = 150u64;
                let a4574 = v1;
                nat_sub(a4573, a4574)
            });
            let a4578 = {
                let a4575 = v2;
                let a4576 = v3;
                let a4577 = 0u64;
                nat_rem(a4575, a4576, a4577)
            };
            let a4579 = 0u64;
            if nat_eq(a4578, a4579) {
                let a4580 = v2;
                let a4581 = v3;
                let a4582 = 0u64;
                Ok(nat_quot(a4580, a4581, a4582))
            } else {
                Ok(0u64)
            }
        } else {
            let a4585 = f242({
                let a4583 = v1;
                let a4584 = 150u64;
                nat_sub(a4583, a4584)
            });
            let a4586 = v2;
            nat_mul(a4585, a4586)
        }
    } else {
        Ok(0u64)
    }
}

pub fn f180(v0: Adt14, v1: Adt14) -> bool {
    let a4589 = match v0.clone() {
        Adt14::C0(_, h4587, _, _, _, _, _, _, _, _, _) => {
            h4587
        }
    };
    let a4590 = match v1.clone() {
        Adt14::C0(_, h4588, _, _, _, _, _, _, _, _, _) => {
            h4588
        }
    };
    if if if nat_eq(a4589, a4590) {
        let a4593 = match v0.clone() {
            Adt14::C0(_, _, h4591, _, _, _, _, _, _, _, _) => {
                h4591
            }
        };
        let a4594 = match v1.clone() {
            Adt14::C0(_, _, h4592, _, _, _, _, _, _, _, _) => {
                h4592
            }
        };
        nat_eq(a4593, a4594)
    } else {
        false
    } {
        let a4597 = match v0.clone() {
            Adt14::C0(_, _, _, h4595, _, _, _, _, _, _, _) => {
                h4595
            }
        };
        let a4598 = match v1.clone() {
            Adt14::C0(_, _, _, h4596, _, _, _, _, _, _, _) => {
                h4596
            }
        };
        nat_eq(a4597, a4598)
    } else {
        false
    } {
        let a4601 = match v0 {
            Adt14::C0(_, _, _, _, h4599, _, _, _, _, _, _) => {
                h4599
            }
        };
        let a4602 = match v1 {
            Adt14::C0(_, _, _, _, h4600, _, _, _, _, _, _) => {
                h4600
            }
        };
        nat_eq(a4601, a4602)
    } else {
        false
    }
}

pub fn f181(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a4603 = v0;
    let a4604 = 0u64;
    if if if nat_eq(a4603, a4604) {
        true
    } else {
        let a4605 = v1;
        let a4606 = 0u64;
        nat_eq(a4605, a4606)
    } {
        true
    } else {
        let a4607 = v2;
        let a4608 = 0u64;
        nat_eq(a4607, a4608)
    } {
        true
    } else {
        let a4609 = v3;
        let a4610 = 0u64;
        nat_eq(a4609, a4610)
    }
}

pub fn f182(v0: Adt14, v1: Adt14) -> bool {
    if f188(match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4611, _, _, _, _, _) => {
            h4611
        }
    }, match v1 {
        Adt14::C0(_, _, _, _, _, h4612, _, _, _, _, _) => {
            h4612
        }
    }) {
        let a4614 = match v0 {
            Adt14::C0(_, h4613, _, _, _, _, _, _, _, _, _) => {
                h4613
            }
        };
        let a4615 = 2147483647u64;
        nat_le(a4614, a4615)
    } else {
        true
    }
}

pub fn f183(v0: Adt14) -> bool {
    let a4620 = {
        let a4617 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4616) => {
                h4616
            }
        };
        let a4618 = 4u64;
        let a4619 = 0u64;
        nat_rem(a4617, a4618, a4619)
    };
    let a4621 = 0u64;
    if if nat_eq(a4620, a4621) {
        f243(match v0.clone() {
            Adt14::C0(_, h4622, _, _, _, _, _, _, _, _, _) => {
                h4622
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4623, _, _, _, _, _) => {
                h4623
            }
        })
    } else {
        false
    } {
        f86(match v0.clone() {
            Adt14::C0(_, _, h4624, _, _, _, _, _, _, _, _) => {
                h4624
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4625, _, _, _, _, _, _, _) => {
                h4625
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4626, _, _, _, _, _, _) => {
                h4626
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4627, _, _, _, _) => {
                h4627
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4628, _, _, _) => {
                h4628
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4629, _, _) => {
                h4629
            }
        }, 4u64)
    } else {
        false
    }
}

pub fn f184(v0: Adt14, v1: Adt14) -> R<bool> {
    let a4631 = match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4630, _) => {
            h4630
        }
    };
    let a4632 = 0u64;
    if nat_eq(a4631, a4632) {
        Ok(true)
    } else {
        let a4634 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, h4633, _) => {
                h4633
            }
        };
        let a4635 = 0u64;
        if nat_eq(a4634, a4635) {
            Ok(true)
        } else {
            let a4637 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4636) => {
                    h4636
                }
            };
            let a4638 = 0u64;
            if nat_eq(a4637, a4638) {
                Ok(true)
            } else {
                let a4640 = match v1.clone() {
                    Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4639) => {
                        h4639
                    }
                };
                let a4641 = 0u64;
                if nat_eq(a4640, a4641) {
                    Ok(true)
                } else {
                    let a4647 = {
                        let a4644 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4642) => {
                                h4642
                            }
                        };
                        let a4645 = match v0.clone() {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, h4643, _) => {
                                h4643
                            }
                        };
                        nat_add(a4644, a4645)?
                    };
                    let a4648 = match v1.clone() {
                        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4646) => {
                            h4646
                        }
                    };
                    if nat_le(a4647, a4648) {
                        Ok(true)
                    } else {
                        let a4654 = {
                            let a4651 = match v1.clone() {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4649) => {
                                    h4649
                                }
                            };
                            let a4652 = match v1 {
                                Adt14::C0(_, _, _, _, _, _, _, _, _, h4650, _) => {
                                    h4650
                                }
                            };
                            nat_add(a4651, a4652)?
                        };
                        let a4655 = match v0 {
                            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4653) => {
                                h4653
                            }
                        };
                        Ok(nat_le(a4654, a4655))
                    }
                }
            }
        }
    }
}

pub fn f185(v0: Adt14, v1: Adt14) -> bool {
    if if if if if if f180(v0.clone(), v1.clone()) {
        let a4658 = match v0.clone() {
            Adt14::C0(h4656, _, _, _, _, _, _, _, _, _, _) => {
                h4656
            }
        };
        let a4659 = match v1.clone() {
            Adt14::C0(h4657, _, _, _, _, _, _, _, _, _, _) => {
                h4657
            }
        };
        nat_eq(a4658, a4659)
    } else {
        false
    } {
        let a4662 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4660) => {
                h4660
            }
        };
        let a4663 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4661) => {
                h4661
            }
        };
        nat_eq(a4662, a4663)
    } else {
        false
    } {
        let a4666 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4664, _, _, _, _, _) => {
                h4664
            }
        };
        let a4667 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, h4665, _, _, _, _, _) => {
                h4665
            }
        };
        nat_eq(a4666, a4667)
    } else {
        false
    } {
        let a4670 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4668, _, _, _, _) => {
                h4668
            }
        };
        let a4671 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, h4669, _, _, _, _) => {
                h4669
            }
        };
        nat_eq(a4670, a4671)
    } else {
        false
    } {
        let a4674 = match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4672, _, _, _) => {
                h4672
            }
        };
        let a4675 = match v1.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4673, _, _, _) => {
                h4673
            }
        };
        nat_eq(a4674, a4675)
    } else {
        false
    } {
        let a4678 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4676, _, _) => {
                h4676
            }
        };
        let a4679 = match v1 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4677, _, _) => {
                h4677
            }
        };
        nat_eq(a4678, a4679)
    } else {
        false
    }
}

pub fn f186(v0: Adt14) -> R<bool> {
    if f244(match v0.clone() {
        Adt14::C0(_, h4680, _, _, _, _, _, _, _, _, _) => {
            h4680
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, h4681, _, _, _, _, _) => {
            h4681
        }
    }) {
        f87(f245(match v0.clone() {
            Adt14::C0(_, h4682, _, _, _, _, _, _, _, _, _) => {
                h4682
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, h4683, _, _, _, _, _) => {
                h4683
            }
        })?, match v0.clone() {
            Adt14::C0(_, _, h4684, _, _, _, _, _, _, _, _) => {
                h4684
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, h4685, _, _, _, _, _, _, _) => {
                h4685
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, h4686, _, _, _, _, _, _) => {
                h4686
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, h4687, _, _, _, _) => {
                h4687
            }
        }, match v0.clone() {
            Adt14::C0(_, _, _, _, _, _, _, h4688, _, _, _) => {
                h4688
            }
        }, match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4689, _, _) => {
                h4689
            }
        })
    } else {
        Ok(false)
    }
}

pub fn f187(v0: bool, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a4690 = v1;
    let a4691 = 0u64;
    if if nat_eq(a4690, a4691) {
        true
    } else {
        let a4692 = v2;
        let a4693 = 0u64;
        nat_eq(a4692, a4693)
    } {
        Ok(0u64)
    } else {
        let a4696 = f88(v1, v2, v3)?;
        let a4697 = {
            let a4694 = v2;
            let a4695 = f246(v0, v1)?;
            nat_mul(a4694, a4695)?
        };
        nat_add(a4696, a4697)
    }
}

pub fn f188(v0: u64, v1: u64) -> bool {
    let a4698 = v0;
    let a4699 = 4u64;
    if nat_eq(a4698, a4699) {
        let a4700 = v1;
        let a4701 = 4u64;
        nat_eq(a4700, a4701)
    } else {
        false
    }
}

pub fn f189(v0: u64) -> R<u64> {
    let a4702 = 6u64;
    let a4703 = v0;
    nat_add(a4702, a4703)
}

pub fn f190(v0: Adt14) -> Adt14 {
    Adt14::C0(match v0.clone() {
        Adt14::C0(h4704, _, _, _, _, _, _, _, _, _, _) => {
            h4704
        }
    }, match v0.clone() {
        Adt14::C0(_, h4705, _, _, _, _, _, _, _, _, _) => {
            h4705
        }
    }, match v0.clone() {
        Adt14::C0(_, _, h4706, _, _, _, _, _, _, _, _) => {
            h4706
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, h4707, _, _, _, _, _, _, _) => {
            h4707
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, h4708, _, _, _, _, _, _) => {
            h4708
        }
    }, 4u64, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, h4709, _, _, _, _) => {
            h4709
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, h4710, _, _, _) => {
            h4710
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, h4711, _, _) => {
            h4711
        }
    }, match v0.clone() {
        Adt14::C0(_, _, _, _, _, _, _, _, _, h4712, _) => {
            h4712
        }
    }, match v0 {
        Adt14::C0(_, _, _, _, _, _, _, _, _, _, h4713) => {
            h4713
        }
    })
}

pub fn f191(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let a4714 = v0;
    let a4715 = 2147483647u64;
    if if if nat_le(a4714, a4715) {
        let a4716 = 0u64;
        let a4717 = v3;
        if nat_lt(a4716, a4717) {
            let a4718 = v3;
            let a4719 = 2147483647u64;
            nat_le(a4718, a4719)
        } else {
            false
        }
    } else {
        false
    } {
        let a4724 = {
            let a4722 = {
                let a4720 = v2;
                let a4721 = v3;
                nat_add(a4720, a4721)?
            };
            let a4723 = 1u64;
            nat_sub(a4722, a4723)
        };
        let a4725 = 2147483647u64;
        nat_le(a4724, a4725)
    } else {
        false
    } {
        let a4730 = {
            let a4728 = {
                let a4726 = v0;
                let a4727 = v1;
                nat_mul(a4726, a4727)?
            };
            let a4729 = f247(v2, v3)?;
            nat_mul(a4728, a4729)?
        };
        let a4731 = 9223372036854775807u64;
        Ok(nat_le(a4730, a4731))
    } else {
        Ok(false)
    }
}

pub fn f192(v0: Adt14) -> R<u64> {
    let a4762 = {
        let a4754 = {
            let a4746 = {
                let a4738 = 4u64;
                let a4739 = {
                    let a4736 = {
                        let a4733 = match v0.clone() {
                            Adt14::C0(_, h4732, _, _, _, _, _, _, _, _, _) => {
                                h4732
                            }
                        };
                        let a4734 = 1u64;
                        nat_sub(a4733, a4734)
                    };
                    let a4737 = match v0.clone() {
                        Adt14::C0(_, _, _, _, _, h4735, _, _, _, _, _) => {
                            h4735
                        }
                    };
                    nat_mul(a4736, a4737)?
                };
                nat_add(a4738, a4739)?
            };
            let a4747 = {
                let a4744 = {
                    let a4741 = match v0.clone() {
                        Adt14::C0(_, _, h4740, _, _, _, _, _, _, _, _) => {
                            h4740
                        }
                    };
                    let a4742 = 1u64;
                    nat_sub(a4741, a4742)
                };
                let a4745 = match v0.clone() {
                    Adt14::C0(_, _, _, _, _, _, h4743, _, _, _, _) => {
                        h4743
                    }
                };
                nat_mul(a4744, a4745)?
            };
            nat_add(a4746, a4747)?
        };
        let a4755 = {
            let a4752 = {
                let a4749 = match v0.clone() {
                    Adt14::C0(_, _, _, h4748, _, _, _, _, _, _, _) => {
                        h4748
                    }
                };
                let a4750 = 1u64;
                nat_sub(a4749, a4750)
            };
            let a4753 = match v0.clone() {
                Adt14::C0(_, _, _, _, _, _, _, h4751, _, _, _) => {
                    h4751
                }
            };
            nat_mul(a4752, a4753)?
        };
        nat_add(a4754, a4755)?
    };
    let a4763 = {
        let a4760 = {
            let a4757 = match v0.clone() {
                Adt14::C0(_, _, _, _, h4756, _, _, _, _, _, _) => {
                    h4756
                }
            };
            let a4758 = 1u64;
            nat_sub(a4757, a4758)
        };
        let a4761 = match v0 {
            Adt14::C0(_, _, _, _, _, _, _, _, h4759, _, _) => {
                h4759
            }
        };
        nat_mul(a4760, a4761)?
    };
    nat_add(a4762, a4763)
}

pub fn f193(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<u64> {
    let a4764 = v0;
    let a4765 = 0u64;
    if if if if nat_eq(a4764, a4765) {
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
    } {
        Ok(0u64)
    } else {
        let a4772 = f248(v0, v1, v2, v3, v5)?;
        let a4773 = f249(v0, v1, v2, v3, v4)?;
        nat_add(a4772, a4773)
    }
}

pub fn f194(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a4774 = v0;
    let a4775 = 0u64;
    if if nat_eq(a4774, a4775) {
        true
    } else {
        let a4776 = v1;
        let a4777 = 0u64;
        nat_eq(a4776, a4777)
    } {
        Ok(0u64)
    } else {
        let a4780 = f88(v0, v1, v2)?;
        let a4781 = {
            let a4778 = v1;
            let a4779 = f250(v0)?;
            nat_mul(a4778, a4779)?
        };
        nat_add(a4780, a4781)
    }
}

pub fn f195(v0: u64, v1: u64, v2: u64) -> bool {
    let a4782 = v0;
    let a4783 = 1u64;
    if nat_le(a4782, a4783) {
        true
    } else {
        let a4787 = {
            let a4784 = v1;
            let a4785 = v2;
            let a4786 = 1u64;
            nat_rem(a4784, a4785, a4786)
        };
        let a4788 = 0u64;
        nat_eq(a4787, a4788)
    }
}

pub fn f196(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> R<bool> {
    let a4789 = v1;
    let a4790 = 1u64;
    if if if nat_le(a4789, a4790) {
        true
    } else {
        let a4791 = v0;
        let a4792 = v2;
        nat_le(a4791, a4792)
    } {
        let a4793 = v3;
        let a4794 = 1u64;
        if nat_le(a4793, a4794) {
            true
        } else {
            let a4801 = {
                let a4799 = v0;
                let a4800 = {
                    let a4797 = {
                        let a4795 = v1;
                        let a4796 = 1u64;
                        nat_sub(a4795, a4796)
                    };
                    let a4798 = v2;
                    nat_mul(a4797, a4798)?
                };
                nat_add(a4799, a4800)?
            };
            let a4802 = v4;
            nat_le(a4801, a4802)
        }
    } else {
        false
    } {
        let a4803 = v5;
        let a4804 = 1u64;
        if nat_le(a4803, a4804) {
            Ok(true)
        } else {
            let a4817 = {
                let a4815 = {
                    let a4809 = v0;
                    let a4810 = {
                        let a4807 = {
                            let a4805 = v1;
                            let a4806 = 1u64;
                            nat_sub(a4805, a4806)
                        };
                        let a4808 = v2;
                        nat_mul(a4807, a4808)?
                    };
                    nat_add(a4809, a4810)?
                };
                let a4816 = {
                    let a4813 = {
                        let a4811 = v3;
                        let a4812 = 1u64;
                        nat_sub(a4811, a4812)
                    };
                    let a4814 = v4;
                    nat_mul(a4813, a4814)?
                };
                nat_add(a4815, a4816)?
            };
            let a4818 = v6;
            Ok(nat_le(a4817, a4818))
        }
    } else {
        Ok(false)
    }
}

pub fn f197(v0: u64) -> Adt18 {
    let a4819 = v0;
    let a4820 = 0u64;
    if nat_eq(a4819, a4820) {
        Adt18::C1(Adt19::C0(0u64, 1u64, 4u64))
    } else {
        let a4821 = v0;
        let a4822 = 1u64;
        if nat_eq(a4821, a4822) {
            Adt18::C1(Adt19::C0(1u64, 1u64, 2u64))
        } else {
            let a4823 = v0;
            let a4824 = 2u64;
            if nat_eq(a4823, a4824) {
                Adt18::C1(Adt19::C0(2u64, 32u64, 18u64))
            } else {
                let a4825 = v0;
                let a4826 = 3u64;
                if nat_eq(a4825, a4826) {
                    Adt18::C1(Adt19::C0(3u64, 32u64, 20u64))
                } else {
                    let a4827 = v0;
                    let a4828 = 6u64;
                    if nat_eq(a4827, a4828) {
                        Adt18::C1(Adt19::C0(6u64, 32u64, 22u64))
                    } else {
                        let a4829 = v0;
                        let a4830 = 7u64;
                        if nat_eq(a4829, a4830) {
                            Adt18::C1(Adt19::C0(7u64, 32u64, 24u64))
                        } else {
                            let a4831 = v0;
                            let a4832 = 8u64;
                            if nat_eq(a4831, a4832) {
                                Adt18::C1(Adt19::C0(8u64, 32u64, 34u64))
                            } else {
                                let a4833 = v0;
                                let a4834 = 9u64;
                                if nat_eq(a4833, a4834) {
                                    Adt18::C1(Adt19::C0(9u64, 32u64, 36u64))
                                } else {
                                    let a4835 = v0;
                                    let a4836 = 10u64;
                                    if nat_eq(a4835, a4836) {
                                        Adt18::C1(Adt19::C0(10u64, 256u64, 84u64))
                                    } else {
                                        let a4837 = v0;
                                        let a4838 = 11u64;
                                        if nat_eq(a4837, a4838) {
                                            Adt18::C1(Adt19::C0(11u64, 256u64, 110u64))
                                        } else {
                                            let a4839 = v0;
                                            let a4840 = 12u64;
                                            if nat_eq(a4839, a4840) {
                                                Adt18::C1(Adt19::C0(12u64, 256u64, 144u64))
                                            } else {
                                                let a4841 = v0;
                                                let a4842 = 13u64;
                                                if nat_eq(a4841, a4842) {
                                                    Adt18::C1(Adt19::C0(13u64, 256u64, 176u64))
                                                } else {
                                                    let a4843 = v0;
                                                    let a4844 = 14u64;
                                                    if nat_eq(a4843, a4844) {
                                                        Adt18::C1(Adt19::C0(14u64, 256u64, 210u64))
                                                    } else {
                                                        let a4845 = v0;
                                                        let a4846 = 15u64;
                                                        if nat_eq(a4845, a4846) {
                                                            Adt18::C1(Adt19::C0(15u64, 256u64, 292u64))
                                                        } else {
                                                            let a4847 = v0;
                                                            let a4848 = 16u64;
                                                            if nat_eq(a4847, a4848) {
                                                                Adt18::C1(Adt19::C0(16u64, 256u64, 66u64))
                                                            } else {
                                                                let a4849 = v0;
                                                                let a4850 = 17u64;
                                                                if nat_eq(a4849, a4850) {
                                                                    Adt18::C1(Adt19::C0(17u64, 256u64, 74u64))
                                                                } else {
                                                                    let a4851 = v0;
                                                                    let a4852 = 18u64;
                                                                    if nat_eq(a4851, a4852) {
                                                                        Adt18::C1(Adt19::C0(18u64, 256u64, 98u64))
                                                                    } else {
                                                                        let a4853 = v0;
                                                                        let a4854 = 19u64;
                                                                        if nat_eq(a4853, a4854) {
                                                                            Adt18::C1(Adt19::C0(19u64, 256u64, 50u64))
                                                                        } else {
                                                                            let a4855 = v0;
                                                                            let a4856 = 20u64;
                                                                            if nat_eq(a4855, a4856) {
                                                                                Adt18::C1(Adt19::C0(20u64, 32u64, 18u64))
                                                                            } else {
                                                                                let a4857 = v0;
                                                                                let a4858 = 21u64;
                                                                                if nat_eq(a4857, a4858) {
                                                                                    Adt18::C1(Adt19::C0(21u64, 256u64, 110u64))
                                                                                } else {
                                                                                    let a4859 = v0;
                                                                                    let a4860 = 22u64;
                                                                                    if nat_eq(a4859, a4860) {
                                                                                        Adt18::C1(Adt19::C0(22u64, 256u64, 82u64))
                                                                                    } else {
                                                                                        let a4861 = v0;
                                                                                        let a4862 = 23u64;
                                                                                        if nat_eq(a4861, a4862) {
                                                                                            Adt18::C1(Adt19::C0(23u64, 256u64, 136u64))
                                                                                        } else {
                                                                                            let a4863 = v0;
                                                                                            let a4864 = 24u64;
                                                                                            if nat_eq(a4863, a4864) {
                                                                                                Adt18::C1(Adt19::C0(24u64, 1u64, 1u64))
                                                                                            } else {
                                                                                                let a4865 = v0;
                                                                                                let a4866 = 25u64;
                                                                                                if nat_eq(a4865, a4866) {
                                                                                                    Adt18::C1(Adt19::C0(25u64, 1u64, 2u64))
                                                                                                } else {
                                                                                                    let a4867 = v0;
                                                                                                    let a4868 = 26u64;
                                                                                                    if nat_eq(a4867, a4868) {
                                                                                                        Adt18::C1(Adt19::C0(26u64, 1u64, 4u64))
                                                                                                    } else {
                                                                                                        let a4869 = v0;
                                                                                                        let a4870 = 27u64;
                                                                                                        if nat_eq(a4869, a4870) {
                                                                                                            Adt18::C1(Adt19::C0(27u64, 1u64, 8u64))
                                                                                                        } else {
                                                                                                            let a4871 = v0;
                                                                                                            let a4872 = 28u64;
                                                                                                            if nat_eq(a4871, a4872) {
                                                                                                                Adt18::C1(Adt19::C0(28u64, 1u64, 8u64))
                                                                                                            } else {
                                                                                                                let a4873 = v0;
                                                                                                                let a4874 = 29u64;
                                                                                                                if nat_eq(a4873, a4874) {
                                                                                                                    Adt18::C1(Adt19::C0(29u64, 256u64, 56u64))
                                                                                                                } else {
                                                                                                                    let a4875 = v0;
                                                                                                                    let a4876 = 30u64;
                                                                                                                    if nat_eq(a4875, a4876) {
                                                                                                                        Adt18::C1(Adt19::C0(30u64, 1u64, 2u64))
                                                                                                                    } else {
                                                                                                                        let a4877 = v0;
                                                                                                                        let a4878 = 34u64;
                                                                                                                        if nat_eq(a4877, a4878) {
                                                                                                                            Adt18::C1(Adt19::C0(34u64, 256u64, 54u64))
                                                                                                                        } else {
                                                                                                                            let a4879 = v0;
                                                                                                                            let a4880 = 35u64;
                                                                                                                            if nat_eq(a4879, a4880) {
                                                                                                                                Adt18::C1(Adt19::C0(35u64, 256u64, 66u64))
                                                                                                                            } else {
                                                                                                                                let a4881 = v0;
                                                                                                                                let a4882 = 39u64;
                                                                                                                                if nat_eq(a4881, a4882) {
                                                                                                                                    Adt18::C1(Adt19::C0(39u64, 32u64, 17u64))
                                                                                                                                } else {
                                                                                                                                    let a4883 = v0;
                                                                                                                                    let a4884 = 40u64;
                                                                                                                                    if nat_eq(a4883, a4884) {
                                                                                                                                        Adt18::C1(Adt19::C0(40u64, 64u64, 36u64))
                                                                                                                                    } else {
                                                                                                                                        let a4885 = v0;
                                                                                                                                        let a4886 = 41u64;
                                                                                                                                        if nat_eq(a4885, a4886) {
                                                                                                                                            Adt18::C1(Adt19::C0(41u64, 128u64, 18u64))
                                                                                                                                        } else {
                                                                                                                                            let a4887 = v0;
                                                                                                                                            let a4888 = 42u64;
                                                                                                                                            if nat_eq(a4887, a4888) {
                                                                                                                                                Adt18::C1(Adt19::C0(42u64, 64u64, 18u64))
                                                                                                                                            } else {
                                                                                                                                                let a4889 = v0;
                                                                                                                                                let a4890 = 43u64;
                                                                                                                                                if nat_eq(a4889, a4890) {
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

pub fn f198(v0: Adt2, v1: u64, v2: bool, v3: bool) -> R<bool> {
    if v2 {
        if v3 {
            let a4892 = match v0.clone() {
                Adt2::C0(_, _, _, _, h4891, _, _, _, _, _, _) => {
                    h4891
                }
            };
            let a4893 = 1u64;
            if if if if if nat_le(a4892, a4893) {
                true
            } else {
                let a4895 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h4894, _, _) => {
                        h4894
                    }
                };
                let a4896 = v1;
                nat_eq(a4895, a4896)
            } {
                let a4898 = match v0.clone() {
                    Adt2::C0(_, h4897, _, _, _, _, _, _, _, _, _) => {
                        h4897
                    }
                };
                let a4899 = 1u64;
                if nat_le(a4898, a4899) {
                    true
                } else {
                    let a4904 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4900, _, _, _, _, _) => {
                            h4900
                        }
                    };
                    let a4905 = {
                        let a4902 = v1;
                        let a4903 = match v0.clone() {
                            Adt2::C0(_, _, _, _, h4901, _, _, _, _, _, _) => {
                                h4901
                            }
                        };
                        nat_mul(a4902, a4903)?
                    };
                    nat_eq(a4904, a4905)
                }
            } else {
                false
            } {
                let a4907 = match v0.clone() {
                    Adt2::C0(_, _, h4906, _, _, _, _, _, _, _, _) => {
                        h4906
                    }
                };
                let a4908 = 1u64;
                if nat_le(a4907, a4908) {
                    true
                } else {
                    let a4916 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4909, _, _, _, _) => {
                            h4909
                        }
                    };
                    let a4917 = {
                        let a4914 = {
                            let a4911 = v1;
                            let a4912 = match v0.clone() {
                                Adt2::C0(_, _, _, _, h4910, _, _, _, _, _, _) => {
                                    h4910
                                }
                            };
                            nat_mul(a4911, a4912)?
                        };
                        let a4915 = match v0.clone() {
                            Adt2::C0(_, h4913, _, _, _, _, _, _, _, _, _) => {
                                h4913
                            }
                        };
                        nat_mul(a4914, a4915)?
                    };
                    nat_eq(a4916, a4917)
                }
            } else {
                false
            } {
                let a4919 = match v0.clone() {
                    Adt2::C0(_, _, _, h4918, _, _, _, _, _, _, _) => {
                        h4918
                    }
                };
                let a4920 = 1u64;
                if nat_le(a4919, a4920) {
                    true
                } else {
                    let a4931 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h4921, _, _, _) => {
                            h4921
                        }
                    };
                    let a4932 = {
                        let a4929 = {
                            let a4926 = {
                                let a4923 = v1;
                                let a4924 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4922, _, _, _, _, _, _) => {
                                        h4922
                                    }
                                };
                                nat_mul(a4923, a4924)?
                            };
                            let a4927 = match v0.clone() {
                                Adt2::C0(_, h4925, _, _, _, _, _, _, _, _, _) => {
                                    h4925
                                }
                            };
                            nat_mul(a4926, a4927)?
                        };
                        let a4930 = match v0.clone() {
                            Adt2::C0(_, _, h4928, _, _, _, _, _, _, _, _) => {
                                h4928
                            }
                        };
                        nat_mul(a4929, a4930)?
                    };
                    nat_eq(a4931, a4932)
                }
            } else {
                false
            } {
                let a4946 = {
                    let a4943 = {
                        let a4940 = {
                            let a4937 = {
                                let a4934 = v1;
                                let a4935 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, h4933, _, _, _, _, _, _) => {
                                        h4933
                                    }
                                };
                                nat_mul(a4934, a4935)?
                            };
                            let a4938 = match v0.clone() {
                                Adt2::C0(_, h4936, _, _, _, _, _, _, _, _, _) => {
                                    h4936
                                }
                            };
                            nat_mul(a4937, a4938)?
                        };
                        let a4941 = match v0.clone() {
                            Adt2::C0(_, _, h4939, _, _, _, _, _, _, _, _) => {
                                h4939
                            }
                        };
                        nat_mul(a4940, a4941)?
                    };
                    let a4944 = match v0.clone() {
                        Adt2::C0(_, _, _, h4942, _, _, _, _, _, _, _) => {
                            h4942
                        }
                    };
                    nat_mul(a4943, a4944)?
                };
                let a4947 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h4945, _) => {
                        h4945
                    }
                };
                Ok(nat_le(a4946, a4947))
            } else {
                Ok(false)
            }
        } else {
            let a4949 = match v0.clone() {
                Adt2::C0(_, _, _, h4948, _, _, _, _, _, _, _) => {
                    h4948
                }
            };
            let a4950 = 1u64;
            if if if if if nat_le(a4949, a4950) {
                true
            } else {
                let a4952 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h4951, _, _, _) => {
                        h4951
                    }
                };
                let a4953 = v1;
                nat_eq(a4952, a4953)
            } {
                let a4955 = match v0.clone() {
                    Adt2::C0(_, h4954, _, _, _, _, _, _, _, _, _) => {
                        h4954
                    }
                };
                let a4956 = 1u64;
                if nat_le(a4955, a4956) {
                    true
                } else {
                    let a4961 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h4957, _, _, _, _, _) => {
                            h4957
                        }
                    };
                    let a4962 = {
                        let a4959 = v1;
                        let a4960 = match v0.clone() {
                            Adt2::C0(_, _, _, h4958, _, _, _, _, _, _, _) => {
                                h4958
                            }
                        };
                        nat_mul(a4959, a4960)?
                    };
                    nat_eq(a4961, a4962)
                }
            } else {
                false
            } {
                let a4964 = match v0.clone() {
                    Adt2::C0(_, _, h4963, _, _, _, _, _, _, _, _) => {
                        h4963
                    }
                };
                let a4965 = 1u64;
                if nat_le(a4964, a4965) {
                    true
                } else {
                    let a4973 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h4966, _, _, _, _) => {
                            h4966
                        }
                    };
                    let a4974 = {
                        let a4971 = {
                            let a4968 = v1;
                            let a4969 = match v0.clone() {
                                Adt2::C0(_, _, _, h4967, _, _, _, _, _, _, _) => {
                                    h4967
                                }
                            };
                            nat_mul(a4968, a4969)?
                        };
                        let a4972 = match v0.clone() {
                            Adt2::C0(_, h4970, _, _, _, _, _, _, _, _, _) => {
                                h4970
                            }
                        };
                        nat_mul(a4971, a4972)?
                    };
                    nat_eq(a4973, a4974)
                }
            } else {
                false
            } {
                let a4976 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h4975, _, _, _, _, _, _) => {
                        h4975
                    }
                };
                let a4977 = 1u64;
                if nat_le(a4976, a4977) {
                    true
                } else {
                    let a4988 = match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h4978, _, _) => {
                            h4978
                        }
                    };
                    let a4989 = {
                        let a4986 = {
                            let a4983 = {
                                let a4980 = v1;
                                let a4981 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4979, _, _, _, _, _, _, _) => {
                                        h4979
                                    }
                                };
                                nat_mul(a4980, a4981)?
                            };
                            let a4984 = match v0.clone() {
                                Adt2::C0(_, h4982, _, _, _, _, _, _, _, _, _) => {
                                    h4982
                                }
                            };
                            nat_mul(a4983, a4984)?
                        };
                        let a4987 = match v0.clone() {
                            Adt2::C0(_, _, h4985, _, _, _, _, _, _, _, _) => {
                                h4985
                            }
                        };
                        nat_mul(a4986, a4987)?
                    };
                    nat_eq(a4988, a4989)
                }
            } else {
                false
            } {
                let a5003 = {
                    let a5000 = {
                        let a4997 = {
                            let a4994 = {
                                let a4991 = v1;
                                let a4992 = match v0.clone() {
                                    Adt2::C0(_, _, _, h4990, _, _, _, _, _, _, _) => {
                                        h4990
                                    }
                                };
                                nat_mul(a4991, a4992)?
                            };
                            let a4995 = match v0.clone() {
                                Adt2::C0(_, h4993, _, _, _, _, _, _, _, _, _) => {
                                    h4993
                                }
                            };
                            nat_mul(a4994, a4995)?
                        };
                        let a4998 = match v0.clone() {
                            Adt2::C0(_, _, h4996, _, _, _, _, _, _, _, _) => {
                                h4996
                            }
                        };
                        nat_mul(a4997, a4998)?
                    };
                    let a5001 = match v0.clone() {
                        Adt2::C0(_, _, _, _, h4999, _, _, _, _, _, _) => {
                            h4999
                        }
                    };
                    nat_mul(a5000, a5001)?
                };
                let a5004 = match v0 {
                    Adt2::C0(_, _, _, _, _, _, _, _, _, h5002, _) => {
                        h5002
                    }
                };
                Ok(nat_le(a5003, a5004))
            } else {
                Ok(false)
            }
        }
    } else {
        let a5006 = match v0.clone() {
            Adt2::C0(_, h5005, _, _, _, _, _, _, _, _, _) => {
                h5005
            }
        };
        let a5007 = 1u64;
        if if if if if nat_le(a5006, a5007) {
            true
        } else {
            let a5009 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, h5008, _, _, _, _, _) => {
                    h5008
                }
            };
            let a5010 = v1;
            nat_eq(a5009, a5010)
        } {
            let a5012 = match v0.clone() {
                Adt2::C0(_, _, h5011, _, _, _, _, _, _, _, _) => {
                    h5011
                }
            };
            let a5013 = 1u64;
            if nat_le(a5012, a5013) {
                true
            } else {
                let a5018 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, h5014, _, _, _, _) => {
                        h5014
                    }
                };
                let a5019 = {
                    let a5016 = v1;
                    let a5017 = match v0.clone() {
                        Adt2::C0(_, h5015, _, _, _, _, _, _, _, _, _) => {
                            h5015
                        }
                    };
                    nat_mul(a5016, a5017)?
                };
                nat_eq(a5018, a5019)
            }
        } else {
            false
        } {
            let a5021 = match v0.clone() {
                Adt2::C0(_, _, _, h5020, _, _, _, _, _, _, _) => {
                    h5020
                }
            };
            let a5022 = 1u64;
            if nat_le(a5021, a5022) {
                true
            } else {
                let a5030 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, h5023, _, _, _) => {
                        h5023
                    }
                };
                let a5031 = {
                    let a5028 = {
                        let a5025 = v1;
                        let a5026 = match v0.clone() {
                            Adt2::C0(_, h5024, _, _, _, _, _, _, _, _, _) => {
                                h5024
                            }
                        };
                        nat_mul(a5025, a5026)?
                    };
                    let a5029 = match v0.clone() {
                        Adt2::C0(_, _, h5027, _, _, _, _, _, _, _, _) => {
                            h5027
                        }
                    };
                    nat_mul(a5028, a5029)?
                };
                nat_eq(a5030, a5031)
            }
        } else {
            false
        } {
            let a5033 = match v0.clone() {
                Adt2::C0(_, _, _, _, h5032, _, _, _, _, _, _) => {
                    h5032
                }
            };
            let a5034 = 1u64;
            if nat_le(a5033, a5034) {
                true
            } else {
                let a5045 = match v0.clone() {
                    Adt2::C0(_, _, _, _, _, _, _, _, h5035, _, _) => {
                        h5035
                    }
                };
                let a5046 = {
                    let a5043 = {
                        let a5040 = {
                            let a5037 = v1;
                            let a5038 = match v0.clone() {
                                Adt2::C0(_, h5036, _, _, _, _, _, _, _, _, _) => {
                                    h5036
                                }
                            };
                            nat_mul(a5037, a5038)?
                        };
                        let a5041 = match v0.clone() {
                            Adt2::C0(_, _, h5039, _, _, _, _, _, _, _, _) => {
                                h5039
                            }
                        };
                        nat_mul(a5040, a5041)?
                    };
                    let a5044 = match v0.clone() {
                        Adt2::C0(_, _, _, h5042, _, _, _, _, _, _, _) => {
                            h5042
                        }
                    };
                    nat_mul(a5043, a5044)?
                };
                nat_eq(a5045, a5046)
            }
        } else {
            false
        } {
            let a5060 = {
                let a5057 = {
                    let a5054 = {
                        let a5051 = {
                            let a5048 = v1;
                            let a5049 = match v0.clone() {
                                Adt2::C0(_, h5047, _, _, _, _, _, _, _, _, _) => {
                                    h5047
                                }
                            };
                            nat_mul(a5048, a5049)?
                        };
                        let a5052 = match v0.clone() {
                            Adt2::C0(_, _, h5050, _, _, _, _, _, _, _, _) => {
                                h5050
                            }
                        };
                        nat_mul(a5051, a5052)?
                    };
                    let a5055 = match v0.clone() {
                        Adt2::C0(_, _, _, h5053, _, _, _, _, _, _, _) => {
                            h5053
                        }
                    };
                    nat_mul(a5054, a5055)?
                };
                let a5058 = match v0.clone() {
                    Adt2::C0(_, _, _, _, h5056, _, _, _, _, _, _) => {
                        h5056
                    }
                };
                nat_mul(a5057, a5058)?
            };
            let a5061 = match v0 {
                Adt2::C0(_, _, _, _, _, _, _, _, _, h5059, _) => {
                    h5059
                }
            };
            Ok(nat_le(a5060, a5061))
        } else {
            Ok(false)
        }
    }
}

pub fn f199(v0: Adt2, v1: u64) -> R<bool> {
    let a5063 = match v0.clone() {
        Adt2::C0(h5062, _, _, _, _, _, _, _, _, _, _) => {
            h5062
        }
    };
    let a5064 = 0u64;
    if if nat_eq(a5063, a5064) {
        f30(v0.clone())?
    } else {
        false
    } {
        let a5065 = v1;
        let a5066 = 1u64;
        if nat_le(a5065, a5066) {
            Ok(true)
        } else {
            let a5071 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5067, _, _, _, _) => {
                    h5067
                }
            };
            let a5072 = {
                let a5069 = 4u64;
                let a5070 = match v0 {
                    Adt2::C0(_, h5068, _, _, _, _, _, _, _, _, _) => {
                        h5068
                    }
                };
                nat_mul(a5069, a5070)?
            };
            Ok(nat_eq(a5071, a5072))
        }
    } else {
        Ok(false)
    }
}

pub fn f200(v0: Adt2, v1: u64) -> R<bool> {
    let a5074 = match v0.clone() {
        Adt2::C0(_, _, _, h5073, _, _, _, _, _, _, _) => {
            h5073
        }
    };
    let a5075 = 1u64;
    if if if nat_eq(a5074, a5075) {
        let a5077 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5076, _, _, _, _, _, _) => {
                h5076
            }
        };
        let a5078 = 1u64;
        nat_eq(a5077, a5078)
    } else {
        false
    } {
        let a5080 = match v0.clone() {
            Adt2::C0(_, _, _, _, _, h5079, _, _, _, _, _) => {
                h5079
            }
        };
        let a5081 = v1;
        nat_eq(a5080, a5081)
    } else {
        false
    } {
        let a5083 = match v0.clone() {
            Adt2::C0(_, _, h5082, _, _, _, _, _, _, _, _) => {
                h5082
            }
        };
        let a5084 = 1u64;
        if nat_eq(a5083, a5084) {
            Ok(true)
        } else {
            let a5089 = match v0.clone() {
                Adt2::C0(_, _, _, _, _, _, h5085, _, _, _, _) => {
                    h5085
                }
            };
            let a5090 = {
                let a5087 = match v0 {
                    Adt2::C0(_, h5086, _, _, _, _, _, _, _, _, _) => {
                        h5086
                    }
                };
                let a5088 = v1;
                nat_mul(a5087, a5088)?
            };
            Ok(nat_eq(a5089, a5090))
        }
    } else {
        Ok(false)
    }
}

pub fn f201(v0: Adt6, v1: u64, v2: u64) -> R<bool> {
    let v3: u64 = {
        let a5094 = {
            let a5091 = v2;
            let a5092 = 1u64;
            nat_sub(a5091, a5092)
        };
        let a5095 = match v0.clone() {
            Adt6::C0(_, h5093, _, _) => {
                h5093
            }
        };
        nat_mul(a5094, a5095)?
    };
    let a5097 = 0u64;
    let a5098 = match v0.clone() {
        Adt6::C0(h5096, _, _, _) => {
            h5096
        }
    };
    if if if if if if if if nat_lt(a5097, a5098) {
        let a5100 = match v0.clone() {
            Adt6::C0(h5099, _, _, _) => {
                h5099
            }
        };
        let a5101 = 2147483648u64;
        nat_lt(a5100, a5101)
    } else {
        false
    } {
        let a5103 = 0u64;
        let a5104 = match v0.clone() {
            Adt6::C0(_, h5102, _, _) => {
                h5102
            }
        };
        nat_lt(a5103, a5104)
    } else {
        false
    } {
        let a5106 = match v0.clone() {
            Adt6::C0(_, h5105, _, _) => {
                h5105
            }
        };
        let a5107 = 2147483648u64;
        nat_lt(a5106, a5107)
    } else {
        false
    } {
        let a5108 = v1;
        let a5109 = 2147483648u64;
        nat_lt(a5108, a5109)
    } else {
        false
    } {
        let a5110 = 0u64;
        let a5111 = v2;
        nat_lt(a5110, a5111)
    } else {
        false
    } {
        let a5112 = v3;
        let a5113 = 2147483648u64;
        nat_lt(a5112, a5113)
    } else {
        false
    } {
        let a5125 = {
            let a5120 = {
                let a5117 = v3;
                let a5118 = {
                    let a5115 = match v0.clone() {
                        Adt6::C0(h5114, _, _, _) => {
                            h5114
                        }
                    };
                    let a5116 = 1u64;
                    nat_sub(a5115, a5116)
                };
                nat_add(a5117, a5118)?
            };
            let a5121 = f127(match v0.clone() {
                Adt6::C0(_, _, h5119, _) => {
                    h5119
                }
            });
            nat_add(a5120, a5121)?
        };
        let a5126 = {
            let a5123 = 2147483647u64;
            let a5124 = f128(match v0.clone() {
                Adt6::C0(_, _, h5122, _) => {
                    h5122
                }
            });
            nat_add(a5123, a5124)?
        };
        nat_le(a5125, a5126)
    } else {
        false
    } {
        let a5128 = match v0 {
            Adt6::C0(_, _, h5127, _) => {
                h5127
            }
        };
        let a5129 = 2147483648u64;
        Ok(!nat_eq(a5128, a5129))
    } else {
        Ok(false)
    }
}

pub fn f202(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if f251(match v0.clone() {
        Adt2::C0(h5130, _, _, _, _, _, _, _, _, _, _) => {
            h5130
        }
    }, match v0.clone() {
        Adt2::C0(_, h5131, _, _, _, _, _, _, _, _, _) => {
            h5131
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5132, _, _, _, _, _, _, _, _) => {
            h5132
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5133, _, _, _, _, _, _, _) => {
            h5133
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5134, _, _, _, _, _, _) => {
            h5134
        }
    }, match v1.clone() {
        Adt2::C0(h5135, _, _, _, _, _, _, _, _, _, _) => {
            h5135
        }
    }, match v1.clone() {
        Adt2::C0(_, h5136, _, _, _, _, _, _, _, _, _) => {
            h5136
        }
    }, match v1.clone() {
        Adt2::C0(_, _, h5137, _, _, _, _, _, _, _, _) => {
            h5137
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, h5138, _, _, _, _, _, _, _) => {
            h5138
        }
    }, match v1.clone() {
        Adt2::C0(_, _, _, _, h5139, _, _, _, _, _, _) => {
            h5139
        }
    }, match v2.clone() {
        Adt2::C0(h5140, _, _, _, _, _, _, _, _, _, _) => {
            h5140
        }
    }, match v2.clone() {
        Adt2::C0(_, h5141, _, _, _, _, _, _, _, _, _) => {
            h5141
        }
    }, match v2.clone() {
        Adt2::C0(_, _, h5142, _, _, _, _, _, _, _, _) => {
            h5142
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, h5143, _, _, _, _, _, _, _) => {
            h5143
        }
    }, match v2.clone() {
        Adt2::C0(_, _, _, _, h5144, _, _, _, _, _, _) => {
            h5144
        }
    }) {
        let m5146 = f197(match v0.clone() {
            Adt2::C0(h5145, _, _, _, _, _, _, _, _, _, _) => {
                h5145
            }
        });
        match m5146 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v3) => {
                let a5152 = {
                    let a5149 = match v0.clone() {
                        Adt2::C0(_, h5147, _, _, _, _, _, _, _, _, _) => {
                            h5147
                        }
                    };
                    let a5150 = match v3.clone() {
                        Adt19::C0(_, h5148, _) => {
                            h5148
                        }
                    };
                    let a5151 = 0u64;
                    nat_rem(a5149, a5150, a5151)
                };
                let a5153 = 0u64;
                if if nat_eq(a5152, a5153) {
                    if f252(match v0.clone() {
                        Adt2::C0(_, h5154, _, _, _, _, _, _, _, _, _) => {
                            h5154
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5155, _, _, _, _, _, _, _, _, _) => {
                            h5155
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5156, _, _, _, _, _, _, _, _) => {
                            h5156
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5157, _, _, _, _, _, _, _) => {
                            h5157
                        }
                    }) {
                        true
                    } else {
                        let a5159 = 4u64;
                        let a5160 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5158, _, _, _, _, _) => {
                                h5158
                            }
                        };
                        if if nat_le(a5159, a5160) {
                            let a5162 = match v3.clone() {
                                Adt19::C0(_, h5161, _) => {
                                    h5161
                                }
                            };
                            let a5163 = 1u64;
                            if nat_eq(a5162, a5163) {
                                true
                            } else {
                                let a5166 = match v3.clone() {
                                    Adt19::C0(_, _, h5164) => {
                                        h5164
                                    }
                                };
                                let a5167 = match v0.clone() {
                                    Adt2::C0(_, _, _, _, _, h5165, _, _, _, _, _) => {
                                        h5165
                                    }
                                };
                                nat_le(a5166, a5167)
                            }
                        } else {
                            false
                        } {
                            if f20(v0.clone(), v2.clone())? {
                                f20(v1.clone(), v2.clone())?
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
                    let v4: Adt3 = f23(v0.clone(), match v3 {
                        Adt19::C0(_, h5168, _) => {
                            h5168
                        }
                    });
                    Ok(Adt10::C4(if f252(match v0.clone() {
                        Adt2::C0(_, h5169, _, _, _, _, _, _, _, _, _) => {
                            h5169
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5170, _, _, _, _, _, _, _, _, _) => {
                            h5170
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, h5171, _, _, _, _, _, _, _, _) => {
                            h5171
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, _, _, h5172, _, _, _, _, _, _, _) => {
                            h5172
                        }
                    }) {
                        0u64
                    } else {
                        f161(v4.clone(), f23(v1.clone(), 1u64))?
                    }, 4u64, v4, f23(v2, 1u64), Adt4::C20(match v0 {
                        Adt2::C0(h5173, _, _, _, _, _, _, _, _, _, _) => {
                            h5173
                        }
                    }, f23(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f203(v0: Adt2, v1: Adt2, v2: Adt2) -> R<Adt10> {
    if if if if if if if if if if if if if f204(match v0.clone() {
        Adt2::C0(h5174, _, _, _, _, _, _, _, _, _, _) => {
            h5174
        }
    }) {
        let a5180 = {
            let a5177 = match v0.clone() {
                Adt2::C0(_, h5175, _, _, _, _, _, _, _, _, _) => {
                    h5175
                }
            };
            let a5178 = f234(match v0.clone() {
                Adt2::C0(h5176, _, _, _, _, _, _, _, _, _, _) => {
                    h5176
                }
            });
            let a5179 = 1u64;
            nat_rem(a5177, a5178, a5179)
        };
        let a5181 = 0u64;
        nat_eq(a5180, a5181)
    } else {
        false
    } {
        let a5183 = match v1.clone() {
            Adt2::C0(h5182, _, _, _, _, _, _, _, _, _, _) => {
                h5182
            }
        };
        let a5184 = 26u64;
        nat_eq(a5183, a5184)
    } else {
        false
    } {
        let a5186 = match v2.clone() {
            Adt2::C0(h5185, _, _, _, _, _, _, _, _, _, _) => {
                h5185
            }
        };
        let a5187 = 0u64;
        nat_eq(a5186, a5187)
    } else {
        false
    } {
        let a5190 = match v0.clone() {
            Adt2::C0(_, h5188, _, _, _, _, _, _, _, _, _) => {
                h5188
            }
        };
        let a5191 = match v2.clone() {
            Adt2::C0(_, h5189, _, _, _, _, _, _, _, _, _) => {
                h5189
            }
        };
        nat_eq(a5190, a5191)
    } else {
        false
    } {
        let a5194 = match v1.clone() {
            Adt2::C0(_, h5192, _, _, _, _, _, _, _, _, _) => {
                h5192
            }
        };
        let a5195 = match v0.clone() {
            Adt2::C0(_, _, h5193, _, _, _, _, _, _, _, _) => {
                h5193
            }
        };
        nat_le(a5194, a5195)
    } else {
        false
    } {
        let a5197 = match v0.clone() {
            Adt2::C0(_, _, _, h5196, _, _, _, _, _, _, _) => {
                h5196
            }
        };
        let a5198 = 1u64;
        nat_eq(a5197, a5198)
    } else {
        false
    } {
        let a5200 = match v0.clone() {
            Adt2::C0(_, _, _, _, h5199, _, _, _, _, _, _) => {
                h5199
            }
        };
        let a5201 = 1u64;
        nat_eq(a5200, a5201)
    } else {
        false
    } {
        let a5203 = match v2.clone() {
            Adt2::C0(_, _, _, h5202, _, _, _, _, _, _, _) => {
                h5202
            }
        };
        let a5204 = 1u64;
        nat_eq(a5203, a5204)
    } else {
        false
    } {
        let a5206 = match v2.clone() {
            Adt2::C0(_, _, _, _, h5205, _, _, _, _, _, _) => {
                h5205
            }
        };
        let a5207 = 1u64;
        nat_eq(a5206, a5207)
    } else {
        false
    } {
        let a5209 = match v1.clone() {
            Adt2::C0(_, _, h5208, _, _, _, _, _, _, _, _) => {
                h5208
            }
        };
        let a5210 = 1u64;
        nat_eq(a5209, a5210)
    } else {
        false
    } {
        let a5212 = match v1.clone() {
            Adt2::C0(_, _, _, h5211, _, _, _, _, _, _, _) => {
                h5211
            }
        };
        let a5213 = 1u64;
        nat_eq(a5212, a5213)
    } else {
        false
    } {
        let a5215 = match v1.clone() {
            Adt2::C0(_, _, _, _, h5214, _, _, _, _, _, _) => {
                h5214
            }
        };
        let a5216 = 1u64;
        nat_eq(a5215, a5216)
    } else {
        false
    } {
        if f40(match v2.clone() {
            Adt2::C0(_, h5217, _, _, _, _, _, _, _, _, _) => {
                h5217
            }
        }, match v2.clone() {
            Adt2::C0(_, _, h5218, _, _, _, _, _, _, _, _) => {
                h5218
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, h5219, _, _, _, _, _, _, _) => {
                h5219
            }
        }, match v2.clone() {
            Adt2::C0(_, _, _, _, h5220, _, _, _, _, _, _) => {
                h5220
            }
        }) {
            Ok(Adt10::C4(0u64, 4u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C9(0u64, 0u64)))
        } else {
            let a5222 = match v1.clone() {
                Adt2::C0(_, h5221, _, _, _, _, _, _, _, _, _) => {
                    h5221
                }
            };
            let a5223 = 0u64;
            if nat_eq(a5222, a5223) {
                Ok(Adt10::C4(f19(v2.clone(), 1u64)?, 4u64, f23(v0, 1u64), f23(v2, 1u64), Adt4::C9(0u64, 0u64)))
            } else {
                let a5225 = match v0.clone() {
                    Adt2::C0(_, h5224, _, _, _, _, _, _, _, _, _) => {
                        h5224
                    }
                };
                let a5226 = 2147483647u64;
                if if if if if if nat_le(a5225, a5226) {
                    let a5228 = match v1.clone() {
                        Adt2::C0(_, h5227, _, _, _, _, _, _, _, _, _) => {
                            h5227
                        }
                    };
                    let a5229 = 2147483647u64;
                    nat_le(a5228, a5229)
                } else {
                    false
                } {
                    let a5231 = 4u64;
                    let a5232 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, h5230, _, _, _, _, _) => {
                            h5230
                        }
                    };
                    nat_le(a5231, a5232)
                } else {
                    false
                } {
                    let a5238 = {
                        let a5235 = match v2.clone() {
                            Adt2::C0(_, h5233, _, _, _, _, _, _, _, _, _) => {
                                h5233
                            }
                        };
                        let a5236 = match v2.clone() {
                            Adt2::C0(_, _, _, _, _, h5234, _, _, _, _, _) => {
                                h5234
                            }
                        };
                        nat_mul(a5235, a5236)?
                    };
                    let a5239 = match v2.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5237, _, _, _, _) => {
                            h5237
                        }
                    };
                    nat_le(a5238, a5239)
                } else {
                    false
                } {
                    f20(v0.clone(), v2.clone())?
                } else {
                    false
                } {
                    f20(v1.clone(), v2.clone())?
                } else {
                    false
                } {
                    Ok(Adt10::C4({
                        let a5252 = {
                            let a5243 = {
                                let a5241 = 2u64;
                                let a5242 = match v1.clone() {
                                    Adt2::C0(_, h5240, _, _, _, _, _, _, _, _, _) => {
                                        h5240
                                    }
                                };
                                nat_mul(a5241, a5242)?
                            };
                            let a5244 = f19(v2.clone(), 1u64)?;
                            nat_add(a5243, a5244)?
                        };
                        let a5253 = {
                            let a5250 = match v1.clone() {
                                Adt2::C0(_, h5245, _, _, _, _, _, _, _, _, _) => {
                                    h5245
                                }
                            };
                            let a5251 = {
                                let a5248 = 1u64;
                                let a5249 = f158(match v0.clone() {
                                    Adt2::C0(h5246, _, _, _, _, _, _, _, _, _, _) => {
                                        h5246
                                    }
                                }, match v0.clone() {
                                    Adt2::C0(_, h5247, _, _, _, _, _, _, _, _, _) => {
                                        h5247
                                    }
                                })?;
                                nat_add(a5248, a5249)?
                            };
                            nat_mul(a5250, a5251)?
                        };
                        nat_add(a5252, a5253)?
                    }, 4u64, Adt3::C0(match v0.clone() {
                        Adt2::C0(_, h5254, _, _, _, _, _, _, _, _, _) => {
                            h5254
                        }
                    }, match v1.clone() {
                        Adt2::C0(_, h5255, _, _, _, _, _, _, _, _, _) => {
                            h5255
                        }
                    }, 1u64, 1u64, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, h5256, _, _, _, _, _) => {
                            h5256
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, h5257, _, _, _, _) => {
                            h5257
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, h5258, _, _, _) => {
                            h5258
                        }
                    }, match v0.clone() {
                        Adt2::C0(_, _, _, _, _, _, _, _, h5259, _, _) => {
                            h5259
                        }
                    }), f23(v2, 1u64), Adt4::C19(match v0 {
                        Adt2::C0(h5260, _, _, _, _, _, _, _, _, _, _) => {
                            h5260
                        }
                    }, f23(v1, 1u64))))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f204(v0: u64) -> bool {
    let a5261 = v0;
    let a5262 = 0u64;
    if if if if if if if if if if if if if if if if if if if if if if if if if if if if if nat_eq(a5261, a5262) {
        true
    } else {
        let a5263 = v0;
        let a5264 = 1u64;
        nat_eq(a5263, a5264)
    } {
        true
    } else {
        let a5265 = v0;
        let a5266 = 2u64;
        nat_eq(a5265, a5266)
    } {
        true
    } else {
        let a5267 = v0;
        let a5268 = 3u64;
        nat_eq(a5267, a5268)
    } {
        true
    } else {
        let a5269 = v0;
        let a5270 = 6u64;
        nat_eq(a5269, a5270)
    } {
        true
    } else {
        let a5271 = v0;
        let a5272 = 7u64;
        nat_eq(a5271, a5272)
    } {
        true
    } else {
        let a5273 = v0;
        let a5274 = 8u64;
        nat_eq(a5273, a5274)
    } {
        true
    } else {
        let a5275 = v0;
        let a5276 = 10u64;
        nat_eq(a5275, a5276)
    } {
        true
    } else {
        let a5277 = v0;
        let a5278 = 11u64;
        nat_eq(a5277, a5278)
    } {
        true
    } else {
        let a5279 = v0;
        let a5280 = 12u64;
        nat_eq(a5279, a5280)
    } {
        true
    } else {
        let a5281 = v0;
        let a5282 = 13u64;
        nat_eq(a5281, a5282)
    } {
        true
    } else {
        let a5283 = v0;
        let a5284 = 14u64;
        nat_eq(a5283, a5284)
    } {
        true
    } else {
        let a5285 = v0;
        let a5286 = 16u64;
        nat_eq(a5285, a5286)
    } {
        true
    } else {
        let a5287 = v0;
        let a5288 = 17u64;
        nat_eq(a5287, a5288)
    } {
        true
    } else {
        let a5289 = v0;
        let a5290 = 18u64;
        nat_eq(a5289, a5290)
    } {
        true
    } else {
        let a5291 = v0;
        let a5292 = 19u64;
        nat_eq(a5291, a5292)
    } {
        true
    } else {
        let a5293 = v0;
        let a5294 = 20u64;
        nat_eq(a5293, a5294)
    } {
        true
    } else {
        let a5295 = v0;
        let a5296 = 21u64;
        nat_eq(a5295, a5296)
    } {
        true
    } else {
        let a5297 = v0;
        let a5298 = 22u64;
        nat_eq(a5297, a5298)
    } {
        true
    } else {
        let a5299 = v0;
        let a5300 = 23u64;
        nat_eq(a5299, a5300)
    } {
        true
    } else {
        let a5301 = v0;
        let a5302 = 26u64;
        nat_eq(a5301, a5302)
    } {
        true
    } else {
        let a5303 = v0;
        let a5304 = 29u64;
        nat_eq(a5303, a5304)
    } {
        true
    } else {
        let a5305 = v0;
        let a5306 = 30u64;
        nat_eq(a5305, a5306)
    } {
        true
    } else {
        let a5307 = v0;
        let a5308 = 34u64;
        nat_eq(a5307, a5308)
    } {
        true
    } else {
        let a5309 = v0;
        let a5310 = 35u64;
        nat_eq(a5309, a5310)
    } {
        true
    } else {
        let a5311 = v0;
        let a5312 = 39u64;
        nat_eq(a5311, a5312)
    } {
        true
    } else {
        let a5313 = v0;
        let a5314 = 40u64;
        nat_eq(a5313, a5314)
    } {
        true
    } else {
        let a5315 = v0;
        let a5316 = 41u64;
        nat_eq(a5315, a5316)
    } {
        true
    } else {
        let a5317 = v0;
        let a5318 = 42u64;
        nat_eq(a5317, a5318)
    } {
        true
    } else {
        let a5319 = v0;
        let a5320 = 43u64;
        nat_eq(a5319, a5320)
    }
}

pub fn f205(v0: u64) -> bool {
    if f159(v0) {
        true
    } else {
        let a5321 = v0;
        let a5322 = 43u64;
        nat_eq(a5321, a5322)
    }
}

pub fn f206(v0: u64, v1: Adt2, v2: bool, v3: Adt2) -> R<Adt10> {
    let a5325 = match v1.clone() {
        Adt2::C0(h5323, _, _, _, _, _, _, _, _, _, _) => {
            h5323
        }
    };
    let a5326 = match v3.clone() {
        Adt2::C0(h5324, _, _, _, _, _, _, _, _, _, _) => {
            h5324
        }
    };
    if if nat_eq(a5325, a5326) {
        let a5327 = v0;
        let a5328 = 33u64;
        if nat_eq(a5327, a5328) {
            v2
        } else {
            true
        }
    } else {
        false
    } {
        let m5330 = f197(match v1.clone() {
            Adt2::C0(h5329, _, _, _, _, _, _, _, _, _, _) => {
                h5329
            }
        });
        match m5330 {
            Adt18::C0 => {
                Ok(Adt10::C0)
            }
            Adt18::C1(v4) => {
                let v5: u64 = match v4.clone() {
                    Adt19::C0(_, h5331, _) => {
                        h5331
                    }
                };
                let v6: u64 = f19(v1.clone(), v5)?;
                let a5336 = {
                    let a5333 = match v1.clone() {
                        Adt2::C0(_, h5332, _, _, _, _, _, _, _, _, _) => {
                            h5332
                        }
                    };
                    let a5334 = v5;
                    let a5335 = 1u64;
                    nat_rem(a5333, a5334, a5335)
                };
                let a5337 = 0u64;
                if if if nat_eq(a5336, a5337) {
                    let a5342 = {
                        let a5339 = match v3.clone() {
                            Adt2::C0(_, h5338, _, _, _, _, _, _, _, _, _) => {
                                h5338
                            }
                        };
                        let a5340 = v5;
                        let a5341 = 1u64;
                        nat_rem(a5339, a5340, a5341)
                    };
                    let a5343 = 0u64;
                    nat_eq(a5342, a5343)
                } else {
                    false
                } {
                    let a5344 = v6;
                    let a5345 = f19(v3.clone(), v5)?;
                    nat_eq(a5344, a5345)
                } else {
                    false
                } {
                    Ok(Adt10::C4(v6, match v4 {
                        Adt19::C0(_, _, h5346) => {
                            h5346
                        }
                    }, f23(v1, v5), f23(v3, v5), Adt4::C0))
                } else {
                    Ok(Adt10::C1)
                }
            }
        }
    } else {
        Ok(Adt10::C1)
    }
}

pub fn f207(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64) -> bool {
    if f40(v4, v5, v6, v7) {
        f40(v0, v1, v2, v3)
    } else {
        let a5350 = {
            let a5347 = v0;
            let a5348 = v4;
            let a5349 = 1u64;
            nat_rem(a5347, a5348, a5349)
        };
        let a5351 = 0u64;
        if if if nat_eq(a5350, a5351) {
            let a5355 = {
                let a5352 = v1;
                let a5353 = v5;
                let a5354 = 1u64;
                nat_rem(a5352, a5353, a5354)
            };
            let a5356 = 0u64;
            nat_eq(a5355, a5356)
        } else {
            false
        } {
            let a5360 = {
                let a5357 = v2;
                let a5358 = v6;
                let a5359 = 1u64;
                nat_rem(a5357, a5358, a5359)
            };
            let a5361 = 0u64;
            nat_eq(a5360, a5361)
        } else {
            false
        } {
            let a5365 = {
                let a5362 = v3;
                let a5363 = v7;
                let a5364 = 1u64;
                nat_rem(a5362, a5363, a5364)
            };
            let a5366 = 0u64;
            nat_eq(a5365, a5366)
        } else {
            false
        }
    }
}

pub fn f208(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<bool> {
    let a5367 = v0;
    let a5368 = v1;
    if nat_eq(a5367, a5368) {
        let a5371 = v4;
        let a5372 = {
            let a5369 = v2;
            let a5370 = v3;
            nat_add(a5369, a5370)?
        };
        Ok(nat_eq(a5371, a5372))
    } else {
        let a5373 = v2;
        let a5374 = v3;
        if nat_eq(a5373, a5374) {
            let a5375 = v4;
            let a5376 = v2;
            Ok(nat_eq(a5375, a5376))
        } else {
            Ok(false)
        }
    }
}

pub fn f209(v0: u64, v1: u64, v2: u64, v3: u64) -> R<bool> {
    let v4: u64 = {
        let a5379 = {
            let a5377 = v0;
            let a5378 = f128(v1);
            nat_add(a5377, a5378)?
        };
        let a5380 = f128(v2);
        nat_add(a5379, a5380)?
    };
    let v5: u64 = {
        let a5381 = f127(v1);
        let a5382 = f127(v2);
        nat_add(a5381, a5382)?
    };
    let a5383 = v1;
    let a5384 = 4294967295u64;
    if if nat_le(a5383, a5384) {
        let a5385 = v2;
        let a5386 = 4294967295u64;
        nat_le(a5385, a5386)
    } else {
        false
    } {
        let a5387 = v5;
        let a5388 = v4;
        if nat_le(a5387, a5388) {
            let a5391 = v3;
            let a5392 = {
                let a5389 = v4;
                let a5390 = v5;
                nat_sub(a5389, a5390)
            };
            Ok(nat_eq(a5391, a5392))
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

pub fn f210(v0: u64, v1: u64) -> R<bool> {
    let a5393 = v1;
    let a5394 = 4294967295u64;
    if nat_le(a5393, a5394) {
        let a5397 = {
            let a5395 = f128(v1);
            let a5396 = f127(v1);
            nat_add(a5395, a5396)?
        };
        let a5398 = v0;
        Ok(nat_lt(a5397, a5398))
    } else {
        Ok(false)
    }
}

pub fn f211(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: bool) -> R<bool> {
    let v5: u64 = f253(v0.clone(), false)?;
    let v6: u64 = f253(v0, true)?;
    let a5399 = v6;
    let a5400 = v1;
    if if nat_le(a5399, a5400) {
        let a5407 = {
            let a5405 = {
                let a5401 = v1;
                let a5402 = v5;
                nat_add(a5401, a5402)?
            };
            let a5406 = {
                let a5403 = v3;
                let a5404 = 0u64;
                if nat_eq(a5403, a5404) {
                    0u64
                } else {
                    4u64
                }
            };
            nat_add(a5405, a5406)?
        };
        let a5408 = v2;
        nat_le(a5407, a5408)
    } else {
        false
    } {
        if v4 {
            let a5413 = {
                let a5411 = {
                    let a5409 = v1;
                    let a5410 = v5;
                    nat_add(a5409, a5410)?
                };
                let a5412 = v6;
                nat_sub(a5411, a5412)
            };
            let a5414 = v2;
            Ok(nat_lt(a5413, a5414))
        } else {
            let a5419 = {
                let a5417 = {
                    let a5415 = v1;
                    let a5416 = v5;
                    nat_add(a5415, a5416)?
                };
                let a5418 = v6;
                nat_sub(a5417, a5418)
            };
            let a5420 = v2;
            Ok(nat_le(a5419, a5420))
        }
    } else {
        Ok(false)
    }
}

pub fn f212(v0: u64, v1: u64) -> bool {
    let a5421 = v0;
    let a5422 = 0u64;
    if if nat_eq(a5421, a5422) {
        true
    } else {
        let a5423 = v0;
        let a5424 = 1u64;
        nat_eq(a5423, a5424)
    } {
        let a5425 = v1;
        let a5426 = 4294967296u64;
        nat_lt(a5425, a5426)
    } else {
        false
    }
}

pub fn f213(v0: u64, v1: u64) -> R<u64> {
    let a5431 = {
        let a5429 = v0;
        let a5430 = {
            let a5427 = v1;
            let a5428 = 1u64;
            nat_sub(a5427, a5428)
        };
        nat_add(a5429, a5430)?
    };
    let a5432 = v1;
    let a5433 = 0u64;
    Ok(nat_quot(a5431, a5432, a5433))
}

pub fn f214(v0: Adt2, v1: Adt2, v2: u64, v3: u64) -> R<bool> {
    if f40(match v0.clone() {
        Adt2::C0(_, h5434, _, _, _, _, _, _, _, _, _) => {
            h5434
        }
    }, match v0.clone() {
        Adt2::C0(_, _, h5435, _, _, _, _, _, _, _, _) => {
            h5435
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, h5436, _, _, _, _, _, _, _) => {
            h5436
        }
    }, match v0.clone() {
        Adt2::C0(_, _, _, _, h5437, _, _, _, _, _, _) => {
            h5437
        }
    }) {
        Ok(true)
    } else {
        let v4: u64 = {
            let a5440 = match v1.clone() {
                Adt2::C0(_, h5438, _, _, _, _, _, _, _, _, _) => {
                    h5438
                }
            };
            let a5441 = match v1.clone() {
                Adt2::C0(_, _, h5439, _, _, _, _, _, _, _, _) => {
                    h5439
                }
            };
            nat_mul(a5440, a5441)?
        };
        let v5: u64 = {
            let a5443 = v4;
            let a5444 = match v1.clone() {
                Adt2::C0(_, _, _, h5442, _, _, _, _, _, _, _) => {
                    h5442
                }
            };
            nat_mul(a5443, a5444)?
        };
        let a5457 = {
            let a5455 = {
                let a5446 = match v0.clone() {
                    Adt2::C0(_, h5445, _, _, _, _, _, _, _, _, _) => {
                        h5445
                    }
                };
                let a5447 = 1u64;
                nat_sub(a5446, a5447)
            };
            let a5456 = {
                let a5453 = f254(match v0.clone() {
                    Adt2::C0(_, _, h5448, _, _, _, _, _, _, _, _) => {
                        h5448
                    }
                }, v2, v5, match v1.clone() {
                    Adt2::C0(_, h5449, _, _, _, _, _, _, _, _, _) => {
                        h5449
                    }
                })?;
                let a5454 = f254(match v0 {
                    Adt2::C0(_, _, _, h5450, _, _, _, _, _, _, _) => {
                        h5450
                    }
                }, v2, {
                    let a5451 = v3;
                    let a5452 = v5;
                    nat_mul(a5451, a5452)?
                }, v4)?;
                nat_add(a5453, a5454)?
            };
            nat_add(a5455, a5456)?
        };
        let a5458 = f19(v1, 1u64)?;
        Ok(nat_lt(a5457, a5458))
    }
}

pub fn f215(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5459 = v0;
    let a5460 = 2147483648u64;
    if if if if nat_lt(a5459, a5460) {
        let a5461 = v1;
        let a5462 = 2147483648u64;
        nat_lt(a5461, a5462)
    } else {
        false
    } {
        let a5465 = {
            let a5463 = v0;
            let a5464 = v1;
            nat_mul(a5463, a5464)?
        };
        let a5466 = 2147483648u64;
        nat_lt(a5465, a5466)
    } else {
        false
    } {
        let a5471 = {
            let a5469 = {
                let a5467 = v0;
                let a5468 = 1u64;
                nat_sub(a5467, a5468)
            };
            let a5470 = v2;
            nat_mul(a5469, a5470)?
        };
        let a5472 = 2147483648u64;
        nat_lt(a5471, a5472)
    } else {
        false
    } {
        let a5477 = {
            let a5475 = {
                let a5473 = v1;
                let a5474 = 1u64;
                nat_sub(a5473, a5474)
            };
            let a5476 = v2;
            nat_mul(a5475, a5476)?
        };
        let a5478 = 2147483648u64;
        Ok(nat_lt(a5477, a5478))
    } else {
        Ok(false)
    }
}

pub fn f216(v0: Adt3, v1: u64, v2: u64) -> R<u64> {
    let a5479 = v1;
    let a5480 = 13u64;
    if if nat_eq(a5479, a5480) {
        f40(match v0.clone() {
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
        })
    } else {
        false
    } {
        Ok(2u64)
    } else {
        let v3: u64 = {
            let a5490 = match v0.clone() {
                Adt3::C0(_, h5485, _, _, _, _, _, _) => {
                    h5485
                }
            };
            let a5491 = {
                let a5488 = match v0.clone() {
                    Adt3::C0(_, _, h5486, _, _, _, _, _) => {
                        h5486
                    }
                };
                let a5489 = match v0.clone() {
                    Adt3::C0(_, _, _, h5487, _, _, _, _) => {
                        h5487
                    }
                };
                nat_mul(a5488, a5489)?
            };
            nat_mul(a5490, a5491)?
        };
        let a5492 = v1;
        let a5493 = 13u64;
        if nat_eq(a5492, a5493) {
            let a5502 = 2u64;
            let a5503 = if f255(v0.clone(), v2) {
                let a5495 = v3;
                let a5496 = f228(match v0 {
                    Adt3::C0(h5494, _, _, _, _, _, _, _) => {
                        h5494
                    }
                }, false)?;
                nat_mul(a5495, a5496)?
            } else {
                let a5500 = v3;
                let a5501 = {
                    let a5498 = match v0 {
                        Adt3::C0(h5497, _, _, _, _, _, _, _) => {
                            h5497
                        }
                    };
                    let a5499 = 2u64;
                    nat_mul(a5498, a5499)?
                };
                nat_mul(a5500, a5501)?
            };
            nat_add(a5502, a5503)
        } else {
            let a5507 = v3;
            let a5508 = f228(match v0 {
                Adt3::C0(h5504, _, _, _, _, _, _, _) => {
                    h5504
                }
            }, {
                let a5505 = v1;
                let a5506 = 16u64;
                nat_eq(a5505, a5506)
            })?;
            nat_mul(a5507, a5508)
        }
    }
}

pub fn f217(v0: u64, v1: u64, v2: Adt2, v3: Option<Adt2>, v4: Adt2) -> R<Adt10> {
    let a5509 = v0;
    let a5510 = 30u64;
    if if nat_eq(a5509, a5510) {
        let a5513 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, _, h5511, _, _, _, _) => {
                h5511
            }
        };
        let a5514 = match v2.clone() {
            Adt2::C0(_, _, _, _, _, h5512, _, _, _, _, _) => {
                h5512
            }
        };
        nat_lt(a5513, a5514)
    } else {
        false
    } {
        Ok(Adt10::C1)
    } else {
        let m5515 = v3;
        match m5515 {
            None => {
                f256(v0, v1, match v2.clone() {
                    Adt2::C0(h5516, _, _, _, _, _, _, _, _, _, _) => {
                        h5516
                    }
                }, 0u64, match v4.clone() {
                    Adt2::C0(h5517, _, _, _, _, _, _, _, _, _, _) => {
                        h5517
                    }
                }, false, match v2.clone() {
                    Adt2::C0(_, h5518, _, _, _, _, _, _, _, _, _) => {
                        h5518
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5519, _, _, _, _, _, _, _, _) => {
                        h5519
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5520, _, _, _, _, _, _, _) => {
                        h5520
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5521, _, _, _, _, _, _) => {
                        h5521
                    }
                }, 1u64, 1u64, 1u64, 1u64, match v4.clone() {
                    Adt2::C0(_, h5522, _, _, _, _, _, _, _, _, _) => {
                        h5522
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5523, _, _, _, _, _, _, _, _) => {
                        h5523
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5524, _, _, _, _, _, _, _) => {
                        h5524
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5525, _, _, _, _, _, _) => {
                        h5525
                    }
                })
            }
            Some(v5) => {
                f256(v0, v1, match v2.clone() {
                    Adt2::C0(h5526, _, _, _, _, _, _, _, _, _, _) => {
                        h5526
                    }
                }, match v5.clone() {
                    Adt2::C0(h5527, _, _, _, _, _, _, _, _, _, _) => {
                        h5527
                    }
                }, match v4.clone() {
                    Adt2::C0(h5528, _, _, _, _, _, _, _, _, _, _) => {
                        h5528
                    }
                }, true, match v2.clone() {
                    Adt2::C0(_, h5529, _, _, _, _, _, _, _, _, _) => {
                        h5529
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, h5530, _, _, _, _, _, _, _, _) => {
                        h5530
                    }
                }, match v2.clone() {
                    Adt2::C0(_, _, _, h5531, _, _, _, _, _, _, _) => {
                        h5531
                    }
                }, match v2 {
                    Adt2::C0(_, _, _, _, h5532, _, _, _, _, _, _) => {
                        h5532
                    }
                }, match v5.clone() {
                    Adt2::C0(_, h5533, _, _, _, _, _, _, _, _, _) => {
                        h5533
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, h5534, _, _, _, _, _, _, _, _) => {
                        h5534
                    }
                }, match v5.clone() {
                    Adt2::C0(_, _, _, h5535, _, _, _, _, _, _, _) => {
                        h5535
                    }
                }, match v5 {
                    Adt2::C0(_, _, _, _, h5536, _, _, _, _, _, _) => {
                        h5536
                    }
                }, match v4.clone() {
                    Adt2::C0(_, h5537, _, _, _, _, _, _, _, _, _) => {
                        h5537
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, h5538, _, _, _, _, _, _, _, _) => {
                        h5538
                    }
                }, match v4.clone() {
                    Adt2::C0(_, _, _, h5539, _, _, _, _, _, _, _) => {
                        h5539
                    }
                }, match v4 {
                    Adt2::C0(_, _, _, _, h5540, _, _, _, _, _, _) => {
                        h5540
                    }
                })
            }
        }
    }
}

pub fn f218(v0: u64, v1: u64) -> u64 {
    let a5541 = v0;
    let a5542 = 23u64;
    if nat_eq(a5541, a5542) {
        let a5543 = v1;
        let a5544 = 0u64;
        if nat_eq(a5543, a5544) {
            2u64
        } else {
            3u64
        }
    } else {
        1u64
    }
}

pub fn f219(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5549 = {
        let a5547 = {
            let a5545 = v0;
            let a5546 = v1;
            nat_sub(a5545, a5546)
        };
        let a5548 = 1u64;
        nat_sub(a5547, a5548)
    };
    let a5550 = v2;
    nat_add(a5549, a5550)
}

pub fn f220(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64) -> R<u64> {
    f126({
        let a5551 = v0;
        let a5552 = v4;
        let a5553 = 0u64;
        nat_rem(a5551, a5552, a5553)
    }, {
        let a5554 = v1;
        let a5555 = v5;
        let a5556 = 0u64;
        nat_rem(a5554, a5555, a5556)
    }, {
        let a5557 = v2;
        let a5558 = v6;
        let a5559 = 0u64;
        nat_rem(a5557, a5558, a5559)
    }, {
        let a5560 = v3;
        let a5561 = v7;
        let a5562 = 0u64;
        nat_rem(a5560, a5561, a5562)
    }, v8, v9, v10, v11)
}

pub fn f221(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5563 = v1;
    let a5564 = 2147483648u64;
    if nat_lt(a5563, a5564) {
        let a5575 = {
            let a5573 = {
                let a5568 = {
                    let a5565 = v0;
                    let a5566 = v2;
                    let a5567 = 0u64;
                    nat_rem(a5565, a5566, a5567)
                };
                let a5569 = v2;
                nat_add(a5568, a5569)?
            };
            let a5574 = {
                let a5570 = v1;
                let a5571 = v2;
                let a5572 = 0u64;
                nat_rem(a5570, a5571, a5572)
            };
            nat_sub(a5573, a5574)
        };
        let a5576 = v2;
        let a5577 = 0u64;
        Ok(nat_rem(a5575, a5576, a5577))
    } else {
        let a5580 = {
            let a5578 = v0;
            let a5579 = f127(v1);
            nat_add(a5578, a5579)?
        };
        let a5581 = v2;
        let a5582 = 0u64;
        Ok(nat_rem(a5580, a5581, a5582))
    }
}

pub fn f222(v0: u64, v1: u64, v2: u64) -> R<bool> {
    let a5585 = f128(v1);
    let a5586 = {
        let a5583 = v0;
        let a5584 = f127(v1);
        nat_add(a5583, a5584)?
    };
    if nat_le(a5585, a5586) {
        let a5587 = f223(v0, v1)?;
        let a5588 = v2;
        Ok(nat_lt(a5587, a5588))
    } else {
        Ok(false)
    }
}

pub fn f223(v0: u64, v1: u64) -> R<u64> {
    let a5591 = {
        let a5589 = v0;
        let a5590 = f127(v1);
        nat_add(a5589, a5590)?
    };
    let a5592 = f128(v1);
    Ok(nat_sub(a5591, a5592))
}

pub fn f224(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a5593 = v0;
    let a5594 = v1;
    if nat_lt(a5593, a5594) {
        let a5595 = v1;
        let a5596 = v0;
        Ok(nat_sub(a5595, a5596))
    } else {
        let a5599 = v0;
        let a5600 = {
            let a5597 = v1;
            let a5598 = v2;
            nat_add(a5597, a5598)?
        };
        if nat_lt(a5599, a5600) {
            let a5601 = v0;
            let a5602 = v1;
            Ok(nat_sub(a5601, a5602))
        } else {
            let a5609 = {
                let a5605 = 2u64;
                let a5606 = {
                    let a5603 = v2;
                    let a5604 = 1u64;
                    nat_sub(a5603, a5604)
                };
                nat_mul(a5605, a5606)?
            };
            let a5610 = {
                let a5607 = v0;
                let a5608 = v1;
                nat_sub(a5607, a5608)
            };
            Ok(nat_sub(a5609, a5610))
        }
    }
}

pub fn f225(v0: u64, v1: Adt3, v2: u64) -> R<u64> {
    let a5613 = {
        let a5611 = v2;
        let a5612 = f257(v1.clone(), f50(v0, v1.clone(), 0u64), f50(v0, v1.clone(), 1u64), f50(v0, v1.clone(), 2u64), f50(v0, v1.clone(), 3u64), false)?;
        nat_add(a5611, a5612)?
    };
    let a5614 = f257(v1.clone(), f50(v0, v1.clone(), 0u64), f50(v0, v1.clone(), 1u64), f50(v0, v1.clone(), 2u64), f50(v0, v1, 3u64), true)?;
    Ok(nat_sub(a5613, a5614))
}

pub fn f226(v0: u64, v1: u64, v2: u64) -> bool {
    let a5615 = v0;
    let a5616 = 0u64;
    if nat_eq(a5615, a5616) {
        let a5617 = v2;
        let a5618 = v1;
        nat_le(a5617, a5618)
    } else {
        let a5619 = v0;
        let a5620 = 1u64;
        if nat_eq(a5619, a5620) {
            let a5621 = v2;
            let a5622 = v1;
            nat_lt(a5621, a5622)
        } else {
            let a5623 = v0;
            let a5624 = 2u64;
            if nat_eq(a5623, a5624) {
                let a5625 = v1;
                let a5626 = v2;
                nat_le(a5625, a5626)
            } else {
                let a5627 = v0;
                let a5628 = 3u64;
                if nat_eq(a5627, a5628) {
                    let a5629 = v1;
                    let a5630 = v2;
                    nat_lt(a5629, a5630)
                } else {
                    let a5631 = v1;
                    let a5632 = v2;
                    nat_eq(a5631, a5632)
                }
            }
        }
    }
}

pub fn f227(v0: u64, v1: Adt3, v2: u64) -> R<Adt9> {
    let a5633 = v0;
    let a5634 = 0u64;
    if nat_eq(a5633, a5634) {
        Ok(f240(0u64))
    } else {
        let a5637 = {
            let a5635 = v0;
            let a5636 = 1u64;
            nat_add(a5635, a5636)?
        };
        let a5638 = f216(v1.clone(), 13u64, v2)?;
        if nat_eq(a5637, a5638) {
            Ok(Adt9::C4({
                let a5639 = v2;
                let a5640 = 0u64;
                if nat_eq(a5639, a5640) {
                    2u64
                } else {
                    7u64
                }
            }, 1u64, 0u64, 2u64, 0u64, 3u64, 0u64))
        } else {
            let v3: u64 = {
                let a5641 = v0;
                let a5642 = 1u64;
                nat_sub(a5641, a5642)
            };
            if f255(v1.clone(), v2) {
                let v4: u64 = f228(match v1.clone() {
                    Adt3::C0(h5643, _, _, _, _, _, _, _) => {
                        h5643
                    }
                }, false)?;
                let v5: u64 = {
                    let a5644 = v3;
                    let a5645 = v4;
                    let a5646 = 0u64;
                    nat_quot(a5644, a5645, a5646)
                };
                let v6: u64 = {
                    let a5647 = v3;
                    let a5648 = v4;
                    let a5649 = 0u64;
                    nat_rem(a5647, a5648, a5649)
                };
                let a5652 = {
                    let a5650 = v6;
                    let a5651 = 1u64;
                    nat_add(a5650, a5651)?
                };
                let a5653 = v4;
                if nat_eq(a5652, a5653) {
                    Ok(Adt9::C4({
                        let a5654 = v2;
                        let a5655 = 0u64;
                        if nat_eq(a5654, a5655) {
                            0u64
                        } else {
                            4u64
                        }
                    }, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    f229(v6, v5, v1, v2)
                }
            } else {
                f258({
                    let a5656 = v3;
                    let a5657 = 2u64;
                    let a5658 = 0u64;
                    nat_quot(a5656, a5657, a5658)
                }, {
                    let a5659 = v3;
                    let a5660 = 2u64;
                    let a5661 = 0u64;
                    nat_rem(a5659, a5660, a5661)
                }, v1, v2, 0u64)
            }
        }
    }
}

pub fn f228(v0: u64, v1: bool) -> R<u64> {
    let a5664 = {
        let a5662 = v0;
        let a5663 = 2u64;
        nat_mul(a5662, a5663)?
    };
    let a5665 = if v1 {
        4u64
    } else {
        2u64
    };
    nat_add(a5664, a5665)
}

pub fn f229(v0: u64, v1: u64, v2: Adt3, v3: u64) -> R<Adt9> {
    let a5666 = v0;
    let a5667 = 0u64;
    if nat_eq(a5666, a5667) {
        Ok(f240(1u64))
    } else {
        f258({
            let a5676 = {
                let a5669 = v1;
                let a5670 = match v2.clone() {
                    Adt3::C0(h5668, _, _, _, _, _, _, _) => {
                        h5668
                    }
                };
                nat_mul(a5669, a5670)?
            };
            let a5677 = {
                let a5673 = {
                    let a5671 = v0;
                    let a5672 = 1u64;
                    nat_sub(a5671, a5672)
                };
                let a5674 = 2u64;
                let a5675 = 0u64;
                nat_quot(a5673, a5674, a5675)
            };
            nat_add(a5676, a5677)?
        }, {
            let a5680 = {
                let a5678 = v0;
                let a5679 = 1u64;
                nat_sub(a5678, a5679)
            };
            let a5681 = 2u64;
            let a5682 = 0u64;
            nat_rem(a5680, a5681, a5682)
        }, v2, v3, 1u64)
    }
}

pub fn f230(v0: u64, v1: u64) -> R<u64> {
    let a5683 = v0;
    let a5684 = f82(v1)?;
    let a5685 = 0u64;
    Ok(nat_rem(a5683, a5684, a5685))
}

pub fn f231(v0: u64, v1: u64) -> R<u64> {
    let a5686 = 4u64;
    let a5687 = f259(v0, v1)?;
    nat_mul(a5686, a5687)
}

pub fn f232(v0: u64, v1: u64) -> R<u64> {
    let a5688 = 4u64;
    let a5689 = f260(v0, v1)?;
    nat_mul(a5688, a5689)
}

pub fn f233(v0: u64, v1: u64) -> R<bool> {
    let a5695 = {
        let a5692 = {
            let a5690 = f230(v0, v1)?;
            let a5691 = 2u64;
            nat_sub(a5690, a5691)
        };
        let a5693 = 2u64;
        let a5694 = 0u64;
        nat_rem(a5692, a5693, a5694)
    };
    let a5696 = 0u64;
    Ok(nat_eq(a5695, a5696))
}

pub fn f234(v0: u64) -> u64 {
    let m5697 = f197(v0);
    match m5697 {
        Adt18::C0 => {
            0u64
        }
        Adt18::C1(v1) => {
            match v1 {
                Adt19::C0(_, h5698, _) => {
                    h5698
                }
            }
        }
    }
}

pub fn f235(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64) -> R<Option<u64>> {
    let m5699 = f197(v0);
    match m5699 {
        Adt18::C0 => {
            Ok(None::<u64>)
        }
        Adt18::C1(v13) => {
            if if if if if f204(v0) {
                let a5700 = v1;
                let a5701 = v7;
                nat_lt(a5700, a5701)
            } else {
                false
            } {
                f164(v8, v2)
            } else {
                false
            } {
                let a5702 = v3;
                let a5703 = f261(v4, v5, v6)?;
                nat_lt(a5702, a5703)
            } else {
                false
            } {
                let a5708 = {
                    let a5705 = v7;
                    let a5706 = match v13.clone() {
                        Adt19::C0(_, h5704, _) => {
                            h5704
                        }
                    };
                    let a5707 = 0u64;
                    nat_rem(a5705, a5706, a5707)
                };
                let a5709 = 0u64;
                nat_eq(a5708, a5709)
            } else {
                false
            } {
                Ok(Some({
                    let a5738 = {
                        let a5728 = {
                            let a5718 = {
                                let a5714 = {
                                    let a5711 = v1;
                                    let a5712 = match v13 {
                                        Adt19::C0(_, h5710, _) => {
                                            h5710
                                        }
                                    };
                                    let a5713 = 0u64;
                                    nat_quot(a5711, a5712, a5713)
                                };
                                let a5715 = v9;
                                nat_mul(a5714, a5715)?
                            };
                            let a5719 = {
                                let a5716 = v2;
                                let a5717 = v10;
                                nat_mul(a5716, a5717)?
                            };
                            nat_add(a5718, a5719)?
                        };
                        let a5729 = {
                            let a5726 = {
                                let a5723 = {
                                    let a5720 = v3;
                                    let a5721 = v4;
                                    let a5722 = 0u64;
                                    nat_quot(a5720, a5721, a5722)
                                };
                                let a5724 = v5;
                                let a5725 = 0u64;
                                nat_rem(a5723, a5724, a5725)
                            };
                            let a5727 = v11;
                            nat_mul(a5726, a5727)?
                        };
                        nat_add(a5728, a5729)?
                    };
                    let a5739 = {
                        let a5736 = {
                            let a5733 = {
                                let a5730 = v3;
                                let a5731 = v4;
                                let a5732 = 0u64;
                                nat_quot(a5730, a5731, a5732)
                            };
                            let a5734 = v5;
                            let a5735 = 0u64;
                            nat_quot(a5733, a5734, a5735)
                        };
                        let a5737 = v12;
                        nat_mul(a5736, a5737)?
                    };
                    nat_add(a5738, a5739)?
                }))
            } else {
                Ok(None::<u64>)
            }
        }
    }
}

pub fn f236(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> R<Adt17> {
    f262(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), v13, v14)
}

pub fn f237(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64, v18: u64, v19: u64, v20: u64) -> R<Adt17> {
    f263(v0, Adt20::C0(v1, v2, v3, v4, v5, v6, v7, v8), Adt20::C0(v1, v2, v3, v4, v9, v10, v11, v12), Adt20::C0(v1, v2, v3, v4, v13, v14, v15, v16), v17, v18, v19, v20)
}

pub fn f238(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt17> {
    let a5740 = v0;
    let a5741 = 64u64;
    if nat_eq(a5740, a5741) {
        f264(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
    } else {
        let a5742 = v0;
        let a5743 = 65u64;
        if nat_eq(a5742, a5743) {
            f265(v1, Adt20::C0(v2, v3, v4, v5, v6, v7, v8, v9), Adt20::C0(v2, v3, v4, v5, v10, v11, v12, v13), v14, v15, v16, v17)
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f239(v0: u64) -> u64 {
    let a5744 = v0;
    let a5745 = 30u64;
    if nat_eq(a5744, a5745) {
        4u64
    } else {
        3u64
    }
}

pub fn f240(v0: u64) -> Adt9 {
    Adt9::C4(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f241(v0: u64) -> bool {
    let a5746 = 0u64;
    let a5747 = v0;
    if nat_lt(a5746, a5747) {
        let a5748 = v0;
        let a5749 = 2139095040u64;
        nat_le(a5748, a5749)
    } else {
        false
    }
}

pub fn f242(v0: u64) -> u64 {
    let a5750 = v0;
    let a5751 = 40u64;
    if nat_lt(a5750, a5751) {
        let a5752 = v0;
        let a5753 = 20u64;
        if nat_lt(a5752, a5753) {
            let a5754 = v0;
            let a5755 = 10u64;
            if nat_lt(a5754, a5755) {
                let a5756 = v0;
                let a5757 = 5u64;
                if nat_lt(a5756, a5757) {
                    let a5758 = v0;
                    let a5759 = 2u64;
                    if nat_lt(a5758, a5759) {
                        let a5760 = v0;
                        let a5761 = 1u64;
                        if nat_lt(a5760, a5761) {
                            1u64
                        } else {
                            2u64
                        }
                    } else {
                        let a5762 = v0;
                        let a5763 = 3u64;
                        if nat_lt(a5762, a5763) {
                            4u64
                        } else {
                            let a5764 = v0;
                            let a5765 = 4u64;
                            if nat_lt(a5764, a5765) {
                                8u64
                            } else {
                                16u64
                            }
                        }
                    }
                } else {
                    let a5766 = v0;
                    let a5767 = 7u64;
                    if nat_lt(a5766, a5767) {
                        let a5768 = v0;
                        let a5769 = 6u64;
                        if nat_lt(a5768, a5769) {
                            32u64
                        } else {
                            64u64
                        }
                    } else {
                        let a5770 = v0;
                        let a5771 = 8u64;
                        if nat_lt(a5770, a5771) {
                            128u64
                        } else {
                            let a5772 = v0;
                            let a5773 = 9u64;
                            if nat_lt(a5772, a5773) {
                                256u64
                            } else {
                                512u64
                            }
                        }
                    }
                }
            } else {
                let a5774 = v0;
                let a5775 = 15u64;
                if nat_lt(a5774, a5775) {
                    let a5776 = v0;
                    let a5777 = 12u64;
                    if nat_lt(a5776, a5777) {
                        let a5778 = v0;
                        let a5779 = 11u64;
                        if nat_lt(a5778, a5779) {
                            1024u64
                        } else {
                            2048u64
                        }
                    } else {
                        let a5780 = v0;
                        let a5781 = 13u64;
                        if nat_lt(a5780, a5781) {
                            4096u64
                        } else {
                            let a5782 = v0;
                            let a5783 = 14u64;
                            if nat_lt(a5782, a5783) {
                                8192u64
                            } else {
                                16384u64
                            }
                        }
                    }
                } else {
                    let a5784 = v0;
                    let a5785 = 17u64;
                    if nat_lt(a5784, a5785) {
                        let a5786 = v0;
                        let a5787 = 16u64;
                        if nat_lt(a5786, a5787) {
                            32768u64
                        } else {
                            65536u64
                        }
                    } else {
                        let a5788 = v0;
                        let a5789 = 18u64;
                        if nat_lt(a5788, a5789) {
                            131072u64
                        } else {
                            let a5790 = v0;
                            let a5791 = 19u64;
                            if nat_lt(a5790, a5791) {
                                262144u64
                            } else {
                                524288u64
                            }
                        }
                    }
                }
            }
        } else {
            let a5792 = v0;
            let a5793 = 30u64;
            if nat_lt(a5792, a5793) {
                let a5794 = v0;
                let a5795 = 25u64;
                if nat_lt(a5794, a5795) {
                    let a5796 = v0;
                    let a5797 = 22u64;
                    if nat_lt(a5796, a5797) {
                        let a5798 = v0;
                        let a5799 = 21u64;
                        if nat_lt(a5798, a5799) {
                            1048576u64
                        } else {
                            2097152u64
                        }
                    } else {
                        let a5800 = v0;
                        let a5801 = 23u64;
                        if nat_lt(a5800, a5801) {
                            4194304u64
                        } else {
                            let a5802 = v0;
                            let a5803 = 24u64;
                            if nat_lt(a5802, a5803) {
                                8388608u64
                            } else {
                                16777216u64
                            }
                        }
                    }
                } else {
                    let a5804 = v0;
                    let a5805 = 27u64;
                    if nat_lt(a5804, a5805) {
                        let a5806 = v0;
                        let a5807 = 26u64;
                        if nat_lt(a5806, a5807) {
                            33554432u64
                        } else {
                            67108864u64
                        }
                    } else {
                        let a5808 = v0;
                        let a5809 = 28u64;
                        if nat_lt(a5808, a5809) {
                            134217728u64
                        } else {
                            let a5810 = v0;
                            let a5811 = 29u64;
                            if nat_lt(a5810, a5811) {
                                268435456u64
                            } else {
                                536870912u64
                            }
                        }
                    }
                }
            } else {
                let a5812 = v0;
                let a5813 = 35u64;
                if nat_lt(a5812, a5813) {
                    let a5814 = v0;
                    let a5815 = 32u64;
                    if nat_lt(a5814, a5815) {
                        let a5816 = v0;
                        let a5817 = 31u64;
                        if nat_lt(a5816, a5817) {
                            1073741824u64
                        } else {
                            2147483648u64
                        }
                    } else {
                        let a5818 = v0;
                        let a5819 = 33u64;
                        if nat_lt(a5818, a5819) {
                            4294967296u64
                        } else {
                            let a5820 = v0;
                            let a5821 = 34u64;
                            if nat_lt(a5820, a5821) {
                                8589934592u64
                            } else {
                                17179869184u64
                            }
                        }
                    }
                } else {
                    let a5822 = v0;
                    let a5823 = 37u64;
                    if nat_lt(a5822, a5823) {
                        let a5824 = v0;
                        let a5825 = 36u64;
                        if nat_lt(a5824, a5825) {
                            34359738368u64
                        } else {
                            68719476736u64
                        }
                    } else {
                        let a5826 = v0;
                        let a5827 = 38u64;
                        if nat_lt(a5826, a5827) {
                            137438953472u64
                        } else {
                            let a5828 = v0;
                            let a5829 = 39u64;
                            if nat_lt(a5828, a5829) {
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

pub fn f243(v0: u64, v1: u64) -> bool {
    let a5830 = v0;
    let a5831 = 1u64;
    if nat_le(a5830, a5831) {
        true
    } else {
        let a5835 = {
            let a5832 = v1;
            let a5833 = 4u64;
            let a5834 = 0u64;
            nat_rem(a5832, a5833, a5834)
        };
        let a5836 = 0u64;
        nat_eq(a5835, a5836)
    }
}

pub fn f244(v0: u64, v1: u64) -> bool {
    let a5837 = v0;
    let a5838 = 1u64;
    if nat_le(a5837, a5838) {
        true
    } else {
        let a5839 = 4u64;
        let a5840 = v1;
        nat_le(a5839, a5840)
    }
}

pub fn f245(v0: u64, v1: u64) -> R<u64> {
    let a5841 = v0;
    let a5842 = 0u64;
    if nat_eq(a5841, a5842) {
        Ok(0u64)
    } else {
        let a5847 = 4u64;
        let a5848 = {
            let a5845 = {
                let a5843 = v0;
                let a5844 = 1u64;
                nat_sub(a5843, a5844)
            };
            let a5846 = v1;
            nat_mul(a5845, a5846)?
        };
        nat_add(a5847, a5848)
    }
}

pub fn f246(v0: bool, v1: u64) -> R<u64> {
    let a5853 = {
        let a5851 = {
            let a5849 = f266(v0, v1)?;
            let a5850 = f267(v0, v1)?;
            nat_add(a5849, a5850)?
        };
        let a5852 = f268(v0);
        nat_add(a5851, a5852)?
    };
    let a5854 = v1;
    nat_add(a5853, a5854)
}

pub fn f247(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f269(v0, v1)?;
    let v3: u64 = {
        let a5859 = {
            let a5857 = {
                let a5855 = v1;
                let a5856 = 1u64;
                nat_sub(a5855, a5856)
            };
            let a5858 = v2;
            nat_mul(a5857, a5858)?
        };
        let a5860 = v0;
        nat_sub(a5859, a5860)
    };
    let a5861 = v2;
    let a5862 = v3;
    if nat_lt(a5861, a5862) {
        Ok(v3)
    } else {
        Ok(v2)
    }
}

pub fn f248(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5863 = v4;
    let a5864 = 0u64;
    if nat_eq(a5863, a5864) {
        Ok(0u64)
    } else {
        let a5871 = 2u64;
        let a5872 = {
            let a5869 = {
                let a5865 = v0;
                let a5866 = v1;
                nat_mul(a5865, a5866)?
            };
            let a5870 = {
                let a5867 = v2;
                let a5868 = v3;
                nat_mul(a5867, a5868)?
            };
            nat_mul(a5869, a5870)?
        };
        nat_mul(a5871, a5872)
    }
}

pub fn f249(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> R<u64> {
    let a5877 = v3;
    let a5878 = {
        let a5875 = {
            let a5873 = f270(v2, v4)?;
            let a5874 = f271(v0, v1, f269(v2, v4)?)?;
            nat_mul(a5873, a5874)?
        };
        let a5876 = f271(v0, v1, f272(v2, v4)?)?;
        nat_add(a5875, a5876)?
    };
    nat_mul(a5877, a5878)
}

pub fn f250(v0: u64) -> R<u64> {
    let a5881 = 13u64;
    let a5882 = {
        let a5879 = 9u64;
        let a5880 = v0;
        nat_mul(a5879, a5880)?
    };
    nat_add(a5881, a5882)
}

pub fn f251(v0: u64, v1: u64, _: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64) -> bool {
    if if if if if if if if if f273(v0, v1, v6, v7, v8) {
        let a5883 = v5;
        let a5884 = 26u64;
        nat_eq(a5883, a5884)
    } else {
        false
    } {
        let a5885 = v3;
        let a5886 = v7;
        nat_eq(a5885, a5886)
    } else {
        false
    } {
        let a5887 = v4;
        let a5888 = v8;
        nat_eq(a5887, a5888)
    } else {
        false
    } {
        let a5889 = v9;
        let a5890 = 1u64;
        nat_eq(a5889, a5890)
    } else {
        false
    } {
        let a5891 = v10;
        let a5892 = f274(v0);
        nat_eq(a5891, a5892)
    } else {
        false
    } {
        let a5893 = v11;
        let a5894 = v1;
        nat_eq(a5893, a5894)
    } else {
        false
    } {
        let a5895 = v12;
        let a5896 = v6;
        nat_eq(a5895, a5896)
    } else {
        false
    } {
        let a5897 = v13;
        let a5898 = v7;
        nat_eq(a5897, a5898)
    } else {
        false
    } {
        let a5899 = v14;
        let a5900 = v8;
        nat_eq(a5899, a5900)
    } else {
        false
    }
}

pub fn f252(v0: u64, v1: u64, v2: u64, v3: u64) -> bool {
    let a5901 = v0;
    let a5902 = 0u64;
    if if if nat_eq(a5901, a5902) {
        true
    } else {
        let a5903 = v1;
        let a5904 = 0u64;
        nat_eq(a5903, a5904)
    } {
        true
    } else {
        let a5905 = v2;
        let a5906 = 0u64;
        nat_eq(a5905, a5906)
    } {
        true
    } else {
        let a5907 = v3;
        let a5908 = 0u64;
        nat_eq(a5907, a5908)
    }
}

pub fn f253(v0: Adt3, v1: bool) -> R<u64> {
    f257(v0.clone(), {
        let a5910 = match v0.clone() {
            Adt3::C0(h5909, _, _, _, _, _, _, _) => {
                h5909
            }
        };
        let a5911 = 1u64;
        nat_sub(a5910, a5911)
    }, {
        let a5913 = match v0.clone() {
            Adt3::C0(_, h5912, _, _, _, _, _, _) => {
                h5912
            }
        };
        let a5914 = 1u64;
        nat_sub(a5913, a5914)
    }, {
        let a5916 = match v0.clone() {
            Adt3::C0(_, _, h5915, _, _, _, _, _) => {
                h5915
            }
        };
        let a5917 = 1u64;
        nat_sub(a5916, a5917)
    }, {
        let a5919 = match v0 {
            Adt3::C0(_, _, _, h5918, _, _, _, _) => {
                h5918
            }
        };
        let a5920 = 1u64;
        nat_sub(a5919, a5920)
    }, v1)
}

pub fn f254(v0: u64, v1: u64, v2: u64, v3: u64) -> R<u64> {
    let a5921 = v0;
    let a5922 = 0u64;
    if nat_eq(a5921, a5922) {
        Ok(0u64)
    } else {
        let v4: u64 = {
            let a5923 = v0;
            let a5924 = 1u64;
            nat_sub(a5923, a5924)
        };
        let v5: u64 = {
            let a5925 = v4;
            let a5926 = v1;
            let a5927 = 0u64;
            nat_quot(a5925, a5926, a5927)
        };
        let v6: u64 = {
            let a5935 = {
                let a5928 = v5;
                let a5929 = v2;
                nat_mul(a5928, a5929)?
            };
            let a5936 = {
                let a5933 = {
                    let a5930 = v4;
                    let a5931 = v1;
                    let a5932 = 0u64;
                    nat_rem(a5930, a5931, a5932)
                };
                let a5934 = v3;
                nat_mul(a5933, a5934)?
            };
            nat_add(a5935, a5936)?
        };
        let a5937 = v5;
        let a5938 = 0u64;
        if nat_eq(a5937, a5938) {
            Ok(v6)
        } else {
            let v7: u64 = {
                let a5947 = {
                    let a5941 = {
                        let a5939 = v5;
                        let a5940 = 1u64;
                        nat_sub(a5939, a5940)
                    };
                    let a5942 = v2;
                    nat_mul(a5941, a5942)?
                };
                let a5948 = {
                    let a5945 = {
                        let a5943 = v1;
                        let a5944 = 1u64;
                        nat_sub(a5943, a5944)
                    };
                    let a5946 = v3;
                    nat_mul(a5945, a5946)?
                };
                nat_add(a5947, a5948)?
            };
            let a5949 = v6;
            let a5950 = v7;
            if nat_lt(a5949, a5950) {
                Ok(v7)
            } else {
                Ok(v6)
            }
        }
    }
}

pub fn f255(v0: Adt3, v1: u64) -> bool {
    let a5951 = v1;
    let a5952 = 0u64;
    if !nat_eq(a5951, a5952) {
        true
    } else {
        let a5954 = match v0 {
            Adt3::C0(_, _, _, _, h5953, _, _, _) => {
                h5953
            }
        };
        let a5955 = 4u64;
        nat_eq(a5954, a5955)
    }
}

pub fn f256(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64, v16: u64, v17: u64) -> R<Adt10> {
    let m5956 = f275(v0, v1);
    match m5956 {
        None => {
            if v5 {
                let m5957 = f276(v0, v2, v3, v4, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)?;
                match m5957 {
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
                let a5958 = 4u64;
                let a5959 = v28;
                nat_le(a5958, a5959)
            };
            let a5960 = v2;
            let a5961 = 0u64;
            if if if if if nat_eq(a5960, a5961) {
                let a5962 = v4;
                let a5963 = 0u64;
                nat_eq(a5962, a5963)
            } else {
                false
            } {
                let a5964 = v6;
                let a5965 = v14;
                if nat_eq(a5964, a5965) {
                    let a5966 = v7;
                    let a5967 = v15;
                    nat_eq(a5966, a5967)
                } else {
                    false
                }
            } else {
                false
            } {
                let a5968 = v8;
                let a5969 = v16;
                if nat_eq(a5968, a5969) {
                    let a5970 = v9;
                    let a5971 = v17;
                    nat_eq(a5970, a5971)
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
                        let a5972 = v3;
                        let a5973 = 0u64;
                        nat_eq(a5972, a5973)
                    } else {
                        false
                    } {
                        f207(v6, v7, v8, v9, v10, v11, v12, v13)
                    } else {
                        false
                    }
                }
            } else {
                false
            } {
                Ok(Adt10::C2(v28, v29, v6, v7, v8, v9, {
                    let a5978 = {
                        let a5974 = v6;
                        let a5975 = v7;
                        nat_mul(a5974, a5975)?
                    };
                    let a5979 = {
                        let a5976 = v8;
                        let a5977 = v9;
                        nat_mul(a5976, a5977)?
                    };
                    nat_mul(a5978, a5979)?
                }))
            } else {
                Ok(Adt10::C1)
            }
        }
    }
}

pub fn f257(v0: Adt3, v1: u64, v2: u64, v3: u64, v4: u64, v5: bool) -> R<u64> {
    let a6000 = {
        let a5994 = {
            let a5988 = {
                let a5982 = v1;
                let a5983 = if v5 {
                    f127(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5980, _, _, _) => {
                            h5980
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, h5981, _, _, _) => {
                            h5981
                        }
                    })
                };
                nat_mul(a5982, a5983)?
            };
            let a5989 = {
                let a5986 = v2;
                let a5987 = if v5 {
                    f127(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5984, _, _) => {
                            h5984
                        }
                    })
                } else {
                    f128(match v0.clone() {
                        Adt3::C0(_, _, _, _, _, h5985, _, _) => {
                            h5985
                        }
                    })
                };
                nat_mul(a5986, a5987)?
            };
            nat_add(a5988, a5989)?
        };
        let a5995 = {
            let a5992 = v3;
            let a5993 = if v5 {
                f127(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5990, _) => {
                        h5990
                    }
                })
            } else {
                f128(match v0.clone() {
                    Adt3::C0(_, _, _, _, _, _, h5991, _) => {
                        h5991
                    }
                })
            };
            nat_mul(a5992, a5993)?
        };
        nat_add(a5994, a5995)?
    };
    let a6001 = {
        let a5998 = v4;
        let a5999 = if v5 {
            f127(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5996) => {
                    h5996
                }
            })
        } else {
            f128(match v0 {
                Adt3::C0(_, _, _, _, _, _, _, h5997) => {
                    h5997
                }
            })
        };
        nat_mul(a5998, a5999)?
    };
    nat_add(a6000, a6001)
}

pub fn f258(v0: u64, v1: u64, v2: Adt3, v3: u64, v4: u64) -> R<Adt9> {
    let a6002 = v1;
    let a6003 = 0u64;
    if nat_eq(a6002, a6003) {
        Ok(Adt9::C4({
            let a6004 = v3;
            let a6005 = 0u64;
            if nat_eq(a6004, a6005) {
                1u64
            } else {
                6u64
            }
        }, 0u64, f79(v0, v2)?, 2u64, 0u64, 1u64, 2u64))
    } else {
        Ok(Adt9::C4({
            let a6006 = v3;
            let a6007 = 0u64;
            if nat_eq(a6006, a6007) {
                0u64
            } else {
                4u64
            }
        }, 1u64, v4, 1u64, 2u64, 1u64, v4))
    }
}

pub fn f259(v0: u64, v1: u64) -> R<u64> {
    let a6008 = v0;
    let a6009 = f82(v1)?;
    let a6010 = 0u64;
    Ok(nat_quot(a6008, a6009, a6010))
}

pub fn f260(v0: u64, v1: u64) -> R<u64> {
    let v2: u64 = f259(v0, v1)?;
    let v3: u64 = {
        let a6013 = {
            let a6011 = f230(v0, v1)?;
            let a6012 = 2u64;
            nat_sub(a6011, a6012)
        };
        let a6014 = 2u64;
        let a6015 = 0u64;
        nat_quot(a6013, a6014, a6015)
    };
    let v4: u64 = {
        let a6016 = v2;
        let a6017 = v1;
        nat_mul(a6016, a6017)?
    };
    if f233(v0, v1)? {
        let a6018 = v4;
        let a6019 = v3;
        nat_add(a6018, a6019)
    } else {
        let a6031 = {
            let a6027 = v4;
            let a6028 = {
                let a6025 = {
                    let a6020 = v1;
                    let a6021 = 1u64;
                    nat_sub(a6020, a6021)
                };
                let a6026 = {
                    let a6022 = v2;
                    let a6023 = v1;
                    let a6024 = 0u64;
                    nat_rem(a6022, a6023, a6024)
                };
                nat_mul(a6025, a6026)?
            };
            nat_sub(a6027, a6028)
        };
        let a6032 = {
            let a6029 = v3;
            let a6030 = v1;
            nat_mul(a6029, a6030)?
        };
        nat_add(a6031, a6032)
    }
}

pub fn f261(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6033 = v0;
    let a6034 = 0u64;
    if if if nat_eq(a6033, a6034) {
        true
    } else {
        let a6035 = v1;
        let a6036 = 0u64;
        nat_eq(a6035, a6036)
    } {
        true
    } else {
        let a6037 = v2;
        let a6038 = 0u64;
        nat_eq(a6037, a6038)
    } {
        Ok(0u64)
    } else {
        let a6041 = {
            let a6039 = v0;
            let a6040 = v1;
            nat_mul(a6039, a6040)?
        };
        let a6042 = v2;
        nat_mul(a6041, a6042)
    }
}

pub fn f262(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = f91(match v1.clone() {
        Adt20::C0(h6043, _, _, _, _, _, _, _) => {
            h6043
        }
    })?;
    let v6: u64 = {
        let a6049 = match v1.clone() {
            Adt20::C0(_, h6044, _, _, _, _, _, _) => {
                h6044
            }
        };
        let a6050 = {
            let a6047 = match v1.clone() {
                Adt20::C0(_, _, h6045, _, _, _, _, _) => {
                    h6045
                }
            };
            let a6048 = match v1.clone() {
                Adt20::C0(_, _, _, h6046, _, _, _, _) => {
                    h6046
                }
            };
            nat_mul(a6047, a6048)?
        };
        nat_mul(a6049, a6050)?
    };
    let a6052 = v0;
    let a6053 = f45(match v1.clone() {
        Adt20::C0(h6051, _, _, _, _, _, _, _) => {
            h6051
        }
    }, v6)?;
    if nat_lt(a6052, a6053) {
        f277({
            let a6054 = v0;
            let a6055 = v5;
            let a6056 = 0u64;
            nat_rem(a6054, a6055, a6056)
        }, {
            let a6057 = v0;
            let a6058 = v5;
            let a6059 = 0u64;
            nat_quot(a6057, a6058, a6059)
        }, v1, v2, v3, v4)
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f263(v0: u64, v1: Adt20, v2: Adt20, v3: Adt20, v4: u64, v5: u64, v6: u64, v7: u64) -> R<Adt17> {
    let v8: u64 = {
        let a6060 = v6;
        let a6061 = 6u64;
        nat_sub(a6060, a6061)
    };
    let v9: Adt20 = f278(v3);
    let a6063 = v0;
    let a6064 = f194(match v1.clone() {
        Adt20::C0(h6062, _, _, _, _, _, _, _) => {
            h6062
        }
    }, v5, v8)?;
    if nat_lt(a6063, a6064) {
        let v10: u64 = f88(match v1.clone() {
            Adt20::C0(h6065, _, _, _, _, _, _, _) => {
                h6065
            }
        }, v5, v8)?;
        let a6066 = v0;
        let a6067 = v10;
        if nat_lt(a6066, a6067) {
            f279(v0, v9, 4u64, 6u64, v7)
        } else {
            let v11: u64 = {
                let a6068 = v0;
                let a6069 = v10;
                nat_sub(a6068, a6069)
            };
            let v12: u64 = f250(match v1.clone() {
                Adt20::C0(h6070, _, _, _, _, _, _, _) => {
                    h6070
                }
            })?;
            f280({
                let a6071 = v11;
                let a6072 = v12;
                let a6073 = 0u64;
                nat_rem(a6071, a6072, a6073)
            }, {
                let a6074 = v11;
                let a6075 = v12;
                let a6076 = 0u64;
                nat_quot(a6074, a6075, a6076)
            }, v1, v2, v9, v4)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f264(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6077 = v5;
        let a6078 = 6u64;
        nat_sub(a6077, a6078)
    };
    let v8: bool = f188(match v1.clone() {
        Adt20::C0(_, _, _, _, h6079, _, _, _) => {
            h6079
        }
    }, match v2.clone() {
        Adt20::C0(_, _, _, _, h6080, _, _, _) => {
            h6080
        }
    });
    let a6082 = v0;
    let a6083 = f187(v8, match v1.clone() {
        Adt20::C0(h6081, _, _, _, _, _, _, _) => {
            h6081
        }
    }, v4, v7)?;
    if nat_lt(a6082, a6083) {
        let v9: u64 = f88(match v1.clone() {
            Adt20::C0(h6084, _, _, _, _, _, _, _) => {
                h6084
            }
        }, v4, v7)?;
        let a6085 = v0;
        let a6086 = v9;
        if nat_lt(a6085, a6086) {
            f279(v0, v2, 4u64, 6u64, v6)
        } else {
            let v10: u64 = {
                let a6087 = v0;
                let a6088 = v9;
                nat_sub(a6087, a6088)
            };
            let v11: u64 = f246(v8, match v1.clone() {
                Adt20::C0(h6089, _, _, _, _, _, _, _) => {
                    h6089
                }
            })?;
            f281({
                let a6090 = v10;
                let a6091 = v11;
                let a6092 = 0u64;
                nat_rem(a6090, a6091, a6092)
            }, {
                let a6093 = v10;
                let a6094 = v11;
                let a6095 = 0u64;
                nat_quot(a6093, a6094, a6095)
            }, v1, v2, v3, v8)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f265(v0: u64, v1: Adt20, v2: Adt20, v3: u64, v4: u64, v5: u64, v6: u64) -> R<Adt17> {
    let v7: u64 = {
        let a6096 = v5;
        let a6097 = 6u64;
        nat_sub(a6096, a6097)
    };
    let v8: Adt20 = f278(v2);
    let v9: u64 = f248(match v1.clone() {
        Adt20::C0(h6098, _, _, _, _, _, _, _) => {
            h6098
        }
    }, match v1.clone() {
        Adt20::C0(_, h6099, _, _, _, _, _, _) => {
            h6099
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6100, _, _, _, _, _) => {
            h6100
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6101, _, _, _, _) => {
            h6101
        }
    }, v7)?;
    let a6106 = v0;
    let a6107 = f193(match v1.clone() {
        Adt20::C0(h6102, _, _, _, _, _, _, _) => {
            h6102
        }
    }, match v1.clone() {
        Adt20::C0(_, h6103, _, _, _, _, _, _) => {
            h6103
        }
    }, match v1.clone() {
        Adt20::C0(_, _, h6104, _, _, _, _, _) => {
            h6104
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, h6105, _, _, _, _) => {
            h6105
        }
    }, v4, v7)?;
    if nat_lt(a6106, a6107) {
        let a6108 = v0;
        let a6109 = v9;
        if nat_lt(a6108, a6109) {
            f279(v0, v8, 4u64, 6u64, v6)
        } else {
            let v10: Adt21 = f282({
                let a6110 = v0;
                let a6111 = v9;
                nat_sub(a6110, a6111)
            }, match v1.clone() {
                Adt20::C0(h6112, _, _, _, _, _, _, _) => {
                    h6112
                }
            }, match v1.clone() {
                Adt20::C0(_, h6113, _, _, _, _, _, _) => {
                    h6113
                }
            }, match v1.clone() {
                Adt20::C0(_, _, h6114, _, _, _, _, _) => {
                    h6114
                }
            }, match v1.clone() {
                Adt20::C0(_, _, _, h6115, _, _, _, _) => {
                    h6115
                }
            }, v4)?;
            f283(v10, v1, v8, v3)
        }
    } else {
        Ok(Adt17::C0)
    }
}

pub fn f266(v0: bool, v1: u64) -> R<u64> {
    let a6118 = if v0 {
        5u64
    } else {
        4u64
    };
    let a6119 = {
        let a6116 = f284(v0);
        let a6117 = v1;
        nat_mul(a6116, a6117)?
    };
    nat_add(a6118, a6119)
}

pub fn f267(v0: bool, v1: u64) -> R<u64> {
    if v0 {
        let a6127 = {
            let a6120 = 13u64;
            let a6121 = f285(v1);
            nat_mul(a6120, a6121)?
        };
        let a6128 = {
            let a6125 = 4u64;
            let a6126 = {
                let a6122 = v1;
                let a6123 = 4u64;
                let a6124 = 0u64;
                nat_rem(a6122, a6123, a6124)
            };
            nat_mul(a6125, a6126)?
        };
        nat_add(a6127, a6128)
    } else {
        let a6129 = 3u64;
        let a6130 = v1;
        nat_mul(a6129, a6130)
    }
}

pub fn f268(v0: bool) -> u64 {
    if v0 {
        6u64
    } else {
        5u64
    }
}

pub fn f269(v0: u64, v1: u64) -> R<u64> {
    let a6135 = {
        let a6133 = {
            let a6131 = v0;
            let a6132 = v1;
            nat_add(a6131, a6132)?
        };
        let a6134 = 1u64;
        nat_sub(a6133, a6134)
    };
    let a6136 = v1;
    let a6137 = 0u64;
    Ok(nat_quot(a6135, a6136, a6137))
}

pub fn f270(v0: u64, v1: u64) -> R<u64> {
    let a6138 = v0;
    let a6139 = f269(v0, v1)?;
    let a6140 = 0u64;
    Ok(nat_quot(a6138, a6139, a6140))
}

pub fn f271(v0: u64, v1: u64, v2: u64) -> R<u64> {
    let a6141 = v2;
    let a6142 = 0u64;
    if nat_eq(a6141, a6142) {
        Ok(0u64)
    } else {
        let a6147 = 11u64;
        let a6148 = {
            let a6145 = {
                let a6143 = v1;
                let a6144 = v2;
                nat_mul(a6143, a6144)?
            };
            let a6146 = f286(v0)?;
            nat_mul(a6145, a6146)?
        };
        nat_add(a6147, a6148)
    }
}

pub fn f272(v0: u64, v1: u64) -> R<u64> {
    let a6149 = v0;
    let a6150 = f269(v0, v1)?;
    let a6151 = 0u64;
    Ok(nat_rem(a6149, a6150, a6151))
}

pub fn f273(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64) -> bool {
    if f252(v1, v2, v3, v4) {
        f287(v0)
    } else {
        f204(v0)
    }
}

pub fn f274(v0: u64) -> u64 {
    let a6152 = v0;
    let a6153 = 26u64;
    if nat_eq(a6152, a6153) {
        26u64
    } else {
        0u64
    }
}

pub fn f275(v0: u64, v1: u64) -> Option<u64> {
    let a6154 = v0;
    let a6155 = 2u64;
    if nat_eq(a6154, a6155) {
        Some(0u64)
    } else {
        let a6156 = v0;
        let a6157 = 5u64;
        if nat_eq(a6156, a6157) {
            Some(1u64)
        } else {
            let a6158 = v0;
            let a6159 = 6u64;
            if nat_eq(a6158, a6159) {
                Some(2u64)
            } else {
                let a6160 = v0;
                let a6161 = 7u64;
                if nat_eq(a6160, a6161) {
                    Some(3u64)
                } else {
                    let a6162 = v0;
                    let a6163 = 9u64;
                    if nat_eq(a6162, a6163) {
                        Some(4u64)
                    } else {
                        let a6164 = v0;
                        let a6165 = 8u64;
                        if nat_eq(a6164, a6165) {
                            Some(5u64)
                        } else {
                            let a6166 = v0;
                            let a6167 = 89u64;
                            if nat_eq(a6166, a6167) {
                                let a6168 = v1;
                                let a6169 = 2u64;
                                if nat_eq(a6168, a6169) {
                                    Some(6u64)
                                } else {
                                    let a6170 = v1;
                                    let a6171 = 0u64;
                                    if nat_eq(a6170, a6171) {
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

pub fn f276(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6172 = v0;
    let a6173 = 30u64;
    if nat_eq(a6172, a6173) {
        let a6174 = v2;
        let a6175 = 0u64;
        if if if nat_eq(a6174, a6175) {
            true
        } else {
            let a6176 = v2;
            let a6177 = 1u64;
            nat_eq(a6176, a6177)
        } {
            let a6178 = v1;
            let a6179 = 0u64;
            if nat_eq(a6178, a6179) {
                true
            } else {
                f288(v1, v4)
            }
        } else {
            false
        } {
            f289(28u64, 0u64, 0u64, v3, v5, v4, v6, v7, v9, v8, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    } else {
        if if if if f290(v1) {
            f290(v2)
        } else {
            false
        } {
            let a6180 = v1;
            let a6181 = 0u64;
            if if nat_eq(a6180, a6181) {
                true
            } else {
                let a6182 = v2;
                let a6183 = 0u64;
                nat_eq(a6182, a6183)
            } {
                true
            } else {
                let a6184 = v1;
                let a6185 = v2;
                nat_eq(a6184, a6185)
            }
        } else {
            false
        } {
            true
        } else {
            let a6186 = v1;
            let a6187 = 43u64;
            if nat_eq(a6186, a6187) {
                f290(v2)
            } else {
                false
            }
        } {
            f289(v0, 0u64, 0u64, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
        } else {
            Ok(Adt22::C0)
        }
    }
}

pub fn f277(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: u64) -> R<Adt17> {
    let v6: u64 = match v2.clone() {
        Adt20::C0(h6188, _, _, _, _, _, _, _) => {
            h6188
        }
    };
    let v7: u64 = {
        let a6191 = 1u64;
        let a6192 = {
            let a6189 = 3u64;
            let a6190 = v6;
            nat_mul(a6189, a6190)?
        };
        nat_add(a6191, a6192)?
    };
    let a6193 = v0;
    let a6194 = 0u64;
    if nat_eq(a6193, a6194) {
        Ok(f291(0u64))
    } else {
        let a6195 = v0;
        let a6196 = v7;
        if nat_lt(a6195, a6196) {
            let v8: u64 = {
                let a6197 = v0;
                let a6198 = 1u64;
                nat_sub(a6197, a6198)
            };
            let v9: u64 = {
                let a6204 = {
                    let a6199 = v1;
                    let a6200 = v6;
                    nat_mul(a6199, a6200)?
                };
                let a6205 = {
                    let a6201 = v8;
                    let a6202 = 3u64;
                    let a6203 = 0u64;
                    nat_quot(a6201, a6202, a6203)
                };
                nat_add(a6204, a6205)?
            };
            let v10: u64 = f292(v9, v2)?;
            let v11: u64 = {
                let a6206 = v8;
                let a6207 = 3u64;
                let a6208 = 0u64;
                nat_rem(a6206, a6207, a6208)
            };
            let a6209 = v11;
            let a6210 = 0u64;
            if nat_eq(a6209, a6210) {
                Ok(f293(10u64, 0u64, v10, 0u64, v10, 1u64, 1u64))
            } else {
                let a6211 = v11;
                let a6212 = 1u64;
                if nat_eq(a6211, a6212) {
                    Ok(f293(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f293(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            }
        } else {
            let a6213 = v0;
            let a6214 = v7;
            if nat_eq(a6213, a6214) {
                Ok(f293(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 1u64))
            } else {
                let a6217 = v0;
                let a6218 = {
                    let a6215 = v7;
                    let a6216 = 1u64;
                    nat_add(a6215, a6216)?
                };
                if nat_eq(a6217, a6218) {
                    Ok(f293(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6221 = v0;
                    let a6222 = {
                        let a6219 = v7;
                        let a6220 = 2u64;
                        nat_add(a6219, a6220)?
                    };
                    if nat_eq(a6221, a6222) {
                        Ok(f293(16u64, 1u64, 1u64, 2u64, v4, 1u64, 2u64))
                    } else {
                        let a6225 = v0;
                        let a6226 = {
                            let a6223 = v7;
                            let a6224 = 3u64;
                            nat_add(a6223, a6224)?
                        };
                        if nat_eq(a6225, a6226) {
                            let a6227 = v5;
                            let a6228 = 1u64;
                            if if nat_eq(a6227, a6228) {
                                true
                            } else {
                                let a6229 = v5;
                                let a6230 = 2u64;
                                nat_eq(a6229, a6230)
                            } {
                                Ok(f293(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 2u64))
                            } else {
                                Ok(f293(11u64, 2u64, 1065353216u64, 2u64, v4, 1u64, 2u64))
                            }
                        } else {
                            let v12: u64 = {
                                let a6237 = {
                                    let a6231 = v1;
                                    let a6232 = v6;
                                    nat_mul(a6231, a6232)?
                                };
                                let a6238 = {
                                    let a6235 = v0;
                                    let a6236 = {
                                        let a6233 = v7;
                                        let a6234 = 4u64;
                                        nat_add(a6233, a6234)?
                                    };
                                    nat_sub(a6235, a6236)
                                };
                                nat_add(a6237, a6238)?
                            };
                            Ok(f293(10u64, 0u64, f292(v12, v2)?, 1u64, 2u64, 3u64, f292(v12, v3)?))
                        }
                    }
                }
            }
        }
    }
}

pub fn f278(v0: Adt20) -> Adt20 {
    Adt20::C0(match v0.clone() {
        Adt20::C0(h6239, _, _, _, _, _, _, _) => {
            h6239
        }
    }, match v0.clone() {
        Adt20::C0(_, h6240, _, _, _, _, _, _) => {
            h6240
        }
    }, match v0.clone() {
        Adt20::C0(_, _, h6241, _, _, _, _, _) => {
            h6241
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, h6242, _, _, _, _) => {
            h6242
        }
    }, 4u64, match v0.clone() {
        Adt20::C0(_, _, _, _, _, h6243, _, _) => {
            h6243
        }
    }, match v0.clone() {
        Adt20::C0(_, _, _, _, _, _, h6244, _) => {
            h6244
        }
    }, match v0 {
        Adt20::C0(_, _, _, _, _, _, _, h6245) => {
            h6245
        }
    })
}

pub fn f279(v0: u64, v1: Adt20, v2: u64, v3: u64, v4: u64) -> R<Adt17> {
    let v5: u64 = {
        let a6252 = v3;
        let a6253 = {
            let a6249 = f292({
                let a6246 = v0;
                let a6247 = 2u64;
                let a6248 = 0u64;
                nat_quot(a6246, a6247, a6248)
            }, v1)?;
            let a6250 = v2;
            let a6251 = 0u64;
            nat_quot(a6249, a6250, a6251)
        };
        nat_add(a6252, a6253)?
    };
    let a6257 = {
        let a6254 = v0;
        let a6255 = 2u64;
        let a6256 = 0u64;
        nat_rem(a6254, a6255, a6256)
    };
    let a6258 = 0u64;
    if nat_eq(a6257, a6258) {
        Ok(f293(19u64, 1u64, v5, 2u64, 0u64, 1u64, 2u64))
    } else {
        let a6259 = v4;
        let a6260 = 0u64;
        if nat_eq(a6259, a6260) {
            Ok(f293(19u64, 2u64, 1u64, 2u64, 0u64, 1u64, v5))
        } else {
            Ok(Adt17::C0)
        }
    }
}

pub fn f280(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20, v5: u64) -> R<Adt17> {
    let v6: u64 = f294(match v2.clone() {
        Adt20::C0(h6261, _, _, _, _, _, _, _) => {
            h6261
        }
    })?;
    let v7: u64 = f295(match v2.clone() {
        Adt20::C0(h6262, _, _, _, _, _, _, _) => {
            h6262
        }
    })?;
    let a6263 = v0;
    let a6264 = 2u64;
    if nat_lt(a6263, a6264) {
        Ok(f291(v0))
    } else {
        let a6265 = v0;
        let a6266 = v6;
        if nat_lt(a6265, a6266) {
            f296({
                let a6267 = v0;
                let a6268 = 2u64;
                nat_sub(a6267, a6268)
            }, v1, v2, v3)
        } else {
            let a6269 = v0;
            let a6270 = v7;
            if nat_lt(a6269, a6270) {
                Ok(f297({
                    let a6271 = v0;
                    let a6272 = v6;
                    nat_sub(a6271, a6272)
                }, match v2 {
                    Adt20::C0(h6273, _, _, _, _, _, _, _) => {
                        h6273
                    }
                }, v5))
            } else {
                let a6275 = v0;
                let a6276 = f250(match v2.clone() {
                    Adt20::C0(h6274, _, _, _, _, _, _, _) => {
                        h6274
                    }
                })?;
                if nat_lt(a6275, a6276) {
                    f298({
                        let a6277 = v0;
                        let a6278 = v7;
                        nat_sub(a6277, a6278)
                    }, v1, v2, v3, v4)
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f281(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: u64, v5: bool) -> R<Adt17> {
    let v6: u64 = f266(v5, match v2.clone() {
        Adt20::C0(h6279, _, _, _, _, _, _, _) => {
            h6279
        }
    })?;
    let v7: u64 = {
        let a6281 = v6;
        let a6282 = f267(v5, match v2.clone() {
            Adt20::C0(h6280, _, _, _, _, _, _, _) => {
                h6280
            }
        })?;
        nat_add(a6281, a6282)?
    };
    let v8: u64 = {
        let a6283 = v7;
        let a6284 = f268(v5);
        nat_add(a6283, a6284)?
    };
    let a6285 = v0;
    let a6286 = v6;
    if nat_lt(a6285, a6286) {
        f299(v0, v1, v2, v5)
    } else {
        let a6287 = v0;
        let a6288 = v7;
        if nat_lt(a6287, a6288) {
            f300({
                let a6289 = v0;
                let a6290 = v6;
                nat_sub(a6289, a6290)
            }, v1, v2, v3, v5)
        } else {
            let a6291 = v0;
            let a6292 = v8;
            if nat_lt(a6291, a6292) {
                Ok(f301({
                    let a6293 = v0;
                    let a6294 = v7;
                    nat_sub(a6293, a6294)
                }, match v2 {
                    Adt20::C0(h6295, _, _, _, _, _, _, _) => {
                        h6295
                    }
                }, v4, v5))
            } else {
                let a6299 = v0;
                let a6300 = {
                    let a6297 = v8;
                    let a6298 = match v2.clone() {
                        Adt20::C0(h6296, _, _, _, _, _, _, _) => {
                            h6296
                        }
                    };
                    nat_add(a6297, a6298)?
                };
                if nat_lt(a6299, a6300) {
                    let v9: u64 = f292({
                        let a6306 = {
                            let a6302 = v1;
                            let a6303 = match v2 {
                                Adt20::C0(h6301, _, _, _, _, _, _, _) => {
                                    h6301
                                }
                            };
                            nat_mul(a6302, a6303)?
                        };
                        let a6307 = {
                            let a6304 = v0;
                            let a6305 = v8;
                            nat_sub(a6304, a6305)
                        };
                        nat_add(a6306, a6307)?
                    }, v3)?;
                    Ok(f293(10u64, 3u64, v9, 1u64, 1u64, 3u64, v9))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f282(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64) -> R<Adt21> {
    let v6: u64 = f269(v3, v5)?;
    let v7: u64 = f271(v1, v2, v6)?;
    let v8: u64 = {
        let a6310 = {
            let a6308 = f270(v3, v5)?;
            let a6309 = v4;
            nat_mul(a6308, a6309)?
        };
        let a6311 = v7;
        nat_mul(a6310, a6311)?
    };
    let a6312 = v0;
    let a6313 = v8;
    if nat_lt(a6312, a6313) {
        let v9: u64 = {
            let a6314 = v0;
            let a6315 = v7;
            let a6316 = 0u64;
            nat_quot(a6314, a6315, a6316)
        };
        Ok(Adt21::C0({
            let a6320 = {
                let a6317 = v9;
                let a6318 = v4;
                let a6319 = 0u64;
                nat_quot(a6317, a6318, a6319)
            };
            let a6321 = v6;
            nat_mul(a6320, a6321)?
        }, v6, {
            let a6322 = v9;
            let a6323 = v4;
            let a6324 = 0u64;
            nat_rem(a6322, a6323, a6324)
        }, {
            let a6325 = v0;
            let a6326 = v7;
            let a6327 = 0u64;
            nat_rem(a6325, a6326, a6327)
        }))
    } else {
        let v10: u64 = {
            let a6328 = v0;
            let a6329 = v8;
            nat_sub(a6328, a6329)
        };
        let v11: u64 = f271(v1, v2, f272(v3, v5)?)?;
        Ok(Adt21::C0({
            let a6330 = f270(v3, v5)?;
            let a6331 = v6;
            nat_mul(a6330, a6331)?
        }, f272(v3, v5)?, {
            let a6332 = v10;
            let a6333 = v11;
            let a6334 = 0u64;
            nat_quot(a6332, a6333, a6334)
        }, {
            let a6335 = v10;
            let a6336 = v11;
            let a6337 = 0u64;
            nat_rem(a6335, a6336, a6337)
        }))
    }
}

pub fn f283(v0: Adt21, v1: Adt20, v2: Adt20, v3: u64) -> R<Adt17> {
    let v4: u64 = match v0.clone() {
        Adt21::C0(_, _, _, h6338) => {
            h6338
        }
    };
    let v5: u64 = match v1.clone() {
        Adt20::C0(h6339, _, _, _, _, _, _, _) => {
            h6339
        }
    };
    let v6: u64 = {
        let a6342 = match v0.clone() {
            Adt21::C0(_, h6340, _, _) => {
                h6340
            }
        };
        let a6343 = match v1.clone() {
            Adt20::C0(_, h6341, _, _, _, _, _, _) => {
                h6341
            }
        };
        nat_mul(a6342, a6343)?
    };
    let v7: u64 = {
        let a6344 = v6;
        let a6345 = v5;
        nat_mul(a6344, a6345)?
    };
    let v8: u64 = {
        let a6348 = {
            let a6346 = 2u64;
            let a6347 = v5;
            nat_mul(a6346, a6347)?
        };
        let a6349 = 2u64;
        nat_add(a6348, a6349)?
    };
    let v9: u64 = {
        let a6352 = {
            let a6350 = 4u64;
            let a6351 = v5;
            nat_mul(a6350, a6351)?
        };
        let a6353 = 2u64;
        nat_add(a6352, a6353)?
    };
    let v10: u64 = {
        let a6356 = 1u64;
        let a6357 = {
            let a6354 = v6;
            let a6355 = v8;
            nat_mul(a6354, a6355)?
        };
        nat_add(a6356, a6357)?
    };
    let v11: u64 = {
        let a6358 = v10;
        let a6359 = 4u64;
        nat_add(a6358, a6359)?
    };
    let v12: u64 = {
        let a6362 = v11;
        let a6363 = {
            let a6360 = v6;
            let a6361 = v9;
            nat_mul(a6360, a6361)?
        };
        nat_add(a6362, a6363)?
    };
    let v13: u64 = {
        let a6364 = v12;
        let a6365 = 6u64;
        nat_add(a6364, a6365)?
    };
    let a6366 = v4;
    let a6367 = 0u64;
    if nat_eq(a6366, a6367) {
        Ok(f291(0u64))
    } else {
        let a6368 = v4;
        let a6369 = v10;
        if nat_lt(a6368, a6369) {
            let v14: u64 = {
                let a6370 = v4;
                let a6371 = 1u64;
                nat_sub(a6370, a6371)
            };
            f302(false, {
                let a6372 = v14;
                let a6373 = v8;
                let a6374 = 0u64;
                nat_rem(a6372, a6373, a6374)
            }, {
                let a6375 = v14;
                let a6376 = v8;
                let a6377 = 0u64;
                nat_quot(a6375, a6376, a6377)
            }, v0, v1, v2)
        } else {
            let a6378 = v4;
            let a6379 = v11;
            if nat_lt(a6378, a6379) {
                Ok(f303({
                    let a6380 = v4;
                    let a6381 = v10;
                    nat_sub(a6380, a6381)
                }, v7))
            } else {
                let a6382 = v4;
                let a6383 = v12;
                if nat_lt(a6382, a6383) {
                    let v15: u64 = {
                        let a6384 = v4;
                        let a6385 = v11;
                        nat_sub(a6384, a6385)
                    };
                    f302(true, {
                        let a6386 = v15;
                        let a6387 = v9;
                        let a6388 = 0u64;
                        nat_rem(a6386, a6387, a6388)
                    }, {
                        let a6389 = v15;
                        let a6390 = v9;
                        let a6391 = 0u64;
                        nat_quot(a6389, a6390, a6391)
                    }, v0, v1, v2)
                } else {
                    let a6392 = v4;
                    let a6393 = v13;
                    if nat_lt(a6392, a6393) {
                        Ok(f304({
                            let a6394 = v4;
                            let a6395 = v12;
                            nat_sub(a6394, a6395)
                        }, v7, v3))
                    } else {
                        let a6398 = v4;
                        let a6399 = {
                            let a6396 = v13;
                            let a6397 = v7;
                            nat_add(a6396, a6397)?
                        };
                        if nat_lt(a6398, a6399) {
                            let v16: u64 = {
                                let a6400 = v4;
                                let a6401 = v13;
                                nat_sub(a6400, a6401)
                            };
                            let v17: u64 = {
                                let a6402 = v16;
                                let a6403 = v5;
                                let a6404 = 0u64;
                                nat_quot(a6402, a6403, a6404)
                            };
                            let v18: u64 = {
                                let a6410 = {
                                    let a6405 = f305(v17, v0, v1)?;
                                    let a6406 = v5;
                                    nat_mul(a6405, a6406)?
                                };
                                let a6411 = {
                                    let a6407 = v16;
                                    let a6408 = v5;
                                    let a6409 = 0u64;
                                    nat_rem(a6407, a6408, a6409)
                                };
                                nat_add(a6410, a6411)?
                            };
                            let v19: u64 = f292(v18, v2)?;
                            Ok(f293(10u64, 3u64, v19, 1u64, 1u64, 3u64, v19))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f284(v0: bool) -> u64 {
    if v0 {
        2u64
    } else {
        1u64
    }
}

pub fn f285(v0: u64) -> u64 {
    let a6412 = v0;
    let a6413 = 4u64;
    let a6414 = 0u64;
    nat_quot(a6412, a6413, a6414)
}

pub fn f286(v0: u64) -> R<u64> {
    let a6417 = {
        let a6415 = 7u64;
        let a6416 = v0;
        nat_mul(a6415, a6416)?
    };
    let a6418 = 4u64;
    nat_add(a6417, a6418)
}

pub fn f287(v0: u64) -> bool {
    let m6419 = f197(v0);
    match m6419 {
        Adt18::C0 => {
            false
        }
        Adt18::C1(_) => {
            true
        }
    }
}

pub fn f288(v0: u64, v1: u64) -> bool {
    let a6420 = v0;
    let a6421 = 1u64;
    if nat_eq(a6420, a6421) {
        true
    } else {
        let v2: u64 = f306(v0);
        let a6422 = 0u64;
        let a6423 = v2;
        if nat_lt(a6422, a6423) {
            let a6427 = {
                let a6424 = v1;
                let a6425 = v2;
                let a6426 = 1u64;
                nat_rem(a6424, a6425, a6426)
            };
            let a6428 = 0u64;
            nat_eq(a6427, a6428)
        } else {
            false
        }
    }
}

pub fn f289(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64, v7: u64, v8: u64, v9: u64, v10: u64, v11: u64, v12: u64, v13: u64, v14: u64, v15: u64) -> R<Adt22> {
    let a6429 = v0;
    let a6430 = 28u64;
    if if if if nat_eq(a6429, a6430) {
        let a6431 = v1;
        let a6432 = 0u64;
        nat_eq(a6431, a6432)
    } else {
        false
    } {
        let a6433 = v2;
        let a6434 = 0u64;
        nat_eq(a6433, a6434)
    } else {
        false
    } {
        let a6435 = v3;
        let a6436 = 0u64;
        nat_eq(a6435, a6436)
    } else {
        false
    } {
        let a6437 = v4;
        let a6438 = v8;
        if if if if if if if if nat_eq(a6437, a6438) {
            let a6439 = v5;
            let a6440 = v12;
            nat_eq(a6439, a6440)
        } else {
            false
        } {
            let a6441 = v9;
            let a6442 = v13;
            nat_eq(a6441, a6442)
        } else {
            false
        } {
            let a6443 = v10;
            let a6444 = v14;
            nat_eq(a6443, a6444)
        } else {
            false
        } {
            let a6445 = v11;
            let a6446 = v15;
            nat_eq(a6445, a6446)
        } else {
            false
        } {
            let a6447 = 0u64;
            let a6448 = v6;
            nat_lt(a6447, a6448)
        } else {
            false
        } {
            let a6449 = 0u64;
            let a6450 = v7;
            nat_lt(a6449, a6450)
        } else {
            false
        } {
            let a6454 = {
                let a6451 = v10;
                let a6452 = v6;
                let a6453 = 1u64;
                nat_rem(a6451, a6452, a6453)
            };
            let a6455 = 0u64;
            if nat_eq(a6454, a6455) {
                let a6459 = {
                    let a6456 = v11;
                    let a6457 = v7;
                    let a6458 = 1u64;
                    nat_rem(a6456, a6457, a6458)
                };
                let a6460 = 0u64;
                nat_eq(a6459, a6460)
            } else {
                false
            }
        } else {
            false
        } {
            Ok(Adt22::C2(v5, v4, v9, v10, v11, {
                let a6461 = v10;
                let a6462 = v6;
                let a6463 = 0u64;
                nat_quot(a6461, a6462, a6463)
            }, {
                let a6464 = v11;
                let a6465 = v7;
                let a6466 = 0u64;
                nat_quot(a6464, a6465, a6466)
            }, {
                let a6467 = v5;
                let a6468 = v4;
                nat_mul(a6467, a6468)?
            }, {
                let a6469 = v4;
                let a6470 = v9;
                nat_mul(a6469, a6470)?
            }, {
                let a6471 = v5;
                let a6472 = v9;
                nat_mul(a6471, a6472)?
            }))
        } else {
            Ok(Adt22::C1)
        }
    } else {
        Ok(Adt22::C0)
    }
}

pub fn f290(v0: u64) -> bool {
    let a6473 = v0;
    let a6474 = 0u64;
    if if nat_eq(a6473, a6474) {
        true
    } else {
        let a6475 = v0;
        let a6476 = 1u64;
        nat_eq(a6475, a6476)
    } {
        true
    } else {
        let a6477 = v0;
        let a6478 = 30u64;
        nat_eq(a6477, a6478)
    }
}

pub fn f291(v0: u64) -> Adt17 {
    f293(0u64, 2u64, 0u64, 2u64, 0u64, 1u64, v0)
}

pub fn f292(v0: u64, v1: Adt20) -> R<u64> {
    let v2: u64 = {
        let a6480 = v0;
        let a6481 = match v1.clone() {
            Adt20::C0(h6479, _, _, _, _, _, _, _) => {
                h6479
            }
        };
        let a6482 = 0u64;
        nat_quot(a6480, a6481, a6482)
    };
    let v3: u64 = {
        let a6484 = v2;
        let a6485 = match v1.clone() {
            Adt20::C0(_, h6483, _, _, _, _, _, _) => {
                h6483
            }
        };
        let a6486 = 0u64;
        nat_quot(a6484, a6485, a6486)
    };
    f126({
        let a6488 = v0;
        let a6489 = match v1.clone() {
            Adt20::C0(h6487, _, _, _, _, _, _, _) => {
                h6487
            }
        };
        let a6490 = 0u64;
        nat_rem(a6488, a6489, a6490)
    }, {
        let a6492 = v2;
        let a6493 = match v1.clone() {
            Adt20::C0(_, h6491, _, _, _, _, _, _) => {
                h6491
            }
        };
        let a6494 = 0u64;
        nat_rem(a6492, a6493, a6494)
    }, {
        let a6496 = v3;
        let a6497 = match v1.clone() {
            Adt20::C0(_, _, h6495, _, _, _, _, _) => {
                h6495
            }
        };
        let a6498 = 0u64;
        nat_rem(a6496, a6497, a6498)
    }, {
        let a6500 = v3;
        let a6501 = match v1.clone() {
            Adt20::C0(_, _, h6499, _, _, _, _, _) => {
                h6499
            }
        };
        let a6502 = 0u64;
        nat_quot(a6500, a6501, a6502)
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, h6503, _, _, _) => {
            h6503
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, h6504, _, _) => {
            h6504
        }
    }, match v1.clone() {
        Adt20::C0(_, _, _, _, _, _, h6505, _) => {
            h6505
        }
    }, match v1 {
        Adt20::C0(_, _, _, _, _, _, _, h6506) => {
            h6506
        }
    })
}

pub fn f293(v0: u64, v1: u64, v2: u64, v3: u64, v4: u64, v5: u64, v6: u64) -> Adt17 {
    let a6507 = v0;
    let a6508 = 4u64;
    if if if nat_le(a6507, a6508) {
        true
    } else {
        let a6509 = 9u64;
        let a6510 = v0;
        if nat_le(a6509, a6510) {
            let a6511 = v0;
            let a6512 = 12u64;
            nat_le(a6511, a6512)
        } else {
            false
        }
    } {
        true
    } else {
        let a6513 = v0;
        let a6514 = 16u64;
        if if nat_eq(a6513, a6514) {
            true
        } else {
            let a6515 = v0;
            let a6516 = 19u64;
            nat_eq(a6515, a6516)
        } {
            true
        } else {
            let a6517 = v0;
            let a6518 = 21u64;
            if nat_eq(a6517, a6518) {
                true
            } else {
                let a6519 = v0;
                let a6520 = 22u64;
                nat_eq(a6519, a6520)
            }
        }
    } {
        Adt17::C1({
            let a6521 = v0;
            let a6522 = 4u64;
            if if nat_lt(a6521, a6522) {
                true
            } else {
                let a6523 = 21u64;
                let a6524 = v0;
                nat_le(a6523, a6524)
            } {
                0u64
            } else {
                let a6525 = v0;
                let a6526 = 16u64;
                if if nat_eq(a6525, a6526) {
                    true
                } else {
                    let a6527 = v0;
                    let a6528 = 19u64;
                    nat_eq(a6527, a6528)
                } {
                    2u64
                } else {
                    1u64
                }
            }
        }, {
            let a6529 = v0;
            let a6530 = 4u64;
            if nat_lt(a6529, a6530) {
                v0
            } else {
                let a6531 = v0;
                let a6532 = 4u64;
                if if nat_eq(a6531, a6532) {
                    true
                } else {
                    let a6533 = v0;
                    let a6534 = 16u64;
                    nat_eq(a6533, a6534)
                } {
                    0u64
                } else {
                    let a6535 = v0;
                    let a6536 = 19u64;
                    if nat_eq(a6535, a6536) {
                        3u64
                    } else {
                        let a6537 = 21u64;
                        let a6538 = v0;
                        if nat_le(a6537, a6538) {
                            let a6539 = v0;
                            let a6540 = 17u64;
                            nat_sub(a6539, a6540)
                        } else {
                            let a6541 = v0;
                            let a6542 = 8u64;
                            nat_sub(a6541, a6542)
                        }
                    }
                }
            }
        }, v1, v2, v3, v4, v5, v6)
    } else {
        Adt17::C0
    }
}

pub fn f294(v0: u64) -> R<u64> {
    let a6545 = 2u64;
    let a6546 = {
        let a6543 = 6u64;
        let a6544 = v0;
        nat_mul(a6543, a6544)?
    };
    nat_add(a6545, a6546)
}

pub fn f295(v0: u64) -> R<u64> {
    let a6547 = f294(v0)?;
    let a6548 = 11u64;
    nat_add(a6547, a6548)
}

pub fn f296(v0: u64, v1: u64, v2: Adt20, v3: Adt20) -> R<Adt17> {
    let v4: u64 = {
        let a6549 = v0;
        let a6550 = 6u64;
        let a6551 = 0u64;
        nat_rem(a6549, a6550, a6551)
    };
    let v5: u64 = {
        let a6558 = {
            let a6553 = v1;
            let a6554 = match v2.clone() {
                Adt20::C0(h6552, _, _, _, _, _, _, _) => {
                    h6552
                }
            };
            nat_mul(a6553, a6554)?
        };
        let a6559 = {
            let a6555 = v0;
            let a6556 = 6u64;
            let a6557 = 0u64;
            nat_quot(a6555, a6556, a6557)
        };
        nat_add(a6558, a6559)?
    };
    let v6: u64 = f292(v5, v3)?;
    let v7: u64 = f292(v5, v2)?;
    let a6560 = v4;
    let a6561 = 0u64;
    if nat_eq(a6560, a6561) {
        Ok(f293(10u64, 4u64, v6, 4u64, v6, 1u64, 2u64))
    } else {
        let a6562 = v4;
        let a6563 = 1u64;
        if nat_eq(a6562, a6563) {
            Ok(f293(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
        } else {
            let a6564 = v4;
            let a6565 = 2u64;
            if nat_eq(a6564, a6565) {
                Ok(f293(0u64, 1u64, 0u64, 1u64, 2u64, 1u64, 0u64))
            } else {
                let a6566 = v4;
                let a6567 = 3u64;
                if nat_eq(a6566, a6567) {
                    Ok(f293(10u64, 4u64, v6, 0u64, v7, 1u64, 2u64))
                } else {
                    let a6568 = v4;
                    let a6569 = 4u64;
                    if nat_eq(a6568, a6569) {
                        Ok(f293(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                    } else {
                        Ok(f293(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                    }
                }
            }
        }
    }
}

pub fn f297(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6570 = v0;
    let a6571 = 0u64;
    if nat_eq(a6570, a6571) {
        f293(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 3u64)
    } else {
        let a6572 = v0;
        let a6573 = 1u64;
        if nat_eq(a6572, a6573) {
            f293(3u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
        } else {
            let a6574 = v0;
            let a6575 = 2u64;
            if nat_eq(a6574, a6575) {
                f293(11u64, 1u64, 3u64, 1u64, 2u64, 1u64, 4u64)
            } else {
                let a6576 = v0;
                let a6577 = 3u64;
                if nat_eq(a6576, a6577) {
                    f293(4u64, 1u64, 4u64, 2u64, v2, 1u64, 4u64)
                } else {
                    let a6578 = v0;
                    let a6579 = 4u64;
                    if nat_eq(a6578, a6579) {
                        f293(12u64, 1u64, 4u64, 2u64, 0u64, 1u64, 4u64)
                    } else {
                        let a6580 = v0;
                        let a6581 = 5u64;
                        if nat_eq(a6580, a6581) {
                            f293(11u64, 2u64, 1065353216u64, 1u64, 4u64, 1u64, 4u64)
                        } else {
                            let a6582 = v0;
                            let a6583 = 6u64;
                            if nat_eq(a6582, a6583) {
                                f293(10u64, 2u64, v2, 1u64, 2u64, 1u64, 5u64)
                            } else {
                                let a6584 = v0;
                                let a6585 = 7u64;
                                if nat_eq(a6584, a6585) {
                                    f293(4u64, 1u64, 3u64, 1u64, 5u64, 1u64, 5u64)
                                } else {
                                    let a6586 = v0;
                                    let a6587 = 8u64;
                                    if nat_eq(a6586, a6587) {
                                        f293(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 2u64)
                                    } else {
                                        let a6588 = v0;
                                        let a6589 = 9u64;
                                        if nat_eq(a6588, a6589) {
                                            Adt17::C1(3u64, 0u64, 1u64, 2u64, 2u64, 2u64, 1u64, 2u64)
                                        } else {
                                            let a6590 = v0;
                                            let a6591 = 10u64;
                                            if nat_eq(a6590, a6591) {
                                                f293(11u64, 1u64, 2u64, 1u64, 5u64, 1u64, 5u64)
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

pub fn f298(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6592 = v0;
        let a6593 = 3u64;
        let a6594 = 0u64;
        nat_rem(a6592, a6593, a6594)
    };
    let v6: u64 = {
        let a6601 = {
            let a6596 = v1;
            let a6597 = match v2.clone() {
                Adt20::C0(h6595, _, _, _, _, _, _, _) => {
                    h6595
                }
            };
            nat_mul(a6596, a6597)?
        };
        let a6602 = {
            let a6598 = v0;
            let a6599 = 3u64;
            let a6600 = 0u64;
            nat_quot(a6598, a6599, a6600)
        };
        nat_add(a6601, a6602)?
    };
    let v7: u64 = f292(v6, v3)?;
    let v8: u64 = f292(v6, v2)?;
    let v9: u64 = f292(v6, v4)?;
    let a6603 = v5;
    let a6604 = 0u64;
    if nat_eq(a6603, a6604) {
        Ok(f293(10u64, 4u64, v7, 1u64, 5u64, 1u64, 2u64))
    } else {
        let a6605 = v5;
        let a6606 = 1u64;
        if nat_eq(a6605, a6606) {
            Ok(f293(4u64, 0u64, v8, 1u64, 2u64, 1u64, 2u64))
        } else {
            Ok(f293(10u64, 1u64, 2u64, 1u64, 4u64, 3u64, v9))
        }
    }
}

pub fn f299(v0: u64, v1: u64, v2: Adt20, v3: bool) -> R<Adt17> {
    let v4: u64 = match v2.clone() {
        Adt20::C0(h6607, _, _, _, _, _, _, _) => {
            h6607
        }
    };
    let v5: u64 = {
        let a6610 = 1u64;
        let a6611 = {
            let a6608 = v4;
            let a6609 = f284(v3);
            nat_mul(a6608, a6609)?
        };
        nat_add(a6610, a6611)?
    };
    let a6612 = v0;
    let a6613 = 0u64;
    if nat_eq(a6612, a6613) {
        Ok(f291(0u64))
    } else {
        let a6614 = v0;
        let a6615 = v5;
        if nat_lt(a6614, a6615) {
            let v6: u64 = {
                let a6616 = v0;
                let a6617 = 1u64;
                nat_sub(a6616, a6617)
            };
            let v7: u64 = f292({
                let a6623 = {
                    let a6618 = v1;
                    let a6619 = v4;
                    nat_mul(a6618, a6619)?
                };
                let a6624 = {
                    let a6620 = v6;
                    let a6621 = f284(v3);
                    let a6622 = 0u64;
                    nat_quot(a6620, a6621, a6622)
                };
                nat_add(a6623, a6624)?
            }, v2)?;
            if v3 {
                let a6628 = {
                    let a6625 = v6;
                    let a6626 = 2u64;
                    let a6627 = 0u64;
                    nat_rem(a6625, a6626, a6627)
                };
                let a6629 = 0u64;
                if nat_eq(a6628, a6629) {
                    Ok(f293(1u64, 0u64, v7, 2u64, 0u64, 1u64, 1u64))
                } else {
                    Ok(f293(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                }
            } else {
                Ok(f293(4u64, 1u64, 0u64, 0u64, v7, 1u64, 0u64))
            }
        } else {
            let v8: u64 = {
                let a6630 = v0;
                let a6631 = v5;
                nat_sub(a6630, a6631)
            };
            let a6632 = v8;
            let a6633 = 0u64;
            if nat_eq(a6632, a6633) {
                Ok(f293(3u64, 2u64, v4, 2u64, 0u64, 1u64, 1u64))
            } else {
                if if v3 {
                    let a6634 = v8;
                    let a6635 = 1u64;
                    nat_eq(a6634, a6635)
                } else {
                    false
                } {
                    Ok(f293(2u64, 1u64, 0u64, 2u64, 0u64, 1u64, 0u64))
                } else {
                    let a6636 = v8;
                    let a6637 = if v3 {
                        2u64
                    } else {
                        1u64
                    };
                    if nat_eq(a6636, a6637) {
                        Ok(f293(11u64, 1u64, 0u64, 1u64, 1u64, 1u64, 3u64))
                    } else {
                        let a6638 = v8;
                        let a6639 = if v3 {
                            3u64
                        } else {
                            2u64
                        };
                        if nat_eq(a6638, a6639) {
                            Ok(f291(0u64))
                        } else {
                            Ok(Adt17::C0)
                        }
                    }
                }
            }
        }
    }
}

pub fn f300(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = if v4 {
        let a6641 = 13u64;
        let a6642 = f285(match v2.clone() {
            Adt20::C0(h6640, _, _, _, _, _, _, _) => {
                h6640
            }
        });
        nat_mul(a6641, a6642)?
    } else {
        0u64
    };
    let a6643 = v0;
    let a6644 = v5;
    if nat_lt(a6643, a6644) {
        f307({
            let a6645 = v0;
            let a6646 = 13u64;
            let a6647 = 0u64;
            nat_rem(a6645, a6646, a6647)
        }, v1, {
            let a6648 = v0;
            let a6649 = 13u64;
            let a6650 = 0u64;
            nat_quot(a6648, a6649, a6650)
        }, v2, v3)
    } else {
        let v6: u64 = {
            let a6651 = v0;
            let a6652 = v5;
            nat_sub(a6651, a6652)
        };
        let v7: u64 = if v4 {
            4u64
        } else {
            3u64
        };
        f308({
            let a6653 = v6;
            let a6654 = v7;
            let a6655 = 0u64;
            nat_rem(a6653, a6654, a6655)
        }, {
            let a6665 = {
                let a6660 = {
                    let a6657 = v1;
                    let a6658 = match v2.clone() {
                        Adt20::C0(h6656, _, _, _, _, _, _, _) => {
                            h6656
                        }
                    };
                    nat_mul(a6657, a6658)?
                };
                let a6661 = if v4 {
                    f309(match v2.clone() {
                        Adt20::C0(h6659, _, _, _, _, _, _, _) => {
                            h6659
                        }
                    })?
                } else {
                    0u64
                };
                nat_add(a6660, a6661)?
            };
            let a6666 = {
                let a6662 = v6;
                let a6663 = v7;
                let a6664 = 0u64;
                nat_quot(a6662, a6663, a6664)
            };
            nat_add(a6665, a6666)?
        }, v2, v3, v4)
    }
}

pub fn f301(v0: u64, v1: u64, v2: u64, v3: bool) -> Adt17 {
    let a6667 = v0;
    let a6668 = 0u64;
    if nat_eq(a6667, a6668) {
        f293(if v3 {
            22u64
        } else {
            3u64
        }, 2u64, v1, 2u64, 0u64, 1u64, 1u64)
    } else {
        let a6669 = v0;
        let a6670 = 1u64;
        if nat_eq(a6669, a6670) {
            f293(if v3 {
                21u64
            } else {
                11u64
            }, 1u64, 0u64, 1u64, 1u64, 1u64, 1u64)
        } else {
            if if v3 {
                let a6671 = v0;
                let a6672 = 2u64;
                nat_eq(a6671, a6672)
            } else {
                false
            } {
                f293(2u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
            } else {
                let a6673 = v0;
                let a6674 = if v3 {
                    3u64
                } else {
                    2u64
                };
                if nat_eq(a6673, a6674) {
                    f293(4u64, 1u64, 1u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6675 = v0;
                    let a6676 = if v3 {
                        4u64
                    } else {
                        3u64
                    };
                    if nat_eq(a6675, a6676) {
                        f293(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6677 = v0;
                        let a6678 = if v3 {
                            5u64
                        } else {
                            4u64
                        };
                        if nat_eq(a6677, a6678) {
                            f293(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f302(v0: bool, v1: u64, v2: u64, v3: Adt21, v4: Adt20, v5: Adt20) -> R<Adt17> {
    let v6: u64 = if v0 {
        4u64
    } else {
        2u64
    };
    let v7: u64 = {
        let a6682 = 1u64;
        let a6683 = {
            let a6680 = match v4.clone() {
                Adt20::C0(h6679, _, _, _, _, _, _, _) => {
                    h6679
                }
            };
            let a6681 = v6;
            nat_mul(a6680, a6681)?
        };
        nat_add(a6682, a6683)?
    };
    let a6684 = v1;
    let a6685 = 0u64;
    if nat_eq(a6684, a6685) {
        Ok(f291(1u64))
    } else {
        let a6686 = v1;
        let a6687 = v7;
        if nat_lt(a6686, a6687) {
            let v8: u64 = {
                let a6688 = v1;
                let a6689 = 1u64;
                nat_sub(a6688, a6689)
            };
            let v9: u64 = {
                let a6696 = {
                    let a6691 = f305(v2, v3, v4.clone())?;
                    let a6692 = match v4.clone() {
                        Adt20::C0(h6690, _, _, _, _, _, _, _) => {
                            h6690
                        }
                    };
                    nat_mul(a6691, a6692)?
                };
                let a6697 = {
                    let a6693 = v8;
                    let a6694 = v6;
                    let a6695 = 0u64;
                    nat_quot(a6693, a6694, a6695)
                };
                nat_add(a6696, a6697)?
            };
            let v10: u64 = f292(v9, v4)?;
            let v11: u64 = f292(v9, v5)?;
            let v12: u64 = {
                let a6698 = v8;
                let a6699 = v6;
                let a6700 = 0u64;
                nat_rem(a6698, a6699, a6700)
            };
            if v0 {
                let a6701 = v12;
                let a6702 = 0u64;
                if nat_eq(a6701, a6702) {
                    Ok(f293(9u64, 0u64, v10, 1u64, 3u64, 3u64, v11))
                } else {
                    let a6703 = v12;
                    let a6704 = 1u64;
                    if nat_eq(a6703, a6704) {
                        Ok(f293(10u64, 3u64, v11, 3u64, v11, 1u64, 2u64))
                    } else {
                        let a6705 = v12;
                        let a6706 = 2u64;
                        if nat_eq(a6705, a6706) {
                            Ok(f293(1u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64))
                        } else {
                            Ok(f293(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                        }
                    }
                }
            } else {
                let a6707 = v12;
                let a6708 = 0u64;
                if nat_eq(a6707, a6708) {
                    Ok(f293(1u64, 0u64, v10, 2u64, 0u64, 1u64, 2u64))
                } else {
                    Ok(f293(0u64, 1u64, 1u64, 1u64, 2u64, 1u64, 1u64))
                }
            }
        } else {
            let a6709 = v1;
            let a6710 = v7;
            if nat_eq(a6709, a6710) {
                Ok(f293(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
            } else {
                Ok(Adt17::C0)
            }
        }
    }
}

pub fn f303(v0: u64, v1: u64) -> Adt17 {
    let a6711 = v0;
    let a6712 = 0u64;
    if nat_eq(a6711, a6712) {
        f293(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6713 = v0;
        let a6714 = 1u64;
        if nat_eq(a6713, a6714) {
            f293(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 3u64)
        } else {
            let a6715 = v0;
            let a6716 = 2u64;
            if nat_eq(a6715, a6716) {
                f293(2u64, 1u64, 3u64, 2u64, 0u64, 1u64, 3u64)
            } else {
                let a6717 = v0;
                let a6718 = 3u64;
                if nat_eq(a6717, a6718) {
                    f291(0u64)
                } else {
                    Adt17::C0
                }
            }
        }
    }
}

pub fn f304(v0: u64, v1: u64, v2: u64) -> Adt17 {
    let a6719 = v0;
    let a6720 = 0u64;
    if nat_eq(a6719, a6720) {
        f293(22u64, 2u64, v1, 2u64, 0u64, 1u64, 2u64)
    } else {
        let a6721 = v0;
        let a6722 = 1u64;
        if nat_eq(a6721, a6722) {
            f293(21u64, 1u64, 0u64, 1u64, 2u64, 1u64, 2u64)
        } else {
            let a6723 = v0;
            let a6724 = 2u64;
            if nat_eq(a6723, a6724) {
                f293(2u64, 1u64, 2u64, 2u64, 0u64, 1u64, 2u64)
            } else {
                let a6725 = v0;
                let a6726 = 3u64;
                if nat_eq(a6725, a6726) {
                    f293(4u64, 1u64, 2u64, 2u64, v2, 1u64, 1u64)
                } else {
                    let a6727 = v0;
                    let a6728 = 4u64;
                    if nat_eq(a6727, a6728) {
                        f293(12u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64)
                    } else {
                        let a6729 = v0;
                        let a6730 = 5u64;
                        if nat_eq(a6729, a6730) {
                            f293(11u64, 2u64, 1065353216u64, 1u64, 1u64, 1u64, 1u64)
                        } else {
                            Adt17::C0
                        }
                    }
                }
            }
        }
    }
}

pub fn f305(v0: u64, v1: Adt21, v2: Adt20) -> R<u64> {
    let a6741 = {
        let a6739 = {
            let a6736 = {
                let a6733 = match v1.clone() {
                    Adt21::C0(_, _, h6731, _) => {
                        h6731
                    }
                };
                let a6734 = match v2.clone() {
                    Adt20::C0(_, _, h6732, _, _, _, _, _) => {
                        h6732
                    }
                };
                nat_mul(a6733, a6734)?
            };
            let a6737 = match v1 {
                Adt21::C0(h6735, _, _, _) => {
                    h6735
                }
            };
            nat_add(a6736, a6737)?
        };
        let a6740 = match v2 {
            Adt20::C0(_, h6738, _, _, _, _, _, _) => {
                h6738
            }
        };
        nat_mul(a6739, a6740)?
    };
    let a6742 = v0;
    nat_add(a6741, a6742)
}

pub fn f306(v0: u64) -> u64 {
    if f204(v0) {
        let m6743 = f197(v0);
        match m6743 {
            Adt18::C0 => {
                0u64
            }
            Adt18::C1(v1) => {
                let a6745 = 1u64;
                let a6746 = match v1.clone() {
                    Adt19::C0(_, h6744, _) => {
                        h6744
                    }
                };
                if if nat_lt(a6745, a6746) {
                    true
                } else {
                    let a6747 = v0;
                    let a6748 = 43u64;
                    nat_eq(a6747, a6748)
                } {
                    match v1 {
                        Adt19::C0(_, h6749, _) => {
                            h6749
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

pub fn f307(v0: u64, v1: u64, v2: u64, v3: Adt20, v4: Adt20) -> R<Adt17> {
    let v5: u64 = {
        let a6757 = {
            let a6755 = {
                let a6751 = v1;
                let a6752 = match v3.clone() {
                    Adt20::C0(h6750, _, _, _, _, _, _, _) => {
                        h6750
                    }
                };
                nat_mul(a6751, a6752)?
            };
            let a6756 = {
                let a6753 = v2;
                let a6754 = 4u64;
                nat_mul(a6753, a6754)?
            };
            nat_add(a6755, a6756)?
        };
        let a6758 = f310(v0);
        nat_add(a6757, a6758)?
    };
    let v6: u64 = f292(v5, v3)?;
    let v7: u64 = f292(v5, v4)?;
    let a6759 = v0;
    let a6760 = 0u64;
    if if if if nat_eq(a6759, a6760) {
        true
    } else {
        let a6761 = v0;
        let a6762 = 2u64;
        nat_eq(a6761, a6762)
    } {
        true
    } else {
        let a6763 = v0;
        let a6764 = 5u64;
        nat_eq(a6763, a6764)
    } {
        true
    } else {
        let a6765 = v0;
        let a6766 = 7u64;
        nat_eq(a6765, a6766)
    } {
        Ok(f293(9u64, 0u64, v6, 1u64, 3u64, 3u64, v7))
    } else {
        let a6767 = v0;
        let a6768 = 1u64;
        if if if if nat_eq(a6767, a6768) {
            true
        } else {
            let a6769 = v0;
            let a6770 = 3u64;
            nat_eq(a6769, a6770)
        } {
            true
        } else {
            let a6771 = v0;
            let a6772 = 6u64;
            nat_eq(a6771, a6772)
        } {
            true
        } else {
            let a6773 = v0;
            let a6774 = 8u64;
            nat_eq(a6773, a6774)
        } {
            Ok(f293(10u64, 3u64, v7, 3u64, v7, 1u64, {
                let a6775 = v0;
                let a6776 = 1u64;
                if nat_eq(a6775, a6776) {
                    4u64
                } else {
                    let a6777 = v0;
                    let a6778 = 6u64;
                    if nat_eq(a6777, a6778) {
                        5u64
                    } else {
                        1u64
                    }
                }
            }))
        } else {
            let a6779 = v0;
            let a6780 = 4u64;
            if nat_eq(a6779, a6780) {
                Ok(f293(4u64, 1u64, 4u64, 1u64, 1u64, 1u64, 4u64))
            } else {
                let a6781 = v0;
                let a6782 = 9u64;
                if nat_eq(a6781, a6782) {
                    Ok(f293(4u64, 1u64, 5u64, 1u64, 1u64, 1u64, 5u64))
                } else {
                    let a6783 = v0;
                    let a6784 = 10u64;
                    if nat_eq(a6783, a6784) {
                        Ok(f293(4u64, 1u64, 4u64, 1u64, 5u64, 1u64, 1u64))
                    } else {
                        let a6785 = v0;
                        let a6786 = 11u64;
                        if nat_eq(a6785, a6786) {
                            Ok(f293(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                        } else {
                            let a6787 = v0;
                            let a6788 = 12u64;
                            if nat_eq(a6787, a6788) {
                                Ok(f293(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
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

pub fn f308(v0: u64, v1: u64, v2: Adt20, v3: Adt20, v4: bool) -> R<Adt17> {
    let v5: u64 = f292(v1, v2)?;
    let v6: u64 = f292(v1, v3)?;
    let a6789 = v0;
    let a6790 = 0u64;
    if nat_eq(a6789, a6790) {
        Ok(f293(9u64, 0u64, v5, 1u64, 3u64, 3u64, v6))
    } else {
        let a6791 = v0;
        let a6792 = 1u64;
        if nat_eq(a6791, a6792) {
            Ok(f293(10u64, 3u64, v6, 3u64, v6, 1u64, 1u64))
        } else {
            if v4 {
                let a6793 = v0;
                let a6794 = 2u64;
                if nat_eq(a6793, a6794) {
                    Ok(f293(1u64, 1u64, 1u64, 2u64, 0u64, 1u64, 1u64))
                } else {
                    let a6795 = v0;
                    let a6796 = 3u64;
                    if nat_eq(a6795, a6796) {
                        Ok(f293(0u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                    } else {
                        Ok(Adt17::C0)
                    }
                }
            } else {
                let a6797 = v0;
                let a6798 = 2u64;
                if nat_eq(a6797, a6798) {
                    Ok(f293(4u64, 1u64, 0u64, 1u64, 1u64, 1u64, 0u64))
                } else {
                    Ok(Adt17::C0)
                }
            }
        }
    }
}

pub fn f309(v0: u64) -> R<u64> {
    let a6799 = 4u64;
    let a6800 = f285(v0);
    nat_mul(a6799, a6800)
}

pub fn f310(v0: u64) -> u64 {
    let a6801 = v0;
    let a6802 = 5u64;
    if nat_lt(a6801, a6802) {
        let a6803 = v0;
        let a6804 = 2u64;
        if nat_lt(a6803, a6804) {
            0u64
        } else {
            1u64
        }
    } else {
        let a6805 = v0;
        let a6806 = 7u64;
        if nat_lt(a6805, a6806) {
            2u64
        } else {
            3u64
        }
    }
}
